# Wrong-answer inventory — `ein` @ `71dd237` (master), taken 2026-10-14

**The evidence [M1e2](README.md) was planned from**, carried as written. A
source document a milestone processes lives inside the milestone, which is the
[`m1d/ideas.md`](../../docs/history/m1d_satisfiability/ideas.md) and M1e
`review/` precedent. Two kinds of edit were made: scratch paths became
[`probes/`](probes/), and a sentence about this directory appearing mid-survey
was dropped. Every case was **re-run** on the release binary on the day.
Nothing in the repo was edited by the survey. The case ids **W1–W12, C1,
S1–S9, P1** are the ones M1e2's stages and the
[case register](p1e2.1_seeing_the_answer/s1e2.1.1_the_case_register.md) cite.

Binary: `ein.rs/target/release/ein` (`ein 0.1.0`, features `einb, parallel, snmalloc`, stdlib = the checkout).
[`probes/capture.sh`](probes/capture.sh) re-takes every witness into `build/m1e2_probes/<id>.txt`;
`probes/scan.py` / `verify.py` / `cascade_scan.py` / `lever_scan.py` are the corpus sweeps quoted in § Reach.
`$S` below = [`probes/`](probes/).

Reference semantics used for "correct": the user's ruling **Q-M1e.6** (solution = saturated ∧ consistent ∧
(owes nothing | every remaining L1 hypothesis makes it dead); quantified over `alive₀`), **Q-M1e.7 ruling A**
(the recorded object is the state, `K` = engine-written negatives are entailed consequences), and the
2026-08-25 user ruling that an owed-and-unpayable state is `Open`, not `Contradiction`
(`tests/stdlib/closure/03_closed_and_owing.ein`). Where a case's verdict depends on a reading those rulings do
not settle, the row says **contested** and gives both answers.

---

## 1. Table

Class: 1 verdict/k · 2 model set · 3 false certification · 4 secondary answer (core) · 5 surface only.
"Lever" = needs a non-default flag/config/env.

### Wrong answers (classes 1–4)

| id | owner ids | class | default / lever | owner today | witness | wrong → right |
|---|---|---|---|---|---|---|
| **W1** `maximal-not-recorded` | Q-M1e.8 (= Q5, F4 Q40, README *Known gaps*) | 1+3 (+4: core of hypotheses) | **default** (`branching/15`, and two new 9-line probes); lever `-L` / `(config :enable-pre-branch-lookahead false)` on 9 corpus entries | **unassigned** (fix *sketched* in D3, no stage) | `examples/branching/15_lookahead_two_step_on.ein`; `$S/forced-2-two.ein`; `$S/owed-two-step.ein`; lever: `examples/lattice/02_genuine_3set_death.ein -L`, `examples/branching/16_lookahead_two_step_off.ein` | `branching/15`: Solution k=1 exhausted → **Ambiguity k=2**; `forced-2-two`: "No solution — contradictory" k=0 exhausted → **Solution k=1, model ∅**; `owed-two-step`: Contradiction → **Open — owes 1**; `lattice/02 -L`: Contradiction → **Ambiguity k=3** |
| **W2** `forced-positive-refutes` | **none (new)** | 1+3+4 (empty core) | **default** | unassigned | `$S/forced-2.ein` (9 lines) | "No solution — contradictory", k=0, exhausted, **core 0 facts**, 0 enterings → **Solution k=1, model ∅** |
| **W3** `naf-upward-closure` | Q-M1e.9, F18, S1f.10.8 (D4) | 2 (default); 1+3 (lever) | **default**; `:enable-singleton-writeback false` + `-L` gives k=0 | F18 (parked followup) + S1f.10.8 (M1f, language half) | `examples/ein-bugs/naf-upward-closure.ein` | model `{(q A)}` (+`(not (p A))`) → **`{(p A),(q A)}`**; lever row: Contradiction k=0 → Solution k=1 |
| **W4** `fresh-name-unreachable` | Q-M1e.21, ST-M1 | 2 (default); 1+3 (as banked, lookahead off) | **default** (header-less variant); banked file sets lookahead off | unassigned | `$S/fresh-name-default.ein` (= banked file minus its `(config …)` line); `examples/ein-bugs/alive-set-fresh-name.ein` (+`-declared` control) | default: Solution k=1 over a model with **no `q`** → **Solution k=1, `{(q A Z),(q B Z)}`**; banked: Contradiction k=0 exhausted → same |
| **W5** `obligations-join-superset` | **none (new)** | 1+2+3 | **default** (any program declaring an obligation rule, lattice traversal) | unassigned | `$S/owed-seats.ein` (9 lines, stdlib `total-owed`+`surjective-owed`) | Ambiguity **k=3 exhausted**: two matchings + `{AC1,AC2,BC1}` (a superset of model 2), and *which* triple depends on the constants' names → **k=2** under the lattice's own design (complete nodes leave the frontier) or **k=7** under Q-M1e.6's literal "owes nothing"; never 3. Tree on the same file: k=6 |
| **W6** `uncovered-certified` | M1d README "Eight measurements" #4, `completeness.md` §3c | 3 (contested) | **default** | unassigned (S1d.10.5 closed without it) | `$S/uncovered.ein` (the §3c fixture, verbatim); `$S/arity3.ein` | k=3 `exhausted = true` while `knows` (owed by nothing, not closed) is never hypothesised — adding `(knows Ann Bob)` gives 3 further consistent models. Right **by Q-M1e.6's 2nd clarification** (alive₀ = the rung's L1 set); an over-claim as a statement about the program |
| **W7** `binding-key-layout` | Q-M1e.16 (ex-Q-M1a.8, S1e.1.4 / review Q3) | 2 (lost derivation) | **default** (release build; debug build panics) | unassigned | 13-line program of `defined_behaviour.md` §3.2 → `$S/binding-key-both.ein` (+`-swapped`, `-nested-only`) | derived `noted` = {(1 1),(2 1),(2 3)} → **{(1 1),(1 3),(2 1),(2 3)}** (swapping the two activator lines restores it) |
| **W8** `stale-naf-positive` | **none (new)**; adjacent to `absent_semantics.md` C1/C6, F18 (negatives only) | 2 (contested) | **default** | unassigned (nearest: S1f.10.8) | `examples/features/07_unstratifiable.ein`; `$S/stale-naf-fork.ein` | records `{(p one),(q one)}` — the file's own header: *"`{p, q}` is not a model of this program under any reading"* — because `(p one)`, derived under `(absent (q one))`, survives the commitment of `(q one)`. Under Q-M1e.6 read literally (append-only, saturated = quiescent) it qualifies |
| **W9** `kill-cache-off-entailment` | consequence of Q-M1e.7 ruling A | 1 (+2) | lever `-K` only | none (ruling says "-K is the cache being less complete") | `examples/ein-bugs/complete-records-stale.ein -K`; `examples/ein-bugs/alive-empty-phase1.ein -K` | `-K`: Ambiguity k=2 → **Solution k=1**; `-K`: Solution k=1 → **Contradiction k=0** |
| **W10** `alt-cap-core` | Q-M1e.15 (review Q2, S1e.1.3) | 4 | **default** | unassigned | `examples/ein-bugs/alt-cap-core.ein` vs `alt-cap-core-reordered.ein` (one `:priority` apart) | core 3 facts `(p1 Y01)(p2 Y01)(p3 Y01)` → **2 facts `(w1 X)(w2 X)`** (the smallest frontier) |
| **W11** `core-id-order` | `corpus/fuzz_findings/unsat-core-id-space.md` (design/02 class); `d3-unsat-core.ein` | 4 | **default** (moves under a permuted interner) | none ("a decision nobody has taken") | `corpus/fuzz_findings/unsat-core-id-space.ein`; `corpus/fuzz_findings/d3-unsat-core.ein` | core `[(r1 o4 o5)]` vs permuted `[(r1 o1 o5) (r1 o4 o1)]`; `d3-unsat-core` 4 vs **6** facts at seed 15839 (its README says green) → one core, chosen canonically |
| **W12** `empty-core` | S1e.3.1 § "did not do" (called "honest") | 4 (contested) | **default** | none | `examples/ein-bugs/alive-empty-phase1.ein` (also `$S/forced-2.ein`) | `unsat core (0 facts)` on an exhausted `Contradiction`; the empty program is satisfiable, `{(is-a A T),(is-a B T)}` is not |
| **C1** `types-as-objects` | Q-M1e.12 (D8) — *"not filed as a defect"* | 2 (contested), hedged | **default** | unassigned | `examples/branching/06_lookahead_on.ein` | k=22 *(lower bound)*: **all 22** recorded models contain a `co-located` fact over a type (`Color`/`House`); 10 of 22 have a goal row `?h = Color`/`House`, 6 print it. Right answer depends on Q-M1e.12's reading (a/b/c) |

