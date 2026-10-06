# ADR-0014: Evaluator authority is none

## Context
MetaHarness evaluators can score candidates but must not become a deployment authority.
## Alternatives
Auto-merge winners; emit a recommendation; require human promotion.
## Decision
Darwin and Flywheel outputs are advisory receipts with `authority: none`.
## Tradeoffs
Promotion is slower but preserves the safety envelope.
## Consequences
No evaluator credential can publish, deploy, or change the default policy.
## Validation
Evaluation JSON exposes no mutation target and the workflow contains no promotion command.
