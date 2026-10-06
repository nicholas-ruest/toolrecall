# DDD overview

ToolRecall has four bounded contexts: Catalog Intake, Recall Decision, Evidence, and Evaluation. OpenAI and MCP are anti-corruption adapters; RuVector/RVF form the authorized evidence context; MetaHarness is an advisory downstream evaluator.

The aggregate root is `CatalogSnapshot`. A `SelectionPolicy` and `QueryIntent` produce a `SelectionReceipt`. The receipt cannot be successful until its selected tools validate and evidence persistence reads back. The central invariant is: **a required source present in the snapshot must be represented in a successful source-guard selection**.

Detailed domain views are indexed in [`docs/ddd/README.md`](ddd/README.md).
