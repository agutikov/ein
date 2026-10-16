# How far route 1 reaches — 2026-10-16

**T1e2.1a.9.1's measurement**, for
[S1e2.1a.9](s1e2.1a.9_solve_describes_the_region.md). Route 1 is a monotone
program's region computed from its alive down-set
([`ein_infer::downset`](../../../ein.rs/crates/ein-infer/src/downset.rs),
T1e2.1a.9.3). It visits the alive commitments and `N`, the minimal dead sets,
where the oracle visits all 2ⁿ. This page says what that reaches, what it
costs, and where its budget should sit. The budget is
[`solve_flags.md`](solve_flags.md)'s third question for the user.

```sh
cargo run --release -p ein-infer --example route1_reach -- --budget 262144 --oracle > rows.jsonl
cargo run --release -p ein-infer --example route1_reach -- --budget 1048576 --jobs 12 examples/zebra.ein …
```

Taken 2026-10-16 on `3fde4ef1` with this stage's route 1, release build, one
job unless a row says otherwise, on a 32-thread machine with nothing else
running. One JSON line per program. The tables were read off the lines.

**What it selects.** Every file
[`corpus_files()`](../../../ein.rs/crates/ein-corpus/src/lib.rs) lists, plus the
register's witnesses, that loads and compiles: **195 programs**. That is
[`region_census.md`](region_census.md)'s `--all` selection less the 55 that
do not load, plus one `n = 0` program the census's selection does not hold.
Its numbers at `n ≤ 16` are therefore the census's to within that one row.

**Why route 1 is right where it answers** is a test, not this page:
`ein-infer/tests/region.rs`'s `route_1_is_the_oracle_on_every_monotone_program_it_reaches`
holds route 1 to the oracle node for node on every monotone program with
`n ≤ 16`. It checks:

- the visited set is exactly alive ∪ N;
- both status bits agree on every node visited;
- the lines, k_c, k_s, components, sub-cubes, states, closures and merges all
  agree;
- root afterwards is `sat(program)`;
- route 1 declines on every flagged program.

---

## 1. The three routes over the corpus

At route 1's proposed default (§ 4), and at every budget from 2¹⁵ to 2¹⁸
visited commitments:

| route | programs | of which `n > 16` |
|---|---:|---:|
| **1**, route 1 describes R | **162** | 15 |
| **2**, the oracle (flagged, `n ≤ 16`) | 13 | — |
| **3**, neither: the walk, partial | 20 | 20 |
| | **195** | |

- **Every monotone program at `n ≤ 16` is route 1's**: 147 of 147.
- **Route 1 never declined because the cube contradicted the static verdict.**
  It saw no alive, discharged node with an alive owing node one fact up, on
  any program.
- Route 3's twenty are **12 monotone programs past the budget** (§ 6) and
  **8 flagged programs past the oracle's limit**:
  - `lattice/06_friends_tables`;
  - `features/01`, `/04`, `/05`, `/12`;
  - `square-unique/cul-de-sac`, `/corner-house`, `/terminus`.

## 2. Route 1 against the oracle, at `n ≤ 16`

Over the 147 monotone programs both routes reach, at one job:

| | route 1 | the oracle |
|---|---:|---:|
| nodes saturated | **49 303** | 268 138 |
| share | 18.4 % | 100 % |
| wall | **1.75 s** | 21.18 s |

The census's first take of the same ratio was 18.4 % by arithmetic over the
oracle's dump ([`region_census.md` § 10](region_census.md#10-route-1s-reach-for-s1e21a9)).
This is the route itself, and it agrees.

| program | n | route 1 visits | of | route 1 | oracle |
|---|---:|---:|---:|---:|---:|
| `tests/stdlib/bijection/06_blind_enumeration` | 16 | 17 | 65 536 | 0.001 s | 8.27 s |
| `branching/01_saturate_only` | 16 | 17 | 65 536 | 0.001 s | 6.00 s |
| `lattice/07_two_pairs_two_tables` | 16 | 177 | 65 536 | 0.002 s | 4.55 s |
| `features/02_star_in_identifiers` | 15 | 32 768 | 32 768 | 1.64 s | 1.63 s |
| `type-exclusivity/pets` | 15 | 11 351 | 32 768 | 0.073 s | 0.35 s |
| `syntax/constraint-scopes` | 12 | 4 096 | 4 096 | 0.014 s | 0.012 s |

