# Q-M1e2.2 — A node is a fork of root, an assumption on a flagged program

Every node of the lattice is built by fork from root: root's fixpoint, the node's commitment added, saturated.
On a monotone program that is the node's one state, whatever the order facts arrive in. On a program the static
check flags as not monotone, where a fact derived under an `(absent …)` can survive the arrival of the fact the
`absent` denied (`p ← ¬q, q ← ¬p`, with `q` committed), the fork is an assumption: the engine answers under it,
warns that the program is not monotone, prints the assumption in the read-out's `assumes` row, and claims nothing
about the program's solutions.

Whether such a fact disqualifies the state (a guard as a premise), whether the program should be refused, or
whether the state stands as derived, is not decided.

**Why.** Ein reasons about monotone programs, where a node's state does not depend on the path to it, and a
non-monotone program is detected, warned of and solved under one stated reading rather than answered as if it
were monotone. Choosing a semantics for programs whose state depends on the order facts arrive is a question about
that class of program as a whole, and is taken up with it, not one witness at a time. A refutation resting on an
`absent` a commitment can still fill is already refused at load.

**Held by.** `ein_infer::detector::ASSUMES`, the `assumes` row of a flagged program's read-out;
`ein-cli/tests/region_modes.rs`: `the_monotone_question_is_about_the_program`; `ein-infer/tests/explore.rs`:
`every_mode_is_the_oracle`, the oracle building every node by the same fork. The page:
[`monotonicity.md`](../../ein/reasoning/kb-evolution/monotonicity.md).
