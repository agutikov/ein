# Q-M1f.208 — Two guards say that an operand is a name

`operand-undeclared` says that an operand is a name by two guards on a variable nothing else binds,
`(neq ?v z) (eq ?v ?x)`, in that order: the first ranges `?v` over the program's names but `z`, and the second
keeps the one that is the operand, so a number, which is a fact, never passes. Ten more guards leave the digits
out, so a digit as an operand has one flag, `malformed`. It costs a comparison a name of the program, for each
operand of each request.

**Why.** No pattern says *not of the shape `(n P D)`*, and the `absent` over `let` holds of a number too, so the
rule says *a name* by guards.

**Held by.** `stdlib/math/decimal-arith.ein`'s `operand-undeclared`; `ein-infer/tests/arith_theory.rs`:
`an_undeclared_name_is_flagged_and_a_let_answers_it`, `the_theory_writes_one_absent_over_let`;
`ein-infer/tests/arith_batch.rs`, which pins the cost.
