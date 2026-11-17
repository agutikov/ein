# Q-M1f.177 — A module over the decimal arithmetic tags each call with its own request

`std.math.int` and `std.math.rat` compute with `std.math.decimal-arith`'s walks and calls, and edit none of its
rules. A call's tag is the module's own request: `(int-add X Y)` tags its own computation, and the rule that
answers reads `(int-done (int-add ?x ?y) ?s)`. Each module looks its operands up through `std.math.decimal-arith`'s
`operand`, and extends the look-up by a rule of its own for its own kind of number, `int-operand` and
`rat-operand`, so that a name goes through the one `let`.

**Why.** A request is a fact a state holds, so it is an object, and a tag that is the request needs no relation of
its own. Looking an operand up through the one `let` keeps one kind of named constant in a program.

**Held by.** `ein-infer/tests/int_theory.rs` and `rat_theory.rs`: `named_operands_work_for_every_request`,
`the_circuit_of_each_operation`; `ein-infer/tests/arith_theory.rs`: `the_tagged_calls_keep_their_shapes`.
