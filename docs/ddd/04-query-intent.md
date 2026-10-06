# Query Intent model

`QueryIntent` is a value object containing bounded text and a set of required source IDs. The source set expresses registry or workflow knowledge; it is not inferred by a model.

Invariants:

- query text is valid UTF-8 and within the configured byte limit;
- source obligations are unique;
- each obligated source must exist in the snapshot;
- ordering has no semantic effect.

Domain events are `QueryAccepted` and `QueryRejected`. The CLI adapter owns deserialization; the domain owns semantic validation. Tests cover empty/oversized input and absent sources.
