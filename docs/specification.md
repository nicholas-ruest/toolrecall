# Specification

## Problem

Deferred tool search can optimize global relevance while silently omitting an entire tool source. The failure appears before planning: the model cannot select a capability it never sees.

## Input

`QueryRequest` contains a bounded UTF-8 query, `required_sources`, a `top_k` policy, a strategy, a sidecar timeout, and a schema version. The catalog adapter returns typed `ToolDescriptor` values with stable IDs, source IDs, descriptions, and JSON Schemas.

## Output

`SelectionReceipt` records strategy, ordered selected IDs, missing sources before and after repair, catalog digest, decision digest, policy version, and `authority: none`. The CLI augments it with MCP validation, RuVector readback, and an RVF witness.

## Acceptance criteria

1. Global top-k, source-guard, and full-catalog consume the identical catalog snapshot.
2. Source-guard exposes at least one tool from every required source when such a tool exists.
3. A required source absent from the catalog fails loudly; it is never synthesized.
4. OpenAI and MCP adapters exercise upstream types, not local lookalikes.
5. RuVector writes are read back before success is reported.
6. Every receipt binds the exact catalog digest and policy version.
7. Timeouts, cancellation, malformed schemas, and persistence failures return typed errors.

## Non-goals

ToolRecall does not execute tools, decide IAM, call a model, learn automatically, or claim semantic retrieval quality. It is not a replacement for an MCP server or OpenAI tool search.

## Frozen experiment

The versioned corpus contains eight tools from four sources and six queries. The primary metric is required-source coverage. Secondary metrics are target recall and exposed catalog size. A candidate passes only if it has zero missing required sources, no invalid receipts, and does not expose the full catalog for every case.