**Route 1's worst case is the program where everything is alive.** There it
*is* the oracle, plus the join. The two entries above that cost route 1 as
much as the oracle are R = the whole cube, one sub-cube [∅ … ⊤]. Describing
such an R without visiting it is
[P1e2.1c](../p1e2.1c_the_region_search/README.md)'s duality stage, S1e2.1c.2.

## 3. What a node costs

A visit's cost is its saturation, and the saturation's size is the program's.
Over the programs with the most route-1 work:

| program | facts beyond root, per node | µs per node | ns per fact |
|---|---:|---:|---:|
| `zebra2-minus-15` | 110.7 | 1 276 | 11 526 |
| `zebra.ein` | 111.6 | 885 | 7 924 |
| `zebra2.ein` | 55.8 | 553 | 9 912 |
| `zebra2-hints` | 36.9 | 405 | 10 986 |
| `features/02_star_in_identifiers` | 14.4 | 49 | 3 414 |
| `square-*/` (six) | 12.8 | 21–23 | 1 625–1 748 |
| `syntax/arg-kinds` | 7.7 | 3.2 | 409 |

Per node, the cost spreads **400×** across the corpus. Per fact derived, it
spreads **28×**. So *facts beyond root, summed over the visited nodes*
measures the work far better than *visited nodes* does. It is also a count,
deterministic and machine-free, which a budget has to be (§ 4).

## 4. The budget

The budget is checked before each layer. A declined run has saturated whole
layers `0 … k`, and what it spent is the decline's cost: route 3 starts only
after it. The cumulative per-layer costs of the 2¹⁸-visit sweep give every
smaller budget's outcome exactly. The walls are that sweep's per-layer walls
at one job.

**In visited commitments:**

| budget | describes (`n ≤ 16`, `n > 16`) | declines | worst decline | all declines | all descriptions |
|---|---|---:|---|---:|---:|
| 2¹⁰ | 154 (144, 10) | 20 | 2.18 s `zebra` | 5.15 s | 0.02 s |
| 2¹² | 156 (145, 11) | 18 | 3.33 s `zebra` | 9.13 s | 0.08 s |
| 2¹⁴ | 159 (146, 13) | 15 | 14.96 s `zebra` | 49.11 s | 3.67 s |
| 2¹⁵ | 162 (147, 15) | 12 | 14.96 s `zebra` | 48.85 s | 6.21 s |
| 2¹⁶ | 162 (147, 15) | 12 | 70.46 s `zebra2-minus-15-obligations` | 222.00 s | 6.21 s |
| 2¹⁸ | 162 (147, 15) | 12 | 228.51 s `zebra` | 672.61 s | 6.21 s |

**In facts beyond root.** A layer's facts are known only once it has run, so
the rule that can be implemented *projects* each layer: its size, known from
the join, times the facts per node of the layer before. A run declines when
the facts spent plus that projection pass the budget. It is still a count. The
table is that rule, replayed over the same layers:

| budget | describes (`n ≤ 16`, `n > 16`) | declines | worst decline | all declines | all descriptions |
|---|---|---:|---|---:|---:|
| 2¹⁶ | 157 (145, 12) | 17 | 1.22 s `zebra2` | 4.32 s | 0.21 s |
| 2¹⁷ | 158 (146, 12) | 16 | 5.74 s `zebra2-minus-15-obligations` | 18.78 s | 0.27 s |
| 2¹⁸ | 160 (146, 14) | 14 | 5.74 s `zebra2-minus-15-obligations` | 22.76 s | 1.21 s |
| **2¹⁹** | **162 (147, 15)** | **12** | **5.74 s `zebra2-minus-15-obligations`** | **24.23 s** | **6.21 s** |
| 2²⁰ | 162 (147, 15) | 12 | 10.41 s `zebra2` | 45.06 s | 6.21 s |
| 2²¹ | 162 (147, 15) | 12 | 22.03 s `zebra2` | 89.16 s | 6.21 s |
| 2²² | 162 (147, 15) | 12 | 40.59 s `zebra2` | 142.75 s | 6.21 s |

