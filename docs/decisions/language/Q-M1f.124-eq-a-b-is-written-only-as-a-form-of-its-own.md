# Q-M1f.124 — `(eq a b)` is written only as a form of its own, never inside another

`(eq a b)` and `(not (eq a b))` are written as a fact, a premise, a conclusion and a claim's fact of their own, each
checked and none stored. Inside another form they are refused. In a fact, `(q (eq k j))` or `(q (not (eq k j)))`, in
a fact's `:using ((eq a a))`, in a trace step's `:derives (eq a b)`, as a conclusion's or a guard's argument and in
a claim's `(p (eq a b))`, the grammar refuses them at the `eq`, with the note *eq is computed and never stored, so
(eq a b) is no value: write it as a premise or a conclusion of its own, where it is checked, and never inside
another fact or pattern*, *value* being the note's word for an object. As a premise pattern's argument, `(q (eq ?a
?b))`, written or left by a macro's expansion, the loader refuses them
([Q-M1f.125](Q-M1f.125-a-connective-as-a-patterns-argument-is-refused.md)). `neq` is refused in each of those places
too.

**Why.** No state stores an equality, so `(eq a b)` inside a fact is no object, no fact a state could hold, and a
pattern matching it by shape would derive a conclusion from two names that are not equal. A `:using` list names
facts a derivation rested on and a trace step derives a fact; neither can be an equality.

**Held by.** `ein-ir/src/ein.y`'s `EqFact` and `Negated`, where the forms of their own are taken;
`ein-cli/tests/value_rule.rs`: `a_form_that_is_no_value_is_refused_on_every_command` (its `eq-` shapes, `eq-using`
among them) and `what_a_state_could_hold_loads` (`an-equality-as-a-fact`, `a-negated-equality-as-a-fact`, as a
premise and as a conclusion).
