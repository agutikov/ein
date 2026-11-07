# M1e2 inventory: the 13 open items with no owner

> **Taken 2026-11-06** at `916d97ac`, on the user's request of that day: *"skip fixed and solved, skip not wrong, skip postponed to followups, what is left open in m1e2 and belong to nowhere else?"*, then *"make a separate inventory of only those 13 open items"*. It is a cut of [the inventory of 2026-11-06](inventory_2026-11-06.md), whose *For the close* section lists them as having no owner. **It waits on the user's reading. The close has not started.** No engine code or answer changed.

**How it was taken.** Every command block below was run from the repository's root on the release binary of `916d97ac`, and its lines are pasted from that run, filtered to the rows that matter. The whole output of each is in the review bundle, `build/review/`, whose entry page is `build/review/README.md` and whose page for an item is `build/review/items/<id>.md`. `build/` is gitignored, so those paths are not links. A number in a table was taken by the command its column names, on the same date. A number quoted from the inventory says so.

**What is left out.** Everything fixed or ruled not wrong. Everything deferred to a followup or to M1f: the ten open register rows, and W-20 and W-23, which have no row but do have an owner. So is every gap the inventory counts as owned by its page, apart from § 3 below.

---

## 0. Summary

**Only W-22 is a known wrong answer.** The S, D and P items leave R, its lines and every count right: they misreport a right answer, misdescribe it on a page, or cost work. G-21 and G-22 are not called wrong until they are ruled: each asks whether a program that loads today should load at all.

