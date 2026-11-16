# Q-M1f.317 — A name no `let` states is flagged as undeclared

A request that names an operand no `let` states derives `(undeclared NAME REQUEST)`, and `(false)` from it, as a
division by zero does. The rule, `operand-undeclared`, holds the base's one `absent`, over `let`, a relation a
program writes, no rule derives and the theory closes: the premise is decided at root, and a program that imports
the theory is monotone by the static check. Binary's is `bin-operand-undeclared`, over `bin-let`.

**Why.** A name used and not declared names no number. To a monotone rule *no `let` yet* and *no `let` ever* are
one state, so only an `absent` can say it, and over a relation nothing extends it is decided once.

**Held by.** `ein-infer/tests/arith_theory.rs`: `an_undeclared_name_is_flagged_and_a_let_answers_it`,
`the_theory_writes_one_absent_over_let`; `tests/stdlib/math/decimal-arith/29_undeclared_name.ein` and
`30_declared_name.ein`.
