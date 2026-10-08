# Applied core-memory guidance — ToolRecall recovery

**Repository guidance snapshot:** `nicholas-ruest/core-memory@515c6445e28e75d33ebe75f34ea3e8f3a31a2361`  
**Status:** `REPOSITORY_GUIDANCE_APPLIED`  
**Live memory:** `LIVE_MEMORY_RETRIEVED = false` — no authorized live memory/embedding retrieval namespace was exposed in this runtime.

Private source text is not reproduced. This receipt records only public-safe engineering decisions applied to ToolRecall.

| Source | Applied decision | Demonstration |
|---|---|---|
| `AGENTS.md` / `CLAUDE.md` | Use Ruflo as coordination/provenance only; retain executor-produced code/tests/evidence as implementation truth | Ruflo federation identity receipt plus immutable GitHub commits and CI evidence |
| `AGENTS.md` | Follow Recall→Inspect→Route→Plan→Execute→Test→Validate→Benchmark→Receipt→Handoff→Publish | research/build reports mirror the lifecycle and refuse completion without publication readback |
| `AGENTS.md` | One writer per worktree; only integration owner changes shared files | single recovery branch, sequential asset/README commits, one integration PR |
| `AGENTS.md` | Typed boundaries, boundary validation, focused files, safe paths, no secrets | existing domain/application ports, CLI validation, bounded OpenAI sidecar, public-safe reports |
| `docs/architecture-overview.md` | Keep core/domain independent of adapters and make dependency direction explicit | README workspace map and `docs/architecture.md` link domain→application→adapters→CLI |
| `docs/adr/0002-storage-engine-anti-corruption-adapter.md` | Keep RuVector behind an anti-corruption adapter and fail loudly on integrity mismatch | `toolrecall-adapter-ruvnet`, readback assertion, RVF tamper tests |
| `docs/adr/0004-write-governance-capability-gate.md` | Treat coordination claims as leases, never authorization; fail closed | receipts and evaluator bundles carry `authority: none`; no auto-publish/deploy/promote |
| `docs/adr/0008-consolidation-benchmarking-promotion-gate.md` | Compare against baselines and block promotion on regression or insufficient evidence | global-top-k/source-guard/full-catalog table; explicit six-case limitation and REVISE decision |
| `docs/ddd/memory-store.md` | Append auditable evidence and verify readback through authorized interfaces | RuVector insert/search/get path plus exact receipt/witness evidence |
| `docs/ddd/governance.md` | Separate recommendation from authority and preserve human graduation | Darwin/Flywheel advisory-only; owner decision remains REVISE/GRADUATE/DISCARD |
| `docs/ddd/consolidation-benchmarking.md` | Freeze dataset, seed, environment, baseline, and candidate before claims | committed corpus/evidence, immutable implementation SHA, exact-main CI link |

## Validation revisit

- **Compliant:** repository structure remains 25 ADRs, 12 DDD documents, seven packages, typed ports/adapters, immutable evidence, explicit baselines, retained failed attempts, and human promotion authority.
- **Compliant recovery:** README uses versioned assets, self-contained animated/static-readable diagrams, accessible labels, reduced-motion CSS, attribution, working local paths, and no live-metric implication.
- **Deviation:** no live core-memory retrieval was possible; repository guidance was still read and applied. No claim of embedding search is made.
- **Open gate:** public Gist publication/readback remains independent of repository delivery.
