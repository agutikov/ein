# Q-M1f.318 — A flag means malformed, and no flag does not mean well-formed

No mechanism reports a request that no rule completes. Each module's header lists what stays silent, a request
neither refused nor answered: a numeral whose prefix is a name that is neither `z`, a digit nor a named constant,
`(n end 1)`; a fact of another relation where a number goes, `(add (pair 1 2) X)`; a name that stands for either;
and an operand of another kind of number
([Q-M1f.277](Q-M1f.277-an-operand-of-another-kind-of-number-is-answered.md)).

**Why.** *This request has no answer* is an absence, which no monotone rule can conclude; the calculator reads one
`absent` a base, over a relation no rule derives
([Q-M1f.317](decimal/Q-M1f.317-a-name-no-let-states-is-flagged-as-undeclared.md)).

**Held by.** `ein-infer/tests/arith_theory.rs` and `binary_theory.rs`: `what_stays_silent`;
`tests/stdlib/math/decimal/05_what_stays_silent.ein`, `tests/stdlib/math/decimal-arith/38_what_stays_silent.ein`,
`tests/stdlib/math/binary-arith/22_what_stays_silent.ein`, `tests/stdlib/math/gcd/12_what_stays_silent.ein`.
