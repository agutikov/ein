# Q-M1f.261 — The decimal arithmetic is `std.math.decimal-arith`

The arithmetic of decimal numerals is the module `std.math.decimal-arith`, `stdlib/math/decimal-arith.ein`, whose
entry is the closing rule `decimal-arith`. Its requests and results keep their bare names: `add`, `sub`, `cmp`,
`mul` and `div`, answered by `sum-of`, `diff-of`, `cmp-of`, `product-of` and `quot-of`.

**Why.** The name says the representation and the subject, beside `std.math.binary-arith`, and a program writes
`(import std.math.decimal-arith :symbols (decimal-arith add sum-of))`.

**Held by.** `stdlib/math/decimal-arith.ein`; `ein-cli/tests/imports_by_full_name.rs`:
`the_calculator_by_the_names_a_program_writes`.
