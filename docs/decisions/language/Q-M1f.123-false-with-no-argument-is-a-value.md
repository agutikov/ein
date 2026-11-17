# Q-M1f.123 — `(false)` with no argument is an object

`false` takes no argument anywhere: a fact, an argument, a premise, a goal and a conclusion write it only as
`(false)`, and `(false k)` is a parse error at the argument, with the note *false takes no argument: (false) says
that the state is impossible, and names nothing*. Written so, `(false)` is an object wherever an object goes: `(q
(false))` and `(not (false))` load.

**Why.** An object is an atom or a fact a state could hold
([Q-M1f.113](Q-M1f.113-a-value-is-a-name-or-a-fact-a-state-could-hold.md)), and `(false)` is such a fact: a dead
state holds it. An argument names nothing the state's impossibility depends on.

**Held by.** `ein-cli/tests/value_rule.rs`: `what_a_state_could_hold_loads`, whose `false-alone-as-a-value` and
`false-alone-negated` programs load, and `a_form_that_is_no_value_is_refused_on_every_command` for `false` with an
argument in every place.