Two things the curves show:

- **Above the knee, a larger budget buys nothing until the zebra family
  itself fits.** From 2¹⁵ visits, or 2¹⁹ facts, up to 2¹⁸ visits, the
  described set does not move. Only the price of declining grows. The zebra
  family fits at about 2²⁵ facts (§ 6), which costs minutes at one job.
- **Per visit, the price of a decline is dominated by node size.**
  - At 2¹⁶ visits, `zebra2-minus-15` pays 70 s and the six `square-*` cubes
    1 s each, for the same count.
  - At 2¹⁹ facts the worst decline is 5.7 s, and every `square-*` cube
    declines in 0.3 s.

**Proposed default: 2¹⁹ facts beyond root.** It is the smallest budget that
loses nothing the corpus has below the zebra family:

- 162 programs described, 15 of them past the oracle's limit;
- the worst decline is 5.7 s at one job and all twelve cost 24 s together,
  each on a program route 3 then answers partially anyway;
- `--jobs` divides it;
- `--region-budget` raises it for a reader who wants zebra's R (§ 6).

2¹⁸ saves under a second in all and sends `zebra2-hints` (n = 36, 309 545
facts) and `features/02` to the other routes. The library takes a visit budget
today (`downset::Options::budget`), and the unit moves with the ruling. The ruling is the user's ([`solve_flags.md` § 9](solve_flags.md#9-what-needs-the-users-ruling)).

## 5. Past the oracle's limit: the fifteen

Route 1 describes these, and the oracle refuses each one at `n > 16`:

| program | n | visited | alive | \|N\| | k_c | k_s | min | max | facts | wall |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `branching/10_kill_cache_on` | 18 | 76 | 64 | 12 | 64 | 15 | 1 | 1 | 672 | 0.003 s |
| `branching/11_kill_cache_off` | 18 | 76 | 64 | 12 | 64 | 15 | 1 | 1 | 672 | 0.003 s |
| `lattice/06_friends_tables_everyone_sits` | 19 | 345 | 277 | 68 | 8 | 8 | 8 | 8 | 969 | 0.004 s |
| `branching/12_typed_blind_solve` | 25 | 32 | 7 | 25 | 7 | 3 | 1 | 2 | 356 | 0.003 s |
| `zebra2-hints` | 36 | 8 389 | 8 316 | 73 | 8 316 | 6 | 1 | 2 | 309 545 | 3.409 s |
| `branching/08_hypothesis_relation_whitelist` | 38 | 5 946 | 5 782 | 164 | 5 782 | 34 | 1 | 8 | 55 506 | 0.131 s |
| `branching/04_two_levels` | 46 | 85 | 31 | 54 | 31 | 7 | 1 | 2 | 284 | 0.001 s |
| `branching/02_one_dead_one_alive` | 48 | 2 164 | 2 062 | 102 | 2 062 | 20 | 1 | 4 | 19 502 | 0.045 s |
| `ein-bugs/alt-cap-core-reordered` | 70 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0.000 s |
| `ein-bugs/alt-cap-core` | 70 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0.000 s |
| `features/09_adjacent_via_same_house` | 82 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0.000 s |
| `ein-bugs/zebra2-hints` | 90 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0.001 s |
| `branching/03_five_hyps_one_alive` | 132 | 134 | 4 | 130 | 4 | 2 | 1 | 1 | 442 | 0.003 s |
| `branching/06_lookahead_on` | 204 | 19 192 | 18 622 | 570 | 18 622 | 74 | 1 | 22 | 180 962 | 0.481 s |
| `branching/07_lookahead_off` | 204 | 19 192 | 18 622 | 570 | 18 622 | 74 | 1 | 22 | 180 962 | 0.483 s |

- **Four have a dead root** (`|N| = 1`, the empty set): R is empty at
  `n = 70…90` for one saturation. Today the walk reports them through its own
  machinery; route 1 certifies them.
- **`n` is not what makes a program expensive.** `branching/06` at
  `n = 204` costs 0.5 s, because its alive set is 18 622 small nodes.
  `zebra2-hints` at `n = 36` costs 3.4 s for half as many nodes, each derived
  ten times as far.

