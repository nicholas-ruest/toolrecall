# Dream Machine build — ToolRecall — 2026-10-05

ToolRecall accepts a real OpenAI deferred-tool catalog, runs deterministic source-aware recall, validates the selected tools with the official MCP Rust SDK, persists and reads back outcome evidence through RuVector, and seals a chained RVF witness.

The seven-package Rust workspace includes three frozen strategies, integration/error tests, a reproducible benchmark, 25 ADRs, 12 DDD views, Mermaid diagrams, and a project-specific SVG. Darwin and Flywheel are bounded evaluators with no promotion authority.

Reproduce:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --all-features --locked
cargo deny check
cargo run --locked -- --query "has invoice 77 been paid" --required-source billing --required-tool get_invoice_status --python python3
cargo run --locked -p toolrecall-evaluation --bin toolrecall-benchmark -- --json
```

Repository: https://github.com/nicholas-ruest/toolrecall  
Merged build snapshot: `f50d612fe375d317f84a7ef1c7ae22ebced72f77`  
Exact-main CI: https://github.com/nicholas-ruest/toolrecall/actions/runs/37559009063

Results: four Rust tests passed; source-aware coverage was 1.000 with 2.167 tools loaded on average, versus 0.667/1.000 for global top-k and 1.000/8.000 for full catalog. Darwin selected `top_k=1,max_loaded=2` at 0.9600 versus the 0.9567 baseline. Flywheel replay verified every receipt/authenticity check with `authority: none`.

Merged delivery: architecture PR #1, validation PR #3, and implementation PR #2. Limitations: lexical corpus, no live model call, no production deployment, public Gists not yet published, and human approval required for graduation.

This repository file is publication-ready but is not evidence of Gist publication until a public Gist owned by `nicholas-ruest` is read back.
