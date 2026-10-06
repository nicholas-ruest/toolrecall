# ADR-0008: Source identity is first-class

## Context
Coverage cannot be measured if provenance is implicit in descriptions.
## Alternatives
Infer source from names; use URLs; require a stable source ID.
## Decision
Every tool carries an opaque, non-empty `source_id`; policies refer only to those IDs.
## Tradeoffs
Registry adapters must supply identity and migrations must preserve it.
## Consequences
Source coverage becomes computable without interpreting natural language.
## Validation
Catalog validation and metrics tests exercise duplicate tools across distinct sources.
