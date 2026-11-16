# Q-M1f.182 — A zero denominator is a division by zero

A request with an operand whose denominator is zero, however the zero is written, and a `rat-div` by a rational
that is zero, derive `std.math.arith`'s flag, `(division-by-zero REQUEST)`, the request as written, and its rule
derives `(false)`. Each operand's denominator is compared with zero as it is looked up. A `let` that names a
rational over zero, and that no request uses, is not flagged.

**Why.** A rational over zero is a division by zero written down, and one flag with one rule to `(false)` is one
thing for a program's rule to match in the whole calculator
([Q-M1f.269](../Q-M1f.269-division-by-zero-is-a-contradiction.md)).

**Held by.** `ein-infer/tests/rat_theory.rs`: `a_zero_denominator_is_flagged_and_false`;
`tests/stdlib/math/rat/10_div_by_zero_rational.ein` to `13_zero_denominator_one_operand.ein`.
