use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolDescriptor {
    pub source: String,
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub deferred: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCase {
    pub id: String,
    pub text: String,
    pub required_sources: BTreeSet<String>,
    pub required_tools: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Strategy {
    GlobalTopK,
    SourceGuard,
    FullCatalog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallPolicy {
    pub strategy: Strategy,
    pub top_k: usize,
    pub min_per_required_source: usize,
    pub max_loaded: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionReceipt {
    pub query_id: String,
    pub selected: Vec<ToolDescriptor>,
    pub missing_sources: Vec<String>,
    pub missing_tools: Vec<String>,
    pub catalog_sha256: String,
    pub authority: String,
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("catalog must not be empty")]
    EmptyCatalog,
    #[error("policy bounds are invalid")]
    InvalidPolicy,
}

pub fn select(
    query: &QueryCase,
    catalog: &[ToolDescriptor],
    policy: &RecallPolicy,
) -> Result<SelectionReceipt, DomainError> {
    if catalog.is_empty() {
        return Err(DomainError::EmptyCatalog);
    }
    if policy.top_k == 0 || policy.max_loaded == 0 || policy.top_k > policy.max_loaded {
        return Err(DomainError::InvalidPolicy);
    }
    let q = tokens(&query.text);
    let mut ranked: Vec<(usize, &ToolDescriptor)> = catalog
        .iter()
        .map(|t| {
            let mut text = t.name.clone();
            text.push(' ');
            text.push_str(&t.description);
            text.push(' ');
            text.push_str(&t.source);
            let score = tokens(&text).intersection(&q).count();
            (score, t)
        })
        .collect();
    ranked.sort_by(|a, b| {
        b.0.cmp(&a.0)
            .then_with(|| a.1.source.cmp(&b.1.source))
            .then_with(|| a.1.name.cmp(&b.1.name))
    });
    let mut selected: Vec<ToolDescriptor> = match policy.strategy {
        Strategy::FullCatalog => catalog.to_vec(),
        _ => ranked
            .iter()
            .take(policy.top_k)
            .map(|(_, t)| (*t).clone())
            .collect(),
    };
    if policy.strategy == Strategy::SourceGuard {
        for source in &query.required_sources {
            let have = selected.iter().filter(|t| &t.source == source).count();
            if have < policy.min_per_required_source {
                for (_, tool) in ranked.iter().filter(|(_, t)| &t.source == source) {
                    if selected.iter().all(|x| x.name != tool.name) {
                        selected.push((*tool).clone());
                    }
                    if selected.iter().filter(|t| &t.source == source).count()
                        >= policy.min_per_required_source
                    {
                        break;
                    }
                }
            }
        }
    }
    selected.truncate(policy.max_loaded);
    let sources: BTreeSet<_> = selected.iter().map(|t| t.source.clone()).collect();
    let names: BTreeSet<_> = selected.iter().map(|t| t.name.clone()).collect();
    Ok(SelectionReceipt {
        query_id: query.id.clone(),
        missing_sources: query
            .required_sources
            .difference(&sources)
            .cloned()
            .collect(),
        missing_tools: query.required_tools.difference(&names).cloned().collect(),
        catalog_sha256: catalog_digest(catalog),
        selected,
        authority: "none".into(),
    })
}

fn tokens(s: &str) -> BTreeSet<String> {
    s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|x| x.len() > 1)
        .map(str::to_string)
        .collect()
}

pub fn catalog_digest(catalog: &[ToolDescriptor]) -> String {
    let mut stable = catalog.to_vec();
    stable.sort_by(|a, b| a.source.cmp(&b.source).then(a.name.cmp(&b.name)));
    hex::encode(Sha256::digest(
        serde_json::to_vec(&stable).expect("serializable"),
    ))
}

pub fn metrics(cases: &[(QueryCase, SelectionReceipt)]) -> BTreeMap<String, f64> {
    let n = cases.len().max(1) as f64;
    let complete = cases
        .iter()
        .filter(|(_, r)| r.missing_sources.is_empty() && r.missing_tools.is_empty())
        .count() as f64;
    let avg_loaded = cases
        .iter()
        .map(|(_, r)| r.selected.len() as f64)
        .sum::<f64>()
        / n;
    BTreeMap::from([
        ("coverage".into(), complete / n),
        ("avg_loaded".into(), avg_loaded),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    fn td(s: &str, n: &str, d: &str) -> ToolDescriptor {
        ToolDescriptor {
            source: s.into(),
            name: n.into(),
            description: d.into(),
            input_schema: serde_json::json!({"type":"object"}),
            deferred: true,
        }
    }
    #[test]
    fn source_guard_recovers_collapsed_source() {
        let cat = vec![
            td("crm", "profile", "customer profile"),
            td("billing", "invoice", "invoice status"),
        ];
        let q = QueryCase {
            id: "q".into(),
            text: "customer status".into(),
            required_sources: BTreeSet::from(["billing".into()]),
            required_tools: BTreeSet::from(["invoice".into()]),
        };
        let p = RecallPolicy {
            strategy: Strategy::SourceGuard,
            top_k: 1,
            min_per_required_source: 1,
            max_loaded: 2,
        };
        let r = select(&q, &cat, &p).unwrap();
        assert!(r.missing_sources.is_empty());
        assert!(r.missing_tools.is_empty());
        assert_eq!(r.authority, "none");
    }
    #[test]
    fn invalid_policy_fails() {
        let cat = vec![td("a", "b", "c")];
        let q = QueryCase {
            id: "q".into(),
            text: "x".into(),
            required_sources: BTreeSet::new(),
            required_tools: BTreeSet::new(),
        };
        let p = RecallPolicy {
            strategy: Strategy::GlobalTopK,
            top_k: 2,
            min_per_required_source: 0,
            max_loaded: 1,
        };
        assert!(select(&q, &cat, &p).is_err());
    }
}
