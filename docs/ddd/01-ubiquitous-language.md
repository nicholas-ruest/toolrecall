# Ubiquitous language

| Term | Meaning | Owner |
|---|---|---|
| Catalog snapshot | Canonical immutable set of normalized tools | Catalog Intake |
| Source | Stable origin/administrative namespace of tools | Catalog Intake |
| Source obligation | Source that policy requires to remain visible | Recall Decision |
| Recall collapse | Required source absent from selected tools despite existing in snapshot | Recall Decision |
| Repair | Add the best tool for a missing obligated source | Recall Decision |
| Exposure | Count of tools shown downstream | Evaluation |
| Receipt | Deterministic decision plus immutable digests | Evidence |
| Witness | RVF cryptographic link between snapshot and decision | Evidence |
| Promotion | Human-authorized change of default policy | Governance |

These terms appear in public types, ADRs, CLI JSON, and tests. “Recall” never means execution authorization.
