# ADR-0009: Bounded repair

## Context
Coverage repair could degenerate into full-catalog exposure.
## Alternatives
Expose all tools; retry with larger k; add one best tool per missing source.
## Decision
Add at most one highest-ranked tool for each required source missing from the initial top-k.
## Tradeoffs
One representative may not satisfy multi-capability queries from the same source.
## Consequences
Exposure growth is bounded by the number of missing obligations.
## Validation
Tests assert `selected.len() <= top_k + required_sources.len()` and unique IDs.
