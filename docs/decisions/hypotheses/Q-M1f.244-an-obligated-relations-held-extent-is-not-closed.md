# Q-M1f.244 — An obligated relation's held extent is not closed, whatever generates

An obligation that owes a fact of a relation, and a `(__closed__ R)` of the same relation, written or derived, are a
conflict, refused at load whichever rung builds H, beside a declared hrule too. The two lists conflict with an
obligation only where no hrule is declared, since they filter only the rungs below it: `:hypothesis-relations`
leaving out, or `:no-hypothesis` naming, a relation an obligation owes. The refusal is one line naming the
obligation, its activator, the closure and where each comes from.

**Why.** A relation is closed when no commitment extends its held extent, and an obligation is paid only by a fact
of the relation it owes. Beside an hrule that does not propose that relation, a state that owes it could never be
paid; a list beside an hrule filters nothing, so it takes nothing from the obligation.

**Held by.** `ein-cli/tests/mechanism_conflicts.rs`: `a_conflict_is_refused_on_every_command`, whose shapes include
an obligation and a derived closure beside an hrule; `ein-infer/tests/obligation_reports.rs`:
`the_closed_and_owing_corner_is_a_conflict`; `examples/ein-bugs/closed-and-owing.ein`;
[`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another) § 2.27.
