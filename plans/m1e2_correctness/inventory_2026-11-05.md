# M1e2 inventory: known incorrect results, solution gaps, and where they meet R

> **Taken 2026-11-05** at `cdcc3994`, by a read-only agent on a snapshot build, at the user's request before the close. The user ruled its items the same day, [Q-M1e2.16](open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items): F1 and F2 are W22 and W23, and every item without an owner has one. The close re-takes it. Its probes, findings, scripts and `corpus.csv` are in [`inventory_2026-11-05/`](inventory_2026-11-05/). The JSON sweeps were not kept, because the scripts re-take them.

**Taken 2026-11-05 from the snapshot at `cdcc3994`** (`git archive cdcc3994`, under
`build/m1e2_inventory/src/`). The binary is a release build of that tree
(`cargo build --release -p ein-cli`, 32 s), run with `EIN_STDLIB=src/stdlib`.
A number marked *re-taken* was taken on that binary on 2026-11-05, and its row
says which command took it. Any other number is quoted from the page it cites. The milestone's close waits on this document: it lists what
the engine still gets wrong, where it gives no answer or only part of one, and
which programs each defect can reach.

Links are relative to this file's place in the repository (`plans/m1e2_correctness/`). It was written under `build/m1e2_inventory/`, and the commands below run from there.
The words follow
[`solution_semantics.md`](../../docs/kernel/inference/solution_semantics.md) and
[`monotonicity.md`](../../docs/kernel/inference/monotonicity.md). *Monotone* and
*non-monotone* describe a **program**, never the lattice.

---

## 0. Summary

**The register.** All 71 rows answer what they record on the snapshot binary
(`cargo test --release -p ein-cli --test wrong_answers`, and `-p ein-infer --test
register_region`; both pass). It reads 68 fixed, 2 open and 1 not-wrong. The
2 open rows are the ones the user deferred: `W8/features-07` to F21 and
`W10/alt-cap-shared` to F22.

**Live items, by class** (§ A has one row each; fixed items are left out):

| class | live | no owner | re-taken live today | contested (a ruling decides) | latent (no witness) | not re-takable from the CLI |
|---|---:|---:|---:|---:|---:|---:|
| **W** wrong answer (R, verdict, N, core, H, exit) | 15 | 7 | 13 | 5 | 2 | 0 |
| **S** a surface misreports a right answer | 14 | 10 | 7 | 0 | 3 | 4 |
| **D** diagnostic or spelling | 11 | 11 | 8 | 1 | 0 | 2 (D-9 was not run) |
| **G** gap: no answer, a refusal, a scoped answer, a lower bound (§ B) | 19 | — | — | — | — | — |
| **P** cost only (§ A.4) | 13 | — | — | — | — | — |

**Two new findings.** Neither is named by a register row, a stage, a note or a
question. Their probes are banked under [`findings/`](inventory_2026-11-05/findings/).

- **F1 (W). A rule pattern splits a name that begins with `neq`, `or` or
  `and`.** Inside a rule pattern, `(neqw ?x)` parses as `(neq w ?x)`, and the
  same happens to `(orz ?x)` and `(andz ?x)`. The grammar says the opposite:
  [`00_ebnf.md` § 1](../../docs/kernel/ir/03-ein-lang/00_ebnf.md), fact 1 reads
  *"`neq` is not a SYMBOL; `neq_test` is"*. Three things follow, all silently:
  - a rule stores a fact of the reserved relation `neq` or `or`;
  - `w` becomes a name of the program;
  - a query goal `(neqw ?y)` turns into a guard.

  On [`F1_keyword_prefix_split.ein`](inventory_2026-11-05/findings/F1_keyword_prefix_split.ein),
  `ein solve` answers k_s 8 over H = 3 and prints the query as `(neq w ?y)`.
  The right answer is R = {∅}, H = ∅ and one class, ?y = a. No corpus program
  writes such a name (checked with grep).
