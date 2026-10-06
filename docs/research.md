# Research and candidate comparison — 2026-10-05

## Method

Two independent streams were rerun: frontier primary research published or revised in the preceding 60 days, and enterprise open-source repositories materially changed in the preceding 30 days. Dates, licenses, files, and immutable revisions below were inspected before selection.

## Frontier research

### Source-style collapse in executable capability retrieval

*When Tool-Backed Skill Retrieval Fails: Source-Style Collapse in Executable Capability Retrieval* (arXiv:2608.16502, 2026-08-17) isolates a failure in which global retrieval suppresses whole tool sources. Its ToolScout mechanism uses source-aware routing; the reported mixed-query coverage rises from 22.3% to 86.1%, and five collapsed sources from 1.3% to 53.9%. ToolRecall adopts the falsifiable source-obligation mechanism, not those performance claims or implementation.

### Declarative tool security

*ToolGuardian: Declarative Security for Agent-Tool Interactions* (2026-07) motivates treating tool exposure as an enforceable control-plane decision. It is complementary: ToolGuardian governs permitted interaction, while ToolRecall asks whether required sources survived recall.

### Skills as a systems layer

*A Systems Foundation for Agentic Skills* (2026-08-30) and the agentic SDLC control-plane work (2026-09-04) reinforce explicit lifecycle, version, and governance boundaries. They informed receipts and non-promotion constraints, not the retrieval kernel.

## Enterprise OSS inspection

| Project | Material date | Pinned revision/package | Inspected interface | License |
|---|---:|---|---|---|
| OpenAI Agents SDK | 2026-10-05 repository update | `openai-agents 0.23.1`; repo `d7e52c375021248973f60ccbea8e7b69bc5b16e3` | `function_tool(defer_loading=True)`, tool search example | MIT |
| MCP Rust SDK | 2026-10-05 release 3.5.1 source; compatible crate pinned at 3.4.0 | repo `79437f291b2c44053d00dcd5db969fd0cca7c887` | `rmcp::model::Tool::new` | Apache-2.0 for current contributions; repository history includes MIT transition notice |
| NVIDIA Skills | 2026-10-02 catalog regeneration | repo `0e0d506f4eb67204a62586ac5f19df3cb7ad9b1f` | live-catalog router and capability-discovery contract | CC-BY-4.0 AND Apache-2.0 |
| RuVector | inspected 2026-10-05 | `5a93328f2fceb0307c25929ed38cd7a0911fdf00`, crate `2.3.1` | `VectorDB`, insert/search/get | Apache-2.0 |
| MetaHarness | inspected 2026-10-05 | `9ce8b8dd89045c3b9a1f809ae58f3589029db4a4`; Darwin `0.10.3`, Flywheel `0.1.12` | bounded candidate CLI; replay bundle APIs | Apache-2.0 |

The OpenAI integration is a justified Python sidecar because the maintained SDK is Python. The MCP, RuVector, and RVF boundaries use real Rust crates. No dependency is present solely for branding.

NVIDIA Skills was inspected as part of the independent enterprise radar, not integrated into ToolRecall: its live-catalog routing is adjacent prior art, but adding it would not strengthen the selected vertical slice.

## Ranked composition matrix

Scores are 1–5 for problem evidence, novelty, complementary composition, bounded testability, and enterprise usefulness.

| Candidate | Evidence | Novelty | Composition | Testability | Usefulness | Total | Decision |
|---|---:|---:|---:|---:|---:|---:|---|
| ToolRecall | 5 | 5 | 5 | 5 | 5 | **25** | selected |
| ToolGuardian-to-RVF policy compiler | 4 | 3 | 5 | 4 | 5 | 21 | rejected: overlaps existing policy engines |
| Agentic SDLC receipt packager | 4 | 3 | 4 | 4 | 4 | 19 | rejected: too close to prior evidence projects |
| RRSI retry simulator | 4 | 2 | 3 | 5 | 3 | 17 | rejected: JitterMap already occupies this mechanism |

## Selected hypothesis

For mixed-source deferred catalogs, an explicit source-coverage gate can eliminate total required-source collapse while exposing materially fewer tools than a full-catalog fallback. The hypothesis fails if source-guard misses any required source, worsens target recall versus full catalog, or expands to full-catalog exposure on all cases.

## Prior art boundary

ToolRecall is not ToolScout, a semantic retriever, an authorization engine, or an MCP server. It turns the research observation into a small protocol-and-evidence layer that composes with actual deferred-tool and MCP interfaces and Ruvnet evidence systems.
