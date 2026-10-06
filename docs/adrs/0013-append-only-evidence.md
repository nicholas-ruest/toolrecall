# ADR-0013: Append-only evidence

## Context
Optimization history becomes misleading when prior failures are overwritten.
## Alternatives
Mutable latest row; event log; append-only RuVector entries with immutable IDs.
## Decision
Write each evaluated decision under a new content-derived ID; never update existing evidence.
## Tradeoffs
Storage grows and correction requires compensating records.
## Consequences
Audits retain failed and superseded candidates.
## Validation
Adapter tests assert duplicate IDs are detected rather than silently replaced.
