# Q-M1f.323 — Conversion between the bases is a module of its own

`std.math.convert` converts decimal to binary, `(to-bin X)` answered by `(bin-of X B)`, and binary to decimal,
`(to-dec B)` answered by `(dec-of B X)`, each answer in its representation's normal form. It is a theory over both
bases, which it calls and edits neither of. A program that computes in binary alone imports `std.math.binary-arith`
and loads nothing of decimal.

**Why.** A program in one base pays nothing for the other's tables, and one that converts imports both by one
import.

**Held by.** `ein-infer/tests/binary_theory.rs`: `conversions_are_right_both_ways`;
`tests/stdlib/math/convert/01_to_binary.ein`, `02_to_decimal.ein` and `03_round_trip.ein`.
