use serde::Serialize;
use toolrecall_domain::{RecallPolicy, Strategy, metrics, select};
#[derive(Serialize)]
struct Row {
    strategy: Strategy,
    coverage: f64,
    avg_loaded: f64,
    authority: String,
}
fn main() {
    let cat = toolrecall_evaluation::catalog();
    let cases = toolrecall_evaluation::cases();
    let policies = [
        RecallPolicy {
            strategy: Strategy::GlobalTopK,
            top_k: 1,
            min_per_required_source: 0,
            max_loaded: 1,
        },
        RecallPolicy {
            strategy: Strategy::SourceGuard,
            top_k: 1,
            min_per_required_source: 2,
            max_loaded: 3,
        },
        RecallPolicy {
            strategy: Strategy::FullCatalog,
            top_k: 1,
            min_per_required_source: 0,
            max_loaded: cat.len(),
        },
    ];
    let rows: Vec<_> = policies
        .into_iter()
        .map(|p| {
            let evaluated: Vec<_> = cases
                .iter()
                .cloned()
                .map(|q| {
                    let r = select(&q, &cat, &p).unwrap();
                    (q, r)
                })
                .collect();
            let m = metrics(&evaluated);
            Row {
                strategy: p.strategy,
                coverage: m["coverage"],
                avg_loaded: m["avg_loaded"],
                authority: "none".into(),
            }
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&rows).unwrap());
}
