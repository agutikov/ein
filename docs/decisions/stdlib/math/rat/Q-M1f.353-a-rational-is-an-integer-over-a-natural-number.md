# Q-M1f.353 — A rational is an integer over a natural number

`std.math.rat` writes a rational `(rat Z D)`: Z, the numerator, an integer of `std.math.int`, `(int P N)`, and D,
the denominator, a natural number of `std.math.decimal-arith` that is not zero. 1/2 is `(rat (int (n z 1) z) (n z
2))`, -3/4 is `(rat (int z (n z 3)) (n z 4))`, zero is `(int z z)` over any denominator. The sign is the
numerator's alone. A denominator that is zero is a division by zero
([Q-M1f.182](Q-M1f.182-a-zero-denominator-is-a-division-by-zero.md)); a digit or a named constant where the
numerator or the denominator goes is a malformed rational, flagged where it is written
([Q-M1f.276](../Q-M1f.276-an-integers-and-a-rationals-own-shape-is-checked.md)). An integer or a natural number is
no rational operand, and is answered by silence
([Q-M1f.277](../Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)).

**Why.** A rational with one place for its sign has one spelling of each sign: the numerator carries it as an
integer does, and the denominator, a natural number, has none to carry. The numerators' arithmetic is then
`std.math.int`'s and the denominators' `std.math.decimal-arith`'s, the two theories below it.

**Held by.** `ein-infer/tests/rat_theory.rs`: `every_pair_of_small_fractions_is_right`,
`an_integer_is_not_a_rational_operand`, `a_malformed_rational_is_flagged_as_written`,
`a_zero_denominator_is_flagged_and_false`; `tests/stdlib/math/rat/24_digit_as_the_numerator.ein` and
`27_digit_as_the_denominator.ein`. The page: [`stdlib.md`](../../../../ein/user/reference/stdlib.md),
`std.math.rat`.
