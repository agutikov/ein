# Q-M1f.237 — A name prints as the main file writes it, else by its full name

Every surface that prints a name, a state, a trace, a core, `refuted by`, `--events`, prints the main file's
spelling of it: the short name the main file lists, else the spelling of its first alias
([Q-M1f.253](Q-M1f.253-under-an-alias-a-name-prints-as-the-main-file.md)), else the full name. A program that lists
`add` reads `(add …)`; a step of the circuit it never names reads `std.math.decimal-arith.strip`. A spelling
another file writes as a name of its own is never taken, so two declarations never print as one.

**Why.** A program's facts print as the program writes them, so a reader meets the words of the file they wrote;
what the program gives no spelling of its own is printed by the one name that is the same everywhere.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `what_prints`; `ein-ir/tests/imports_semantics.rs`:
`one_module_under_two_spellings_is_one_copy`; `ein-cli/tests/imports_by_full_name.rs`:
`the_calculator_by_the_names_a_program_writes`, the program's names and the circuit's full ones in one trace.
