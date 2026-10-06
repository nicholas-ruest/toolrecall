# CORE_MEMORY_READ receipt — 2026-10-05

Canonical source: private `nicholas-ruest/core-memory`, default branch commit `515c6445e28e75d33ebe75f34ea3e8f3a31a2361`.

| File | Blob SHA |
|---|---|
| `AGENTS.md` | `4afe6f9008076140486d2a1e5424ddc6e40a0671` |
| `CLAUDE.md` | `0178167276c981ff0ba503e2e564eee8d347e3f7` |
| `docs/architecture-overview.md` | `8ba7f76674e59555dbcd5f971554884c54085944` |
| storage/ACL ADR | `9a8bdfe08dd41abc4402ab46a4ad359356815b6e` |
| benchmark-promotion ADR | `15e782267153e3d4251bf7fc196763de83b9669b` |
| trust-root/RVF ADR | `a550f0a481ca0804ceb6a4a442653cc0179dcda7` |
| DDD context map | `ddda3371b416dc7394b15957451fc3a829a9fe21` |
| DDD governance | `2c1e98630374c966699c5a808305daa6718b4cfc` |
| DDD memory store | `b00275577f9ee60b192b4aa2712632f2e2439ef7` |

## Applied decisions

- Ruflo records coordination/provenance; Rust/Node/Python executors perform actual work.
- One integration owner writes each worktree; dependencies flow inward from adapters.
- Public boundaries are typed, bounded, and fail loudly.
- RuVector is the only intelligence/persistence layer; ToolRecall owns no service database.
- Evidence is append-only and exact-source/commit bound; RVF witnesses do not imply authority.
- Darwin and Flywheel recommend only and cannot self-promote.
- Implementation, validation, merge, and publication are separate gates.
