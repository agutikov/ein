# Q-M1f.295 — The binary numeral is `std.math.binary`'s, which checks it

`std.math.binary` is binary's representation, on `std.math.decimal`'s pattern: it declares `b2` and `bin-named`,
the check `bin-check` an operation asks of a numeral it takes and its label `bin-valid`
([Q-M1f.404](../Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)), and four rules over the check's
walk flag `(not-binary NUMERAL THING)`, from which `not-binary-is-false` derives `(false)`. A decimal digit as a prefix, `(b2 7 1)`, is a name in binary's eyes, and
silent, as `(n end 1)` is in decimal. The module's entry is `binary`.

**Why.** A malformed binary numeral is its representation's fault
([Q-M1f.210](../Q-M1f.210-a-malformed-number-is-its-representations-fault.md)), and binary's representation stands
apart from its arithmetic, as decimal's does.

**Held by.** `stdlib/math/binary.ein`; `tests/stdlib/math/binary/01_well_formed_everywhere.ein` to
`07_a_programs_own_named_z.ein`.
