# Q-M1f.192 — A program that reads the binary tables imports `std.math.bits`

Binary's tables are one module, `std.math.bits`, and every program that reads them imports it: the binary logic
examples set a circuit of gates against `bin-fa`, the binary full adder, by importing the module. No example holds
a copy of a table.

**Why.** The stdlib is found from every program's directory, so a table it holds needs no copy beside the program
that reads it, and one source cannot drift from a copy.

**Held by.** `examples/binary_logic/adder_against_table.ein`; `ein-infer/tests/bool_theory.rs`:
`the_full_adder_from_gates_is_the_bits_table`.
