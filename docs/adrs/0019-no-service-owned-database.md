# ADR-0019: No service-owned database

## Context
Creating local persistence would bypass Nick’s RuVector single-intelligence-layer rule.
## Alternatives
SQLite; Postgres tables owned by ToolRecall; RuVector port only.
## Decision
ToolRecall owns no schema or database; the adapter targets RuVector and production Cloud SQL remains behind it.
## Tradeoffs
Offline tests use temporary RuVector storage rather than a simpler fixture DB.
## Consequences
Deployment cannot bypass ACL or evidence policy with a private store.
## Validation
Dependency review and repository search reject SQL clients and migration files.
