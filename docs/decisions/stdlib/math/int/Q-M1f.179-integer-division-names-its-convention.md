# Q-M1f.179 — Integer division names its convention

`std.math.int` has no bare division. Each of the three usual conventions is a request of its own, answered by one
fact with a quotient Q and a remainder R, X = Q × Y + R:

- `(int-div-euclid X Y)`, answered by `(int-quot-euclid-of X Y Q R)`: 0 ≤ R < |Y|, and R is a natural number of
  `std.math.decimal-arith`, since it is never negative;
- `(int-div-trunc X Y)`, answered by `(int-quot-trunc-of X Y Q R)`: Q rounded toward zero, R with the dividend's
  sign;
- `(int-div-floor X Y)`, answered by `(int-quot-floor-of X Y Q R)`: Q rounded down, R with the divisor's sign.

**Why.** The conventions differ only when an operand is negative, and each is the one some reader expects: number
theory the Euclidean, C and Rust the truncated, Python the floored. A request that names its convention leaves no
reader to guess which one the theory took.

**Held by.** `ein-infer/tests/int_theory.rs`, against Rust's integers on every pair of small parts and on requests
of every length and sign; `tests/stdlib/math/int/09_div_euclid.ein` to `14_div_floor_by_zero.ein`. The results'
names: [Q-M1f.200](Q-M1f.200-an-integer-divisions-result-is-named-after.md).
