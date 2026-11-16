# Q-M1a.7 — A job count moves nothing a run prints

`--jobs N` is the computation `--jobs 1` is: every answer, count, core and line of narration, the event stream
included, is the same byte for byte at any job count. A job count reaches only a route's fan-out, the nodes of one
step computed on a pool of N threads. The default is 1; `--jobs auto` is the machine's count of logical CPUs, and
`--jobs 0` is refused. It is an option of the command and never a `(config …)` field, so no program sets its own
thread count. A build without the `parallel` feature accepts the option and computes every node on the calling
thread.

**Why.** An answer may not depend on the machine it was computed on. Holding the narration to the byte as well
leaves no room for a difference a scheduler made to pass as an equally valid explanation.

**Held by.** `ein-render/tests/jobs_invariance.rs`: `jobs_does_not_move_any_observable`, every rendering of every
corpus program that searches at one job count and at several, and `a_job_count_that_reaches_an_output_is_caught`;
`ein-infer/tests/explore.rs`: `every_mode_at_four_jobs_is_every_mode_at_one`; `ein-cli/tests/region_modes.rs`:
`the_event_stream_is_the_runs_and_jobs_move_no_line`; `ein-cli/tests/cli_semantics.rs`:
`jobs_takes_a_count_or_auto_and_nothing_else`. Why it holds:
[Q-M1e.23](Q-M1e.23-a-routes-workers-share-nothing-they-write.md).
