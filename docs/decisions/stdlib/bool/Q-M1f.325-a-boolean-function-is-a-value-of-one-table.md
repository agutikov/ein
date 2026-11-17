# Q-M1f.325 — A Boolean function is an argument in one table and a relation of its own

`std.bool.gates` writes the sixteen functions of two inputs in two layouts: one table, `(gate2 NAME A B OUT)`, where
a function is an argument, so a rule about any function matches `(gate2 ?f …)` and a search can make the function a
hypothesis; and a relation a function, `(Xor A B OUT)`, its four rows derived from the table by sixteen rules, for a
rule that names the function it means. Each name, `Zero`, `And`, … `One`, is declared once, as the relation, and is
the same atom, under one full name, as the table's argument.

**Why.** Each layout serves a rule the other cannot: a rule over every function, and a rule over one.

**Held by.** `ein-infer/tests/bool_theory.rs`: `the_table_is_the_sixteen_columns`,
`each_relation_of_a_function_is_its_rows`; `examples/binary_logic/synthesis.ein`, a search over the function.
