# Q-M1f.181 — A rational operand is a rational written out, or a name

An operand of `std.math.rat` is a rational written out, `(rat (int P N) D)`, the numerator an integer simplified or
not and the denominator a natural number with or without high zeros, or a name that a
`(let NAME (rat (int P N) D))` states. An integer or a natural number is no rational operand: 5 is written
`(rat (int (n z 5) z) (n z 1))`.

**Why.** As for the integers ([Q-M1f.175](../int/Q-M1f.175-an-integer-operand-is-an-integer-written-out.md)): each
kind of number has one shape, and the program writes the rational it means.

**Held by.** `ein-infer/tests/rat_theory.rs`: `named_operands_work_for_every_request`,
`an_integer_is_not_a_rational_operand`; `tests/stdlib/math/rat/09_named_and_chained.ein`.
