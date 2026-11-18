# Q-M1f.392 — A saved state is saved and loaded as itself, in both directions

Save and load modify nothing and add or remove no bit of information, in either direction: **save → load → save is
byte for byte** (a saved file, loaded and its root saved again, is the same bytes), and **load → save → load is dump
for dump** (the two loads print the same `ein saturate --dump` KB section). A saved state is **the program unchanged
with the choices made**, and the state is derived again on load, every fact by the rule that derived it, with the
origin it had: the first load of a saved file is the state the run saved, fact for fact and origin for origin. **A
saved file loads with no other file beside it.** The format is
[Q-M1f.400](../read-out/Q-M1f.400-a-saved-file-states-the-choices-made-in-one.md) and
[Q-M1f.401](../read-out/Q-M1f.401-a-saved-file-carries-its-program-and-modules.md), what a saved file is to every
command [Q-M1f.402](Q-M1f.402-a-saved-file-is-a-node-of-its-programs-lattice.md), and its claims
[Q-M1f.403](Q-M1f.403-a-saved-file-with-a-commitment-checks-none.md).

**Why.** A state is a function of the program and of the hypotheses committed, so a file that holds exactly those two
loses nothing and adds nothing. A file that wrote each fact of the state as text a program states would read a
derived fact as given, lose its origin, and grow on every save; a choice written as a given fact is not a choice.

**Held by.** `ein-cli/tests/answer_round_trip.rs`: `every_state_file_is_saved_and_loaded_as_itself_0` to `_7`, every
state file of every corpus program held to the run's state and to both invariances, and the binary on each program's
first and last file; `the_witnesses_are_saved_and_loaded_as_their_states`. `ein-cli/tests/saved_files.rs`:
`a_rule_built_fact_is_saved_and_loaded_as_one_solution`, `a_choice_is_saved_as_a_commitment_and_loaded_as_the_node`,
on `tests/saved/`. The page: [10-answer-dump.md](../../ein/user/04-reference/10-answer-dump.md).
