# The read-out in real runs

**What this page is.** The examples for [`read_out.md`](read_out.md), the read-out reference: every block below is one
run of the binary, written by [`utils/read_out_examples.py`](../../../utils/read_out_examples.py), stdout, then each
stderr line as `stderr: …`, then the exit code. A block captioned `--dump-answer DIR` is the `DIR/answer.txt` that run
writes: the read-out with no listing cut. [`ein-cli/tests/read_out_examples.rs`](../../../ein.rs/crates/ein-cli/tests/read_out_examples.rs)
runs every caption again and compares the block, so the page is the read-out's golden, and
`ein-cli/tests/read_out_reference.rs` holds that every row label the printer has is printed by one of the blocks. One
block is not one whole run: X4's scratch variant, whose query block is quoted. Re-write it with
`python3 utils/read_out_examples.py` in the commit that changes what the read-out prints.

**Since M1f S1f.5.16** (2026-11-09, `79fc14b0`). Until then the page was M1e2's, written for the user's rulings of 2026-10-16; that
page is frozen where it was, beside [`read_out.md`'s record](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/read_out.md), and no test reads it.


---

## Contents

| | program | n | what it shows |
|---|---|---:|---|
| [X1](#x1--branching05_mini_zebraein-everything-at-once) | `branching/05_mini_zebra.ein` | 9 | the default read-out, every row; `(merged …)` with `(or …)`; a query with an undetermined class |
| [X2](#x2--lattice02_genuine_3set_deathein-one-program-every-mode) | `lattice/02_genuine_3set_death.ein` | 3 | one program under every question: describe, `--maximal`, `--minimal`, `-n 2`, `--single`, `--maximal --single`, `--count`, `--monotone`, and `-y` |
| [X3](#x3--features11_expect_ambiguityein-a-query-over-the-maximal-line) | `features/11_expect_ambiguity.ein` | 4 | a query `:over maximal`, and its claim holding |
| [X4](#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes) | `lattice/07_two_pairs_two_tables.ein` | 16 | 8 components, and 2 projection classes of 4 |
| [X5](#x5--latticesynthmerged_statesein-every-state-merged) | `lattice/synth/merged_states.ein` | 5 | ∅ not in R, and what it owes; a `(merged …)` on every state; two components |
| [X6](#x6--testsregion04_owingein-r-empty-alive-nodes-owe) | `tests/region/04_owing.ein` | 0 | empty R, the owing kind; `--single` says `Open` |
| [X7](#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) | `tests/stdlib/algebra/09_irreflexive_violated.ein` | 0 | empty R, the dead kind; `--single` says `Contradiction` |
| [X8](#x8--lattice04_obligation_trigger_holeein-a-hole) | `lattice/04_obligation_trigger_hole.ein` | 2 | a program the static check flags: route 2, a hole, the warning |
| [X9](#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) | `lattice/06_friends_tables.ein` | 19 | flagged and past `--region-limit`: the line members describe certifies, under describe and `--maximal`; `--minimal` decided at root, ∅ being in R; `--count` |
| [X10](#x10--zebra2ein-the-puzzle) | `zebra2.ein` | 56 | the puzzle: cut on stdout and whole in `answer.txt`, its region and the 36 instances root owes; `--maximal -n 1`; `--single` |
| [X11](#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) | `saturation/square-bwd/floors.ein` | 20 | R is the whole cube: decided, its states counted in part; the first questions in 1–11 saturations |
| [X12](#x12--testsregion03_merged_orein-a-program-that-states-its-region) | `tests/region/03_merged_or.ein` | 3 | a program that states its region: the printed region is its claim |
| [X13](#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines) | `saturation/type-exclusivity/colors.ein` | 8 | an exhausted R past stdout's 100 lines: what fits, and where the rest is |
| [X14](#x14--features07_unstratifiableein-every-row-of-the-warning-and-the-strata) | `features/07_unstratifiable.ein` | 1 | every row of the warning, the strata, and `--monotone` on a flagged program |
| [X15](#x15--features01_not_and_absentein-the-warning-past-four-rows) | `features/01_not_and_absent.ein` | 35 | the warning past four stale nodes |
| [X16](#x16--ein-bugszebra2-badein-a-dead-roots-core-its-sources-and-a-cut-search) | `ein-bugs/zebra2-bad.ein`, `ein-bugs/wrong-answers/core-search-cut.ein` | 0 | why root is dead: its core, a fact's `:source`, and a core search cut at its budget |
| [X17](#x17--ein-oracle-the-cubes-line) | `lattice/04_obligation_trigger_hole.ein`, `lattice/02_genuine_3set_death.ein` | 2, 3 | `ein oracle`: the read-out over the cube, and the cube's line |

## X1 — `branching/05_mini_zebra.ein`: everything at once

[`region_examples.md`](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_examples.md)'s E1. Two people, two drinks and two pets; the clues put Alice
with the tea and the cat, and Bob is open. Every row of the default read-out is here. ∅ is in R, root being alive and
owing nothing (the program states no obligation), so the minimal line is {∅}. The maximal state is reached by four commitments, three
of them minimal, and that is the one `(merged …)`. The query asks who drinks the coffee and which pet they keep: on
four of the five states the goal has no row, and that class is `(model)`. H leaves `is-a` out, and the `outside` row
says what R holds of it.

`ein solve examples/branching/05_mini_zebra.ein`

```text
solve · examples/branching/05_mini_zebra.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      5
  commitments (k_c) 8
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      1 state
  components        1
  sub-cubes         1, of dimension 3
  merged            1 state reached by more than one commitment, with 3 minimal
  N                 6 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          9 candidates, 6 dead alone
    outside         is-a — no candidate: no hrule proposes it. R holds of it only what the program states and saturation derives
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              13 saturations · 60 facts beyond root · 4 layers

  (region
    ;; state 1 of 5 — ∅, root itself · minimal
    (model (co-located Alice Cat) (co-located Alice Tea)
           (co-located Cat Alice) (co-located Cat Tea) (co-located Tea Alice)
           (co-located Tea Cat))
    ;; state 2 of 5 — 1 candidate · from {(co-located Bob Coffee)}
    (model (co-located Alice Cat) (co-located Alice Tea)
           (co-located Bob Coffee) (co-located Cat Alice) (co-located Cat Tea)
           (co-located Coffee Bob) (co-located Tea Alice) (co-located Tea Cat))
    ;; state 3 of 5 — 1 candidate · from {(co-located Bob Dog)}
    (model (co-located Alice Cat) (co-located Alice Tea) (co-located Bob Dog)
           (co-located Cat Alice) (co-located Cat Tea) (co-located Dog Bob)
           (co-located Tea Alice) (co-located Tea Cat))
    ;; state 4 of 5 — 1 candidate · from {(co-located Coffee Dog)}
    (model (co-located Alice Cat) (co-located Alice Tea)
           (co-located Cat Alice) (co-located Cat Tea) (co-located Coffee Dog)
           (co-located Dog Coffee) (co-located Tea Alice) (co-located Tea Cat))
    ;; state 5 of 5 — 3 candidates · maximal · 4 commitments, 3 minimal
    (model (co-located Alice Cat) (co-located Alice Tea)
           (co-located Bob Coffee) (co-located Bob Dog) (co-located Cat Alice)
           (co-located Cat Tea) (co-located Coffee Bob)
           (co-located Coffee Dog) (co-located Dog Bob)
           (co-located Dog Coffee) (co-located Tea Alice) (co-located Tea Cat))
    ;; state 5 is reached by 4 commitments; its 3 minimal
    (merged (model (co-located Alice Cat) (co-located Alice Tea)
                   (co-located Bob Coffee) (co-located Bob Dog)
                   (co-located Cat Alice) (co-located Cat Tea)
                   (co-located Coffee Bob) (co-located Coffee Dog)
                   (co-located Dog Bob) (co-located Dog Coffee)
                   (co-located Tea Alice) (co-located Tea Cat))
            (or (model (co-located Bob Coffee) (co-located Bob Dog))
                (model (co-located Bob Coffee) (co-located Coffee Dog))
                (model (co-located Bob Dog) (co-located Coffee Dog)))))

  query             (and (co-located ?n Coffee) (co-located ?n ?p) (is-a ?p Pet))
  over              all of R — 5 states → 2 projection classes
    class 1 of 2    4 states (1, 2, 3, 4) — the goal has no row: undetermined here
    class 2 of 2    1 state (5)
                      ?n = Bob · ?p = Dog
  as :expect        (or (model)
                        (model (co-located Bob Coffee) (co-located Bob Dog)
                               (is-a Dog Pet)))

exit 0
```

## X2 — `lattice/02_genuine_3set_death.ein`: one program, every mode

[`solution_semantics.md` § 2.5](solution_semantics.md#25-worked-exampleslattice02_genuine_3set_deathein)'s worked
example: three candidates, every pair alive, the triple dead, no obligation. One run per question, so the same R can be
read through each.

**Describe.** Seven states, the three pairs on the maximal line, three sub-cubes of dimension 2.

`ein solve examples/lattice/02_genuine_3set_death.ein`

```text
solve · examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      7
  commitments (k_c) 7
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      3 states
  components        1
  sub-cubes         3, each of dimension 2
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 1 minimal dead set
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              8 saturations · 13 facts beyond root · 4 layers

  (region
    ;; state 1 of 7 — ∅, root itself · minimal
    (model)
    ;; state 2 of 7 — 1 candidate · from {(a-prop X)}
    (model (a-prop X))
    ;; state 3 of 7 — 1 candidate · from {(b-prop X)}
    (model (b-prop X))
    ;; state 4 of 7 — 1 candidate · from {(c-prop X)}
    (model (c-prop X))
    ;; state 5 of 7 — 2 candidates · maximal · from {(a-prop X), (b-prop X)}
    (model (a-prop X) (b-prop X))
    ;; state 6 of 7 — 2 candidates · maximal · from {(a-prop X), (c-prop X)}
    (model (a-prop X) (c-prop X))
    ;; state 7 of 7 — 2 candidates · maximal · from {(b-prop X), (c-prop X)}
    (model (b-prop X) (c-prop X)))

  query             (a-prop ?x)
  over              all of R — 7 states → 2 projection classes
    class 1 of 2    4 states (1, 3, 4, 7) — the goal has no row: undetermined here
    class 2 of 2    3 states (2, 5, 6)
                      ?x = X
  as :expect        (or (model)
                        (model (a-prop X)))

exit 0
```

**`--maximal`.** The line is listed as states, not as a `(region …)`: a line is not a claim the language can state
(only a query's `:over` names one), and a `(region …)` read back would say that these three are all of R.

`ein solve --maximal examples/lattice/02_genuine_3set_death.ein`

```text
solve · --maximal examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             the maximal line — every state on it
  maximal line      3 states — all of them
  exhausted         yes — this is all of the maximal line
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              7 saturations · 12 facts beyond root

  ;; state 1 of 3 — 2 candidates · maximal · from {(a-prop X), (b-prop X)}
  (model (a-prop X) (b-prop X))
  ;; state 2 of 3 — 2 candidates · maximal · from {(a-prop X), (c-prop X)}
  (model (a-prop X) (c-prop X))
  ;; state 3 of 3 — 2 candidates · maximal · from {(b-prop X), (c-prop X)}
  (model (b-prop X) (c-prop X))

  query             (a-prop ?x)
  over              the 3 states listed — 3 states → 2 projection classes  (a lower bound)
    class 1 of 2    1 state (3) — the goal has no row: undetermined here
    class 2 of 2    2 states (1, 2)
                      ?x = X
  as :expect        not printed: these classes are over the states listed, and the query asks about all of R

exit 0
```

**`--minimal`.** ∅ is in R, so the line is {∅}: one saturation.

`ein solve --minimal examples/lattice/02_genuine_3set_death.ein`

```text
solve · --minimal examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             the minimal line — every state on it
  minimal line      1 state — all of them
  exhausted         yes — this is all of the minimal line
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed
  work              1 saturation · 0 facts beyond root

  ;; state 1 of 1 — ∅, root itself · minimal
  (model)

  query             (a-prop ?x)
  over              the 1 state listed — 1 state → 1 projection class  (a lower bound)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        not printed: these classes are over the states listed, and the query asks about all of R

exit 0
```

**`-n 2`.** The first two states in the route's order, the same two under every lever. Nothing is said about the
rest, and the exit code is 0: a request, not an expectation.

`ein solve -n 2 examples/lattice/02_genuine_3set_death.ein`

```text
solve · -n 2 examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             2 states of R (-n 2)
  states of R       2 states, as asked — R may hold more
  exhausted         no — stopped as asked; each state shown is certified, nothing is said of the rest
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's
  work              2 saturations · 1 fact beyond root

  ;; state 1 of 2 — ∅, root itself
  (model)
  ;; state 2 of 2 — 1 candidate · from {(a-prop X)}
  (model (a-prop X))

  query             (a-prop ?x)
  over              all of R — 2 states → 2 projection classes  (a lower bound)
    class 1 of 2    1 state — the goal has no row: undetermined here
    class 2 of 2    1 state
                      ?x = X
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

**`--single`.** The run's own expectation, so the verdict word is printed: `Ambiguity` after two saturations. It
exits 0, as every answer does.

`ein solve --single examples/lattice/02_genuine_3set_death.ein`

```text
solve · --single examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Ambiguity — R holds two states or more; two of them below
  decided           yes — two states settle it; the rest of R was not visited
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed
  work              2 saturations · 1 fact beyond root

  ;; state 1 of 2 — ∅, root itself
  (model)
  ;; state 2 of 2 — 1 candidate · from {(a-prop X)}
  (model (a-prop X))

  query             (a-prop ?x)
  over              all of R — 2 states → 2 projection classes  (a lower bound)
    class 1 of 2    1 state — the goal has no row: undetermined here
    class 2 of 2    1 state
                      ?x = X
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

**`--maximal --single`.** The question a restrictions-only program means by *unique*: the maximal line has three
states, so `Ambiguity` again, with two of them.

`ein solve --maximal --single examples/lattice/02_genuine_3set_death.ein`

```text
solve · --maximal --single examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             is the maximal line one state? (--single)
  verdict           Ambiguity — the maximal line holds two states or more; two of them below
  decided           yes — two states settle it; the rest of the maximal line was not visited
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              6 saturations · 10 facts beyond root

  ;; state 1 of 2 — 2 candidates · maximal · from {(a-prop X), (b-prop X)}
  (model (a-prop X) (b-prop X))
  ;; state 2 of 2 — 2 candidates · maximal · from {(a-prop X), (c-prop X)}
  (model (a-prop X) (c-prop X))

  query             (a-prop ?x)
  over              the 2 states listed — 2 states → 1 projection class  (a lower bound)
    class 1 of 1    2 states
                      ?x = X
  as :expect        not printed: these classes are over the states listed, and the query asks about all of R

exit 0
```

**`--count`.** *How many?*, and nothing else: describe's route and its two counts, with the rows that qualify them,
each describe's text. No state is read, so nothing is listed and the query is not answered. Over all of R only: beside
`--minimal` or `--maximal` it is a usage error that names a bare `ein solve`.

`ein solve --count examples/lattice/02_genuine_3set_death.ein`

```text
solve · --count examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             how many states and commitments R holds (--count)
  states (k_s)      7
  commitments (k_c) 7
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              8 saturations · 13 facts beyond root · 4 layers

exit 0
```

**`--monotone`.** A question about the program: root's fixpoint and the static check, no node built. The program
is monotone, so there is no warning.

`ein solve --monotone examples/lattice/02_genuine_3set_death.ein`

```text
solve · --monotone examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             is the program monotone? — the static check, on root's fixpoint
  monotone          yes — no site: dead and discharged are upward-closed, so R has no hole and its two lines describe it
  universe          3 candidates
  route             none — a question about the program: root's fixpoint, then the static check
  work              1 saturation, root's · 0 facts beyond root

exit 0
```

**`-y`.** The parent-path spot check, every one-step parent of every printed state: under a monotone program's
`lattice` row it prints its `paths` row, which the theorem says must find nothing. It moves no other row.

`ein solve -y examples/lattice/02_genuine_3set_death.ein`

```text
solve · examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      7
  commitments (k_c) 7
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      3 states
  components        1
  sub-cubes         3, each of dimension 2
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 1 minimal dead set
  lattice           monotone (the static check) — R is convex: no hole
    paths           every parent path checked reaches the same state — 6 parents of 3 printed states (-y: every one-step parent)
  universe          3 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              8 saturations · 13 facts beyond root · 4 layers

  (region
    ;; state 1 of 7 — ∅, root itself · minimal
    (model)
    ;; state 2 of 7 — 1 candidate · from {(a-prop X)}
    (model (a-prop X))
    ;; state 3 of 7 — 1 candidate · from {(b-prop X)}
    (model (b-prop X))
    ;; state 4 of 7 — 1 candidate · from {(c-prop X)}
    (model (c-prop X))
    ;; state 5 of 7 — 2 candidates · maximal · from {(a-prop X), (b-prop X)}
    (model (a-prop X) (b-prop X))
    ;; state 6 of 7 — 2 candidates · maximal · from {(a-prop X), (c-prop X)}
    (model (a-prop X) (c-prop X))
    ;; state 7 of 7 — 2 candidates · maximal · from {(b-prop X), (c-prop X)}
    (model (b-prop X) (c-prop X)))

  query             (a-prop ?x)
  over              all of R — 7 states → 2 projection classes
    class 1 of 2    4 states (1, 3, 4, 7) — the goal has no row: undetermined here
    class 2 of 2    3 states (2, 5, 6)
                      ?x = X
  as :expect        (or (model)
                        (model (a-prop X)))

exit 0
```

## X3 — `features/11_expect_ambiguity.ein`: a query over the maximal line

[`region_examples.md`](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_examples.md)'s E5. Two people, two seats, restrictions only. The query is
`:over maximal`: over all of R the goal `(seat ?who ?slot)` would see every seat fact, so each state would be its own
class and ∅ the undetermined one. Over the maximal line its answer is two classes, and its claim holds
([`region_language.md` § 7](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_language.md#7-the-migration-named) migrated it).

`ein solve examples/features/11_expect_ambiguity.ein`

```text
solve · examples/features/11_expect_ambiguity.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      7
  commitments (k_c) 7
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      2 states
  components        1
  sub-cubes         2, each of dimension 2
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 4 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          4 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              11 saturations · 20 facts beyond root · 4 layers

  (region
    ;; state 1 of 7 — ∅, root itself · minimal
    (model)
    ;; state 2 of 7 — 1 candidate · from {(seat Ann S1)}
    (model (seat Ann S1))
    ;; state 3 of 7 — 1 candidate · from {(seat Ann S2)}
    (model (seat Ann S2))
    ;; state 4 of 7 — 1 candidate · from {(seat Bob S1)}
    (model (seat Bob S1))
    ;; state 5 of 7 — 1 candidate · from {(seat Bob S2)}
    (model (seat Bob S2))
    ;; state 6 of 7 — 2 candidates · maximal · from {(seat Ann S1), (seat Bob S2)}
    (model (seat Ann S1) (seat Bob S2))
    ;; state 7 of 7 — 2 candidates · maximal · from {(seat Ann S2), (seat Bob S1)}
    (model (seat Ann S2) (seat Bob S1)))

  query             (seat ?who ?slot)
  over              the maximal line — 2 states → 2 projection classes
    class 1 of 2    1 state (6)
                      ?slot = S1 · ?who = Ann
                      ?slot = S2 · ?who = Bob
    class 2 of 2    1 state (7)
                      ?slot = S1 · ?who = Bob
                      ?slot = S2 · ?who = Ann
  as :expect        (or (model (seat Ann S1) (seat Bob S2))
                        (model (seat Ann S2) (seat Bob S1)))
  :expect           holds

exit 0
```

## X4 — `lattice/07_two_pairs_two_tables.ein`: components and projection classes

Two pairs of friends, two tables of two seats, friends sit together. R is the eight full seatings. Each is its own
**component**, because moving one friend passes through a node that is dead or owing, and the minimal and maximal
lines coincide. The goal `(sits ?p ?s)` names the seats, so each state is its own projection class.

`ein solve examples/lattice/07_two_pairs_two_tables.ein`

```text
solve · examples/lattice/07_two_pairs_two_tables.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      8
  commitments (k_c) 8
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 4 instances
    owed            sits owes Ann a Seat.
    owed            sits owes Bob a Seat.
    owed            sits owes Cat a Seat.
    owed            sits owes Dan a Seat.
  minimal line      8 states, from 8 commitments — the least guess is 4 candidates
  maximal line      8 states
  components        8, each a single commitment
  sub-cubes         8, each of dimension 0
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 64 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          16 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              177 saturations · 448 facts beyond root · 6 layers

  (region
    ;; state 1 of 8 — 4 candidates · minimal and maximal · from {(sits Ann S1), (sits Bob S2), (sits Cat S3), (sits Dan S4)}
    (model (sits Ann S1) (sits Bob S2) (sits Cat S3) (sits Dan S4))
    ;; state 2 of 8 — 4 candidates · minimal and maximal · from {(sits Ann S1), (sits Bob S2), (sits Cat S4), (sits Dan S3)}
    (model (sits Ann S1) (sits Bob S2) (sits Cat S4) (sits Dan S3))
    ;; state 3 of 8 — 4 candidates · minimal and maximal · from {(sits Ann S2), (sits Bob S1), (sits Cat S3), (sits Dan S4)}
    (model (sits Ann S2) (sits Bob S1) (sits Cat S3) (sits Dan S4))
    ;; state 4 of 8 — 4 candidates · minimal and maximal · from {(sits Ann S2), (sits Bob S1), (sits Cat S4), (sits Dan S3)}
    (model (sits Ann S2) (sits Bob S1) (sits Cat S4) (sits Dan S3))
    ;; state 5 of 8 — 4 candidates · minimal and maximal · from {(sits Ann S3), (sits Bob S4), (sits Cat S1), (sits Dan S2)}
    (model (sits Ann S3) (sits Bob S4) (sits Cat S1) (sits Dan S2))
    ;; state 6 of 8 — 4 candidates · minimal and maximal · from {(sits Ann S3), (sits Bob S4), (sits Cat S2), (sits Dan S1)}
    (model (sits Ann S3) (sits Bob S4) (sits Cat S2) (sits Dan S1))
    ;; state 7 of 8 — 4 candidates · minimal and maximal · from {(sits Ann S4), (sits Bob S3), (sits Cat S1), (sits Dan S2)}
    (model (sits Ann S4) (sits Bob S3) (sits Cat S1) (sits Dan S2))
    ;; state 8 of 8 — 4 candidates · minimal and maximal · from {(sits Ann S4), (sits Bob S3), (sits Cat S2), (sits Dan S1)}
    (model (sits Ann S4) (sits Bob S3) (sits Cat S2) (sits Dan S1)))

  query             (sits ?p ?s)
  over              all of R — 8 states → 8 projection classes
    class 1 of 8    1 state (1)
                      ?p = Ann · ?s = S1
                      ?p = Bob · ?s = S2
                      ?p = Cat · ?s = S3
                      ?p = Dan · ?s = S4
    class 2 of 8    1 state (2)
                      ?p = Ann · ?s = S1
                      ?p = Bob · ?s = S2
                      ?p = Cat · ?s = S4
                      ?p = Dan · ?s = S3
    class 3 of 8    1 state (3)
                      ?p = Ann · ?s = S2
                      ?p = Bob · ?s = S1
                      ?p = Cat · ?s = S3
                      ?p = Dan · ?s = S4
    class 4 of 8    1 state (4)
                      ?p = Ann · ?s = S2
                      ?p = Bob · ?s = S1
                      ?p = Cat · ?s = S4
                      ?p = Dan · ?s = S3
    class 5 of 8    1 state (5)
                      ?p = Ann · ?s = S3
                      ?p = Bob · ?s = S4
                      ?p = Cat · ?s = S1
                      ?p = Dan · ?s = S2
    class 6 of 8    1 state (6)
                      ?p = Ann · ?s = S3
                      ?p = Bob · ?s = S4
                      ?p = Cat · ?s = S2
                      ?p = Dan · ?s = S1
    class 7 of 8    1 state (7)
                      ?p = Ann · ?s = S4
                      ?p = Bob · ?s = S3
                      ?p = Cat · ?s = S1
                      ?p = Dan · ?s = S2
    class 8 of 8    1 state (8)
                      ?p = Ann · ?s = S4
                      ?p = Bob · ?s = S3
                      ?p = Cat · ?s = S2
                      ?p = Dan · ?s = S1
  as :expect        (or (model (sits Ann S1) (sits Bob S2) (sits Cat S3)
                               (sits Dan S4))
                        (model (sits Ann S1) (sits Bob S2) (sits Cat S4)
                               (sits Dan S3))
                        (model (sits Ann S2) (sits Bob S1) (sits Cat S3)
                               (sits Dan S4))
                        (model (sits Ann S2) (sits Bob S1) (sits Cat S4)
                               (sits Dan S3))
                        (model (sits Ann S3) (sits Bob S4) (sits Cat S1)
                               (sits Dan S2))
                        (model (sits Ann S3) (sits Bob S4) (sits Cat S2)
                               (sits Dan S1))
                        (model (sits Ann S4) (sits Bob S3) (sits Cat S1)
                               (sits Dan S2))
                        (model (sits Ann S4) (sits Bob S3) (sits Cat S2)
                               (sits Dan S1)))

exit 0
```

A goal that hides the seat needs a relation that does not carry it: a projection class is the goal's **atoms**
under its bindings. The program below is 07 with one relation and one rule added, and it is **not a corpus file**:

```lisp
(relation pair-at Group Table :why "pair {?1} sits at table {?2}")

(rule pair-at ()
  :match  (and (member ?a ?g) (sits ?a ?s) (at ?s ?t))
  :assert (pair-at ?g ?t)
  :why    "{?a} of pair {?g} sits in {?s}, a seat at table {?t}")

(query :goal (pair-at ?g ?t))
```

R does not move (the rule derives no candidate), and the query sees two classes of four: which pair took which table.

`ein solve 07_two_pairs_two_tables+pair-at.ein`, the query block

```text
  query             (pair-at ?g ?t)
  over              all of R — 8 states → 2 projection classes
    class 1 of 2    4 states (1, 2, 3, 4)
                      ?g = P1 · ?t = T1
                      ?g = P2 · ?t = T2
    class 2 of 2    4 states (5, 6, 7, 8)
                      ?g = P1 · ?t = T2
                      ?g = P2 · ?t = T1
  as :expect        (or (model (pair-at P1 T1) (pair-at P2 T2))
                        (model (pair-at P1 T2) (pair-at P2 T1)))

exit 0
```

## X5 — `lattice/synth/merged_states.ein`: every state merged

*A brings B* and *C brings D*; an obligation needs A or C, and A with C is dead. ∅ owes, so it is not in R, and the
`owed` row under `∅` names the one instance it owes, through the obligation rule's `:why` (M1f S1f.6.26). The
minimal line is two states, each reached from one candidate. Every state is reached by exactly two commitments, its
minimal one and that one plus what it brings, so every state gets a `(merged …)`: k_c = 16 against k_s = 8. The
A-side and the C-side never meet, which is the two components.

`ein solve examples/lattice/synth/merged_states.ein`

```text
solve · examples/lattice/synth/merged_states.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      8
  commitments (k_c) 16
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 1 instance
    owed            from the start: one of A C is required
  minimal line      2 states, from 2 commitments — the least guess is 1 candidate
  maximal line      2 states
  components        2, with 8 + 8 commitments
  sub-cubes         2, each of dimension 3
  merged            8 states reached by more than one commitment, with 8 minimal
  N                 1 minimal dead set
  lattice           monotone (the static check) — R is convex: no hole
  universe          5 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              18 saturations · 46 facts beyond root · 6 layers

  (region
    ;; state 1 of 8 — 2 candidates · minimal · 2 commitments, 1 minimal
    (model (h A) (h B))
    ;; state 2 of 8 — 2 candidates · minimal · 2 commitments, 1 minimal
    (model (h C) (h D))
    ;; state 3 of 8 — 3 candidates · 2 commitments, 1 minimal
    (model (h A) (h B) (h D))
    ;; state 4 of 8 — 3 candidates · 2 commitments, 1 minimal
    (model (h A) (h B) (h E))
    ;; state 5 of 8 — 3 candidates · 2 commitments, 1 minimal
    (model (h B) (h C) (h D))
    ;; state 6 of 8 — 3 candidates · 2 commitments, 1 minimal
    (model (h C) (h D) (h E))
    ;; state 7 of 8 — 4 candidates · maximal · 2 commitments, 1 minimal
    (model (h A) (h B) (h D) (h E))
    ;; state 8 of 8 — 4 candidates · maximal · 2 commitments, 1 minimal
    (model (h B) (h C) (h D) (h E))
    ;; state 1 is reached by 2 commitments; its 1 minimal
    (merged (model (h A) (h B))
            (model (h A)))
    ;; state 2 is reached by 2 commitments; its 1 minimal
    (merged (model (h C) (h D))
            (model (h C)))
    ;; state 3 is reached by 2 commitments; its 1 minimal
    (merged (model (h A) (h B) (h D))
            (model (h A) (h D)))
    ;; state 4 is reached by 2 commitments; its 1 minimal
    (merged (model (h A) (h B) (h E))
            (model (h A) (h E)))
    ;; state 5 is reached by 2 commitments; its 1 minimal
    (merged (model (h B) (h C) (h D))
            (model (h B) (h C)))
    ;; state 6 is reached by 2 commitments; its 1 minimal
    (merged (model (h C) (h D) (h E))
            (model (h C) (h E)))
    ;; state 7 is reached by 2 commitments; its 1 minimal
    (merged (model (h A) (h B) (h D) (h E))
            (model (h A) (h D) (h E)))
    ;; state 8 is reached by 2 commitments; its 1 minimal
    (merged (model (h B) (h C) (h D) (h E))
            (model (h B) (h C) (h E))))

  query             (h ?x)
  over              all of R — 8 states → 8 projection classes
    class 1 of 8    1 state (1)
                      ?x = A
                      ?x = B
    class 2 of 8    1 state (2)
                      ?x = C
                      ?x = D
    class 3 of 8    1 state (3)
                      ?x = A
                      ?x = B
                      ?x = D
    class 4 of 8    1 state (4)
                      ?x = A
                      ?x = B
                      ?x = E
    class 5 of 8    1 state (5)
                      ?x = B
                      ?x = C
                      ?x = D
    class 6 of 8    1 state (6)
                      ?x = C
                      ?x = D
                      ?x = E
    class 7 of 8    1 state (7)
                      ?x = A
                      ?x = B
                      ?x = D
                      ?x = E
    class 8 of 8    1 state (8)
                      ?x = B
                      ?x = C
                      ?x = D
                      ?x = E
  as :expect        (or (model (h A) (h B))
                        (model (h C) (h D))
                        (model (h A) (h B) (h D))
                        (model (h A) (h B) (h E))
                        (model (h B) (h C) (h D))
                        (model (h C) (h D) (h E))
                        (model (h A) (h B) (h D) (h E))
                        (model (h B) (h C) (h D) (h E)))

exit 0
```

## X6 — `tests/region/04_owing.ein`: R empty, alive nodes owe

Root owes one `total-owed` instance, Ann's Food, and both foods are stored as ones she does not like, so the
obligation's two candidates are refuted before they are proposed: no candidate, and R is empty of the second kind.
The `owing` rows are the frontier, the alive owing nodes with no alive node one fact up (here root itself), each with
what it owes through the rule's `:why`. The printed form is `(region (owing (model)))`: it names no fact because the
printer closes the **candidates'** relations, which tell R's states apart, and this program has none. The query's
`over` row says which kind of empty R is: root is alive and owes one instance. (Until M1f S1f.6.34 the example was
`tests/stdlib/closure/03_closed_and_owing.ein`, an obligation on a closed relation, refused at load since.)

`ein solve tests/region/04_owing.ein`

```text
solve · tests/region/04_owing.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every alive node owes — no commitment pays every debt
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 2 layers

  owing             ∅ — owes 1
                      likes owes Ann a Food.

  (region (owing (model)))

  query             (likes ?who ?what)
  over              all of R — R is empty, root is alive and owes 1 instance: no state to answer from
  as :expect        (false)

  (solve :expect)   holds

exit 0
```

**`--single`.** Expected one state, found none of the owing kind: `Open`.

`ein solve --single tests/region/04_owing.ein`

```text
solve · --single tests/region/04_owing.ein
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Open — R is empty: every alive node owes
  decided           yes — R is empty
  lattice           monotone (the static check) — R is convex: no hole
  universe          0 candidates
  route             the obligation tree, over states — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local · L5 an owed instance's alternatives are exhaustive
  work              1 saturation · 0 facts beyond root

  query             (likes ?who ?what)
  over              all of R — R is empty, root is alive and owes 1 instance: no state to answer from
  as :expect        (false)

  (solve :expect)   NOT CHECKED
                    the claim is about all of R, and this run asked whether it is one state (--single)

stderr: note: tests/region/04_owing.ein: (solve :expect) NOT CHECKED — the claim is about all of R, and this run asked whether it is one state (--single)

exit 0
```

## X7 — `tests/stdlib/algebra/09_irreflexive_violated.ein`: R empty, every node dead

Root holds `(loops A A)`, which `irreflexive` refutes. Root's core and the rule that refutes it say why, and the form
is `(false)`. The query's `over` row says that root is dead, where X6's says that it owes: the claim under each is
`(false)`.

`ein solve tests/stdlib/algebra/09_irreflexive_violated.ein`

```text
solve · tests/stdlib/algebra/09_irreflexive_violated.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every node is dead — root's own fixpoint derives (false)
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 1 layer

  unsat core        (loops A A)
  refuted by        irreflexive

  (false)

  query             (loops ?x ?y)
  over              all of R — R is empty, root is dead: no state to answer from
  as :expect        (false)

  (solve :expect)   holds

exit 0
```

**`--single`.** Expected one state, found none of the dead kind: `Contradiction`.

`ein solve --single tests/stdlib/algebra/09_irreflexive_violated.ein`

```text
solve · --single tests/stdlib/algebra/09_irreflexive_violated.ein
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Contradiction — R is empty: every node is dead
  decided           yes — R is empty
  lattice           monotone (the static check) — R is convex: no hole
  universe          0 candidates
  route             1, the alive down-set — L1 dead is upward-closed
  work              1 saturation · 0 facts beyond root

  query             (loops ?x ?y)
  over              all of R — R is empty, root is dead: no state to answer from
  as :expect        (false)

  (solve :expect)   NOT CHECKED
                    the claim is about all of R, and this run asked whether it is one state (--single)

stderr: note: tests/stdlib/algebra/09_irreflexive_violated.ein: (solve :expect) NOT CHECKED — the claim is about all of R, and this run asked whether it is one state (--single)

exit 0
```

## X8 — `lattice/04_obligation_trigger_hole.ein`: a hole

A hole with no `absent` in any saturation rule. `total-owed meal is-a` makes every diner owe a meal, and
`seated-dines` derives `(is-a ?g Diner)` from a seat, so the obligation's trigger grows with the commitment: `{(seats
Ann C1)}` owes, between ∅ and `{(meal Ann Soup), (seats Ann C1)}`, both in R. The static check flags the program and
route 2 takes the whole cube, four saturations. R is `{∅, {meal}, {meal, seats}}`, and `{seats}` is a **hole**: outside
R, between two members of it. The `lattice` block is the detector's warning: what is assumed, the site and what it puts
at stake, the status the cube shows (exercised, by `total-owed`), the state (no stale fact), the parent paths checked,
and then each minimal hole with why it is out. The `E` row is the cube's alone.

`ein solve examples/lattice/04_obligation_trigger_hole.ein`

```text
solve · examples/lattice/04_obligation_trigger_hole.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      3
  commitments (k_c) 3
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      1 state — ⊤, the trivial line: no restriction bites
  components        1
  sub-cubes         2, each of dimension 1
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 0 minimal dead sets
  E                 1 minimal discharging set
  lattice           not monotone: 1 site (the static check) — 1 hole
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            total-owed [activator meal is-a]: obligation-trigger reads is-a at absent depth 0, concludes (open …) — at stake: the status: owes
    status          non-monotone here (exercised, by total-owed [activator meal is-a]) — the cube: 1 hole · 0 dead nodes with an alive superset · 1 discharged node with an owing superset
    state           no stale fact in the cube's 4 nodes
    paths           every parent path checked reaches the same state — 2 parents of 1 printed state
    minimal hole    {(seats Ann C1)} — owes 1
  universe          2 candidates, 0 dead alone
    outside         is-a — no candidate: no hrule proposes it. R holds of it only what the program states and saturation derives
  route             2, the cube — no licence: the static check flags the program
  work              4 saturations · 6 facts beyond root

  (region
    ;; state 1 of 3 — ∅, root itself · minimal
    (model)
    ;; state 2 of 3 — 1 candidate · from {(meal Ann Soup)}
    (model (meal Ann Soup))
    ;; state 3 of 3 — 2 candidates · maximal · from {(meal Ann Soup), (seats Ann C1)}
    (model (meal Ann Soup) (seats Ann C1)))

  query             (meal ?g ?f)
  over              all of R — 3 states → 2 projection classes
    class 1 of 2    1 state (1) — the goal has no row: undetermined here
    class 2 of 2    2 states (2, 3)
                      ?f = Soup · ?g = Ann
  as :expect        (or (model)
                        (model (meal Ann Soup)))

exit 0
```

## X9 — `lattice/06_friends_tables.ein`: flagged, and past the limit

The everyone-sits twin's sibling, where *a group that does not fit goes elsewhere*: the obligation's trigger grows
with the commitment, so the program is not monotone. n = 19 is past `--region-limit 16`, so no route can describe R.
The run visits the cube in layer order up to 2¹⁶ nodes. It meets 21 members, and it cannot know that they are all of
R, so every count is a lower bound. What it *can* certify is one line: ∅ is in R, so the minimal line is exactly {∅},
whatever the program. That is the one state it lists. The other 20 members it met go to `--dump-answer`, and the query
is answered over all 21 as a lower bound, with no `as :expect`: read back, that claim would say these classes are all.
The warning says the cube's half is not known past the limit, reads the stale facts of every node the run computed,
and checks no parent path, because the read-out prints no state of two commitments or more.

`ein solve examples/lattice/06_friends_tables.ein`

```text
solve · examples/lattice/06_friends_tables.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      21   (a lower bound: the run stopped before R was exhausted)
  commitments (k_c) 21   (a lower bound: the run stopped before R was exhausted)
  exhausted         no — not monotone, and the question needs the whole cube: 2^19, past --region-limit 16
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      not known — without a licence it needs the whole cube
  components        not known: R was not exhausted
  sub-cubes         not known
  N                 ≥ 68 minimal dead sets — those met in layer order
  lattice           not monotone: 1 site (the static check) — holes not known: the cube was not enumerated
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            group-seated-owed: obligation-trigger reads sits at absent depth 0, concludes (open …) — at stake: the status: owes
    status          not known — the cube is 2^19 nodes, past --region-limit 16
    state           no stale fact in the 65 299 nodes the run computed
    paths           not checked — the read-out prints no state of two commitments or more; its 21 members met would cost up to 104 saturations
  universe          19 candidates, dead alone not known: the run stopped before it visited each alone
  route             2, the cube — no licence: the static check flags the program
  work              65 299 saturations · 455 247 facts beyond root

  ;; the line members this run certified. Not a (region …): read back, it would claim these are all of R.
  ;; state 1 of 1 — ∅, root itself · minimal
  (model)
  ;; 20 other members met are not certified on a line, and not listed: --dump-answer DIR writes each to DIR/models/

  query             (sits ?p ?s)
  over              the 21 members met — 21 states → 21 projection classes  (a lower bound)
    class 1 of 21   1 state — the goal has no row: undetermined here
    class 2 of 21   1 state
                      ?p = Ann · ?s = B1
                      ?p = Bob · ?s = B2
    class 3 of 21   1 state
                      ?p = Ann · ?s = B1
                      ?p = Bob · ?s = B3
    class 4 of 21   1 state
                      ?p = Ann · ?s = B2
                      ?p = Bob · ?s = B1
    class 5 of 21   1 state
                      ?p = Ann · ?s = B2
                      ?p = Bob · ?s = B3
    class 6 of 21   1 state
                      ?p = Ann · ?s = B3
                      ?p = Bob · ?s = B1
    class 7 of 21   1 state
                      ?p = Ann · ?s = B3
                      ?p = Bob · ?s = B2
    class 8 of 21   1 state
                      ?p = Ann · ?s = S1
                      ?p = Bob · ?s = S2
    class 9 of 21   1 state
                      ?p = Ann · ?s = S2
                      ?p = Bob · ?s = S1
    class 10 of 21  1 state
                      ?p = Cat · ?s = B1
                      ?p = Dan · ?s = B2
                      ?p = Eve · ?s = B3
    class 11 of 21  1 state
                      ?p = Cat · ?s = B1
                      ?p = Dan · ?s = B3
                      ?p = Eve · ?s = B2
    class 12 of 21  1 state
                      ?p = Cat · ?s = B2
                      ?p = Dan · ?s = B1
                      ?p = Eve · ?s = B3
    class 13 of 21  1 state
                      ?p = Cat · ?s = B3
                      ?p = Dan · ?s = B1
                      ?p = Eve · ?s = B2
    class 14 of 21  1 state
                      ?p = Ann · ?s = S1
                      ?p = Bob · ?s = S2
                      ?p = Cat · ?s = B1
                      ?p = Dan · ?s = B2
                      ?p = Eve · ?s = B3
    class 15 of 21  1 state
                      ?p = Ann · ?s = S1
                      ?p = Bob · ?s = S2
                      ?p = Cat · ?s = B1
                      ?p = Dan · ?s = B3
                      ?p = Eve · ?s = B2
    class 16 of 21  1 state
                      ?p = Ann · ?s = S1
                      ?p = Bob · ?s = S2
                      ?p = Cat · ?s = B2
                      ?p = Dan · ?s = B1
                      ?p = Eve · ?s = B3
    class 17 of 21  1 state
                      ?p = Ann · ?s = S1
                      ?p = Bob · ?s = S2
                      ?p = Cat · ?s = B3
                      ?p = Dan · ?s = B1
                      ?p = Eve · ?s = B2
    class 18 of 21  1 state
                      ?p = Ann · ?s = S2
                      ?p = Bob · ?s = S1
                      ?p = Cat · ?s = B1
                      ?p = Dan · ?s = B2
                      ?p = Eve · ?s = B3
    class 19 of 21  1 state
                      ?p = Ann · ?s = S2
                      ?p = Bob · ?s = S1
                      ?p = Cat · ?s = B1
                      ?p = Dan · ?s = B3
                      ?p = Eve · ?s = B2
    class 20 of 21  1 state
                      ?p = Ann · ?s = S2
                      ?p = Bob · ?s = S1
                      ?p = Cat · ?s = B2
                      ?p = Dan · ?s = B1
                      ?p = Eve · ?s = B3
    class 21 of 21  1 state
                      ?p = Ann · ?s = S2
                      ?p = Bob · ?s = S1
                      ?p = Cat · ?s = B3
                      ?p = Dan · ?s = B1
                      ?p = Eve · ?s = B2
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

stderr: note: examples/lattice/06_friends_tables.ein: partial — the question needs 2^19 saturations, past --region-limit 16

exit 0
```

**`--count`** on the same program: the counts are describe's lower bounds, for the same reason, and the warning is
printed whole, since it qualifies them. Its `paths` row says the spot check was not run: `--count` reads no state.

`ein solve --count examples/lattice/06_friends_tables.ein`

```text
solve · --count examples/lattice/06_friends_tables.ein
──────────────────────────────────────────────────────────────
  asked             how many states and commitments R holds (--count)
  states (k_s)      21   (a lower bound: the run stopped before R was exhausted)
  commitments (k_c) 21   (a lower bound: the run stopped before R was exhausted)
  exhausted         no — not monotone, and the question needs the whole cube: 2^19, past --region-limit 16
  ∅                 in R — root is alive and owes nothing
  lattice           not monotone: 1 site (the static check) — holes not known: the cube was not enumerated
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            group-seated-owed: obligation-trigger reads sits at absent depth 0, concludes (open …) — at stake: the status: owes
    status          not known — the cube is 2^19 nodes, past --region-limit 16
    state           no stale fact in the 65 299 nodes the run computed
    paths           not checked — the read-out prints no state of two commitments or more; its 21 members met would cost up to 104 saturations
  universe          19 candidates, dead alone not known: the run stopped before it visited each alone
  route             2, the cube — no licence: the static check flags the program
  work              65 299 saturations · 455 247 facts beyond root

stderr: note: examples/lattice/06_friends_tables.ein: partial — the question needs 2^19 saturations, past --region-limit 16

exit 0
```

**`--minimal`** on the same program stops at root: ∅ is in R, so the minimal line is {∅} and no other state can be
on it, decided after one saturation though the program is flagged and past the limit (M1f S1f.5.6). Until then it
walked the cube to the limit, certified what describe certifies, and counted the other 20 members met.

`ein solve --minimal examples/lattice/06_friends_tables.ein`

```text
solve · --minimal examples/lattice/06_friends_tables.ein
──────────────────────────────────────────────────────────────
  asked             the minimal line — every state on it
  minimal line      1 state — all of them
  exhausted         yes — this is all of the minimal line
  lattice           not monotone: 1 site (the static check) — holes not known: the cube was not enumerated
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            group-seated-owed: obligation-trigger reads sits at absent depth 0, concludes (open …) — at stake: the status: owes
    status          not known — the cube is 2^19 nodes, past --region-limit 16
    state           no stale fact in the 1 node the run computed
    paths           no state the read-out prints has an alive parent to check
  universe          19 candidates
  route             2, the cube — L7 the cube
  work              1 saturation · 0 facts beyond root

  ;; state 1 of 1 — ∅, root itself · minimal
  (model)

  query             (sits ?p ?s)
  over              the 1 state listed — 1 state → 1 projection class  (a lower bound)
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        not printed: these classes are over the states listed, and the query asks about all of R

exit 0
```

**`--maximal`** lists no state: the maximal line needs the whole cube, and the row says so, as describe's does.
The 21 members met are counted and in `--dump-answer`.

`ein solve --maximal examples/lattice/06_friends_tables.ein`

```text
solve · --maximal examples/lattice/06_friends_tables.ein
──────────────────────────────────────────────────────────────
  asked             the maximal line — every state on it
  maximal line      not known — without a licence it needs the whole cube
  exhausted         no — not monotone, and the question needs the whole cube: 2^19, past --region-limit 16
  lattice           not monotone: 1 site (the static check) — holes not known: the cube was not enumerated
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            group-seated-owed: obligation-trigger reads sits at absent depth 0, concludes (open …) — at stake: the status: owes
    status          not known — the cube is 2^19 nodes, past --region-limit 16
    state           no stale fact in the 65 299 nodes the run computed
    paths           not checked — the read-out prints no state of two commitments or more; its 21 members met would cost up to 104 saturations
  universe          19 candidates
  route             2, the cube — L7 the cube
  work              65 299 saturations · 455 247 facts beyond root

  ;; 21 other members met are not certified on a line, and not listed: --dump-answer DIR writes each to DIR/models/

  query             (sits ?p ?s)
  over              the 0 states listed — 0 states → 0 projection classes  (a lower bound)
  as :expect        not printed: these classes are over the states listed, and the query asks about all of R

stderr: note: examples/lattice/06_friends_tables.ein: partial — the question needs 2^19 saturations, past --region-limit 16

exit 0
```

## X10 — `zebra2.ein`: the puzzle

**At the default budget** the state route describes R exhausted. R is one state, 261 982 commitments, 63 minimal
generators, and four of them single facts. Root owes 36 instances, a house for each of 18 values the clues have not
placed and a value for each of 18 places still empty: the `owed` rows name four, and one line counts the other 32. R
as one `(region …)` is past stdout's 100 lines, so stdout lists the state and leaves out the `(merged …)`, and says
where it is. The query's class is exact, so it is printed with its `as :expect`.

`ein solve examples/zebra2.ein`

```text
solve · examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      1
  commitments (k_c) 261 982
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 36 instances
    owed            color-loc owes Red a House.
    owed            color-loc owes Green a House.
    owed            color-loc owes Ivory a House.
    owed            nation-loc owes Englishman a House.
                    … 32 more instances: --dump-answer DIR writes every one, to DIR/answer.txt
  minimal line      1 state, from 63 commitments — the least guess is 1 candidate
  maximal line      1 state
  components        1
  sub-cubes         63, 4 of dimension 17, 59 of dimension 16
  merged            1 state reached by more than one commitment, with 63 minimal
  N                 366 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates, 21 dead alone
    outside         is-a*, next-to — no candidate: no hrule proposes them. R holds of them only what the program states and saturation derives
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              374 saturations · 42 705 facts beyond root · 12 layers

  ;; R as one (region …) is 111 lines, past stdout's 100: what fits is below, not wrapped in a (region …)
  ;; state 1 of 1 — 18 candidates · minimal and maximal · 261 982 commitments, 63 minimal
  (model (color-loc Blue House-2) (color-loc Green House-5)
         (color-loc Ivory House-4) (color-loc Red House-3)
         (color-loc Yellow House-1) (drink-loc Coffee House-5)
         (drink-loc Juice House-4) (drink-loc Milk House-3)
         (drink-loc Tea House-2) (drink-loc Water House-1)
         (nation-loc Englishman House-3) (nation-loc Japanese House-5)
         (nation-loc Norwegian House-1) (nation-loc Spaniard House-4)
         (nation-loc Ukrainian House-2) (pet-loc Dog House-4)
         (pet-loc Fox House-1) (pet-loc Horse House-2)
         (pet-loc Snail House-3) (pet-loc Zebra House-5)
         (smoke-loc Chesterfields House-2) (smoke-loc Kools House-1)
         (smoke-loc Lucky_Strike House-4) (smoke-loc Old_Gold House-3)
         (smoke-loc Parliament House-5))
  ;; not shown: 1 (merged …) — --dump-answer DIR writes the whole (region …) to DIR/answer.txt

  query             (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?h_water = House-1 · ?h_zebra = House-5 · ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”
  as :expect        (model (drink-loc Water House-1)
                           (nation-loc Japanese House-5)
                           (nation-loc Norwegian House-1)
                           (pet-loc Zebra House-5))

exit 0
```

**What `--dump-answer DIR` writes to `DIR/answer.txt`** for the same run: all 36 `owed` rows, and the whole
`(region …)`, whose `(merged …)` lists all 63 minimal commitments. That is what pins k_c, and it reads back as the
claim.

`ein solve --dump-answer DIR examples/zebra2.ein` — `DIR/answer.txt`

```text
solve · examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      1
  commitments (k_c) 261 982
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 36 instances
    owed            color-loc owes Red a House.
    owed            color-loc owes Green a House.
    owed            color-loc owes Ivory a House.
    owed            nation-loc owes Englishman a House.
    owed            nation-loc owes Spaniard a House.
    owed            nation-loc owes Ukrainian a House.
    owed            nation-loc owes Japanese a House.
    owed            drink-loc owes Coffee a House.
    owed            drink-loc owes Tea a House.
    owed            drink-loc owes Juice a House.
    owed            smoke-loc owes Old_Gold a House.
    owed            smoke-loc owes Chesterfields a House.
    owed            smoke-loc owes Lucky_Strike a House.
    owed            smoke-loc owes Parliament a House.
    owed            pet-loc owes Dog a House.
    owed            pet-loc owes Snail a House.
    owed            pet-loc owes Fox a House.
    owed            pet-loc owes Zebra a House.
    owed            color-loc owes House-3 a preimage in Color.
    owed            color-loc owes House-4 a preimage in Color.
    owed            color-loc owes House-5 a preimage in Color.
    owed            nation-loc owes House-2 a preimage in Nationality.
    owed            nation-loc owes House-3 a preimage in Nationality.
    owed            nation-loc owes House-4 a preimage in Nationality.
    owed            nation-loc owes House-5 a preimage in Nationality.
    owed            drink-loc owes House-2 a preimage in Drink.
    owed            drink-loc owes House-4 a preimage in Drink.
    owed            drink-loc owes House-5 a preimage in Drink.
    owed            smoke-loc owes House-2 a preimage in Cigarette.
    owed            smoke-loc owes House-3 a preimage in Cigarette.
    owed            smoke-loc owes House-4 a preimage in Cigarette.
    owed            smoke-loc owes House-5 a preimage in Cigarette.
    owed            pet-loc owes House-1 a preimage in Pet.
    owed            pet-loc owes House-3 a preimage in Pet.
    owed            pet-loc owes House-4 a preimage in Pet.
    owed            pet-loc owes House-5 a preimage in Pet.
  minimal line      1 state, from 63 commitments — the least guess is 1 candidate
  maximal line      1 state
  components        1
  sub-cubes         63, 4 of dimension 17, 59 of dimension 16
  merged            1 state reached by more than one commitment, with 63 minimal
  N                 366 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates, 21 dead alone
    outside         is-a*, next-to — no candidate: no hrule proposes them. R holds of them only what the program states and saturation derives
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              374 saturations · 42 705 facts beyond root · 12 layers

  (region
    ;; state 1 of 1 — 18 candidates · minimal and maximal · 261 982 commitments, 63 minimal
    (model (color-loc Blue House-2) (color-loc Green House-5)
           (color-loc Ivory House-4) (color-loc Red House-3)
           (color-loc Yellow House-1) (drink-loc Coffee House-5)
           (drink-loc Juice House-4) (drink-loc Milk House-3)
           (drink-loc Tea House-2) (drink-loc Water House-1)
           (nation-loc Englishman House-3) (nation-loc Japanese House-5)
           (nation-loc Norwegian House-1) (nation-loc Spaniard House-4)
           (nation-loc Ukrainian House-2) (pet-loc Dog House-4)
           (pet-loc Fox House-1) (pet-loc Horse House-2)
           (pet-loc Snail House-3) (pet-loc Zebra House-5)
           (smoke-loc Chesterfields House-2) (smoke-loc Kools House-1)
           (smoke-loc Lucky_Strike House-4) (smoke-loc Old_Gold House-3)
           (smoke-loc Parliament House-5))
    ;; state 1 is reached by 261 982 commitments; its 63 minimal
    (merged (model (color-loc Blue House-2) (color-loc Green House-5)
                   (color-loc Ivory House-4) (color-loc Red House-3)
                   (color-loc Yellow House-1) (drink-loc Coffee House-5)
                   (drink-loc Juice House-4) (drink-loc Milk House-3)
                   (drink-loc Tea House-2) (drink-loc Water House-1)
                   (nation-loc Englishman House-3)
                   (nation-loc Japanese House-5)
                   (nation-loc Norwegian House-1)
                   (nation-loc Spaniard House-4)
                   (nation-loc Ukrainian House-2) (pet-loc Dog House-4)
                   (pet-loc Fox House-1) (pet-loc Horse House-2)
                   (pet-loc Snail House-3) (pet-loc Zebra House-5)
                   (smoke-loc Chesterfields House-2) (smoke-loc Kools House-1)
                   (smoke-loc Lucky_Strike House-4)
                   (smoke-loc Old_Gold House-3) (smoke-loc Parliament House-5))
            (or (model (color-loc Green House-5) (drink-loc Tea House-2))
                (model (color-loc Green House-5) (nation-loc Spaniard House-4))
                (model (color-loc Green House-5)
                       (nation-loc Ukrainian House-2))
                (model (color-loc Green House-5) (pet-loc Dog House-4))
                (model (color-loc Green House-5) (pet-loc Fox House-1))
                (model (color-loc Green House-5) (pet-loc Zebra House-5))
                (model (color-loc Green House-5)
                       (smoke-loc Chesterfields House-2))
                (model (color-loc Ivory House-4) (drink-loc Tea House-2))
                (model (color-loc Ivory House-4) (nation-loc Spaniard House-4))
                (model (color-loc Ivory House-4)
                       (nation-loc Ukrainian House-2))
                (model (color-loc Ivory House-4) (pet-loc Dog House-4))
                (model (color-loc Ivory House-4) (pet-loc Fox House-1))
                (model (color-loc Ivory House-4) (pet-loc Zebra House-5))
                (model (color-loc Ivory House-4)
                       (smoke-loc Chesterfields House-2))
                (model (color-loc Red House-3) (drink-loc Tea House-2))
                (model (color-loc Red House-3) (nation-loc Spaniard House-4))
                (model (color-loc Red House-3) (nation-loc Ukrainian House-2))
                (model (color-loc Red House-3) (pet-loc Dog House-4))
                (model (color-loc Red House-3) (pet-loc Fox House-1))
                (model (color-loc Red House-3) (pet-loc Zebra House-5))
                (model (color-loc Red House-3)
                       (smoke-loc Chesterfields House-2))
                (model (drink-loc Coffee House-5) (drink-loc Tea House-2))
                (model (drink-loc Coffee House-5)
                       (nation-loc Spaniard House-4))
                (model (drink-loc Coffee House-5)
                       (nation-loc Ukrainian House-2))
                (model (drink-loc Coffee House-5) (pet-loc Dog House-4))
                (model (drink-loc Coffee House-5) (pet-loc Fox House-1))
                (model (drink-loc Coffee House-5) (pet-loc Zebra House-5))
                (model (drink-loc Coffee House-5)
                       (smoke-loc Chesterfields House-2))
                (model (drink-loc Juice House-4))
                (model (drink-loc Tea House-2) (nation-loc Englishman House-3))
                (model (drink-loc Tea House-2) (nation-loc Spaniard House-4))
                (model (drink-loc Tea House-2) (pet-loc Dog House-4))
                (model (drink-loc Tea House-2) (pet-loc Snail House-3))
                (model (drink-loc Tea House-2) (smoke-loc Old_Gold House-3))
                (model (nation-loc Englishman House-3)
                       (nation-loc Spaniard House-4))
                (model (nation-loc Englishman House-3)
                       (nation-loc Ukrainian House-2))
                (model (nation-loc Englishman House-3) (pet-loc Dog House-4))
                (model (nation-loc Englishman House-3) (pet-loc Fox House-1))
                (model (nation-loc Englishman House-3) (pet-loc Zebra House-5))
                (model (nation-loc Englishman House-3)
                       (smoke-loc Chesterfields House-2))
                (model (nation-loc Japanese House-5))
                (model (nation-loc Spaniard House-4)
                       (nation-loc Ukrainian House-2))
                (model (nation-loc Spaniard House-4) (pet-loc Fox House-1))
                (model (nation-loc Spaniard House-4) (pet-loc Snail House-3))
                (model (nation-loc Spaniard House-4) (pet-loc Zebra House-5))
                (model (nation-loc Spaniard House-4)
                       (smoke-loc Chesterfields House-2))
                (model (nation-loc Spaniard House-4)
                       (smoke-loc Old_Gold House-3))
                (model (nation-loc Ukrainian House-2) (pet-loc Dog House-4))
                (model (nation-loc Ukrainian House-2) (pet-loc Snail House-3))
                (model (nation-loc Ukrainian House-2)
                       (smoke-loc Old_Gold House-3))
                (model (pet-loc Dog House-4) (pet-loc Fox House-1))
                (model (pet-loc Dog House-4) (pet-loc Snail House-3))
                (model (pet-loc Dog House-4) (pet-loc Zebra House-5))
                (model (pet-loc Dog House-4) (smoke-loc Chesterfields House-2))
                (model (pet-loc Dog House-4) (smoke-loc Old_Gold House-3))
                (model (pet-loc Fox House-1) (pet-loc Snail House-3))
                (model (pet-loc Fox House-1) (smoke-loc Old_Gold House-3))
                (model (pet-loc Snail House-3) (pet-loc Zebra House-5))
                (model (pet-loc Snail House-3)
                       (smoke-loc Chesterfields House-2))
                (model (pet-loc Zebra House-5) (smoke-loc Old_Gold House-3))
                (model (smoke-loc Chesterfields House-2)
                       (smoke-loc Old_Gold House-3))
                (model (smoke-loc Lucky_Strike House-4))
                (model (smoke-loc Parliament House-5)))))

  query             (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?h_water = House-1 · ?h_zebra = House-5 · ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”
  as :expect        (model (drink-loc Water House-1)
                           (nation-loc Japanese House-5)
                           (nation-loc Norwegian House-1)
                           (pet-loc Zebra House-5))

exit 0
```

**`--maximal -n 1`**: a complete solution, by the obligation tree's first leaf, then the climb.

`ein solve --maximal -n 1 examples/zebra2.ein`

```text
solve · --maximal -n 1 examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             1 state of the maximal line (-n 1)
  maximal line      1 state, as asked — the maximal line may hold more
  exhausted         no — stopped as asked; each state shown is certified, nothing is said of the rest
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates
    outside         is-a*, next-to — no candidate: no hrule proposes them. R holds of them only what the program states and saturation derives
  route             the obligation tree, over states — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local · L5 an owed instance's alternatives are exhaustive
  work              42 saturations · 7 036 facts beyond root

  ;; state 1 of 1 — 18 candidates · maximal · from {(color-loc Green House-5), (color-loc Ivory House-4), (color-loc Red House-3), (drink-loc Coffee House-5), (drink-loc Juice House-4), (drink-loc Tea House-2), (nation-loc Englishman House-3), (nation-loc Japanese House-5), (nation-loc Spaniard House-4), (nation-loc Ukrainian House-2), (pet-loc Dog House-4), (pet-loc Fox House-1), (pet-loc Snail House-3), (pet-loc Zebra House-5), (smoke-loc Chesterfields House-2), (smoke-loc Lucky_Strike House-4), (smoke-loc Old_Gold House-3), (smoke-loc Parliament House-5)}
  (model (color-loc Blue House-2) (color-loc Green House-5)
         (color-loc Ivory House-4) (color-loc Red House-3)
         (color-loc Yellow House-1) (drink-loc Coffee House-5)
         (drink-loc Juice House-4) (drink-loc Milk House-3)
         (drink-loc Tea House-2) (drink-loc Water House-1)
         (nation-loc Englishman House-3) (nation-loc Japanese House-5)
         (nation-loc Norwegian House-1) (nation-loc Spaniard House-4)
         (nation-loc Ukrainian House-2) (pet-loc Dog House-4)
         (pet-loc Fox House-1) (pet-loc Horse House-2) (pet-loc Snail House-3)
         (pet-loc Zebra House-5) (smoke-loc Chesterfields House-2)
         (smoke-loc Kools House-1) (smoke-loc Lucky_Strike House-4)
         (smoke-loc Old_Gold House-3) (smoke-loc Parliament House-5))

  query             (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  over              the 1 state listed — 1 state → 1 projection class  (a lower bound)
    class 1 of 1    1 state
                      ?h_water = House-1 · ?h_zebra = House-5 · ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”
  as :expect        not printed: these classes are over the states listed, and the query asks about all of R

exit 0
```

**`--single`**: `Solution`, by the obligation tree, whose one leaf has every extension dead (L4, L5).

`ein solve --single examples/zebra2.ein`

```text
solve · --single examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates
    outside         is-a*, next-to — no candidate: no hrule proposes them. R holds of them only what the program states and saturation derives
  route             the obligation tree, over states — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local · L5 an owed instance's alternatives are exhaustive
  work              45 saturations · 7 582 facts beyond root

  ;; state 1 of 1 — 18 candidates · from {(color-loc Green House-5), (nation-loc Ukrainian House-2)}
  (model (color-loc Blue House-2) (color-loc Green House-5)
         (color-loc Ivory House-4) (color-loc Red House-3)
         (color-loc Yellow House-1) (drink-loc Coffee House-5)
         (drink-loc Juice House-4) (drink-loc Milk House-3)
         (drink-loc Tea House-2) (drink-loc Water House-1)
         (nation-loc Englishman House-3) (nation-loc Japanese House-5)
         (nation-loc Norwegian House-1) (nation-loc Spaniard House-4)
         (nation-loc Ukrainian House-2) (pet-loc Dog House-4)
         (pet-loc Fox House-1) (pet-loc Horse House-2) (pet-loc Snail House-3)
         (pet-loc Zebra House-5) (smoke-loc Chesterfields House-2)
         (smoke-loc Kools House-1) (smoke-loc Lucky_Strike House-4)
         (smoke-loc Old_Gold House-3) (smoke-loc Parliament House-5))

  query             (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  over              all of R — 1 state → 1 projection class
    class 1 of 1    1 state (1)
                      ?h_water = House-1 · ?h_zebra = House-5 · ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”
  as :expect        (model (drink-loc Water House-1)
                           (nation-loc Japanese House-5)
                           (nation-loc Norwegian House-1)
                           (pet-loc Zebra House-5))

exit 0
```

## X11 — `saturation/square-bwd/floors.ein`: a region that is the whole cube

No restriction and no obligation: every commitment is alive and in R, 2²⁰ of them. **Describe** stops at the default
budget, partial, and R is still decided: ∅ is in R, and one saturation of ⊤ finds it alive and owing nothing, so R is
the whole cube (L1, L2). k_c, both lines, the one component, the one sub-cube and N are exact. What is not is the
states, and the state route counts those of up to eight candidates before the default budget of 2²⁰ facts. The two line
states are what it lists. The other members it met go to `--dump-answer`, not to the screen, and the query's one class
is printed without `as :expect`, since it is a lower bound.

`ein solve examples/saturation/square-bwd/floors.ein`

```text
solve · examples/saturation/square-bwd/floors.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      23 390   (a lower bound: complete up to states of 8 candidates)
  commitments (k_c) 1 048 576
  exhausted         no — R is decided: ∅ and ⊤ are both in it, so it is the whole cube (L1, L2), and only its states are not all counted. The state route stopped before the states of 9 candidates: their 16 853 extensions project 185 383 facts on top of the 994 197 spent, past --region-budget 1048576
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      1 state — ⊤, every candidate: no node is dead, the trivial line, certified by one saturation
  components        1
  sub-cubes         1, of dimension 20
  merged            not known: the run did not meet every state
  N                 0 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          20 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              85 574 saturations · 994 221 facts beyond root · 10 layers

  ;; the line members this run certified. Not a (region …): read back, it would claim these are all of R.
  ;; state 1 of 2 — ∅, root itself · minimal
  (model (above-of Floor2 Floor1) (above-of Floor5 Floor4)
         (co-located Floor1 Floor4) (co-located Floor2 Floor5))
  ;; state 2 of 2 — 20 candidates · maximal
  (model (above-of Floor1 Floor2) (above-of Floor1 Floor4)
         (above-of Floor1 Floor5) (above-of Floor2 Floor1)
         (above-of Floor2 Floor4) (above-of Floor2 Floor5)
         (above-of Floor4 Floor1) (above-of Floor4 Floor2)
         (above-of Floor4 Floor5) (above-of Floor5 Floor1)
         (above-of Floor5 Floor2) (above-of Floor5 Floor4)
         (co-located Floor1 Floor1) (co-located Floor1 Floor2)
         (co-located Floor1 Floor4) (co-located Floor1 Floor5)
         (co-located Floor2 Floor1) (co-located Floor2 Floor2)
         (co-located Floor2 Floor4) (co-located Floor2 Floor5)
         (co-located Floor4 Floor1) (co-located Floor4 Floor2)
         (co-located Floor4 Floor4) (co-located Floor4 Floor5)
         (co-located Floor5 Floor1) (co-located Floor5 Floor2)
         (co-located Floor5 Floor4) (co-located Floor5 Floor5))
  ;; 23 388 other members met are not certified on a line, and not listed: --dump-answer DIR writes each to DIR/models/

  query             (co-located Floor2 Floor5)
  over              all of R — 23 390 states → 1 projection class  (a lower bound)
    class 1 of 1    23 390 states
                      the goal holds: it has no variable
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

stderr: note: examples/saturation/square-bwd/floors.ein: partial — the state route stopped before the states of 9 candidates at --region-budget 1048576

exit 0
```

**The first questions** cost what [`solve_interface.md` § 3.4](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md#34-what-it-costs)
says they do: `-n 1` is root (one saturation), `--single` is `Ambiguity` in two, `--maximal -n 1` is the climb
(eleven). The climb ends at ⊤, so every node is alive (L1) and ⊤ is the whole maximal line, and the run says so.

`ein solve -n 1 examples/saturation/square-bwd/floors.ein`

```text
solve · -n 1 examples/saturation/square-bwd/floors.ein
──────────────────────────────────────────────────────────────
  asked             1 state of R (-n 1)
  states of R       1 state, as asked — R may hold more
  exhausted         no — stopped as asked; each state shown is certified, nothing is said of the rest
  lattice           monotone (the static check) — R is convex: no hole
  universe          20 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's
  work              1 saturation · 0 facts beyond root

  ;; state 1 of 1 — ∅, root itself
  (model (above-of Floor2 Floor1) (above-of Floor5 Floor4)
         (co-located Floor1 Floor4) (co-located Floor2 Floor5))

  query             (co-located Floor2 Floor5)
  over              all of R — 1 state → 1 projection class  (a lower bound)
    class 1 of 1    1 state
                      the goal holds: it has no variable
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

`ein solve --single examples/saturation/square-bwd/floors.ein`

```text
solve · --single examples/saturation/square-bwd/floors.ein
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Ambiguity — R holds two states or more; two of them below
  decided           yes — two states settle it; the rest of R was not visited
  lattice           monotone (the static check) — R is convex: no hole
  universe          20 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed
  work              2 saturations · 1 fact beyond root

  ;; state 1 of 2 — ∅, root itself
  (model (above-of Floor2 Floor1) (above-of Floor5 Floor4)
         (co-located Floor1 Floor4) (co-located Floor2 Floor5))
  ;; state 2 of 2 — 1 candidate · from {(above-of Floor1 Floor2)}
  (model (above-of Floor1 Floor2) (above-of Floor2 Floor1)
         (above-of Floor5 Floor4) (co-located Floor1 Floor4)
         (co-located Floor2 Floor5))

  query             (co-located Floor2 Floor5)
  over              all of R — 2 states → 1 projection class  (a lower bound)
    class 1 of 1    2 states
                      the goal holds: it has no variable
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

`ein solve --maximal -n 1 examples/saturation/square-bwd/floors.ein`

```text
solve · --maximal -n 1 examples/saturation/square-bwd/floors.ein
──────────────────────────────────────────────────────────────
  asked             1 state of the maximal line (-n 1)
  maximal line      1 state — all of them
  exhausted         yes — this is all of the maximal line
  lattice           monotone (the static check) — R is convex: no hole
  universe          20 candidates
  route             the state route, each state at most once — L1 dead is upward-closed · L2 discharged is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              11 saturations · 139 facts beyond root

  ;; state 1 of 1 — 20 candidates · maximal · from {(above-of Floor1 Floor2), (above-of Floor1 Floor4), (above-of Floor1 Floor5), (above-of Floor2 Floor4), (above-of Floor2 Floor5), (above-of Floor4 Floor1), (above-of Floor4 Floor2), (above-of Floor4 Floor5), (above-of Floor5 Floor1), (above-of Floor5 Floor2), (co-located Floor1 Floor2), (co-located Floor1 Floor5), (co-located Floor2 Floor1), (co-located Floor2 Floor4), (co-located Floor4 Floor1), (co-located Floor4 Floor2), (co-located Floor4 Floor5), (co-located Floor5 Floor1), (co-located Floor5 Floor2), (co-located Floor5 Floor4)}
  (model (above-of Floor1 Floor2) (above-of Floor1 Floor4)
         (above-of Floor1 Floor5) (above-of Floor2 Floor1)
         (above-of Floor2 Floor4) (above-of Floor2 Floor5)
         (above-of Floor4 Floor1) (above-of Floor4 Floor2)
         (above-of Floor4 Floor5) (above-of Floor5 Floor1)
         (above-of Floor5 Floor2) (above-of Floor5 Floor4)
         (co-located Floor1 Floor1) (co-located Floor1 Floor2)
         (co-located Floor1 Floor4) (co-located Floor1 Floor5)
         (co-located Floor2 Floor1) (co-located Floor2 Floor2)
         (co-located Floor2 Floor4) (co-located Floor2 Floor5)
         (co-located Floor4 Floor1) (co-located Floor4 Floor2)
         (co-located Floor4 Floor4) (co-located Floor4 Floor5)
         (co-located Floor5 Floor1) (co-located Floor5 Floor2)
         (co-located Floor5 Floor4) (co-located Floor5 Floor5))

  query             (co-located Floor2 Floor5)
  over              the 1 state listed — 1 state → 1 projection class  (a lower bound)
    class 1 of 1    1 state (1)
                      the goal holds: it has no variable
  as :expect        not printed: these classes are over the states listed, and the query asks about all of R

exit 0
```

## X12 — `tests/region/03_merged_or.ein`: a program that states its region

The fixture for `(merged …)` with `(or …)`: the colocation chain, [`region_examples.md`](../../../plans/m1e2_correctness/p1e2.1a_the_solution_region/region_examples.md)'s
E2. Its `(solve … :expect (region …))` is checked against the R the run described, and the `(region …)` printed above
the query is that claim's own text: one form, both directions.

`ein solve tests/region/03_merged_or.ein`

```text
solve · tests/region/03_merged_or.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      4
  commitments (k_c) 8
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      1 state — ⊤, the trivial line: no restriction bites
  components        1
  sub-cubes         1, of dimension 3
  merged            1 state reached by more than one commitment, with 2 minimal
  N                 0 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              7 saturations · 14 facts beyond root · 4 layers

  (region
    ;; state 1 of 4 — ∅, root itself · minimal
    (model (co-located A B) (co-located A C) (co-located B C))
    ;; state 2 of 4 — 1 candidate · from {(co-located B A)}
    (model (co-located A B) (co-located A C) (co-located B A) (co-located B C))
    ;; state 3 of 4 — 1 candidate · from {(co-located C B)}
    (model (co-located A B) (co-located A C) (co-located B C) (co-located C B))
    ;; state 4 of 4 — 3 candidates · maximal · 5 commitments, 2 minimal
    (model (co-located A B) (co-located A C) (co-located B A) (co-located B C)
           (co-located C A) (co-located C B))
    ;; state 4 is reached by 5 commitments; its 2 minimal
    (merged (model (co-located A B) (co-located A C) (co-located B A)
                   (co-located B C) (co-located C A) (co-located C B))
            (or (model (co-located B A) (co-located C B))
                (model (co-located C A)))))

  query             (co-located A C)
  over              all of R — 4 states → 1 projection class
    class 1 of 1    4 states (1, 2, 3, 4)
                      the goal holds: it has no variable
  as :expect        (model (co-located A C))

  (solve :expect)   holds

exit 0
```

**`--count`** on the same program checks no claim, since it reads no state, and says so in one `claims` row, with a
`note:` on stderr per claim. It exits 0: a claim not checked is not refuted.

`ein solve --count tests/region/03_merged_or.ein`

```text
solve · --count tests/region/03_merged_or.ein
──────────────────────────────────────────────────────────────
  asked             how many states and commitments R holds (--count)
  states (k_s)      4
  commitments (k_c) 8
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates, 0 dead alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              7 saturations · 14 facts beyond root · 4 layers

  claims            not checked — 1 claim, the (solve :expect): --count reads no state, so it checks no claim; a bare `ein solve FILE` checks it

stderr: note: tests/region/03_merged_or.ein: (solve :expect) NOT CHECKED — --count reads no state, so it checks no claim: a bare `ein solve FILE` checks it

exit 0
```

## X13 — `saturation/type-exclusivity/colors.ein`: an exhausted R past 100 lines

Two colours, one rule that keeps them from co-locating, and no obligation: the universe is eight `is-a` and
`co-located` facts, 4 minimal dead sets keep R from being the cube, and R is 169 states. As one `(region …)` it is
537 lines. **Stdout's listings share 100 lines**, each printing whole items while they last and at least
one. The region comes first and takes them: stdout lists states 1–37 outside a `(region …)`, since a cut wrapped
in one would claim to be all of R, then says what it left out and that `--dump-answer DIR` writes the whole
`(region …)` to `DIR/answer.txt`. The query still gets its `over` row, its one class and its one-line `as :expect`.
The rows are never cut. Stdout is 126 lines, and `answer.txt` is 562. The state numbers are the
same in both, so *state 37* on the screen is *state 37* in the file. The blind enumerator binds the type
`Color` as an object, and the `types` row says so.

`ein solve examples/saturation/type-exclusivity/colors.ein`

```text
solve · examples/saturation/type-exclusivity/colors.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      169
  commitments (k_c) 169
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      9 states
  components        1
  sub-cubes         9, 4 of dimension 6, 4 of dimension 5, 1 of dimension 4
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 4 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          8 candidates, 0 dead alone
    types           Color — on the right of an is-a, and 6 candidates of co-located, is-a name it as an object: the blind enumerator is untyped, and no signature names it
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              173 saturations · 644 facts beyond root · 8 layers

  ;; R as one (region …) is 537 lines, past stdout's 100: what fits is below, not wrapped in a (region …)
  ;; state 1 of 169 — ∅, root itself · minimal
  (model (is-a Blue Color) (is-a Red Color))
  ;; state 2 of 169 — 1 candidate · from {(co-located Blue Color)}
  (model (co-located Blue Color) (is-a Blue Color) (is-a Red Color))
  ;; state 3 of 169 — 1 candidate · from {(co-located Color Blue)}
  (model (co-located Color Blue) (is-a Blue Color) (is-a Red Color))
  ;; state 4 of 169 — 1 candidate · from {(co-located Color Red)}
  (model (co-located Color Red) (is-a Blue Color) (is-a Red Color))
  ;; state 5 of 169 — 1 candidate · from {(co-located Red Color)}
  (model (co-located Red Color) (is-a Blue Color) (is-a Red Color))
  ;; state 6 of 169 — 1 candidate · from {(is-a Blue Red)}
  (model (is-a Blue Color) (is-a Blue Red) (is-a Red Color))
  ;; state 7 of 169 — 1 candidate · from {(is-a Color Blue)}
  (model (is-a Blue Color) (is-a Color Blue) (is-a Red Color))
  ;; state 8 of 169 — 1 candidate · from {(is-a Color Red)}
  (model (is-a Blue Color) (is-a Color Red) (is-a Red Color))
  ;; state 9 of 169 — 1 candidate · from {(is-a Red Blue)}
  (model (is-a Blue Color) (is-a Red Blue) (is-a Red Color))
  ;; state 10 of 169 — 2 candidates · from {(co-located Blue Color), (co-located Color Blue)}
  (model (co-located Blue Color) (co-located Color Blue) (is-a Blue Color)
         (is-a Red Color))
  ;; state 11 of 169 — 2 candidates · from {(co-located Blue Color), (co-located Color Red)}
  (model (co-located Blue Color) (co-located Color Red) (is-a Blue Color)
         (is-a Red Color))
  ;; state 12 of 169 — 2 candidates · from {(co-located Blue Color), (co-located Red Color)}
  (model (co-located Blue Color) (co-located Red Color) (is-a Blue Color)
         (is-a Red Color))
  ;; state 13 of 169 — 2 candidates · from {(co-located Blue Color), (is-a Blue Red)}
  (model (co-located Blue Color) (is-a Blue Color) (is-a Blue Red)
         (is-a Red Color))
  ;; state 14 of 169 — 2 candidates · from {(co-located Blue Color), (is-a Color Blue)}
  (model (co-located Blue Color) (is-a Blue Color) (is-a Color Blue)
         (is-a Red Color))
  ;; state 15 of 169 — 2 candidates · from {(co-located Blue Color), (is-a Color Red)}
  (model (co-located Blue Color) (is-a Blue Color) (is-a Color Red)
         (is-a Red Color))
  ;; state 16 of 169 — 2 candidates · from {(co-located Blue Color), (is-a Red Blue)}
  (model (co-located Blue Color) (is-a Blue Color) (is-a Red Blue)
         (is-a Red Color))
  ;; state 17 of 169 — 2 candidates · from {(co-located Color Blue), (co-located Color Red)}
  (model (co-located Color Blue) (co-located Color Red) (is-a Blue Color)
         (is-a Red Color))
  ;; state 18 of 169 — 2 candidates · from {(co-located Color Blue), (co-located Red Color)}
  (model (co-located Color Blue) (co-located Red Color) (is-a Blue Color)
         (is-a Red Color))
  ;; state 19 of 169 — 2 candidates · from {(co-located Color Blue), (is-a Blue Red)}
  (model (co-located Color Blue) (is-a Blue Color) (is-a Blue Red)
         (is-a Red Color))
  ;; state 20 of 169 — 2 candidates · from {(co-located Color Blue), (is-a Color Blue)}
  (model (co-located Color Blue) (is-a Blue Color) (is-a Color Blue)
         (is-a Red Color))
  ;; state 21 of 169 — 2 candidates · from {(co-located Color Blue), (is-a Color Red)}
  (model (co-located Color Blue) (is-a Blue Color) (is-a Color Red)
         (is-a Red Color))
  ;; state 22 of 169 — 2 candidates · from {(co-located Color Blue), (is-a Red Blue)}
  (model (co-located Color Blue) (is-a Blue Color) (is-a Red Blue)
         (is-a Red Color))
  ;; state 23 of 169 — 2 candidates · from {(co-located Color Red), (co-located Red Color)}
  (model (co-located Color Red) (co-located Red Color) (is-a Blue Color)
         (is-a Red Color))
  ;; state 24 of 169 — 2 candidates · from {(co-located Color Red), (is-a Blue Red)}
  (model (co-located Color Red) (is-a Blue Color) (is-a Blue Red)
         (is-a Red Color))
  ;; state 25 of 169 — 2 candidates · from {(co-located Color Red), (is-a Color Blue)}
  (model (co-located Color Red) (is-a Blue Color) (is-a Color Blue)
         (is-a Red Color))
  ;; state 26 of 169 — 2 candidates · from {(co-located Color Red), (is-a Color Red)}
  (model (co-located Color Red) (is-a Blue Color) (is-a Color Red)
         (is-a Red Color))
  ;; state 27 of 169 — 2 candidates · from {(co-located Color Red), (is-a Red Blue)}
  (model (co-located Color Red) (is-a Blue Color) (is-a Red Blue)
         (is-a Red Color))
  ;; state 28 of 169 — 2 candidates · from {(co-located Red Color), (is-a Blue Red)}
  (model (co-located Red Color) (is-a Blue Color) (is-a Blue Red)
         (is-a Red Color))
  ;; state 29 of 169 — 2 candidates · from {(co-located Red Color), (is-a Color Blue)}
  (model (co-located Red Color) (is-a Blue Color) (is-a Color Blue)
         (is-a Red Color))
  ;; state 30 of 169 — 2 candidates · from {(co-located Red Color), (is-a Color Red)}
  (model (co-located Red Color) (is-a Blue Color) (is-a Color Red)
         (is-a Red Color))
  ;; state 31 of 169 — 2 candidates · from {(co-located Red Color), (is-a Red Blue)}
  (model (co-located Red Color) (is-a Blue Color) (is-a Red Blue)
         (is-a Red Color))
  ;; state 32 of 169 — 2 candidates · from {(is-a Blue Red), (is-a Color Blue)}
  (model (is-a Blue Color) (is-a Blue Red) (is-a Color Blue)
         (is-a Red Color))
  ;; state 33 of 169 — 2 candidates · from {(is-a Blue Red), (is-a Color Red)}
  (model (is-a Blue Color) (is-a Blue Red) (is-a Color Red) (is-a Red Color))
  ;; state 34 of 169 — 2 candidates · from {(is-a Blue Red), (is-a Red Blue)}
  (model (is-a Blue Color) (is-a Blue Red) (is-a Red Blue) (is-a Red Color))
  ;; state 35 of 169 — 2 candidates · from {(is-a Color Blue), (is-a Color Red)}
  (model (is-a Blue Color) (is-a Color Blue) (is-a Color Red)
         (is-a Red Color))
  ;; state 36 of 169 — 2 candidates · from {(is-a Color Blue), (is-a Red Blue)}
  (model (is-a Blue Color) (is-a Color Blue) (is-a Red Blue)
         (is-a Red Color))
  ;; state 37 of 169 — 2 candidates · from {(is-a Color Red), (is-a Red Blue)}
  (model (is-a Blue Color) (is-a Color Red) (is-a Red Blue) (is-a Red Color))
  ;; not shown: 132 more states — --dump-answer DIR writes the whole (region …) to DIR/answer.txt

  query             (not (co-located Red Blue))
  over              all of R — 169 states → 1 projection class
    class 1 of 1    169 states (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, …)
                      the goal holds: it has no variable
  as :expect        (model (not (co-located Red Blue)))

exit 0
```

## X14 — `features/07_unstratifiable.ein`: every row of the warning, and the strata

Two rules, each deriving what the other's `absent` reads: the static check flags both, the detector finds an even cycle
through an `absent` and an order finding, and the rules' dependency graph does not stratify, so `strata` and its
`negative cycle` follow the warning. On the cube one node holds a stale fact, which the `stale` row names. Every answer
stands: the rows are diagnostics.

`ein solve examples/features/07_unstratifiable.ein`

```text
solve · examples/features/07_unstratifiable.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      2
  commitments (k_c) 2
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      1 state — ⊤, the trivial line: no restriction bites
  components        1
  sub-cubes         1, of dimension 1
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 0 minimal dead sets
  E                 1 minimal discharging set
  lattice           not monotone: 2 sites (the static check) — no hole in this cube
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            derive-p: absent reads q at absent depth 1, concludes a positive fact — at stake: the state
    site            derive-q: absent reads p at absent depth 1, concludes a positive fact — at stake: the state
    cycle           even through an absent — derive-p: q →¬ p · derive-q: p →¬ q — several states with no stale fact can exist
    order           derive-p and derive-q each read under an absent what the other concludes: which the boundary admits first is the order they are written in
    status          monotone here (conservative) — the cube: no hole, no dead node with an alive superset, no discharged node with an owing superset
    state           stale facts — 1 of the cube's 2 nodes holds one: a fact whose absent guard the node's own fixpoint falsifies
    stale           {(q one)}, in R: (p one) ← derive-p (absent (q one))
    paths           no state the read-out prints has an alive parent to check
  strata            not stratifiable: 1 cycle through an absent — root's fixpoint may depend on the boundary's admission order, and need not be unique
    negative cycle  p, q — derive-p: q →¬ p · derive-q: p →¬ q — even: several fixpoints may exist, and which one root reaches may depend on the order the boundary admits in
  universe          1 candidate, 0 dead alone
  route             2, the cube — no licence: the static check flags the program
  work              2 saturations · 1 fact beyond root

  (region
    ;; state 1 of 2 — ∅, root itself · minimal
    (model)
    ;; state 2 of 2 — 1 candidate · maximal · from {(q one)}
    (model (q one)))

exit 0
```

**`--monotone`** on the same program: the warning's static half, and `bit`, which says what would compute the
cube's half and at what cost. `strata` is printed by a question about R, not by this one.

`ein solve --monotone examples/features/07_unstratifiable.ein`

```text
solve · --monotone examples/features/07_unstratifiable.ein
──────────────────────────────────────────────────────────────
  asked             is the program monotone? — the static check, on root's fixpoint
  monotone          no — 2 sites
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            derive-p: absent reads q at absent depth 1, concludes a positive fact — at stake: the state
    site            derive-q: absent reads p at absent depth 1, concludes a positive fact — at stake: the state
    cycle           even through an absent — derive-p: q →¬ p · derive-q: p →¬ q — several states with no stale fact can exist
    order           derive-p and derive-q each read under an absent what the other concludes: which the boundary admits first is the order they are written in
    bit             not computed — the cube is 2^1 = 2 saturations: `ein oracle FILE` computes it, and so does a bare `ein solve FILE`
  universe          1 candidate
  route             none — a question about the program: root's fixpoint, then the static check
  work              1 saturation, root's · 0 facts beyond root

exit 0
```

## X15 — `features/01_not_and_absent.ein`: the warning past four rows

A flagged program past `--region-limit` whose nodes hold thousands of stale facts: stdout names four `stale` nodes,
then one `…` row counts the rest and names what writes them, so the warning stays bounded on its own and takes none of
the listings' 100 lines.

`ein solve examples/features/01_not_and_absent.ein`

```text
solve · examples/features/01_not_and_absent.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      64 655   (a lower bound: the run stopped before R was exhausted)
  commitments (k_c) 64 655   (a lower bound: the run stopped before R was exhausted)
  exhausted         no — not monotone, and the question needs the whole cube: 2^35, past --region-limit 16
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      not known — without a licence it needs the whole cube
  components        not known: R was not exhausted
  sub-cubes         not known
  N                 ≥ 0 minimal dead sets — those met in layer order
  lattice           not monotone: 2 sites (the static check) — holes not known: the cube was not enumerated
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            unknown-feeling: its activator relation is one a commitment can change; a plan root compiled of it concludes a positive fact — at stake: the state, through a plan a node compiles
    site            unknown-feeling [activator Person]: absent reads likes at absent depth 1, concludes a positive fact — at stake: the state
    status          not known — the cube is 2^35 nodes, past --region-limit 16
    state           stale facts — 24 754 of the 64 655 nodes the run computed hold one: a fact whose absent guard the node's own fixpoint falsifies
    stale           {(likes Bob Alice)}, in R: (unknown-feeling Bob Alice) ← unknown-feeling (absent (likes Bob Alice)) (absent (not (likes Bob Alice)))
    stale           {(likes Bob Carol)}, in R: (unknown-feeling Bob Carol) ← unknown-feeling (absent (likes Bob Carol)) (absent (not (likes Bob Carol)))
    stale           {(likes Carol Alice)}, in R: (unknown-feeling Carol Alice) ← unknown-feeling (absent (likes Carol Alice)) (absent (not (likes Carol Alice)))
    stale           {(likes Carol Bob)}, in R: (unknown-feeling Carol Bob) ← unknown-feeling (absent (likes Carol Bob)) (absent (not (likes Carol Bob)))
    …               24 750 more: --dump-answer DIR writes them
    paths           not checked — the read-out prints no state of two commitments or more; its 64 655 members met would cost up to 386 524 saturations
  universe          35 candidates, dead alone not known: the run stopped before it visited each alone
    types           Person — on the right of an is-a, and 13 candidates of is-a, likes name it as an object: the blind enumerator is untyped, and no signature names it
  route             2, the cube — no licence: the static check flags the program
  work              64 655 saturations · 294 793 facts beyond root

  ;; the line members this run certified. Not a (region …): read back, it would claim these are all of R.
  ;; state 1 of 1 — ∅, root itself · minimal
  (model (is-a Alice Person) (is-a Bob Person) (is-a Carol Person)
         (is-a Person T) (likes Alice Bob))
  ;; 64 654 other members met are not certified on a line, and not listed: --dump-answer DIR writes each to DIR/models/

  query             (unknown-feeling ?a ?b)
  over              the 64 655 members met — 64 655 states → 43 projection classes  (a lower bound)
    class 1 of 43   57 579 states
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
    class 2 of 43   1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Bob · ?b = explicitly-dislikes
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = Carol · ?b = explicitly-dislikes
    class 3 of 43   1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Bob · ?b = explicitly-dislikes
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = explicitly-dislikes · ?b = Alice
    class 4 of 43   1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Bob · ?b = explicitly-dislikes
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = explicitly-dislikes · ?b = Bob
    class 5 of 43   1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Bob · ?b = explicitly-dislikes
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = explicitly-dislikes · ?b = Carol
    class 6 of 43   1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = Carol · ?b = explicitly-dislikes
                      ?a = explicitly-dislikes · ?b = Alice
    class 7 of 43   1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = Carol · ?b = explicitly-dislikes
                      ?a = explicitly-dislikes · ?b = Bob
    class 8 of 43   1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = Carol · ?b = explicitly-dislikes
                      ?a = explicitly-dislikes · ?b = Carol
    class 9 of 43   1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = explicitly-dislikes · ?b = Alice
                      ?a = explicitly-dislikes · ?b = Bob
    class 10 of 43  1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = explicitly-dislikes · ?b = Alice
                      ?a = explicitly-dislikes · ?b = Carol
    class 11 of 43  1 state
                      ?a = Alice · ?b = explicitly-dislikes
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = explicitly-dislikes · ?b = Bob
                      ?a = explicitly-dislikes · ?b = Carol
    class 12 of 43  1 state
                      ?a = Bob · ?b = Alice
                      ?a = Bob · ?b = Carol
                      ?a = Bob · ?b = explicitly-dislikes
                      ?a = Carol · ?b = Alice
                      ?a = Carol · ?b = Bob
                      ?a = Carol · ?b = explicitly-dislikes
                      ?a = explicitly-dislikes · ?b = Alice
                    … 31 more classes: --dump-answer DIR writes them to DIR/answer.txt
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

stderr: note: examples/features/01_not_and_absent.ein: partial — the question needs 2^35 saturations, past --region-limit 16

exit 0
```

## X16 — `ein-bugs/zebra2-bad.ein`: a dead root's core, its sources and a cut search

`zebra2.ein` with one clue added that contradicts the rest: root's own fixpoint derives `(false)`, so R is empty of the
dead kind. Each fact of root's core is an `unsat core` row, a fact the program states with a `:source` gets a `source`
row, and `refuted by` names the step the core's derivation takes for each fact of the contradiction, `(given)` for one
the core holds.

`ein solve examples/ein-bugs/zebra2-bad.ein`

```text
solve · examples/ein-bugs/zebra2-bad.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every node is dead — root's own fixpoint derives (false)
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
    outside         is-a*, next-to — no candidate: no hrule proposes them. R holds of them only what the program states and saturation derives
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 1 layer

  unsat core        (color-loc Green House-1)
  unsat core        (is-a House-1 House)
  unsat core        (relation color-loc Color House)
  source            (color-loc Green House-1) :source "injected contradiction"
  refuted by        (given), co-located-negative

  (false)

  query             (and (drink-loc Water ?h_water) (nation-loc ?who_water ?h_water) (pet-loc Zebra ?h_zebra) (nation-loc ?who_zebra ?h_zebra))
  over              all of R — R is empty, root is dead: no state to answer from
  as :expect        (false)

exit 0
```

**A core search cut at its budget** (`ein-bugs/wrong-answers/core-search-cut.ein`, W23's witness): the core
printed is one the search found within 64 environments per fact, and the `core search` row says that a smaller core may
exist.

`ein solve examples/ein-bugs/wrong-answers/core-search-cut.ein`

```text
solve · examples/ein-bugs/wrong-answers/core-search-cut.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every node is dead — root's own fixpoint derives (false)
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             the state route, every alive state once — L1 dead is upward-closed · L3 a commitment's state is its closure's · L4 the lines are local
  work              1 saturation · 0 facts beyond root · 1 layer

  unsat core        (x1 k)
  unsat core        (x2 k)
  unsat core        (y1 k)
  unsat core        (y2 k)
  unsat core        (y3 k)
  unsat core        (y4 k)
  unsat core        (y5 k)
  unsat core        (y6 k)
  unsat core        (y7 k)
  unsat core        (y8 k)
  core search       cut at its budget of 64 environments per fact: a smaller core may exist
  refuted by        clash

  (false)

exit 0
```

## X17 — `ein oracle`: the cube's line

`ein oracle FILE` saturates every node of the cube, asked for by name, and prints the describe read-out over it, then
one line that only the cube can say: its dead nodes with an alive superset and discharged nodes with an owing superset,
and whether the static verdict held. On X8's flagged program the status is not monotone, by the obligation's trigger.

`ein oracle examples/lattice/04_obligation_trigger_hole.ein`

```text
oracle · examples/lattice/04_obligation_trigger_hole.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      3
  commitments (k_c) 3
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      1 state — ⊤, the trivial line: no restriction bites
  components        1
  sub-cubes         2, each of dimension 1
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 0 minimal dead sets
  E                 1 minimal discharging set
  lattice           not monotone: 1 site (the static check) — 1 hole
    assumes         every node is a fork of root (root's fixpoint, its commitment added, saturated); what a node is on this program is open: Q-M1e2.2, F21
    site            total-owed [activator meal is-a]: obligation-trigger reads is-a at absent depth 0, concludes (open …) — at stake: the status: owes
    status          non-monotone here (exercised, by total-owed [activator meal is-a]) — the cube: 1 hole · 0 dead nodes with an alive superset · 1 discharged node with an owing superset
    state           no stale fact in the cube's 4 nodes
    paths           every parent path checked reaches the same state — 2 parents of 1 printed state
    minimal hole    {(seats Ann C1)} — owes 1
  universe          2 candidates, 0 dead alone
    outside         is-a — no candidate: no hrule proposes it. R holds of it only what the program states and saturation derives
  route             2, the cube — no licence: asked for by name
  work              4 saturations · 6 facts beyond root

  (region
    ;; state 1 of 3 — ∅, root itself · minimal
    (model)
    ;; state 2 of 3 — 1 candidate · from {(meal Ann Soup)}
    (model (meal Ann Soup))
    ;; state 3 of 3 — 2 candidates · maximal · from {(meal Ann Soup), (seats Ann C1)}
    (model (meal Ann Soup) (seats Ann C1)))

  query             (meal ?g ?f)
  over              all of R — 3 states → 2 projection classes
    class 1 of 2    1 state (1) — the goal has no row: undetermined here
    class 2 of 2    2 states (2, 3)
                      ?f = Soup · ?g = Ann
  as :expect        (or (model)
                        (model (meal Ann Soup)))

the cube   0 dead nodes with an alive superset · 1 discharged node with an owing superset — the status is not monotone here, by total-owed [activator meal is-a]

exit 0
```

On X2's monotone program the static verdict holds: no dead node has an alive superset, and none discharged an
owing one.

`ein oracle examples/lattice/02_genuine_3set_death.ein`

```text
oracle · examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      7
  commitments (k_c) 7
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅
  maximal line      3 states
  components        1
  sub-cubes         3, each of dimension 2
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 1 minimal dead set
  E                 1 minimal discharging set
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates, 0 dead alone
  route             2, the cube — no licence: asked for by name
  work              8 saturations · 13 facts beyond root

  (region
    ;; state 1 of 7 — ∅, root itself · minimal
    (model)
    ;; state 2 of 7 — 1 candidate · from {(a-prop X)}
    (model (a-prop X))
    ;; state 3 of 7 — 1 candidate · from {(b-prop X)}
    (model (b-prop X))
    ;; state 4 of 7 — 1 candidate · from {(c-prop X)}
    (model (c-prop X))
    ;; state 5 of 7 — 2 candidates · maximal · from {(a-prop X), (b-prop X)}
    (model (a-prop X) (b-prop X))
    ;; state 6 of 7 — 2 candidates · maximal · from {(a-prop X), (c-prop X)}
    (model (a-prop X) (c-prop X))
    ;; state 7 of 7 — 2 candidates · maximal · from {(b-prop X), (c-prop X)}
    (model (b-prop X) (c-prop X)))

  query             (a-prop ?x)
  over              all of R — 7 states → 2 projection classes
    class 1 of 2    4 states (1, 3, 4, 7) — the goal has no row: undetermined here
    class 2 of 2    3 states (2, 5, 6)
                      ?x = X
  as :expect        (or (model)
                        (model (a-prop X)))

the cube   0 dead nodes with an alive superset · 0 discharged nodes with an owing superset — the static verdict holds

exit 0
```
