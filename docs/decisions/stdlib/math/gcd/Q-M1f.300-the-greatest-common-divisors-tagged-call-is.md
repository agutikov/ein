# Q-M1f.300 — The greatest common divisor's tagged call is an interface

`(gcd-call TAG X Y)` is answered by `(gcd-done TAG G)`, an interface as the decimal arithmetic's tagged calls are
([Q-M1f.189](../Q-M1f.189-the-decimal-arithmetics-tagged-calls-are.md)). X and Y are natural numbers, high zeros
allowed, and never a name; G is in normal form; the TAG is the caller's and names one call. The public requests,
`gcd` and `lcm`, are built on it.

**Why.** A rule of another module cannot read `(gcd-of X Y G)` without joining it to its own state on a number
([Q-M1f.362](../Q-M1f.362-no-rule-of-the-calculator-joins-two-facts-on.md)); `std.math.rat`'s lowest terms calls it
instead.

**Held by.** `ein-infer/tests/gcd_theory.rs`: `the_tagged_call_answers_under_its_tag`;
`ein-infer/tests/arith_theory.rs`: `the_tagged_calls_keep_their_shapes`; `tests/stdlib/math/gcd/07_gcd_call.ein`.
