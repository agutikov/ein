# Q-M1f.100 — An artefact written to a closed pipe ends the process too

An artefact flag that names a pipe whose reader has gone, `--events /dev/stdout` under `| head` or a FIFO nobody
reads any more, ends the process by `SIGPIPE` at that write, as stdout does
([Q-M1f.68](Q-M1f.68-a-closed-stdout-ends-ein-by-sigpipe.md)). Every other failure of an artefact's write, a
directory, a full disk, a permission, prints `error: --<flag> <path>: <os error>` on stderr and leaves the exit code
alone, but for the directories a run makes before it starts.

**Why.** A signal's disposition is the process's, not a write's: it reaches every pipe the process writes. Flipping
it around each artefact's writes would not be thread-safe and would need more `unsafe` code for a case no run of the
corpus reaches.

**Held by.** `ein-cli/tests/closed_stdout.rs`: `a_reader_closed_before_the_run_ends_every_run_by_sigpipe`, which runs
the artefact flags on `/dev/stdout`; `ein-cli/tests/artefact_contract.rs`:
`every_artefact_failure_names_its_flag_and_its_path`,
`a_failed_artefact_write_leaves_the_exit_code_alone_except_for_the_directories`. The page:
[07-diagnostics.md](../../ein/user/04-reference/07-diagnostics.md#24-an-artefact-flags-failure) § 2.4 and § 4.19.
