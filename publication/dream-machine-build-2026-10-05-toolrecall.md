# Dream Machine build — ToolRecall — 2026-10-05

ToolRecall accepts a real OpenAI deferred-tool catalog, runs deterministic source-aware recall, validates the selected tools with the official MCP Rust SDK, persists/read-backs outcome evidence through RuVector, and seals a chained RVF witness.

The seven-package Rust workspace includes three frozen strategies, integration/error tests, a reproducible benchmark, 25 ADRs, 12 DDD views, Mermaid diagrams, and a project-specific SVG. Darwin and Flywheel are bounded evaluators with no promotion authority.

Reproduce:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --all-features --locked
cargo deny check
cargo run --locked -p toolrecall-evaluation
```

Repository: https://github.com/nicholas-ruest/toolrecall

Limitations: lexical corpus, no live model call, no production deployment, human approval required for graduation. Exact commit and measurements must be filled from the merged validation receipt before public posting.

This file is publication-ready but is not evidence of Gist publication until a public Gist owned by `nicholas-ruest` is read back.
