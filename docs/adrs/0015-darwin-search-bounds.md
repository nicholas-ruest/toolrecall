# ADR-0015: Darwin search bounds

## Context
Unbounded parameter search can overfit the six-case corpus.
## Alternatives
Free-form evolution; fixed policy; bounded numeric candidates.
## Decision
Darwin may compare only `top_k` values 1–4 and the three frozen strategies.
## Tradeoffs
The search cannot invent a better retriever, but its outputs remain comparable.
## Consequences
Genome bounds are versioned and part of the evidence digest.
## Validation
The candidate rejects values outside the genome and reports every evaluated configuration.
