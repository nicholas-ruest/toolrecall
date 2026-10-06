# ADR-0021: Coverage-first metrics

## Context
Average target recall can hide complete loss of a required source.
## Alternatives
Precision only; aggregate recall; required-source coverage as the primary gate.
## Decision
Reject any candidate with a missing obligated source, then compare target recall and exposure.
## Tradeoffs
Policy obligations must be curated and may over-constrain some queries.
## Consequences
Efficiency cannot compensate for an invisible required source.
## Validation
Benchmark exit status is non-zero when coverage is below 100%.
