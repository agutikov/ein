# Q-M1d.1 — A search stops early only on a decided question or a bound

A run classifies commitments until its question is decided, and no further:

- **Its question decided.** `--single` stops at a second state or where the part is shown empty, `-n N` at the
  N-th state, `--minimal` and `--maximal` at their line; each stop rests on a property the route proves, such as
  *a dead set's every superset is dead* on a monotone program, never on a guess. A bare `ein solve` asks for all
  of R, so it stops only when every commitment is classified, and says `exhausted yes`.
- **A bound.** `--region-budget`, or `--region-limit` on a flagged program, stops a run before its question is
  decided, and the run says so: `exhausted` or `decided` reads *no* with the reason, the description is partial
  and bounds what it did not certify, a stderr `note:` gives what was spent and what the next step would cost,
  and a claim the run could not decide is NOT CHECKED rather than held or refuted. `--max-time` ends a run that
  has not decided its question, with nothing on stdout.

No run stops on a heuristic, *no new state for k layers* or the like, and none says `exhausted` of a part it did
not classify. A criterion that would certify all of R sooner than the routes do, on purpose and for a class of
program, is not used; one is adopted only with the proof that licenses it.

**Why.** An answer is a statement about R, which [`ein oracle`](../../ein/reasoning/02-kb-evolution/07-finding-r.md#2-the-oracle-the-cube)
computes by brute force, and every mode must agree with it. A stop that rests on a proof keeps that agreement; a
stop on a guess would print a different answer as if it were the same one, and a bounded run that did not say it
was bounded would too.

**Held by.** `ein-infer/tests/explore.rs`: `every_mode_is_the_oracle`, every mode of `ein solve` against the
oracle's region on every corpus program it enumerates, and a stop inside the oracle's limit a failure;
`ein-cli/tests/test_cli.rs`: `a_budget_stopped_run_is_not_checked_and_says_why`; `ein-cli/tests/cli_semantics.rs`:
`a_stopped_run_writes_its_summary_and_says_why`. The page: [`08-read-out.md`](../../ein/user/04-reference/08-read-out.md),
the rows `exhausted` and `decided`.
