# Q-M1e2.30 — No configuration option, lever or flag changes R

R is a property of the program: of its lattice of commitments, each node read by saturation. No `(config …)` field,
command-line option, environment name or lever changes it. Under every one, a run that discovers R whole reports the
same R; an option may change how much work a run does and how fast, never which states are solutions. An option may
also decline to discover R, which is not a change of R: `--no-search` and `(solve :no-search)` judge root alone and
state no R, and a budget stops a run with a part of R that the run says is a part. A flag found to change R is
retired and refused, not kept as a mode.

One thing besides R may vary with an option: a dead node's core under `:record-alternative-justifications false`
([Q-M1e2.11](Q-M1e2.11-a-core-is-the-smallest-over-every-derivation.md)), since a core is an explanation and not
part of the answer ([Q-M1f.6](Q-M1f.6-a-core-is-an-explanation.md)).

**Why.** The solutions are the program's, not the way the engine walks its lattice: a lever under which the answer
moves gives a wrong answer, not a weaker mode. Holding every option to R is what lets an option be chosen for speed
alone, and lets every command be held to the oracle
([Q-M1e2.32](Q-M1e2.32-no-two-commands-disagree-and-the-oracle-is.md)).

**Held by.** `ein-cli/tests/lever_invariance.rs`: `every_declared_lever_cell_moves_no_line_of_any_mode`, every mode
under every lever a corpus entry declares, `--no-search` among them; `docs/ein/user/04-reference/06-config.md`, whose every flag
row says whether it changes the answer, held by `ein-cli/tests/config_reference.rs`
(`every_flag_has_a_row_and_no_row_is_orphaned`, `the_retired_flags_are_refused_with_why`).
