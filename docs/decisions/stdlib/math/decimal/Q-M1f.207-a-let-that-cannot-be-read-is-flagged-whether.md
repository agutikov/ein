# Q-M1f.207 — A `let` that cannot be read is flagged, whether or not a request uses it

The flag names the `let`, `(malformed (let NAME NUMBER))`, and is derived from the `let` alone: where a name is
stated twice with two numbers, on each of its two `let`s; where the name is `z`; where it is a digit, `0` among
them. A name stated twice with one number is no fault. A request that uses such a name is still computed, and root
is dead, so no query reads the answer.

**Why.** Each is a second meaning for a name, and each would give one request two answers.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_let_that_cannot_be_read_is_flagged`;
`tests/stdlib/math/decimal-arith/33_let_twice.ein` to `37_let_zero_digit.ein`.
