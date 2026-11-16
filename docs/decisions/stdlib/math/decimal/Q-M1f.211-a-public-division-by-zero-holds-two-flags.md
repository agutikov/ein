# Q-M1f.211 — A public division by zero holds two flags, the request's and its call's

A public division whose divisor is zero derives two flags: the request's, `(division-by-zero (div X Y))`, by
`div-by-zero`, written first so that it is derived first; and the call's, `(division-by-zero (div-call TAG X Y))`,
by `div-call-by-zero`, the call as it was asserted, the divisor as the caller wrote it. Each derives `(false)`, and
`ein solve` names the request in the core.

**Why.** The request's flag reads as the program wrote it, and the call's names a fact that is in the state; a
trace reads the request's flag, then the call's.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_division_by_zero_is_flagged_and_false`,
`a_tagged_call_by_zero_is_flagged_whoever_made_it`; `tests/stdlib/math/decimal-arith/20_div_by_zero.ein`.
