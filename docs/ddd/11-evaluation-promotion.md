# Evaluation and promotion context

`EvaluationRun` aggregates one frozen corpus and results for all three strategies. `CandidateScore` is a value object; `ReplayBundle` is a signed evidence artifact.

Darwin may explore only frozen strategy/top-k bounds. Flywheel may sign and verify replay bundles. Neither can update repository state or the default policy.

The primary invariant is zero missing required sources. Target recall and exposure rank candidates only after that gate. Promotion is a separate human/repository workflow and requires exact-commit review.
