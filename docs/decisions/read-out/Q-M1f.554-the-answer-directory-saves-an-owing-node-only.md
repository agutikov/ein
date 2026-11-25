# Q-M1f.554 — The answer directory saves an owing node only where R is empty

`ein solve FILE --dump-answer DIR` saves a node as a file that loads to it
([Q-M1f.402](../answer/Q-M1f.402-a-saved-file-is-a-node-of-its-programs-lattice.md)) in three cases: each state of R the
run reached, under `models/`; each node of the owing frontier where R is empty of the owing kind, under `owing/`; and
root under `--no-search`. An owing node below a non-empty R is in none of the three, and no command writes its file.
Written by hand, `models/NN.ein` with its `(commit …)` cut to the node's commitment, it loads as the node, and the
read-out says what it owes.

**Why.** The owing frontier is said of an empty R, where every alive node owes and the frontier is where the search
stopped. Below a non-empty R an owing node can have an extension in R, and is then no maximal alive state: the frontier
of a non-empty R is not defined, so there is nothing of it to save.

**Held by.** `ein_render::dump::region_answer::write`, which writes `owing/` from the read-out's owing listing alone;
`ein-cli/tests/answer_dump.rs`: `every_recorded_state_loads_to_its_own_state`, every file under `models/` and
`owing/` loaded to its node. The page:
[the answer dump § The directory](../../ein/user/04-reference/10-answer-dump.md#the-directory).
