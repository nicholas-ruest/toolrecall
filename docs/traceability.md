# Design and implementation traceability

| Acceptance criterion | Context | Crate/interface | Decision | Evidence |
|---|---|---|---|---|
| Same catalog snapshot | Tool Catalog | `CatalogSnapshot` | ADR-002 | domain digest tests |
| Required-source coverage | Recall Policy | `select_tools` | ADR-001, ADR-009 | frozen corpus benchmark |
| Missing source fails | Policy | `DomainError` | ADR-010 | invalid-policy test |
| Real OpenAI path | OpenAI Integration | `PythonCatalogAdapter` | ADR-005 | sidecar integration test |
| Real MCP path | MCP Integration | `McpValidator` | ADR-006 | adapter tests |
| RuVector readback | Outcome Memory | `RuVectorEvidence` | ADR-011 | integration test |
| RVF binding | Evidence | `RvfWitnessPort` | ADR-012 | witness verification test |
| Deterministic baselines | Evaluation | `run_benchmark` | ADR-003, ADR-020 | benchmark JSON |
| Bounded execution | Operations | timeout/cancellation | ADR-017, ADR-024 | timeout test |
| Human promotion | Governance | evaluation receipt | ADR-014, ADR-016 | Flywheel replay check |
