# The read-out in real runs — the examples for T1e2.1a.5.1

**Stage:** [S1e2.1a.5](s1e2.1a.5_the_read_out.md), task T1e2.1a.5.1. The page
diff these illustrate is [`read_out.md`](read_out.md).
**Status:** written 2026-10-16, for the user to read with the page diff, and
re-generated the same day after the user's three rulings
([`read_out.md` § 11](read_out.md#11-the-users-rulings-2026-10-16)): every
answer exits 0, and a listing past 100 lines is cut on stdout and written whole
by `--dump-answer` (X13). **Nothing in `ein solve` has moved.**
**How it was made:** every block below is the stdout of
[`read_out_preview`](../../../ein.rs/crates/ein-infer/examples/read_out_preview.rs),
an `ein-infer` example that prints the proposed read-out from the library's own
computation: R from route 1 or the cube, a mode's answer from
`ein_infer::explore`, each state's facts from its own fixpoint, and each
query's projection classes from `goal_bindings` on those fixpoints. The
caption names the `ein solve` run the block stands for; the preview takes the
same flags (`--over` stands in for a query's `:over`, which does not parse
yet). The last line of each block is the exit code the proposal gives that
run, and a `stderr:` line is what it would write there. A block captioned
`--dump-answer DIR` is what that run writes to `DIR/answer.txt`: the read-out
with no listing cut, which the preview prints under `--uncut`. Re-take any
block with

```sh
cargo run --release -p ein-infer --example read_out_preview -- [FLAGS] FILE
```

The page was generated at `b07dbc25` plus the preview's two changes for the
rulings. No fact or number in a
block was typed by hand.

---

## Contents

| | program | n | what it shows |
|---|---|---:|---|
| [X1](#x1--branching05_mini_zebraein-everything-at-once) | `branching/05_mini_zebra.ein` | 9 | the default read-out, every row; `(merged …)` with `(or …)`; a query with an undetermined class |
| [X2](#x2--lattice02_genuine_3set_deathein-one-program-every-mode) | `lattice/02_genuine_3set_death.ein` | 3 | one program under every mode: describe, `--maximal`, `--minimal`, `-n 2`, `--single`, `--maximal --single` |
| [X3](#x3--features11_expect_ambiguityein-a-query-over-the-maximal-line) | `features/11_expect_ambiguity.ein` | 4 | a query over all of R against one `:over maximal`: today's only `(or …)` claim, migrated |
| [X4](#x4--lattice07_two_pairs_two_tablesein-components-and-projection-classes) | `lattice/07_two_pairs_two_tables.ein` | 16 | the user's two tables: 8 components, and 2 projection classes of 4 |
| [X5](#x5--latticesynthmerged_statesein-every-state-merged) | `lattice/synth/merged_states.ein` | 5 | ∅ not in R; a `(merged …)` on every state; two components |
| [X6](#x6--testsstdlibclosure03_closed_and_owingein-r-empty-alive-nodes-owe) | `tests/stdlib/closure/03_closed_and_owing.ein` | 0 | empty R, the owing kind; `--single` says `Open` |
| [X7](#x7--testsstdlibalgebra09_irreflexive_violatedein-r-empty-every-node-dead) | `tests/stdlib/algebra/09_irreflexive_violated.ein` | 0 | empty R, the dead kind; `--single` says `Contradiction` |
| [X8](#x8--ein-bugsnaf-upward-closureein-a-hole) | `ein-bugs/naf-upward-closure.ein` | 2 | a program the static check flags: route 2, a hole, the stub |
| [X9](#x9--lattice06_friends_tablesein-flagged-and-past-the-limit) | `lattice/06_friends_tables.ein` | 19 | flagged and past `--region-limit`: members, never line members |
| [X10](#x10--zebra2ein-the-puzzle) | `zebra2.ein` | 56 | the puzzle: partial at the default budget and already answered; exhausted at 2²⁵, cut on stdout and whole in `answer.txt`; `--maximal -n 1`; `--single` |
| [X11](#x11--saturationsquare-bwdfloorsein-a-region-that-is-the-whole-cube) | `saturation/square-bwd/floors.ein` | 20 | R is the whole cube: partial, with both trivial lines certified; the first questions in 1–11 saturations |
| [X12](#x12--testsregion03_merged_orein-a-program-that-states-its-region) | `tests/region/03_merged_or.ein` | 3 | a program that states its region: the printed region is its claim |
| [X13](#x13--saturationtype-exclusivitycolorsein-an-exhausted-r-past-100-lines) | `saturation/type-exclusivity/colors.ein` | 8 | an exhausted R past stdout's 100 lines: what fits, and where the rest is |

## X1 — `branching/05_mini_zebra.ein`: everything at once

[`region_examples.md`](region_examples.md)'s E1. Two people, two drinks and two
pets; the clues put Alice with the tea and the cat, and Bob is open. Every row
of the default read-out is here. ∅ is in R because the program states no
obligation, so the minimal line is the trivial {∅}. The maximal state is
reached by four commitments, three of them minimal, and that is the one
`(merged …)`. The query asks who drinks the coffee and which pet they keep: on
four of the five states the goal has no row, and that class is `(model)`.

`ein solve examples/branching/05_mini_zebra.ein`

```text
solve · examples/branching/05_mini_zebra.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      5
  commitments (k_c) 8
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅, the trivial line: no obligation is stated
  maximal line      1 state
  components        1
  sub-cubes         1, of dimension 3
  merged            1 state reached by more than one commitment, with 3 minimal
  N                 6 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          9 candidates, 6 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              14 saturations · 66 facts beyond root · 4 layers

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

[`solution_semantics.md` § 2.5](../../../docs/kernel/inference/solution_semantics.md#25-worked-exampleslattice02_genuine_3set_deathein)'s
worked example: three candidates, every pair alive, the triple dead, no
obligation. Six runs, one per mode, so the same R can be read through each
question.

**Describe.** Seven states, the three pairs on the maximal line, three
sub-cubes of dimension 2.

`ein solve examples/lattice/02_genuine_3set_death.ein`

```text
solve · examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      7
  commitments (k_c) 7
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅, the trivial line: no obligation is stated
  maximal line      3 states
  components        1
  sub-cubes         3, each of dimension 2
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 1 minimal dead set
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates, 0 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
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

**`--maximal`.** The line is listed as states, not as a `(region …)`: a line
is not a claim the language can state (only a query's `:over` names one), and a
`(region …)` read back would say that these three are all of R.

`ein solve --maximal examples/lattice/02_genuine_3set_death.ein`

```text
solve · --maximal examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             the maximal line — every state on it
  maximal line      3 states — all of them
  exhausted         yes — this is all of the maximal line
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed · L4 the lines are local
  work              7 saturations · 12 facts beyond root

  ;; state 1 of 3 — 2 candidates · maximal · from {(a-prop X), (b-prop X)}
  (model (a-prop X) (b-prop X))
  ;; state 2 of 3 — 2 candidates · maximal · from {(a-prop X), (c-prop X)}
  (model (a-prop X) (c-prop X))
  ;; state 3 of 3 — 2 candidates · maximal · from {(b-prop X), (c-prop X)}
  (model (b-prop X) (c-prop X))

  query             (a-prop ?x)
  over              the 3 states listed — 3 states → 2 projection classes
    class 1 of 2    1 state (3) — the goal has no row: undetermined here
    class 2 of 2    2 states (1, 2)
                      ?x = X
  as :expect        (or (model)
                        (model (a-prop X)))

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
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed
  work              1 saturation · 0 facts beyond root

  ;; state 1 of 1 — ∅, root itself · minimal
  (model)

  query             (a-prop ?x)
  over              the 1 state listed — 1 state → 1 projection class
    class 1 of 1    1 state (1) — the goal has no row: undetermined here
  as :expect        (model)

exit 0
```

**`-n 2`.** The first two states in the route's order, the same two under
every lever (C4). Nothing is said about the rest, and the exit code is 0: a
request, not an expectation.

`ein solve -n 2 examples/lattice/02_genuine_3set_death.ein`

```text
solve · -n 2 examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             2 states of R (-n 2)
  states of R       2 states, as asked — R may hold more
  exhausted         no — stopped as asked; each state shown is certified, nothing is said of the rest
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             1, the alive down-set — L1 dead is upward-closed
  work              2 saturations · 1 fact beyond root

  ;; state 1 of 2 — ∅, root itself
  (model)
  ;; state 2 of 2 — 1 candidate · from {(a-prop X)}
  (model (a-prop X))

  query             (a-prop ?x)
  over              the 2 states listed — 2 states → 2 projection classes  (a lower bound)
    class 1 of 2    1 state — the goal has no row: undetermined here
    class 2 of 2    1 state
                      ?x = X
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

**`--single`.** The run's own expectation, so the verdict word is printed:
`Ambiguity` after two saturations. It exits 0, as every answer does: the
word is the answer, and an answer is a successful run.

`ein solve --single examples/lattice/02_genuine_3set_death.ein`

```text
solve · --single examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Ambiguity — R holds two states or more; two of them below
  decided           yes — two states settle it; the rest of R was not visited
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed
  work              2 saturations · 1 fact beyond root

  ;; state 1 of 2 — ∅, root itself
  (model)
  ;; state 2 of 2 — 1 candidate · from {(a-prop X)}
  (model (a-prop X))

  query             (a-prop ?x)
  over              the 2 states listed — 2 states → 2 projection classes  (a lower bound)
    class 1 of 2    1 state — the goal has no row: undetermined here
    class 2 of 2    1 state
                      ?x = X
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

**`--maximal --single`.** The question a restrictions-only program means by
*unique*: the maximal line has three states, so `Ambiguity` again, with two of
them.

`ein solve --maximal --single examples/lattice/02_genuine_3set_death.ein`

```text
solve · --maximal --single examples/lattice/02_genuine_3set_death.ein
──────────────────────────────────────────────────────────────
  asked             is the maximal line one state? (--single)
  verdict           Ambiguity — the maximal line holds two states or more; two of them below
  decided           yes — two states settle it; the rest of the maximal line was not visited
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed · L4 the lines are local
  work              6 saturations · 10 facts beyond root

  ;; state 1 of 2 — 2 candidates · maximal · from {(a-prop X), (b-prop X)}
  (model (a-prop X) (b-prop X))
  ;; state 2 of 2 — 2 candidates · maximal · from {(a-prop X), (c-prop X)}
  (model (a-prop X) (c-prop X))

  query             (a-prop ?x)
  over              the 2 states listed — 2 states → 1 projection class  (a lower bound)
    class 1 of 1    2 states
                      ?x = X
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

## X3 — `features/11_expect_ambiguity.ein`: a query over the maximal line

[`region_examples.md`](region_examples.md)'s E5. Two people, two seats,
restrictions only. Today's claim, the corpus's only `(or …)`, lists **R's
maximal line**: two of R's seven states. Over all of R the goal `(seat ?who
?slot)` sees every seat fact, so each state is its own class, and `∅` is the
undetermined one.

`ein solve examples/features/11_expect_ambiguity.ein`

```text
solve · examples/features/11_expect_ambiguity.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      7
  commitments (k_c) 7
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅, the trivial line: no obligation is stated
  maximal line      2 states
  components        1
  sub-cubes         2, each of dimension 2
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 4 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          4 candidates, 0 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              11 saturations · 20 facts beyond root · 3 layers

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
  over              all of R — 7 states → 7 projection classes
    class 1 of 7    1 state (1) — the goal has no row: undetermined here
    class 2 of 7    1 state (2)
                      ?slot = S1 · ?who = Ann
    class 3 of 7    1 state (3)
                      ?slot = S2 · ?who = Ann
    class 4 of 7    1 state (4)
                      ?slot = S1 · ?who = Bob
    class 5 of 7    1 state (5)
                      ?slot = S2 · ?who = Bob
    class 6 of 7    1 state (6)
                      ?slot = S1 · ?who = Ann
                      ?slot = S2 · ?who = Bob
    class 7 of 7    1 state (7)
                      ?slot = S1 · ?who = Bob
                      ?slot = S2 · ?who = Ann
  as :expect        (or (model)
                        (model (seat Ann S1))
                        (model (seat Ann S2))
                        (model (seat Bob S1))
                        (model (seat Bob S2))
                        (model (seat Ann S1) (seat Bob S2))
                        (model (seat Ann S2) (seat Bob S1)))

exit 0
```

Asked `:over maximal` (the preview's `--over maximal`; the keyword lands at
the switch), the query's answer is exactly today's claim, word for word. That
is how [`region_language.md` § 7](region_language.md#7-the-migration-named)
migrates it. Only the query block differs, so only it is shown:

`ein solve examples/features/11_expect_ambiguity.ein`, the query asked `:over maximal`

```text
  query             (seat ?who ?slot)
  over              the maximal line — 2 states → 2 projection classes
    class 1 of 2    1 state (1)
                      ?slot = S1 · ?who = Ann
                      ?slot = S2 · ?who = Bob
    class 2 of 2    1 state (2)
                      ?slot = S1 · ?who = Bob
                      ?slot = S2 · ?who = Ann
  as :expect        (or (model (seat Ann S1) (seat Bob S2))
                        (model (seat Ann S2) (seat Bob S1)))

exit 0
```

## X4 — `lattice/07_two_pairs_two_tables.ein`: components and projection classes

The user's example of 2026-10-16 (ruling (i)): two pairs of friends, two tables
of two seats, friends sit together. R is the eight full seatings. Each is its
own **component**, because moving one friend passes through a node that is dead
or owing, and minimal and maximal lines coincide. The goal `(sits ?p ?s)` names
the seats, so each state is its own projection class.

`ein solve examples/lattice/07_two_pairs_two_tables.ein`

```text
solve · examples/lattice/07_two_pairs_two_tables.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      8
  commitments (k_c) 8
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 4 instances
  minimal line      8 states, from 8 commitments — the least guess is 4 candidates
  maximal line      8 states
  components        8, each a single commitment
  sub-cubes         8, each of dimension 0
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 64 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          16 candidates, 0 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              177 saturations · 448 facts beyond root · 5 layers

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

The user's *"query can ask e.g. only what table selected, and then seats will
be hidden information"* needs a goal that does not name the seat. A projection
class is the goal's **atoms** under its bindings (ruled at S1e2.1a.4), so the
seat is hidden only by a relation that does not carry it. The program below is
07 with one relation and one rule added, and it is **not a corpus file**:

```lisp
(relation pair-at Group Table)

(rule pair-at ()
  :match  (and (member ?a ?g) (sits ?a ?s) (at ?s ?t))
  :assert (pair-at ?g ?t))

(query :goal (pair-at ?g ?t))
```

R does not move (the rule derives no candidate), and the query now sees two
classes of four: which pair took which table.

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

*A brings B* and *C brings D*; an obligation needs A or C, and A with C is dead.
∅ owes, so it is not in R and the minimal line is two states, each reached from
one candidate. Every state is reached by exactly two commitments, its minimal
one and that one plus what it brings, so every state gets a `(merged …)`:
k_c = 16 against k_s = 8. The A-side and the C-side never meet, which is the two
components.

`ein solve examples/lattice/synth/merged_states.ein`

```text
solve · examples/lattice/synth/merged_states.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      8
  commitments (k_c) 16
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 1 instance
  minimal line      2 states, from 2 commitments — the least guess is 1 candidate
  maximal line      2 states
  components        2, with 8 + 8 commitments
  sub-cubes         2, each of dimension 3
  merged            8 states reached by more than one commitment, with 8 minimal
  N                 1 minimal dead set
  lattice           monotone (the static check) — R is convex: no hole
  universe          5 candidates, 0 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              25 saturations · 65 facts beyond root · 5 layers

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

## X6 — `tests/stdlib/closure/03_closed_and_owing.ein`: R empty, alive nodes owe

No candidate, and root owes one `total-owed` instance: R is empty of the
second kind. The `owing` rows are the frontier, the alive owing nodes with no
alive node one fact up (here root itself), each with what it owes through the
rule's `:why`. The printed form is `(region (owing (model)))`. It names no fact
because the printer closes the **candidates'** relations, which tell R's states
apart, and this program has none. The fixture's own migrated claim pins
`(closed-here r)` as well, which a claim may: naming a relation closes it, and
naming more is allowed.

`ein solve tests/stdlib/closure/03_closed_and_owing.ein`

```text
solve · tests/stdlib/closure/03_closed_and_owing.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      0
  commitments (k_c) 0
  exhausted         yes — every commitment is classified: this is all of R
  empty             every alive node owes — no commitment pays every debt
  lattice           monotone (the static check) — R is convex: no hole
  universe          no candidate: R is {∅} or empty, decided by root alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              1 saturation · 0 facts beyond root · 1 layer

  owing             ∅ — owes 1
                      r owes a1 a B.

  (region (owing (model)))

  query             (closed-here ?R)
  over              all of R — R is empty: no state to answer from
  as :expect        (false)

exit 0
```

**`--single`.** Expected one state, found none of the owing kind: `Open`.

`ein solve --single tests/stdlib/closure/03_closed_and_owing.ein`

```text
solve · --single tests/stdlib/closure/03_closed_and_owing.ein
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Open — R is empty: every alive node owes
  decided           yes — R is empty
  lattice           monotone (the static check) — R is convex: no hole
  universe          0 candidates
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed · L4 the lines are local
  work              1 saturation · 0 facts beyond root

  query             (closed-here ?R)
  over              the 0 states listed — R is empty: no state to answer from
  as :expect        (false)

exit 0
```

## X7 — `tests/stdlib/algebra/09_irreflexive_violated.ein`: R empty, every node dead

Root holds `(loops A A)`, which `irreflexive` refutes. Root's core and the rule
that refutes it say why, and the form is `(false)`.

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
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              1 saturation · 0 facts beyond root · 1 layer

  unsat core        (loops A A)
  refuted by        irreflexive

  (false)

  query             (loops ?x ?y)
  over              all of R — R is empty: no state to answer from
  as :expect        (false)

exit 0
```

**`--single`.** Expected one state, found none of the dead kind:
`Contradiction`.

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
  over              the 0 states listed — R is empty: no state to answer from
  as :expect        (false)

exit 0
```

## X8 — `ein-bugs/naf-upward-closure.ein`: a hole

W3's witness. A guard's `absent` makes `{(p A)}` dead alone and alive beside
`(q A)`, so the static check flags the program and route 2 takes the whole cube,
four saturations. R is `{∅, {q}, {p, q}}`, and `{p}` is a **hole**: outside R,
between two members of it. The `lattice` row is S1e2.1a.1's stub, which says so,
counts the holes and names each minimal one with why it is out; the `E` row is
the cube's alone.

`ein solve examples/ein-bugs/naf-upward-closure.ein`

```text
solve · examples/ein-bugs/naf-upward-closure.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      3
  commitments (k_c) 3
  exhausted         yes — every commitment of the cube is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state
  maximal line      1 state
  components        1
  sub-cubes         2
  N                 1 minimal dead set
  E                 1 minimal discharging set
  lattice           not monotone: 1 site (the static check) — 1 hole; assumes a monotone lattice for what a node is (P1e2.1b)
    minimal hole    {(p A)} — dead: (p A)
  universe          2 candidates, 1 dead alone
  route             2, the cube — no licence: the static check flags the program
  work              4 saturations · 2^2

  (region
    ;; state 1 of 3 — ∅, root itself · minimal
    (model)
    ;; state 2 of 3 — 1 candidate · from {(q A)}
    (model (q A))
    ;; state 3 of 3 — 2 candidates · maximal · from {(p A), (q A)}
    (model (p A) (q A)))

  query             (is-a ?x T)
  over              all of R — 3 states → 1 projection class
    class 1 of 1    3 states (1, 2, 3)
                      ?x = A
  as :expect        (model (is-a A T))

exit 0
```

## X9 — `lattice/06_friends_tables.ein`: flagged, and past the limit

The everyone-sits twin's sibling, where *a group that does not fit goes
elsewhere*: the obligation's trigger grows with the commitment, so the program
is not monotone (the third answer). n = 19 is past `--region-limit 16`, so no
route can describe R. The run visits the cube in layer order up to 2¹⁶ nodes.
It meets 21 members, which is all of R by the third answer's count, and it
cannot know that, so every count is a lower bound. What it *can* certify is
one line: ∅ is in R, so the minimal line is exactly {∅}, whatever the program.
That is the one state it lists. The other 20 members it met go to
`--dump-answer`, and the query is answered over all 21 as a lower bound, with
no `as :expect`: read back, that claim would say these classes are all.

`ein solve examples/lattice/06_friends_tables.ein`

```text
solve · examples/lattice/06_friends_tables.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      21   (a lower bound: the run stopped before R was exhausted)
  commitments (k_c) 21   (a lower bound: the run stopped before R was exhausted)
  exhausted         no — not monotone, and the question needs the whole cube: 2^19, past --region-limit 16
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅: root is in R, so nothing in R is below it
  maximal line      not known — without a licence it needs the whole cube
  lattice           not monotone: 1 site (the static check) — holes not known: the cube was not enumerated; assumes a monotone lattice for what a node is (P1e2.1b)
  universe          19 candidates
  route             2, the cube — L7 the cube
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

## X10 — `zebra2.ein`: the puzzle

**At the default budget** route 1 stops before layer 5. It is partial, and it
has already answered the puzzle: the one state it found is certified minimal,
reached from the single fact `(drink-loc Juice House-4)`, and certified
maximal. The query's class is printed with the lower-bound label.

`ein solve examples/zebra2.ein`

```text
solve · examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      1   (a lower bound: complete up to commitments of 4 candidates)
  commitments (k_c) 3 916   (a lower bound: complete up to commitments of 4 candidates)
  exhausted         no — route 1 stopped before layer 5: its 10 131 commitments project 526 812 facts on top of the 353 194 spent, past --region-budget 524288
  ∅                 not in R — root owes 36 instances
  minimal line      ≥ 1 state — complete up to commitments of 4 candidates
  maximal line      ≥ 1 state — all those a commitment of up to 3 candidates reaches
  components        not known: R was not exhausted
  sub-cubes         not known
  N                 ≥ 366 minimal dead sets — complete up to size 4
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates, 21 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              5 908 saturations · 353 194 facts beyond root · 5 layers

  ;; the line members this run certified. Not a (region …): read back, it would claim these are all of R.
  ;; state 1 of 1 — 18 candidates · minimal and maximal · from {(drink-loc Juice House-4)}
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
  over              all of R — 1 state → 1 projection class  (a lower bound)
    class 1 of 1    1 state
                      ?h_water = House-1 · ?h_zebra = House-5 · ?who_water = Norwegian · ?who_zebra = Japanese
                        “The Norwegian drinks water in House-1; the Japanese owns zebra in House-5”
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

stderr: note: examples/zebra2.ein: partial — route 1 stopped before layer 5 at --region-budget 524288

exit 0
```

**At `--region-budget 33554432`** (2²⁵ facts; 22 s at `-j 8`) R is exhausted:
one state, 261 982 commitments, 63 minimal generators, and four of them single
facts. R as one `(region …)` is 111 lines, past stdout's 100, so stdout lists
the state and leaves out the `(merged …)`, and says where it is.

`ein solve --region-budget 33554432 examples/zebra2.ein` — 22 s at `-j 8`

```text
solve · --region-budget 33554432 examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      1
  commitments (k_c) 261 982
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 36 instances
  minimal line      1 state, from 63 commitments — the least guess is 1 candidate
  maximal line      1 state
  components        1
  sub-cubes         63, 4 of dimension 17, 59 of dimension 16
  merged            1 state reached by more than one commitment, with 63 minimal
  N                 366 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates, 21 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              270 579 saturations · 15 094 278 facts beyond root · 19 layers

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

**What `--dump-answer DIR` writes to `DIR/answer.txt`** for the same run: the
whole `(region …)`, whose `(merged …)` lists all 63 minimal commitments. That
is what pins k_c, and it reads back as the claim.

`ein solve --dump-answer DIR --region-budget 33554432 examples/zebra2.ein` — `DIR/answer.txt`

```text
solve · --dump-answer DIR --region-budget 33554432 examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      1
  commitments (k_c) 261 982
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 not in R — root owes 36 instances
  minimal line      1 state, from 63 commitments — the least guess is 1 candidate
  maximal line      1 state
  components        1
  sub-cubes         63, 4 of dimension 17, 59 of dimension 16
  merged            1 state reached by more than one commitment, with 63 minimal
  N                 366 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates, 21 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              270 579 saturations · 15 094 278 facts beyond root · 19 layers

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

**`--maximal -n 1`**: a complete solution, 54 saturations.

`ein solve --maximal -n 1 examples/zebra2.ein`

```text
solve · --maximal -n 1 examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             1 state of the maximal line (-n 1)
  maximal line      1 state, as asked — the maximal line may hold more
  exhausted         no — stopped as asked; each state shown is certified, nothing is said of the rest
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed · L4 the lines are local
  work              54 saturations · 8 016 facts beyond root

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
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

**`--single`**: `Solution`, 8 616 saturations, at the default budget.

`ein solve --single examples/zebra2.ein`

```text
solve · --single examples/zebra2.ein
──────────────────────────────────────────────────────────────
  asked             is R one state? (--single)
  verdict           Solution — R is one state
  decided           yes — R holds no other state
  lattice           monotone (the static check) — R is convex: no hole
  universe          56 candidates
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed · L4 the lines are local
  work              8 616 saturations · 418 806 facts beyond root

  ;; state 1 of 1 — 18 candidates · from {(drink-loc Juice House-4)}
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
  over              the 1 state listed — 1 state → 1 projection class
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

No restriction and no obligation: every commitment is alive and in R, 2²⁰ of
them. **Describe** stops after layer 5, partial. Both lines are still exact:
∅ is in R, so the minimal line is {∅}, and one saturation of ⊤ finds it alive
and owing nothing, so the maximal line is {⊤}. Those two are what it lists.
The 10 075 other members it met go to `--dump-answer`, not to the screen, and
the query's one class is printed without `as :expect`, since it is a lower
bound.

`ein solve examples/saturation/square-bwd/floors.ein`

```text
solve · examples/saturation/square-bwd/floors.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      10 077   (a lower bound: complete up to commitments of 5 candidates)
  commitments (k_c) 21 701   (a lower bound: complete up to commitments of 5 candidates)
  exhausted         no — route 1 stopped before layer 6: its 38 760 commitments project 387 600 facts on top of the 197 809 spent, past --region-budget 524288
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅, the trivial line: no obligation is stated
  maximal line      1 state — ⊤, every candidate: no node is dead, the trivial line, certified by one saturation
  components        not known: R was not exhausted
  sub-cubes         not known
  N                 ≥ 0 minimal dead sets — complete up to size 5
  lattice           monotone (the static check) — R is convex: no hole
  universe          20 candidates, 0 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              21 700 saturations · 197 809 facts beyond root · 6 layers

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
  ;; 10 075 other members met are not certified on a line, and not listed: --dump-answer DIR writes each to DIR/models/

  query             (co-located Floor2 Floor5)
  over              all of R — 10 077 states → 1 projection class  (a lower bound)
    class 1 of 1    10 077 states
                      the goal holds: it has no variable
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

stderr: note: examples/saturation/square-bwd/floors.ein: partial — route 1 stopped before layer 6 at --region-budget 524288

exit 0
```

**The first questions** cost what [`solve_interface.md` § 3.4](solve_interface.md#34-what-it-costs)
said they would: `-n 1` is root (one saturation), `--single` is `Ambiguity`
in two, `--maximal -n 1` is the climb (eleven).

`ein solve -n 1 examples/saturation/square-bwd/floors.ein`

```text
solve · -n 1 examples/saturation/square-bwd/floors.ein
──────────────────────────────────────────────────────────────
  asked             1 state of R (-n 1)
  states of R       1 state, as asked — R may hold more
  exhausted         no — stopped as asked; each state shown is certified, nothing is said of the rest
  lattice           monotone (the static check) — R is convex: no hole
  universe          20 candidates
  route             1, the alive down-set — L1 dead is upward-closed
  work              1 saturation · 0 facts beyond root

  ;; state 1 of 1 — ∅, root itself
  (model (above-of Floor2 Floor1) (above-of Floor5 Floor4)
         (co-located Floor1 Floor4) (co-located Floor2 Floor5))

  query             (co-located Floor2 Floor5)
  over              the 1 state listed — 1 state → 1 projection class  (a lower bound)
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
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed
  work              2 saturations · 1 fact beyond root

  ;; state 1 of 2 — ∅, root itself
  (model (above-of Floor2 Floor1) (above-of Floor5 Floor4)
         (co-located Floor1 Floor4) (co-located Floor2 Floor5))
  ;; state 2 of 2 — 1 candidate · from {(above-of Floor1 Floor2)}
  (model (above-of Floor1 Floor2) (above-of Floor2 Floor1)
         (above-of Floor5 Floor4) (co-located Floor1 Floor4)
         (co-located Floor2 Floor5))

  query             (co-located Floor2 Floor5)
  over              the 2 states listed — 2 states → 1 projection class  (a lower bound)
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
  maximal line      1 state, as asked — the maximal line may hold more
  exhausted         no — stopped as asked; each state shown is certified, nothing is said of the rest
  lattice           monotone (the static check) — R is convex: no hole
  universe          20 candidates
  route             1, the alive down-set — L1 dead is upward-closed · L2 discharged is upward-closed · L4 the lines are local
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
    class 1 of 1    1 state
                      the goal holds: it has no variable
  as :expect        not printed: these classes are a lower bound, and the claim would say they are all

exit 0
```

## X12 — `tests/region/03_merged_or.ein`: a program that states its region

S1e2.1a.4's fixture for `(merged …)` with `(or …)`: the colocation chain,
[`region_examples.md`](region_examples.md)'s E2. Its `(solve … :expect (region …))`
is checked against the R the run described, and the `(region …)` printed above
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
  minimal line      1 state — ∅, the trivial line: no obligation is stated
  maximal line      1 state — ⊤, the trivial line: no restriction bites
  components        1
  sub-cubes         1, of dimension 3
  merged            1 state reached by more than one commitment, with 2 minimal
  N                 0 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          3 candidates, 0 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              8 saturations · 17 facts beyond root · 4 layers

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

## X13 — `saturation/type-exclusivity/colors.ein`: an exhausted R past 100 lines

Two colours, one rule that keeps them from co-locating, and no obligation: the
universe is eight `is-a` and `co-located` facts, 4 minimal dead sets keep
R from being the cube, and R is 169 states. As one `(region …)` it is
537 lines. The user's ruling: **stdout's listings share 100
lines**, each printing whole items while they last and at least one. The
region comes first and takes them: stdout lists states 1–37 outside a
`(region …)`, since a cut wrapped in one would claim to be all of R, then says
what it left out and that `--dump-answer DIR` writes the whole `(region …)` to
`DIR/answer.txt`. The query still gets its `over` row, its one class and its
one-line `as :expect`. The summary rows are never cut. Stdout is
125 lines, and `answer.txt` is 561. The state numbers are
the same in both, so *state 37* on the screen is *state 37* in the
file.

`ein solve examples/saturation/type-exclusivity/colors.ein`

```text
solve · examples/saturation/type-exclusivity/colors.ein
──────────────────────────────────────────────────────────────
  asked             all of R — describe
  states (k_s)      169
  commitments (k_c) 169
  exhausted         yes — every commitment is classified: this is all of R
  ∅                 in R — root is alive and owes nothing
  minimal line      1 state — ∅, the trivial line: no obligation is stated
  maximal line      9 states
  components        1
  sub-cubes         9, 4 of dimension 6, 4 of dimension 5, 1 of dimension 4
  merged            none — every state is reached by one commitment, so k_c = k_s
  N                 4 minimal dead sets
  lattice           monotone (the static check) — R is convex: no hole
  universe          8 candidates, 0 dead alone
  route             1, the alive down-set — licence L1, dead is upward-closed
  work              173 saturations · 644 facts beyond root · 7 layers

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
