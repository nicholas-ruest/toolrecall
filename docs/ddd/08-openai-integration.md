# OpenAI integration context

The OpenAI adapter invokes a pinned Python sidecar that builds real deferred `FunctionTool` objects. The sidecar emits only stable catalog data; it does not call a model or remote API.

The Rust adapter owns process lifecycle, timeout, stdout bounds, exit status, and JSON normalization. The Python file owns SDK construction. A non-zero exit, timeout, malformed output, or missing deferred marker rejects the catalog.

This sidecar boundary preserves Rust as the core while avoiding a local copy of the OpenAI SDK’s tool schema.
