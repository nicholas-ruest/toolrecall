# ADR-0005: OpenAI SDK sidecar

## Context
The maintained OpenAI Agents SDK is Python while ToolRecall core is Rust.
## Alternatives
Reimplement its schema; embed Python; use a bounded JSON sidecar.
## Decision
Spawn a pinned Python process that constructs real `@function_tool(defer_loading=True)` objects and emits typed JSON.
## Tradeoffs
Process startup and Python packaging are added, but SDK compatibility stays upstream-owned.
## Consequences
The adapter performs no model/network call and is killed on timeout.
## Validation
An integration test asserts deferred flags, names, schemas, timeout, and malformed-output handling.
