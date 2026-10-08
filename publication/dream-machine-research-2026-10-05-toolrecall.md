# Dream Machine research — ToolRecall

**Original research/build date:** 2026-10-05  
**Recovery publication preparation:** 2026-10-07/08  
**Repository:** https://github.com/nicholas-ruest/toolrecall  
**Immutable implementation snapshot:** `f50d612fe375d317f84a7ef1c7ae22ebced72f77`  
**Final delivery readback before visual recovery:** `44122bfb2fa1450a47393653b7f13979efba7c04`

## Research question

Can a small, explicit source-coverage gate prevent total source collapse in deferred agent tool catalogs while exposing materially fewer tools than a full-catalog fallback?

ToolRecall is not a semantic retriever or authorization engine. It is an admission-and-evidence layer: normalize a real deferred catalog, apply a deterministic retrieval policy, enforce declared source obligations, validate selected tools against the official MCP Rust model, persist an outcome through RuVector, and seal the decision with an RVF witness.

## Method and date bounds

The October 5 selection reran two independent research streams: frontier primary research published or revised in the preceding 60 days, and enterprise OSS materially changed in the preceding 30 days. Sources were inspected before selection; older Ruvnet components were treated as pinned dependencies, not mislabeled as new releases. This recovery report preserves the original dates and adds no retroactive performance claim.

## Frontier mechanisms

### Source-style collapse

*When Tool-Backed Skill Retrieval Fails: Source-Style Collapse in Executable Capability Retrieval* (arXiv:2608.16502, 2026-08-17) isolates a failure where global retrieval suppresses entire tool sources. Its ToolScout mechanism uses source-aware routing. ToolRecall carries forward only the falsifiable mechanism—declared source obligations plus source-aware expansion—not the paper’s reported performance or implementation.

Source: https://arxiv.org/abs/2608.16502

### Declarative tool security

*ToolGuardian: Declarative Security for Agent-Tool Interactions* (2026-07) motivates treating tool exposure as an enforceable control-plane decision. ToolGuardian and ToolRecall are complementary: one governs what may be used; the other checks whether required sources survived recall.

### Skills as a systems layer

*A Systems Foundation for Agentic Skills* (2026-08-30) and agentic-SDLC control-plane work published 2026-09-04 informed the explicit lifecycle, receipt, and non-promotion boundaries. They did not supply the retrieval kernel.

## Enterprise and Ruvnet OSS inspection

| Project | Material date | Pinned source/package | Inspected interface | License | Integrated? |
|---|---:|---|---|---|---|
| OpenAI Agents SDK | repository update 2026-10-05 | `openai-agents 0.23.1`; `d7e52c375021248973f60ccbea8e7b69bc5b16e3` | `function_tool(defer_loading=True)` and tool-search example | MIT | yes, typed Python sidecar |
| MCP Rust SDK | release source inspected 2026-10-05 | repository `79437f291b2c44053d00dcd5db969fd0cca7c887`; compatible crate `rmcp 3.4.0` | `rmcp::model::Tool::new` | Apache-2.0 for current contributions; history includes transition notice | yes, Rust adapter |
| NVIDIA Skills | catalog regeneration 2026-10-02 | `0e0d506f4eb67204a62586ac5f19df3cb7ad9b1f` | live-catalog router and discovery contract | CC-BY-4.0 AND Apache-2.0 | inspected, rejected from slice |
| RuVector | inspected 2026-10-05 | `5a93328f2fceb0307c25929ed38cd7a0911fdf00`; crate `2.3.1` | `VectorDB` insert/search/get | Apache-2.0 | yes, outcome memory/readback |
| RVF | inspected 2026-10-05 | `rvf-crypto 0.2.0` | witness creation and verification | MIT OR Apache-2.0 | yes, evidence seal |
| MetaHarness | inspected 2026-10-05 | `9ce8b8dd89045c3b9a1f809ae58f3589029db4a4`; Darwin `0.10.3`, Flywheel `0.1.12` | bounded numeric candidate CLI and replay-bundle APIs | Apache-2.0 | yes, advisory evaluation |

The maintained OpenAI SDK is Python, so ToolRecall uses a narrow JSON sidecar instead of reimplementing it in Rust. MCP, RuVector, and RVF use real Rust APIs. NVIDIA Skills remained prior art: adding it would have expanded the surface without strengthening the chosen vertical slice.

