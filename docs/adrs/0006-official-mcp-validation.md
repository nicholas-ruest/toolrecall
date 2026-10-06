# ADR-0006: Official MCP validation

## Context
Locally invented tool structs can drift from the interoperability boundary.
## Alternatives
JSON Schema only; duplicate MCP types; instantiate official `rmcp::model::Tool` values.
## Decision
Validate every selected tool through `rmcp 3.4.0` before persistence.
## Tradeoffs
The workspace inherits the SDK’s release cadence and license transition.
## Consequences
Invalid names or schemas fail the vertical slice before a success receipt.
## Validation
Adapter tests cover valid tools and rejected malformed schemas.
