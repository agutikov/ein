# Q-M1f.163 — A division by zero is flagged wherever division is called

Every division goes through the tagged call `(div-call TAG X Y)`, answered by `(div-done TAG Q R)`. A call whose Y
is zero by value derives `(division-by-zero (div-call TAG X Y))`, and `(false)` from it, whoever made the call:
this module for a public request, a module over it, or a rule of a program's own. What a public request's flags are
is [Q-M1f.211](Q-M1f.211-a-public-division-by-zero-holds-two-flags.md).

**Why.** A division by zero has no answer wherever it is asked, and a caller that did not say so itself would be
left alive and silent.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_tagged_call_by_zero_is_flagged_whoever_made_it`;
`tests/stdlib/math/decimal-arith/39_div_call_by_zero.ein`, `40_div_call_by_zero_spelt.ein` and
`41_div_call_not_zero.ein`.
