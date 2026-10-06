# ADR-0010: Fail on unknown source

## Context
A policy may require a source absent from the supplied snapshot.
## Alternatives
Ignore it; synthesize a placeholder; return a typed error.
## Decision
Abort selection with `RequiredSourceMissing` before emitting a receipt.
## Tradeoffs
Availability is lower than silent fallback, while safety and diagnosis improve.
## Consequences
Adapters must distinguish registry incompleteness from zero-scoring tools.
## Validation
`invalid_policy_is_rejected` covers an absent required source.
