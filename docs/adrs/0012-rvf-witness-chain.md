# ADR-0012: RVF witness chain

## Context
A JSON receipt alone does not prove catalog-to-decision linkage.
## Alternatives
Unsigned logs; one digest; RVF chained witnesses.
## Decision
Chain the catalog digest and decision digest with `rvf-crypto`, returning the terminal witness.
## Tradeoffs
The witness proves integrity and sequence, not actor authorization.
## Consequences
Consumers can verify evidence without granting ToolRecall execution authority.
## Validation
Witness tests reproduce the chain and fail after digest mutation.
