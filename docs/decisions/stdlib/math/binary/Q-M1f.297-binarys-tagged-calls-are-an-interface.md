# Q-M1f.297 — Binary's tagged calls are an interface

`std.math.binary-arith`'s tagged calls are an interface, as decimal's are: `(bin-add-walk TAG X Y C z)`,
`(bin-sub-walk TAG X Y 0 z)`, `(bin-cmp-walk TAG X Y eq)`, `(bin-mul-call TAG X Y)`, `(bin-div-call TAG X Y)` and
`(bin-operand TAG X)`, answered by `bin-done`, `bin-short`, `bin-order-is`, `bin-div-done` and `bin-operand-is`.
`std.math.convert`, a module of the stdlib, also reads binary's circuit: `bin-strip`, `bin-done`, `bin-top-first`,
`bin-top-first-is` and `bin-b`. Binary's module writes no fact that names its closing rule.

**Why.** Binary is decimal's twin, and a rule of a program's own may call it for a purpose of its own; a module of
the stdlib may read another's circuit, since the two change together.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_tagged_calls_keep_their_shapes`;
`ein-infer/tests/binary_theory.rs`: `conversions_are_right_both_ways`.
