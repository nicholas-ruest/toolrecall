# ADR-0022: Catalog exposure cost

## Context
Full-catalog selection trivially maximizes coverage but consumes planner context.
## Alternatives
Ignore exposure; estimate tokens; count selected normalized tools.
## Decision
Record selected tool count as a deterministic exposure proxy alongside coverage.
## Tradeoffs
Count does not capture real tokenizer costs or description length.
## Consequences
A candidate cannot win by always exposing every tool.
## Validation
The gate requires source-guard exposure below full-catalog exposure on the frozen corpus.
