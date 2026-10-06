# ADR-0020: Frozen evaluation corpus

## Context
Changing examples after seeing results creates benchmark leakage.
## Alternatives
Ad hoc prompts; live marketplace data; versioned synthetic mechanism corpus.
## Decision
Commit eight tools, six queries, labels, source obligations, seed, and environment metadata before evaluation.
## Tradeoffs
External validity is limited, but causal interpretation is stronger.
## Consequences
New cases require a new corpus version and cannot rewrite earlier evidence.
## Validation
The evaluator hashes the corpus and records the digest with every result.
