# Q-M1f.265 — Each `std.bool` module is entered by its last name, and closes its relations by rule

The four modules' entries are their last names, `gates`, `props`, `circuit` and `inputs`, each a closing rule that
closes every relation of its module, a table among them, but the hypothesis `in` of `std.bool.inputs`, and names
every rule. `std.bool.gates` is written by hand but for the table's rows, which are generated. The expected
property facts are the claims of programs under `tests/stdlib/bool/props/`, a family of relations a program, and
the second layout's of one under `tests/stdlib/bool/gates/`; those programs are generated too.

**Why.** As every module of the stdlib: an entry a program may name, and a closing rule that says which of the
module's relations are written or derived, and so never guessed.

**Held by.** `ein-infer/tests/bool_theory.rs`: `each_closing_rule_names_every_rule_and_closes_every_relation`,
`the_table_is_the_sixteen_columns`.
