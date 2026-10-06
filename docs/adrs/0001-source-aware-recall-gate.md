# ADR-0001: Source-aware recall gate

## Context
Global top-k can score well while returning no tool from an obligated source.
## Alternatives
Accept global ranking; expose the full catalog; add a post-ranking source gate.
## Decision
Run global ranking, then repair only missing required sources from the same frozen snapshot.
## Tradeoffs
Coverage improves at the cost of a slightly larger catalog and reliance on explicit obligations.
## Consequences
Source requirements become typed policy, not prompt prose; unknown sources are errors.
## Validation
`source_guard_recovers_collapsed_source` and the frozen benchmark require complete source coverage.
