# ADR-0004: Deterministic lexical kernel

## Context
The experiment tests source collapse, not embedding quality.
## Alternatives
Hosted embeddings; local neural embeddings; transparent token overlap.
## Decision
Use lowercase alphanumeric token overlap with stable ID tie-breaking.
## Tradeoffs
Semantic recall is limited, but results are offline, reproducible, and inspectable.
## Consequences
Production retrievers can replace the kernel only behind the same frozen contract.
## Validation
Ranking and tie-break tests assert byte-for-byte stable selection.
