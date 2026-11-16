# Q-M1d.5 — The read-out lists the states, each count qualified, and the key is a form on request

An answer of many states is listed, and every count says whether it is all of what was asked: an exhausted run
reports *the* states of R or of the line asked, and one that was not reports the states *found*, the part complete
up to where the `exhausted` row says. A compact description is a form asked for after the read-out:
`ein solve --models key` prints the smallest set of slots that tells the listed states apart and the table of the
combinations that occur, and declines, with the reason, where the key is unaffordable, leaving the listing as the
form. It adds lines to stdout and reaches nothing recorded: no count, state, claim or exit code moves with it.

**Why.** Every reader of an answer reads states, so no compact form may be one only the engine can read. A key's
rows are read off states that exist, so a state the run did not find can add a row but never contradict one; a count
with no qualifier would claim a completeness the run did not establish.

**Held by.** `ein-cli/tests/model_set_report.rs`: `an_exhausted_run_reports_the_states`,
`an_unexhausted_run_reports_states_found`, `the_key_form_changes_stdout_and_nothing_recorded`,
`the_key_table_names_its_size_its_alternatives_and_one_row_per_state`, `an_unaffordable_key_says_why`. The rows:
[read-out.md](../../ein/user/reference/read-out.md#2-describe-a-bare-ein-solve---count-and-ein-oracle) § 2 and § 8.
