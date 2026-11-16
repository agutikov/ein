# Q-M1f.299 — The greatest common divisor of zero and zero is zero

gcd(X, 0) and gcd(0, X) are X, and gcd(0, 0) is zero; lcm(X, 0), lcm(0, X) and lcm(0, 0) are zero, the last by
`lcm-zero`, which divides nothing. No request of `std.math.gcd` divides by zero.

**Why.** Zero is the greatest element of divisibility, so gcd(X, 0) = X holds at X = 0 too, and Euclid's chain
gives it with no rule of its own.

**Held by.** `ein-infer/tests/gcd_theory.rs`: `zero_in_each_place`; `tests/stdlib/math/gcd/02_gcd_zero.ein` and
`06_lcm_zero.ein`.
