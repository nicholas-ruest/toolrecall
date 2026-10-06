# Operations and failure behavior

Failures are classified as input rejection, upstream incompatibility, timeout/cancellation, persistence/readback failure, witness failure, test failure, or publication blocker. They are never collapsed into a passing receipt.

The CLI returns non-zero and structured stderr without secrets. Child processes are killed on timeout. Paths are explicit arguments, not shell strings. Logs identify versions and digests but exclude credentials and raw private catalogs.

Operational ownership is split: application owns error semantics, adapters own cleanup, CI owns repeatable gates, and the release owner controls merge/publication. Recovery appends evidence instead of overwriting a failed run.
