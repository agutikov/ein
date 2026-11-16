# Q-M1f.239 — A program declares no name under the full name of a module it imports

The main file may not declare a name under the full name of a module the program imports: `(relation m.mine …)`
beside an import of `m` is refused at load, on every command. A full name of `m` is one `m` declares. A dotted name
whose prefix is no module of the program is a name like another, so a `--dump-answer` state file, which imports
nothing and declares the full names its program was loaded with, loads.

**Why.** A full name names one declaration of one module wherever it is written; a program that could add to a
module's names would make a full name mean something the module never declared.

**Held by.** `ein-ir/tests/imports_full_names.rs`: `the_refusals`, the declaration refused and a state file's
declarations loaded; `ein-cli/tests/answer_round_trip.rs`: `every_state_file_reloads_to_its_own_facts_by_value_0`
to `_7`, every state file of every corpus program reloaded.
