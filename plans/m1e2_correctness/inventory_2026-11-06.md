# M1e2 inventory, re-taken: known incorrect results, solution gaps, and where they meet R

> **Re-taken 2026-11-06** at `c523f3ab`, once P1e2.2 and P1e2.3 were complete, by a read-only agent on a snapshot build, at the user's request of 2026-11-05: *"stop before close m1e2, I want to see inventory of known yet incorrect results, known solution gaps and their intersections with monotone/non-monotone lattices and region"*. It re-takes [the inventory of 2026-11-05](inventory_2026-11-05.md), which stays the record and is cited per item. **It waits on the user's reading. The close has not started.** No engine code and no answer was changed. Its scripts, probes, finding and `corpus.csv` are in [`inventory_2026-11-06/`](inventory_2026-11-06/). The JSON sweeps were not kept, and § Files says how to re-take them.

**How it was taken.**
- **The tree.** `git archive c523f3ab`, unpacked under `build/m1e2_inventory_2026-11-06/src/`.
- **The binary.** A release build of that tree, made fresh into the directory's own `target/`: `cargo build --release -p ein-cli --manifest-path src/ein.rs/Cargo.toml --target-dir target`, 12 s, sha256 `8f2f1ff5…`.
- **The runs.** Every run used `EIN_STDLIB=src/stdlib`, one engine process at a time, under `nice -n 19`.
- **The numbers.** A number marked *re-taken* was taken on that binary on 2026-11-06, by the command in its row or in § C.1. Any other number is quoted from the page it cites.
- **The paths.** Commands run from the snapshot's root, so every path is repo-relative. Each command also runs as written from the repository's root, with `ein.rs/target/release/ein`.
- **The words.** *Monotone* and *non-monotone* describe a **program**. R is [`solution_semantics.md`](../../docs/kernel/inference/solution_semantics.md)'s.

---

## 0. Summary

