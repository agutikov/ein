# Q-M1f.324 — Decimal division is long division, each digit found by bisection

`std.math.decimal-arith` divides by school long division: the divisor's multiples by 1 to 9 computed once and
carried in the state, the dividend's digits brought down one at a time, and each digit of the quotient found by
halving the range 0 to 10, at most four trials a digit, each decided by subtraction's walk. It is the module's one
division.

**Why.** One division that is right and searches nothing is enough; another algorithm would change no answer.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_circuit_of_one_division`;
`tests/stdlib/math/decimal-arith/16_div.ein` and `17_div_every_digit.ein`.
