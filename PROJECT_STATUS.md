# Project status

ToolRecall is an experimental research build. It is not deployed and no crate is published.

## Verified delivery

- Architecture and contracts: merged through [PR #1](https://github.com/nicholas-ruest/toolrecall/pull/1) at `9783200fd58a771902e96e08256fc297a5e819ae`.
- Validation evidence: merged through [PR #3](https://github.com/nicholas-ruest/toolrecall/pull/3) at `6412124ea2323c66eab0ac790de6848b1319f2b8`.
- Implementation and integrations: merged through [PR #2](https://github.com/nicholas-ruest/toolrecall/pull/2) at `f50d612fe375d317f84a7ef1c7ae22ebced72f77`.
- Final build readback: merged through [PR #4](https://github.com/nicholas-ruest/toolrecall/pull/4) at `44122bfb2fa1450a47393653b7f13979efba7c04`.
- Exact merged-main CI: [run 37559009063](https://github.com/nicholas-ruest/toolrecall/actions/runs/37559009063) passed formatting, strict Clippy, locked workspace tests, cargo-deny, the OpenAI→MCP→RuVector→RVF vertical slice, and the frozen benchmark.
- Darwin/Flywheel evaluation: bounded advisory execution only; `authority: none`.
- Repository: public, with `main` as the default branch.

## Recovery in progress

- README visual recovery: [PR #5](https://github.com/nicholas-ruest/toolrecall/pull/5) replaces remote avatar stand-ins with versioned assets, adds two custom animated/static-readable SVGs, adds reduced-motion behavior and attribution, and strengthens navigation/evidence traceability.
- Applied guidance: [`docs/applied-guidance-2026-10-07.md`](docs/applied-guidance-2026-10-07.md) records how the current core-memory engineering profile was applied and revisited.
- Publication drafts: the dated research and build files are complete and self-contained under [`publication/`](publication/).
- Public Gists: **BLOCKED_GIST_PUBLISH**. The GitHub repository connector exposes no Gist action and RuOS `gh auth status` is logged out. The authenticated browser proves ownership and shows no duplicate ToolRecall Gists, but browser publication is a representational post that cannot be submitted unattended under the active computer-use confirmation policy.

## Completion gates

- IMPLEMENTED: yes, at `f50d612fe375d317f84a7ef1c7ae22ebced72f77`.
- VALIDATED: yes, with immutable exact-main CI and committed evaluator evidence.
- REPO_PUBLISHED: yes; PR #5 is a separate recovery increment until merged.
- RESEARCH_GIST: no — publication-ready, not published/read back.
- BUILD_GIST: no — publication-ready, not published/read back.
- BUILT: no under the factory completion gate because both dated public Gists are missing.

## Known gaps

- The retriever is lexical and the frozen corpus has six cases.
- No live Vertex/OpenAI model call was made; downstream task impact remains unmeasured.
- Production RuVector/Cloud SQL persistence is not included.
- Policy promotion and graduation remain owner decisions.
- The first integration attempt used an obsolete interpreter path and failed before integration code ran; the corrected rerun passed and both attempts are retained.
- The first evaluator restore run lacked a built candidate binary; Darwin and Flywheel passed after the exact candidate was built, with both attempts retained.

**Current owner decision:** REVISE before graduation.