### Surfaces that state something false (class 5)

| id | owner ids | default / lever | owner today | witness | what is false |
|---|---|---|---|---|---|
| **S1** `not-exhausted-reason` | Q-M1e.5 (4th instance, S1e.3.2) + **new site** `verdict.rs:200` | lever (tree) for `ein test`; **default** for the hint (`-e` hitting `-m`) | Q-M1e.5 → T1d.10.6.4 / S1f.10.7 | `EIN_TRAVERSAL=tree ein test -v $S/tree-owed.ein`; `ein solve -e -m 1 examples/zebra2.ein` | NOT CHECKED says *"Either the run stopped at -n, or the frontier is still alive at --max-set-size"* — neither applies under the tree; `(not certified — pass --exhaustive)` is printed on runs that **did** pass `-e` |
| **S2** `trace-headline-unqualified` | **none (new)**; the half of T1d.10.5.2b / S1d.3.3 that did not reach `--trace` | **default** (declared run of `pets.ein`) | unassigned | `ein solve -e examples/saturation/type-exclusivity/pets.ein --trace …` | trace says **"No solution — 12 commitments refuted"** on a program with 39 models (search truncated at `-m 5`); "Solved … N solution(s)" is likewise unqualified. **Pinned** by `ein-render/tests/golden/trace_unsat.md` (named "unsat") via `golden_trace.rs` |
| **S3** `json-truncated-contradiction` | Q-M1d.1 (stopping half open) | **default** | — (closed milestone) | `pets.ein --json-summary` | `verdict.type = "Contradiction"` with a 6-fact `unsat_core` on a satisfiable program; only `exhausted: false` disagrees |
| **S4** `hypgen-readouts-disagree` | M1d "Eight measurements" #5 (and #2, `emit_closed`) | **default** | none | `-H` vs `--json-summary` on `zebra2.ein`, `zebra2-minus-15-obligations.ein`, `$S/arity3.ein` | `zebra2`: -H emitted 56 (= layer-1 alive), JSON 38; `minus-15-obligations`: -H 81, JSON 96 (= layer-1 alive); `arity3`: -H "0 candidates", run enters 3 |
| **S5** `clamped-budget` | Q-M1e.17 | lever (negative budget) | unassigned | `ein solve -E=-3 examples/zebra2.ein`; `-m=-3` | prints `max-enterings (0)`; `-m=-3` silently is `-m 0` |
| **S6** `no-query-goal-to-project` | **none (new)** | **default** | none | `ein solve -e examples/lattice/02_genuine_3set_death.ein` (model 3/3) | prints *"(no query goal to project)"* for a model in which the program's goal simply has no row (`answer.rs:316-317`) |
| **S7** `goal-row-unsorted` | `defined_behaviour.md` §6, `corpus/fuzz_findings/d3-goal-row-order.md` | **default** | none (filed) | `corpus/fuzz_findings/d3-goal-row-order.ein` under a permuted interner | the one binding row the table shows is `rows[0]` of an unsorted match: `?x = A` vs `?x = B` |
| **S8** `pyrepr-in-core` | D1 § Related (unfiled aside) | **default** | none | `$S/nested-core.ein` | core prints `(not Fact(relation_name='p', args=('A',)))` |
| **S9** `tree-layers-explored` | SE-M3 (documented) | lever (tree) | T1d.10.6.4 | any tree run | `layers_explored` carries the deepest node |

