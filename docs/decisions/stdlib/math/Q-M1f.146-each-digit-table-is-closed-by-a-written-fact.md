# Q-M1f.146 — Each digit table is closed by a written fact

`std.math.digits` closes each of its seven tables with a written `(__closed__ R)` fact, and `std.math.bits` each of
its four: a table is complete, and a row is never a hypothesis. The facts come with every import of the module, as
its rows do. A theory that reads a table closes it in its own closing rule too, as `std.math.decimal-arith` and
`std.math.decimal` do.

**Why.** A library of facts holds no rule, so its closure is written as facts; a table left open would be a
relation whose rows the search may guess.

**Held by.** `ein-ir/tests/digit_tables.rs`: `the_module_is_seven_closed_tables_and_nothing_else`,
`the_bits_are_four_closed_tables_and_nothing_else`; `tests/stdlib/math/digits/01_tables.ein` and
`tests/stdlib/math/bits/01_tables.ein`, each a program whose R is {∅}.
