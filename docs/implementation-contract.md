# Frozen implementation contract

The experiment freezes the following contract before evaluation:

```text
select(catalog_snapshot, query, policy, strategy) -> SelectionReceipt
```

`catalog_snapshot` is ordered by stable tool ID before hashing. Tokenization lowercases ASCII alphanumerics and splits on all other characters. Ranking is descending overlap, then ascending tool ID.

## Strategies

- `global-top-k`: return the highest-ranked `k` tools.
- `source-guard`: start with global top-k, then add the highest-ranked tool for each missing required source.
- `full-catalog`: expose every tool; this is the coverage ceiling and exposure-cost baseline.

All strategies use the same scoring kernel. A strategy may not change the corpus, target labels, source obligations, policy bounds, or digest algorithm.

## Receipt invariants

- selected IDs are unique and stable;
- unknown required sources are errors;
- the digest covers every normalized field;
- `authority` is always `none`;
- a persisted receipt is successful only after readback;
- evaluation cannot update the default policy automatically.
