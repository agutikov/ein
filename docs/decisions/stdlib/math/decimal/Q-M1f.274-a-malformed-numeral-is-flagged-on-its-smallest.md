# Q-M1f.274 — A malformed numeral is flagged on its smallest term at fault

Four rules of `std.math.decimal` read every place of every numeral a file writes, `(written (n P A))`, and flag a
place at fault on its own `n` term, `(not-decimal NUMERAL THING)`: THING stands where a digit or a prefix goes and
is neither, a name that is no digit, a fact, a digit as the prefix, or a named constant as the prefix.
`not-decimal-is-false` derives `(false)` from the flag. A well-formed place costs no firing. Binary's flag is
`(not-binary NUMERAL THING)`, by the same rules over `b2`.

**Why.** Every place of a numeral is a nested fact of its own, so the place at fault is read where it is, with no
walk down the numeral again.

**Held by.** `tests/stdlib/math/decimal/01_well_formed_everywhere.ein` to `04_in_a_rules_conclusion.ein`;
`tests/stdlib/math/binary/02_a_name_in_a_bits_place.ein` to `05_ends_in_a_name.ein`;
`ein-cli/tests/arith_search_free.rs`: `every_arithmetic_program_is_search_free`.
