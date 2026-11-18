# Q-M1f.276 — An integer's and a rational's own shape is checked by its own module

Each part of `(int P N)` is a natural number: a digit or a named constant where a part goes, `(int 7 z)` or
`(int price z)`, is flagged `(not-integer INTEGER THING)` by `std.math.int`. A digit or a named constant where a
rational's numerator or denominator goes is flagged `(not-rational RATIONAL THING)` by `std.math.rat`. Each is read
off the module's check, `(int-check INTEGER)` or `(rat-check RATIONAL)`, which an operation asks of the integer or
the rational it takes, and a call of the module of each its parts make
([Q-M1f.404](Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)), and each flag derives `(false)`. The
check asks its parts' checks a rule a shape: a part that is a numeral is asked `std.math.decimal`'s, which flags a
malformed one ([Q-M1f.274](decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)); a rational's
integer numerator is `std.math.int`'s to check, and a numerator that is a natural number `std.math.decimal`'s. A
part `z` is valid already, and one that is a digit or a name is the shape rules'.

**Why.** An integer and a rational are representations too, and their shapes are their own modules' to check
([Q-M1f.210](Q-M1f.210-a-malformed-number-is-its-representations-fault.md)).

**Held by.** `ein-infer/tests/int_theory.rs`: `a_malformed_integer_and_an_operands_fault_are_flagged`, and
`every_rule_has_one_premise`, each shape rule reading the check and at most a probe; `rat_theory.rs`:
`a_malformed_rational_is_flagged`, and the same; `tests/stdlib/math/int/23_digit_as_a_part.ein`,
`29_named_constant_as_a_part.ein`; `tests/stdlib/math/rat/24_digit_as_the_numerator.ein` to
`29_named_constant_as_the_denominator.ein`.
