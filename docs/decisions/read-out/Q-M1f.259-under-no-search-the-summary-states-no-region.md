# Q-M1f.259 — Under `--no-search` the summary states no region, and root's standing is in `root`

A `--json-summary` under `--no-search` is `ein-summary/2` with every key it has on every run: `asked.ask` is
`"no-search"`, `region` and `verdict` are `null`, and the queries and claims are as checked. Root's standing is three
keys of the `root` block, on every run whatever was asked: `dead`, `owes` and `owed`, read off root's fixpoint, where
every route starts. On a full run they equal the region block's `root`, `root_owes` and `root_owed`.

**Why.** A run that states no R has no region to report, and root's standing does not belong to a region: it is
root's, the same on every run. Adding the keys to `root` moves no other key, so the schema keeps its id.

**Held by.** `ein-cli/tests/no_search.rs`: `the_summary_states_no_region_and_the_dump_writes_root`;
`ein-cli/tests/lever_invariance.rs`: `no_search_reports_root_as_the_full_run_reaches_it_and_states_no_r`. The keys:
[read_out.md](../../kernel/inference/read_out.md) § 11.
