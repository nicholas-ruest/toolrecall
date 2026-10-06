# ADR-0023: Schema and policy versioning

## Context
Receipts become ambiguous when structures or defaults change.
## Alternatives
Rely on crate version; unversioned JSON; explicit schema and policy IDs.
## Decision
Every input and receipt records a schema version and immutable policy version.
## Tradeoffs
Adapters must support migrations and reject unsupported versions.
## Consequences
Historical evidence remains interpretable after code changes.
## Validation
Deserialization tests reject unknown major versions and preserve known minor fields.
