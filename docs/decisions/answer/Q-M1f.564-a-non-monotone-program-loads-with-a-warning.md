# Q-M1f.564 — A non-monotone program loads with a warning, and nothing is claimed of its answer

A program the static check cannot prove monotone loads as any other program does: no command refuses a program for
an `absent`, whatever its rules conclude from it. Every command that answers about a commitment warns that the
program is not monotone, in the read-out's `lattice` rows, which name the sites, and in the JSON summary's
`monotonicity` block. The answer printed is the cube's, under the reading of a node as a fork of root
([Q-M1e2.2](Q-M1e2.2-a-node-is-a-fork-of-root-an-assumption-on.md)) and with no licence used. It is not claimed to
be right, sound or complete, on any page, in any comment or by any test.

No suite of the project validates what such a program answers. A flagged program may state an `:expect`, and
`ein test FILE` checks it for whoever asks, as it checks any claim; the project's tests, its gate and its CI check
none. No program under `tests/` is flagged; a flagged entry of the corpus declares no `test` run, and one whose
program states a claim declares only runs that check none; and no regression row, no line of the shape golden for
an answer and no assertion holds a flagged program to what it answers. What the project does hold of one compares a
run with a run and states no answer: that it loads and every command exits 0, that the static check flags it and
warns of it, and that every command and every mode gives the same R of it, under any job count, lever, import form,
renaming and numbering of the objects
([Q-M1e2.32](Q-M1e2.32-no-two-commands-disagree-and-the-oracle-is.md)).

**Why.** Ein reasons about monotone programs. Where a rule reads an `absent` that a commitment can still fill, a
commitment that grows can take a conclusion away or bring a dead state back to life, and what a node of such a
program holds depends on a reading Ein has not chosen. A refusal would say that such a program is an error, which
is not known; an answer given without a warning would claim a meaning Ein does not have. So the program is read,
the class is detected, the answer under the one stated reading is printed, and the reader is told plainly that it
carries no claim.

**Held by.** `ein_infer::monotone`, the static check. `ein-infer/tests/non_monotone_loads.rs`:
`a_refutation_under_an_absent_loads_flagged_and_unlicensed`, the corpus programs whose refutation rests on an
`absent` a commitment can fill, each loaded, flagged and answered by the cube with no licence.
`ein-infer/tests/explore.rs`: `every_mode_is_the_oracle`, every mode of every flagged corpus program against the
oracle's cube. `ein-corpus`'s `a_flagged_program_declares_no_run_that_checks_a_claim`;
`ein-render/tests/id_order_invariance.rs`, which banks no answer of a flagged program. The pages: [`08-monotonicity.md`](../../ein/reasoning/02-kb-evolution/08-monotonicity.md) and
[`08-read-out.md`](../../ein/user/04-reference/08-read-out.md#51-under-lattice-and-monotone-the-warning).
