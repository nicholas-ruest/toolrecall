# Dream Machine build — ToolRecall

**Original build date:** 2026-10-05  
**Recovery publication preparation:** 2026-10-07/08  
**Repository:** https://github.com/nicholas-ruest/toolrecall  
**Immutable implementation merge:** `f50d612fe375d317f84a7ef1c7ae22ebced72f77`  
**Visual recovery source:** `2cabbcda38c64e8954b4ef4fc5f253758a93be7c`

## What was built

ToolRecall accepts a real OpenAI deferred-tool catalog, validates typed input, runs deterministic source-aware recall, validates selected tools with the official MCP Rust SDK, persists and reads back outcome evidence through RuVector, and seals catalog/decision digests with an RVF witness. The CLI returns an inspectable JSON receipt and fails closed on invalid policy, malformed sidecar data, timeout, MCP validation failure, memory readback failure, or witness mismatch.

The system is advisory. Every receipt and evaluator result carries `authority: none`; no component can execute a tool, promote a policy, publish, deploy, or expand its safety envelope.

## Implemented composition

| Ingredient | Pin | Executed capability |
|---|---|---|
| OpenAI Agents SDK | `openai-agents==0.23.1` | materializes real `defer_loading=True` tool definitions through a bounded Python sidecar |
| MCP Rust SDK | `rmcp 3.4.0` | validates tool names and schemas with the official model |
| RuVector | source `5a93328f…`, crate `2.3.1` | inserts outcome vectors and asserts exact readback |
| RVF | `rvf-crypto 0.2.0` | creates/verifies a witness chain and rejects tampering |
| MetaHarness | Darwin `0.10.3`, Flywheel `0.1.12` | scores bounded candidates and verifies replay bundles without promotion authority |

## Workspace and domain boundaries

The seven-package Rust workspace is derived from the domain and ports/adapters architecture:

- `toolrecall-domain`: catalog, source obligation, strategies, receipt, metrics;
- `toolrecall-application`: workflow and typed ports;
- `toolrecall-adapter-openai`: isolated SDK sidecar boundary;
- `toolrecall-adapter-mcp`: official MCP model validation;
- `toolrecall-adapter-ruvnet`: RuVector memory/readback and RVF sealing;
- `toolrecall-evaluation`: frozen corpus, baselines, ablations, evaluator candidate;
- root CLI: boundary validation and complete vertical slice.

The repository contains 25 substantive ADRs and 12 detailed DDD documents, plus specification, architecture, frozen implementation contract, and acceptance traceability.

## Reproduce

```bash
python3 -m venv .venv
.venv/bin/pip install openai-agents==0.23.1

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --all-features --locked
cargo deny check

cargo run --locked -- \
  --query "has invoice 77 been paid" \
  --required-source billing \
  --required-tool get_invoice_status \
  --python .venv/bin/python

cargo run --locked -p toolrecall-evaluation \
  --bin toolrecall-benchmark -- --json
```

The exact merged-main GitHub run is https://github.com/nicholas-ruest/toolrecall/actions/runs/37559009063. It passed formatting, strict Clippy, locked workspace tests, dependency/security policy, the OpenAI→MCP→RuVector→RVF vertical slice, and the frozen benchmark. Four Rust tests passed. The inherited `RUSTSEC-2025-0141` exception is explicit in the dependency policy; it is not silently ignored.

## Measured result

| Strategy | Required-source coverage | Target recall | Average tools exposed |
|---|---:|---:|---:|
| Global top-k | 0.667 | 1.000 | 1.000 |
| Source guard | 1.000 | 1.000 | 2.167 |
| Full catalog | 1.000 | 1.000 | 8.000 |

On the frozen six-case lexical corpus, source guard eliminated required-source collapse and preserved target recall while exposing fewer tools than full catalog. This is mechanism evidence, not a production superiority claim.

Darwin selected `top_k=1,max_loaded=2` at `0.9600`, versus a `0.9567` baseline. Flywheel replay verified every receipt and authenticity check. Both reported `authority: none`.

## Delivery ledger

| Scope | Branch / PR | Merge commit |
|---|---|---|
| architecture and contracts | `dream/2026-10-05-design` / [PR #1](https://github.com/nicholas-ruest/toolrecall/pull/1) | `9783200fd58a771902e96e08256fc297a5e819ae` |
| validation evidence | `dream/2026-10-05-validation` / [PR #3](https://github.com/nicholas-ruest/toolrecall/pull/3) | `6412124ea2323c66eab0ac790de6848b1319f2b8` |
| implementation and integrations | `dream/2026-10-05-implementation` / [PR #2](https://github.com/nicholas-ruest/toolrecall/pull/2) | `f50d612fe375d317f84a7ef1c7ae22ebced72f77` |
| final readback | `dream/2026-10-06-final-readback` / [PR #4](https://github.com/nicholas-ruest/toolrecall/pull/4) | `44122bfb2fa1450a47393653b7f13979efba7c04` |
| visual/publication recovery | `recovery/2026-10-07-publication-visuals` / [PR #5](https://github.com/nicholas-ruest/toolrecall/pull/5) | visual source `2cabbcda38c64e8954b4ef4fc5f253758a93be7c`; merge receipt pending |

## README visual recovery

The recovery replaces remote avatars and mislabeled generic icons with repository-versioned SVG assets and explicit typographic identifiers. It adds:

- `docs/assets/toolrecall-hero-animated.svg` — source gate overview;
- `docs/assets/toolrecall-flow-animated.svg` — execution/evidence flow;
- `docs/assets/marks/` — versioned Rust mark and clearly labeled project identifiers;
- `docs/assets/ATTRIBUTION.md` — provenance and trademark/endorsement boundary.

Both diagrams are script-free SVG, use no external fonts or resources, retain static labels, and include reduced-motion behavior. They explain implemented behavior and do not imply live telemetry.

## Evidence and limitations

- Evidence: https://github.com/nicholas-ruest/toolrecall/tree/main/evidence
- Architecture graphic: https://github.com/nicholas-ruest/toolrecall/blob/main/docs/assets/toolrecall-flow-animated.svg
- ADR index: https://github.com/nicholas-ruest/toolrecall/blob/main/docs/adrs/README.md
- DDD index: https://github.com/nicholas-ruest/toolrecall/blob/main/docs/ddd/README.md
- Research report: `dream-machine-research-2026-10-05-toolrecall.md`

Limitations: lexical retrieval, a six-case corpus, no live model call, no production RuVector/Cloud SQL deployment, and no autonomous policy promotion. The first integration attempt failed on an obsolete interpreter path; the corrected run passed and both attempts remain in evidence. The first evaluator restore run lacked a built candidate binary; the exact candidate was then built and both Darwin and Flywheel passed, with the failed attempt retained.

**Decision:** REVISE before graduation. Preserve the source-obligation and evidence boundaries while validating semantic retrieval and live-model task outcomes on a larger held-out corpus.

This announcement contains no secrets, credentials, private repository content, personal data, or confidential material.
