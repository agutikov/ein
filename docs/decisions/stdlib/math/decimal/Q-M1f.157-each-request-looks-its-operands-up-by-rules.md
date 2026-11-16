# Q-M1f.157 — Each request looks its operands up by rules of its own

Each of the five requests looks its two operands up in turn, by three rules and two tags of its own: `add-ask` asks
for the first under `(add-first X Y)`, `add-ask-second` for the second under `(add-second X Y U)`, and `add-start`
starts the walk; `sub`, `cmp`, `mul` and `div` likewise.

**Why.** Each rule keeps one premise, and the tags say in the state which operand of which request is being read.

**Held by.** `std.math.positional`'s rules `add-ask` to `div-start`; `ein-infer/tests/arith_theory.rs`:
`the_circuit_of_one_addition`.
