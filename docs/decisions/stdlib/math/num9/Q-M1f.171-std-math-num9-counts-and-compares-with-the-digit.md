# Q-M1f.171 — `std.math.num9` counts and compares with the digit tables

`std.math.num9` reads `std.math.digits`: `adj`, the pairs of bounds with nothing between them, counts a
computation's steps, and `cl`, the comparator, compares two numbers in one rule. The module adds no table of its
own.

**Why.** No fact is added to what every program that imports the module loads.

**Held by.** `stdlib/math/num9.ein`; `ein-infer/tests/num9_theory.rs`: `the_circuit_of_each_operation`.
