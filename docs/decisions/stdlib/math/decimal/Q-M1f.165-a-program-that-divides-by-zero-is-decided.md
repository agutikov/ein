# Q-M1f.165 — A program that divides by zero is decided by root alone

A program that divides by zero searches nothing, as every arithmetic program does: no candidate, one saturation, no
fork. Its R is empty, decided by root alone; the core names the request, and `refuted by` the rule that derives
`(false)` from the flag. Such a program claims only `(solve :expect (false))`.

**Why.** Nothing is searched either way; a division by zero changes the count of states, from one to none.

**Held by.** `ein-cli/tests/arith_search_free.rs`: `every_arithmetic_program_is_search_free`;
`tests/stdlib/math/decimal-arith/20_div_by_zero.ein` and `21_div_by_zero_spelt_and_named.ein`.
