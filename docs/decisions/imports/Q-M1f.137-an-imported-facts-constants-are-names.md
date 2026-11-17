# Q-M1f.137 — An imported fact's constants are names of the program

A fact an import brings is a fact of the program, and the constants it writes are names of the program like the ones
the main file writes: they are guessable atoms. A library of n constants widens the universe of every relation whose
held extent is not closed by n. A program that wants no guesses over a library's relation closes its held extent, or
the library closes its own.

**Why.** The program's names are the constants its facts and rules write, and an import brings its module's text
into the program whole ([Q-M1f.247](Q-M1f.247-every-import-brings-its-module-whole.md)). Marking a library's
constants apart would make a program answer differently from the same program with the module written in place.

**Held by.** `ein-cli/tests/import_facts.rs`:
`a_flat_import_of_facts_answers_as_the_facts_written_in_place_on_every_command`, each program against its twin
with the module written where the import stands, on every command that loads, `solve` and `oracle` among them.
