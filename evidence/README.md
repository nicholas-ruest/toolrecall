# Validation evidence

All claims here bind to source commit `910bd04097d228ca2bb83511f78f4fedc2b1cffe`. Executor transcripts are named in `commands.tsv`; their paths are machine-local audit locations and are not presented as public links.

- `commands.tsv` records exact commands, exit status, result, and transcript path.
- `benchmark.json` freezes the strategy comparison.
- `vertical-slice.json` records the real OpenAI/MCP/RuVector/RVF execution.
- `evaluators.json` records successful Darwin/Flywheel reruns and preserves preceding failed attempts.
- `factory/RECEIPT.json` is checked by the pinned control-plane consistency gate. Consistency is not independent proof.

No evaluator had publication, deployment, or policy authority.
