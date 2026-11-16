# Q-M1f.281 — A `let` that names a digit is flagged

A `let` whose value is a digit, `(let seven 7)`, `0` among them, is flagged `(malformed (let NAME DIGIT))` by
`let-to-digit` and `let-to-digit-zero`, whether or not a request names it; so is a `let` whose value is another
named constant, by `let-to-name`.

**Why.** A `let` names a number, and a digit is no number: seven is `(n z 7)`. A `let`'s value is the `let`'s to
read.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_let_that_cannot_be_read_is_flagged`;
`tests/stdlib/math/decimal-arith/42_let_to_a_digit.ein`, `43_let_to_the_digit_zero.ein` and
`32_malformed_name_for_a_name.ein`.
