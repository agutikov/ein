# Q-M1f.159 — Multiplication is called with a tag

A product is a call under a tag, `(mul-call TAG X Y)`, answered by `(done TAG P)`. The public request calls it
under the tag `(product X Y)`, and `mul-answer` reads the answer into `(product-of X Y P)`.

**Why.** A theory over this one multiplies inside its own circuits, as `std.math.rat` and `std.math.gcd` do.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_circuit_of_one_multiplication`,
`the_tagged_calls_keep_their_shapes`.
