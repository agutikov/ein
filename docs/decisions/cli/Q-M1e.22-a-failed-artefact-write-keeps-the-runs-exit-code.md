# Q-M1e.22 — A failed artefact write keeps the run's exit code

An option that writes a file beside the answer, `--events`, `--json-summary`, `--trace` or `test --json-report`,
may fail to write it. The run then prints `error: --<flag> <path>: <os error>` on stderr and exits as it would
have: 0 on a run that answered, the answer on stdout complete. A directory the run writes into, `--dump-answer`,
`--dump-lattice` or `oracle --dump`, is made before the run, and a run that cannot make it exits 1 without
solving. An empty path is a usage error, exit 2, refused before anything runs.

There is no exit code of its own for a lost artefact.

**Why.** The answer on stdout is correct and complete, and the file is an addition to it, which is what each of
those options' `--help` says: stdout, stderr and the exit code are unchanged. The exit codes are few and each
already carries a meaning: 0 every answer, 1 a refuted claim or a program refused (2 under `ein test`), 2 a usage
error or `--max-time` reached. A lost artefact would be one more meaning on one of them, or a new code that every script testing for a
non-zero exit would meet. It waits for a change to the exit codes as a whole; until then a consumer that needs
the file checks that it exists, or reads stderr, where the line names the option and the path.

**Held by.** `artefact_failed` (`ein-cli/src/solve.rs`), the one line every such failure prints;
`ein-cli/tests/artefact_contract.rs`: `a_failed_artefact_write_leaves_the_exit_code_alone_except_for_the_directories`,
`every_artefact_failure_names_its_flag_and_its_path` and `an_empty_artefact_path_is_refused`. The page:
[`defined_behaviour.md` § 4.4](../../kernel/defined_behaviour.md).
