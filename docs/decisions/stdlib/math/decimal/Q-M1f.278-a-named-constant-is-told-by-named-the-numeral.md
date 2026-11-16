# Q-M1f.278 — A named constant is told by `named`, the numeral module's relation

`named` is declared by `std.math.decimal`: a name that stands for a number. `std.math.decimal-arith`, which
declares `let`, writes `(named NAME)` for every name a `let` states, by `let-named`, and the numeral's check reads
it by a probe, `numeral-prefix-named`, to flag a named constant inside a numeral, `(n price 1)`. A prefix that is
any other name, `(n end 1)`, is not flagged
([Q-M1f.318](../Q-M1f.318-a-flag-means-malformed-and-no-flag-does-not-mean.md)). Binary's is `bin-named`, the same
way.

**Why.** What makes a name a named constant stays the module's that declares `let`, and the module below reads it
by a relation of its own; a well-formed numeral pays nothing for the probe.

**Held by.** `stdlib/math/decimal.ein`'s `numeral-prefix-named`; `stdlib/math/positional.ein`'s `let-named`;
`tests/stdlib/math/decimal-arith/26_malformed_ends_in_a_name.ein`.
