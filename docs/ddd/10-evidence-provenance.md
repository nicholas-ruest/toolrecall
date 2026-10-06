# Evidence and provenance context

`SelectionReceipt`, `OutcomeRecord`, and `Witness` are immutable value objects. The RVF chain binds snapshot digest → decision digest → terminal witness.

Invariants:

- receipts declare `authority: none`;
- evidence is append-only;
- exact versions and corpus digest are present;
- mutation invalidates witness verification;
- test and benchmark claims name the exact commit.

Evidence proves what was evaluated, not that a tool is safe or authorized. The Ruvnet adapter owns persistence and witness construction; publication reads this context but cannot rewrite it.
