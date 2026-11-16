# Q-M1f.176 — A product of integers is computed on the parts, with no case on a sign

The product of P1 less N1 and P2 less N2 is (P1 × P2 + N1 × N2) less (P1 × N2 + N1 × P2): four products and two
sums of natural numbers, then one simplification. No rule looks at a sign.

**Why.** A formula with no case has no case to get wrong, and its operands need not be simplified first.

**Held by.** `ein-infer/tests/int_theory.rs`: `every_pair_of_small_parts_is_right`,
`the_circuit_of_each_operation`; `tests/stdlib/math/int/05_mul.ein`.
