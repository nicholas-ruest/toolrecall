# MCP integration context

The MCP adapter is an anti-corruption layer from `ToolDescriptor` to official `rmcp::model::Tool`. It owns name/description/schema translation and reports typed validation errors.

The adapter does not run an MCP server, discover tools, or authorize execution. Its invariant is that every tool exposed by a successful receipt can be represented by the pinned official SDK.

Contract tests instantiate valid upstream types and reject malformed schemas. The domain remains independent of `rmcp`, allowing SDK upgrades to be reviewed at one boundary.
