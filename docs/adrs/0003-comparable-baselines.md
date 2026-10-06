# ADR-0003: Comparable baselines

## Context
A combined policy needs honest external-only and coverage-ceiling comparisons.
## Alternatives
Compare only against top-k; use historical numbers; execute three strategies on identical inputs.
## Decision
Freeze `global-top-k`, `source-guard`, and `full-catalog` behind one API and scoring kernel.
## Tradeoffs
Full catalog is intentionally inefficient but isolates the recall/exposure frontier.
## Consequences
No candidate may alter scoring, labels, or fixtures to win.
## Validation
Evaluation asserts equal case counts and catalog digests across strategies.
