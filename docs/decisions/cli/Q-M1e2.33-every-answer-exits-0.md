# Q-M1e2.33 — Every answer exits 0

`ein` exits non-zero only on an error. Every answer a program has is a successful run and exits 0: R empty, R of one
state or of many, a run a budget stopped with a part of R, and each of `--single`'s four words, `Solution`,
`Ambiguity`, `Contradiction` and `Open`. A claim the run could not check is a `note:` on stderr under `ein solve`,
and exits 0. What exits non-zero is an error: a claim the program states and the run refutes, 1; a program the
loader refuses, 1, and 2 under `ein test`; a usage error and a run `--max-time` aborts, 2. `ein test`, whose job is
to check claims, also exits 1 on a claim it could not check, except one it leaves unchecked under `--no-search`, and
2 on a file with nothing to check.

**Why.** An answer is not a failure of the tool: an empty R is what the program says, as much as one solution is.
Where a script needs a status from an answer, the program states the answer it expects as a claim, and a refuted
claim is the failure.

**Held by.** `ein-cli/tests/region_modes.rs`: `every_answer_exits_0_and_a_stopped_run_says_so`,
`describe_is_the_oracles_description_and_exits_0`; `ein-cli/tests/test_cli.rs`:
`a_false_claim_exits_one_and_names_the_fact`, `an_error_dominates_a_failure_in_the_exit_code`;
`ein-cli/tests/corpus_cli.rs`: `every_cell_reproduces_its_exit_code`, every corpus run's exit code banked.