## Ranked composition matrix

Scores are 1–5 for problem evidence, novelty, complementary composition, bounded testability, and enterprise usefulness.

| Candidate | Evidence | Novelty | Composition | Testability | Usefulness | Total | Decision |
|---|---:|---:|---:|---:|---:|---:|---|
| ToolRecall source-coverage firewall | 5 | 5 | 5 | 5 | 5 | **25** | selected |
| ToolGuardian-to-RVF policy compiler | 4 | 3 | 5 | 4 | 5 | 21 | rejected: overlaps established policy engines |
| Agentic-SDLC receipt packager | 4 | 3 | 4 | 4 | 4 | 19 | rejected: too close to existing evidence projects |
| RRSI retry simulator | 4 | 2 | 3 | 5 | 3 | 17 | rejected: JitterMap already occupies that mechanism |

## Selected hypothesis and falsification

For mixed-source deferred catalogs, a source-coverage gate can eliminate total required-source collapse while exposing materially fewer tools than a full-catalog fallback.

The hypothesis fails if any of the following occurs on the frozen corpus:

1. source guard misses a declared required source;
2. source guard reduces target recall below full catalog;
3. source guard expands to full-catalog exposure on every case;
4. a malformed or unvalidated tool reaches the accepted receipt;
5. RuVector readback or the RVF witness cannot be verified.

## Mechanism-to-code trace

| Ingredient or mechanism | Executed code path | Evidence |
|---|---|---|
| OpenAI deferred tools | `toolrecall-adapter-openai` sidecar materializes real SDK tool objects | vertical-slice test and CLI receipt |
| source-aware recall | domain strategies implement global top-k, source guard, and full catalog | frozen benchmark and ablation table |
| MCP validation | `toolrecall-adapter-mcp` constructs/validates official MCP tools | adapter and error-path tests |
| RuVector memory | `toolrecall-adapter-ruvnet` inserts and reads back outcomes | readback assertion in integration evidence |
| RVF witness | catalog and decision digests are chained and verified | witness and tamper-rejection tests |
| Darwin/Flywheel | candidate scoring and signed replay consume the compiled candidate | evaluator JSON with `authority: none` |

## Frozen result

| Strategy | Required-source coverage | Target recall | Average tools exposed |
|---|---:|---:|---:|
| Global top-k | 0.667 | 1.000 | 1.000 |
| Source guard | 1.000 | 1.000 | 2.167 |
| Full catalog | 1.000 | 1.000 | 8.000 |

The result supports the mechanism on six frozen lexical cases: source guard preserved target recall and required-source coverage while exposing fewer tools than full catalog. It does not establish production semantic-retrieval quality.

Darwin selected `top_k=1,max_loaded=2` at `0.9600`, versus a `0.9567` baseline. Flywheel verified the replay bundle and authenticity checks. Both remained advisory and emitted `authority: none`.

## Reproducibility and evidence

- repository: https://github.com/nicholas-ruest/toolrecall
- implementation merge: https://github.com/nicholas-ruest/toolrecall/commit/f50d612fe375d317f84a7ef1c7ae22ebced72f77
- exact-main CI: https://github.com/nicholas-ruest/toolrecall/actions/runs/37559009063
- architecture: https://github.com/nicholas-ruest/toolrecall/blob/main/docs/architecture.md
- research source record: https://github.com/nicholas-ruest/toolrecall/blob/main/docs/research.md
- raw evidence directory: https://github.com/nicholas-ruest/toolrecall/tree/main/evidence
- 25 ADR index: https://github.com/nicholas-ruest/toolrecall/blob/main/docs/adrs/README.md
- 12 DDD index: https://github.com/nicholas-ruest/toolrecall/blob/main/docs/ddd/README.md

## Limitations and handoff

- The corpus is small and lexical; no production-representativeness claim is made.
- No live model request was executed, so downstream task impact is unmeasured.
- Local RuVector evidence is not a production Cloud SQL deployment.
- Declared source obligations must come from an authorized policy boundary.
- ToolRecall cannot execute tools, publish, deploy, or promote itself.

**Owner decision:** REVISE before graduation. The next falsifiable increment is a larger held-out catalog with semantic retrieval and live-model task evaluation while preserving the same source-obligation and evidence gates.

This report contains no secrets, credentials, private repository content, personal data, or confidential material.
