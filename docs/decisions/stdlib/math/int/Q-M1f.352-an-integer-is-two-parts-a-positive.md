# Q-M1f.352 — An integer is two parts, a positive and a negative

`std.math.int` writes an integer `(int P N)`, P and N two natural numbers of `std.math.decimal-arith`, and its value
is P less N: 7 is `(int (n z 7) z)`, -7 is `(int z (n z 7))`, zero is `(int z z)`, and `(int (n z 5) (n z 3))` is 2,
written otherwise. Every integer has many spellings and one normal form, one part `z` and the other with no high
zero. An operand is read by value, whichever spelling it is written in, and every answer is in normal form. An
integer is written with no sign: a sum, a difference, a product and a comparison look at none, and a division alone
reads its operands' signs off their normal forms, to round as its convention says
([Q-M1f.179](Q-M1f.179-integer-division-names-its-convention.md)).

**Why.** Ein has no numbers ([Q-M1e2.24](../../../language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)), and a
theory over the natural numbers needs no case on a sign when an integer is a difference of two of them: a sum, a
difference and a product are each computed on the parts by the natural numbers' own operations
([Q-M1f.176](Q-M1f.176-a-product-of-integers-is-computed-on-the-parts.md)), and an operand need not be simplified
first.

**Held by.** `ein-infer/tests/int_theory.rs`: `every_pair_of_small_parts_is_right`, `an_operand_is_read_by_value`,
`simplification_and_negation_are_right`; `tests/stdlib/math/int/01_simplify.ein` and `07_unsimplified.ein`. The
page: [`04-stdlib/11-integers-rationals-gcd.md`](../../../../ein/user/04-reference/04-stdlib/11-integers-rationals-gcd.md#using-stdmathint--integers), `std.math.int`.
