use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::{self, Read},
};
use toolrecall_domain::{RecallPolicy, Strategy, metrics, select};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    variant_id: String,
    genome: BTreeMap<String, f64>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Score {
    primary: f64,
    regressed: bool,
    noop_rate: f64,
    cost_per_win: f64,
    raw: Raw,
}
#[derive(Serialize)]
struct Raw {
    variant_id: String,
    coverage: f64,
    avg_loaded: f64,
    authority: String,
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn required(g: &BTreeMap<String, f64>, k: &str) -> Result<usize, String> {
    let v = *g.get(k).ok_or_else(|| format!("missing {k}"))?;
    if !v.is_finite() || v < 1.0 || v.fract() != 0.0 {
        return Err(format!("invalid {k}"));
    }
    Ok(v as usize)
}
fn run() -> Result<(), String> {
    let mut s = String::new();
    io::stdin()
        .read_to_string(&mut s)
        .map_err(|e| e.to_string())?;
    let c: Candidate = serde_json::from_str(&s).map_err(|e| e.to_string())?;
    let top_k = required(&c.genome, "top_k")?;
    let max_loaded = required(&c.genome, "max_loaded")?;
    let p = RecallPolicy {
        strategy: Strategy::SourceGuard,
        top_k,
        min_per_required_source: 1,
        max_loaded,
    };
    let cat = toolrecall_evaluation::catalog();
    let ev: Vec<_> = toolrecall_evaluation::cases()
        .into_iter()
        .map(|q| {
            let r = select(&q, &cat, &p).unwrap();
            (q, r)
        })
        .collect();
    let m = metrics(&ev);
    let coverage = m["coverage"];
    let avg = m["avg_loaded"];
    let primary = coverage - 0.02 * avg;
    let out = Score {
        primary,
        regressed: coverage < 1.0,
        noop_rate: 1.0 - coverage,
        cost_per_win: avg / coverage.max(0.01),
        raw: Raw {
            variant_id: c.variant_id,
            coverage,
            avg_loaded: avg,
            authority: "none".into(),
        },
    };
    println!("{}", serde_json::to_string(&out).unwrap());
    Ok(())
}
