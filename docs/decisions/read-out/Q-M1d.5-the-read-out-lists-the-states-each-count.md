# Q-M1d.5 — The read-out lists the states, each count qualified

An answer of many states is listed, and every count says whether it is all of what was asked: an exhausted run
reports *the* states of R or of the line asked, and one that was not reports the states *found*, the part complete
up to where the `exhausted` row says.

**Why.** Every reader of an answer reads states, and the listing gives each with the commitment it was read from, in
Ein; a count with no qualifier would claim a completeness the run did not establish.

**Held by.** `ein-cli/tests/model_set_report.rs`: `an_exhausted_run_reports_the_states`,
`an_unexhausted_run_reports_states_found`. The rows:
[08-read-out.md](../../ein/user/04-reference/08-read-out.md#2-describe-a-bare-ein-solve---count-and-ein-oracle) § 2.
