# Q-M1f.148 — The seven digit tables are one module

The decimal digits' seven tables, `fa`, `fs`, `md`, `cl`, `nz`, `mid` and `adj`, are one module, `std.math.digits`;
binary's four are another, `std.math.bits`. An import brings every table of the module and its rows, whichever of
them the program writes.

**Why.** One name for one ontology. A module is read only where an import names it, so a program that does not
compute pays nothing, and one that does pays the parse once a program, not once a request.

**Held by.** `ein-ir/tests/digit_tables.rs`: `the_module_is_seven_closed_tables_and_nothing_else`;
`tests/stdlib/math/digits/01_tables.ein`.
