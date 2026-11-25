# Q-M1f.454 — `ein test --diff` shows how a claim's models differ from what its run found, and decides nothing

`ein test --diff stats|full` follows the line of every claim a run decided, held or FAILED, with the claim's models
against what the run found, read as the claim's own check reads them: a region claim's against R's states, over the
relations the claim names, a listed `(not …)` read for its presence alone; an `(owing …)` claim's against the owing
frontier, *owing node i* in the read-out's order; a query claim's against its classes over the part its `:over`
names, marked *a part of R* where the claim was refuted from a part. A claim no run decided has no diff, nor has the
region's `(false)`, which lists no model. A `(merged …)` is not diffed: the diff is the models', so a claim FAILED by
its `(merged …)` alone has an all-match diff, and the claim's own lines say why.

**The pairing.** A **match** is a pair the check pairs, taken as a maximum matching, so a held claim's diff is all
matches and a model listed twice against one found item is one match and one missing. Then a model no found item
matched is **near** a found item no model matched, the nearest pairs first, while the two are at most half the
larger of them off, and at least one fact. How many facts off is an edit distance, the larger of the facts the
model lacks and the facts the found item has beyond it, so a fact replaced, `(in C 1)` for `(in C 0)`, is one fact
off, as a fact lacking or one too many is; ties go to fewer facts differing, then to the found items' order, then
to the models'. What is left is **missing**, each such model shown against the found item nearest it among all
found, with the facts that differ, and **extra**, each such found item naming the model nearest it and how far off,
without its facts, which are the read-out's. The near pairing's table is priced before it is allocated, against a
fixed bound of 16 MiB that is no option: past it nothing is paired near, no nearest is sought, and the block says
so.

**What prints.** `--diff stats`, or a bare `--diff` after the paths, prints what was compared and the counts: how
many models match, are near and how many facts off, are missing, and how many found items are extra. `--diff full`
adds each model in the claim's order, `=` matched, `~` near, `-` missing, with the facts that differ under each
pair, `-` the model's that the found item lacks and `+` the found item's beyond the model, then each extra found
item. A bare `--diff` before a path takes the path for its level and is refused, exit 2, naming the two levels. The
block follows the claim's own lines and precedes the warning and the *outside the universe* line printed beside
the claim. By default and under `-v` the line of
every claim with a diff prints, a held claim's too; under `-q` only the claims `-q` prints anyway show theirs.
`--json-report` writes `diff` on every row, one shape on every arm, its indices 1-based as printed, `items` `null`
under `stats`, and `diff` itself `null` without the option, on a claim no run decided and on the region's `(false)`.

**Why.** A FAILED claim says that its models and the run disagree, not where; a model one fact off reads best as the
fact that differs, not as a missing model beside an extra state. Read as the check reads the claim, the diff
explains a verdict and cannot contradict it: every verdict, every other line and the exit code are the same with
the option and without. A fact replaced is one mistake, so it counts once; the bound of half the larger keeps two
unrelated models a missing one and an extra one rather than a misleading pair; and a fixed bound on the pairing's
memory keeps an explanation from costing more than the run it explains.

**Held by.** `ein_render::claim_diff` (`diff_sets`, `lines`, `json`, `PAIR_TABLE_BOUND`) and its unit tests,
`a_fact_replaced_is_one_fact_off`, `matches_are_as_many_as_the_check_pairs` and
`unrelated_models_are_not_paired_near`; `ein_infer::claim::compared` and `ein_infer::read_out::query_compared`, what
the check compares; `ein-cli/tests/test_cli.rs`: `the_diff_moves_no_verdict_and_no_exit_code`, over every claim of
the corpus, `a_claim_one_fact_off_reads_as_one_fact_off`, `a_region_claim_names_the_missing_and_the_extra`,
`quiet_diffs_only_the_claims_it_prints` and `a_bare_diff_before_a_path_is_refused_with_its_levels`;
`ein-render/tests/diff_declines.rs`: `the_diff_declines_its_near_pairing_past_the_bound`. The page:
[05-cli.md § 2.3](../../ein/user/04-reference/05-cli.md#23-ein-test).
