# ADR-0016: Flywheel promotion boundary

## Context
Replayable evaluation must not silently make a learned rule canonical.
## Alternatives
Auto-promote best score; store all runs only; sign a replay bundle and stop at recommendation.
## Decision
Flywheel signs and verifies the bounded run bundle but cannot alter policy or repository state.
## Tradeoffs
Validated improvements require a separate reviewed change.
## Consequences
The replay signature supports audit, not autonomous graduation.
## Validation
The evaluator script verifies the bundle and asserts the selected candidate remains advisory.