## 6. Where route 1 declines, and zebra's region counted

Two families, and they decline for opposite reasons.

**Everything alive: `square-*` (six, n = 20) and `syntax/arg-kinds`
(n = 19).** Every commitment visited up to the budget is alive and in R. The
2¹⁸-visit sweep saw layers that are exactly the binomials, C(20, k) and
C(19, k). The cube is presumably all of R, one sub-cube of 2²⁰ or 2¹⁹ nodes.
"Presumably" because route 1 did not finish, and the oracle refuses at
`n > 16`. Describing it costs 2ⁿ by enumeration and one sub-cube by duality,
which is the case P1e2.1c's S1e2.1c.2 exists for. Meanwhile route 3, the walk
at `-m 5`, reports `Contradiction`, *"no model found"*, for all six `square-*`
programs (§ 7).

**The zebra family: the alive set is finite, and now counted.** At
2²⁰ visits on 12 jobs, route 1 finishes `zebra.ein`, `zebra2.ein` and
`zebra2-obligations.ein`:

| | `zebra` | `zebra2` | `zebra2-obligations` |
|---|---:|---:|---:|
| n | 56 | 56 | 56 |
| visited | 270 912 | 270 579 | 270 579 |
| alive | 270 454 | 270 213 | 270 213 |
| \|N\| | 458 | 366 | 366 |
| **k_c** | **261 982** | **261 982** | **261 982** |
| **k_s** | **1** | **1** | **1** |
| minimal line | 63 | 63 | 63 |
| maximal line | 1 | 1 | 1 |
| components | 1 | 1 | 1 |
| facts beyond root, summed | 30 251 060 | 15 094 278 | 15 094 278 |
| wall, 12 jobs | 25.2 s | 17.3 s | 18.5 s |

Its layers (visited, dead, owing, in R), for `zebra.ein`:

| size | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| visited | 1 | 56 | 703 | 1 303 | 4 085 | 10 187 | 20 433 | 33 415 | 44 749 | 49 060 | 43 890 | 31 848 | 18 566 | 8 568 | 3 060 | 816 | 153 | 18 | 1 |
| dead | 0 | 18 | 414 | 26 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| owing | 1 | 34 | 168 | 506 | 1 065 | 1 641 | 1 876 | 1 592 | 991 | 440 | 132 | 24 | 2 | 0 | 0 | 0 | 0 | 0 | 0 |
| R | 0 | 4 | 121 | 771 | 3 020 | 8 546 | 18 557 | 31 823 | 43 758 | 48 620 | 43 758 | 31 824 | 18 564 | 8 568 | 3 060 | 816 | 153 | 18 | 1 |

What it says, in R's terms:

- **The puzzle has exactly one solution state, certified**: k_s = 1, by the
  static verdict, `N` (458 minimal dead sets, every one of size ≤ 3) and the
  minimal line. Today `ein solve -e zebra.ein` says the same by the walk, and
  the walk's `exhausted` certifies the walk
  ([Q-M1e.8](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)).
  This certifies R.
- **∅ owes, so ∅ ∉ R**. The maximal line is one commitment of 18 facts: the
  18 candidates the solution makes true. Layers 13–18 are exactly C(18, k):
  every commitment of 13 or more facts that is alive is a subset of the
  solution.
- **k_c = 261 982 = 2¹⁸ − 162.** All but 162 of the solution's 2¹⁸ subsets
  already determine it. The minimal line is the **63 smallest commitments that
  determine the puzzle**, and four of them are single facts. That is the
  third answer's `(merged (to from) …)` with one `to` and 63 `from`s.
- `zebra2`'s R is `zebra`'s, through a different encoding. It has 92 fewer
  minimal dead sets and derives half the facts per node.

`zebra2-minus-15` (n = 96, the puzzle with clue 15 removed, 32 models in the
walk) is the family's other half, and it is not countable this way. At 2²¹
visits on 12 jobs, route 1 declines at size 6 after 154.6 s:

