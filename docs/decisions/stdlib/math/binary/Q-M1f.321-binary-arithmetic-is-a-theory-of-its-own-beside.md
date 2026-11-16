# Q-M1f.321 — Binary arithmetic is a theory of its own, beside decimal's

Binary numbers have a shape of their own, `(b2 PREFIX BIT)`, and requests of their own, `(bin-add X Y)`, over
binary's tables: a number says its base, and both bases stand in one program. They share `z`, the names `0` and
`1`, the division by zero and the rules of `std.math.positional`, and no relation else. A program that imports
binary alone loads nothing of decimal.

**Why.** A numeral that says its base keeps the two apart in one program, and the rules they share are written once
([Q-M1f.266](../Q-M1f.266-a-base-is-connected-to-the-shared-rules-by-one.md)).

**Held by.** `ein-infer/tests/binary_theory.rs`: `both_bases_in_one_program_agree`, `every_small_request_is_right`;
`tests/stdlib/math/binary-arith/21_both_bases.ein`.
