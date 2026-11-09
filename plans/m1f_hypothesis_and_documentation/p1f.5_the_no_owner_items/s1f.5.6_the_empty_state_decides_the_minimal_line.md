# S1f.5.6 — Where ∅ is in R, every minimal-line question stops at root, and says only ∅

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.6**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).

**Status:** specified 2026-11-07 on the user's rulings of that day on S-16, which closes P-14 with it,
and on S-17 and F1 ([§ The clause](#s-17-and-f1-the-clause-goes)). **Running since 2026-11-09**, on the
user's go-ahead for P1f.5 of 2026-11-08 and the instruction of 2026-11-09 (*"just continue with default
decisions till the end of the phase"*): T1f.5.6.1, every move listed, is done ([its record](#t1-record-2026-11-09)), so are T1f.5.6.2, the rule
([its record](#t2-record-2026-11-09)), and T1f.5.6.3, the clause ([its record](#t3-record-2026-11-09)).
The coordinator handed the stage a fifth item the same day, S1f.5.16's Q-M1f.22, a class label glued to its
value ([§ Task T1f.5.6.5](#task-t1f565--a-label-never-touches-its-value-q-m1f22)).
**Depends on:** nothing. It touches `explore.rs` and the read-out, and none of the grammar stages do.

## The ruling

The agent put two choices for S-16: (a) leave `--minimal --single` and `--minimal -n N` weaker than
describe, or (b) use the ∅ rule in both, so that where root is alive and owes nothing, `--single` answers
*Solution* and `-n N` says ∅ is the whole minimal line, each after one saturation. The user, 2026-11-07:
*"b, use the ∅ rule in both"*.

## The rule

The minimal line is the members of R with no smaller member of R. ∅ is a subset of every commitment, so
**where ∅ is in R, the minimal line is {∅}**, and no other state can be on it. Whether ∅ is in R is root's
fixpoint alone: root alive and owing nothing, one saturation. The rule holds on every program, monotone or
not, within `--region-limit` or past it. It needs no assumption about what a node is
([Q-M1e2.2](../../m1e2_correctness/open_questions.md), F21), since ∅ is root itself.

## The case

Taken on 2026-11-07 at `916d97ac`, on the release binary: every `.ein` under `examples/`, `tests/`,
`stdlib/` and `corpus/`, 390 files, under `ein solve`, `--minimal -n 1`, `--minimal -n 2`,
`--minimal --single` and `--minimal`.

- **213 have ∅ in R**, by describe's `∅` row.
- **190 of them already answer at root.** Every minimal question costs one saturation and says so:
  `minimal line  1 state — all of them`, `exhausted  yes — this is all of the minimal line`, and
  `--single`'s `verdict  Solution — the minimal line is one state`.
- **The other 23 are the programs the static check flags**, answered by `flagged` in
  [`explore.rs`](../../../ein.rs/crates/ein-infer/src/explore.rs) (line 1358), which walks the cube in layer
  order and stops when its question is decided. It does not see that root decides it.
  - **All 23:** `-n 1` stops at root and says `1 state, as asked — the minimal line may hold more`.
  - **16 within the limit:** `-n 2`, `--single` and `--minimal` answer right, after 2, 4 or 8 saturations.
  - **7 past the limit:** `--single` says `not decided — the run stopped first` and `-n 2` says
    `1 state   (a lower bound: the run stopped first)`. `--minimal` lists ∅, from describe's partial
    description, and says `exhausted  no`. Each runs the cube to the limit:

| program | `--single`, `-n 2` and `--minimal`: saturations each |
|---|---:|
| `ein-bugs/wrong-answers/past-limit-lines` | 65 536 |
| `saturation/square-unique/corner-house` | 65 389 |
| `lattice/06_friends_tables` | 65 299 |
| `saturation/square-unique/terminus` | 65 029 |
| `saturation/square-unique/cul-de-sac` | 64 923 |
| `features/04_open` | 64 761 |
| `features/01_not_and_absent` | 64 655 |

Over the 213, each of the three questions cost 455 848 saturations. With the rule, 213.

## What it changes

**The route.** In `flagged`, before the layer walk: where the question is on the minimal line (`Which::Minimal`,
under any `Ask`) and root is alive and owes nothing, the answer is {∅}, complete for the line, after one
saturation. No other route moves, since the 190 already stop there.

**The words.** The 23 print what the 190 print:

| question | today, on the 23 | after |
|---|---|---|
| `--minimal -n N` | `1 state, as asked — the minimal line may hold more`; past the limit under `-n 2`, `1 state   (a lower bound: the run stopped first)` | `1 state — all of them`, `exhausted  yes — this is all of the minimal line` |
| `--minimal --single` | past the limit, `verdict  not decided — the run stopped first` | `verdict  Solution — the minimal line is one state`, `decided  yes — the minimal line holds no other state` |
| `--minimal` | past the limit, ∅ listed and `exhausted  no — not monotone, and the question needs the whole cube` | `1 state — all of them`, `exhausted  yes — this is all of the minimal line` |

`--minimal` is not named in the ruling, but it is the same question without a count, and the rule answers
it the same way. Describe's own row (`1 state — ∅, the trivial line: no obligation is stated`) words the
same line differently on 206 of the 213. That sentence is [S-17 and F1](README.md)'s, and this stage leaves
it alone.

**What else moves, and what does not.** On the 23, `--json-summary`'s verdict, `exhausted` and `stopped`,
`--trace`, and `--dump-answer`, which writes no other members met, since one node is met. No R moves, and
no state is listed that was not: every answer is still {∅}, as the oracle has it.

**The register.** W27's two minimal rows ask `--minimal` on two of the 23, past the limit, and record more
than the states. `W27/past-limit-lines-minimal` holds `exhausted = false` and `others = 2` in its `right` and
its `now`, and `W27/fifo-gap-limit-1-minimal` (`--region-limit 1`) holds `exhausted = false` and
`others = 1`. Their `right` is restated on this ruling in the same commit, `exhausted = true` and
`others = 0`, and both stay `fixed`. The register counts do not move.

**The corpus.** Two runs ask a minimal question on a flagged program: `fifo_gap`'s `solve --minimal
--region-limit 1` and `past-limit-lines`' `solve --minimal --region-limit 8`. Their `exhausted` and `work`
rows move. Step 1 lists every golden before it moves.

## S-17 and F1: the clause goes

**The ruling.** The agent put three choices: (a) drop the clause, (b) keep one made true, *root owes
nothing*, or (c) keep *no obligation is stated* only where the program states none. The user, 2026-11-07:
*"a, drop the clause"*.

**The case.** Taken on 2026-11-07 at `916d97ac`. Describe's minimal-line row reads
`1 state — ∅, the trivial line: no obligation is stated` where ∅ is in R and no node the run met owes
(`d.owing_met`, [`read_out.rs`](../../../ein.rs/crates/ein-render/src/read_out.rs) line 836), whatever the
program states:

| program | states an obligation | the row today |
|---|---|---|
| 8 `tests/stdlib/` programs, among them `algebra/24_total_owed_satisfied` | yes, and root discharges it | `1 state — ∅, the trivial line: no obligation is stated` |
| the inventory's F1 probe, [`examples/ein-bugs/minimal-line-stated-obligation.ein`](../../../examples/ein-bugs/minimal-line-stated-obligation.ein) since T1f.5.6.3 | yes, `(total-owed likes is-a)` | the same |
| its control, the same program without that line | no | the same |
| `past-limit-lines` at `--region-limit 1` | yes | the same |
| `past-limit-lines` at `--region-limit 2` | yes | `1 state — ∅` |

The `--json-summary` field `minimal.trivial` is read off the same bit
([`region_answer.rs`](../../../ein.rs/crates/ein-render/src/dump/region_answer.rs) line 250).

**What it changes.**
- **The row.** Where ∅ is in R, describe's minimal-line row reads `1 state — ∅` on every program, at every
  limit. The `∅` row above it, `in R — root is alive and owes nothing`, is the reason, and is unchanged.
- **The summary.** `minimal.trivial` reads *∅ is in R* (`root.in_r()`), so the field and the row agree.
  The field keeps its name and place, and the page that lists the summary's fields says what it means.
- **What stays.** The maximal line's clause, `⊤, the trivial line: no restriction bites`, is printed only
  on an exhausted run, where no node is dead in the whole cube, so it is true. No answer, R or register row
  moves.
- **What moves.** Describe's row on most of the 213 programs with ∅ in R (206 print the clause today), the
  two trace goldens that print it (`ein-render/tests/golden/trace_ambiguous.md`, `trace_unconditional.md`)
  and the read-out's golden page. Step 1 lists each.

## The method

### Task T1f.5.6.1 — every move listed

The sweep above re-taken on the build, with `--json-summary` and `--dump-answer`, and `cargo test
--workspace` run on a prototype. Every move must be one of the 23's rows above.

### T1 record, 2026-11-09

**Re-taken at `4aa9f3ef`**, after S1f.5.5's swap and S1f.5.16, on the release binary, by
`build/s1f.5.6/sweep.py` (gitignored, kept with its logs): every `.ein` under `examples/`, `tests/`, `stdlib/` and
`corpus/`, 423 files, under `ein solve`, `--minimal -n 1`, `--minimal -n 2`, `--minimal --single` and `--minimal`,
each with `--json-summary` and `--dump-answer`; and on every program the static check flags, the same five at
`--region-limit 1` and at `--region-limit 2`: 2 405 runs. The same sweep on a prototype of T2's rule and T3's clause,
and `build/s1f.5.6/diff.py` puts every run whose exit code, stdout, stderr, summary or any answer file moved in one
class.

**The case, today** (`python3 build/s1f.5.6/census.py build/s1f.5.6/base.jsonl`, 2026-11-09). 298 of the 423 files
answer a bare `ein solve`; the rest are refused at load or by *forbid*, or are stdlib modules.

- **211 have ∅ in R** (213 at `916d97ac`), by describe's `∅` row, the summary's `region.root = "in-r"`.
- **188 answer every minimal question at root**, one saturation each.
- **The other 23 are the 23 above**, every one flagged: the 16 within the limit spend 2, 4 or 8 saturations on `-n 2`,
  `--single` and `--minimal`, and the 7 past it the table's counts, unchanged to the node (65 536, 65 389, 65 299,
  65 029, 64 923, 64 761 and 64 655). Past the limit `--single` says `not decided — the run stopped first`, `-n 2`
  `1 state   (a lower bound: the run stopped first)` and `--minimal` `exhausted  no — not monotone, and the question
  needs the whole cube`; `-n 1` says `1 state, as asked — the minimal line may hold more` on all 23.
- **Over the 23, each of `-n 2`, `--single` and `--minimal` costs 455 658 saturations**, and over the 211, 455 846.
- **Describe's row prints the clause on 204 of the 211** (206 of 213 at `916d97ac`): 196 programs that state no
  obligation, and the 8 `tests/stdlib/` programs that state one and whose root discharges it
  (`algebra/24_total_owed_satisfied`, `algebra/26_surjective_owed_satisfied`, `bijection/06_blind_enumeration`,
  `closure/02_closed_and_satisfied`, `slots/01_partition`, `slots/08_spatial_adjacent`, `slots/10_owed_room_satisfied`,
  `slots/12_owed_fill_satisfied`). The other 7 print `1 state — ∅`: each a flagged program whose run met a node that
  owes. At `--region-limit 1` 21 of the 23 flagged programs print the clause, and at `--region-limit 2` 16.

**Every move, on the prototype** (`python3 build/s1f.5.6/diff.py build/s1f.5.6/base.jsonl
build/s1f.5.6/proto.jsonl`, 2026-11-09): 533 of the 2 405 runs move, every one in a class, and **0 runs in no
class**. No exit code moves.

- **The rule: 276 runs, on the 23 programs and no other**, every minimal question the sweep asks of each, the four
  at each of three limits. Each now lists ∅ alone, exhausted, and `--single` says `Solution`, after one saturation:
  over the 23, each of `-n 2`, `--single` and `--minimal` costs 23 saturations where it cost 455 658, and at
  `--region-limit 1` and `2`, 46 and 86. The rows that move are the words table's: the count row and `exhausted` on
  141 runs, `verdict` and `decided` on 36, `work` on 207. The `note: … partial` stderr line goes from the 108 runs
  that stopped at a limit. Beside them, **the warning's dynamic half**, because the run reads one node: its `state` row
  counts one node computed (207 runs), and the stale facts of the nodes it no longer meets go (33 runs); and on the 99
  runs within the limit whose walk had read the whole cube, the `status` row says ``not known — this run stopped before
  it read the cube's 2^n nodes: `ein oracle FILE` reads them, and so does a bare `ein solve FILE` ``
  where it gave the cube's reading: `monotone here (conservative)` on 69 and `non-monotone here (exercised, …)` on 30.
  The members met past the limit and not listed go (`region.states` on 10 runs, and `--dump-answer`'s state files,
  64 760 of them on `features/04_open`), and with them the `paths` row's count of them. Every answer is {∅}, as the
  oracle has it, and no R moves.
- **The clause: 257 describe runs, on the 211 programs**, every describe the sweep asks of a program with ∅ in R:
  the row loses `, the trivial line: no obligation is stated` on 241 (204 at the default limit, 21 at
  `--region-limit 1` and 16 at `2`), and `region.minimal.trivial` turns true on the 16 where it was false (the 7
  above, and 9 at the lowered limits). Nothing else of a describe moves.

**The goldens and tests that move** (`cargo test --workspace --no-fail-fast` on the prototype, 2026-11-09, and
`EIN_BLESS=1` on `corpus_shapes` to read its manifest's diff), each named before it moves:

- **T1f.5.6.2, the rule.** `ein-render/tests/golden/corpus_shapes.md5`: `read-out[minimal]` and `cores[minimal]` of the
  23 programs, 46 of its 13 678 renderings. `read_out_examples.md`'s X9 `--minimal` block, and the prose above it
  (`utils/read_out_examples.py`). The register's two W27 minimal rows, restated as above, and with them
  `ein-corpus`'s schema `/11` rule, which held a right that states `others` to `exhausted = false`: a line decided at
  root meets no member it does not list, so `others = 0` and `exhausted = true` (`the_register_is_well_formed`,
  `a_line_past_the_limit_counts_what_it_does_not_list`). `ein-render/tests/line_modes_past_the_limit.rs`, which held a
  minimal question past the limit to describe's row and to the members describe met: on the 23 it is held to the
  rule instead.
- **T1f.5.6.3, the clause.** `corpus_shapes.md5`: `read-out[describe]` and `region-trace` of 198 programs, and
  `cores[describe]` of the 7 whose `minimal.trivial` turns true, 403 renderings. The two trace goldens that print the
  clause, `ein-render/tests/golden/trace_ambiguous.md` and `trace_unconditional.md`. Five describe blocks of
  `read_out_examples.md` (X1, X2's describe and `-y`, X3 and X11); ten in fact, as T3 found when it wrote the page
  ([its record](#t3-record-2026-11-09)), the test's failure having listed the first.
- **Nothing else.** `corpus_cli`, `wrong_answers` (whose W27 rows answer the restated `right`), `explore`,
  `region_modes`, `lever_invariance` and every other test pass unchanged on the prototype.

### Task T1f.5.6.2 — the rule

The check in `flagged`, and three tests:
- **`ein-render/tests/line_modes_past_the_limit.rs`:** where describe says ∅ is in R, `--minimal -n N`,
  `--single` and `--minimal` list ∅ alone, decided, after one saturation. Today the file leaves `-n N` and
  `--single` out of its row check.
- **`ein-infer/tests/explore.rs`**, which holds every mode to the oracle: every minimal question on a
  program whose root is in R costs one saturation, on every route. That is *minimum work per question*,
  held where the corpus can break it.
- **`ein-cli/tests/region_modes.rs`:** the three rows of the words table, through the binary, on
  `past-limit-lines.ein`.

The register's two rows are restated in the same commit, and `ein-cli/tests/wrong_answers.rs` holds them.

### T2 record, 2026-11-09

**The rule** is `Space::empty_state` in [`explore.rs`](../../../ein.rs/crates/ein-infer/src/explore.rs), which
`flagged` takes before the layer walk and before the whole cube, for `Which::Minimal` under every ask, wherever root's
fixpoint owes nothing: root is alive there, a dead root having taken route 1 already. **Whether ∅ is in R is read off
root's fixpoint**, the obligations' tally `region::prepare_with` already took and threw away, now kept as
`Prepared::root_owes`, so a program whose ∅ is not in R takes exactly the route it took before, the whole cube
included, with no node computed twice. Where ∅ is in R, root's node is computed once, for what the read-out prints of
it (its closure, its stale facts, its narration), and the answer is {∅}, exhausted, `Solution` under `--single`;
root's node is root's fixpoint, so a node that disagrees is an engine defect, refused with a line that names `ein
oracle`. The monotone routes are untouched: `tree::minimal` has stopped there since M1e2 S1e2.1c.4.

**What moved** is T1's list for the rule and nothing else: `corpus_shapes.md5`'s 46 renderings, `read-out[minimal]`
and `cores[minimal]` of the 23 (`EIN_BLESS=1`, the manifest's diff read before it was kept); X9's `--minimal` block of
the examples page and its prose; and the register's two W27 minimal rows, restated with schema `/11`'s rule as T1
said. The live page, [`read_out.md`](../../../docs/kernel/inference/read_out.md) § 3, says it in the same commit: the
`verdict` and mode `minimal line` rows, and a paragraph under the table.

**The tests**, the three the task names:
- `ein-render/tests/line_modes_past_the_limit.rs` holds every minimal question past the limit, `-n 1`, `-n 2`,
  `--single` and `--minimal` alike, where describe at the same limit says ∅ is in R, to ∅ alone, decided, after one
  saturation, its row `1 state — all of them` or `Solution — the minimal line is one state`: 324 of its 800 runs on
  2026-11-09, held to at least 300.
- `ein-infer/tests/explore.rs`'s `every_mode_is_the_oracle` holds every minimal cell, five per program, on every
  program whose oracle has ∅ in R, to the same, on every route and at the C3 budget too (`empty_rule`): 835 monotone
  cells and 80 flagged, the five of each of 16 flagged programs, on 2026-11-09, held to at least 800 and 75.
- `ein-cli/tests/region_modes.rs`'s `where_empty_is_in_r_every_minimal_question_stops_at_root` holds the words table's
  three rows, `-n 1` and `-n 2` among them, through the binary on `past-limit-lines.ein` past the default limit, and
  that nothing is written to stderr.

**One choice, taken by default** on the user's instruction of 2026-11-09:
[Q-M1f.23](../open_questions.md#q-m1f23--what-the-route-row-names-where-root-decides-a-flagged-programs-minimal-line),
what the `route` row names where root decides a flagged program's minimal line: (a), the cube and `L7 the cube`, as
`-n 1` printed there before the rule, rather than a new licence. A second consequence is no choice, being the rule's
own: a run that reads one node does not read the cube, so within the limit the warning's `status` row says `not
known` on the 16 programs where the walk had read the whole cube and given its reading (T1's record), and names the
two commands that read it.

### Task T1f.5.6.3 — the clause (S-17, F1)

`minimal_line` in the read-out and `minimal.trivial` in the summary, as above, in their own commit. A test
in `ein-cli/tests/region_modes.rs` holds the five rows of the case to `1 state — ∅`, and the summary's field
to *∅ is in R*, on the F1 probe moved into `examples/` beside its control.

### T3 record, 2026-11-09

**The clause** is gone from `minimal_line` in [`read_out.rs`](../../../ein.rs/crates/ein-render/src/read_out.rs):
where ∅ is in R the row reads `1 state — ∅` on every program and at every limit, and the `∅` row above it says why.
**`region.minimal.trivial`** in [`region_answer.rs`](../../../ein.rs/crates/ein-render/src/dump/region_answer.rs)
reads `root.in_r()`, so the field and the row agree. `Described::owing_met`, the bit both read, stays: it is a fact
about the description, and `ein-infer/tests/obligation_reports.rs` reads it. The maximal line's `⊤, the trivial line: no
restriction bites`, printed only on an exhausted run where no node is dead, is true and stays.

**What moved** is T1's list for the clause, with one count corrected: `corpus_shapes.md5`'s 403 renderings,
`read-out[describe]` and `region-trace` of 198 programs and `cores[describe]` of the 7 whose `trivial` turned true; the
two trace goldens; and on the examples page **ten** blocks, not T1's five, its test having listed the first of them:
X1, X2's describe and `-y`, X3, X11, X12, X13, X14, X15 and X17's `ein oracle` on `lattice/02`, each the row's clause
and nothing else, with X1's prose, which called the line *trivial* because the program states no obligation. The live
page's describe `minimal line` row and its `region.minimal.trivial` field say what each reads now.

**The F1 probe and its control moved** from the inventory's findings to
[`examples/ein-bugs/minimal-line-stated-obligation.ein`](../../../examples/ein-bugs/minimal-line-stated-obligation.ein)
and its `-control`, each with a `:why` on its two relations and a header, so `--require-why` loads them; the inventory's
link and its re-take script follow them. Each is a corpus entry (`solve`, `saturate`): the exit table's four new
cells (`corpus_exits.txt`, `EIN_BLESS=1`) and 82 new renderings of `corpus_shapes.md5`, every one new and none moved.

**The test** is `ein-cli/tests/region_modes.rs`'s `the_minimal_line_row_names_empty_and_no_clause`: the case's five
rows, the eight `tests/stdlib/` programs, the probe, its control and `past-limit-lines` at `--region-limit 1` and `2`,
each held to `∅  in R — root is alive and owes nothing`, `minimal line  1 state — ∅`, `region.root = "in-r"` and
`region.minimal.trivial = true`; and `tests/stdlib/closure/03_closed_and_owing.ein`, whose R is empty and owing, to
`trivial = false`.

### Task T1f.5.6.4 — the pages

The live read-out reference, [`read_out.md`](../../../docs/kernel/inference/read_out.md) (S1f.5.16): its rows for `-n N` and `--single` past the
limit, its `minimal line` row (no name for {∅}) and its summary fields, in the commit that changes the output;
`solve_interface.md` § 3.3; and the read-out's golden page, re-written by `utils/read_out_examples.py`. M1e2's
`read_out.md` is a record, and is not edited. The inventory's S-16, P-14, S-17 and F1 say what
fixed them. The probe outputs under `p1e2.1b_non_monotone_programs/detector_proposal/outputs/` are a record,
and do not change.

### Task T1f.5.6.5 — a label never touches its value (Q-M1f.22)

**Handed to the stage by the coordinator on 2026-11-09**, from S1f.5.16's T1f.5.16.4, which found it and documented
it as [Q-M1f.22](../open_questions.md#q-m1f22--a-class-label-of-sixteen-characters-glues-to-its-value) without filing
or fixing it. The read-out pads a block's label to sixteen characters and prints the value straight after it, so a
label of sixteen characters or more touches its value: `examples/features/04_open.ein`'s `--dump-answer` `answer.txt`
prints `class 100 of 27728 states` for class 100 of 277, 28 states, and a reader reads a wrong count. Under the user's
rule a line that garbles a count is a wrong answer on a surface, so it is a register row before it is fixed. It touches
the dump's printer and nothing T1f.5.6.2 or .3 does, so it runs after them, in two commits, one per task:

- **T1f.5.6.5, the row.** W42, filed `open` with today's text as its `now`, and what the register test needs to hold a
  line the run prints, read off stdout or off `--dump-answer`'s `answer.txt`, under schema `/17`. Every other surface
  that pads a label to a width is checked by the same column logic, and one that glues is another witness of the case.
- **T1f.5.6.6, the fix.** The printer puts one space at least between a label and its value, at every width; every
  golden that moves is named first, and W42's rows flip in the same commit, `[case.fixed]` carrying `0000000`, which the
  next commit writes. Q-M1f.22 says it is fixed, and by which commit.

## Acceptance

- On every corpus program with ∅ in R, `--minimal -n N` for every `N`, `--minimal --single` and `--minimal`
  answer {∅}, decided, after one saturation.
- `--single` says *Solution* on the 7 past the limit, and `-n N` says ∅ is all of the minimal line on the 23.
- Every mode still agrees with the oracle within the limit, and with describe past it.
- Where ∅ is in R, describe's minimal-line row reads `1 state — ∅` on every program and at every
  `--region-limit`, and `minimal.trivial` is true exactly where ∅ is in R.
- No R moves. W27's two minimal rows have their `right` restated, and stay fixed.
- `./run_tests.sh` is green.
