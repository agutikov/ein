# Q-M1f.210 — A malformed number is its representation's fault

Whether a term is a well-formed number is asked of the module that defines its representation: `std.math.decimal`
and `std.math.binary` check a numeral
([Q-M1f.273](decimal/Q-M1f.273-the-decimal-numeral-is-std-math-decimal-s-which.md),
[Q-M1f.295](binary/Q-M1f.295-the-binary-numeral-is-std-math-binary-s-which.md)), `std.math.int` an integer's own
shape and `std.math.rat` a rational's ([Q-M1f.276](Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)).
An operation that takes a number asks that module's check, and computes nothing of how the number is spelt
([Q-M1f.404](Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)). The walk that asks it of each place
knows no base and is `std.math.positional`'s; what a numeral of the base is, the check's relations, its label and
its flags, are the representation's. What is wrong with an operand, a
digit where a number goes or a name no `let` states, is the fault of the module that reads the operand, flagged on
the request ([Q-M1f.206](decimal/Q-M1f.206-an-operands-fault-travels-with-its-tag.md)).

**Why.** A numeral that holds a letter is no number, as on paper: what a number is belongs to its representation,
and a module that computes with numbers need not know how each is spelt.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_malformed_request_is_flagged_once_and_never_answered`, every flag
`std.math.decimal`'s and none the arithmetic's; `int_theory.rs`:
`a_malformed_integer_and_an_operands_fault_are_flagged`; `rat_theory.rs`: `a_malformed_rational_is_flagged`; the
closing rules' lists, which name each check among its module's rules (`the_closing_rule_names_every_rule_and_closes_every_relation`).
