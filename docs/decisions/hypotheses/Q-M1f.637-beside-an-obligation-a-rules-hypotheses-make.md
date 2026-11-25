# Q-M1f.637 — Beside an obligation a rule's hypotheses make the rung `union`, and stuck is owing with an empty H

Where root activates an obligation and a rule that concludes `(open F)`, or the program, writes hypotheses beside it,
$H$ is the union of the writers
([Q-M1f.570](Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)), and the `rung` event's mode is `union`: the
counts it carries are the obligations', and where the obligations offer no candidate, $H$ is the other writers'
alone. A root is **stuck** only where it owes and no writer, whichever, gives it a candidate that survives:

$$
\operatorname{stuck}(P) \iff \operatorname{Owed}(\mathrm{root}) \neq \emptyset \ \land\ H = \emptyset
$$

The lattice is then root alone, root owes, and $R$ is empty; a stuck state is not discharged, and nothing reports it
as complete. An obligation that offers nothing, no payment formed or every one settled, makes no stuck root by
itself: a rule's candidates beside it may pay what root owes, and $R$ is what they give, as
`examples/lattice/18_two_candidates.ein` has. Apart from stuck, where root owes an instance whose payments can be
formed and every one of which it settles, $R$ is empty on a monotone program whatever another writer proposes.

**Why.** A rule's hypotheses and an obligation's payments are one $H$. A mode that named the rules alone would hide
that an obligation is activated, and a stuck read off the obligations' share alone would call a root that has
candidates to branch on one that has none.

**Held by.** `ein_infer::oblgen::Mode`, its `Union` and `Stuck`; `ein-infer/tests/obligation_rung.rs`:
`the_ladder_dispatches_by_what_the_program_declares`, `examples/zebra2.ein` and
`examples/lattice/18_two_candidates.ein` taking `union` and a root whose every payment is refuted `stuck`, and
`owing_and_unable_to_branch_is_stuck_and_says_so`;
[hypothesis generation § 4.5](../../ein/reasoning/02-kb-evolution/04-hypothesis-generation.md#45-stuck-owing-with-nothing-to-propose);
the `rung` event of [`05-events.md`](../../ein/ein.rs/05-events.md).
