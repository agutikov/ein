# Q-M1f.276 — An integer's and a rational's own shape is checked where it is written

Each part of `(int P N)` is a natural number: a digit or a named constant where a part goes, `(int 7 z)` or
`(int price z)`, is flagged `(not-integer INTEGER THING)` by `std.math.int`. A digit or a named constant where a
rational's numerator or denominator goes is flagged `(not-rational RATIONAL THING)` by `std.math.rat`. Both are
read off the loader's `(written T)`, wherever the term is written, and each flag derives `(false)`. A part that is
a malformed numeral is `std.math.decimal`'s to flag
([Q-M1f.274](decimal/Q-M1f.274-a-malformed-numeral-is-flagged-on-its-smallest.md)).

**Why.** An integer and a rational are representations too, and their shapes are their own modules' to check, as
soon as they are written ([Q-M1f.210](Q-M1f.210-a-malformed-number-is-its-representations-fault.md)).

**Held by.** `ein-infer/tests/int_theory.rs`: `a_malformed_integer_and_an_operands_fault_are_flagged_as_written`;
`rat_theory.rs`: `a_malformed_rational_is_flagged_as_written`; `tests/stdlib/math/int/23_digit_as_a_part.ein`,
`29_named_constant_as_a_part.ein`; `tests/stdlib/math/rat/24_digit_as_the_numerator.ein` to
`29_named_constant_as_the_denominator.ein`.
