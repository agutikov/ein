# Q-M1f.302 — An unreduced answer has its numerator in normal form and no high zero in its denominator

Every answer of an operation of `std.math.rat` has its numerator in `std.math.int`'s normal form, one part `z`, and
its denominator a natural number with no high zero, put into normal form by an addition walk to zero where it is
not a product; nothing is divided. The tagged call `(rat-norm TAG P N D)` answers so: P less N over D, the
numerator in normal form and D with no high zero, not reduced.

**Why.** One sentence says what every answer is, and a caller of the interface reads it whatever it hands over.

**Held by.** `ein-infer/tests/rat_theory.rs`: `simplification_and_negation_are_right`;
`ein-infer/tests/arith_theory.rs`: `the_tagged_calls_keep_their_shapes`;
`tests/stdlib/math/rat/08_unsimplified.ein`.
