# Q-M1f.403 — A saved file with a commitment checks none of its program's claims

A saved file holds its program unchanged, its `(query … :expect …)` and `(solve … :expect …)` among it, and those are
claims about the program's R. On a file with a `(commit …)` none is checked: the read-out names each NOT CHECKED, *the
claim is the program's, about the program's R, and this saved file is a node of its lattice*, and the run exits 0.
`ein test` asks it nothing, not root either: no run is made, each claim is NOT CHECKED with that line and counted not
checked, moving no exit code, its `-v` header names no question, its `--json-report` row carries no run's data, and
`--events` and `--json-summary` write nothing for it. A node's container is asked nothing the same way. A file of root
has no commitment, so it loads to the program's root and checks its claims as the program does.

**Why.** A node's part of R is not the program's R, so a claim about the one read against the other would fail where
nothing is wrong. Checked against the program's R, the claim is the program's to check, from the program. No question
about the node decides a claim of its program, so `ein test` asks none.

**Held by.** `ein_infer::read_out` (`SAVED_CHECKS_NO_CLAIM`); `ein-cli/src/test.rs` (`NotRun`);
`ein-cli/tests/saved_files.rs`: `a_choice_is_saved_as_a_commitment_and_loaded_as_the_node`, the claims, the lines
and the report's rows with no run's data; `ein-cli/tests/einb_cli.rs`: `a_nodes_container_opens_as_the_node`. The page:
[10-answer-dump.md](../../ein/user/04-reference/10-answer-dump.md#a-saved-file-is-a-node-of-its-programs-lattice).
