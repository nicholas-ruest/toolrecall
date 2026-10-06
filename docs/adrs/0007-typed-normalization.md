# ADR-0007: Typed normalization

## Context
Catalog payloads are untrusted and vary across SDKs.
## Alternatives
Pass raw JSON; prompt-normalize; deserialize into a strict boundary model.
## Decision
Convert upstream values into `ToolDescriptor` with non-empty stable IDs, source IDs, descriptions, and object schemas.
## Tradeoffs
Unknown fields are discarded and adapters must evolve explicitly.
## Consequences
The domain never depends on Python or MCP serialization details.
## Validation
Boundary tests reject duplicates, blank identifiers, and non-object schemas.
