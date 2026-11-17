# Q2 — Ein hands no problem to an external solver

The engine decides every program itself: saturation to a fixpoint, the hypotheses read off the program, and the
routes over the commitments that find the region R. No part of a program is handed to a SAT, SMT or constraint
solver, there is no back-end to choose, and none is planned. Arithmetic is a theory of the stdlib, computed by the
same saturation.

**Why.** An answer the engine gives is one it can explain, fact by fact, from the program's own rules; a sub-problem
decided elsewhere would come back as a verdict with no derivation behind it. Where a graph-native engine is weakest,
arithmetic-dominated constraints, large combinatorial search and global cardinality, is a statement of Ein's scope,
not a trigger for a solver call.

**Held by.** The workspace manifest, `ein.rs/Cargo.toml`, whose dependencies name no solver;
[01-architecture.md](../../ein/ein.rs/01-architecture.md#2-one-ein-solve-from-file-to-read-out), the path from a `.ein` file to its answer. Arithmetic as a
theory: [Q-M1e2.24](../language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md).
