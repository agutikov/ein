# Q-M1f.351 — A bare variable or name is no whole conclusion

A rule's `:assert` is a fact, an `(and …)` of conclusions, a stored negation `(not …)`, `(false)`, a guard, `()` or
`(open …)`: the grammar's `Assert` and `Conclusion`. A variable or a name written alone where a conclusion goes,
`:assert ?a`, `:assert foo`, or a conjunct of an `(and …)`, is refused at the parse, on every command, at its
position, *unexpected `?a`: a variable*, with the repair that writes it as a form. A macro's body that is its own
parameter leaves one where its call is a conclusion, `(macro id (?x) ?x)` called as `:assert (id ?a)`; the loader
refuses that after expansion, as it refuses every malformed primitive a macro writes
([Q-M1f.81](Q-M1f.81-a-malformed-primitive-is-a-form-the-grammar.md)): *rule 'r': its :assert concludes ?a, which is
not a fact*. A variable stands whole in a conclusion only under `not`, `(not ?x)`
([Q-M1f.127](Q-M1f.127-a-conclusion-not-x-is-refused-where-the-program.md)).

**Why.** A conclusion is what a firing adds to a state, and a state holds facts. Written as a form, `(?r ?a)` or
`(p ?a)`, a conclusion is a fact whatever its variables are bound to. A variable alone may be bound to a name, which
no state holds, so whether a firing added anything would depend on each binding, and a name alone is never a fact.

**Held by.** `ein-ir/src/ein.y`'s `Assert` and `Conclusion`; `ein-cli/tests/cli_semantics.rs`:
`a_conclusion_no_fact_can_hold_is_one_line_in_eins_spelling`, a name and a variable as the whole conclusion and a
variable as a conjunct; `ein-cli/tests/macro_bodies.rs`: `every_malformed_primitive_is_refused_before_any_path_runs_it`,
a body that is its own parameter called with a variable and with a name.
