# Q-M1f.328 — Boolean functions and circuits ship as four `std.bool` modules

Boolean functions and circuits are four modules of the stdlib: `std.bool.gates`, the sixteen functions;
`std.bool.props`, what is known of them; `std.bool.circuit`, a circuit of gates simulated by saturation; and
`std.bool.inputs`, a circuit's inputs as hypotheses that each owe one value, and the values a program requires. The
applications are programs under `examples/binary_logic/`, a full adder built of gates among them, held equal to
binary's full-adder table.

**Why.** Each module is a subject a program imports alone, and an application reads only what it imports.

**Held by.** `ein-infer/tests/bool_theory.rs`: `the_full_adder_from_gates_is_the_bits_table`; `tests/stdlib/bool/`;
`examples/binary_logic/adder_against_table.ein`.
