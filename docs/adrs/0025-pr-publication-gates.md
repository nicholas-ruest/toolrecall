# ADR-0025: PR and publication gates

## Context
Evidence is not complete if implementation, validation, or public reporting bypasses review.
## Alternatives
Direct main writes; one omnibus PR; architecture, implementation, and validation PRs.
## Decision
Use meaningful staged branches, require exact-commit CI, merge only green PRs, and separately read back repository and Gist publication.
## Tradeoffs
The workflow takes longer and can end in partial failure when publication auth is unavailable.
## Consequences
Consistency receipts never substitute for public readback or test evidence.
## Validation
The final report records branch names, PR URLs, merge SHAs, Gist owner/files, and main tree inventory.
