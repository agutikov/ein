# Q-M1f.277 — An operand of another kind of number is answered by silence

A natural number where `std.math.int` expects an integer, and an integer or a natural number where `std.math.rat`
expects a rational, is neither flagged nor answered, and no kind of number is promoted to another. Each module's
header names the silence.

**Why.** Such an operand is a well-formed term of another module, so no representation is at fault, and *not of
this shape* is no monotone rule's to say
([Q-M1f.318](Q-M1f.318-a-flag-means-malformed-and-no-flag-does-not-mean.md)).

**Held by.** `ein-infer/tests/int_theory.rs`: `a_natural_number_is_not_an_integer_operand`; `rat_theory.rs`:
`an_integer_is_not_a_rational_operand`; `tests/stdlib/math/int/28_natural_where_an_integer_goes.ein`,
`tests/stdlib/math/rat/22_integer_where_a_rational_goes.ein` and `23_natural_where_a_rational_goes.ein`.
