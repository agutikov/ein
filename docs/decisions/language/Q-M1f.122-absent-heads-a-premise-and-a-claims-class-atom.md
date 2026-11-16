# Q-M1f.122 — `absent` heads a premise and a claim's class atom, and nothing else

`absent` is negation as failure, a test a premise or a goal makes of a state, `(absent <premise>)`. It heads no
fact, value or conclusion: `(absent k)`, `(q (absent k))`, `(not (absent k))` and `:assert (absent (p ?x))` are
parse errors at the `absent`, with a note pointing at the stored negation, `(not …)`. The one other place it heads
a form is a claim's class atom: a claim lists a query's projection classes, and the class of a goal's
`(absent (q ?y))` is written `(absent (q a))`, as in `(model (absent (q a)) (p a))`, which is no fact and no value.

**Why.** No state holds a fact of `absent`, so a fact, a value or a conclusion headed by it states nothing a
premise could read; written where a negation was meant, it would leave a state alive beside the fact it meant to
deny. A class over an `absent` is what a goal's projection says a state does not hold, and a claim must be able to
state it.

**Held by.** `ein-ir/src/ein.y`'s `ClaimFact` and `ClaimAtom`, the one place outside a premise that takes
`absent`; `ein-cli/tests/value_rule.rs`: `a_form_that_is_no_value_is_refused_on_every_command` and
`what_a_state_could_hold_loads`, whose `absent-in-a-claims-class` program loads.
