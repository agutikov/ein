# Q-M1f.168 — Overflow is a fact and no number, and a division by zero is a contradiction

A sum or a product of `std.math.num9` that does not fit nine digits derives `(overflow REQUEST)`, the request as
written, and no result: never a truncated number. Root stays alive, so the program's other requests are answered,
and a rule may match the flag. A division by zero derives `std.math.arith`'s flag, and its rule derives `(false)`.

**Why.** An overflow is an answer about a width, which a program may read and go on from; a division by zero asks
for what does not exist ([Q-M1f.269](../Q-M1f.269-division-by-zero-is-a-contradiction.md)).

**Held by.** `ein-infer/tests/num9_theory.rs`: `width_nine_is_right_on_a_sample`,
`a_division_by_zero_is_flagged_and_false`; `tests/stdlib/math/num9/02_add_overflow.ein`, `06_mul_overflow.ein` and
`09_div_by_zero.ein`.
