# Q-M1f.240 — A module keeps its entry rule, which a program may list

Each module of the calculator keeps its entry, the closing rule that names every rule of the module and closes
every relation of it (`arith`, `decimal`, `decimal-arith`, `int-arith`, `rat-arith`, `gcd`, `arith9`, …), with the
relation that lists its rules. A program may list the entry beside the names it writes, and needs not: an import
brings the module whole whether or not the entry is listed, and the entry brings nothing a whole import does not.

**Why.** The closing rule fires in every program that imports the module, and it is what keeps the theory's
relations out of the search; listing it is a name like another, which a whole import already binds in full.

**Held by.** `ein-infer/tests/arith_theory.rs`: `the_closing_rule_names_every_rule_and_closes_every_relation`;
`ein-cli/tests/imports_by_full_name.rs`: `the_calculator_by_the_names_a_program_writes`, a program that lists no
entry and computes.
