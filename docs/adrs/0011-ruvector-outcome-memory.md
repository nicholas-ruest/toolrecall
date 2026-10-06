# ADR-0011: RuVector outcome memory

## Context
Validated outcomes must be queryable without creating a parallel intelligence store.
## Alternatives
SQLite; JSON files; RuVector behind an application port.
## Decision
Use `ruvector-core::VectorDB` to append the decision vector and read it back by exact ID.
## Tradeoffs
Native dependencies and vector encoding add build weight.
## Consequences
The service owns no database; production persistence remains RuVector/Cloud SQL behind authorized ACLs.
## Validation
The integration test inserts, searches, and exact-reads a receipt before success.
