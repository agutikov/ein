# Q-M1f.143 — A module's fact arrives once, however many imports reach it

A module is one copy per program, read once whoever imports it and through however many paths, so each fact it
states arrives once, where the module first arrives. A fact the program writes itself beside the same fact of a
library is another statement, and the loader keeps one fact of the two
([Q-M1f.139](Q-M1f.139-a-fact-stated-twice-keeps-its-first-statement.md)).

**Why.** A module reached by two paths is the same text, and handing its facts to the loader once per path would
repeat every row of a large table, such as the calculator's, for nothing: the knowledge base is a set, so no answer
would differ.

**Held by.** `ein-ir/tests/imports_semantics.rs`: `a_diamond_of_flat_imports_yields_each_fact_once`;
`ein-cli/tests/imports_by_full_name.rs`: `an_alias_makes_no_copy`, the calculator listed and aliased holding one
set of tables.
