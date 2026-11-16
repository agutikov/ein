# Q-M1f.183 — A rational is reduced only on request

An operation answers unreduced, and `rat-simplify` alone answers in lowest terms: the numerator simplified, its
sign taken off, the greatest common divisor of its size and the denominator asked of `std.math.gcd`'s tagged call,
and both divided by it. Two equal rationals may be two different facts, and a rule that wants *the same number*
asks `rat-cmp`, which compares by cross products and needs no lowest terms. What an unreduced answer fixes is
[Q-M1f.302](Q-M1f.302-an-unreduced-answer-has-its-numerator-in-normal.md).

**Why.** Most of a reduction's cost is division, and most rationals of a program need not be reduced: a program
asks for lowest terms where it wants them.

**Held by.** `ein-infer/tests/rat_theory.rs`: `only_rat_simplify_reduces`,
`equal_values_are_two_facts_and_rat_cmp_says_eq`; `tests/stdlib/math/rat/01_simplify.ein` and
`31_equal_values_two_facts.ein`.
