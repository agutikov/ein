# Q-M1f.125 — A connective as a pattern's argument is refused by the loader, after expansion

A premise pattern's argument is what an object can be: a name, a variable, a pattern of an ordinary relation, or
`(not <pattern>)`, a stored negation. The grammar takes a connective there, and the loader refuses it once the
clause's macros are expanded: `and`, `or` and `neq`, and `eq`, `absent` and `false` with an argument, as a pattern's
argument in a rule, an hrule, an obligation or a goal, under `(absent …)` too, one line each, placed where the form
is written: `rule 'r': its :match writes (q (and (p ?x))), and (and (p ?x)) is a connective where a value goes — …`,
*value* being the line's word for an object.

**Why.** A macro is called as a pattern, and a call may take premises, as `std.macro`'s `forall` does, `(forall ?b
(and …) (B))`; a context-free grammar cannot tell a macro's name from a relation's, so only the expanded clause
shows whether a connective stands where an object goes. A pattern over a relation `and` matches nothing, and under
`(absent …)` would hold for every binding.

**Held by.** `ein_ir::from_ir`'s `Shapes`, the walk that holds expanded clauses to the grammar's shapes;
`ein-cli/tests/value_rule.rs`: `a_form_that_is_no_value_is_refused_on_every_command` (`and-pattern-argument`,
`or-pattern-argument`, `neq-pattern-argument` and their hrule, obligation and goal sites) and
`what_a_state_could_hold_loads` (`forall-called-with-premises`, `a-stored-negation-as-a-pattern-argument`).
