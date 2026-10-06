# ADR-0018: Workspace boundaries

## Context
One crate would couple the policy kernel to Python, MCP, storage, and evaluation lifecycles.
## Alternatives
Single crate; one crate per type; seven packages aligned to ownership.
## Decision
Use domain, application, three adapter, evaluation, and CLI packages with inward dependency direction.
## Tradeoffs
More manifests and compile units increase maintenance.
## Consequences
No crate is empty; each exposes exercised behavior owned by one bounded context.
## Validation
`cargo tree` and workspace tests check dependency direction and callers.
