# Q-M1f.156 — Comparison is called with a tag

A comparison is a walk under a tag, `(cmp-walk TAG X Y eq)`, which hands its order over as `(order-is TAG O)`. The
public request calls it under the tag `(order X Y)`, and `cmp-answer` reads `(order-is (order X Y) O)` into
`(cmp-of X Y O)`.

**Why.** A theory over this one compares numbers inside its own circuits, and a call from inside a theory starts a
walk with a tag of its own.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_circuit_of_one_subtraction_and_one_comparison`,
`the_tagged_calls_keep_their_shapes`.
