# The walk's last census — 2026-10-16

**T1e2.1a.10.1's measurement**, for
[S1e2.1a.10](s1e2.1a.10_the_register_under_r.md). It compares the walk's
answer with R on every cell the repo **declares**, while the walk can still
be run. S1e2.1a.5's switch removes the last path to it, so a disagreement
this page does not find is lost to the record. Its predecessor,
[`walk_and_region.md`](walk_and_region.md), ran the walk six ways of its own
choosing and explained *why* the two differ. This page asks *where*, cell by
cell, and which register case each disagreement belongs to.

```sh
cargo build --release -p ein-cli
cargo run --release -p ein-infer --example walk_census -- --jobs 16 > cells.jsonl
```

Taken 2026-10-16 on `a9d4013a`, release build. Its engine sources are
`06d62a7c`'s: the two commits since add an example and plan pages. The run
writes one JSON line per cell on stdout and the census on stderr, and the
tables below were read off both.
[`walk_census.rs`](../../../ein.rs/crates/ein-infer/examples/walk_census.rs)
is the example. R comes from
[`shared/known.rs`](../../../ein.rs/crates/ein-infer/examples/shared/known.rs),
which `walk_vs_region` now shares.

---

## 1. What it measures

**The cells.** A cell is one file, one run and one environment. Cells are
deduplicated on those three, which gives **465 cells on 220 programs**. They
are every way the repo declares a walk:

- every `solve` run and every `levers` cell `corpus/corpus.toml` lists
  (`Entry::all_runs`). Runs that carry a region flag are left out, because
  they are not the walk;
- every `test` run, as the `solve -e` walk that `ein test` makes: no stop,
  `-m 5` ([`test.rs`](../../../ein.rs/crates/ein-cli/src/test.rs)). This adds
  57 cells, and all 57 agree;
- every run a `corpus/wrong_answers.toml` row records, with its `env`: the
  wrong run, the control and the twins.

**R is known for 402 cells on 161 programs**, by `walk_vs_region`'s rule:
route 1 under 2²⁰ visits, or the oracle at n ≤ 16 on a flagged program. The
161 are 147 monotone programs, 13 flagged ones, and one program whose only
cell stops before it writes a summary (§ 4). The other 63 cells, on 59
programs, have no R:

- 52 negative fixtures that do not load or compile (41 load-, 7 compile- and
  4 parse-negative), and two regression fixtures the compiler refuses,
  `unbound-assert-var` and `unbound-relation-head`;
- three flagged programs over n = 16: `features/01_not_and_absent` (35),
  `features/12_expect_false` (20) and `lattice/06_friends_tables` (19);
- `zebra2-minus-15.ein` and its `-obligations` twin, which are past route 1's
  budget.

**A cell's answer is the CLI's.** The census runs `$EIN_BIN` with the argv
the corpus uses ([`plan::argv`](../../../ein.rs/crates/ein-corpus/src/plan.rs)),
then reads back its `--json-summary`. It also runs the walk in-process with
the cell's own flags and holds that run to the CLI's. **0 of the 400
compared differ.** The 401st cell runs under `EIN_TRAVERSAL=tree` and has no
in-process run. So the
attribution in § 3 is about the run the CLI made.

