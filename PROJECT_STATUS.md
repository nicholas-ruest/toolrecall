# Project status

ToolRecall is an experimental research build. It is not deployed and no crate is published.

## Verified delivery

- Architecture and contracts: implemented in PR #1; merge commit `9783200fd58a771902e96e08256fc297a5e819ae`.
- Validation evidence: merged through PR #3 into the implementation branch; merge commit `6412124ea2323c66eab0ac790de6848b1319f2b8`.
- Implementation and integrations: merged to `main` through PR #2 at `f50d612fe375d317f84a7ef1c7ae22ebced72f77`.
- Exact merged-main CI: run [37559009063](https://github.com/nicholas-ruest/toolrecall/actions/runs/37559009063) passed formatting, strict Clippy, locked workspace tests, cargo-deny, the OpenAI-to-MCP-to-RuVector-to-RVF vertical slice, and the frozen benchmark.
- Darwin/Flywheel evaluation: bounded advisory execution only; `authority: none`.
- Repository: public, with `main` as the default branch.
- Gists: blocked. The authenticated repository connector exposes no Gist action, the executor CLI is logged out, and unattended browser publication cannot authorize a representational post.

## Known gaps

- The retriever is lexical and intentionally small.
- The frozen six-case corpus is mechanism evidence, not production representativeness.
- No Vertex/OpenAI model call is made; live model impact remains unmeasured.
- Production ACL/Cloud SQL deployment is not included.
- Public research and build Gists have not been published or read back.
- Policy promotion and graduation remain owner decisions.
- The first integration attempt used an obsolete interpreter path and failed before integration code ran; the corrected rerun passed and both attempts are retained.
- The first evaluator restore run lacked a built candidate binary; Darwin and Flywheel passed after the exact candidate was built, with both attempts retained.
