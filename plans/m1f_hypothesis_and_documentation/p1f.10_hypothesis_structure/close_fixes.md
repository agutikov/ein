# P1f.10 — the close's fixes (2026-11-16)

**Phase:** [P1f.10](README.md). The work [the close](close.md#the-users-answers-at-the-close-2026-11-16) waits on
besides the walk's last code and H9-1 and H9-4: the `lever_invariance` unit the container gate found running for
more than 45 minutes, the user's question about that test's granularity, and findings K-19 and H15-6, which the user
accepted to fix now. Done in one worktree, on `cb0fbde8`, in five commits: `37a72a9c` (K-19), `9bffeaf8` (H15-6),
`812aa93b` (§ 1, with `lever_invariance`'s units), `c79f4480` (the other sweeps' units) and this record's.

## 1. The `lever_invariance` unit that ran for 45 minutes

**Found.** `bb3579d1` took `examples/zebra.ein` out of the slow set, so the per-commit sweep of
`no_config_flag_moves_a_line_of_any_mode`, which skips `slow` entries, read it. The main session's probe: the cell
that never finished is `zebra.ein` with `:record-alternative-justifications` flipped on (its own `(config …)` sets it
off), question `--minimal`, inside `ein_infer::read_out::run`; the same cell through `ein solve COPY --minimal`
answers in about a second.

**The cause: the library's question took work the binary's does not.** The test asks each question through
`explore::Options::new(mode)`, whose `cores` were `Cores::ALL`, documented as *every core any surface prints: the
library's default*. `ALL.visited` takes an unsat core at every dead node a route visits, the core `--events`' `enter`
lines and `--dump-lattice`'s `nodes.json` print; a run with no watch has no reader for them. `ein solve` builds its
options from the same `Options::new` and then sets `cores` from the artefacts asked (`ein-cli/src/modes.rs`): with
none, `Cores::READ_OUT`, root's core and the cube's minimal holes', which is all the read-out and the summary print.
A core is an explanation search over the AND/OR proof graph. With alternatives recorded, `zebra.ein`'s graph is
densely re-derived, which is why its config sets the flag off (*"measured at ~22.7 s PER dead branch"*): each core
there runs to its environment cap. `--minimal` visits 318 nodes, 98 of them dead (`ein solve examples/zebra.ein
--minimal --events F`, the flag as written; the route is the same with it on), so the cell asked 98 such searches.
Measured through the binary on the copy with the flag on: `ein solve COPY --minimal` 0.29 s; the same with `--events
F` wrote five dead nodes' cores in 120 s, each `cut-environments`, and was stopped there; with `-T 10` it aborted at
29.7 s, inside its second core. So the in-process path did not differ from the binary's: it asked the binary's
`--events` question, of which the sweep reads nothing. **No answer moved and nothing is wrong**: every core taken is
an explanation, and no answer depends on which are taken (`ein-infer/tests/explore.rs`'
`a_run_that_prints_no_core_takes_none_and_answers_alike`).

**The fix, in the engine: `Options::new` asks what `ein solve` asks.** Its `cores` default is `Cores::READ_OUT`, so
`read_out::run(…, &Options::new(mode))` does the binary's work for the same question, and a caller that writes an
artefact printing another core, or reads a visited node's core through a watch, asks for it as the binary does.
`docs/api/rust.md` already said *`explore::Options::new(mode)` takes the defaults `ein solve` takes*; it was untrue of
`cores`, and the page's field table gains its row. Callers that print such cores now ask for them: `ein render
lattice` (`ein-cli/src/render.rs`, visited, as `--dump-lattice`), the shape golden's ops (`corpus_ops`, `ALL`, as
before, so no digest moves), `golden_trace.rs` (N's, as `--trace`), `region_render.rs` and `cli_semantics.rs`'
events stream (visited, as `--dump-lattice` and `--events`), `corpus_cores.rs` and `explore.rs`' eager run (`ALL`,
as before). `--dump-answer`'s and `--trace`'s writers refuse a member of N with no core, so a caller that forgets
fails loudly. The explanation search's own budget is untouched: its caps hold (each of those searches ended, cut at
its environments); what was unbounded in practice was the number of searches, one per dead node visited, which no
surface the run fed asked for. `ein solve COPY --minimal --events F` still takes that long, which zebra's config
block says and K-82 records.

**The test that pins it**: `lever_invariance.rs`' `the_librarys_question_does_the_binarys_work`. On `zebra.ein`
with the flag flipped on, every question of the sweep is asked in process through `Options::new` with a deadline of
a minute, which a core at each dead node passes within its first few nodes, and its read-out is held to `ein
solve`'s stdout and exit for the same question; and `Options::new`'s cores are the read-out's. It fails, rather than
hangs, if the default goes back. **Committed** as `812aa93b`, with `lever_invariance`'s units (§ 2). No answer, read-out,
summary or golden moved: the shape golden, the trace golden, `answer_round_trip`, `corpus_cores` and `explore.rs` pass
as they were.

## 2. The sweeps' granularity

**The user's question**, 2026-11-16: *"currently lever_invariance runs in a single thread, we already parallelized
tests, why not this?"* **The answer**: it was on the pool, one unit per (program, setting), each unit asking its
program its sixteen questions in turn. The one thread the container gate showed was the hang of § 1: the unit of
`zebra.ein` with the flag on, every other unit long done. With the hang fixed the sweep is not held by its tail at
sixteen workers: its units summed 1 212 s and the largest was 44 s (`count-past-2-128.ein` under the spot check,
which the pool starts first), against 76 s of work a worker. But one program could still hold one worker for a whole
setting, so the unit is now a (program, setting, question), and the same for every sweep of that shape:

| sweep | the unit was | the unit is |
|---|---|---|
| `lever_invariance`' `no_config_flag_moves_a_line_of_any_mode` | (program, setting): 16 questions | (program, setting, question) |
| `lever_invariance`' `the_describe_route_moves_no_part_of_r` | a program: 4 runs | (program, question, route) |
| `lever_invariance`' `no_search_reports_root_…` | a program: 2 runs | (program, run) |
| `import_invariance` | a program: 3 forms, their `--single` | the program as written, then (program, form) |
| `tree_classes`' `every_field_both_routes_compute_is_one_on_the_corpus` | a program: 4 descriptions | (program, description) |
| `first_contradiction` | a searching program: 6 questions × 2 | (program, question, with the stop or not) |
| `known_dead`' `every_question_answers_alike_with_the_denials_and_from_deaths_alone` | the same: 12 runs | (program, question, run) |
| `agenda` | a program: 5 questions × 2, then its claims × 2 × 2 | four passes: load, first question, the rest, the claims |

Each sweep folds its results back per program in the units' order, so every verdict, count and message reads as it
did, the pool's contract: the `EIN_CORPUS_SLOW=1` runs below printed the same tallies before and after, line for
line. No work is added, but in `tree_classes`, where a route does not take a program its other two descriptions are
asked all the same and end at its root (840 units, 16 s of some 440 s on the slow sweep), and in `agenda`, which
waits for each program's first question before it asks the rest, as the serial loop did, so that a flagged program
is read no further. A two-pass `tree_classes`, asking the second pair only where the first was taken, did 15 % less
CPU and took 57 s against 38 s: the pass waits for the first one's tail. Not split: `explore.rs` (its four sweeps
summed 2 209 s of units, 138 s a worker, and no program's unit is the tail), `claim_routes` (a unit is one oracle cube
every claim of the program is read off), the oracle sweeps of `known_dead` and `tree_classes` (a unit is one cube),
`order` and `route_invariance` (split already), and the calculator's batches (a unit is one program of every
request).

**The walls.** The `cargo test` binaries before (`cb0fbde8` with § 1's fix, its units unsplit) and after (`c79f4480`),
each test run alone, one after the other, at the gate profile with the default sixteen workers; the machine on AC
(`utils/bench_env.sh --report`: i9-14900HX, 16 P + 16 E threads, `powersave`, turbo on) and **not idle**: two other
agents' test runs were loading it, the load average beside each pair. A wall here is good to a few seconds at best;
the largest unit, what the split is about, is read off the pool's instrument (`EIN_TEST_UNITS`), per commit in the
whole runs below and with `EIN_CORPUS_SLOW=1` in the runs of the table.

Per commit:

| test | before | after | load | largest unit, before → after |
|---|---|---|---|---|
| `lever_invariance` `no_config_flag_…` | 99 s | 95 s | 34–57 | 44.0 → 5.3 |
| `lever_invariance` `the_describe_route_…` | 13 s | 10 s | 35–40 | 15.4 → 4.4 |
| `lever_invariance` `no_search_…` | 4 s | 4 s | 32–35 | 7.4 → 3.8 |
| `lever_invariance`, the binary (after: + § 1's test) | 97 s | 100 s | 30–52 | |
| `import_invariance` | 13 s | 13 s | 38–52 | 22.4 → 4.4 |
| `tree_classes` `every_field_…` | 9 s | 8 s | 34–38 | 7.1 → 2.8 |
| `first_contradiction` | 16 s | 8 s | 29–34 | 9.9 → 3.6 |
| `agenda` | 21 s | 20 s | 28–30 | 12.2 → 2.6 |
| `known_dead` `every_question_…` | 10 s | 18 s | 26–39 | 26.0 → 3.7 |

With `EIN_CORPUS_SLOW=1`:

| test | before | after | load | largest unit, before → after |
|---|---|---|---|---|
| `lever_invariance` `every_declared_lever_cell_…` (units unchanged) | 0.6 s | 1.0 s | 32 | 0.3 → 0.6 |
| `import_invariance` | 51.3 s | 44.8 s | 22–32 | 50.8 → 21.4 |
| `tree_classes` `every_field_…` | 50.0 s | 43.6 s | 12–22 | 50.0 → 34.7 |
| `first_contradiction` | 182.2 s | 55.1 s | 19–22 | 182.2 → 35.8 |
| `agenda` | 86.0 s | 81.4 s | 16–33 | 85.9 → 38.0 |
| `known_dead` `every_question_…` | 121.6 s | 54.9 s | 15–33 | 121.5 → 34.8 |

And each binary's wall inside a whole run of the test crates (`cargo test --profile gate -p ein-infer -p ein-cli
-p ein-render -p ein-corpus`, `ein-einb` too before; sixteen workers), before at load 15 to 50 and after at 15 to 28: `lever_invariance` 107.4
→ 70.6 s, `import_invariance` 22.8 → 14.9 s, `known_dead` 28.1 → 14.0 s, `first_contradiction` 9.9 → 8.3 s,
`agenda` 23.0 → 22.8 s, `tree_classes` 13.7 → 17.6 s (its oracle sweep, unsplit, has a 13.7 s unit of its own).

So the slow halves gain most, where one program (`zebra2-minus-15` or its obligations twin) was a sweep's whole tail:
`first_contradiction` takes under a third of its time and `known_dead` under half, and every slow sweep's largest unit
is now under 40 s. Per commit, `import_invariance`, `first_contradiction`, `known_dead` and `tree_classes` were held by
one program each (a largest unit past the sweep's work a worker), and are not now: every per-commit sweep's largest
unit is under 6 s. `lever_invariance`' flags sweep and `agenda` were held by their work, which the split does not
change, and their walls moved within the noise of a loaded machine. The nightly's steps do the same work as before,
spread finer, so a shard's wall can only drop; they were not re-taken on four P-core threads, the machine being
loaded.

## 3. K-19 and H15-6

**K-19: a support's check dropped where root's layer passed its budget.** `structure::theory_in` read each excluded
pair's own node for the check through `Space::eval`, which asks `afford(0)` and refuses even a step of no new node
once `spent` is past the budget; a layer's last step is afforded by a projection and may carry it there, and the
refusal was taken for an empty list, so every `Support::check` stayed `None` and nothing said so. **The fix** is the
finding's: the nodes are read off the cache, `Space::cached`, which asks no budget, as K-17's holders are; and where
a pair's node is not in the cache, which a layer that excluded the pair cannot leave, each read support's check is
`unread` with the reason, never left out. **The test**: `ein-infer/tests/theory_share.rs`'
`a_supports_check_is_read_where_roots_layer_passed_its_budget`: on the one-relation zebra, one fact under what root's
layer spends, the layer reads every pair, every support is checked, each check is the default budget's, and the
readings are. Closed by `37a72a9c`; the test fails with the fix taken out (*a support left unchecked*).

**H15-6: the register's bless wrote a region block into a quoted note.** `register_region.rs`' `bless` placed a block
before the first occurrence of the text `[case.right]` in a row, a note's included. **The fix**: both tables, the
`[case.region]` it replaces and the `[case.right]` it writes before, are found by their header line, a line that is
exactly the header (`header_at`). **The test**: `the_bless_writes_before_the_right_table_and_not_into_a_note`, a
row whose note names both tables, with no region block and with one: the note stays whole, the block lands on the
line before the `[case.right]` header, an old block is replaced, the next row is untouched, and the text parses as
TOML (`toml` is now a dev-dependency of `ein-infer` for it). Closed by `9bffeaf8`.

## Findings

K-81 and K-82 in [`findings.md`](findings.md): the oracle's library defaults still take every core, and an
explanation search's budget bounds its environments, rounds and facts but not its work.

## Verification, on the final tree

- `cargo test --manifest-path ein.rs/Cargo.toml --profile gate -p ein-infer -p ein-cli -p ein-render -p ein-corpus`:
  green, `lever_invariance` included (70.6 s).
- `EIN_CORPUS_SLOW=1`, the same, for `lever_invariance`, `import_invariance`, `tree_classes`'
  `every_field_both_routes_compute_is_one_on_the_corpus`, `first_contradiction`, `agenda` and `known_dead`'
  `every_question_answers_alike_with_the_denials_and_from_deaths_alone`: green (77.6, 36.5, 38.0, 49.5, 74.6 and 73.5 s, at load 15 to 40).
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `RUSTDOCFLAGS="-D warnings"
  cargo doc --workspace --no-deps`, `utils/check_hashmap_iteration.py` and `utils/doc_audit.py --links --check …`:
  clean.
- Not run, as asked: `./run_tests.sh`, the nightly, `utils/ci_container.sh`.

**What did not move**: no answer, count, claim, read-out, summary or golden. The changes to what a run computes are
`Options::new`'s cores, which no answer depends on, and K-19's check, which only the census example and its test ask.
