# Q6 — Symmetry is the program's, not the engine's

The engine does no symmetry breaking. A program that declares `House-1` to `House-5` and states `(right-of House-2
House-1)` has already chosen an orientation; the engine sees named instances in one layout and has no second
orientation to explore or canonicalise. Two states that differ by a symmetry of the puzzle are two states of R,
counted and listed as such.

**Why.** Which way the houses are numbered is the author's convention, made when the program is written, not a step
of a derivation. An engine that broke symmetries itself would answer a different question from the program's, and
R would no longer be every alive, discharged node.

**Held by.** `ein oracle`, which classifies every subset of H, and `ein-infer/tests/explore.rs`:
`every_mode_is_the_oracle`, so no route drops a state as another's mirror; the remark on numbering direction in
[06_zebra_walkthrough.md](../../ein/user/guide/06_zebra_walkthrough.md).
