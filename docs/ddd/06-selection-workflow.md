# Selection application workflow

The application service performs: load catalog → validate snapshot → select → validate selected tools → persist evidence → read back → witness → return receipt.

Ports:

- `CatalogPort` supplies raw upstream tools;
- `ToolValidationPort` checks the interoperability boundary;
- `EvidencePort` appends and reads back outcomes;
- `WitnessPort` binds immutable digests.

No adapter can skip a prior stage. A failed stage emits an error and no successful receipt. Cancellation propagates to owned child processes. The application crate owns sequencing tests; the CLI owns presentation only.
