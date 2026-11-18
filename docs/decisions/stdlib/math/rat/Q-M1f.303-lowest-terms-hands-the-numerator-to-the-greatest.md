# Q-M1f.303 — Lowest terms hands the numerator to the greatest common divisor first

`rat-simplify` calls `(gcd-run TAG A D)`, the chain behind the tagged call `gcd-call`, with A the numerator's size
and D the denominator, in that order.

**Why.** Euclid's chain turns the two round in its first step where A is the smaller
([Q-M1f.301](../gcd/Q-M1f.301-the-greatest-common-divisor-puts-each-number.md)): one division more on a proper
fraction and none on an improper one, and the answer the same either way.

**Held by.** `stdlib/math/rat.ein`'s `rat-simplify-pos` and `rat-simplify-neg`; `ein-infer/tests/rat_theory.rs`:
`the_circuit_of_each_operation`.
