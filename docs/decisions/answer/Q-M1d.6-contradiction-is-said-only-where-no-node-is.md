# Q-M1d.6 — `Contradiction` is said only where no node is alive

`--single`'s word for an empty R names which empty it is: `Contradiction` where no node is alive, `Open` where
nodes are alive and every one of them owes something it has not paid. A state that owes is never called
contradictory, and a run that stopped before it decided the word says *not decided — the run stopped first*, never
`Contradiction`.

**Why.** A debt is not a refutation: an alive node that owes may be discharged by a commitment the run did not
reach, and a node a budget left unvisited is not known to be dead. Only a node every extension of which derives
`(false)` makes R empty for a reason a core can name.

**Held by.** `ein_infer::explore::Verdict`, its four words read off R; `ein-infer/tests/explore.rs`:
`every_mode_is_the_oracle`, which holds every `--single` word to the oracle's, `Open` where some node is alive and
`Contradiction` where none is, at the default budget and at one small enough to stop most runs. The two kinds of
empty R: [solutions-and-models.md § Two kinds of empty R](../../ein/user/syntax-and-semantics/solutions-and-models.md#two-kinds-of-empty-r).
