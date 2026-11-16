# Q-M1d.2 — An obligation is a rule that concludes `(open ?R)`

A requirement that something exist is stated as a rule, not as a kernel object or a stored fact. An obligation rule's
whole `:assert` is `(open ?R)`, and its `:match` is the requirement's guard, as a rule's is: `std.algebra`'s
`total-owed` matches `(absent (and (?isa ?b ?B) (?R ?a ?b)))`, *no witness exists*. The loader routes such a rule into
the program's obligations. It derives nothing and is not in the saturation agenda; one pass over each quiescent KB
reads it, and an instance it matches is a debt of that state. Firing is being undischarged: the rule matches while
the witness is missing and stops matching once it is there. The debt is state of the node, beside its dead bit, and
never a fact. The facts that would pay it are not stored either: they are read off the same guard, with its witness
step skipped.

**Why.** `(false)` can be a fact because a dead state stays dead under any addition; a debt exists to be paid by an
addition, so a stored one would survive its own payment in a fork that paid it. A rule shape adds nothing to the data
model, the `.einb` container or the copy-on-write layer, and a requirement stated once, in its guard, cannot disagree
with a second statement of it.

**Held by.** `ein_infer::obligations`, the pass over a quiescent KB; `ein-infer/tests/obligation_reports.rs`:
`an_obligation_rule_never_reaches_the_firing_stream`, `each_obligation_rule_owes_exactly_what_its_fixture_states`,
`a_state_that_owes_is_open_and_not_a_solution`; the obligation rules of `stdlib/algebra.ein` and `stdlib/slots.ein`.