| id | kind | what is wrong | touches | programs it reaches | what it needs |
|---|---|---|---|---|---|
| **W-22** | wrong answer | `_` as a fact's argument is dropped at load, in silence | **R, H** | monotone: `syntax/arg-kinds` (stopped), `syntax/is-a-shapes` (complete), and a probe | **ruled 2026-11-06**: a bare `_` is refused, [S1f.5.1](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md), not started |
| **S-15** | surface | `--diff-models` compares facts as text, not as values | a surface | any: probes only | **ruled 2026-11-07**: [S1f.5.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#task-t1f556----diff-models-compares-by-value-s-15) |
| **S-16** | surface | past the limit, `--minimal --single` and `--minimal -n 1` say less than describe certifies | a surface | non-monotone: all 7 stopped at the limit | **ruled 2026-11-07, (b)**: [S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md) |
| **S-17** | surface | the minimal-line row says *no obligation is stated* where no node the run met owes | a surface | non-monotone: a probe, at a lowered limit | **ruled 2026-11-07, (a)**: [S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md#s-17-and-f1-the-clause-goes) |
| **F1** | surface | the same sentence on complete runs of programs that state one | a surface | monotone: 8 under `tests/stdlib/` | **ruled 2026-11-07, (a)**: [S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md#s-17-and-f1-the-clause-goes) |
| **S-18** | surface | a guard as a query's goal prints *the goal holds: it has no variable* | a surface | monotone: a probe | **ruled 2026-11-07**: [S1f.5.7](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md) |
| **D-14** | spelling | an owing hole that is not dead is written `core: []`, where `null` means no core taken | a surface | non-monotone: 8 holes on 7 programs | **ruled 2026-11-07**: [S1f.5.8](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md) |
| **D-15** | page | `02_patterns.md` describes `:where` as a filter, and its T3 example no longer loads | a page | refused at load: the page's own example | [S1f.5.11](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md), ruled 2026-11-07 |
| **G-21** | gap | a rule may conclude a fact of the reserved `neq` | unruled | monotone: a probe, 0 corpus programs | **ruled 2026-11-07**: [S1f.5.7](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md#conclusions-g-21) |
| **G-22** | gap | a keyword pair beside an `:assert`'s conjuncts is dropped in silence | unruled | monotone: a probe, 0 corpus programs | [S1f.5.11](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md), ruled 2026-11-07 |
| **P-14** | cost | past the limit, `--minimal -n 2` and `--minimal --single` visit about 2^limit nodes where one saturation decides | cost | non-monotone: all 7 stopped at the limit | **closed by S-16's ruling, 2026-11-07**: [S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md) |
| **P-15** | cost | N's cores past the limit are computed, then written `not-kept` | cost | non-monotone: `lattice/06`, `past-limit-lines` | **ruled 2026-11-08, (c)**: [S1f.5.12](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md) |
| **P-16** | cost | four dumps past the limit are most of one test's time | cost | that test | **ruled 2026-11-08, the slow tier**: [S1f.5.13](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md) |

**The user's review is done (2026-11-08): every item is decided**, and each decision is a plan in [P1f.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md) (P1e2.5 until the user moved it to M1f on 2026-11-08, its stages renumbered `S1e2.5.<n>` → `S1f.5.<n>`), whose table says where each item stands. P-14 was closed by S-16's ruling, and the rest each have a stage. Nothing is built: the stages wait for the user to say which run, and in what order.

---

## 1. The items

### W-22: `_` as a fact's argument is dropped at load

**What is false.** `(r _ ?x)` against the declared `(relation r T T)` is stored as the one-argument `(r "?x")`, with no line. The string `"?x"` then becomes a name of the program, and so a candidate object. The same `_` in a rule's guard has been refused at load since W24, so the shape is refused in a rule and read as nothing in a fact.

The probe, [`fact_wild_arg.ein`](inventory_2026-11-06/probes/fact_wild_arg.ein):

```lisp
(relation r T T)
(r _ ?x)
(r a b)
```

```text
$ ein saturate plans/m1e2_correctness/inventory_2026-11-06/probes/fact_wild_arg.ein --dump
  (r "?x")
  (r a b)
[exit 0]
```

```text
$ ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/fact_wild_arg.ein
  states (k_s)      32
  lattice           monotone (the static check) — R is convex: no hole
  universe          5 candidates, 0 dead alone
    (model (r "?x") (r a b))
    ;; state 2 of 32 — 1 candidate · from {(r "?x" a)}
    (model (r "?x" a) (r "?x") (r a b))
    ;; state 3 of 32 — 1 candidate · from {(r "?x" b)}
    (model (r "?x" b) (r "?x") (r a b))
… 57 more lines
[exit 0]
```

The corpus program that writes the shape, stopped at its budget:

```text
$ ein saturate examples/syntax/arg-kinds.ein --dump
  (r 7 "a string")
  (r "1..5" "1..*")
  (r "?x")
  (r (nested a b) c)
[exit 0]
```

```text
$ ein solve examples/syntax/arg-kinds.ein
  states (k_s)      112 793   (a lower bound: complete up to states of 3 candidates)
  exhausted         no — R is decided: ∅ and ⊤ are both in it, so it is the whole cube (L1, L2), and only its states are not all counted. The state route stopped before the states of 4 cand…
  lattice           monotone (the static check) — R is convex: no hole
  universe          41 candidates, 0 dead alone
[exit 0]
```

**A second corpus program writes the shape**, which the inventory of 2026-11-06 does not name (found 2026-11-06, reading this item): [`syntax/is-a-shapes.ein`](../../examples/syntax/is-a-shapes.ein) writes `(is-a _ T)`, and `ein saturate FILE --dump` stores `(is-a T)`. Its run is complete, with one state. The same drop reaches an activator, where `(q _)` stores `(q)`, and a nested fact, where `(r (f _ a) c)` stores `(r (f a) c)`.

The same `_` in a rule, as a control:

```text
$ ein solve plans/m1e2_correctness/inventory_2026-11-05/probes/wild_in_guard.ein

kb load error: rule 'e': its :match writes (eq ?x _), and _ names no value — a guard compares two values, and _ stands for an argument of a pattern; rule 'n': its :match writes (neq ?x _), …
[exit 1]
```

**Right.** The probe's header gave two answers: refused at load, or kept as written. [S1e2.3.8 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md#notes) put it *beside W24's shapes*. **The user ruled 2026-11-06** ([Q-M1e2.22](open_questions.md#q-m1e222--does-ein-need-a-wildcard)): a bare `_` is reserved and refused, and `_` is valid anywhere in an atom's or a variable's name. The plan is [S1f.5.1](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md), not started.

**Where.** [`from_ir::fact_args`](../../ein.rs/crates/ein-ir/src/from_ir.rs), line 589: `atomic_value` answers nothing for a wildcard, the form branch does not take it, and the argument is skipped.

### S-15: `--diff-models` compares text

**What is misreported.** A models file that writes a fact in another spelling of the same value does not match it. `(p "a")` and `(p a)` are one value.

```text
$ ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/diff_models_text.ein --diff-models plans/m1e2_correctness/inventory_2026-11-06/probes/diff_models_text.models
  0 matched, 0 near, 1 missing, 4 spurious — the sets differ
  - missing: model 1 of the file [(p "a") (q "x y")]
  + spurious: state 1 of 4
  + spurious: state 2 of 4
  + spurious: state 3 of 4
  + spurious: state 4 of 4
[exit 0]
```

```text
$ ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/diff_models_text.ein --diff-models plans/m1e2_correctness/inventory_2026-11-06/probes/diff_models_text_control.models
  1 matched, 0 near, 0 missing, 3 spurious — the sets differ
  + spurious: state 2 of 4
  + spurious: state 3 of 4
  + spurious: state 4 of 4
[exit 0]
```

**Right.** [S1e2.3.8 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md#notes): normalising the file's facts through the parser would make it compare by value. It is a report, and no answer moves.

**Ruled 2026-11-07, as recommended:** the file's facts are read through the parser and compared by value, in [S1f.5.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#task-t1f556----diff-models-compares-by-value-s-15). After S1f.5.2 and S1f.5.4 what is left is spacing (`(p  a)`, `( p a )`), a trailing comment, and a fact with no `)`, which loads in silence and never matches (re-taken 2026-11-07).

**Where.** The option is in [`cmdline.rs`](../../ein.rs/crates/ein-cli/src/cmdline.rs), line 606, and the pairing in [`models.rs`](../../ein.rs/crates/ein-render/src/models.rs).

### S-16: past the limit, two line questions say less than describe

**What is misreported.** Where ∅ ∈ R, the minimal line is {∅}, and describe certifies it. `--minimal --single` says *not decided* and `--minimal -n 1` says *may hold more*. This is weaker than describe, not contrary to it: no state is mislabelled and no word is stronger than the run certifies.

| program | `--minimal --single`: verdict | `--minimal -n 1`: minimal line | bare `ein solve`: minimal line |
|---|---|---|---|
| `features/01_not_and_absent` | *not decided — the run stopped first* | *1 state, as asked — the minimal line may hold more* | *1 state — ∅, the trivial line: no obligation is stated* |
| `features/04_open` | *not decided — the run stopped first* | *1 state, as asked — the minimal line may hold more* | *1 state — ∅, the trivial line: no obligation is stated* |
| `lattice/06_friends_tables` | *not decided — the run stopped first* | *1 state, as asked — the minimal line may hold more* | *1 state — ∅* |
| `saturation/square-unique/corner-house` | *not decided — the run stopped first* | *1 state, as asked — the minimal line may hold more* | *1 state — ∅, the trivial line: no obligation is stated* |
| `saturation/square-unique/cul-de-sac` | *not decided — the run stopped first* | *1 state, as asked — the minimal line may hold more* | *1 state — ∅, the trivial line: no obligation is stated* |
| `saturation/square-unique/terminus` | *not decided — the run stopped first* | *1 state, as asked — the minimal line may hold more* | *1 state — ∅, the trivial line: no obligation is stated* |
| `ein-bugs/wrong-answers/past-limit-lines` | *not decided — the run stopped first* | *1 state, as asked — the minimal line may hold more* | *1 state — ∅* |

`--minimal -n 1`'s wording holds within the limit too, where `--single` does decide:

```text
$ ein solve examples/lattice/10_fifo_gap.ein --minimal -n 1
  asked             1 state of the minimal line (-n 1)
  minimal line      1 state, as asked — the minimal line may hold more
  exhausted         no — stopped as asked; each state shown is certified, nothing is said of the rest
[exit 0]
```

```text
$ ein solve examples/lattice/10_fifo_gap.ein --minimal --single
  asked             is the minimal line one state? (--single)
  verdict           Solution — the minimal line is one state
  decided           yes — the minimal line holds no other state
[exit 0]
```

**What it needs: the user's ruling.** [S1e2.2.26 § Notes](p1e2.2_wrong_answers/s1e2.2.26_line_modes_past_the_limit.md#notes): *making either say what describe says would change an answer the ruling does not name*. So Q-M1e2.19 does not cover it. Either leave both words weaker than describe, or have them say what describe says.

**Ruled 2026-11-07, (b):** *"b, use the ∅ rule in both"*. Where ∅ is in R the minimal line is {∅}, so every minimal question answers it after one saturation. The plan is [S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md), which closes P-14 with it: of the 213 corpus programs with ∅ in R, the 23 the static check flags are the ones that do not stop at root (2026-11-07).

**Where.** [`read_out.rs`](../../ein.rs/crates/ein-render/src/read_out.rs), lines 1297 and 1323.

### S-17 and F1: *"no obligation is stated"*

**What is misreported.** The row reads whether a node the run met owes (`owing_met`), not whether the program states an obligation. One fix covers both items. They are two only because S-17 was found on stopped runs and F1 on complete ones.

**S-17, on a stopped run.** `past-limit-lines.ein` states `total-owed`, and a node that owes is first met at limit 2:

| `--region-limit` | obligation plans | the row it prints |
|---|---|---|
| 0 | 1 | *1 state — ∅, the trivial line: no obligation is stated* |
| 1 | 1 | *1 state — ∅, the trivial line: no obligation is stated* |
| 2 | 1 | *1 state — ∅* |

**F1, on complete runs.** Each of the 8 corpus programs states an obligation that root discharges, so no node owes and ∅ is the minimal line. That is right, and the reason given is false. The last two rows are the finding's probe and its control, which states none and prints the same row:

| program | obligation plans | root owes | the row it prints |
|---|---|---|---|
| `tests/stdlib/algebra/24_total_owed_satisfied` | 1 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `tests/stdlib/algebra/26_surjective_owed_satisfied` | 1 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `tests/stdlib/bijection/06_blind_enumeration` | 3 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `tests/stdlib/closure/02_closed_and_satisfied` | 1 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `tests/stdlib/slots/01_partition` | 2 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `tests/stdlib/slots/08_spatial_adjacent` | 2 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `tests/stdlib/slots/10_owed_room_satisfied` | 1 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `tests/stdlib/slots/12_owed_fill_satisfied` | 1 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `F1_trivial_line_stated_obligation` | 1 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |
| `F1_trivial_line_stated_obligation_control` | 0 | 0 | *1 state — ∅, the trivial line: no obligation is stated* |

**Right.** The finding's header: `∅` with no reason, or a reason that is true, *root owes nothing*. The summary's `minimal.trivial` is `true` on the same runs, read off the same bit ([`region_answer.rs`](../../ein.rs/crates/ein-render/src/dump/region_answer.rs), line 250).

**Where.** [`read_out.rs`](../../ein.rs/crates/ein-render/src/read_out.rs)'s `minimal_line`, line 831.

**Ruled 2026-11-07, (a):** *"a, drop the clause"*. Where ∅ is in R the row reads `1 state — ∅` on every program and at every limit, the `∅` row above it gives the reason, and `minimal.trivial` reads *∅ is in R*. The plan is [S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md#s-17-and-f1-the-clause-goes), beside S-16's rule.

### S-18: a guard as a query's goal

**What is misreported.** `(query :goal (neq w ?y))` says the goal has no variable, and `?y` is one. It also suggests a claim the loader refuses. No corpus program writes a guard as a goal.

```lisp
(relation p Node)
(p a)
(query :goal (neq w ?y))
```

```text
$ ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/goal_guard.ein
  query             (neq w ?y)
                      the goal holds: it has no variable
  as :expect        (model (neq w ?y))
[exit 0]
```

The suggested claim, written down:

```text
$ ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/goal_guard_claim.ein

kb load error: :expect — `?y` is a variable; an expectation is an answer, not a pattern
[exit 1]
```

**Where.** [`read_out.rs`](../../ein.rs/crates/ein-render/src/read_out.rs), line 1823: a row with no bound variable reads as a goal with none.

**Ruled 2026-11-07** ([Q-M1e2.26](open_questions.md#q-m1e226--are-eq-and-neq-relations)): `eq` and `neq` are relations of equality and non-equality, and a query over them is answered. `(neq w ?y)` gives every name of the program but `w`, `(eq w ?y)` gives `?y = w`, and a claim may name them. The re-take found it wider than written: with a guard in its goal, even `(and (p ?y) (neq w ?y))` can carry no claim that holds, since the class lists `(neq w a)` and the loader refuses a claim naming `neq`. The plan is [S1f.5.7](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md).

### D-14: an owing hole written `core: []`

**What is wrong.** In the summary's `region.holes.minimal`, a hole that is not dead carries `core: []` beside `core_status: null`. [S1e2.3.9 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md#notes): the empty list is true, since a hole that is not dead has no contradiction to explain, but `null` is that stage's spelling for *no core*. A spelling, and no answer moves.

`ein solve FILE --json-summary J`, every minimal hole of the six programs the inventory lists and of the past-limit probe. 8 of 8 are written `[]`:

| program | hole | dead | core | core_status | owes |
|---|---|---|---|---|---|
| `lattice/04_obligation_trigger_hole` | `[(seats Ann C1)]` | false | `[]` | `null` | 1 |
| `lattice/08_absent_witness_hole` | `[(allergic Ann), (seats Ann C1)]` | false | `[]` | `null` | 1 |
| `lattice/09_activator_hole` | `[(opens K1)]` | false | `[]` | `null` | 1 |
| `lattice/12_activator_chain_hole` | `[(opens K1)]` | false | `[]` | `null` | 1 |
| `lattice/synth/local_not_global` | `[(h A)]` | false | `[]` | `null` | 1 |
| `lattice/synth/local_not_global` | `[(h B)]` | false | `[]` | `null` | 1 |
| `tests/region/07_cube` | `[(seats Ann C1)]` | false | `[]` | `null` | 1 |
| `ein-bugs/wrong-answers/past-limit-lines` `--region-limit 17` | `[(seats Ann C1)]` | false | `[]` | `null` | 1 |

**Where.** [`region_answer.rs`](../../ein.rs/crates/ein-render/src/dump/region_answer.rs), line 304.

**Ruled 2026-11-07** ([Q-M1e2.27](open_questions.md#q-m1e227--what-do--and-null-mean-in-eins-json)): `[]` is the empty set and `null` an object that does not exist, so a node that is not dead has `core: null`. The re-take found it wider than written: `ein oracle --dump`'s `region.json` writes `core: []` and `refuted_by: []` on every node that is not dead, members of R included. The plan is [S1f.5.8](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.8_no_contradiction_no_core.md).

### D-15: `02_patterns.md` and `:where`

**What is wrong.** [`02_patterns.md`](../../docs/kernel/ir/03-ein-lang/02_patterns.md)'s construct table has two `:where` rows, *`:where` filter* (line 52) and *relation-property tag* (line 53), that describe it as a filter, and so does its section *As a T3 structural rule* (line 181). Line 61 of the same page says the keyword pair is dropped, which is what the compiler does and a test holds ([`compile_semantics.rs`](../../ein.rs/crates/ein-infer/tests/compile_semantics.rs), line 158). The T3 example itself has not loaded since `3ab9a231`:

```text
$ ein solve plans/m1e2_correctness/inventory_2026-11-06/probes/where_t3_example.ein

kb load error: rule 'triangle-composition': its :match writes (?r ?a ?b) with ?r as its relation, which no parameter binds — a relation is named by a name or by a parameter its activator bi…
[exit 1]
```

**Right.** The page is what moves. [S1e2.2.24 § Notes](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md#notes) parked it: the stage kept the drop.

**Ruled 2026-11-07** ([Q-M1e2.28](open_questions.md#q-m1e228--which-keywords-does-each-form-take)): *"we must have only fixed defined kwargs"*. Each form takes a fixed set of keywords, any other is an error, and a term takes none, so `:where` is refused wherever it is written, not dropped. The sweep found six current kernel pages that describe `:where`, not one: `02_patterns.md`, `01-ein-graph/02_rules.md` and its README, `03_examples.md` (whose `transitive` rule loads today with its guard dropped), `01_grammar.md`, the glossary and `defined_behaviour.md`. The plan is [S1f.5.11](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md), which also settles G-15.

### G-21: a rule may conclude `neq`

**The question.** `neq` is reserved and computed, and a rule can still conclude a fact of it, with no line. Is a conclusion of a computed predicate refused at load? 0 corpus programs write it, by the inventory.

```lisp
(relation p Node)
(p a)
(rule r () :match (p ?x) :assert (neq w ?x))
```

```text
$ ein saturate plans/m1e2_correctness/inventory_2026-11-06/probes/assert_neq.ein --dump
  (p a)
  (neq w a :rule r)
[exit 0]
```

**What it needs: the user's ruling.** [S1e2.2.22 § Notes](p1e2.2_wrong_answers/s1e2.2.22_keyword_prefix_split.md#notes): *a label and a question, no answer here*. Under Q-M1e2.17 (i)(a), a guard written `(neq-1 ?x)` in an `:assert` reaches it too.

**Where.** [`compile.rs`](../../ein.rs/crates/ein-infer/src/compile.rs), line 627, is where a built-in predicate is read in a `:match`. The `:assert` side has no such check.

**Ruled 2026-11-07** ([Q-M1e2.26](open_questions.md#q-m1e226--are-eq-and-neq-relations)): a rule may conclude `eq` or `neq`, with no restriction. A true conclusion adds nothing, and a false one, such as `(neq a a)`, is a contradiction, as `(false)` is. Today the conclusion is stored as an ordinary fact, so `(neq a a)` sits in root's KB while a guard computes it false, and R keeps ∅ (2026-11-07). The plan is [S1f.5.7](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.7_eq_and_neq_are_relations.md#conclusions-g-21).

### G-22: a keyword pair beside an `:assert`'s conjuncts

**The question.** In `(and (p ?y) :k v)` the `:k v` is gone, with no line. It is G-15's neighbour: a keyword pair in a `:match` is dropped by ruling (Q32). Is this one refused at load, or dropped the same way? 0 corpus programs write it, by the inventory.

```lisp
(relation hit Node) (relation p Node) (relation tag Node Node)
(hit w)
(rule r () :match (hit ?y) :assert (and (p ?y) :k v))
```

```text
$ ein saturate plans/m1e2_correctness/inventory_2026-11-06/probes/assert_and_kw.ein --dump
  (hit w)
  (p w :rule r)
[exit 0]
```

**What it needs: the user's ruling.** [S1e2.2.24 § Notes](p1e2.2_wrong_answers/s1e2.2.24_silent_patterns_refused.md#notes) parked it, and the stage left it.

**Where.** [`compile.rs`](../../ein.rs/crates/ein-infer/src/compile.rs), line 480: `assert_conjuncts` keeps the positional arguments.

**Ruled 2026-11-07**, with D-15 ([Q-M1e2.28](open_questions.md#q-m1e228--which-keywords-does-each-form-take)): *"G-22 already answered also - there are fixed set of allowed kwargs, if parser sees :k or :G :notes or whatever else unknown kwarg - it is an error"*. A keyword pair beside an `:assert`'s conjuncts is refused at load. The plan is [S1f.5.11](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.11_closed_keywords.md), whose register case holds this probe as a witness.

### P-14: past the limit, work the question does not need

**The cost.** Where ∅ ∈ R, one saturation decides the minimal line. `-n 1` stops there, and `-n 2` and `--single` go on to the limit. No answer moves. *Minimum work per question* is the standard it falls short of.

| program | `--minimal -n 1`: saturations | `--minimal -n 2`: saturations | `--minimal --single`: saturations |
|---|---|---|---|
| `features/01_not_and_absent` | 1 | 64 655 | 64 655 |
| `features/04_open` | 1 | 64 761 | 64 761 |
| `lattice/06_friends_tables` | 1 | 65 299 | 65 299 |
| `saturation/square-unique/corner-house` | 1 | 65 389 | 65 389 |
| `saturation/square-unique/cul-de-sac` | 1 | 64 923 | 64 923 |
| `saturation/square-unique/terminus` | 1 | 65 029 | 65 029 |
| `ein-bugs/wrong-answers/past-limit-lines` | 1 | 65 536 | 65 536 |

**Where.** [`explore.rs`](../../ein.rs/crates/ein-infer/src/explore.rs)'s `flagged`, line 1358.

**Closed 2026-11-07 by S-16's ruling:** the same rule stops each minimal question at root, in [S1f.5.6](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.6_the_empty_state_decides_the_minimal_line.md). Within the limit the 16 other flagged programs with ∅ in R spend 2, 4 or 8 saturations on it.

### P-15: N's cores past the limit

**The cost.** A run past the limit takes a core at every dead node it visits, then keeps each node as its two bits, so the members of N are written `not-kept` beside cores the run had in hand.

68 members of N in `nogoods.json`, by `core_status`: `not-kept` 68.

One process at a time, with nothing else of the bundle running:

| the run | three runs | best |
|---|---|---|
| `ein solve lattice/06_friends_tables` | 1.96 s · 1.96 s · 1.96 s | **1.96 s** |
| `ein solve lattice/06_friends_tables --dump-answer D` | 2.57 s · 2.57 s · 2.59 s | **2.57 s** |

The inventory quotes [S1e2.3.9 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.9_a_core_the_run_did_not_take.md#notes): 0.58 s of that difference is 65 022 core searches no surface prints. This document does not re-take that split.

**What it needs: the user's call**, and the default is to leave it. Neither option changes an answer.
- **(a) Keep N's cores.** `nogoods.json` prints 68 cores where it prints `not-kept`, so a core's facts appear where none are printed.
- **(b) Take none** where neither `--dump-lattice` nor `--events` asks for a visited node's core.

**Where.** [`region.rs`](../../ein.rs/crates/ein-infer/src/region.rs), line 1368.

**Ruled 2026-11-08, (c)**: *"P-15 - c"*. No core during the walk, and after it a core for each member of N, which `nogoods.json` and `--trace` print, through the node function against the same snapshot, as the minimal holes' already are. The rule is the one of 2026-11-05, a core only where a surface prints one. `nogoods.json` gives each member past the limit its core (68 on `lattice/06`, 15 on `past-limit-lines`), and `not-kept` retires. The plan is [S1f.5.12](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.12_cores_for_n_only.md).

### P-16: four dumps in one test

**The cost.** [`answer_round_trip.rs`](../../ein.rs/crates/ein-cli/tests/answer_round_trip.rs) dumps four programs past the limit. The inventory did not re-take this. The test's two runs are consecutive, so the second has no compile in it:

| the run | exit | wall | dump size |
|---|---|---|---|
| `cargo test -p ein-cli --test answer_round_trip` | 0 | 34.54 s · 19.59 s | — |
| `ein solve saturation/square-unique/corner-house --dump-answer D` | 0 | 2.85 s | 268 MB |
| `ein solve saturation/square-unique/cul-de-sac --dump-answer D` | 0 | 3.2 s | 275 MB |
| `ein solve saturation/square-unique/terminus --dump-answer D` | 0 | 3.78 s | 334 MB |
| `ein solve features/04_open --dump-answer D` | 0 | 3.63 s | 318 MB |

[S1e2.3.8 § Notes](p1e2.3_surfaces_that_misreport_the_answer/s1e2.3.8_the_fact_printer.md#notes) quotes about 400 MB each and 30 s (2026-11-05). Both measure lower here: 268 to 334 MB, and a test of about 20 s once compiled. The four are not `slow` entries, so the test's default run dumps them. The dump rows are the release binary on its own and the test is a debug build, so this table does not settle whether the dumps are *most of* the test's time.

**Ruled 2026-11-08**: *"move to slow tier"*. `slow` cannot carry the four, since it is a claim about declared runs and theirs are cheap, so a manifest field by state-file count, held in both directions, selects them. The re-take found that the test's slow tier runs in no CI tier (`grep -n answer_round_trip .github/workflows/*.yml run_tests.sh`, empty), so the plan adds the nightly step too. `past-limit-lines` keeps a run past the limit per commit. The plan is [S1f.5.13](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md).

---

## 2. Read across

- **R.** Only W-22 gets R or H wrong, on two corpus programs: `syntax/arg-kinds`, whose R is a lower bound anyway, and `syntax/is-a-shapes`, whose one state holds `(is-a T)`. No other item moves R as things stand. Two could move something if ruled that way: S-16's fix would change what two line questions answer, and a refusal under G-21 or G-22 would stop a program that loads today.
- **Monotone programs.** W-22 and F1 reach corpus programs, W-22 two of them. S-15, S-18, G-21 and G-22 show on probes only.
- **Non-monotone programs.** S-16, S-17, D-14, P-14 and P-15 are all on flagged programs. S-16, P-14 and P-15 need a run past the limit, and D-14 needs a hole.
- **One fix, two ids.** S-17 and F1. S-16 and P-14 are also neighbours, since both come from a line mode that does not stop where ∅ ∈ R decides the line, but the first is a word and the second a cost.

---

## 3. Close to these, not counted

- **G-6, second half.** A variable or a range written in a fact is a name, and so an object of H. The inventory marks this *unruled*, and counts G-6 as owned by its page. It is the same family as W-22.
- **D-11, the part inside `plans/m1e2_correctness/`.** The inventory assigns it to the close. Four probes still do not load: `cfgtest`, `f07-warn`, `fresh-name-declared-default` and `ucid-perm1`, under [`probes/`](probes/).
- **P-1 to P-12.** Older cost items with no owner either, which the inventory does not ask the close to assign.

## 4. Found during the review

Not in the inventory of 2026-11-06. Both are in [P1f.5's table](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#the-decisions).

- **F2: a token needs no separator.** `(r 1a)` stores `(r 1 a)`, `(r "a"b)` stores `(r a b)` and `(r ?x?y)` stores two variables, each with no line. It waits on a ruling. Its table is [in the phase](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#f2-a-token-needs-no-separator).
- **F3: `_` can report a larger core than a variable used once.** Four facts against two, on a probe, at root and at a dead node. W-22's ruling closes it, and [S1f.5.1](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.1_bare_underscore_refused.md) holds the program.
- **F4: strings as names were never ruled.** `(p "a")` is the fact `(p a)`. **The user ruled 2026-11-06** ([Q-M1e2.23](open_questions.md#q-m1e223--what-is-a-string-for)): a string is natural language, only in `:why`, `:source` and `:goal-text`. The plan is [S1f.5.2](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md), not started. S-15 then shrinks to whitespace, such as `(p  a)`. G-6 is ruled with it: `?x` in a fact is refused.
- **F5: an unknown escape is dropped in silence.** `"a\x41"` stores `ax41`, as `00_ebnf.md`'s fact 4 documents. **Ruled 2026-11-06**: strings are Unicode, with reasonable escapes, in [S1f.5.2](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.2_strings_are_natural_language.md).
- **F6: a range has no user.** `1..5` is the first grammar's *UML-style cardinality*, for a `:cardinality` no program has ever written and no code reads. **Ruled 2026-11-06** ([Q-M1e2.24](open_questions.md#q-m1e224--are-there-numbers-in-ein)): no numbers and no ranges in Ein. The plan is [S1f.5.3](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.3_range_retired.md).
- **F7: digits in a value.** Ein has no number operations, but it has an `INT` token and a value kind, and `(p 1) (p 01)` is one fact. **Ruled 2026-11-06**: digits are plain names, and those are two facts. The plan is [S1f.5.4](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.4_digits_are_names.md).
- **F8: a digit label becomes a candidate.** `features/05`'s N goes 21 → 47 with R unmoved, on a simulation. **Ruled 2026-11-07, as recommended**: a label is a candidate like any other name, [in the phase](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/README.md#f8-a-digit-label-becomes-a-candidate).
- **F9: the grammar is too permissive.** One `Value` serves every position, as it did in the generated parser's grammar, so what a position holds is decided by refusals in the loader. The user's instruction: revisit it, in [S1f.5.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md). **The user ruled 2026-11-07 that the parser is generated, by grmtools**, for its features ([S1f.5.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#the-generator-grmtools-ruled)).
- **F10: deep nesting aborts.** `ein solve` on a fact nested 100 000 deep exits 134 with *has overflowed its stack*; 10 000 deep exits 0 (2026-11-07). The corpus's deepest program nests 8. It goes with the grammar: a stated limit.
- **F11: grmtools' recovery is not deterministic.** CPCT+ ranks equal-length repairs in a `HashSet`'s order and stops at a 500 ms wall-clock budget, so two runs on one file can report differently: two outputs over 20 runs of the probe's `four_mistakes.ein` (2026-11-07). **Ruled 2026-11-07: recovery on**, made deterministic by a patch to `lrpar` the stage vendors ([S1f.5.5](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.5_the_grammar.md#f11-grmtools-recovery-is-not-deterministic-and-this-stage-makes-it-so)).
- **F12: an empty `:match` never fires.** `(rule r () :match () :assert (false))` loads and never fires, so ∅ stays in R where an empty conjunction, which is true, kills root; with an activator, no firing either; `:match (and)` is refused at parse (2026-11-07). 0 corpus programs write it. Filed on the user's instruction and **ruled the same day**: it fires, in rules and hrules, and `(and)` stays refused ([S1f.5.9](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.9_an_empty_match_fires.md)).
- **F13: a conclusion's variable that nothing binds stops the run.** `(rule r () :match (p ?x) :assert (q ?y))` loads, and the run stops at its first firing, exit 1, with the Python implementation's `unbound var ?y in :assert — bindings: {'x': 'a'}` (2026-11-07). **Filed and ruled the same day**: refused at load, with a line naming the explicit spellings, a typed premise or `(eq ?y ?y)` ([S1f.5.10](../m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.10_a_conclusion_variable_must_be_bound.md)).
