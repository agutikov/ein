# Q-M1f.160 — A zero digit of the multiplier is a row like any other

A product takes the multiplier a digit at a time from the top, and a digit `0` makes its row as any digit does: the
multiplicand times zero, a walk that strips to `z`, added to ten times the sum so far. No rule tells a zero digit
apart.

**Why.** The answer is the same, and the theory is two rules shorter.

**Held by.** `stdlib/math/decimal-arith.ein`'s `mul-row`;
`tests/stdlib/math/decimal-arith/14_mul_zeros_and_carries.ein`.