**How a cell is judged.** The walk's question is R's maximal line
([`walk_and_region.md` § 3](walk_and_region.md#3-where-they-are-equivalent)).
A cell **agrees** when all three hold:

- every recorded model is on the maximal line, and there are as many as the
  run asked for: `min(N, |max|)` under `-n N`, and every one under `-e`;
- the verdict's kind matches R. That means a model when R is non-empty;
  `Open` when R is empty and an alive node owes; `Contradiction` otherwise;
- when R is empty, the open states are R's maximal owing nodes.

A state is placed on R by its **closure** on a monotone program. On a flagged
program it is placed by its positive facts beyond root's, against the
oracle's own states.

**How a disagreement is attributed.** The cell is re-run in-process, and the
steps are tried in this order:

| step | the walk re-run | if it now agrees, or the test holds |
|---|---|---|
| 1 | with no `-n`, `-m` or `-E` (`-e -m n`) | reason 2, *the cuts* |
| 2 | step 1 with nothing written: no forced positive, kill cache or singleton writeback | reason 3, *a different node* |
| 3 | step 2, on a flagged program, where a missing maximal state was never entered alive | **`prune`** |
| 4 | step 2, on a program that states an obligation | reason 5, *the obligations rung* |
| 5 | otherwise | reason 4, *`complete` is approximate* |

The reasons are
[`walk_and_region.md` § 1](walk_and_region.md#1-the-short-answer)'s. Two
notes on the steps:

- **`prune` is not one of the five.** Those were measured on monotone
  programs, and § 4.6 there left the flagged ones to P1e2.1b. On a flagged
  program, the walk's skip of every superset of a dead node is the step no
  licence covers. A missing state that was never entered is what that skip
  looks like from outside.
- **A cell with two mechanisms is labelled by the first step that clears
  it.** `zebra2-hints` has reasons 3 and 4
  ([§ 4.3 there](walk_and_region.md#43-a-different-node)), and it is labelled
  4, because removing the writes alone does not give its second state back.

A sixth label was checked and **never fired**: a missing maximal state that
the walk judged complete and `record_node` then refused, once the
completeness check's `(not h)` was closed over (M1e `CO-M1`).

## 2. The answer

| | cells |
|---|---:|
| agree, and R is its maximal line | 220 |
| agree, and R holds more than its maximal line (reason 1, by design) | 122 |
| **disagree** | **59**, on 33 programs |
| stopped before a summary | 1 |
| **R known** | **402** |

127 of the 160 programs with a summary agree on every cell. The 122
reason-1 cells are on 41 programs: the ruling of 2026-10-16, *the default
describes R*, and not a defect.

**No recorded model is outside R** (C1), on any of the 401 cells. Every
disagreement either misses a state of R, records a state of R that is off
its maximal line (W3, W5, W6), or prints a word R contradicts.

| mechanism | cells | programs | a defect? |
|---|---:|---:|---|
| 2, the cuts | 7 | 6 | no: every one says `exhausted = false` (§ 4) |
| a declared `-E 1` abort | 1 | 1 | no |
| 3, a different node | 4 | 3 | yes |
| 4, `complete` is approximate | 25 | 15 | yes |
| 5, the obligations rung | 10 | 5 | yes |
| `prune`, on a flagged program | 11 | 4 | yes |
| under `EIN_TRAVERSAL=tree`, not attributed | 1 | 1 | yes |

**The levers.** `-K`, `-y` and `-o score-sum` agree on every cell they
declare: 13, 8 and 8. `-L` disagrees on 5 of 12 `solve -L` cells and on all
3 `solve -e -L` cells.

## 3. Every disagreement, and its case

Of the 51 defects, 22 cells are a register row's own run (§ 3.1), 17 are
another cell on a program that already has a case (§ 3.2), and 12, on 8
programs, have no row anywhere (§ 3.3).

### 3.1 The row exists (22 cells)

| cell | reason | the register's answer |
|---|---|---|
| `fuzz_findings/unsat-core-id-space.ein` `solve -e` | 3 | W11/unsat-core-id-space wrong |
| `ein-bugs/complete-records-stale.ein` `solve -e` | 3 | **W9/complete-records-stale-K control** |
| `ein-bugs/alive-empty-phase1.ein` `solve -e` | 3 | **W9/alive-empty-phase1-K control**, W12/alive-empty-phase1 wrong |
| `branching/15_lookahead_two_step_on.ein` `solve -e` | 4 | W1/branching-15 wrong, W1/branching-16 control |
| `branching/16_lookahead_two_step_off.ein` `solve -e` | 4 | W1/branching-16 wrong |
| `lattice/02_genuine_3set_death.ein` `solve -e -L` | 4 | W1/lattice-02-L wrong |
| `wrong-answers/forced-2-off.ein` `solve -e` | 4 | W1/forced-2-off wrong, W2/forced-2 twin 1 |
| `wrong-answers/forced-2-two.ein` `solve -e` | 4 | W1/forced-2-two wrong |
| `wrong-answers/fresh-name-nospawn.ein` `solve -e` | 4 | W1/fresh-name-nospawn wrong, W4/alive-set-fresh-name twin 1 |
| `wrong-answers/forced-2.ein` `solve -e` | 4 | W2/forced-2 wrong |
| `wrong-answers/forced-1.ein` `solve -e -L` | 4 | W2/forced-1-L wrong |
| `ein-bugs/alive-set-fresh-name.ein` `solve -e` | 4 | W4/alive-set-fresh-name wrong |
| `fuzz_findings/d3-unsat-core.ein` `solve -m 2` | 4 | W11/d3-unsat-core wrong |
| `wrong-answers/owed-two-step.ein` `solve -e` | 5 | W1/owed-two-step wrong |
| `wrong-answers/owed-seats.ein` `solve -e` | 5 | W5/owed-seats wrong |
| `wrong-answers/owed-seats-renamed.ein` `solve -e` | 5 | W5/owed-seats twin 1 |
| `wrong-answers/owed-seats.ein` `solve -e`, `EIN_TRAVERSAL=tree` | tree | W5/owed-seats twin 2 |
| `wrong-answers/uncovered.ein` `solve -e` | 5 | W6/uncovered wrong |
| `wrong-answers/uncovered-knows.ein` `solve -e` | 5 | W6/uncovered twin 1 |
| `ein-bugs/naf-upward-closure.ein` `solve -e` | prune | W3/naf-upward-closure wrong |
| `wrong-answers/naf-nowb.ein` `solve -e -L` | prune | W3/naf-nowb-L wrong |
| `wrong-answers/naf-nowb.ein` `solve -e` | prune | W3/naf-nowb-L control |

What the table adds to the rows:

- **W9's two controls are the wrong answers under R.** Their `-K` runs, the
  rows' `wrong`, agree with R. So do W3/naf-upward-closure's `-K` control and
  every other `-e -K` cell. That is the reversal S1e2.1a.10 makes, measured.
- **W3/naf-nowb-L's control** is W3's wrong model, which the row's `why`
  already says.
- **W11/d3-unsat-core's run is wrong in its verdict, not only in its core.**
  `solve -m 2` says `Contradiction` with `exhausted = false`, and R has 20
  states, two of them maximal. The cut is not the reason: with no cut the walk
  still misses one of them.
- **W1/owed-two-step is labelled 5** because the program states an
  obligation, and step 4 asks only that. Its mechanism is W1's, as
  [`walk_and_region.md` § 4.4](walk_and_region.md) has it: R is empty, its
  one maximal alive node owes, and the walk says `Contradiction` where the
  word is `Open`.

### 3.2 A case exists on the program (17 cells)

Each of these is a `solve` (`-n 1`) or `solve -L` cell beside a row's
`solve -e` or `solve -e -L`. The mechanism is the same, at the run's own
stop. **Proposed: a twin on the case**, so that every disagreeing cell is in
the register before the switch moves it.

| program | cells | reason | case |
|---|---|---|---|
| `branching/16_lookahead_two_step_off.ein` | `solve` | 4 | W1 |
| `wrong-answers/forced-2-off.ein` | `solve` | 4 | W1 (W2) |
| `wrong-answers/forced-2-two.ein` | `solve` | 4 | W1 |
| `wrong-answers/fresh-name-nospawn.ein` | `solve` | 4 | W1 (W4) |
| `lattice/02_genuine_3set_death.ein` | `solve -L` | 4 | W1 |
| `wrong-answers/owed-two-step.ein` | `solve` | 5 | W1 |
| `wrong-answers/forced-2.ein` | `solve` | 4 | W2 |
| `wrong-answers/forced-1.ein` | `solve -L` | 4 | W2 |
| `ein-bugs/naf-upward-closure.ein` | `solve` | prune | W3 |
| `wrong-answers/naf-nowb.ein` | `solve`, `solve -L` | prune | W3 |
| `ein-bugs/alive-set-fresh-name.ein` | `solve` | 4 | W4 |
| `wrong-answers/owed-seats.ein`, `owed-seats-renamed.ein` | `solve` | 5 | W5 |
| `wrong-answers/uncovered.ein`, `uncovered-knows.ein` | `solve` | 5 | W6 |
| `ein-bugs/alive-empty-phase1.ein` | `solve` | 3 | W9 (W12) |

### 3.3 No row on the program (12 cells on 8 programs)

| program | cells | the walk | R | mechanism | proposed case |
|---|---|---|---|---|---|
| `examples/zebra2-hints.ein` (n = 36) | `solve -e` | `Solution`, k = 1 | k_s = 6, maximal line 2 | 4, and 3 | W1, a new row: W1's shape plus the walk's node |
| `examples/branching/07_lookahead_off.ein` (n = 204) | `solve`, `solve -e` | `Contradiction`, `exhausted = false` | k_s = 74, maximal line 22 | 4, with the file's own lookahead off | W1, a new row: W1 under `-L` |
| `examples/ein-bugs/alive-empty-interlayer.ein` (n = 3, flagged) | `solve`, `solve -e` | `Contradiction`, `exhausted = true` | k_s = 4, maximal line `{(p A),(p B)}` | 4: `(p Z)` dies only after two firings (`taint`, then `kill`), so `{(p A),(p B)}` is entered alive and never complete. With the writes on, the singleton writeback's `(not (p Z))` also refutes it through `mutex` | W1, a new row, in W9's direction: its `:expect (false)` states the walk's answer |
| `examples/lattice/synth/three_components.ein` (n = 7, flagged) | `solve -e`, `solve -e --dump-region` | `Solution`, k = 1 | k_s = 13, maximal line 3 | prune: the two missing states are never entered. The sites are `absent` in two `(false)` rules, and the cube has 3 holes | W3, a new row |
| `examples/lattice/synth/trigger_hole.ein` (n = 5, flagged) | `solve`, `solve -e`, `solve -e --dump-region` | `Contradiction`, `exhausted = true` | k_s = 9, maximal line 1, all five candidates | prune. The sites are `absent` in two `(false)` rules and an obligation trigger, and the cube has 7 holes | W3, a new row; its trigger hole is P1e2.1b's |
| `examples/branching/04_two_levels.ein` (n = 46) | `solve -L` | `Contradiction`, `exhausted = true` | k_s = 7, maximal line 2 | 4 | **new.** W1 under `-L`, beside W1/lattice-02-L |
| `examples/lattice/01_subset_pruned.ein` (n = 4) | `solve -L` | `Contradiction`, `exhausted = true` | k_s = 12, maximal line 2 | 4 | **new.** W1 under `-L`, beside W1/lattice-02-L |

**`corpus/fuzz_findings/d3-unsat-core.ein` under `-e`**, the sixth known
disagreement, is not a declared cell, so the census does not run it. It sees
the file only through W11/d3-unsat-core's `solve -m 2` (§ 3.1).
`walk_and_region.md`'s `-e` reading still stands: the walk records 1 model,
and R's maximal line holds 2 states. Filing it means a register run that no
corpus entry declares.

### 3.4 What it found besides the six

**Two disagreements no page had named**: `branching/04_two_levels.ein` and
`lattice/01_subset_pruned.ein`, both under `solve -L`. Both are *certified*
wrong under a lever: `exhausted = true` and `Contradiction`, where R has two
maximal states. The default run of each file agrees. They are W1/lattice-02-L's
shape on two more programs, and the config principle makes them wrong by
itself. Everything else in §§ 3.1–3.3 is one of the six known, a register
row's own run, or another cell of a case that already exists.

## 4. What is not a defect

| cell | the walk | R | why not |
|---|---|---|---|
| `lattice/01_subset_pruned.ein` `solve -m 2` | `Contradiction`, `exhausted = false` | k_s = 12, maximal line 2 | the declared depth cap. It says it did not exhaust |
| `lattice/06_friends_tables_everyone_sits.ein` `solve` | `Open — owes 2`, `exhausted = false` | k_s = 8, maximal line 8 | `-n 1`, which an open state satisfies. The first node the walk records owes, and it stops there. `solve -e` gives the 8 models |
| `lattice/07_two_pairs_two_tables.ein` `solve` | the same | k_s = 8, maximal line 8 | the same |
| `type-exclusivity/colors.ein`, `nationalities.ein` `solve -e` | `Ambiguity`, 5 of 9, `exhausted = false` | k_s = 169, maximal line 9 | `-m 5` |
| `type-exclusivity/pets.ein` `solve`, `solve -e` | `Contradiction`, `exhausted = false` | k_s = 11 339, maximal line 39 | `-m 5` |
| `zebra2.ein` `solve -E 1` | aborts, exit 2 | one state | the declared entering budget |
| `ein-bugs/query-goal-free-head.ein` `solve` | exit 1 before a summary | — | the goal does not compile, by design, so there is no answer to compare |

The `-m 5` rows are `walk_and_region.md` § 4.2's cut, on the cells that
declare it. After the switch `-m` is a usage error, and these cells are
S1e2.1a.5's to re-point. **The two `-n 1` rows are the borderline ones.** The
read-out does say *the search did not exhaust*, but its word is `Open` about
a program with eight models, because the walk's stop counts an open state.
The `-n 1` the switch introduces stops at a state of R and cannot do this.
The words on all of these rows are the surfaces P1e2.3's S2 and S3 name: a
verdict word printed for a run that did not exhaust.

## 5. What it does not cover

- **29 corpus entries declare no `solve` run** (only `saturate` and `render`),
  among them `features/02`, the six `square-*` and `syntax/arg-kinds`. Nothing
  declares the walk on them, so no cell of theirs moves at the switch.
  `walk_and_region.md` measured them with its own variants (§ 4.2's cut).
- **The 63 cells with no R** (§ 1).
- **What the oracle shares with the walk**: the saturator (W7) and the
  candidate list (W4, W6)
  ([`solution_semantics.md` § 6.6](../../../docs/kernel/inference/solution_semantics.md#66-what-the-oracle-does-not-check)).
- **The tree cell** is judged on the CLI's answer and is not attributed.

## 6. What it leaves for T1e2.1a.10.2

- **New rows**, from § 3.3:
  - `zebra2-hints`, `branching/07` and `alive-empty-interlayer` under W1;
  - `three_components` and `trigger_hole` under W3;
  - `branching/04` and `lattice/01` under `-L`, as W1 rows like
    W1/lattice-02-L.

  `d3-unsat-core` under `-e` is the question at the end of § 3.3.
- **Twins**, from § 3.2: 17 cells on cases that exist.
- **The reversals**, measured: W9's controls are wrong under R and its `-K`
  runs agree. W11/unsat-core-id-space's `solve -e` is reason 3.
