# Q-M1d.2 — An obligation is a rule that concludes `(open ?R)`

A requirement that something exist is stated as a rule, not as a construct of the kernel. An obligation rule's
whole `:assert` is `(open ?R)` or `(open R)`, and its `:match` is the requirement's guard, as a rule's is:
`std.algebra`'s `total-owed` matches `(absent (and (?isa ?b ?B) (?R ?a ?b)))`, *no witness exists*. The loader
routes such a rule into the program's obligations. It is not in the saturation agenda; one pass over each state's
fixpoint reads it, and an instance it matches is a debt of that state. Firing is being undischarged: the rule
matches while the witness is missing and stops matching once it is there. A state that owes an instance over `R`
holds the fact `(open R)`, once a relation, in the part of the state no fork inherits
([Q-M1f.566](Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md)); the count of instances is kept beside it.
The facts that would pay a debt are read off the same guard, with its witness step skipped, and an obligation
activated at root writes them as hypotheses, `(open F)`
([Q-M1f.570](Q-M1f.570-h-is-the-union-of-the-hypotheses-root-holds.md)).

**Why.** `(false)` can be a fact every fork inherits because a dead state stays dead under any addition; a debt
exists to be paid by an addition, so an inherited one would survive its own payment in a fork that paid it, and the
fact is the owing state's own instead, inherited by no fork. A rule shape adds nothing to the language beyond the word `open`,
and a requirement stated once, in its guard, cannot disagree with a second statement of it.

**Held by.** `ein_infer::obligations`, the pass that reads an obligation at a fixpoint; `ein_infer::open`, which
writes `(open R)` and the payments; `ein-infer/tests/obligation_reports.rs`:
`an_obligation_rule_never_reaches_the_firing_stream`, `each_obligation_rule_owes_exactly_what_its_fixture_states`,
`a_state_that_owes_is_open_and_not_a_solution`; `ein-infer/tests/open_facts.rs`:
`the_seats_hold_what_they_owe_where_they_owe_it`; the obligation rules of `stdlib/algebra.ein` and
`stdlib/slots.ein`.
