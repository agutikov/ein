# Q-M1f.263 — Every program that imports `std.bool.gates` derives the second layout

`std.bool.gates` holds the sixteen rules, `Zero-rows` to `One-rows`, that derive a relation a function from the
table ([Q-M1f.325](Q-M1f.325-a-boolean-function-is-a-value-of-one-table.md)), and every program that imports the
module derives them at root, beside the table's rows. No program takes the table without them.

**Why.** Every import brings its module whole
([Q-M1f.231](../../imports/Q-M1f.231-every-name-a-module-declares-has-a-full-name.md)), and a function's name is
declared where the table is, so the rules that derive the second layout belong to the table's module.

**Held by.** `ein-infer/tests/bool_theory.rs`: `each_relation_of_a_function_is_its_rows`;
`tests/stdlib/bool/gates/01_a_relation_a_function.ein`.
