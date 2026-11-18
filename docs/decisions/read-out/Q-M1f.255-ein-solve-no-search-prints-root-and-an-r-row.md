# Q-M1f.255 — `ein solve --no-search` prints root, and an `R` row that says it was not searched

Under `--no-search` the read-out is root's: `asked` (*root alone*), `∅` and its `owed` block as describe prints them,
an `R` row, *not searched — --no-search builds no node past root*, the static check's rows, `universe` (H built,
*none visited*), `route` (*none*) and `work` (one saturation); a dead root's core; the queries, read at ∅ where ∅ is
in R; and the claims, each decided where root decides it and not checked otherwise. The `R` row reads the same on a
dead root. Root's facts are drawn by the forms (`-p`, `-P`, `-f`) and written as
`root.ein` by `--dump-answer`; stdout does not list them.

**Why.** The flag declines to discover R; it does not report another one, so every row that counts or lists R gives
way to one that says R was not searched. The commands that take it:
[Q-M1f.258](../cli/Q-M1f.258-ein-solve-and-ein-test-take-no-search-and-ein.md).

**Held by.** `ein-cli/tests/no_search.rs`: `solve_reports_root_and_says_r_was_not_searched`;
`ein-cli/tests/lever_invariance.rs`: `no_search_reports_root_as_the_full_run_reaches_it_and_states_no_r`. The rows:
[08-read-out.md](../../ein/user/04-reference/08-read-out.md#4---monotone-and---no-search-no-question-about-r) § 4.