| size | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---:|---:|---:|---:|---:|---:|---:|
| visited | 1 | 96 | 4 560 | 50 223 | 237 420 | 930 831 | *2 902 185, not visited* |
| dead | 0 | 0 | 1 428 | 13 269 | 11 799 | 1 300 | |
| owing | 1 | 96 | 2 911 | 26 684 | 117 658 | 343 542 | |
| R | 0 | 0 | 221 | 10 270 | 107 963 | 585 989 | |

At sizes 0–5 alone, R holds **704 443** commitments, and each layer is 2.5–4×
the last. Zebra's peaks at size 9 with 49 060. One clue fewer turns a region
of one state into one whose commitment count is in the millions before
size 6. That is the distance between k_s and k_c that S1e2.1a.5's read-out
has to print.

**So the zebra family is not past route 1. It is past route 1's default
budget.** That separates P1e2.1c's question, *how large the alive set
gets* (S1e2.1c.1), from *whether a default should pay for it*.

## 7. Route 3's cost: the walk on the twenty

`ein solve FILE` and `ein solve -e FILE`, release build, one job, 120 s
timeout. Enterings, verdict, k and `exhausted` are from `--json-summary`.

| program | `solve` (`-n 1`) | `solve -e` | enterings | verdict | exhausted |
|---|---:|---:|---:|---|---|
| `features/01_not_and_absent` † | 1.74 s | 1.74 s | 384 167 | Contradiction, k = 0 | false |
| `features/04_open` † | 98.1 s | 97.9 s | 27 373 977 | Contradiction, k = 0 | false |
| `features/05_stdlib_domain_elim` † | 3.02 s | 3.02 s | 384 167 | Contradiction, k = 0 | false |
| `features/12_expect_false` † | 0.005 s | 0.005 s | 0 | Contradiction, k = 0 | true |
| `lattice/06_friends_tables` † | 0.007 s | 0.012 s | 172 / 343 | Open / Ambiguity k = 8 | false / true |
| `square-bwd/*`, `square-fwd/*` (six) | 0.26 s each | 0.26 s each | 21 699 | Contradiction, k = 0 | false |
| `square-unique/corner-house` †, `cul-de-sac` †, `terminus` † | > 120 s | > 120 s | — | timed out | — |
| `zebra2-minus-15`, `-obligations` | | 1 496 s | 17 204 592 | Ambiguity, k = 32 | true |
| `zebra.ein` | 0.025 s | 0.079 s | 13 / 111 | Solution, k = 1 | false / true |
| `zebra2`, `zebra2-obligations` | 0.019 s | 0.048 s | 11 / 101 | Solution, k = 1 | false / true |

† flagged non-monotone. The `zebra2-minus-15` row is
[M1d's](../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)
measurement and was not re-taken.

What it says for the default:

- **Route 3 needs a budget of its own.** Today the walk has none: no `-E`
  default, and `-e` on `zebra2-minus-15` takes 25 minutes. [`solve_flags.md`
  § 10](solve_flags.md#10-decided-here-unless-the-user-objects) gives route 3
  an enterings budget and makes hitting it a *partial* description, not an
  abort.
- **Where it reaches, route 3 is often wrong in R's terms**, and says so only
  through `exhausted = false`. On the six `square-*` cubes it reports `k = 0`.
  Route 1, at 2¹⁸ visits, saw 137 980 commitments there, every one of them
  alive and in R, ∅ among them. That is the walk's `-m 5` truncation, W-class
  in shape. It is
  P1e2.1c's problem and P1e2.2's register's, not this stage's, and the
  read-out stage words it
  ([S1e2.1a.5](s1e2.1a.5_the_read_out.md)).

## 8. What this settles

- **T1e2.1a.9.1's question.** Route 1 is route 1 on 162 of 195 programs at a
  2¹⁹-fact budget, and on every monotone program the oracle reaches. The
  default budget is the user's, with § 4 on the table.
- **For [P1e2.1c](../p1e2.1c_the_region_search/README.md):**
  - **S1e2.1c.1** has its first answer: the zebra family's alive set is
    270 454, and R is one state.
  - **S1e2.1c.2** has its cases: the seven everything-alive cubes, and
    `features/02`.
- **For the register**, nothing yet. Route 1 checks R and not the walk.
  S1e2.1a.6 compares the two.
