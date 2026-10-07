# Dream Machine build — ToolRecall — 2026-10-05

ToolRecall accepts a real OpenAI deferred-tool catalog, runs deterministic source-aware recall, validates the selected tools with the official MCP Rust SDK, persists/read-backs outcome evidence through RuVector, and seals a chained RVF witness.

The seven-package Rust workspace includes three frozen strategies, integration/error tests, a reproducible benchmark, 25 ADRs, 12 DDD views, Mermaid diagrams, and a project-specific SVG. Darwin and Flywheel are bounded evaluators with no promotion authority.

Reproduce:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --all-features --locked
cargo deny check
cargo run --locked -p toolrecall-evaluation --bin toolrecall-benchmark -- --json
```

Repository: https://github.com/nicholas-ruest/toolrecall  
Validated source commit: `910bd04097d228ca2bb83511f78f4fedc2b1cffe`

Results: four Rust tests passed; source-aware coverage was 1.000 with 2.167 tools loaded on average, versus 0.667/1.000 for global top-k and 1.000/8.000 for full catalog. Darwin selected `top_k=1,max_loaded=2` at 0.9600 versus the 0.9567 baseline. Flywheel replay verified every receipt/authenticity check with `authority: none`.

Limitations: lexical corpus, no live model call, no production deployment, repository delivery is tracked through tested PRs #2 and #3, and human approval is required for graduation.

This file is publication-ready but is not evidence of Gist publication until a public Gist owned by `nicholas-ruest` is read back.
