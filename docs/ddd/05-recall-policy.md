# Recall Policy model

`RecallPolicy` is a value object with strategy, `top_k`, schema version, and policy version. `SourceGuard` is the selected policy; `GlobalTopK` and `FullCatalog` are comparison strategies.

Invariants:

- `top_k` is between one and the catalog bound;
- all strategies share the same ranker and snapshot;
- source repair adds no more than one tool per missing obligation;
- selected IDs are unique;
- the policy cannot expand its own bounds.

The domain event `SelectionDecided` carries coverage and exposure facts, not execution authority.
