# Q-M1f.139 — A fact stated twice keeps its first statement

One fact stated twice is one fact. Stated with two `:source`s, by a module and the program or by two modules, it
keeps the first statement's provenance and the second is dropped with no word, as it is when one file writes the
fact twice. Nothing refuses or warns.

**Why.** The knowledge base is a set of facts, and the loader's first occurrence is the most primitive statement it
has. A module's text is the program's, so the rule for two statements in one file is the rule for two across
files.

**Held by.** `ein_core::kb::Kb::add_fact`, the loader's path, first occurrence kept; its unit test in
`ein-core/src/kb.rs`, `the_loader_path_keeps_the_first_occurrence_and_does_not_index`. A fact that reaches the
program through two imports of one module is one statement:
[Q-M1f.143](Q-M1f.143-a-modules-fact-arrives-once-however-many-imports.md).
