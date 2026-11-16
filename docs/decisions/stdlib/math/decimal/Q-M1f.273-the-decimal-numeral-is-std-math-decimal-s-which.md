# Q-M1f.273 — The decimal numeral is `std.math.decimal`'s, which checks it

`std.math.decimal`, the representation, declares the numeral's constructor `n` and `named`, and holds the numeral's
check; it computes nothing. `std.math.decimal-arith`, the arithmetic, imports it. A program that writes a numeral
lists `n` from `std.math.decimal`, since nothing is re-exported.

**Why.** A malformed numeral is the representation's fault
([Q-M1f.210](../Q-M1f.210-a-malformed-number-is-its-representations-fault.md)), and the representation stands apart
from its arithmetic.

**Held by.** `stdlib/math/decimal.ein`; `tests/stdlib/math/decimal/01_well_formed_everywhere.ein` to
`06_a_programs_own_named_z.ein`.
