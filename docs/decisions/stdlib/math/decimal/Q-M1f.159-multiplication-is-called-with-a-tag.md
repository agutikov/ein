# Q-M1f.159 — Multiplication is called with a tag

A product is a call under a tag, `(mul-run TAG X Y)`, answered by `(done TAG P)`; the tagged call `(mul-call TAG X
Y)` checks what it is handed and is handed on to it
([Q-M1f.404](../Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)). The public request makes it under
the tag `(product X Y)`, and `mul-answer` reads the answer into `(product-of X Y P)`.

**Why.** A theory over this one multiplies inside its own circuits, as `std.math.rat` and `std.math.gcd` do.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_circuit_of_one_multiplication`,
`the_tagged_calls_keep_their_shapes`.
