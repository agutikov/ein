# Q-M1f.163 — A division by zero is flagged wherever division is called

Every division runs as `(div-run TAG X Y)`, answered by `(div-done TAG Q R)`: the tagged call `(div-call TAG X Y)`
handed on, or the calculator's own ([Q-M1f.404](../Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)).
A division whose Y is zero by value derives `(division-by-zero (div-call TAG X Y))`, the call as it was asserted,
and `(false)` from it, whoever made the call: this module for a public request, whose division is a call, or a rule
of a program's own. The modules over this one divide by nothing that is zero. What a public request's flags are
is [Q-M1f.211](Q-M1f.211-a-public-division-by-zero-holds-two-flags.md).

**Why.** A division by zero has no answer wherever it is asked, and a caller that did not say so itself would be
left alive and silent.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_tagged_call_by_zero_is_flagged_whoever_made_it`;
`tests/stdlib/math/decimal-arith/39_div_call_by_zero.ein`, `40_div_call_by_zero_spelt.ein` and
`41_div_call_not_zero.ein`.