**The register.** [`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml) is at schema `ein-wrong-answers/12` and holds 114 rows: **103 fixed, 10 open and 1 not-wrong**. Every open row is reviewed `deferred` to its owner: four to F21, five to F22 and one to M1f's S1f.10.6. These are the ten rows [the milestone's acceptance](README.md#acceptance-for-the-milestone) names. Both register tests pass on the snapshot tree, so every row answers what it records:
- `cargo test --release -p ein-cli --test wrong_answers`, 13.2 s;
- `cargo test --release -p ein-infer --test register_region`, 3.3 s.

Both ran with `--test-threads=1` and their own `--target-dir`, on 2026-11-06.

**Describe against the oracle: 0 disagreements on 250 programs.** These are the corpus programs on which both `ein solve` (describe) and `ein oracle` finish. On each, the two agree on k_s, k_c, |N|, the hole count and the static verdict (`classify.py`'s `agree_describe_oracle`). The 11-05 script compared only the first three, though its § C.1 named all five. This re-take compares all five.

**Every mode against the oracle: 0 disagreements**, on the same 250 programs. The modes are `--minimal`, `--maximal`, `--count` and `--single`, and the script `sweep_modes.py` is new. No stop condition fired: the register's tests pass, describe agrees with the oracle, and so does every mode.

**One new finding, F1 (S): the minimal-line row gives a false reason on 8 complete runs of monotone programs.**
- **What it prints.** Describe prints *"minimal line 1 state — ∅, the trivial line: no obligation is stated"*. Each of the 8 programs states an obligation (`total-owed`, `surjective-owed`, `slot-owed-room`, `slot-owed-fill`). Root discharges each obligation, so no node owes and ∅ is the minimal line. But the stated reason is false: an obligation is stated.
- **Why.** The row reads whether a node it met owes (`read_out.rs`'s `minimal_line`, `owing_met`), not whether the program states an obligation.
- **What names it already.** [S1e2.2.26 § Notes](p1e2.2_wrong_answers/s1e2.2.26_line_modes_past_the_limit.md#notes) names this sentence only on a run stopped past `--region-limit` (S-17 here). Its words, *"which on a partial run is not the same as the program stating none"*, leave complete runs out.
- **Where.** The 8 programs are all under `tests/stdlib/` (`sweep_lines.py`): `algebra/24` and `/26`, `bijection/06`, `closure/02`, `slots/01`, `/08`, `/10` and `/12`.
- **The probe.** [`findings/F1_trivial_line_stated_obligation.ein`](inventory_2026-11-06/findings/F1_trivial_line_stated_obligation.ein) prints the same row as its [control](inventory_2026-11-06/findings/F1_trivial_line_stated_obligation_control.ein), which states no obligation.
- **What it changes.** No answer is wrong: R and its lines are right. The sentence's reason is false, and the summary's `minimal.trivial` is `true` on the same runs, read off the same bit. Owner: none.

**Live items, by class.** § A has one row for each item, fixed or live.

| class | live today | of them from 11-05 | new since 11-05 | with a register row | owner none | fixed since 11-05 |
|---|---:|---:|---:|---:|---:|---|
| **W** wrong answer | 13 | 9 | 4 | 10 (the 10 open rows) | 1 (W-22) | 6 of 11-05's, and 4 new items fixed as they were filed |
| **S** a surface misreports a right answer | 11, and F1 | 7 | 4, and F1 | 0 | 4, and F1 | 5; S-10 and S-14 moved to F23 as D-12 and D-13 |
| **D** diagnostic, spelling or page | 15 | 13 (D-1…D-11, S-10, S-14) | 2 | — | 2 (D-14, D-15) | 0. D-6 is half gone, and D-3 is wider |
| **G** gap: no answer, a refusal, a scoped answer (§ B) | 26 | 19 | 7 | — | 2 unruled (G-21, G-22) | — |
| **P** cost only (§ A.4) | 16 | 13 | 3 | — | — | P-17, fixed by `3fb9d712` when found |

**The live W and S items, with their owners:**
- **F21** (non-monotone programs):
  - its rows: W-1 `W8/features-07`, W-2 `W8/f07-rules-swapped`, W-5 `W8/fifo-gap` and W-6 `W8/corner-house`;
  - one item with no row, by class: W-23;
  - its labels: S-6, S-7, S-8, S-9, S-11, S-12 and S-13.
- **F22** (the exact core):
  - its rows: W-3 `W10/alt-cap-shared`, W-8 `W25/guard-only-second`, W-9 `W10/fork-delta-order`, W-10 `W10/closure-shared-given` and W-18 `W10/second-preparation`;
  - one item with no row, by ruling: W-20.
- **M1f's S1f.10.6:** W-15 `W26/oblig-derived-activator`.
- **None:** W-22 (`_` as a fact's argument, dropped at load); S-15, S-16, S-17 and S-18; and F1.

**What 11-05 carried that is now fixed:**

| 11-05 item | fixed by | stage, and the register rows that hold it |
|---|---|---|
| W-4, W21: a pair's core named one side | `67d07fab`. Cores are lazy since `3fb9d712`, and every core naming its set is a test since `af521c6e` | S1e2.2.21: `W21/*`, six rows. Re-taken: 0 of N's 5 477 cores on 114 programs lie wholly inside root's fixpoint while root is alive. On 11-05 that was 201 of 366 on `zebra2`, and more |
| W-7, F2: a core-search cut was printed as the core | `be7893d9` | S1e2.2.23: `W23/core-search-cut`. The core is now printed *"cut at its budget"*. The exact core past the budget is F22's (G-13) |
| W-11, F1: a keyword prefix split a name | `ed45a470` | S1e2.2.22: `W22/*`, fourteen rows |
| W-12, W-13, W-14: shapes in a rule that read nothing | `3ab9a231`: refused at load | S1e2.2.24: `W24/wild-in-guard`, `/bare-premise`, `/nested-unbound-head`. Each witness no longer loads, and that is the fix |
| S-1, N5: the fact printer | `712f23fd` | S1e2.3.8, with W28 |
| S-2, N6: empty cores on a stopped run | `b6139e62` | S1e2.3.9 |
| S-3, N7: `refuted by` | `a3bb9e70` | S1e2.3.10, under Q-M1e2.21 (a). G-20 is the accepted cost |
| S-4, N8: `--json-report`'s rows | `c239754c` | S1e2.3.10 (`ein-test-report/4`) |
| S-5: a reloaded dump's provenance | `712f23fd` | S1e2.3.8's default 4: the provenance is a comment, and a reloaded state's facts read as given (G-26) |

**What 11-05 carried that now has a row and stays open** (S1e2.2.25, `eb882d53`): W-2, W-5, W-6, W-8, W-9, W-10 and W-15. W-10 and W-15 had no witness on 11-05, and have one now. **What moved to a followup:**
- S-6 to S-9 and S-11 went to F21, by Q-M1e2.16 (3);
- D-1 to D-11 went to F23, by Q-M1e2.16 (4);
- S-10 and S-14 went to F23 as D-12 and D-13, by default.

**What is new since 11-05** (the next table, then § A and § B):

| id | item | status | owner |
|---|---|---|---|
| W-16 | W27: past the limit, the line modes labelled every member met | fixed `c21f93fc` | S1e2.2.26 |
| W-17 | W28: a claim and a query's classes compared printed text | fixed `712f23fd` | S1e2.3.8 |
| W-18 | `W10/second-preparation`: a core taken after root is prepared a second time | **live**, row open | F22 |
| W-19 | W21's `forall` gap | fixed `67d07fab` | S1e2.2.21 |
| W-20 | the `forall` fold's residual: 13 of `zebra2-hints`' 73 N cores are larger than the exact search's | **live**, no row | F22, by Q-M1e2.15 (b) |
| W-21 | Q-M1e2.18's four guard and activator shapes | fixed `3ab9a231`, refused at load | S1e2.2.24 |
| W-22 | `_` written as a fact's argument is dropped at load, in silence | **live** | **none** |
| W-23 | W-6's shape within the oracle's limit, on a stratified program: W22's `kw-neq-in-absent` pair | **live**, no row | F21 D2, by class |
| S-15 | `--diff-models` compares a file's facts as text | **live** | **none** |
| S-16 | past the limit, `--minimal --single` and `--minimal -n 1` say less than describe certifies | **live** | **none** |
| S-17 | the minimal-line row says *"no obligation is stated"* where no node met owes | **live**, and F1 widens it | **none** |
| S-18 | a guard written as a query's goal: *"the goal holds: it has no variable"* | **live** | **none** |
| D-14 | an owing hole that is not dead is written `core: []` | **live** | **none** |
| D-15 | `02_patterns.md` describes `:where` as a filter, and its T3 example is refused at load | **live** | **none** |
| P-14 … P-17 | four costs (§ A.4) | P-17 fixed, the rest live | — |
| G-20 … G-26 | seven gaps by ruling or default (§ B) | G-21 and G-22 unruled | — |

**The matrix: item × program class.** § C.3 gives every cell's reason.

The columns are the corpus entries in each class (§ C.2), with the 11-05 count in brackets:

| column | the class | entries today (11-05) |
|---|---|---:|
| **MC** | monotone program, complete | 248 (209) |
| **MS** | monotone program, stopped | 9 (9) |
| **NM** | non-monotone program, flagged | 29 (21) |
| **FB** | refused by *forbid* | 19 (19) |
| **LD** | refused at load | 75 (68) |
| **HS** | H-scoped, `outside` ≠ ∅ | 32 (24) |
| **TO** | types bound as objects | 10 (10) |

The first five partition the corpus. HS and TO are attributes that overlap them.

The cells:
- **●n**: measured on n corpus programs today;
- **○**: shown on a probe only;
- **0**: can reach the class, but measured on none;
- **–**: cannot reach the class;
- **?**: not measured.

The *touches* column says what part of the answer the item gets wrong: R (members or counts), N (the dead set and its cores), H (the universe), or s (a surface only).

| item | owner | MC | MS | NM | FB | LD | HS | TO | touches |
|---|---|---|---|---|---|---|---|---|---|
| W-1 `W8/features-07`, a stale positive | F21 D2 | – | – | ●2 | – | – | 0 | 0 | R |
| W-2 `W8/f07-rules-swapped`, R moves with rule order | F21 D2 | 0 | 0 | ●2 | – | – | 0 | 0 | R, H |
| W-3 `W10/alt-cap-shared` | F22 | ●1 | 0 | 0 | – | – | 0 | 0 | N (core) |
| W-5 `W8/fifo-gap` | F21 D2 | – | – | ●1 | – | – | ●1 | 0 | R |
| W-6 `W8/corner-house`, stale past the limit | F21 D2, D7 | – | – | ●5 | – | – | 0 | ●4 | R (lower bound) |
| W-8 `W25/guard-only-second` | F22 | – | – | ●1 | – | – | ●1 | 0 | N (core) |
| W-9 `W10/fork-delta-order` | F22 | ●1 | ? | ? | – | – | ●1 | ? | N (core) |
| W-10 `W10/closure-shared-given` | F22 | ●1 | ? | ? | – | – | ? | ? | N (core) |
| W-15 `W26/oblig-derived-activator` | M1f S1f.10.6 | – | – | ●1 | – | – | 0 | 0 | H |
| W-18 `W10/second-preparation` | F22 | ●2 | ? | ? | – | – | ●2 | 0 | N (core) |
| W-20 the `forall` fold's residual | F22, no row | ●1 | ? | ? | – | – | ●1 | ? | N (core) |
| W-22 `_` as a fact's argument | **none** | ○ | ●1 | 0 | 0 | 0 | 0 | 0 | R, H |
| W-23 stale facts within the limit | F21 D2, no row | – | – | ●2 | – | – | 0 | 0 | R |
| S-6, S-7 detector labels | F21 | – | – | ? | ? | – | ? | ? | s |
| S-8, S-9 | F21 | – | – | ? | ● (invisible) | – | ? | ? | s |
| S-11 `features/01`'s name collision | F21 | – | – | ●1 | – | – | 0 | ●1 | s |
| S-12 *"⊤, the trivial line"* on a non-monotone program | F21 D4 (d3) | – | – | ●13 | – | – | ●4 | 0 | s |
| S-13 a dead hole's row | F21 D4 (a) | – | – | – | (invisible) | – | – | – | s |
| S-15 `--diff-models` as text | **none** | ○ | ○ | ○ | – | – | ○ | ○ | s |
| S-16 past the limit, `--minimal --single` / `-n 1` | **none** | – | – | ●7 | – | – | ●1 | ●4 | s |
| S-17 and F1 *"no obligation is stated"* | **none** | ●8 | 0 | ○ | – | – | 0 | 0 | s |
| S-18 a guard as a query goal | **none** | ○ | 0 | 0 | – | – | 0 | 0 | s |

**Read across the classes.**
- **R itself.** Every corpus defect in R is on a non-monotone program, and is F21's: W-1, W-2, W-5, W-6 and W-23. One more defect reaches R on a monotone program, W-22, and it reaches one corpus program: `syntax/arg-kinds`, whose R is a lower bound anyway.
- **Cores.** Every other live defect that reaches a monotone program is in a core (W-3, W-9, W-10, W-18, W-20; all F22's), or is a surface (F1 on 8 corpus programs, and S-15 and S-18 on probes).
- **The modes.** No mode and no describe run disagrees with the oracle on any program where both finish (§ C.1).

### For the close

**Every live item, by owner.**
- **F21** (non-monotone programs):
  - rows W8/features-07, /f07-rules-swapped, /fifo-gap and /corner-house (W-1, W-2, W-5, W-6);
  - W-23, no row;
  - labels S-6, S-7, S-8, S-9, S-11, S-12 and S-13;
  - gap G-7, its decisions D1 to D7.
- **F22** (the exact core):
  - rows W10/alt-cap-shared, /fork-delta-order, /closure-shared-given and /second-preparation, and W25/guard-only-second (W-3, W-9, W-10, W-18, W-8);
  - W-20, no row;
  - G-13's residuals.
- **M1f S1f.10.6:** row W26/oblig-derived-activator (W-15).
- **F23:** D-1 to D-13.
- **Gaps by ruling** (§ B), owned by their pages: G-1 to G-20 and G-23 to G-26.

**Items the close must give an owner, because they have none:**
- **W-22**, `_` written as a fact's argument is dropped at load. This is a wrong answer in silence, the shape S1e2.2.24 refused in a rule but not in a fact. Its note suggests refusing it *"beside W24's shapes"*. A corpus program writes it: `syntax/arg-kinds`.
- **S-15**: `--diff-models` compares as text.
- **S-16**: past the limit, `--minimal --single` and `--minimal -n 1` say less than describe certifies.
- **S-17 and F1**: the minimal-line row's *"no obligation is stated"*, on 8 complete runs and on stopped ones.
- **S-18**: a guard written as a query's goal.
- **D-14**: an owing hole written `core: []`.
- **D-15**: `02_patterns.md`'s `:where` rows, and its T3 example, which no longer loads.
- **G-21**: may a rule conclude a fact of the reserved `neq`? Unruled.
- **G-22**: a keyword pair beside an `:assert`'s conjuncts is dropped in silence. Unruled.
- **P-14, P-15 and P-16**: costs. P-15 also asks the user whether to keep N's cores past the limit, because a surface would then print core facts.

**Rows the close will not name.**
- W-20 is F22's with no row, by the ruling on Q-M1e2.15.
- W-23 is F21's class, but no row names its two programs.
- W-22 has no owner and no row.

The ten open rows are exactly the acceptance's list.

**The close's own re-take, as Q-M1e2.16 (6) ruled it,** is the register's tests plus this document's sweep, and it runs as written: § C.1 lists each command. `probes/capture.sh`'s W lines still spell retired options. The ruling took them out of the re-take (G-19).

---

## A. Known incorrect results

**Sources swept.**
- Every row of [`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml). That covers W22–W28 and the ten deferred rows.
- The `§ Notes` section, and every *parked*, *finding* and *default* paragraph, of the stage files of S1e2.2.21 to S1e2.2.26 and S1e2.3.8 to S1e2.3.10.
- Q-M1e2.16 to Q-M1e2.21 in [`open_questions.md`](open_questions.md).
- [F21](../followups/f21_non_monotone_programs_decided.md), [F22](../followups/f22_exact_core_past_the_cap.md) and [F23](../followups/f23_diagnostics_in_eins_words.md).
- Every item of [the inventory of 2026-11-05](inventory_2026-11-05.md), each re-taken.

**The witness runs.** A row's witness was run by `retake_items.py` (§ C.1), which writes each run's whole stdout and stderr under `retake/<id>/`. The *re-taken* cell quotes those runs.

### A.1 W: wrong answers

**The 11-05 items.**

| id | 11-05 said ([§ A.1](inventory_2026-11-05.md)) | status now | owner now | witness re-run (2026-11-06) | what it printed |
|---|---|---|---|---|---|
| **W-1** | `W8/features-07`: the fork node `{(q one)}` holds `(p one)`, derived at root under `(absent (q one))` | **live**. The row is open and deferred, and `wrong_answers.rs` holds it to its `now` | F21 D2 | `ein solve examples/features/07_unstratifiable.ein -p` | State 2 of 2, `{(q one)}`, holds `(p one) (q one)`. *stale facts — 1 of the cube's 2 nodes*: `(p one) ← derive-p (absent (q one))` |
| **W-2** | R moves with which of two priority-200 rules is written first | **live**. Filed as `W8/f07-rules-swapped` by S1e2.2.25 (`eb882d53`), open, deferred | F21 D2 | `ein solve examples/ein-bugs/wrong-answers/f07-rules-swapped.ein` | R = {∅, {(p one)}}, with `(q one)` stale in `{(p one)}`. As written (W-1) it is {∅, {(q one)}} |
| **W-3** | `W10/alt-cap-shared`: core `(a K) (b K) (c C01)`, smallest `(a K) (b K)` | **live**. Open, deferred | F22 | `ein solve examples/ein-bugs/wrong-answers/alt-cap-shared.ein` | `unsat core (a K) (b K) (c C01)`, `refuted by clash` |
| **W-4** | W21: a pair's core is its positive side alone | **fixed** `67d07fab` (S1e2.2.21, `W21/*`). Held by `ein-infer/tests/corpus_cores.rs` | — | `ein solve examples/zebra2.ein --dump-answer D`, and `sweep_cores.py` | The core of `{(drink-loc Tea House-4)}` names `(drink-loc Tea House-4)`: 22 facts, `within-budget`, `refuted by functional-negative, range-elimination`. Over the corpus, 0 of N's 5 477 cores lie wholly inside root's fixpoint while root is alive, and 0 name no fact of their set |
| **W-5** | The FIFO gap: `lattice/10`'s node `{a, b}` holds a stale `p` | **live**. Filed as `W8/fifo-gap` (`eb882d53`), open, deferred | F21 D2 (stratum order) | `ein solve examples/lattice/10_fifo_gap.ein` | `stale {(a one), (b one)}, in R: (p one) ← r2 (absent (q one))`, 1 of 4 nodes. *path-dependent — 1 of 2 parent paths* |
| **W-6** | Members of R past the limit hold a fact their own fixpoint's guard refutes | **live**. Filed as `W8/corner-house`, the cheapest of five, open, deferred | F21 D2, D7 | `ein solve examples/saturation/square-unique/corner-house.ein` | *stale facts — 5 595 of the 65 389 nodes the run computed*. The four others are unmoved (§ C.4) |
| **W-7** | **F2**: a core-search cut was printed as the core | **fixed** `be7893d9` (S1e2.2.23, `W23/core-search-cut`). The residual, the exact core past the budget, is F22's (G-13) | F22 (residual) | `ein solve plans/m1e2_correctness/inventory_2026-11-05/findings/F2_core_cut.ein` | The same 10-fact core, with *core search cut at its budget of 64 environments per fact: a smaller core may exist*. Over the corpus, 19 of N's 5 477 cores are cut (13 at the environments, 6 at the rounds), and so is one root core: `core-search-cut`'s own |
| **W-8** | A premise-less derivation from a node-derived activator is never an alternative | **live**. Filed as `W25/guard-only-second` (`eb882d53`), open, deferred | F22 | `ein solve examples/ein-bugs/wrong-answers/guard-only-second.ein --dump-answer D`, and `guard-only-first.ein` | N `{(p a)}` has core `(b a) (p a) (seed a)` by `early`. The `-first` twin gives `(p a)` by `w` |
| **W-9** | `fork-delta-order`'s smallest core is not the canonical one | **live**. `W10/fork-delta-order`, open, deferred, contested | F22 | `ein solve examples/ein-bugs/fork-delta-order.ein --dump-answer D` | Core `(g G08) (h H40) (on A) (on B)`. The row's right answer is `(g G01) (h H01) (on A) (on B)` |
| **W-10** | The smallest-closure estimate counts a shared given twice (latent on 11-05) | **live, now witnessed**: `W10/closure-shared-given` (`eb882d53`), open, deferred | F22 | `ein solve examples/ein-bugs/wrong-answers/closure-shared-given.ein`, and `-control.ein` | Root's core is `(c C01) (d C01)`. The control's is `(g K)`, which is the row's right answer |
| **W-11** | **F1**: a keyword prefix split a name | **fixed** `ed45a470` (S1e2.2.22, `W22/*`) | — | `ein solve plans/m1e2_correctness/inventory_2026-11-05/findings/F1_keyword_prefix_split.ein`, and `ein saturate …/F1b_or_prefix.ein --dump` | 1 state, *no candidate*, `?y = a`, `as :expect (model (neqw a))`. F1b stores `(orz a :rule r)` |
| **W-12** | `_` in a guard | **refused at load** since `3ab9a231` (S1e2.2.24, `W24/wild-in-guard`) | — | `ein solve plans/m1e2_correctness/inventory_2026-11-05/probes/wild_in_guard.ein` | Exit 1: `kb load error: rule 'e': its :match writes (eq ?x _), and _ names no value — …`, one clause per rule. The witness no longer loads, and that is the fix |
| **W-13** | A premise that is not a form is dropped | **refused at load** since `3ab9a231` (`W24/bare-premise`) | — | `ein solve …/inventory_2026-11-05/probes/bare_premise.ein` | Exit 1: `kb load error: rule 'a': its :match is _, and _ is no premise — …` |
| **W-14** | A nested pattern with an unbound relation never matches | **refused at load** since `3ab9a231` (`W24/nested-unbound-head`) | — | `ein solve …/inventory_2026-11-05/probes/nested_unbound_head.ein` | Exit 1: `kb load error: rule 'r': its :match writes (?r b) with ?r as its relation, which no parameter binds — …` |
| **W-15** | An obligation derived under a hypothesis (Q-M1e.11, latent on 11-05) | **live, now witnessed**: `W26/oblig-derived-activator`, open, deferred | M1f S1f.10.6 | `ein solve examples/ein-bugs/wrong-answers/oblig-derived-activator.ein`, and `-declared.ein` | R is empty: *every alive node owes*. `{(likes Ann Soup)}` owes 1 (*drinks owes Ann a Drink.*), over 1 candidate. The control has 1 state, `(drinks Ann Tea) (likes Ann Soup)`, over 2 candidates |

The 11-05 **moot** shapes are still moot: *forbid* refuses every program that has them (G-1's 19). These are `stale-naf-fork` and its twin, `not_conclusion`, `ACTIVATOR_UNDER_ABSENT` and `ACTIVATOR_REFUTES`.

**New since 11-05.**

| id | what is false | status | owner | source | witness re-run (2026-11-06) | what it printed |
|---|---|---|---|---|---|---|
| **W-16** | **W27**. Past `--region-limit`, on a program the static check flags, `--minimal` and `--maximal` labelled every member met as on their line | **fixed** `c21f93fc` (`W27/*`, four rows). Held by `ein-render/tests/line_modes_past_the_limit.rs` | S1e2.2.26 | Q-M1e2.19 | `ein solve examples/ein-bugs/wrong-answers/past-limit-lines.ein --minimal`, then `--maximal`. `ein solve examples/lattice/10_fifo_gap.ein --region-limit 1 --minimal`, then `--maximal`. `ein oracle examples/lattice/10_fifo_gap.ein` | `minimal line 1 state — ∅`, and *2 other members met are not certified on a line*. `maximal line not known — …`, with 3 others. At limit 1, fifo-gap gives ∅ and *not known*. The oracle gives minimal {∅} and maximal ⊤ |
| **W-17** | **W28**. A claim and a query's classes compared printed text, so the int `1` and the name `"1"` were one value | **fixed** `712f23fd` (`W28/*`, three rows) | S1e2.3.8 | Q-M1e2.20 | `ein test examples/ein-bugs/wrong-answers/claim-int-name.ein -v`. `ein solve …/claim-int-name-solve.ein`. `ein solve …/classes-int-name.ein` | Both claims print `FAILED` and exit 1 (`(model (holds "1"))` against `(holds 1)`). *8 states → 4 projection classes* |
| **W-18** | `W10/second-preparation`. A member of N whose core `--dump-answer` takes after `Nodes::new` prepares root a second time, through the cap | **live**. Open, deferred | F22 | [S1e2.2.23 § Notes](p1e2.2_wrong_answers/s1e2.2.23_core_search_cut.md#notes) | `ein solve examples/zebra2.ein --dump-answer D` | The core of `{(nation-loc Ukrainian House-2), (smoke-loc Old_Gold House-4)}` is 18 facts, `within-budget`. The row's right answer is 17. The page measured 9 of 366 cores on `zebra2` and 9 on `zebra2-obligations` (2026-11-05). That is not re-taken here, because it needs an uncapped build |
| **W-19** | W21's `forall` gap: a contradiction concluded through a `forall`, whose support a firing did not record | **fixed** `67d07fab` (`W21/forall-core-false`, `/forall-core-elim`) | S1e2.2.21 | Q-M1e2.15 (b) | `ein solve examples/ein-bugs/wrong-answers/forall-core-false.ein --dump-answer D`, and `forall-core-elim.ein` | Cores `(item a) (tag s1) (tag s2)` and `(item a) (not (in a s2)) (slot s1) (tag s1)`. Each names its set |
| **W-20** | The `forall` fold's residual. Each fact a universal read joins at its own smallest frontier, so 13 of `zebra2-hints`' 73 N cores are larger than the exact search's | **live**. No row, by the ruling | F22 | [F22](../followups/f22_exact_core_past_the_cap.md), Q-M1e2.15 (b) | **Not re-takable from the CLI.** It needs the exact search S1e2.2.21 prototyped (T1, `build/s1e2.2.21/`, 2026-11-05) | — |
| **W-21** | Q-M1e2.18's four shapes: as a guard's argument, a form, a keyword pair, or a variable no earlier premise binds; and a relation parameter its activator binds to a nested fact | **fixed**, refused at load since `3ab9a231` (`W24/form-in-guard`, `/kwpair-in-guard`, `/guard-before-binder`, `/nested-fact-activator`) | S1e2.2.24 | Q-M1e2.18 | `ein solve examples/ein-bugs/wrong-answers/{form-in-guard,kwpair-in-guard,guard-before-binder,nested-fact-activator}.ein` | Each exits 1 with its load line, e.g. *`(eq ?x (f a))`, and the form (f a) is no value — …*, and *(r (a b)) binds it to the fact (a b) — a relation is named by a symbol, not a fact* |
| **W-22** | **`_` written as a fact's argument is dropped at load, in silence.** `(r _ ?x)` against `(relation r T T)` is stored as the one-argument `(r "?x")`. The string `"?x"` then becomes a name of the program and a candidate object | **live** | **none** | [S1e2.3.8 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md#notes), 2nd item | `ein saturate plans/m1e2_correctness/inventory_2026-11-06/probes/fact_wild_arg.ein --dump`, and `ein solve` on the same file | It stores `(r "?x")` and `(r a b)`, with no line. R is 32 states over 5 candidates, and each state holds `"?x"` as an object (`(r "?x" a)`, `(r a "?x")`, …). In the corpus, `examples/syntax/arg-kinds.ein` writes the shape: its dump holds `(r "?x")`. Right: refused at load, as `_` in a rule's guard has been since W24, or kept as written |
| **W-23** | **W-6's shape within the oracle's limit, on a stratified program.** W22's pair, `kw-neq-in-absent` and its twin, banked by S1e2.2.22. 4 of the cube's 8 nodes, every one in R, hold `(hit a)`, whose guard `(absent (neqw a))` their own fixpoint falsifies. On 11-05 the within-limit stale witnesses were `features/07`, an even cycle, and `lattice/10`, the FIFO gap. This is the plain case: one `absent`, no cycle | **live**, contested as W8's rows are. It is right under F21 D2's reading (A), the fork, which is today's node | F21 D2, by class. **No row** | this sweep. F21 D2 | `ein solve examples/ein-bugs/wrong-answers/kw-neq-in-absent.ein` (the twin is the same) | *stale facts — 4 of the cube's 8 nodes hold one*, at `{(neqw a)}`, `{(hit b), (neqw a)}`, `{(neqw a), (p b)}` and `{(hit b), (neqw a), (p b)}`. The W22 row's `now` is `--minimal` (root alone), and `register_region` records the 8 states under (A) |

### A.2 S: surfaces that misreport a right answer

| id | 11-05 said | status now | owner now | witness re-run (2026-11-06) | what it printed |
|---|---|---|---|---|---|
| **S-1** | **N5**: the fact printer dropped what tells two values apart | **fixed** `712f23fd` (S1e2.3.8) | — | `ein saturate --dump` on `examples/syntax/arg-kinds.ein`, on `examples/ein-bugs/activator-text-int-and-string.ein`, on `…/inventory_2026-11-05/probes/n5_int_vs_string.ein`, and on `plans/m1e2_correctness/inventory_2026-11-06/probes/empty_arg.ein` | `(r 7 "a string")`, `(r "1..5" "1..*")` and `(r "?x")`. `(holds 1)` beside `(holds "1")`, and `(noted 1 "1" …)`. `(f ())`, which was `(f (@empty))` ([S1e2.2.22 § Notes](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md#notes)). `(r _ ?x)` prints as what the loader stored, `(r "?x")`: that is W-22, not the printer |
| **S-2** | **N6**: a run stopped at the limit wrote `core []` for N's members | **fixed** `b6139e62` (S1e2.3.9) | — | `ein solve examples/lattice/06_friends_tables.ein --dump-answer D`, and `sweep_cores.py` | 68 of 68 members are written `core: null`, `core_status: "not-kept"`, `refuted_by: null`, and `past-limit-lines` writes its 15 the same way. No member is written `core: []` on any of the 114 runs |
| **S-3** | **N7**: `refuted by` named a rule the core does not use | **fixed** `a3bb9e70` (S1e2.3.10, Q-M1e2.21 (a)) | — | `ein solve examples/ein-bugs/alt-cap-core.ein`, and `-reordered.ein` | Both print `(w1 X) (w2 X)` and `refuted by wide`. G-20 is the accepted cost |
| **S-4** | **N8**: `--json-report` rows carried no `outside` and no `types` | **fixed** `c239754c` (S1e2.3.10) | — | `ein test tests/region/07_cube.ein --json-report R -v` | `ein-test-report/4`. The row carries `outside: [{relation: is-a, arity: 2, why: no-hrule}]` and `types_as_objects` |
| **S-5** | A reloaded dump read every fact as given, while its file spelled `:rule` and `:hypothesis` | **fixed** `712f23fd`, under S1e2.3.8's default 4: the provenance is a comment, and `answer_dump.md` says a reloaded state's facts are given (G-26) | — | `ein solve …/inventory_2026-11-05/probes/n5_string_space.ein --dump-answer D`, then `ein saturate D/models/38.ein --dump` | The file writes `(holds "x y") ; rule r`, `(holds 1) ; rule s` and `(p A) ; hypothesis 0`. Reloaded, the state's facts come back by value, each one given |
| **S-6** | The detector's `RootPlans` keys root's plans by the activator's symbols | **live**, latent | F21 (Q-M1e2.16 (3)) | No witness. `detector.rs`' `RootPlans` still keys by `(rule, activator_args)` | — |
| **S-7** | The detector's chain attribution reads every application as the activator | **live**, latent | F21 | No witness | — |
| **S-8** | The static check's D does not follow activators | **live**, not re-takable | F21 | `ein solve examples/ein-bugs/wrong-answers/forbid-chain-node-writes.ein --monotone` | Exit 1, with *forbid*'s line: `--monotone` refuses it too |
| **S-9** | `absent_premises` reads the first contradiction's positive side | **live**, library only. [S1e2.3.10 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_refuted_by_and_report_rows.md#notes) and [S1e2.2.21 § Notes](p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md#notes) found it again | F21 | `region.rs:1385`: `absent_premises(&fork, terms, first.witness())` | — |
| **S-10** | The `types` warning reads `is-a` by name only | moved to **F23 as D-12**, latent | F23 | — | — |
| **S-11** | `features/01`'s activator site is an arity-blind name collision | **live** | F21 | `ein solve examples/features/01_not_and_absent.ein --monotone` | Two sites: `unknown-feeling` (its activator relation one a commitment can change), and `unknown-feeling [activator Person]: absent reads likes` |
| **S-12** | *"⊤, the trivial line: no restriction bites"* is printed on a flagged program | **live** | F21 D4 (d3) | `ein solve examples/lattice/04_obligation_trigger_hole.ein`, and every flagged program | Printed beside `1 hole`. 13 of the 22 complete flagged programs print the row (§ C.3), where 11-05 named one |
| **S-13** | A dead hole's row prints its core only | **live**, not re-takable: no dead hole loads | F21 D4 (a) | — | — |
| **S-14** | `corpus_shapes.md5`'s `hyp` rows digest a generator no route runs | moved to **F23 as D-13** | F23 | `ein-infer/src/shape.rs:290` still calls `hypgen::generate` | — |

**New since 11-05.**

| id | what is misreported | status | owner | source | witness re-run (2026-11-06) | what it printed |
|---|---|---|---|---|---|---|
| **S-15** | **`--diff-models FILE` compares facts as text, not as values.** A models file that writes a fact in another spelling of the same value does not match it | **live** | **none** | [S1e2.3.8 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md#notes), 3rd item | `ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/diff_models_text.ein --diff-models …/diff_models_text.models`, and `…_control.models` | The file writes `(p "a")`, which is the value `(p a)`: *0 matched, 0 near, 1 missing, 4 spurious*. The control writes `(p a)`: *1 matched* |
| **S-16** | **Past the limit, `--minimal --single` says *"not decided — the run stopped first"*, and `--minimal -n 1` says *"the minimal line may hold more"*,** where ∅ ∈ R decides the minimal line and describe certifies it as {∅}. This is weaker than describe, not contrary to it | **live** | **none** | [S1e2.2.26 § Notes](p1e2.2_wrong_answers/s1e2.2.26_line_modes_past_the_limit.md#notes), 1st item | `ein solve FILE --minimal --single`, then `--minimal -n 1`, then bare, on the 7 flagged programs stopped at the limit (the log is `s16_sweep.log`, 2026-11-06) | On all 7, *not decided* and *may hold more*, beside describe's `minimal line 1 state — ∅` |
| **S-17** | **Describe's minimal-line row says *"∅, the trivial line: no obligation is stated"* where no node the run met owes,** which is not the same as the program stating none | **live** | **none** | S1e2.2.26 § Notes, 3rd item | `ein solve examples/ein-bugs/wrong-answers/past-limit-lines.ein --region-limit 0`, then `1`, then `2`. The program states `total-owed` | At 0 and 1: *∅, the trivial line: no obligation is stated*. At 2: `∅`. **F1** is the same sentence on 8 complete runs (§ 0) |
| **S-18** | **A query whose goal is a guard** with a variable, `(query :goal (neq w ?y))`, prints *"the goal holds: it has no variable"*. It also suggests `as :expect (model (neq w ?y))`, a claim the loader refuses | **live** | **none** | [S1e2.2.22 § Notes](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md#notes), 2nd item | `ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/goal_guard.ein`, and `goal_guard_claim.ein` | `the goal holds: it has no variable` and `as :expect (model (neq w ?y))`. The suggested claim gives `kb load error: :expect — ?y is a variable; an expectation is an answer, not a pattern`. No corpus program writes a guard as a goal |

### A.3 D: diagnostics, spellings and pages

D-1 to D-13 are F23's ([F23](../followups/f23_diagnostics_in_eins_words.md), on Q-M1e2.16 (4)).

| id | defect | status now | owner | re-taken (2026-11-06) |
|---|---|---|---|---|
| **D-1** | Python spellings, which differ between commands | **live** | F23 | `ein solve nope.ein` prints `FileNotFoundError: [Errno 2] No such file or directory: 'nope.ein'`, where `ein saturate nope.ein` prints `error: nope.ein not found`. `ein saturate examples/ein-bugs/unbound-assert-var.ein` prints `KeyError: "unbound var ?v1 in :assert — bindings: {'v0': 'o2', 'v2': 'o1'}"`, while `solve` and `oracle` print the message alone, with the Python `dict` of bindings ([S1e2.3.8 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md#notes)' *repr* item is this). `ein solve examples/ein-bugs/query-goal-free-head.ein` and `features/02` print `ein.inference.compile.CompileError: … SForm(head=Var(name='R'), …)`. The five `examples/broken/compile/` fixtures print `SForm(…)` and `Loc(…)` (the describe sweep). `unbound-relation-head.ein` no longer does: since `3ab9a231` it is refused at load, in Ein's words |
| **D-2** | Loader messages end `at None` or `at Loc(…)` | **live** | F23 | Over the describe sweep, 30 programs' first stderr line ends `at None`, and 6 carry `Loc(file=…)` |
| **D-3** | `ein test` exits 1 with an empty stderr on a false claim | **live, and wider** | F23 | `ein test …/inventory_2026-11-05/probes/refuted_region_claim.ein` exits 1 with 0 bytes of stderr, where `ein solve` prints `(solve :expect) FAILED — the claim lists 1 state and R has 2`. S1e2.3.8 found the same for a query's `:expect`: `ein test examples/ein-bugs/wrong-answers/claim-int-name.ein` exits 1 with 0 bytes, and so does `claim-int-name-solve.ein`. Because of it, W28's two claim witnesses declare `runs = ["solve", "saturate"]` in `corpus.toml`, since `corpus_cli::every_refusal_carries_a_diagnostic` forbids the shape. So **no corpus run checks `ein test`'s FAILED path** |
| **D-4** | The `[activator …]` label prints symbols only | **live** | F23 | `ein solve examples/ein-bugs/activator-all-int.ein` prints `rule 'alarm': concludes …`. `-all-symbol.ein` prints `rule 'alarm' [activator a]: …` |
| **D-5** | The plan shape's `GUARD` line prints `String(value='b')` | **live**, library only | F23 | `ein-ir/src/ast.rs:334` |
| **D-6** | `ein saturate --max-steps 3` prints `max_steps=3` and `last='includes'` | **half live** | F23 | `ein saturate examples/zebra.ein --max-steps 3` prints `saturate: running with max_steps=3, progress every 500 steps`. `last='includes'` is gone: the step-limit line names the last firing in Ein's words |
| **D-7** | `ein.inference.saturator.SaturatorStepLimitError` | **live**, unreachable from the binary | F23 | `ein-cli/src/common.rs:254` |
| **D-8** | An hrule's template that builds no fact proposes nothing, and nothing says so | **live** | F23 | `ein solve …/inventory_2026-11-05/probes/hrule_no_fact.ein` loads with no line: 2 states over 1 candidate, which the parameter-less `good` proposes. [S1e2.2.24 § Notes](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md#notes) adds `_` in an hrule's `:assert` (`build/s1e2.2.24/extra/hrule-assert-wild.ein`, not re-taken) |
| **D-9** | An unused rule parameter is accepted silently | **live**, re-taken for the first time | F23 | `ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/unused_param.ein` loads with exit 0 and no line. `foo`, which only the activator `(r foo)` names, is a candidate object, and R is 3 states |
| **D-10** | `ein render rules` exits 1 on a file with no rule | **live** | F23 | `ein render rules examples/ein-bugs/kwpair-below-the-filter.ein` exits 1: `no rule forms in …` |
| **D-11** | Pages and records that no longer say what holds | **live** | F23 (outside `plans/m1e2_correctness/`); the close (inside it) | By reading at `c523f3ab`; the list follows the table |
| **D-12** | The `types` warning reads `is-a` by name only (it was S-10) | **live**, latent | F23 | The ten `instance` programs all have H = ∅ (11-05). Not re-taken |
| **D-13** | `corpus_shapes.md5`'s `hyp` rows (it was S-14) | **live**, test golden | F23 | `shape.rs:290` |
| **D-14** | **An owing minimal hole that is not dead is written `core: []`** beside `core_status: null`, in the summary's `region.holes.minimal`. `null` is S1e2.3.9's spelling for *no core taken* | **live**, new | **none** | `ein solve examples/ein-bugs/wrong-answers/past-limit-lines.ein --region-limit 17 --json-summary J` gives `{set: [(seats Ann C1)], dead: false, core: [], core_status: null, owes: 1}`. Over the corpus (`sweep_lines.py`), 7 owing holes on 6 programs are written so: `lattice/04`, `/08`, `/09`, `/12`, `synth/local_not_global` (2) and `tests/region/07`. Since *forbid*, every minimal hole that loads is owing ([S1e2.3.9 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md#notes)) |
| **D-15** | **`02_patterns.md` describes `:where` as a filter**, where Q32 drops it. Its construct table's *`:where` filter* and *relation-property tag* rows say so, and so does its § *As a T3 structural rule*, while line 61 of the same page says the keyword pair is dropped. The T3 example itself is refused at load since `3ab9a231` | **live**, new | **none** | `ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/where_t3_example.ein` (the page's example, verbatim) exits 1: `kb load error: rule 'triangle-composition': its :match writes (?r ?a ?b) with ?r as its relation, which no parameter binds — …`. [S1e2.2.24 § Notes](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md#notes), 1st item |

What D-11 still covers at `c523f3ab`:
- **`configuration.md` § 2** (line 419). It says *"the cube's limit and the deadline stop a run, and then every surface says the answer is partial"*, beside *"A `-T` abort exits 2 and prints no answer"*. Re-taken: `ein solve examples/zebra.ein -T 0` exits 2 and prints `** aborted: --max-time 0 s reached before the question was decided **`.
- **`solution_semantics.md:458`** still says *"No surface computes projection classes yet"*.
- **`monotonicity.md:945`** still has its present-tense quotation.
- **`region.rs`' module doc** (lines 40–43) speaks of `alive₀`, the cascade and the lookahead.
- **`implementation.md:117`** names `--dump-region`.
- **`release.yml:3`** says *"Everything nightly"*. *"two slow entries"* is no longer found.
- **`plans/README.md:184`** says *"its 31 probe programs"*.
- **Global Q28** (`plans/open_questions.md:561`, and `03_ein_model.md:465`) calls `()` a no-op atom. At top level it is a parse error: `ein solve …/inventory_2026-11-05/probes/empty_parens.ein` gives `3:2: unexpected input`, exit 1.
- **M1e's Q-M1e.8 index row** still says open.
- **Inside `plans/m1e2_correctness/`**, four probes still do not load: `ein solve plans/m1e2_correctness/probes/{cfgtest,f07-warn,fresh-name-declared-default,ucid-perm1}.ein` exits 1 on each. 11-05's other in-milestone pages were not re-read.

### A.4 P: cost only

None of these changes an answer. 11-05's thirteen are kept with their 11-05 ids:

| id | cost | status (2026-11-06) |
|---|---|---|
| P-1 | `candidate_objects` walks every fact at every depth on each call (D2X-6) | not measured. `hypgen.rs:753` |
| P-2 | `--models key`'s node budget does not bound time: 66.6 s at k = 1 400 | not re-taken |
| P-3 | A plan duplicated by a nested-fact activator fires twice | not re-taken. S1e2.2.16 keyed plans by value |
| P-4 | L5 licenses only 18 of `zebra.ein`'s 36 debts | not re-taken |
| P-5 | MARCO-style enumeration is not built, and `--minimal` is not output-sized | as 11-05 |
| P-6 | The state route stops between whole layers | as 11-05 |
| P-7 | `arg-kinds`' read-out costs more than its route | as 11-05 |
| P-8 | Recording alternatives, and the smallest-closure bookkeeping, cost +5.8 % on `zebra2` | as 11-05 |
| P-9 | `--jobs` scales sublinearly on the cube | as 11-05 |
| P-10 | The lookahead pre-filter and fail-fast were given up with their flags | as 11-05 |
| P-11 | `hypgen::generate_one_branch` has no caller | live: `hypgen.rs:252`, and `grep` finds no caller |
| P-12 | `utils/fork_split.py` leaks a directory and spells `-e` | live: lines 78–79 and 246 |
| P-13 | F11's D1 and D2 are parked on their triggers | as 11-05 |

The new ones:

| id | cost | status (2026-11-06) |
|---|---|---|
| **P-14** | Past the limit, `--minimal -n 2` and `--minimal --single` visit 2^limit nodes, where ∅ ∈ R decides the minimal line in one saturation (S1e2.2.26 § Notes, 2nd item) | **live**. Re-taken on the 7 flagged programs stopped at the limit (`s16_sweep.log`): 64 655 to 65 536 saturations each. `features/01 --minimal -n 2` takes 64 655 saturations |
| **P-15** | N's cores past the limit are computed, then written `not-kept` ([S1e2.3.9 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md#notes)) | **live**. On `lattice/06`, `--dump-answer` takes 2.57 s and a bare `ein solve` 1.96 s (best of three, 2026-11-06). The page puts 0.58 s of that on 65 022 core searches no surface prints. Two ways out, and each changes no answer. (a) Keep them: `nogoods.json` would print 68 cores where it prints `not-kept`, a core's facts appearing where none are printed, so this is **the user's call**. (b) Take none where no surface asks |
| **P-16** | The round-trip test's four dumps past the limit, about 400 MB each, are most of its 30 s (S1e2.3.8 § Notes) | **live**, not re-taken |
| **P-17** | A bare `ein solve` took a core at every dead node it visited and printed none (S1e2.2.21 § Notes) | **fixed** `3fb9d712`, on the user's *minimum work per question*. The describe sweep's `zebra2` run is 0.53 s |

---

## B. Known solution gaps

Each gap is a place where Ein gives no answer, a refusal, a scoped answer or a lower bound, by design, by a ruling or by a default. The counts are this snapshot's, from § C's sweeps. 11-05's text is in [its § B](inventory_2026-11-05.md); only what moved is written here.

| id | gap | ruling / page | corpus programs today (2026-11-06) |
|---|---|---|---|
| **G-1** | ***forbid***. A command that takes the universe refuses a program whose `(false)` or `(not …)` rests on an `absent` over a relation a commitment can still extend | Q-M1e2.5; `597b2b7e`, `3257488f`, `f5a0dca9` | **19**, the same 19 as 11-05 (§ C.2). Re-taken on `naf-upward-closure.ein`: `ein solve` and `ein oracle` exit 1 with the rule's line, and `ein saturate` exits 0 |
| **G-2** | **Refusals at load** | Q-M1e2.13 (a), Q-M1e2.14 (c), Q-M1e2.18; [`defined_behaviour.md` § 4.8, § 4.9](../../docs/kernel/defined_behaviour.md) | **75**, against 68. 53 `load-negative`, 5 `compile-negative` and 4 `parse-negative`. 12 regressions: `range-in-match`, `wild-in-head`, W24's seven witnesses, `query-goal-free-head`, `unbound-relation-head` and `unbound-assert-var`, the last of which stops at its firing. 1 `positive`: `features/02`, which `ein oracle --dump` still answers (k_s 728, k_c 32 768, in `region_census.json`) before it exits 1. `empty_absent` and `unbound_head` moved to `examples/broken/load/` |
| **G-3** | **R is scoped over H**: the `outside` row | Q-M1e2.3 (a) | **32**, against 24. 27 *no-hrule*, 1 *arity*, 4 *not-owed* (§ C.2). The 8 new ones are register witnesses |
| **G-4** | **Types bound as objects**: the `types` row | Q-M1e.12 (a) | **10**, the same 10 |
| **G-5** | **Below relation grain, unnamed** | `solution_semantics.md` § 1.4 | not measured |
| **G-6** | **Names.** A constant only a query names is no candidate, and Ein makes no name at run time. A variable or a range written in a fact is a name, and so an object of H. This is unruled | Q-M1e2.9 (a); S1e2.2.14 and [S1e2.2.22 § Notes](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md#notes), 3rd item | `syntax/arg-kinds`. Re-taken on `plans/m1e2_correctness/inventory_2026-11-06/probes/var_in_fact.ein`: `(f (p ?a))` loads as `(f (p "?a"))`, and R holds `(f "?a")` |
| **G-7** | **Non-monotone programs: detected, warned, undecided** (F21 D1 to D7) | [F21](../followups/f21_non_monotone_programs_decided.md) | **29** flag and load, against 21. 7 exercised, 15 conservative, and 7 *not known* past the limit (§ C.2) |
| **G-8** | **Stopped runs**: a lower bound | the budget ruling of 2026-11-03; `solve_interface.md` | **16**, against 15. The same 9 monotone programs at the budget, and 7 flagged ones at the limit; the new one is `past-limit-lines` (n 17) |
| **G-9** | **The oracle's 2ⁿ limit** | the ruling of 2026-10-15 | **36** refused by the limit, against 35. Re-taken: `ein oracle examples/zebra.ein` exits 2 with *56 root candidates, over --region-limit 16 … pass --region-limit 56 to compute it* |
| **G-10** | `solution_semantics.md` § 6 | the page | as 11-05 |
| **G-11** | **The modes' scope** (*minimum work per question*) | `solve_interface.md`, `read_out.md` | every program. S-16's two words are this scope as `-n` defines it, read past the limit |
| **G-12** | **Claims the language cannot state** | Q-M1e.13, Q-M1c.6, Q-M1c.7 | as 11-05 |
| **G-13** | **The core's promise.** The smallest frontier over the derivations the program records, bounded by the cap (F22) and the search's budget. Since W23 the cut is said (`core_status`), and the exact core past the budget is F22's. Since Q-M1e2.15 (b) the `forall` fold is a second residual, W-20 | Q-M1e2.11, Q-M1e2.15 | Re-taken: 19 of N's 5 477 cores, and 1 root core, are printed *cut* (`sweep_cores.py`) |
| **G-14** | **Checked at firing only.** A bad `:assert` stops the run at its first firing, so a rule that never fires is never checked | S1e2.2.18's default 4 | Re-taken on `plans/m1e2_correctness/inventory_2026-11-06/probes/assert_never_fires.ein`: `_`, an atom, and a variable the `:match` does not bind, each in a rule that never fires, load with exit 0. A keyword pair *in* an `:assert` pattern is refused at load. `ein solve examples/ein-bugs/unbound-assert-var.ein --count` exits 1 at its firing |
| **G-15** | **A keyword pair in a pattern is dropped** (Q32) | Q32; S1e2.2.20 and S1e2.2.24, default 2 | Re-taken: `…/inventory_2026-11-05/probes/kwpair_in_pattern.ein`, `(n ?x :where 1..3)`, fires on `(n a)` |
| **G-16** | **Obligations.** A ground obligation is refused. With no hrule, H can be empty. A guard over D declines the rung | Q-M1e2.10 (a) | as 11-05 |
| **G-17** | **Artefacts** | Q-M1e.22 | Re-taken: `ein solve … --json-summary /nonexistent/dir/x.json` prints `error: …` and exits 0. `ein test A B --json-summary J` exits 2 |
| **G-18** | **Coverage of the checks** | E1X-29 and others | `tests/mutation_survivors.txt` holds 52 survivors, 42 of them in `slots.ein`: the same as 11-05 |
| **G-19** | **The close's own re-take** | **ruled 2026-11-05** ([Q-M1e2.16](open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items) (6)): the register's tests plus this sweep | the re-take ran as written (§ C.1). `probes/capture.sh`'s W lines and the four § 4 sweeps are out of it |

**New since 11-05:**

| id | gap | ruling / page | corpus programs today (2026-11-06) |
|---|---|---|---|
| **G-20** | **`refuted by` names the first recorded derivation the core supports.** Two programs one `:priority` apart name two rules beside one core | **ruled** Q-M1e2.21 (a), the accepted cost | Re-taken: `ein solve plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.10_probes/two-rules.ein` prints `unsat core (p A)`, `refuted by wide-a`. `-reordered.ein` prints the same core with `refuted by narrow-b`. S1e2.3.10 counts 5 041 of 230 154 printed cores per commit deriving a contradiction's fact through two rules |
| **G-21** | **A rule may conclude a fact of the reserved, computed `neq`** | **unruled**, owner none ([S1e2.2.22 § Notes](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md#notes), 1st item) | Re-taken: `ein saturate plans/m1e2_correctness/inventory_2026-11-06/probes/assert_neq.ein --dump` stores `(neq w a :rule r)` with no line. 0 corpus programs |
| **G-22** | **A keyword pair beside an `:assert`'s conjuncts**, `(and (p ?y) :k v)`, is dropped in silence. This is G-15's neighbour in a conclusion | **unruled**, owner none ([S1e2.2.24 § Notes](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md#notes), 2nd item) | Re-taken: `ein saturate plans/m1e2_correctness/inventory_2026-11-06/probes/assert_and_kw.ein --dump` stores `(p w :rule r)`, and `:k v` is gone. 0 corpus programs |
| **G-23** | **A keyword ends at the word boundary.** `(neq-1 ?x)` reads as `neq` and `-1`. A trace word a word character follows is a parse error | **ruled** [Q-M1e2.17](open_questions.md#q-m1e217--where-does-a-keyword-end) (i)(a), (ii)(a) | 0 programs write the shape (Q-M1e2.17's grep) |
| **G-24** | **The line modes past the limit, on a flagged program.** `--minimal` certifies describe's line, which is {∅} where ∅ ∈ R. `--maximal` certifies none and says the maximal line is not known | **ruled** [Q-M1e2.19](open_questions.md#q-m1e219--what-do---minimal-and---maximal-answer-past-the-limit) (2) | the 7 flagged programs stopped at the limit |
| **G-25** | **A member of N past the limit has no core**: it is written `not-kept` | S1e2.3.9's default. Keeping them is the user's call (P-15 (a)) | `lattice/06` 68, `past-limit-lines` 15 |
| **G-26** | **A reloaded state's facts read as given.** The provenance is a comment the loader skips | S1e2.3.8's default 4; `answer_dump.md` | every `--dump-answer` state |

---

## C. The intersections, measured

### C.1 What was run (2026-11-06, snapshot `c523f3ab`, release binary, `nice -n 19`, one engine process at a time)

| command (from `build/m1e2_inventory_2026-11-06/`) | over | wall | output |
|---|---|---:|---|
| `cargo build --release -p ein-cli --manifest-path src/ein.rs/Cargo.toml --target-dir target` | the snapshot tree | 12 s | `target/release/ein`, sha256 `8f2f1ff5…` |
| `cargo test --release --manifest-path src/ein.rs/Cargo.toml --target-dir target -p ein-cli --test wrong_answers -- --test-threads=1`, then the same with `-p ein-infer --test register_region` (`TMPDIR=scratch/tmp`) | every register row | 13.2 s + 3.3 s run | `register_tests.log`: both `ok` |
| `EIN_STDLIB=src/stdlib python3 sweep_describe.py` | a bare `ein solve FILE --json-summary J` on all 380 programs: every `corpus.toml` entry and every register witness | 46 s | `corpus_describe.json` |
| `TMPDIR=$PWD/scratch/tmp python3 src/utils/region_census.py --all -j 1 --bin $PWD/target/release/ein --json $PWD/region_census.json` | `ein oracle FILE --dump` on the same 380 (the snapshot's own instrument) | 23 s | `region_census.json`, `region_census.txt` |
| `python3 sweep_cores.py` | `ein solve FILE --dump-answer DIR` and `ein saturate --dump` on the 114 entries with N > 0 or a dead root | 92 s | `corpus_cores.json` |
| `python3 classify.py` | the join, with no engine | — | [`corpus.csv`](inventory_2026-11-06/corpus.csv), `corpus_classes.json` |
| `python3 sweep_modes.py` (**new**) | `--minimal`, `--maximal`, `--count` and `--single` on the 250 programs describe and the oracle both finish, against the oracle's own dump | 27 s | `corpus_modes.json`: 250 programs, 0 disagreements |
| `python3 sweep_lines.py` (**new**) | describe's minimal-line row and its owing holes, on the 286 programs that load | 45 s | `corpus_lines.json`: F1's 8, and D-14's 7 holes |
| `python3 retake_items.py` (**new**) | every item's witness (§ A, § B) | 28 s | `retake_items.log`, and `retake/<id>/` |
| the S-16 loop: for each of the 7 flagged programs stopped at the limit, `ein solve FILE --minimal --single`, then `--minimal -n 1`, then a bare `ein solve FILE` | S-16, P-14 | 30 s | `s16_sweep.log` |

**Describe against the oracle.** There are 250 programs where both finish. On each, describe and the oracle agree on k_s, k_c, |N|, the hole count and the static verdict, so there are 0 disagreements (`classify.py`'s `agree_describe_oracle`).
- **A null hole count.** Describe writes `holes: null` on a monotone program. That program prints no hole row, and has no hole by the convexity [`monotonicity.md`](../../docs/kernel/inference/monotonicity.md) states. A null is read as 0 there, and the oracle's count is 0 on all 230 monotone cubes.
- **The two the oracle answers and describe does not.** These are G-2's two query-goal refusals: `query-goal-free-head` and `features/02`.

**The modes against the oracle.**
- **The lines.** `--minimal` and `--maximal` each list exactly the states on the oracle's line: 222 and 283 states over the 250 programs.
- **The counts.** `--count` gives the oracle's k_s and k_c.
- **The words.** `--single`'s word is the one k_s implies: 92 `Solution`, 109 `Ambiguity`, 29 `Contradiction` and 20 `Open`.
- **The result.** 0 disagreements. `ein-infer/tests/explore.rs` holds every mode to the oracle per commit.

**Script changes, each recorded in its file:**
- **`sweep_describe.py`.** The commit it records is `c523f3ab`, or `$EIN_INVENTORY_COMMIT`; it was hard-coded `cdcc3994`. Each row also keeps the summary's `core_status`, which S1e2.2.23 added beside root's core.
- **`sweep_cores.py`.**
  - The commit, as above.
  - Since S1e2.3.9, a member past the limit is written `core: null`, `core_status: "not-kept"`, so `core_empty` now counts `[]` only, and a new `core_null` counts the nulls.
  - Each member's `core_status` is counted, and so is root's.
  - The old count read a null as empty, and would have reported N6 as live.
- **`classify.py`.**
  - `agree_describe_oracle` compares the five fields § C.1 names. It compared three: k_s, k_c and |N|.
  - New columns: `N_cores_null`, `N_core_status` and `root_core_status`.
- **`region_census.py`.** Unchanged: it is `utils/`' own, from the snapshot. Its invocation changed. It runs with the snapshot as its working directory, so 11-05's relative `--bin target/release/ein` did not resolve, and `--bin` and `--json` are given as absolute paths.
- **New scripts.** `sweep_modes.py`, `sweep_lines.py` and `retake_items.py`. Each states its question in its header.

### C.2 The classes, and who is in them

The five classes partition the corpus: 248 + 9 + 29 + 19 + 75 = 380. On 11-05 they were 209 + 9 + 21 + 19 + 68 = 326. The sweep gained 56 programs and lost 2:
- **Gained:** the register witnesses and twins of W21's `forall` rows, W22, W23, W24, W25, W26, W27 and W28, and S1e2.2.25's rows.
- **Lost:** `examples/broken/compile/empty_absent.ein` and `unbound_head.ein`, which moved to `examples/broken/load/`.

No program changed class. HS and TO are attributes of programs that load.

| class | n (11-05) | members | what reaches it |
|---|---:|---|---|
| **monotone program, complete** (MC) | 248 (209) | every loaded program not listed in the next two rows. The 39 new ones are register witnesses and twins | W-3, W-9, W-10, W-18, W-20; W-22 (probe); S-15, S-17 and F1, S-18 (probe); G-3 to G-6, G-11 to G-17, G-20 to G-22, G-26 |
| **monotone program, stopped** (MS) | 9 (9) | `saturation/square-{bwd,fwd}/{floors,houses,meetings}` (k_s ≥ 23 390, k_c ≥ 2²⁰), `syntax/arg-kinds` (k_s ≥ 112 793), and `zebra2-minus-15` and `-obligations` (k_s ≥ 32, k_c ≥ 865, N ≥ 1 495). All stopped at the budget | W-22 (`arg-kinds`); G-6, G-8 |
| **non-monotone program, flagged** (NM) | 29 (21) | the three lists below this table | W-1, W-2, W-5, W-6, W-8, W-15, W-23; S-6 to S-9, S-11, S-12, S-16, S-17 (at a lowered limit); D-14; G-7, G-8, G-9, G-24, G-25 |

**The 22 flagged programs that complete within the limit.** The 7 *exercised* ones, the same as 11-05:
- `lattice/04`, `/08`, `/09`, `/12`;
- `synth/local_not_global`, `synth/maximal_not_top`;
- `tests/region/07_cube`.

The 15 *conservative* ones:
- **8 as on 11-05:** `features/07`, `f07-fpoff`, `lattice/10_fifo_gap`, `core-activator-committed`, `core-activator-guard-only`, `forbid-chain-no-absent`, and `oblig-stdlib-total-owed` and its `-declared` twin.
- **7 new:** `f07-rules-swapped`, `guard-only-first` and `-second`, `kw-neq-in-absent` and `-twin`, and `oblig-derived-activator` and `-declared`.

**The 7 flagged programs stopped at the limit**, all *not known*:
- `features/01`, `features/04_open`, `lattice/06_friends_tables`;
- `square-unique/{corner-house,cul-de-sac,terminus}`;
- **new:** `ein-bugs/wrong-answers/past-limit-lines` (n 17).

| class | n (11-05) | members | what reaches it |
|---|---:|---|---|
| **refused by *forbid*** (FB) | 19 (19) | The same 19: `branching/13`, `/14`; `activator-all-{int,nested,symbol}`; `alive-empty-interlayer`; `d3-unsat-core`; `naf-upward-closure`; `forbid-chain-{0,1,2,node-absent,node-writes,param-absent}`; `naf-nowb`; `stale-naf-fork` and `-warn`; `lattice/synth/three_components`, `trigger_hole`. Each exits 1 with the line of § 4.7 | none of R. S-8, S-9 and S-13 are invisible here. `ein saturate` still loads them, so W-22 could reach one (0). D-4 |
| **refused at load** (LD) | 75 (68) | G-2's list | D-1, D-2 (the messages), D-15 (the page's example); G-2. W-12, W-13, W-14 and W-21 are fixed by being refused here |
| **H-scoped** (HS) | 32 (24) | 27 *no-hrule*, 1 *arity* (`arity3`) and 4 *not-owed* (`uncovered`, `uncovered-knows`, and the two `-obligations` zebras). 21 are MC, 9 NM (`core-activator-guard-only`, `forbid-chain-no-absent`, `guard-only-first`, `guard-only-second`, `lattice/04`, `/10`, `/12`, `tests/region/07`, and `past-limit-lines`, which is stopped), and 2 MS (the `zebra2-minus-15` pair). The 8 new: `forall-core-elim`, `kw-neq-in-goal` and `-twin`, `pair-core-root-derived`, `pair-core-root-negative`, `guard-only-first` and `-second`, and `past-limit-lines` | W-5 (`lattice/10`), W-8 (`guard-only-second`), W-9 (`fork-delta-order`), W-18 (`zebra2`, `zebra2-obligations`), W-20 (`zebra2-hints`); S-12 (4), S-16 (`past-limit-lines`); G-3 |
| **types as objects** (TO) | 10 (10) | The same 10: `branching/02`, `/08`, `transitive/taxonomy`, `type-exclusivity/{colors,nationalities,pets}`, `features/01`, and `square-unique/{corner-house,cul-de-sac,terminus}`. 6 are MC and 4 NM (stopped) | W-6 (`features/01`, `square-unique` ×3), S-11, S-16 (4); G-4 |

**What the detector said on each class**, from `monotonicity` in each summary:
- **MC and MS, all 257 programs:** *monotone (the static check)*.
- **NM, the status bit:** as listed above.
- **NM, stale facts (the `state` row): 11 programs, against 8.**
  - `features/07`, `f07-fpoff` and `f07-rules-swapped`: 1 of 2;
  - `lattice/10`: 1 of 4;
  - `kw-neq-in-absent` and `-twin`: 4 of 8 (**new**, W-23);
  - the five past the limit of W-6: `features/01` 24 754 of 64 655, `features/04_open` 21 042 of 64 761, `corner-house` 5 595 of 65 389, `cul-de-sac` 5 835 of 64 923, and `terminus` 4 876 of 65 029, all unmoved.
  - `lattice/06` (0 of 65 299) and `past-limit-lines` (0 of 65 536) hold none.
- **NM, a parent path differs:** `lattice/08` and `lattice/10`.
- **NM, holes:** on 6 programs. `lattice/04`, `/08`, `/09`, `/12` and `tests/region/07` have 1 each, and `synth/local_not_global` has 2. Every one is owing and written `core: []` (D-14). `synth/maximal_not_top` has none, but discharged is not upward-closed there (2 nodes).
- **The census of the same day** (`region_census.py --all`): 0 violations on 230 monotone cubes. Of the 22 flagged cubes, 6 have a hole, 7 have discharged not upward-closed, and 0 have dead not upward-closed. No dead hole loads, which is *forbid*'s effect.

**The cores, over the 114 programs with N > 0 or a dead root** (`sweep_cores.py`):
- **N's members.** There are 5 477. 5 375 are `within-budget`, 83 `not-kept` (`lattice/06` 68 and `past-limit-lines` 15), and 19 cut: 13 at the environments (`zebra2` 2, `zebra2-obligations` 2, the `zebra2-minus-15` pair 4 each, and `core-search-cut` 1) and 6 at the rounds (`zebra.ein`).
- **No empty core.** 0 are written `core: []`.
- **W21's claim.** 0 lie wholly inside root's fixpoint while root is alive, and 0 name no fact of their set.
- **Root's cores.** 34 programs print one: 33 `within-budget`, and 1 cut, `core-search-cut`.

### C.3 Item × class, cell by cell

The summary's matrix, in full. A "–" names the mechanism that excludes the class.

| item | MC | MS | NM | FB | LD | HS | TO |
|---|---|---|---|---|---|---|---|
| **W-1** | – : a stale positive needs an `absent` site over D, and that site flags the program | – : the same | ● `features/07`, `f07-fpoff`. R: the state's content, which the listing hides and `-p` shows | – : the refuting variant (`stale-naf-fork`) is refused | – | 0 | 0 |
| **W-2** | 0: none of 167 reordered cubes (`node_census -- --invariance`, 2026-11-02; not re-taken) | 0 | ● `features/07` against `f07-rules-swapped`: R and H move | – | – | 0 | 0 |
| **W-3** | ● `alt-cap-shared`, the core of a dead root | 0 | 0 | – : no core is printed | – | 0 | 0 |
| **W-5** | – : by fork, a monotone program's node is one state on every path. `lattice/11_fifo_gap_static` differs only from scratch, which no route builds | – | ● `lattice/10` | – | – | ● `lattice/10` | 0 |
| **W-6** | – : as W-5 | – | ● 5, stopped | – | – | 0 | ● 4 |
| **W-8** | – : it needs an activator a node derives, which is an activator site | – | ● `guard-only-second`. The `-first` twin answers right | – | – | ● the same | 0 |
| **W-9** | ● `fork-delta-order` | ? | ? | – | – | ● the same | ? |
| **W-10** | ● `closure-shared-given`, the core of a dead root | ? | ? | – | – | ? | ? |
| **W-15** | – : it needs an activator site | – | ● `oblig-derived-activator`. H: the debt's discharging fact is never a candidate | – | – | 0 | 0 |
| **W-18** | ● `zebra2`, `zebra2-obligations`: 9 of 366 cores each (S1e2.2.23) | ? : the `zebra2-minus-15` pair is partial and not measured | ? | – | – | ● the same 2 | 0 |
| **W-20** | ● `zebra2-hints`: 13 of 73 (S1e2.2.21 T1) | ? | ? | – | – | ● the same | ? |
| **W-22** | ○ `fact_wild_arg.ein` | ● `syntax/arg-kinds` | 0 | 0 : *forbid* refuses R, and `ein saturate` would still load the fact wrong | 0 : a program refused for another reason may also write the shape | 0 | 0 |
| **W-23** | – : as W-1 | – | ● `kw-neq-in-absent`, `-twin` | – | – | 0 | 0 |
| **S-6, S-7** | – : the detector runs on flagged programs only | – | ? | ? | – | ? | ? |
| **S-8, S-9** | – | – | ? | ● the witnesses (invisible) | – | ? | ? |
| **S-11** | – : relation-grain sites flag the program | – | ● `features/01` | – | – | 0 | ● the same |
| **S-12** | – : a monotone program's lines are trivial by theorem | – | ● 13: `f07-fpoff`, `f07-rules-swapped`, `features/07`, `kw-neq-in-absent` and `-twin`, `oblig-derived-activator-declared`, `lattice/04`, `/08`, `/09`, `/10`, `/12`, `synth/local_not_global`, `tests/region/07` | – | – | ● 4 (`lattice/04`, `/10`, `/12`, `tests/region/07`) | 0 : the TO programs that are flagged are stopped, and print *not known* |
| **S-13** | – | – | – : no dead hole loads | its dead holes, invisible | – | – | – |
| **S-15** | ○ : any program, through its models file | ○ | ○ | – | – | ○ | ○ |
| **S-16** | – : within the limit `--single` decides. `-n 1`'s wording there is G-11 | – : a monotone run stopped at the budget has its own wording | ● 7, every flagged program stopped at the limit | – | – | ● `past-limit-lines` | ● 4 |
| **S-17 and F1** | ● 8 (F1): `tests/stdlib/` programs that state an obligation root discharges | 0 | ○ : `past-limit-lines` at `--region-limit 0` and `1`, which are non-default | – | – | 0 | 0 |
| **S-18** | ○ `goal_guard.ein` | 0 | 0 | – | – | 0 | 0 |

### C.4 The witnesses' own status

Each live W or S item's witness: its detector verdict, and R as `ein solve FILE --json-summary` reports it (2026-11-06, `c523f3ab`; `corpus_describe.json` unless the row says otherwise).

| witness | item | detector | k_s / k_c | N | H | exhausted |
|---|---|---|---|---:|---:|---|
| `examples/features/07_unstratifiable.ein` | W-1, W-2 | flagged, conservative (`derive-p`, `derive-q`: `absent`, an even cycle) | 2 / 2 | 0 | 1 | yes (route 2) |
| `ein-bugs/wrong-answers/f07-fpoff.ein` | W-1 | the same | 2 / 2 | 0 | 1 | yes |
| `ein-bugs/wrong-answers/f07-rules-swapped.ein` | W-2 | the same | 2 / 2, `{(p one)}` | 0 | 1 | yes |
| `ein-bugs/wrong-answers/alt-cap-shared.ein` | W-3 | monotone | 0 / 0, root dead | 1 | 133 | yes |
| `examples/lattice/10_fifo_gap.ein` | W-5 | flagged, conservative (`r2`: `absent`) | 4 / 4 | 0 | 2 | yes |
| `square-unique/corner-house`, `cul-de-sac`, `terminus`; `features/01`, `features/04_open` | W-6 | flagged, *not known* | ≥ 65 214, ≥ 64 817, ≥ 64 514; ≥ 64 655, ≥ 64 761 | 0 | 118, 114, 153; 35, 81 | no (limit) |
| `ein-bugs/wrong-answers/guard-only-second.ein` (and `-first`) | W-8 | flagged, conservative (`w`: activator) | 2 / 2 | 1 | 2 | yes |
| `ein-bugs/fork-delta-order.ein` | W-9 | monotone | 3 / 3 | 1 | 2 | yes |
| `ein-bugs/wrong-answers/closure-shared-given.ein` (and `-control`) | W-10 | monotone | 0 / 0, root dead | 1 | 0 | yes |
| `ein-bugs/wrong-answers/oblig-derived-activator.ein` | W-15 | flagged, conservative (`total-owed`: activator) | 0 / 0, every alive node owes | 0 | 1 | yes |
| `…/oblig-derived-activator-declared.ein` | W-15's control | the same | 1 / 1 | 0 | 2 | yes |
| `examples/zebra2.ein`, `zebra2-obligations.ein` | W-18 | monotone | 1 / 261 982 | 366 | 56 | yes |
| `examples/zebra2-hints.ein` | W-20 | monotone | 6 / 8 316 | 73 | 36 | yes |
| `examples/syntax/arg-kinds.ein` | W-22, G-6 | monotone | ≥ 112 793 / ≥ 2⁴¹ | 0 | 41 | no (budget) |
| `inventory_2026-11-06/probes/fact_wild_arg.ein` | W-22 | monotone | 32 / 32 (`retake_items.py`) | 0 | 5 | yes |
| `ein-bugs/wrong-answers/kw-neq-in-absent.ein` (and `-twin`) | W-23 | flagged, conservative (`r`: `absent`) | 8 / 8 | 0 | 3 | yes |
| `examples/features/01_not_and_absent.ein` | S-11, S-16 | flagged, *not known* | ≥ 64 655 | 0 | 35 | no (limit) |
| `examples/lattice/04_obligation_trigger_hole.ein` | S-12, D-14 | flagged, exercised (`total-owed`: trigger), 1 hole | 3 / 3 | 0 | 2 | yes |
| `ein-bugs/wrong-answers/past-limit-lines.ein` | S-16, S-17, D-14, P-14 | flagged, *not known* (`total-owed`: trigger) | ≥ 3 / ≥ 3 | ≥ 15 (`not-kept`) | 17 | no (limit) |
| `examples/lattice/06_friends_tables.ein` | S-16, P-15, G-25 | flagged, *not known* (`group-seated-owed`: trigger) | ≥ 21 / ≥ 21 | ≥ 68 (`not-kept`) | 19 | no (limit) |
| `tests/stdlib/algebra/24_total_owed_satisfied.ein` | F1 | monotone | 1 / 1 | 0 | 0 | yes |
| `inventory_2026-11-06/findings/F1_trivial_line_stated_obligation.ein` | F1 | monotone | 1 / 1 | 0 | 0 | yes |
| `inventory_2026-11-06/probes/diff_models_text.ein` | S-15 | monotone | 4 / 4 | 0 | 2 | yes |
| `inventory_2026-11-06/probes/goal_guard.ein` | S-18 | monotone | 1 / 1 | 0 | 0 | yes |

---

## D. The 2026-10-14 cases, re-taken

The cases of [`inventory.md`](inventory.md), as they stand at `c523f3ab`. A W case is re-taken through the register: `wrong_answers.rs` passes on the snapshot, so every row answers what it records. Each case reads as on 11-05 ([§ D](inventory_2026-11-05.md)), except as follows:
- **W8** has four rows open, deferred to F21: `features-07`, `f07-rules-swapped`, `fifo-gap` and `corner-house`. `stale-naf-fork` is still fixed by refusal.
- **W10** has five rows open, deferred to F22: `alt-cap-shared`, `fork-delta-order`, `closure-shared-given` and `second-preparation`, beside the fixed `alt-cap-core`. W25, also F22's, is the fifth deferred row.
- **W11** has its three rows fixed. `ein solve examples/ein-bugs/unsat-core-id-space.ein --dump-answer D` prints its four N cores, each naming its set.

The S and P lines, re-run (2026-11-06):
- **S1** `ein solve -n 1 examples/zebra2.ein` prints *no — stopped as asked; each state shown is certified, nothing is said of the rest*.
- **S2** `ein solve …/pets.ein --region-budget 100 --trace T.md`: the trace is headed by *states (k_s) 16 (a lower bound …)*, and *No solution* appears 0 times.
- **S3** The same run's summary has `verdict null`, `exhausted false` and `stopped.why budget`.
- **S5** `ein solve examples/zebra.ein -v -g -3` exits 2.
- **S6** `ein solve examples/lattice/02_genuine_3set_death.ein` prints *the goal has no row: undetermined here*.
- **S8** `stale-naf-fork` is refused.
- **S9** `EIN_TRAVERSAL=tree ein solve examples/branching/01_saturate_only.ein` leaves stdout byte-identical.
- **P1** `ein render constraints` and `ein render program` on `kwpair-below-the-filter.ein` exit 0.

The § 6 *suspected* items:
- **Q-M1e.11** now has a witness, `W26/oblig-derived-activator` (W-15).
- **`hrule-reads-not.ein`** (`corpus/fuzz_findings/`): R = {∅}, no candidate, exit 0. The debug assertion was not re-taken.
- **`emit_closed`** is still run by no route (D-13).

---

## Files

Taken under `build/m1e2_inventory_2026-11-06/`. Kept in the repository:
- **This document**, `inventory_2026-11-06.md`. [The inventory of 2026-11-05](inventory_2026-11-05.md) is unchanged, as the record.
- **The scripts**, as run, in [`inventory_2026-11-06/`](inventory_2026-11-06/):
  - 11-05's three, with the changes § C.1 lists: `sweep_describe.py`, `sweep_cores.py` and `classify.py`;
  - three new ones: `sweep_modes.py`, `sweep_lines.py` and `retake_items.py`.
  - `region_census.py` is `utils/`' own.
- **The join**: [`corpus.csv`](inventory_2026-11-06/corpus.csv), one row per program, 380 rows.
- **The probes**, each with its question in its header, in [`inventory_2026-11-06/probes/`](inventory_2026-11-06/probes/):
  - `fact_wild_arg` (W-22), `diff_models_text` and its two models files (S-15), `goal_guard` and `goal_guard_claim` (S-18);
  - `unused_param` (D-9), `where_t3_example` (D-15), `empty_arg` (S-1);
  - `var_in_fact` (G-6), `assert_never_fires` (G-14), `assert_neq` (G-21), `assert_and_kw` (G-22).
  - Five of them are S1e2.2.22's and S1e2.2.24's notes' programs, copied from `build/` with a header added.
- **The finding**: [`findings/`](inventory_2026-11-06/findings/) holds F1 and its control.

**Not kept in the repository.** The JSON sweeps (`corpus_describe.json`, `region_census.json`, `corpus_cores.json`, `corpus_modes.json`, `corpus_lines.json`, `corpus_classes.json`) and the answer dumps under `retake/` were deleted once this was committed, because the scripts re-take them. The run logs stay under `build/m1e2_inventory_2026-11-06/`: `register_tests.log`, `retake_items.log`, `s16_sweep.log`, each sweep's own log, and `retake/<id>/`'s stdout and stderr.

**To re-take.**
1. Copy `inventory_2026-11-06/` under `build/`.
2. Put a `git archive` of the tree being re-taken in its `src/`, and copy `inventory_2026-11-06/probes/` and `findings/` to the same path inside `src/`, if the tree is older than this document.
3. Build into its `target/`.
4. Run, with `EIN_STDLIB=src/stdlib`: `sweep_describe.py`, then `utils/region_census.py --all` (with absolute `--bin` and `--json`), then `sweep_cores.py`, `classify.py`, `sweep_modes.py`, `sweep_lines.py` and `retake_items.py`.
5. Run the register's two tests on the same tree.

Each script reads the tree from `./src` and the binary from `./target/release/ein`, and writes its JSON beside itself.