- **F2 (W, core). A core search that hit its budget prints a core larger
  than the smallest, with no qualifier.** The core search keeps 64
  environments per label (`ExplanationBudget`). `region.rs:1122` and `:1268`
  call `explain::smallest_contradiction_frontier`, which discards
  `Explanation::exhausted` (`explain.rs:618–623`). So a core from a cut search
  reaches the read-out, `--json-summary` and `nogoods.json` with nothing to say
  it was cut.

  On [`F2_core_cut.ein`](inventory_2026-11-05/findings/F2_core_cut.ein) the core is 10 facts,
  `(x1 k) (x2 k) (y1 k) … (y8 k)`, and the eight `y` facts alone derive
  `(false)` ([`F2_core_cut_control.ein`](inventory_2026-11-05/findings/F2_core_cut_control.ein)).
  With five pairs instead of eight, under the budget,
  ([`F2_core_cut_5.ein`](inventory_2026-11-05/findings/F2_core_cut_5.ein)) the search finds the
  all-`y` core. [S1e2.2.11](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md#two-findings-and-why-the-stage-stops-here)
  says this cut "is reported (`Explanation::exhausted`)", and F22 assumes the
  same. Both hold only for a library caller. F2 is not F22's case: no fact in
  the probe has more than 2 of the 32 alternatives the cap allows. The
  extraction pass found the mechanism by reading the source (D1X-41), and the
  probe was written here.

**The matrix: item × program class** (§ C.3 gives every cell's reason). The
column counts are the corpus entries in each class, from § C.2.

- **Columns.** MC = monotone and complete (209), MS = monotone and stopped (9),
  NM = non-monotone, flagged (21), FB = refused by *forbid* (19), LD = refused
  at load (68), HS = H-scoped, `outside` ≠ ∅ (24), TO = types bound as objects
  (10). HS and TO overlap the first five.
- **Cells.** **●n** = measured on n corpus programs today; **○** = shown on a
  probe only; **0** = can reach the class, but measured on none; **–** = cannot
  reach the class, for the reason in § C.3; **?** = not measured.
- **Touches.** Which part of the answer the item gets wrong: R (members or
  counts), N (the dead set and its cores), H (the universe), s (a surface only).

| item | owner | MC | MS | NM | FB | LD | HS | TO | touches |
|---|---|---|---|---|---|---|---|---|---|
| W-1 `W8/features-07` stale positive | F21 D2 | – | – | ●2 | – | – | 0 | 0 | R |
| W-2 R moves with rule text order | F21 D2 | 0 | 0 | ●2 | – | – | 0 | 0 | R, H |
| W-3 `W10/alt-cap-shared` | F22 | ●1 | 0 | 0 | – | – | 0 | 0 | N (core) |
| W-4 W21 pair core one-sided | S1e2.2.21 | ●7 | ●2 | 0 | – | – | ●6 | ●1 | N (cores) |
| W-5 FIFO-gap stale `p` (`lattice/10`) | F21 D2 | – | – | ●1 | – | – | ●1 | 0 | R |
| W-6 stale facts past the limit | F21 D2/D7 | – | – | ●5 | – | – | 0 | ●4 | R (lower bound) |
| W-7 **F2** core-search cut unreported | **none** | ○ | ? | ? | – | – | ? | ? | N (core) |
| W-8 premise-less alternative lost | **none** | – | – | ○ | – | – | ? | ? | N (core) |
| W-9 non-canonical equal-size core | F22 (default) | ●1 | ? | ? | – | – | ●1 | ? | N (core) |
| W-10 closure estimate heuristic | **none** | ? | ? | ? | – | – | ? | ? | N (core) |
| W-11 **F1** keyword-prefix split | **none** | ○ | 0 | 0 | – | – | 0 | 0 | R, H |
| W-12 `_` in a guard | **none** | ○ | 0 | 0 | – | – | 0 | 0 | R |
| W-13 a premise that is not a form dropped | **none** | ○ | 0 | 0 | – | – | 0 | 0 | R |
| W-14 nested opaque slot never matches | **none** | ○ | 0 | 0 | – | – | 0 | 0 | R |
| W-15 obligation derived under a hypothesis | S1f.10.6 (M1f) | – | – | ? | – | – | ? | ? | H |
| S-1 N5 fact printer | S1e2.3.8 | ●2 | ●1 | ? | ? | – | ? | ? | s |
| S-2 N6 empty cores on a stopped run | S1e2.3.9 | 0 | 0 | ●1 | – | – | 0 | 0 | s (N) |
| S-3 `refuted by` names a rule the core does not use | **none** | ●2 | ? | ? | – | – | ? | ? | s (N) |
| S-4 `ein test --json-report` drops `outside`/`types` | **none** | 0 | 0 | ●1 | – | – | ●1 | 0 | s |
| S-5 a reloaded dump loses provenance | **none** | ○ | ○ | ○ | – | – | ○ | ○ | s |
| S-6…S-11 detector and check labels | **none** | – | – | ●1 (S-11), else ? | ? | – | ? | ●1 (S-11) | s |
| S-12, S-13 hole rows on a flagged program | F21 D4 | – | – | ●1 | – | – | ●1 | ? | s |
| S-14 `hyp` digest of a dead generator | **none** | test surface only | | | | | | | s |

**Every W or S item with owner none** (7 W, 10 S):

| id | item | witness, re-taken 2026-11-05 at `cdcc3994` |
|---|---|---|
| W-7 | **F2**: a core-search cut is printed as *the* core (new) | [`findings/F2_core_cut.ein`](inventory_2026-11-05/findings/F2_core_cut.ein) |
| W-8 | a premise-less derivation from a node-derived activator is never an alternative, so rule priority picks a 3-fact core over the 1-fact one | [`probes/guard_only_second.ein`](inventory_2026-11-05/probes/guard_only_second.ein) vs [`_first`](inventory_2026-11-05/probes/guard_only_first.ein) |
| W-10 | the smallest-closure estimate double-counts a shared given (latent) | none exists |
| W-11 | **F1**: keyword-prefix split in rule patterns (new) | [`findings/F1_keyword_prefix_split.ein`](inventory_2026-11-05/findings/F1_keyword_prefix_split.ein) |
| W-12 | `_` in a guard: `(eq ?x _)` never holds, `(neq ?x _)` always does, `(eq _ _)` holds | [`probes/wild_in_guard.ein`](inventory_2026-11-05/probes/wild_in_guard.ein) |
| W-13 | `:match _`, `(and (hit ?y) _)` and `:match foo` fire vacuously | [`probes/bare_premise.ein`](inventory_2026-11-05/probes/bare_premise.ein) |
| W-14 | a nested pattern with an unbound variable head never matches, in silence; at top level the same shape is a `CompileError` | [`probes/nested_unbound_head.ein`](inventory_2026-11-05/probes/nested_unbound_head.ein) |
| S-3 | `refuted by narrow` beside a core that only `wide` derives | `examples/ein-bugs/alt-cap-core.ein` and `-reordered` |
| S-4 | `ein test --json-report` rows carry no `outside` and no `types` | `tests/region/07_cube.ein` |
| S-5 | a reloaded `--dump-answer` state reads every fact as given | [`probes/n5_string_space.ein`](inventory_2026-11-05/probes/n5_string_space.ein) |
| S-6 | the detector's `RootPlans` is keyed by activator symbols (not probed) | none |
| S-7 | the detector's chain attribution reads every application as the activator (not probed) | none |
| S-8 | the static check's D does not follow activators, so its site list under-reports | `forbid-chain-node-writes.ein`, which *forbid* refuses: no CLI shows it |
| S-9 | `absent_premises` follows neither the activator nor the negative side | library only |
| S-10 | the `types` warning reads `is-a` by name only | 0 programs today |
| S-11 | `features/01`'s activator site is an arity-blind name collision | `examples/features/01_not_and_absent.ein` |
| S-14 | `corpus_shapes.md5`'s `hyp` rows digest a generator no route runs | test surface |

W-10, S-6, S-7 and S-10 have no witness, so they are latent. S-8 and S-9 have
witnesses, but *forbid* refuses them, so no CLI run shows the defect.

**For the close.**
- **Rows the close will not name.** Six live W items have an owner but no
  register row: W-2, W-5 and W-6 (F21 D2's), W-9 (F22, by default), W-15
  (M1f's S1f.10.6), and W-4, W21, whose row S1e2.2.21 is to file. The seven
  with no owner (W-7, W-8, W-10 … W-14) have no row either.
- **The acceptance's re-take cannot run as written.**
  [`probes/capture.sh`](../../plans/m1e2_correctness/probes/capture.sh)'s W
  lines spell retired options, and none exist for W11 or W13–W21. The four § 4
  sweep scripts read the walk's summary.
- **What this document did instead.** The register test above, and the sweeps
  in § C.

---

## A. Known incorrect results

**Sources swept.** Every non-fixed row of
[`corpus/wrong_answers.toml`](../../corpus/wrong_answers.toml). The three
stages filed and not built: S1e2.2.21 (W21), S1e2.3.8 (N5) and S1e2.3.9 (N6).
Every `§ Notes` section, parked paragraph, candidate list and dropped stage in
the 90 Markdown files of `plans/m1e2_correctness/**`, plus the headers of
`probes/`. Every question in
[`open_questions.md`](../../plans/m1e2_correctness/open_questions.md). The
earlier milestones' open questions (`docs/history/*/open_questions.md`,
`plans/open_questions.md`). Every followup. And
[`inventory.md`](../../plans/m1e2_correctness/inventory.md) of 2026-10-14.

The documents were read by seven extraction passes. Their item-by-item records,
374 items with source line, quote and later resolution, are kept beside this
file under `extract/` (not kept; re-taken by `inventory_2026-11-05/`'s scripts). This section keeps the live ones, merged
where two sources name one defect. A row's **re-taken** cell names the run, on
the snapshot binary, on 2026-11-05.

### A.1 W: wrong answers

| id | what is false | owner | source | witness | re-taken | reaches |
|---|---|---|---|---|---|---|
| **W-1** | `W8/features-07`. The fork node `{(q one)}` holds `(p one)`, derived at root under `(absent (q one))`, beside `(q one)`. The file's header says `{p, q}` *"is not a model of this program under any reading"*. Contested, three readings: *guards are premises* gives `{(q one)}` (the row's `right`; per `node_census.md` § 9 it is reading (B)'s answer, B1-3); *refuse* gives a load error; *append-only* says it is not wrong. The read-out's region lists `(model (q one))`, and only `-p` shows the stale `(p one)` | **F21 D2** (register `owner = "F21"`) | [register](../../corpus/wrong_answers.toml) `W8/features-07`; [Q-M1e2.2](../../plans/m1e2_correctness/open_questions.md#q-m1e22--does-a-guards-absent-premise-bind-the-state-the-search-ends-in); [F21 D2](../../plans/followups/f21_non_monotone_programs_decided.md#d2--q-m1e22-what-a-nodes-state-is) | `examples/features/07_unstratifiable.ein`, twin `ein-bugs/wrong-answers/f07-fpoff.ein` | **live**. `wrong_answers.rs` holds the row to its recorded `now` and passes. `ein solve -p` on state 2 shows `(p one) (q one)`. The detector reports *conservative*, *stale facts — 1 of the cube's 2 nodes* | NM |
| **W-2** | `features/07`'s R depends on which of two priority-200 rules is written first. As written R = {∅, {(q one)}}; swapped, R = {∅, {(p one)}}. The answer is not a function of the program (the config principle's *stable*) | F21 D2 (no row) | [`node_census.md` § 7.1](../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/node_census.md) (B1-2); register W8's `review.note` | [`probes/f07_rules_swapped.ein`](inventory_2026-11-05/probes/f07_rules_swapped.ein) vs [`f07_as_written.ein`](inventory_2026-11-05/probes/f07_as_written.ein) | **live**: `(model (q one))` vs `(model (p one))` | NM. These are the only 2 of the 167 cubes `node_census -- --invariance` reordered whose answer moves with rule order, and neither is monotone |
| **W-3** | `W10/alt-cap-shared`. The core is `(a K) (b K) (c C01)`. The smallest over every recorded derivation is `(a K) (b K)`, through a saving that sits behind a full alternatives list | **F22** | register `W10/alt-cap-shared`; [F22](../../plans/followups/f22_exact_core_past_the_cap.md#the-case); [Q-M1e2.11](../../plans/m1e2_correctness/open_questions.md#q-m1e211--which-derivations-is-the-unsat-core-the-smallest-over-and-may-a-config-flag-change-it) (i)(b) | `examples/ein-bugs/wrong-answers/alt-cap-shared.ein` | **live**: `ein solve` prints `unsat core (a K) (b K) (c C01)`, `refuted by clash` | MC (monotone, root dead). S1e2.2.11 found 10 corpus programs that reach the cap, and no other core moved |
| **W-4** | **W21**. A pair contradiction's core is its positive side's frontier alone. Where that side is root's, the core names no fact of the commitment, which is no refutation while root is alive | **S1e2.2.21** (specified, not started; no register row yet) | [S1e2.2.21](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.21_pair_core_one_side.md#the-case); [S1e2.2.19 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.19_core_misses_activator.md#notes) | `examples/zebra2.ein`: N member `{(drink-loc Tea House-4)}`, core `(drink-loc Milk House-3)`, `refuted_by total` | **live**, matching the stage's numbers. Of N's cores, the number that lie wholly inside root's fixpoint while root is alive: `zebra2` and `zebra2-obligations` 201 of 366 each; `zebra.ein` 86 of 458; `zebra2-hints` 73 of 73; the `zebra2-minus-15` pair 693 of 1 495 each (partial runs); `branching/12` 10 of 25; `bijection/06` 7 of 16; `branching/02` 2 of 102. Taken by `sweep_cores.py` (§ C.1). In every case the core also names no fact of its set | MC 7, MS 2, HS 6, TO 1; 0 on NM (§ C.3) |
| **W-5** | The FIFO gap. `lattice/10_fifo_gap`'s node `{a, b}`, built at once, holds `{a, b, p, q}` with `p` stale. The stratified program's perfect model there is `{a, b, q}`. The cause is that the boundary admits equal-priority candidates in FIFO order, not stratum order | F21 D2 (*stratum order*). **No row** | [F21 D2](../../plans/followups/f21_non_monotone_programs_decided.md#d2--q-m1e22-what-a-nodes-state-is); [`monotonicity.md` § 7.9 c10](../../docs/kernel/inference/monotonicity.md#79-the-causes-as-one-table) (Q-40, B1-7) | `examples/lattice/10_fifo_gap.ein`; the written `detector_proposal/stratified_stale.ein` | **live**: `ein solve` prints *conservative*, stale facts in 1 of the cube's 4 nodes, k_s 4 | NM, HS (`outside p, q`) |
| **W-6** | Members of R past the oracle's limit hold a fact whose `absent` guard their own fixpoint falsifies. Under every reading but the fork, they leave R | F21 D2, D7. **No row** | [`alive_set_reach.md` § 6](../../plans/m1e2_correctness/p1e2.1c_the_region_search/alive_set_reach.md) (C1X-2, B1-6); [`monotonicity.md` § 5.3](../../docs/kernel/inference/monotonicity.md#53-the-detector) | `features/01` 24 754 of 64 655 nodes; `features/04_open` 21 042 of 64 761; `square-unique/corner-house` 5 595 of 65 389, `cul-de-sac` 5 835 of 64 923, `terminus` 4 876 of 65 029 | **live**: the counts above, from `ein solve FILE --json-summary` (`monotonicity.state`) | NM (stopped at `--region-limit`), TO 4 |
| **W-7** | **F2 (new)**: a core-search cut is printed as the core. See § 0 | **none** | none ([S1e2.2.11 § Two findings](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md#two-findings-and-why-the-stage-stops-here) claims the cut is reported; D1X-41) | [`findings/F2_core_cut.ein`](inventory_2026-11-05/findings/F2_core_cut.ein), control and twin | **live**: 10-fact core where 8 suffice | MC (probe). Any route that prints a core |
| **W-8** | A premise-less derivation from an activator a node derived is not recorded as an alternative (`Saturator::record_alternative`), so the core is not the smallest over every derivation. Rule priority decides it | **none** (F22 takes the writeback rule as given) | [S1e2.2.19 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.19_core_misses_activator.md#notes), 3rd item (D2X-30, Q-26) | [`probes/guard_only_second.ein`](inventory_2026-11-05/probes/guard_only_second.ein) / [`_first`](inventory_2026-11-05/probes/guard_only_first.ein), the stage's shape re-written here | **live**: N `{(p a)}` has core `(b a) (p a) (seed a)` by `early`, and `(p a)` by `w` with the priorities swapped | NM only: the shape needs an activator a node derives, which is an activator site. 0 corpus programs |
| **W-9** | `fork-delta-order`'s core `(g G08) (h H40) (on A) (on B)` has the smallest size and is stable, but it is not the canonical core an uncapped store gives, `(g G01) (h H01) (on A) (on B)` | F22 by S1e2.2.12's default 2 (no row of its own) | [S1e2.2.12](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md) § Where the order enters (D1X-44) | `examples/ein-bugs/fork-delta-order.ein` | **live**: `nogoods.json` gives `core [(g G08), (h H40), (on A), (on B)]`. Contested: a correct smallest core, but not the canonical one | MC, HS |
| **W-10** | The smallest-closure estimate that decides which arrival a full list keeps is a sum. It counts a shared given twice and freezes a premise's closure when it arrives | **none** (F22 does not name it) | [S1e2.2.11](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md) § The closure size (D1X-40) | none | latent | programs past the cap |
| **W-11** | **F1 (new)**: keyword-prefix split. See § 0 | **none** | none | [`findings/F1_keyword_prefix_split.ein`](inventory_2026-11-05/findings/F1_keyword_prefix_split.ein), [`F1b_or_prefix.ein`](inventory_2026-11-05/findings/F1b_or_prefix.ein) | **live**. `(neqw ?x)` stores `(neq w a)`, `(orz ?x)` stores `(or z a)`, and `(andz ?x)` in an `:assert` stops the run. `(neqw ?x)` as a query goal prints `(neq w ?y)` | every program that loads; 0 in the corpus (grep for `(neq|or|and)` followed by a word character) |
| **W-12** | `_` in a guard compares as a raw IR node, in silence. Unruled: the ruling on `_` in a pattern, Q-M1e2.13 (a), does not reach a guard | **none** | [S1e2.2.20 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md#notes) (D2X-40, Q-27) | [`probes/wild_in_guard.ein`](inventory_2026-11-05/probes/wild_in_guard.ein) | **live**: `ein saturate --dump` derives `(hit-neq a)` and `(hit-both a)`, and no `hit-eq` | every program that loads; no corpus rule |
| **W-13** | A premise that is not a form is dropped (`compile.rs:544`, `_ => return Ok(())`) | **none** | S1e2.2.20 § Notes (D2X-41) | [`probes/bare_premise.ein`](inventory_2026-11-05/probes/bare_premise.ein) | **live**: `(r1 z)`, `(r2 w)`, `(r3 z)` all derived | as W-12 |
| **W-14** | `Slot::Opaque` is left for a keyword pair in a nested pattern, and for a nested head variable the activator does not bind. It never matches, in silence. A top-level `(?r a)` is a `CompileError` | **none** | [S1e2.2.20](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.20_wildcard_and_range_in_a_rule.md) § What was built (D2X-38); `plan.rs:98–104` | [`probes/nested_unbound_head.ein`](inventory_2026-11-05/probes/nested_unbound_head.ein) | **live**: exit 0 and `r` never fires, where `unbound-relation-head.ein`'s top-level shape exits 1 | as W-12 |
| **W-15** | When a rule derives an obligation's activator inside a node, the facts that would discharge the new debt may never be candidates. Suspected, with no witness | S1f.10.6 (M1f) | [Q-M1e.11](../../docs/history/m1e_review_processing/open_questions.md#q-m1e11--what-happens-to-an-obligation-derived-under-a-hypothesis) (Q-31); inventory § 6 | none | latent. Q-M1e2.10's decline covers a guard over D, not this | NM (an activator site) |

Three more W shapes are **moot today**, because *forbid* refuses every program
that has them:
- the probe `not_conclusion` (B1-5);
- `stale-naf-fork` (W8's second row, fixed `597b2b7e`);
- `ACTIVATOR_UNDER_ABSENT` and `ACTIVATOR_REFUTES` (B1-33, B1-34).

### A.2 S: surfaces that misreport a right answer

| id | what is misreported | owner | source | witness | re-taken | reaches |
|---|---|---|---|---|---|---|
| **S-1** | **N5.** The fact printer drops what tells two values apart. A string with a space prints as two arguments, `"1"` prints like the int `1`, and `(r _ ?x)` prints as `(r ?x)`. A `--dump-answer` state file does not reload to its own facts | **S1e2.3.8** (not started) | [P1e2.3 § Candidates, N5](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/README.md#candidates-found-on-the-way); [S1e2.3.8](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md) | `ein saturate --dump examples/syntax/arg-kinds.ein`; `examples/ein-bugs/activator-text-int-and-string.ein` (W16's pair); [`probes/n5_string_space.ein`](inventory_2026-11-05/probes/n5_string_space.ein), [`n5_int_vs_string.ein`](inventory_2026-11-05/probes/n5_int_vs_string.ein) | **live** on all four counts below | every printing surface. `answer_dump.rs` exempts W16's pair (`printed_alike`) |

What the S-1 re-take showed:
- `(r 7 "a string")` prints as `(r 7 a string)`, and `(r _ ?x)` as `(r ?x)`.
- `(holds 1)` and `(holds "1")` print alike: `ein saturate --dump` on W16's `activator-text-int-and-string.ein` lists `(holds 1)` twice and `(noted 1 1)` twice.
- State 38's file writes `(holds x y :rule "r")`, which reloads as a 2-ary fact.
- `(p "x y")` reloads as `(p x y)`.

| id | what is misreported | owner | source | witness | re-taken | reaches |
|---|---|---|---|---|---|---|
| **S-2** | **N6.** A run stopped past `--region-limit` writes each member of N it met into `nogoods.json` with `core []` and `refuted_by []`. That reads as *refuted by nothing*, but the run never took the core | **S1e2.3.9** (not started) | [S1e2.3.9](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md) | `examples/lattice/06_friends_tables.ein --dump-answer DIR` | **live**: 68 of 68 empty. No empty core on any of the 100 other runs with N > 0 or a dead root (`sweep_cores.py`), so the W half of S1e2.3.9's step 1 has no witness | NM, stopped |
| **S-3** | `refuted by` / `refuted_by` names the first contradiction's primary rule, which need not be a rule the core's derivation uses | **none** (*"parked for P1e2.3's next survey"*; no P1e2.3 candidate carries it) | [S1e2.2.11 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.11_alt_cap_core.md#notes), [S1e2.2.12 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.12_core_id_order.md#notes) (D1X-42) | `examples/ein-bugs/alt-cap-core.ein`, `alt-cap-core-reordered.ein` | **live**: both print core `(w1 X) (w2 X)`; one says `refuted by narrow`, the other `wide` | every dead node with two derivations of `(false)` |
| **S-4** | `ein test --json-report` rows carry `region.monotone` and `status`, but no `outside` and no `types`. The text surface prints both beside the claim | **none** (S1e2.2.8's default) | [S1e2.2.8](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.8_uncovered_certified.md) § The sentence and the field (D1X-28) | `ein test tests/region/07_cube.ein --json-report R -v` | **live**. Stdout prints `outside the universe: is-a …`; the row's `region` is `{route, monotone, k_s, k_c, ms, status, stale_nodes, paths_differ}` | HS, TO programs with a claim |
| **S-5** | Provenance does not survive a reload of a dumped state. The writer spells `:rule "name"` and `:hypothesis N`; the loader reads neither, so every reloaded fact reads as given. `answer_dump.md` says the file carries them | **none** | [S1e2.1.7](../../plans/m1e2_correctness/p1e2.1_seeing_the_answer/s1e2.1.7_the_review.md) § Things this phase found (A1-5) | [`probes/n5_string_space.ein`](inventory_2026-11-05/probes/n5_string_space.ein) state 38 | **live**: `ein saturate --dump` on the reloaded file lists `(p A)` with no `:hypothesis` | every `--dump-answer` state |
| **S-6** | The detector's `RootPlans` keys root's plans by the activator's symbols. An all-int obligation activator, derived at a node, would read as root's, and the hole would go unattributed | **none** | [S1e2.2.16 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.16_activator_text_key.md#notes) (D2X-15) | none (*"parked, not probed"*) | not re-taken: no witness | NM (none in the corpus) |
| **S-7** | The detector's chain attribution reads every application of a rule as its activator, and seeds an owing node from symbol arguments only | **none** | [S1e2.2.17 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.17_forbid_activator_chain.md#notes) (D2X-19) | none | not re-taken. S1e2.2.19's `Prov::activator` could feed it, and nothing says it does | NM |
| **S-8** | The static check's D does not follow activators, so the site list calls a read static that a plan compiled at a node makes dynamic. The verdict holds anyway | **none** (S1e2.2.17's default 2) | S1e2.2.17 § Notes (D2X-20) | `ein-bugs/wrong-answers/forbid-chain-node-writes.ein` | **not re-takable**: *forbid* refuses it, `--monotone` included | NM / FB |
| **S-9** | `absent_premises` (dumps, `region.md` *rests on*, the detector's input) is read off the first contradiction's positive side, and does not follow the activator the core follows | **none** | S1e2.2.19 § Notes, 4th item (D2X-31); [`taint_census.md` § 3](../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/taint_census.md) (B1-36) | W17's witnesses studied past the refusal | **not re-takable** from the CLI (library and probe build only) | FB (library) |
| **S-10** | The `types` warning reads `is-a` by name only, so a type relation with another name (`instance`) is never read | **none** | [S1e2.2.10 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.10_types_as_objects.md#notes) (D1X-30) | none (the ten `instance` programs all have H = ∅) | latent | TO |
| **S-11** | `features/01`'s activator site is a relation-grain name collision. The rule writes `unknown-feeling` at arity 2, while its activator has arity 1. The site cannot bite, but it keeps the program flagged | **none** (the grain ruling) | [`site_census.md` § 2](../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/site_census.md) (B1-41) | `examples/features/01_not_and_absent.ein` | **live**: sites `unknown-feeling:activator`, `unknown-feeling:absent` | NM, TO |
| **S-12** | *"⊤, the trivial line: no restriction bites"* is printed on a flagged program. The trivial line is a monotone-only theorem | F21 D4 (d3) | [`hole_census.md`](../../plans/m1e2_correctness/p1e2.1b_non_monotone_programs/hole_census.md) (B1-30) | `examples/lattice/04_obligation_trigger_hole.ein` | **live** | NM |
| **S-13** | A dead hole's row prints its core only, not the `absent` its refutation rests on | F21 D4 (a) | B1-24 | none loads since *forbid* | **not re-takable** | FB |
| **S-14** | `ein-render/tests/golden/corpus_shapes.md5`'s `hyp` rows digest `hypgen::generate`: the walk's ladder with the lookahead, the kill cache and `emit_closed`, which no route and no command runs | **none** (*"not this stage's"*) | [S1e2.3.1](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.1_the_summary_root_block.md) § What applying the ruling measured (E1X-22) | `shape.rs:259` | not a user surface | test golden only |

### A.3 D: diagnostics and spelling

| id | defect | owner | source | re-taken |
|---|---|---|---|---|
| **D-1** | Python spellings, and they differ between commands (details below the table) | **none**. Parked for the user in [S1e2.2.18 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.18_strings_in_rules.md#notes). [`defined_behaviour.md` § 4](../../docs/kernel/defined_behaviour.md#4-errors-and-exit-codes) lists the three | D2X-25, D2X-39, D1X-33, Q-28 | **live** |
| **D-2** | Loader messages end `at None`, or `at Loc(file=…, line=…, col=…)`. Defined behaviour (§ 1.5), but a Python spelling | **none** (*"whoever needs it"*) | [Q-M1a.6](../../docs/history/m1a_rust/open_questions.md) (Q-36, D1X-10) | **live**: e.g. `kb load error: duplicate relation … at None` (`examples/broken/load/`, in the sweep) |
| **D-3** | `ein test` on a refuted `(solve … :expect (region …))` exits 1 with an empty stderr. `ein solve` prints a line | **none** | [S1e2.2.15 § Notes](../../plans/m1e2_correctness/p1e2.2_wrong_answers/s1e2.2.15_obligations_rung_universe.md#notes) (D2X-10) | **live**: [`probes/refuted_region_claim.ein`](inventory_2026-11-05/probes/refuted_region_claim.ein): `test` exit 1 with 0 bytes on stderr; `solve` exit 1 with `(solve :expect) FAILED — the claim lists 1 state and R has 2` |
| **D-4** | The `[activator …]` label prints symbols only, so an all-int refusal names no activator | **none** | S1e2.2.16 § Notes (D2X-16) | **live**: `activator-all-int.ein` prints `rule 'alarm': concludes …`, and `-all-symbol` prints `rule 'alarm' [activator a]: …` |
| **D-5** | The plan shape's `GUARD` line prints `String(value='b')` for the symbol `b` | **none** | S1e2.2.18 § Notes (D2X-26) | library and test golden (`corpus_shapes.md5`) only; no CLI prints it |
| **D-6** | `ein saturate --max-steps 3` prints `running with max_steps=3` and `last='includes'` | **none** | [S1e2.3.7 § Notes](../../plans/m1e2_correctness/p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.7_four_lines_that_misname.md#notes) (E1X-24) | **live**: `saturate: running with max_steps=3, progress every 500 steps` |
| **D-7** | `ein.inference.saturator.SaturatorStepLimitError` and a `{:?}` firing at `common.rs:254` and `saturator.rs:632` | **none** | S1e2.3.7 § Notes (E1X-23) | unreachable from the binary |
| **D-8** | An hrule's template that builds no fact proposes nothing, and nothing says so. `hrule.rs` reads `Err(_) => continue`. Only a range is refused at load | **none** | S1e2.2.18 § Notes (D2X-24); `defined_behaviour.md` § 4 | **live**: [`probes/hrule_no_fact.ein`](inventory_2026-11-05/probes/hrule_no_fact.ein), with `:assert foo`, loads with no diagnostic |
| **D-9** | An unused rule parameter is accepted silently | **none** | S1e2.2.12 § Notes (D1X-46) | not run: a reading of the loader |
| **D-10** | `ein render rules` exits 1 on a file with no rule (*"no rule forms in …"*). Contested against the user's *exit non-zero only on error* | **none** | E1X-40, C1X-71 | **live**: `kwpair-below-the-filter.ein` exits 1 |
| **D-11** | Pages and records that are now false (list below the table) | **none** | as listed | by reading, except the parse of `()` |

What D-1's re-take printed:
- `ein solve nope.ein` prints `FileNotFoundError: [Errno 2] …`, where `ein saturate` prints `error: nope.ein not found`.
- `ein saturate examples/ein-bugs/unbound-assert-var.ein` prints `KeyError: "unbound var ?v1 …"`, and `solve` prints the message alone.
- `solve` and `saturate` prefix `ein.inference.compile.CompileError:`, and `oracle` does not.
- The `CompileError` lines carry the Python IR's repr too: `` `SForm(head=Var(name='R'), args=(…))` ``.

What D-11 covers:
- `configuration.md:407–411` says a deadline stop reports a partial answer, then that a `-T` abort prints none. `read_out.md` § 8.1 lists `stopped.why: deadline`. Re-taken: `-T 0` exits 2 and prints no read-out (E1X-36).
- `solution_semantics.md` § 3 says *"No surface computes projection classes yet"*, while § 5.1 and every read-out print them. Found here.
- `monotonicity.md` § 7.9 quotes *"the flagged sites break nothing here"* in the present tense (B1-44).
- `site_census.md` says the conservative eight *"never make it bite"* (B1-45).
- `route1_reach.md` gives the wrong reason for `query-goal-free-head` (B1-54).
- `region.rs`'s module doc speaks of `alive₀`, the cascade and the lookahead (C1X-46).
- `implementation.md` names `--dump-region` (C1X-47).
- Five obligation fixtures' headers say `:expect` cannot state what a state owes (E1X-25).
- `lever_invariance.rs`'s module doc (E1X-26).
- `release.yml` says *"two slow entries"* and *"everything nightly"* (E1X-27).
- `plans/README.md` says *"31 probe programs"*; there are 11 (E1X-51).
- Four probes no longer load: `cfgtest`, `f07-warn`, `fresh-name-declared-default`, `ucid-perm1` (E1X-50).
- The S9 row's *held by* names checks that are gone (E1X-12).
- F4's "Q40" section still calls a fixed bug live (Q-48).
- M1e's Q-M1e.8 index row still says open (Q-30).
- `hrule-reads-not.md` uses the retired `--max-set-size`.
- Global Q28 says `()` *"parses as a no-op atom"*. At top level it is a parse error (`probes/empty_parens.ein:3:2: unexpected input`, exit 1). Found here.
- W8's register `right` is labelled *guards are premises*, but it is reading (B)'s answer (B1-3).

### A.4 P: cost only

These change no answer:
- `candidate_objects` walks every fact at every depth on each call; not measured (D2X-6).
- `--models key`'s node budget does not bound time: 66.6 s at k = 1 400 (E1X-31, C1X-70).
- A plan duplicated by a nested-fact activator fires twice (D1X-22).
- L5 licenses only 18 of `zebra.ein`'s 36 debts (C1X-14).
- MARCO-style enumeration is not built, and `--minimal` is not output-sized (C1X-13, A1-43).
- The state route stops between whole layers (C1X-8).
- `arg-kinds`' read-out costs more than its route (C1X-10).
- Recording alternatives, and the smallest-closure bookkeeping, cost +5.8 % on `zebra2` (D1X-39).
- `--jobs` scales sublinearly on the cube (A1-44).
- The lookahead pre-filter and fail-fast were given up with their flags (C1X-48).
- `hypgen::generate_one_branch` has no caller (E1X-21).
- `utils/fork_split.py` leaks a directory, and its cells spell `-e` (E1X-42).
- F11's D1 and D2 are parked on their triggers.

---

## B. Known solution gaps

Each gap is a place where Ein gives no answer, a refusal, a scoped answer or a
lower bound, by design or by a ruling. The programs are the snapshot corpus's,
from § C's sweep.

| id | gap | ruling / page | corpus programs today |
|---|---|---|---|
| **G-1** | ***forbid***. A command that takes the universe refuses a program whose `(false)` or `(not …)` rests on an `absent` over a relation a commitment can still extend: directly, *via* a fact derived under one, through an activator, or up an activator chain (D⁺). Re-taken on `naf-upward-closure.ein`: `ein solve`, `ein solve --monotone` and `ein oracle` exit 1 with the rule's line, `ein saturate` exits 0, and the rule views do not refuse either (C1X-54). It does not cover an `absent` that concludes a positive fact (W-1, W-5, W-6), obligation triggers, or the FIFO gap ([`monotonicity.md` § 7.7](../../docs/kernel/inference/monotonicity.md#77-what-forbid-covers-and-option-e)) | Q-M1e2.5 *forbid* (2026-11-02); S1e2.2.5 `597b2b7e` with F2 *via*; S1e2.2.16 `3257488f`; S1e2.2.17 `f5a0dca9`; [`defined_behaviour.md` § 4.7](../../docs/kernel/defined_behaviour.md#47-a-refutation-resting-on-an-absent-a-commitment-can-fill--m1e2-s1e225); option (E) is F21 D1, open | **19** (list below the table) |

The 19 programs *forbid* refuses, each by the rule it names:
- `branching/13` by `self-block`, and `branching/14` by `premature-signoff`.
- `ein-bugs/activator-all-{int,nested,symbol}` by `alarm`.
- `alive-empty-interlayer` by `totality`.
- `d3-unsat-core` by `fire-0` *via* `fire-1`.
- `naf-upward-closure` by `bad`.
- `forbid-chain-{0,1,2,node-absent,node-writes,param-absent}` by `q`, `r2`, `r3`, `r2`, `chk` and `bad`.
- `naf-nowb` by `bad`.
- `stale-naf-fork` and its `-warn` twin by `p-and-q-clash`.
- `lattice/synth/three_components` by `dead-3`, and `trigger_hole` by `dead-1`.

All 19 exit 1 with the one line of § 4.7.

| id | gap | ruling / page | corpus programs today |
|---|---|---|---|
| **G-2** | **Refusals at load.** Since S1e2.2.20, a range in a rule, or a `_` written as a relation. Also: a query goal with a variable head, a variable rule head no parameter binds (`CompileError`), an `:assert` variable an `(or …)` arm leaves unbound (the run stops at firing), and the by-design negative fixtures | Q-M1e2.13 (a), Q-M1e2.14 (c), `eddea251`; [`defined_behaviour.md` § 4, § 4.8](../../docs/kernel/defined_behaviour.md#48-a-wildcard-as-a-relation-in-a-rule--m1e2-s1e2220) | **68**: 51 `load-negative`, 7 `compile-negative`, 4 `parse-negative`, `range-in-match`, `wild-in-head`, and 4 others: `query-goal-free-head`, `features/02_star_in_identifiers` (a `positive` entry: `solve` refuses, while `ein oracle --dump` writes its cube, k_c 32 768 and k_s 728, before exiting 1, and `render lattice` draws R), `unbound-relation-head`, `unbound-assert-var` |
| **G-3** | **R is scoped over H.** R holds of a relation the rung never proposes only what saturation derives. It is named by an `outside` row and `root.hypgen.outside` | Q-M1e2.3 (a), S1e2.2.8 `52004a68`; [`solution_semantics.md` § 1.4](../../docs/kernel/inference/solution_semantics.md#14-candidate-and-universe) | **24** (list below the table) |

The 24 programs with an `outside` row, by the reason the row gives:
- *no-hrule*: `branching/05`, `/10`, `/11` (`is-a`); `fork-delta-order` (`p`, `q`); `core-activator-guard-only` (`w`); `core-activator-root-derived` (`r2`); `forbid-chain-no-absent` (`q`); `zebra2-bad` and `ein-bugs/zebra2-hints` (`is-a*`, `next-to`); `lattice/01` (`incompat`); `lattice/04` (`is-a`); `lattice/10` and `lattice/11` (`p`, `q`); `lattice/12` (`serves`); `zebra.ein` (`instance`, `next-to`); `zebra2`, `zebra2-hints` and `zebra2-minus-15` (`is-a*`, `next-to`); `tests/region/07` (`is-a`).
- *arity*: `arity3` (`r3`).
- *not-owed*: `uncovered` and `uncovered-knows` (`knows`); `zebra2-obligations` and `zebra2-minus-15-obligations` (`is-a*`, `next-to`).

| id | gap | ruling / page | corpus programs today |
|---|---|---|---|
| **G-4** | **Types bound as objects.** The blind enumerator is untyped. A `types` row warns, and R is right for the program as written | Q-M1e.12 (a), S1e2.2.10 | **10**: `branching/02`, `/08`, `transitive/taxonomy`, `type-exclusivity/{colors,nationalities,pets}`, `features/01`, `square-unique/{corner-house,cul-de-sac,terminus}`. Eleven untyped fixtures are left to *"each fixture's owner"* (D1X-31) |
| **G-5** | **Below relation grain, unnamed.** The blind enumerator's self-edges `(R a a)` and an obligation's untyped facts are never candidates, and no row names them, because `outside` works per relation. The declarative universe is not written. A latent W under ruling (h) | `solution_semantics.md` § 1.4 (*"not written yet"*); S1e2.2.8 § Notes (D1X-27, A1-11) | not measured |
| **G-6** | **Names.** A constant only a query names is no candidate. Ein makes no name at run time. A variable or a range written in a fact is an object of H, unruled | Q-M1e2.9 (a); S1e2.2.14 § Notes (D2X-5) | `syntax/arg-kinds`: H holds `?x`, `1..5` and `1..*` |
| **G-7** | **Non-monotone programs: detected, warned, undecided.** A flagged program is answered over nodes built by fork from root, by the cube (route 2), with the detector's block. What a node is (D2), (E) (D1), parent extension (D3), the hole report (D4), the handling (D5) and the search past the limit (D7) are open. The static check reads at relation grain and is sufficient, not necessary, so 8 of the 15 complete flagged programs are *conservative* | P1e2.1b re-planned 2026-11-02; [F21](../../plans/followups/f21_non_monotone_programs_decided.md); [`monotonicity.md` § 8](../../docs/kernel/inference/monotonicity.md#8-what-is-open) | **21** flag and load. Exercised: 7. Conservative: 8. *Not known*, past the limit: 6. List in § C.2 |
| **G-8** | **Stopped runs.** `--region-budget` (2²⁰ facts) and `--region-limit` (16) give a lower bound: `exhausted no` and why, a `note:`, exit 0. Components, sub-cubes and merged are *not known*, and a claim is *NOT CHECKED*. `-T` aborts with exit 2 and no answer. Q-M1d.1's stopping half is open | budget ruled 2026-11-03; [`configuration.md`](../../docs/kernel/configuration.md); `solve_interface.md`; Q-M1d.1 (Q-35) | **15**. 9 monotone at the budget: six `square-{bwd,fwd}/*`, `syntax/arg-kinds`, `zebra2-minus-15`, `-obligations`. 6 flagged at the limit: `features/01` (n 35), `features/04` (n 81), `lattice/06` (n 19), `square-unique/*` (n 118, 114, 153). `arg-kinds` has no row and is a lower bound both ways (Q-21) |
| **G-9** | **The oracle's 2ⁿ limit.** n ≤ 16 runs silently. Past it the oracle refuses, exit 2 (*"over --region-limit 16 … pass --region-limit 56 to compute it"*, re-taken on `zebra.ein`), unless `--region-limit N` raises the limit, with a warning from n = 17. Past the limit there is no independent reference. The oracle shares the saturator and the generator, so it cannot see a lost derivation or a missing candidate (§ 6.6) | the ruling of 2026-10-15 (Q-19); [`solution_semantics.md` § 6.6](../../docs/kernel/inference/solution_semantics.md#66-what-the-oracle-does-not-check) | **35** refused by the limit (`region_census.py`, 2026-11-05) |
| **G-10** | **`solution_semantics.md` § 6.** Every difference that § 6.1–6.5 lists is the walk's, deleted at S1e2.1c.5. What still holds: § 6.6 (G-9); H is defined by the generator, and its declarative definition is not written (G-5); a node is the fork's (G-7) | the page | — |
| **G-11** | **The modes' scope** (*minimum work per question*). `--count` reads no state, checks no claim, and refuses every form and `-H`. `-n N` says nothing of the rest. `--maximal` and `--minimal` give one line. The four words belong to `--single` alone. The forms share stdout's 100 lines, and a key over the shown states may match an unshown state. Not built: the certain and possible fact sets, R per rule, symmetry, the distance between solutions, N and E as a certificate (A1-41), region shapes, a component subset, `:show` (A1-40), and owing dead-ends beside a non-empty R (A1-42) | `solve_interface.md`; [`read_out.md` § 10, § 12](../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md); S1e2.3.3 | every program |
| **G-12** | **Claims the language cannot state.** An empty goal extent (Q-M1e.13, Q-M1c.6). A relation only saturation creates (Q-M1c.7). How much a state owes. A claim on a partial run is *NOT CHECKED* | M1e/M1c questions (Q-32, Q-33), all open with no owner | — |
| **G-13** | **The core's promise.** A core is the smallest frontier over the derivations the program records, bounded by the cap of 32 alternatives (F22) and the search budget (F2, W-7). It is not a subset-minimal MUS. `:record-alternative-justifications false` may change a dead node's core, the config principle's one exemption | Q-M1e2.11 (i)(b), (ii)(b); F9 E19 (Q-49) | — |
| **G-14** | **Checked at firing only.** A bad `:assert` (`_`, a keyword pair, a non-fact, an unbound variable relation) stops the run at its first firing, so a rule that never fires is never checked | S1e2.2.18's default 4 (D2X-23) | — |
| **G-15** | **A keyword pair in a pattern is dropped.** So `(n ?x :where 1..3)` matches any `(n ?x)`, and a range under a keyword pair is not refused | Q32 (M1), S1e2.2.20 defaults 9 and 12 (D2X-37); re-taken on [`probes/kwpair_in_pattern.ein`](inventory_2026-11-05/probes/kwpair_in_pattern.ein): fires on `(n a)` | — |
| **G-16** | **Obligations.** A ground obligation, `(open q)` over `(absent (q ?x))`, is refused at load. An obligation with no hrule can leave H empty, so R = {∅}. When a guard reads D, the rung declines and the untyped blind enumerator builds H (12 and 16 states, not 3 and 1) | A1-47, C1X-59, C1X-60; Q-M1e2.10 (a) (D2X-8) | `oblig-fresh-name-two`, `oblig-stdlib-total-owed` (and `-declared`) |
| **G-17** | **Artefacts.** A failed artefact write exits 0. `ein test` refuses `--events` and `--json-summary` over several files. `--models key` and `--diff-models` decline past a 16 MiB pair table | Q-M1e.22 (Q-34; owner M2 S2.1.2); `configuration.md`; S1e2.4.2 | — |
| **G-18** | **Coverage of the checks.** 52 of 217 stdlib mutants survive, 42 of them in `slots.ein`. There is no renaming-invariance property test (D1X-4). No render sweep runs over `fuzz_findings/` or `probes/`. Several count-option refusals are held by no test. The nightly is green locally only | E1X-29, -34, -35, -30 | — |
| **G-19** | **The close's own re-take.** `probes/capture.sh`'s W lines spell `-e`, `-L`, `-K` and `EIN_TRAVERSAL`, and there are none for W11 or W13–W21. `scan.py`, `verify.py`, `cascade_scan.py`, `lever_scan.py` and `answer_resolve.py` read the walk's summary. The acceptance asks that all of them be re-run | [milestone acceptance](../../plans/m1e2_correctness/README.md#acceptance-for-the-milestone) (E1X-47, -48, -49) | — |

---

## C. The intersections, measured

### C.1 What was run (2026-11-05, snapshot `cdcc3994`, release binary, `nice -n 19`, one engine process at a time)

| command (from `build/m1e2_inventory/`) | over | wall | output |
|---|---|---:|---|
| `EIN_STDLIB=src/stdlib python3 sweep_describe.py` | a bare `ein solve FILE --json-summary J` (describe, default budget and limit) on all 326 `corpus.toml` entries. Every register witness is one of them | 47 s | `corpus_describe.json` (not kept; re-taken by `inventory_2026-11-05/`'s scripts) |
| `TMPDIR=scratch/tmp python3 src/utils/region_census.py --all -j 1 --bin target/release/ein --json region_census.json` | `ein oracle FILE --dump` on the same 326 (the snapshot's own instrument) | 20 s | `region_census.json` (not kept; re-taken by `inventory_2026-11-05/`'s scripts), `region_census.txt` (not kept; re-taken by `inventory_2026-11-05/`'s scripts) |
| `python3 sweep_cores.py` | `ein solve FILE --dump-answer DIR` plus `ein saturate --dump` on the 101 entries with N > 0 or a dead root | 28 s | `corpus_cores.json` (not kept; re-taken by `inventory_2026-11-05/`'s scripts) |
| `python3 classify.py` | the join, with no engine | — | [`corpus.csv`](inventory_2026-11-05/corpus.csv), `corpus_classes.json` (not kept; re-taken by `inventory_2026-11-05/`'s scripts) |
| `cargo test --release -p ein-cli --test wrong_answers`, and `-p ein-infer --test register_region` (snapshot tree, own target dir, `--test-threads=1`) | every register row | 0.5 s + 0.2 s run (8 s + 20 s compile) | `scratch/wrong_answers_test.log` (not kept; re-taken by `inventory_2026-11-05/`'s scripts) |
| `ein solve` / `saturate` / `test` / `render` on each probe | `probes/`, `findings/` | seconds | the A rows |

`route1_reach` was not run. Since S1e2.1c.2, describe takes the state route on
every monotone program, so `corpus_describe.json` is that measurement. The
review bundle's `--regions` gallery was not built: it renders pages, and its
numbers are `region_census`'s.

**Describe against the oracle.** On all 204 programs where both finish, describe
and the oracle agree on k_s, k_c, |N|, the hole count and the static verdict:
0 disagreements (`classify.py`'s `agree_describe_oracle`). The two programs the
oracle answers and describe does not are G-2's query-goal refusals.

### C.2 The classes, and who is in them (the inverse)

These five classes partition the corpus: 209 + 9 + 21 + 19 + 68 = 326. HS and
TO are attributes of programs that load.

| class | n | members | defects and gaps that reach it |
|---|---:|---|---|
| **monotone, complete** (MC) | 209 | every loaded program not listed in the next two rows | W-3, W-4 (7), W-7, W-9, W-11…14 (probes), S-1, S-3, S-5; G-3, G-4, G-5, G-11…G-17 |
| **monotone, stopped** (MS) | 9 | `saturation/square-bwd/{floors,houses,meetings}`, `square-fwd/{floors,houses,meetings}` (k_s ≥ 23 390, k_c ≥ 2²⁰), `syntax/arg-kinds` (k_s ≥ 112 793), `zebra2-minus-15` and `-obligations` (k_s ≥ 32, k_c ≥ 865, N ≥ 1 495) | W-4 (both zebra2-minus-15 files), S-1 (`arg-kinds`); G-8, G-6 (`arg-kinds`) |
| **non-monotone, flagged** (NM) | 21 | see the two lists below the table | W-1, W-2, W-5, W-6, W-8 (probe), W-15; S-2, S-4, S-6…S-9, S-11, S-12; G-7, G-8, G-9 |

The 15 flagged programs that complete within the limit:
- *Exercised*: `lattice/04`, `/08`, `/09`, `/12`, `synth/local_not_global`, `synth/maximal_not_top`, `tests/region/07_cube`.
- *Conservative*: `features/07`, `f07-fpoff`, `lattice/10_fifo_gap`, `core-activator-committed`, `core-activator-guard-only`, `forbid-chain-no-absent`, `oblig-stdlib-total-owed` and its `-declared` twin.

The 6 flagged programs stopped at the limit, all *not known*:
- `features/01`, `features/04_open`, `lattice/06_friends_tables`;
- `square-unique/{corner-house,cul-de-sac,terminus}`.

| class | n | members | defects and gaps that reach it |
|---|---:|---|---|
| **refused by *forbid*** (FB) | 19 | G-1's list | none of R. S-8, S-9 and S-13 are invisible here. `ein saturate` still runs (C1X-54), so S-1 does too |
| **refused at load** (LD) | 68 | G-2's list | D-1, D-2 (the messages); G-2 |
| **H-scoped** (HS) | 24 | G-3's list. 16 are MC, 6 NM (`core-activator-guard-only`, `forbid-chain-no-absent`, `lattice/04`, `/10`, `/12`, `tests/region/07`) and 2 MS (the `zebra2-minus-15` pair) | W-4 (6: `zebra`, `zebra2`, `zebra2-obligations`, `zebra2-hints`, both `-minus-15`), W-5 (`lattice/10`), W-9 (`fork-delta-order`), S-4 (`tests/region/07`); G-3 |
| **types as objects** (TO) | 10 | G-4's list. 6 are MC, 4 NM (stopped) | W-4 (`branching/02`), W-6 (`features/01`, `square-unique` ×3), S-11; G-4 |

What the detector said on each class, from `monotonicity` in each summary:
- **Every MC and MS program**: *monotone (the static check)*.
- **NM**: the status bit listed above. Stale facts on 8 programs: `features/07` and `f07-fpoff` 1 of 2; `lattice/10` 1 of 4; and the five past the limit of W-6. A parent path differs on `lattice/08` and `lattice/10`.
- **Holes** on 6 programs: `lattice/04`, `/08`, `/09`, `/12`, `tests/region/07` (1 each), and `synth/local_not_global` (2). `synth/maximal_not_top` has none, but discharged is not upward-closed there.
- **The census of the same day**: 0 violations on 191 monotone cubes, 6 flagged cubes with a hole, 7 with discharged not upward-closed, 0 with dead not upward-closed. No dead hole loads, which is *forbid*'s effect.

### C.3 Item × class, cell by cell

The summary's matrix in full. A "–" names the mechanism that excludes the
class.

| item | MC | MS | NM | FB | LD | HS | TO |
|---|---|---|---|---|---|---|---|
| **W-1** | – : a stale positive needs an `absent` site over D, and that site flags the program | – : same | ● `features/07`, `f07-fpoff` (R: the state's content; the listing hides it) | – : the refuting variant (`stale-naf-fork`) is refused | – | 0 | 0 |
| **W-2** | 0 of 167 reordered cubes (`node_census -- --invariance`) | 0 | ● the same 2 (R and H move) | – | – | 0 | 0 |
| **W-3** | ● `alt-cap-shared` (core of a dead root). S1e2.2.11 found 10 programs at the cap, and no other core moved | 0 | 0 | – : no core is printed | – | 0 | 0 |
| **W-4** | ● 7 (cores of N) | ● 2 (partial N) | 0: the 5 complete flagged programs with N > 0 (`core-activator-committed`, `-guard-only`, `forbid-chain-no-absent`, `oblig-stdlib-total-owed` and `-declared`) report cores that name their set. `lattice/06`'s 68 are empty, which is S-2 | – | – | ● 6 | ● 1 (`branching/02`) |
| **W-5** | – : by fork, a monotone program's node is one state on every path (`detector.rs` test). `lattice/11_fifo_gap_static` differs only from scratch, which no route builds | – | ● `lattice/10` (R: the state) | – | – | ● `lattice/10` | 0 |
| **W-6** | – : as W-5 | – | ● 5, stopped (R: members of a lower bound) | – | – | 0 | ● 4 |
| **W-7** | ○ probe (core of a dead root) | ? : any printed core | ? | – | – | ? | ? |
| **W-8** | – : needs an activator a node derives, which is an activator site | – | ○ probe (core of N) | – | – | ? | ? |
| **W-9** | ● `fork-delta-order` | ? | ? | – | – | ● | ? |
| **W-10** | ? (latent, past the cap) | ? | ? | – | – | ? | ? |
| **W-11…W-14** | ○ probes (R, plus H for W-11) | 0 | 0 | only if refused for another reason | only if refused for another reason | 0 | 0 |
| **W-15** | – : needs an activator site | – | ? (no witness; H) | – | – | ? | ? |
| **S-1** | ● W16's pair, the probes | ● `arg-kinds` | ? | ○ via `ein saturate --dump`, which does not refuse | – | ? | ? |
| **S-2** | 0 (no empty core on any finished run) | 0 (the `zebra2-minus-15` pair's 1 495 cores are all non-empty) | ● `lattice/06` | – | – | 0 | 0 |
| **S-3** | ● `alt-cap-core` pair | ? | ? | – | – | ? | ? |
| **S-4** | 0 | 0 | ● `tests/region/07` | – | – | ● same | 0 |
| **S-5** | ○ | ○ | ○ | – | – | ○ | ○ |
| **S-6, S-7** | – : the detector runs on flagged programs only | – | ? | ? | – | ? | ? |
| **S-8, S-9** | – | – | ? | ● the witnesses (invisible) | – | ? | ? |
| **S-10** | 0 | 0 | 0 | – | – | 0 | 0 (it needs an `instance`-typed relation with a non-empty H, and none exists) |
| **S-11** | – : relation-grain sites flag the program | – | ● `features/01` | – | – | 0 | ● same |
| **S-12, S-13** | – (monotone lines are trivial by theorem) | – | ● `lattice/04` (S-12) | S-13's dead holes, invisible | – | ● `lattice/04` (S-12) | ? |

### C.4 The witnesses' own status

Each open or parked witness: its detector verdict, and R as `ein solve FILE
--json-summary` reports it (2026-11-05, `cdcc3994`).

| witness | item | detector | k_s / k_c | N | H | exhausted |
|---|---|---|---|---:|---:|---|
| `examples/features/07_unstratifiable.ein` | W-1, W-2 | flagged, conservative (`derive-p`, `derive-q`: `absent`, even cycle) | 2 / 2 | 0 | 1 | yes (route 2) |
| `ein-bugs/wrong-answers/f07-fpoff.ein` | W-1 | the same | 2 / 2 | 0 | 1 | yes |
| `probes/f07_rules_swapped.ein` | W-2 | the same | 2 / 2, `{(p one)}` | 0 | 1 | yes |
| `ein-bugs/wrong-answers/alt-cap-shared.ein` | W-3 | monotone | 0 / 0, root dead | 1 | 133 (all dead alone) | yes |
| `examples/zebra2.ein` | W-4 | monotone | 1 / 261 982 | 366 | 56 | yes |
| `examples/zebra2-minus-15.ein` | W-4 | monotone | ≥ 32 / ≥ 865 | ≥ 1 495 | — | no (budget) |
| `examples/lattice/10_fifo_gap.ein` | W-5 | flagged, conservative (`r2`: `absent`) | 4 / 4 | 0 | 2 | yes |
| `features/01`, `features/04_open`, `square-unique/*` | W-6 | flagged, *not known* | ≥ 64 655, ≥ 64 761, ≥ 65 214 / 64 817 / 64 514 | 0 | 35, 81, 118 / 114 / 153 | no (limit) |
| `findings/F2_core_cut.ein` | W-7 | monotone | 0 / 0, root dead | 1 | 0 | yes |
| `probes/guard_only_{second,first}.ein` | W-8 | flagged, conservative (`w`: activator) | 2 / 2 | 1 | 2 | yes |
| `ein-bugs/fork-delta-order.ein` | W-9 | monotone | 3 / 3 | 1 | — | yes |
| `findings/F1_keyword_prefix_split.ein`, `F1b_or_prefix.ein` | W-11 | monotone | 8 / 8 (right: 1 / 1) | 0 | 3 (right: 0) | yes |
| `probes/wild_in_guard.ein` | W-12 | monotone | 2 / 2 | 0 | 1 | yes |
| `probes/bare_premise.ein` | W-13 | monotone | 12 / 16 | 0 | 4 | yes |
| `probes/nested_unbound_head.ein` | W-14 | monotone | 64 / 64 | 0 | 6 | yes |
| `examples/syntax/arg-kinds.ein` | S-1, G-6 | monotone | ≥ 112 793 / ≥ 2⁴¹ | 0 | 41 | no (budget) |
| `probes/n5_string_space.ein` | S-1, S-5 | monotone | 40 / 64 | 0 | 6 | yes |
| `examples/lattice/06_friends_tables.ein` | S-2 | flagged, *not known* (`group-seated-owed`: trigger) | ≥ 21 / ≥ 21 | ≥ 68 (cores empty) | 19 | no (limit) |
| `ein-bugs/alt-cap-core.ein`, `-reordered` | S-3 | monotone | 0 / 0, root dead | 1 | — | yes |
| `tests/region/07_cube.ein` | S-4 | flagged, exercised (`total-owed`: trigger), 1 hole | 3 / 3 | 0 | 2 | yes |
| `examples/features/01_not_and_absent.ein` | S-11 | flagged, *not known* | ≥ 64 655 | 0 | 35 | no (limit) |
| `examples/lattice/04_obligation_trigger_hole.ein` | S-12 | flagged, exercised, 1 hole | 3 / 3 | 0 | 2 | yes |
| `probes/refuted_region_claim.ein` | D-3 | monotone | 2 / 2 (the claim says 1) | 0 | 1 | yes, exit 1 |
| `probes/hrule_no_fact.ein` | D-8 | monotone | 2 / 2 | 0 | 1 | yes |
| `probes/kwpair_in_pattern.ein` | G-15 | monotone | 1 / 1 | 0 | 0 | yes |

---

## D. The 2026-10-14 cases, re-taken

The cases of [`inventory.md`](../../plans/m1e2_correctness/inventory.md) as they stand at `cdcc3994`. A W
case is re-taken through the register: `wrong_answers.rs` passes on the
snapshot, so every row answers what it records. The S and P lines were
re-run here.

| case | today | settled by |
|---|---|---|
| **W1** maximal not recorded | fixed: 13 rows | 11 by the switch `fbcae3b2` (S1e2.1a.5). `alive-empty-interlayer` and `d3-unsat-core` by *forbid*'s refusal, `597b2b7e`. Stage S1e2.2.1 dropped |
| **W2** forced positive refutes | fixed: 2 rows | `fbcae3b2`. S1e2.2.2 dropped |
| **W3** `dead` not upward-closed under `absent` | fixed by refusal: 4 rows | `597b2b7e` (S1e2.2.5, Q-M1e2.5 *forbid*) |
| **W4** fresh name unreachable | fixed: 2 rows | `02e9fb5f` (S1e2.2.4) |
| **W5** obligations join superset | fixed (k = \|R\| = 7) | `fbcae3b2`. S1e2.2.3 dropped |
| **W6** uncovered certified | fixed as a scoped answer: 2 rows. The gap stays G-3 | `52004a68` (S1e2.2.8, Q-M1e2.3 (a)) |
| **W7** binding-key layout | fixed | `180ec47e` (S1e2.2.7) |
| **W8** stale NAF positive | `stale-naf-fork` fixed by refusal (`597b2b7e`). **`features-07` open** (W-1) | **F21 D2** |
| **W9** kill cache off | fixed: 2 rows; `-K` retired | `fbcae3b2`. S1e2.2.9 dropped |
| **W10** alt-cap core | `alt-cap-core` fixed (`e8258f08`, S1e2.2.11). **`alt-cap-shared` open** (W-3) | **F22** |
| **W11** core id order | fixed: 3 rows | `fbcae3b2`; `597b2b7e` (refused); `10858b96` (`fork-delta-order`, S1e2.2.12) |
| **W12** empty core | fixed | `fbcae3b2`, closed at the revisit S1e2.1.7. S1e2.2.13 dropped |
| **C1** types as objects | not-wrong | Q-M1e.12 (a), 2026-11-03. `branching/06` and `/07` retyped and the `types` row built by S1e2.2.10 (G-4) |
| **S1** not-exhausted reason | gone | `fbcae3b2`. Re-taken: `ein solve -n 1 examples/zebra2.ein` prints *"no — stopped as asked; each state shown is certified, nothing is said of the rest"* |
| **S2** trace headline | gone | `fbcae3b2`. Re-taken on `pets.ein --region-budget 100 --trace`: headed by `states (k_s) 16 (a lower bound …)`, with no *No solution* |
| **S3** JSON on a truncated run | gone | `fbcae3b2`. Re-taken: `verdict null`, `exhausted false`, `stopped.why budget` |
| **S4** hypgen read-outs disagree | fixed | S1e2.3.1 (`c6d9ef8d`). Not re-run here; held by `summary_properties.rs` |
| **S5** clamped budget | fixed under ruling (b) | S1e2.3.4. Re-taken: `-v -g -3` exits 2 |
| **S6** "no query goal to project" | gone | `fbcae3b2`. Re-taken on `lattice/02`: *"the goal has no row: undetermined here"* |
| **S7** goal row unsorted | fixed, pinned | T1e2.1a.5.2 / S1e2.3.4. Not re-run (needs a permuted interner; `id_order_invariance.rs`) |
| **S8** repr in core | fixed | `fbcae3b2`. Re-taken: `unsat core (not (p A))` |
| **S9** tree `layers_explored` | gone | S1e2.1c.5 deleted the walk. Re-taken: `EIN_TRAVERSAL=tree` leaves stdout byte-identical |
| **P1** keyword-pair render panic | fixed | `38b2bff3` (S1e2.3.2). Re-taken: `render constraints` and `render program` exit 0 |
| P2 | debug-only `debug_assert` | not re-taken (no debug build) |

The § 6 "suspected" items:
- **Q-M1e.11** is still without a witness (W-15).
- **`hrule-reads-not.ein`**: release answers R = {∅} with no candidate and exits 0. The debug assertion was not re-taken. No owner.
- **`emit_closed`** is run by no route (S-14).
- **The inter-layer cascade** is moot: the walk was deleted.
- **Non-stratified programs** are flagged, with `cycle` rows (G-7).

---

## Files

Taken under `build/m1e2_inventory/`. Kept in the repository:

- **This document**, `inventory_2026-11-05.md`.
- **The probes**: [`inventory_2026-11-05/probes/`](inventory_2026-11-05/probes/) holds the re-take probes, each with its question in its header. [`inventory_2026-11-05/findings/`](inventory_2026-11-05/findings/) holds the two new findings.
- **The scripts**: `sweep_describe.py`, `sweep_cores.py` and `classify.py`, in [`inventory_2026-11-05/`](inventory_2026-11-05/). `region_census.py` is `utils/`'s.
- **The join**: [`corpus.csv`](inventory_2026-11-05/corpus.csv), one row per corpus program.

The JSON sweeps, the extraction records (374 items with source line and quote) and the run logs were not kept. The scripts re-take the sweeps. Each script writes its JSON beside itself and reads the tree from `./src` and the binary from `./target/release/ein`. So a re-take copies `inventory_2026-11-05/` under `build/`, puts a `git archive` of the tree being re-taken in `src/` and its release build in `target/`, and runs the scripts there: `sweep_describe.py`, `utils/region_census.py --all`, `sweep_cores.py`, then `classify.py`.
