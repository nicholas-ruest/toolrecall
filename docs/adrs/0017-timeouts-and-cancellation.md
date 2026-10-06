# ADR-0017: Timeouts and cancellation

## Context
Sidecars and upstream adapters can hang or outlive a cancelled request.
## Alternatives
No timeout; shell-level timeout; Tokio process ownership with kill-on-timeout.
## Decision
The OpenAI adapter uses a policy-bounded Tokio timeout and terminates the child on expiry.
## Tradeoffs
Aggressive bounds may reject slow cold starts.
## Consequences
No success receipt may include partial sidecar output.
## Validation
A sleeping fixture must produce `CatalogTimeout` within the configured tolerance.
