# ADR-0002: Exact catalog snapshot

## Context
Comparisons are invalid when strategies see different catalogs or mutable ordering.
## Alternatives
Use live enumeration per strategy; timestamp a catalog; hash a normalized snapshot.
## Decision
Normalize, sort by stable ID, serialize canonically, and bind a SHA-256 digest to every receipt.
## Tradeoffs
Canonicalization rejects ambiguous input and adds preprocessing.
## Consequences
Every claim is replayable against one snapshot rather than `HEAD` or an evolving registry.
## Validation
Digest stability and field-sensitivity tests cover ordering and mutation.
