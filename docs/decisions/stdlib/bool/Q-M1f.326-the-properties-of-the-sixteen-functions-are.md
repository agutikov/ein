# Q-M1f.326 — The properties of the sixteen functions are rules, in a module of their own

What is known of the sixteen functions, each function's column, the relations between two functions and the
properties of one, is derived from the table's rows by rules, in `std.bool.props`. No property fact is written. A
program that does not import the module pays nothing for them.

**Why.** Rules can never disagree with the rows, and a module of their own is reusable apart from the table.

**Held by.** `ein-infer/tests/bool_theory.rs`: `every_property_is_its_definition`, `the_three_corrected_counts`;
`tests/stdlib/bool/props/`.
