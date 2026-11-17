# Q-M1f.223 — `ein test` says which question decided each claim

Under `ein test -v` a query claim's line names the question before what the run found, spelt as `ein solve`'s flags
and `describe` for the bare run: *holds (--single: all of R, 1 class over 1 states)*, *holds (-n 4: …)*, and where
the least question left the claim undecided, *holds (describe, after -n 3: …)*. A solve claim's line names the route
describe took. A claim still not checked says which question was asked first and why it did not decide.
`--json-report` writes `ein-test-report/5`, every row carrying `route`, the question its outcome was read off, and
`fell_back_from`, the one asked first where it fell back, both `null` on a row no run checked. Where a file was asked
two questions, `--events` and `--json-summary` hold the last, the describing run.

**Why.** A claim is asked the least question that can decide it
([Q-M1f.221](../answer/Q-M1f.221-ein-test-asks-a-file-the-least-question-that.md)), which varies from file to file;
a reader of a verdict needs to know which run it was read off.

**Held by.** `ein-cli/tests/test_cli.rs`: `a_claim_is_asked_the_least_question_and_the_surfaces_say_which`,
`an_undecided_claim_falls_back_to_the_description`, `the_pages_name_the_reports_schema`. The event stream's run:
[05-events.md](../../ein/ein.rs/05-events.md).
