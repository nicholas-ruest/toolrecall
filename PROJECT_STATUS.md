# Project status

ToolRecall is an experimental research build. It is not deployed and no crate is published.

## Completion gates at source commit `910bd04097d228ca2bb83511f78f4fedc2b1cffe`

- Architecture and contracts: implemented.
- OpenAI, MCP, RuVector and RVF vertical slice: implemented; evidence under `evidence/` is authoritative.
- Rust quality gates and deterministic benchmark: see `evidence/commands.tsv`.
- Darwin/Flywheel evaluation: bounded advisory execution only.
- Repository: public. Architecture PR #1 merged; implementation PR #2 remains open because the exact head has no GitHub-hosted status checks.
- Gists: blocked. The repository connector exposes no Gist action, the executor CLI is logged out, and browser publication cannot be completed noninteractively.

## Known gaps

- The retriever is lexical and intentionally small.
- The frozen corpus is mechanism evidence, not production representativeness.
- No Vertex/OpenAI model call is made; live model impact remains unmeasured.
- Production ACL/Cloud SQL deployment is not included.
- Policy promotion requires Nick’s explicit graduation decision.
- The first evaluator retry after executor restoration lacked a built candidate binary. Darwin produced `-Infinity` and Flywheel failed; after building the exact candidate target, both reruns passed. Both attempts are retained in evidence.