### Panics reachable on a well-formed program, still present

| id | owner | witness | output |
|---|---|---|---|
| **P1** `kwpair-render-panic` | `corpus/fuzz_findings/kwpair-below-the-filter.md` (no owner) | `ein render constraints corpus/fuzz_findings/kwpair-below-the-filter.ein` (loader and `saturate` accept it) | `panicked at crates/ein-render/src/dot_util.rs:152:32: not a value node: KwPair`, exit **101** |
| P2 | Q-M1e.16, `hrule-reads-not.md` | debug builds only (`debug_assert`) — release answers | not a release-binary panic; listed for completeness |

---

## 2. Merge / split decisions

- **W1 merges** Q-M1e.8, review Q5, F4 Q40 and the README *Known gaps* bullet: one mechanism — `complete()`
  approximates maximality and a commitment whose every child died is never recorded (`a_layer` becomes `a_prev`
  and is never asked). **New in this survey:** the same mechanism at **layer 0**. When every layer-1 singleton dies,
  `phase2` breaks at `solve.rs:1821` (`if a_layer.is_empty() { break; }`) without asking whether root is now
  complete, and `finalise` (`solve.rs:2879`) unions the dead cores into a `Contradiction`. That is reachable at
  **stock flags** with two candidates that each die in two firings (`$S/forced-2-two.ein`), and its one-firing
  twin (`$S/forced-1-two.ein`) answers `Solution` — same logical content, opposite verdicts. In an obligations
  program it turns the ruled `Open` into `Contradiction` (`$S/owed-two-step.ein` vs `$S/tree-dead.ein`). D3's
  sketched fix (a bitset over `a_prev`) does not cover root; the fix stage must.
- **W2 is split from W1** although `$S/forced-2.ein` shows both: with the cascade disabled
  (`:enable-forced-positive false`) the same file still answers `Contradiction` via W1 (core `(q one)`), and
  fixing W1 alone would not touch W2, because the cascade poisons root first (`<forced-positive>` written as a
  root fact, `solve.rs:2427-2478`, then `has_contradiction` → `root_dead`, `solve.rs:1368-1377` / `1828-1836`).
  Two fixes, one witness each: `forced-2-two` (W1 only), `forced-2` (W2, then W1).
- **W4 is partly W1.** `alive-set-fresh-name.ein`'s banked `k = 0, exhausted = true` is confounded: delete its
  `spawn` rule (no fresh name at all — `$S/fresh-name-nospawn.ein`) and it still answers `Contradiction k=0
  exhausted`, which is W1 at layer 0 under the file's `lookahead false`. The fresh-name defect in isolation is the
  **default-lookahead** variant (`$S/fresh-name-default.ein`): `Solution k=1` over a model with no `q`. The
  `-declared` control differs in having surviving candidates, so it does not separate the two.
- **W5 is split from W6** although `completeness.md` §3c's fixture shows both: W5 (the lattice records a strict
  superset of a recorded model, chosen by name order) is visible without `knows` (`$S/owed-seats.ein`); W6
  (`exhausted` over relations the rung does not scope) is a definitional over-claim.
- **W9 is not merged into CO-M1** (fixed): CO-M1's fix re-saturates what the engine materialised; under `-K`
  the entailed negatives are never materialised, so the recorded state is "consistent" only because the
  entailment was not noticed (Q-M1e.7 ruling A says so in words).
