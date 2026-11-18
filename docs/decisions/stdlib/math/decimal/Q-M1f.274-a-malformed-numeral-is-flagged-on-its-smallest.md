# Q-M1f.274 — A malformed numeral is flagged on its smallest term at fault

The check of a numeral an operation takes walks it a place a firing from the units
([Q-M1f.404](../Q-M1f.404-a-numeral-is-checked-where-an-operation-takes-it.md)), and four rules of
`std.math.decimal` read each place the walk reaches. The first place at fault stops the walk and is flagged on its
own `n` term, `(not-decimal NUMERAL THING)`: THING stands where a digit or a prefix goes and is neither, a name that
is no digit, a fact, a digit as the prefix, or a named constant as the prefix. `not-decimal-is-false` derives
`(false)` from the flag. A well-formed place is one step of the walk and no flag. Binary's flag is `(not-binary
NUMERAL THING)`, by the same rules over `b2`.

**Why.** Every place of a numeral is a term of its own, so the place at fault is named where it is, and the flag
says what stands there.

**Held by.** `ein-infer/tests/arith_theory.rs`: `a_malformed_request_is_flagged_once_and_never_answered`, each
numeral's one flag on its smallest term at fault; `tests/stdlib/math/decimal-arith/22_malformed_digit.ein` to
`26_malformed_ends_in_a_name.ein`; `tests/stdlib/math/binary/02_a_name_in_a_bits_place.ein` to
`05_ends_in_a_name.ein`; `ein-cli/tests/arith_search_free.rs`: `every_arithmetic_program_is_search_free`.
