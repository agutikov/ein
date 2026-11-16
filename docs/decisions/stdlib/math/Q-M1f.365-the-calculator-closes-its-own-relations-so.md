# Q-M1f.365 — The calculator closes its own relations, so arithmetic searches nothing

Each module of the calculator ends in a closing rule, its entry, with an empty `:match`: it closes every relation
of the module and each table it reads, `(__closed__ R)`, and names every rule. A program that imports the
calculator and states requests has no candidate among the calculator's relations: H is empty, one saturation
decides the program, and its R is {∅}, or empty where it divides by zero or is flagged, alike under `ein solve`,
`ein saturate` and `ein test`. A relation the importing program declares itself is the program's to close.

**Why.** Arithmetic is computed by the tables and never guessed. A `(__closed__ R)` that a rule derives at root
works as a written one, so a program that imports the theory writes nothing to make it search-free.

**Held by.** `ein-cli/tests/arith_search_free.rs`: `every_arithmetic_program_is_search_free`; the tests
`the_closing_rule_names_every_rule_and_closes_every_relation` of `ein-infer/tests/arith_theory.rs`,
`int_theory.rs`, `rat_theory.rs`, `gcd_theory.rs` and `num9_theory.rs`.
