# Q-M1f.269 — Division by zero is a contradiction of arithmetic itself, in every base

`std.math.arith`, which every base imports, declares the flag `(division-by-zero REQUEST)` and the rule
`division-by-zero-is-false`, which derives `(false)` from it. A module that divides derives the flag, naming the
request or the call as it was asserted, and the contradiction is the next step, so a trace shows the flag and then
`(false)`. The rule has no parameter and is no base's own: decimal, binary, `std.math.int`, `std.math.rat` and
`std.math.num9` raise the one flag. A program that divides by zero is dead: R is empty, and the request is what
`ein solve` names as the reason.

**Why.** No number is the quotient, whatever the base, so the contradiction belongs to arithmetic and not to a
representation or to the shared rules. A program that divides by zero asks for what does not exist, and no other
answer of it is wanted.

**Held by.** `ein-infer/tests/arith_theory.rs`, `binary_theory.rs` and `num9_theory.rs`:
`a_division_by_zero_is_flagged_and_false`; `int_theory.rs`:
`a_division_by_zero_is_the_flag_under_every_convention`; `rat_theory.rs`:
`a_zero_denominator_is_flagged_and_false`; `tests/stdlib/math/decimal-arith/20_div_by_zero.ein`,
`tests/stdlib/math/binary-arith/08_div_by_zero.ein`.