- **W11 merges** `unsat-core-id-space` and `d3-unsat-core` (same class: which core is reported depends on id
  order); **W10 stays separate** (the store's retention order, not the interner).
- **W12 is kept separate** from W11 (no order dependence; premise-less engine writes: `<lookahead-dies-immediately>`,
  `<forced-positive>`).
- **S1 merges** Q-M1e.5's fourth instance with the `(not certified — pass --exhaustive)` hint: both read a `bool`
  and guess the reason a search did not exhaust; the fix is the same reason type.
- **S2/S3 are split from pets.ein's fixed CLI wording** (T1d.10.5.2b): the CLI was fixed; the trace headline and
  the JSON `type` were not.
- **Q-M1e.11 (tree rung flip) is not a row**: no witness exists, and the shipped guard makes any loss a lower
  bound (see § 6).

---

## 3. Cases

Command prefix `ein` = `ein.rs/target/release/ein`, cwd = repo root. Output lines verbatim (blank lines and
uninteresting `stats` rows elided).

### W1 — `maximal-not-recorded` (Q-M1e.8)

**Mechanism.** `hypgen::complete` (`hypgen.rs:971`) = "the generator proposes nothing"; a candidate that dies in
≥ 2 firings is still proposed, so a maximal alive commitment is not `complete` and is not recorded. Layer n+1
proves every child dead and the information is dropped; at layer 0 the loop simply `break`s (`solve.rs:1821`).
`finalise` (`solve.rs:2879-2891`) then reports `Contradiction` with the union of the dead cores — hypotheses,
not program facts — and `exhausted = !truncated` certifies it.

Default flags:
```
$ ein solve -e -s examples/branching/15_lookahead_two_step_on.ein
  solutions (k)   1
  verdict         Solution
  exhausted        true
  enterings        6 (alive=4 dead_pre=0 dead_post=2)
```
Right: k=2 — `{(p A)}` (both children dead) and `{(p B),(p C)}` (hand derivation in the file header,
`solution_semantics.md` §2). Re-solving `program + (p A)` cannot confirm it from the CLI because W2/W1 intercept
(`$S/verify.py` shows `Contradiction`), which is itself a symptom.

```
$ ein solve -e -s -p $S/forced-2-two.ein          # (thing one) (thing two); (q ?x) → (bad ?x) → (false)
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (2 facts)
    (q one)  (q one)
    (q two)  (q two)
  exhausted        true
  enterings        2 (alive=0 dead_pre=0 dead_post=2)

$ ein solve -e -s -p $S/forced-1-two.ein          # control: same, but (q ?x) → (false) in ONE firing
  solutions (k)   1
  verdict         Solution
  (not (q one))
  (not (q two))
  exhausted        true
```
Right for both: Solution k=1, model ∅ (root is consistent and every hypothesis dies with it — Q-M1e.6 clause 3).

```
$ ein solve -e -s $S/owed-two-step.ein            # total-owed likes; every (likes Ann ?) dies in two firings
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (2 facts)
    (likes Ann Soup)  (likes Ann Soup)
    (likes Ann Stew)  (likes Ann Stew)
  exhausted        true
$ ein solve -e -s $S/tree-dead.ein                # control: same obligation, alternatives die in ONE firing
  solutions (k)   0
  verdict         Open — owes 1 (likes: 1)
    likes owes Ann a Food.
  exhausted        true
```
Right: `Open — owes 1` for both (user ruling 2026-08-25, `closure/03`).

Levers:
```
$ ein solve -e -s -L examples/lattice/02_genuine_3set_death.ein
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (3 facts)
    (a-prop X)  (a-prop X)
    (b-prop X)  (b-prop X)
    (c-prop X)  (c-prop X)
  exhausted        true
  enterings        7 (alive=6 dead_pre=0 dead_post=1)
$ ein solve -e -s examples/branching/16_lookahead_two_step_off.ein
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (3 facts)
    (p A)  A has p
    (p B)  B has p
    (p C)  C has p
  exhausted        true
```
Right: `lattice/02` k=3 (the default run says so); `branching/16` k=2.
Under `-L`, **9 corpus entries** go from models to a certified `Contradiction` (§ Reach).

**Pinned:** `branching/15`/`16` `:expect` state today's answers (`(model (p B) (p C))`, `(false)`), corpus rows
in `corpus_exits.txt`/`corpus_shapes.md5`; `ein-cli/tests/read_out.rs` `CELLS` uses `branching/16` as the
corpus's exhausted-`Contradiction` cell (`the_table_prints_the_verdicts_own_count`); `lattice/02`'s `-L` lever
cells (16 `corpus_exits` rows, 45 shape rows) and `lattice_semantics.rs` (golden_audit.md). The two new probes
are not in the corpus.

**Fix caveat (not a defect today):** recording "a surviving commitment whose every superset died" must first
materialise the negatives those deaths entail and re-saturate (ruling A); otherwise it would record states like
`complete-records-stale`'s `{(q A)}` when `kill-p` is made two-step.

### W2 — `forced-positive-refutes` (new)

**Mechanism.** `promote_forced_positives` (`solve.rs:2427-2478`): while `alive` is a singleton `{h}` it writes `h`
into root with a premise-less `<forced-positive>` provenance, re-saturates, and if root is now contradictory
returns `term = true`; the callers (`solve.rs:1368-1377` phase 1, `1828-1836` between layers) call `root_dead`
and the run is a `Contradiction`. Promotion is only valid when `root ∪ {h}` is consistent; when it is not, the
right conclusion is `¬h` and root is the solution. `architecture_and_algorithms.md` calls the cascade "unit
propagation"; there is no clause that requires `h`.

```
$ ein solve -e -s -p $S/forced-2.ein              # one object; (q one) → (bad one) → (false)
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (0 facts)
  exhausted        true
  enterings        0 (alive=0 dead_pre=0 dead_post=0)
$ ein solve -e -s -p $S/forced-2-off.ein          # same + (config :enable-forced-positive false)
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (1 facts)
    (q one)  (q one)
  exhausted        true
  enterings        1 (alive=0 dead_pre=0 dead_post=1)        ← now W1 at layer 0
$ ein solve -e -s -p -L $S/forced-1.ein           # one-firing death, lookahead off
  verdict         No solution — the constraints are contradictory
  unsat core (0 facts)
  exhausted        true
$ ein solve -e -s -p $S/forced-1.ein              # control, defaults
  solutions (k)   1
  verdict         Solution
  (not (q one))
```
Right: Solution k=1, model ∅, for all four. **Pinned:** no. **Reach:** no corpus verdict depends on the cascade
(5 entries use it; all answer the same with it off — § Reach).

### W3 — `naf-upward-closure` (Q-M1e.9)

`dead` is not upward-closed under `absent`; the kill cache (`hypgen.rs:464 write_negated`), the singleton
writeback and the width-1 no-good each make `{(p A),(q A)}` unreachable (D4). Re-taken today:
```
$ ein solve -e -s -P examples/ein-bugs/naf-upward-closure.ein
  solutions (k)   1
  verdict         Solution
  (q A)
  exhausted        true
$ ein solve -e -s -P -K examples/ein-bugs/naf-upward-closure.ein      # right, "by accident" (D4)
  (p A)
  (q A)
  [exit=1]   ← the file's :expect states today's wrong answer and fails on the right one
$ ein solve -e -s -P -L $S/naf-nowb.ein     # + :enable-singleton-writeback false
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (1 facts)
    (p A)  (p A)
  exhausted        true
```
`-L` and `-L -K` also answer `{(q A)}` (D4's matrix holds). Right: model `{(p A),(q A)}`, k=1.
**Pinned:** `:expect (model (is-a A T) (q A) (not (p A)))` (today's wrong answer, meant to break),
`ein-infer/tests/refutation_under_absent.rs` (exposed set: 9 rules / 7 entries), `cli_semantics` EVENT_COVER.
Contained only by `(config :warn-derived-naf true)`.

### W4 — `fresh-name-unreachable` (Q-M1e.21)

`hypgen::candidate_objects` (`hypgen.rs:659`) reads `kb.names()`; `alive₀` is taken once at root, so a constant
a rule asserts inside a fork never becomes a hypothesis.
```
$ ein solve -e -s -P $S/fresh-name-default.ein        # banked file without its (config …) line
  solutions (k)   1
  verdict         Solution
  exhausted        true                                # model has no q fact; (no query goal to project)
$ ein solve -e -s -P examples/ein-bugs/alive-set-fresh-name.ein      # as banked (lookahead off)
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (2 facts)
    (q A B)  (q A B)
    (q B A)  (q B A)
  exhausted        true
$ ein solve -e -s -P examples/ein-bugs/alive-set-fresh-name-declared.ein   # control: + (seen Z)
  solutions (k)   1
  verdict         Solution
  (p Z Z)
  (q A Z)
  (q B Z)
$ ein solve -e -s -P $S/fresh-name-nospawn.ein        # banked file minus `spawn`: no fresh name at all
  verdict         No solution — the constraints are contradictory      ← W1, not W4
  exhausted        true
```
Right: Solution k=1, `{(q A Z),(q B Z)}`. **Pinned:** `:expect (false)` (today's answer),
`ein-infer/tests/alive_invariant.rs` (breach set), `cli_semantics` EVENT_COVER (the `warn` row).

### W5 — `obligations-join-superset` (new)

**Mechanism.** `apriori.rs:138-139`: *"The 'every (k−1)-subset ∈ a_prev' condition holds by
[`apriori_prefix_join`]'s construction and is deliberately not re-verified."* False: the prefix join guarantees
only the two parents. A subset that was recorded `complete` has left the frontier, is not dead, and is not
checked, so its superset is joined. Under the blind/hrule rungs such a superset dies (completeness = every
remaining candidate refuted), so nothing shows; under the **obligations rung** completeness means *discharged*, the
superset is alive and discharged, and it is recorded as a further model — only for the supersets the
name-sorted prefix join happens to build.
```
$ ein solve -e -s -P $S/owed-seats.ein
  solutions (k)   3
  verdict         Ambiguous — distinct complete models; the puzzle is under-determined
── solution 1/3 ── (seats Ann C1) (seats Bob C2)
── solution 2/3 ── (seats Ann C2) (seats Bob C1)
── solution 3/3 ── (seats Ann C1) (seats Ann C2) (seats Bob C1)
  exhausted        true
  enterings        11 (alive=11 dead_pre=0 dead_post=0)
$ ein solve -e -s -P $S/owed-seats-renamed.ein          # Ann → Zed, nothing else
── third model ── (seats Bob C1) (seats Bob C2) (seats Zed C1)   ← maps back to {Bob C1, Bob C2, Ann C1}: a different triple
$ EIN_TRAVERSAL=tree ein solve -e -s -P $S/owed-seats.ein
  solutions (k)   6   (a lower bound — the search did not exhaust)
  (the two matchings and all four triples)
```
Right: k=2 by the lattice's own design (`lattice_semantics::a_layer_that_yields_complete_models_stops_descending`:
a complete node "leaves the frontier"), k=7 by Q-M1e.6's literal obligations arm. A model set that changes
under renaming a constant is wrong under any reading. Also falsifies `completeness.md` §3c's *"The tree agrees
with it … the same seatings"* (today: tree 6, lattice 3). **Pinned:** no. **Reach:** no corpus entry has a
recorded model that is a strict superset of another (§ Reach); `zebra2-minus-15-obligations` (tree 32 = lattice 32)
is protected by its functional constraints.

### W6 — `uncovered-certified` (contested)

```
$ ein solve -e -s -P $S/uncovered.ein          # completeness.md §3c, verbatim
  solutions (k)   3
  exhausted        true                        # none of the 3 mentions knows; rung event: uncovered=2
$ ein solve -e -s -P $S/uncovered-knows.ein    # + (knows Ann Bob)
  solutions (k)   3                            # 3 further consistent models, each with (knows Bob Ann)
$ ein solve -e -s -H $S/arity3.ein             # blind rung: "Arity ≥ 3 is unenumerated" (hypgen.rs fill_slot doc)
  solutions (k)   1
  verdict         Solution
  exhausted        true                        # goal (r3 ?x ?y ?z) never hypothesised
```
Right: relative to `alive₀` (Q-M1e.6 2nd clarification) the answer is "correct by definition"; as a claim about
the program it is at least 6 (and W5 also applies to this file). The M1d record calls it *"a claim stated
without the qualifier that licenses it"*. **Pinned:** no.

### W7 — `binding-key-layout` (Q-M1e.16)
```
$ ein saturate --dump $S/binding-key-both.ein          # (note edge 1) then (note edge (src Y))
  (noted 1 1 :rule note)
  (noted 2 1 :rule note)
  (noted 2 3 :rule note)
$ ein saturate --dump $S/binding-key-swapped.ein       # the two activator lines swapped
  (noted 1 1 :rule note)
  (noted 1 3 :rule note)
  (noted 2 1 :rule note)
  (noted 2 3 :rule note)
```
`BindingKey = (rule, activator, values)` compares two register layouts position by position
(`engine.rs:250 check_layout` asserts against it under `debug_assertions` only). **Pinned:**
`ein-infer/tests/rule_semantics.rs::an_int_beside_a_nested_fact_in_one_position_loses_a_derivation` (expects the
missing fact in release, the panic in debug). **Reach:** 0 corpus activators carry an int or nested fact
(S1e.1.4).

### W8 — `stale-naf-positive` (new, contested)
```
$ ein solve -e -s -P examples/features/07_unstratifiable.ein
  solutions (k)   1
  verdict         Solution
  (p one)
  (q one)
  exhausted        true                   # events: writeback (q one) reason forced-positive
$ ein solve -e -s -P $S/f07-fpoff.ein      # cascade off: the same state, reached by a fork
  (p one)
  (q one)
$ ein solve -e -s -p $S/stale-naf-fork-warn.ein   # p ← (absent q); (p ∧ q) → ⊥; :warn-derived-naf true
  solutions (k)   1
  (not (q one))
  (p one)                                  # no `warn` event of either category
```
`(p one)` is derived at root under `(absent (q one))`; a later commitment of `(q one)` falsifies the guard and
nothing retracts (`absent_semantics.md` "Retraction / truth maintenance: not provided"; C1 covers the fork→root
direction only, F18 covers negatives only). The fixture's own header says `{p, q}` *"is not a model of this
program under any reading"*, and the engine prints exactly that state. In the probe, the model `{q}` is
unreachable (the stale `p` kills it), and with `-L` the cascade (W2) turns it into a false `Contradiction`.
**Pinned:** `features/07` only by digest (`corpus_shapes.md5` 45 rows, `corpus_exits` 7 rows). Under Q-M1e.6 read
literally the state qualifies — hence *contested*.

### W9 — `kill-cache-off-entailment` (lever `-K`)
```
$ ein solve -e -s -P -K examples/ein-bugs/complete-records-stale.ein
  solutions (k)   2
  verdict         Ambiguous — distinct complete models; the puzzle is under-determined
  (q A)  |  (p A) (p B)
  exhausted        true
$ ein solve -e -s -K examples/ein-bugs/alive-empty-phase1.ein
  solutions (k)   1
  verdict         Solution
  exhausted        true
```
Right (ruling A): `Solution k=1 {(p A),(p B)}` and `Contradiction k=0` (the default runs). Not in any declared
lever cell for these files.

### W10 — `alt-cap-core` (Q-M1e.15)
```
$ ein solve examples/ein-bugs/alt-cap-core.ein
  unsat core (3 facts)
    (p1 Y01)  Y01 has p1
    (p2 Y01)  Y01 has p2
    (p3 Y01)  Y01 has p3
$ ein solve examples/ein-bugs/alt-cap-core-reordered.ein
  unsat core (2 facts)
    (w1 X)  X has w1
    (w2 X)  X has w2
```
`Kb::record_justification` keeps ≤ `MAX_ALT_JUSTIFICATIONS = 32` (`kb.rs:95`) by premise count; `explain.rs`
minimises frontier size. **Pinned:** `explain_semantics.rs::the_alternatives_cap_can_enlarge_the_reported_core`;
both files' `:expect (false)` hold either way. **Reach:** only `zebra2-bad` hits the cap, core unchanged.

### W11 — `core-id-order`
Run with the prebuilt debug test binary (no build): `EIN_ID_FILES=<dir> ein.rs/target/debug/deps/id_order_invariance-75bd3028eed5727d`
```
6 of 90 (file, op) pairs move when the ids do:
…/unsat-core-id-space.ein [solve[default]] seed 1
    plain:    CORE 1 [(r1 o4 o5)]
    permuted: CORE 2 [(r1 o1 o5) (r1 o4 o1)]
…/unsat-core-id-space.ein [trace[answer]] seed 1
    plain:    No solution — the constraints are contradictory (unsat core: 1 facts).
    permuted: No solution — the constraints are contradictory (unsat core: 2 facts).
(EIN_ID_SEEDS=3)
…/d3-unsat-core.ein [trace[answer]] seed 15839
    plain:    No solution — the constraints are contradictory (unsat core: 4 facts).
    permuted: No solution — the constraints are contradictory (unsat core: 6 facts).
```
`corpus/fuzz_findings/README.md` says `d3-unsat-core` "is green under a permuted id space" — true at seed 1 only.
The note in `unsat-core-id-space.md` says the corpus reaches the class through a symmetric pigeonhole (blind
`features/12`); not re-run here. **Pinned:** no (findings are frontend-fuzzer seeds only).

### W12 — `empty-core` (contested)
```
$ ein solve -e -s examples/ein-bugs/alive-empty-phase1.ein
  solutions (k)   0
  verdict         No solution — the constraints are contradictory
  unsat core (0 facts)
  exhausted        true
```
Verdict right (ruling A). The refutation's premises are kill-cache negatives with provenance
`<lookahead-dies-immediately>` and no premises, so the frontier is ∅; S1e.3.1 recorded this as *"honest"*. As an
unsat core it is false: the program minus everything is satisfiable. Same for W2's `forced-2` (core 0).

### C1 — `types-as-objects` (Q-M1e.12, not filed as a defect)
```
$ ein solve -e examples/branching/06_lookahead_on.ein
  solutions (k)   22   (a lower bound — the search did not exhaust)
```
Counted from `--json-summary`: 22/22 models contain a `co-located` fact over `Color` or `House`; 10/22 have a goal
row binding `?h` to a type; 6/22 print it as the displayed binding (D8 said "20 of 22"). Hedged count.

### S1 — `not-exhausted-reason`
```
$ EIN_TRAVERSAL=tree ein test -v $S/tree-owed.ein        # lattice: "holds (Ambiguity, k = 2)"
NOT CHECKED  …/tree-owed.ein
  :goal (likes ?who ?what) — NOT CHECKED (Ambiguity, k = 2, recorded = 2)
    every listed model matches, but the search was not exhausted — k is a lower bound, so nothing here is established. Either the run stopped at -n, or the frontier is still alive at --max-set-size.
[exit=1]
$ ein solve -e -m 1 -s examples/zebra2.ein
  solutions (k)   1   (not certified — pass --exhaustive)
  exhausted        false
```
Sites: `expect.rs:147-154` (`ok_if_exhausted`), `verdict.rs:200` (and the tree's `-n 1` run prints the same hint;
passing `-e` under the tree never certifies). **Pinned:** the qualifier string in `verdict.rs`'s own table test and
`read_out.rs`; the NOT CHECKED sentence by `expect.rs` only.

### S2 — `trace-headline-unqualified`
```
$ ein solve -e -s examples/saturation/type-exclusivity/pets.ein --trace pets_trace.md
  verdict         No model found — the search did not exhaust the lattice      ← CLI, fixed by T1d.10.5.2b
$ head -4 pets_trace.md
# Solution trace

> No solution — 12 commitments refuted (12 dead).
$ head -4 ein.rs/crates/ein-render/tests/golden/trace_unsat.md     ← identical, golden named "unsat"
$ ein solve -e -m 12 -s examples/saturation/type-exclusivity/pets.ein
  solutions (k)   39
  exhausted        true
```
`trace/linearize.rs:466-487` builds the headline from `solved_flag` alone. **Pinned:** `golden_trace.rs` (`"unsat"`
→ `trace_unsat.md`).

### S3 — `json-truncated-contradiction`
```
$ ein solve -e examples/saturation/type-exclusivity/pets.ein --json-summary pets.json
{'type': 'Contradiction', 'k': 0, 'exhausted': False, 'reason': None, 'unsat_core': '6 items', 'solutions': '0 items', 'open_states': '0 items'}
```

### S4 — `hypgen-readouts-disagree`
```
$ ein solve -H -n 1 examples/zebra2.ein
root hyps        56 candidates across 5 relations
    raw                125
    emitted            56
  json root.hypgen raw 125 emitted 38
  events layer 1 alive 56
$ ein solve -H -n 1 examples/zebra2-minus-15-obligations.ein
root hyps        81 candidates across 4 relations
    raw                190
    emitted            81
  json root.hypgen raw 230 emitted 96
  events layer 1 alive 96
$ ein solve -e -s -H $S/arity3.ein
root hyps        0 candidates
  enterings        3 (alive=3 dead_pre=0 dead_post=0)
```
Each read-out is wrong on one file. `-H` shows `pre.closed_relation` (the `emit_closed` closure, M1d #2, applied
on a fork only for `-H`/JSON). **Pinned:** not checked.

### S5 — `clamped-budget` (Q-M1e.17)
```
$ ein solve -E=-3 examples/zebra2.ein
** aborted: max-enterings (0) reached **
[exit=2]
$ ein solve -m=-3 -s examples/zebra2.ein
  solutions (k)   0   (none found — the search did not exhaust)
  verdict         No model found — the search did not exhaust the lattice
```

### S6 — `no-query-goal-to-project`
```
$ ein solve -e examples/lattice/02_genuine_3set_death.ein
  model 3/3
    (no query goal to project)
```
The query has `:goal (a-prop ?x)`; model 3 (`{b-prop, c-prop}`) has no row. `answer.rs:316-317` prints the
no-goal sentence for an empty row set. Also on `$S/fresh-name-default.ein`, `hrule-reads-not.ein`.

### S7 — `goal-row-unsorted`
```
…/d3-goal-row-order.ein [trace[answer]] seed 1
    plain:        ?x  = A
    permuted:     ?x  = B
```

### S8 — `pyrepr-in-core`
```
$ ein solve $S/nested-core.ein          # (not (p A)) (q A); (not (p ?x)) ∧ (q ?x) → ⊥
  unsat core (2 facts)
    (not Fact(relation_name='p', args=('A',)))  (not Fact(relation_name='p', args=('A',)))
    (q A)                                       (q A)
```

### P1 — `kwpair-render-panic`
```
$ ein render constraints corpus/fuzz_findings/kwpair-below-the-filter.ein
thread 'main' (79377) panicked at crates/ein-render/src/dot_util.rs:152:32:
not a value node: KwPair
[exit=101]
$ ein saturate corpus/fuzz_findings/kwpair-below-the-filter.ein
[exit=0]
```

---

## 4. Reach — corpus sweeps run for this inventory

Scope: the 135 non-slow `positive`/`stdlib`/`regression` entries that declare a `solve` run (5 error out by design
or time out: `zebra2-minus-15{,-obligations}` > 60 s under `-e`; three `regression` crash-parity files).

| sweep | result |
|---|---|
| default `solve -e`, per-entering outcomes (`-v -g 1`): alive commitments with no live child at an exhausted run ("missed maximal") | 16 entries flagged; each re-solved as `program + C` (`verify.py`): all are either dominated by a recorded model, dead under the entailed negatives, or the known witnesses (`branching/15`, `16`, `alive-empty-interlayer` [k=0 right under ruling A], `alive-set-fresh-name`). **No new corpus witness of W1 at default flags.** |
| recorded model ⊊ another recorded model (W5 signature) | **0** entries |
| `EIN_TRAVERSAL=tree` vs lattice model sets | tree accepts only `zebra2-obligations` in the swept set; identical |
| `-L` vs default | **9** entries go from models to an `exhausted = true` "No solution": `branching/01`, `04`, `15`, `features/11`, `features/13`, `lattice/01`, `lattice/02`, `ein-bugs/complete-records-stale`, `tests/stdlib/bijection/06`; 3 more flip under a cap (hedged); `colors`/`nationalities` go Ambiguity 5 → Solution 1 (hedged) |
| `-K` vs default | certified changes on `complete-records-stale` (k 1→2) and `alive-empty-phase1` (Contradiction→Solution); lower-bound changes on `branching/02` (4→2), `06` (22→20), `08` (8→6) |
| `:enable-forced-positive false` vs default | 5 entries use the cascade; **0** verdict changes |

---

## 5. Fixed, for the record (verified today)

| what | fixed by | today |
|---|---|---|
| CO-M1 — record sites recorded a state their own rules refute (`alive-empty-phase1`, `alive-empty-interlayer`, `complete-records-stale`) | S1e.3.1, 2026-08-30 (`record_node` re-saturates) | `Contradiction k=0`, `Contradiction k=0` (core `(p Z)`), `Solution k=1 {(p A),(p B)}` |
| CO-H3 — tree ignored `-n`/`-m`, empty core, root-only rung probe | S1e.2.1, 2026-08-29 | `-n 1` → k=1; `-m 3` refused exit 2 with a reason; dead branches give "refuted so far (2 facts)"; 86 enterings / 32 models |
| CO-H1 / Q-M1e.18 — `(eq ?x)` panic; `(eq ?x A B)`, `(absent a b)` fired on a weaker guard; `(absent)` silent | S1e.2.1 T1 | all `examples/broken/compile/*` exit 1 with a positioned `CompileError` (`activator_arity.ein` exit 0 by design) |
| CO-L1 — 65 536-argument fact panicked (exit 101) | S1e.4.1 | `kb load error: a fact takes at most 65535 arguments`, exit 1 |
| `-m 0` reported `exhausted = true` Contradiction | M1d T1d.10.5.0 | "No model found — the search did not exhaust" |
| truncated `Contradiction` said "the constraints are contradictory" (`pets.ein` at `-m 5…8`) | M1d T1d.10.5.2b | CLI fixed (S2/S3 are the unfixed surfaces) |
| unexhausted `Ambiguity` printed a bare k (`colors.ein` 5 of 9) | M1d S1d.3.3 | "(a lower bound — the search did not exhaust)"; 9 at `-m 8` |
| CO-M2 / SE-M1 — `k` vs `solution_nodes` on three surfaces | S1e.3.4 | `read_out.rs` |
| Q-M1a.8 — "an int activator collides" | closed as stated (S1e.1.4): never existed; live half is W7 | — |
| D3 (lookahead NAF world, `branching/13`), D5 (or-disjunct guards) | S1.21.8 | `branching/13` Solution |
| `disjunctive-prune`'s `neq` guard: `zebra2-minus-15` 55 models vs 32 (a stdlib/puzzle bug) | 2026-08-20 | 32; `zebra2-hints.ein` deliberately frozen on the old rule |
| goal-binding int/Fact, free-head query goal, unbound assert var (M1a S1a.6.6 fuzz) | 2026-08-20 | regression entries |
| `closure/02`+`03` one word for two states | M1d S1d.2.6 | `Open — owes 1` on `03` |

---

## 6. Suspected, not verified

- **Q-M1e.11 / review Q6** — an obligation activator derived inside a fork flips the tree's rung to blind
  (`oblgen.rs:232-262` decline conditions). No witness has been built (S1f.10.6 not run). The shipped guard
  (`solve.rs:1040-1080`) narrates `traversal … declined` and stops descending, so the count is a lower bound —
  a loss would be hedged, not certified.
- **`hrule-reads-not.ein`** — release answers `Solution k=1` "(no query goal to project)"; debug asserts. Nobody
  has said which answer is right (`corpus/fuzz_findings/hrule-reads-not.md`).
- **`emit_closed`** (M1d #2/#3) — closes `nation-loc` on evidence later invalidated; it only feeds `-H`/JSON
  (S4), and a manual `(__closed__ nation-loc)` leaves the model set identical. Ordering bug or sound
  generalisation: undetermined.
- **Inter-layer cascade `term`** (`solve.rs:1828-1836`) — W2's mechanism between layers; not constructed (nodes
  recorded earlier survive `root_dead`, so the visible effect would need no prior node).
- **Non-stratified programs** (`absent_semantics.md` P4) — "the engine reports one model where several exist and
  does not say so", documented as *not provided*; W8 is the hypothesis-layer cousin.

## 7. Excluded, with reason

- **Q-M1e.13** (`:expect` cannot state an empty goal extent) — a language limitation that blocks banking W1/W4
  fixtures against a fix; no answer is wrong.
- **Q-M1e.22** (failed artefact write exits 0) — a written contract (`defined_behaviour.md` §4.4), stderr says so.
- **Q-M1e.10** (inert `print-alive`, `candidate-order-seed`) — echoed in `--dump-config`/JSON, change no answer.
- **Q-M1e.14, Q-M1e.19, Q-M1e.20** — coverage / pathology / reachability, not answers.
- **Unpayable obligation → `Open`** (`tree-dead.ein`, `closure/03`) — the user's 2026-08-25 ruling.
- **`features/12` blind pigeonhole is Ambiguity k=6** — open world, by design (its own header).
- **`zebra2-hints.ein` Contradiction** — the frozen program's own wrong rule, kept on purpose.
- **Activator of the wrong arity silently activates nothing** (`broken/compile/activator_arity.ein`) — defined.
- **The tree omits `enter`/`layer`/`nogood`/`writeback` events** — an omission, documented (SE-M3).

## 8. Stale statements met on the way (docs/fixtures, not engine)

- `examples/README.md` `ein-bugs/` row and `corpus/corpus.toml` notes for `alive-empty-phase1` (≈l.439),
  `alive-empty-interlayer` (≈l.445), `complete-records-stale` (l.469: "Carries **no** `:expect`") still describe
  the pre-S1e.3.1 behaviour.
- `corpus/corpus.toml` `closure/03` note: "Solution, both … S1d.2.6 is the stage that would move it" (it did).
- `docs/kernel/inference/solution_semantics.md` §6 "What is recorded is `S ∪ K` … nothing re-runs saturation
  after they land" — false since S1e.3.1.
- `docs/history/m1d_satisfiability/completeness.md` §3c "The tree agrees with it … the same seatings" (W5).
- `corpus/fuzz_findings/README.md`: `d3-unsat-core` "green under a permuted id space" (W11).
- `examples/ein-bugs/alive-set-fresh-name.ein` header attributes `k = 0` to ST-M1 alone (W4 merge note).
- `AGENTS.md`: "`Contradiction` is deliberately untouched … `pets.ein` … is the fixture that now waits for it" —
  the CLI word was fixed by T1d.10.5.2b; the trace (S2) and JSON (S3) were not.
- D8 / Q-M1e.12: "20 of the 22 models bind `?h` to `Color` or `House`" (today: 22/22 contain a type, 10/22 bind it).
- `ein-infer/src/apriori.rs:138-139` (W5) and `architecture_and_algorithms.md` "forced-positive cascade ≈ unit
  propagation" (W2) state premises that are false.

## 9. Probe files (all under `$S`)

`forced-1.ein`, `forced-1-two.ein`, `forced-2.ein`, `forced-2-two.ein`, `forced-2-off.ein` (W1/W2);
`owed-two-step.ein`, `tree-dead.ein` (W1 Open vs Contradiction); `owed-seats.ein`, `owed-seats-renamed.ein` (W5);
`uncovered.ein`, `uncovered-knows.ein`, `arity3.ein` (W6/S4); `binding-key-{both,swapped,nested-only,int-only}.ein`
(W7); `stale-naf-fork.ein`, `stale-naf-fork-warn.ein`, `f07-fpoff.ein` (W8); `fresh-name-default.ein`,
`fresh-name-nospawn.ein` (W4); `naf-nowb.ein` (W3); `tree-owed.ein` (S1); `nested-core.ein` (S8).
