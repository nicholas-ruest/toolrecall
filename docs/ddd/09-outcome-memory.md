# Outcome Memory context

`OutcomeRecord` is an append-only entity identified by a content-derived ID. Its vector encodes measured coverage, target recall, and exposure; metadata carries snapshot and policy digests.

RuVector is the single intelligence/persistence layer. ToolRecall owns no database schema. A write is successful only after exact-ID readback; search alone is insufficient because nearest-neighbor results can be approximate.

Production Cloud SQL placement belongs behind RuVector’s authorized adapter and is not bypassed. Integration tests use temporary RuVector storage while preserving the same port contract.
