# ADR-0024: Boundary safety

## Context
Queries, paths, schemas, and child-process output are attacker-controlled boundaries.
## Alternatives
Trust caller limits; sanitize after parsing; enforce explicit bounds before work.
## Decision
Bound query length, catalog size, schema bytes, timeout, stdout bytes, and accepted executable path.
## Tradeoffs
Large legitimate catalogs need configured expansion rather than implicit acceptance.
## Consequences
No shell interpolation is used; process arguments and paths are passed structurally.
## Validation
Boundary tests cover oversized query/catalog/output and invalid executable paths.
