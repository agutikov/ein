# Q-M1f.258 — `ein solve` and `ein test` take `--no-search`, and `ein oracle` refuses it

`--no-search` saturates root, reports it, and does not search R. Two commands take it:

- `ein solve`, beside every artefact and form flag (`--json-summary`, `--dump-answer`, `--dump-lattice`, `--trace`,
  `--events`, `--models`, …), each of which then reads root; beside a question about R or about the program (`-n`,
  `--single`, `--minimal`, `--maximal`, `--count`, `--monotone`) or `--describe-route`, it is a usage error, exit 2;
- `ein test`, which then decides only what root decides
  ([Q-M1f.257](Q-M1f.257-under-no-search-a-claim-not-checked-moves-no.md)).

`ein oracle --no-search` is a usage error, exit 2, whose line names `ein solve FILE --no-search`. `ein saturate`,
`ein render`, `ein show` and `ein kb` never search, and take no such flag.

**Why.** The oracle is R by brute force, the reference every mode is held to, so an oracle that does not compute R has
nothing to report. A question about R has no answer from root alone, so the parser refuses the pair rather than
answer another question.

**Held by.** `ein-cli/tests/no_search.rs`: `the_oracle_and_the_question_flags_refuse_it`,
`solve_reports_root_and_says_r_was_not_searched`, `the_summary_states_no_region_and_the_dump_writes_root`;
`ein-cli/tests/lever_invariance.rs`: `no_search_reports_root_as_the_full_run_reaches_it_and_states_no_r`. The row:
[06-config.md](../../ein/user/04-reference/06-config.md).
