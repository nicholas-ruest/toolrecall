# Project status

ToolRecall is an experimental research build. It is not deployed and no crate is published.

## Completion gates

- Architecture and contracts: implemented.
- OpenAI, MCP, RuVector and RVF vertical slice: implemented; evidence under `evidence/` is authoritative.
- Rust quality gates and deterministic benchmark: see `evidence/commands.tsv`.
- Darwin/Flywheel evaluation: bounded advisory execution only.
- Repository/PR/Gist publication: see the final run receipt and public readbacks.

## Known gaps

- The retriever is lexical and intentionally small.
- The frozen corpus is mechanism evidence, not production representativeness.
- No Vertex/OpenAI model call is made; live model impact remains unmeasured.
- Production ACL/Cloud SQL deployment is not included.
- Policy promotion requires Nick’s explicit graduation decision.
