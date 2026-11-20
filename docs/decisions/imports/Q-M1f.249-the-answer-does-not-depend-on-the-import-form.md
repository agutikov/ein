# Q-M1f.249 — The answer does not depend on the import form

A program answers alike under each of the three import forms: with its imports selective, `:symbols (…)`; whole,
`(import M)`, every short name it writes written in full; and aliased, `(import M :as a)`, every such name written
`a.name`. R, its two counts, root's verdict and debts, whether the run exhausted, every query's classes and every
claim's outcome are the same once module prefixes and aliases are taken off. A program one form refuses, the others
refuse. What an import loads chooses no writer of H: a module imported for one rule has the writer and the H of
that rule written out, since an obligation it brings that nothing activates writes nothing.

**Why.** An import form says only which spelling a file writes
([Q-M1f.247](Q-M1f.247-every-import-brings-its-module-whole.md)). Two programs that differ only in it are one
program, and an answer that moved between them would show the import mechanism wrong.

**Held by.** `ein-cli/tests/import_invariance.rs`: `every_corpus_program_answers_alike_under_every_import_form`,
every corpus program that imports, in the three forms;
`a_module_imported_for_one_rule_takes_the_rung_of_the_rule_written_out`.
