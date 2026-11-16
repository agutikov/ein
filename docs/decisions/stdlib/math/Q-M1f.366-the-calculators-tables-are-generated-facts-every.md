# Q-M1f.366 — The calculator's tables are generated facts, every row held to the integers

What school arithmetic knows of the digits is written down once, as tables of facts and no rule: `std.math.digits`
for the ten decimal digits, the full adder, the full subtractor, the digit multiplier, the comparator, *not zero*,
and the bounds a division's bisection reads; `std.math.bits` for binary's four. Both files are generated and not
edited by hand, and a test computes every row again from the integers, none missing and none twice. The theories
are the wiring: every step of theirs is a lookup in a table, and a core that passes through a table names the row,
a premise like any other fact.

**Why.** Ein has no numbers ([Q-M1e2.24](../../language/Q-M1e2.24-ein-has-no-numbers-and-no-ranges.md)): the digits
are names, and what is known of them is held as hardware holds it. A table is the one place a fact about two digits
is stated, and its rows are not typed or checked by eye.

**Held by.** `ein-ir/tests/digit_tables.rs`: `every_row_agrees_with_the_integers`,
`every_bit_row_agrees_with_the_integers`; `tests/stdlib/math/digits/02_a_core_names_its_rows.ein`.
