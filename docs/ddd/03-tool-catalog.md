# Tool Catalog aggregate

`CatalogSnapshot` is the aggregate root. `ToolDescriptor` is an entity identified by stable tool ID; `SourceId`, `ToolName`, and `InputSchema` are value objects.

Invariants:

- tool IDs are unique and non-empty;
- source IDs are explicit and non-empty;
- names meet MCP constraints;
- schemas are JSON objects within size bounds;
- canonical ordering is stable-ID ascending;
- the digest changes for every meaningful field mutation.

The OpenAI adapter may propose raw tools, but only Catalog Intake can establish a snapshot. `toolrecall-domain` owns validation and digest tests; adapters cannot weaken them.
