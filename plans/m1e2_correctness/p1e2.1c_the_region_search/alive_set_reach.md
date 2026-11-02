# How large the alive set gets — 2026-11-03

**T1e2.1c.1's measurement**, for
[S1e2.1c.1](s1e2.1c.1_how_large_the_alive_set_gets.md). Route 1
([`ein_infer::downset`](../../../ein.rs/crates/ein-infer/src/downset.rs)) runs
past its default budget on the twelve monotone programs it declines at the
default, and on the monotone slow entries. Per layer it counts the commitments
visited, dead, owing and in R, **and the distinct states among them**. The
page then decides the everything-alive cubes and tables what a description
costs as the alive set grows, which is the table
[S1e2.1c.2](s1e2.1c.2_the_lines_by_duality.md) and
[S1e2.1c.4](s1e2.1c.4_the_flags_at_scale.md) are measured against. It lists
the flagged programs past the limit with what they get today.
[`route1_reach.md`](../p1e2.1a_the_solution_region/route1_reach.md) is
P1e2.1a's record and the first answer. This page is the second, and it does
not edit the first.

**Status:** T1e2.1c.1.1–.2 written 2026-11-03: §§ 1–3. The cubes and the cost
table follow with T1e2.1c.1.3–.4.

**This stage measures.** No golden, `:expect` or register row moved. The
instrument is example code that no route reads:
[`route1_reach`](../../../ein.rs/crates/ein-infer/examples/route1_reach.rs)
gained ⊤'s status and the facts summed (T1e2.1c.1.1), then the per-layer state
count, the alive states' one-up extensions and the generator total
(T1e2.1c.1.2). Every one of them is read off what the route already computes.

```sh
./build.sh                                                   # the release ein, snmalloc linked
(cd ein.rs && cargo build --release -p ein-infer --example route1_reach)   # the instrument, system allocator
R=ein.rs/target/release/examples/route1_reach

# § 1: every corpus file and register witness, at the default budget (2^19 facts)
utils/bench_env.sh --cores PT $R --budget 18446744073709551615 --facts 524288 --jobs 16 > default.jsonl

# § 2, § 3: past the budget, one process per program, under the cap
systemd-run --user --scope -q -p MemoryMax=11G -p MemorySwapMax=0 \
  python3 peak.py OUT.jsonl -- timeout 900 \
  utils/bench_env.sh --cores PT $R --budget 4194304 --jobs 16 FILE
```

**Taken 2026-11-03 on `d2b6fdf0`.** The engine and `downset` are that
commit's. The instrument is this stage's, which adds counts and changes no
visit. Release builds. `utils/bench_env.sh`'s machine state, as each run
printed it:

- an Intel i9-14900HX, pinned with `--cores PT` to cpu0–15: 16 threads on
  8 physical P-cores;
- governor `powersave` (epp `balance_performance`), turbo on;
- 31 GiB and no swap; `--jobs 16` on every run unless a row says otherwise.

`route1_reach` links the system allocator and the `ein` binary links
snmalloc. One JSON line per program, and the tables were read off the lines.

**The ceiling** is the same for every run past the budget:

- `--budget 4194304`, which is 2²² visited commitments and no facts budget;
- `timeout 900` on the wall;
- `MemoryMax=11G` on memory.

Peak RSS is `wait4`'s `ru_maxrss`, read by `peak.py`, which is
[S1e2.4.2's](../p1e2.4_what_the_switch_left/s1e2.4.2_models_key_declines.md#how-it-is-measured)
verbatim. No run reached the wall or the memory cap. Two reached the visit
budget, the `zebra2-minus-15` pair, at layer 7 (§ 2.3).

---

## 1. The selection, re-taken

`default.jsonl` is every file
[`corpus_files()`](../../../ein.rs/crates/ein-corpus/src/lib.rs) lists, plus
the register's witnesses, at the default budget of 2¹⁹ facts. That is
**219 rows**. Nine do not compile, which leaves **210 programs**:

| | programs | of which `n > 16` |
|---|---:|---:|
| **route 1** describes R | **172** | 16 |
| … monotone | 171 | 15 |
| … flagged, with a dead root (L0) | 1 | 1 |
| **route 2**, the cube (flagged, `n ≤ 16`) | 19 | — |
| **route 3**, neither: partial | 19 | 19 |
| … monotone, past the budget | 12 | 12 |
| … flagged, past `--region-limit` | 7 | 7 |
| | **210** | |

183 programs are monotone and 27 are flagged.
[`route1_reach.md` § 1](../p1e2.1a_the_solution_region/route1_reach.md#1-the-three-routes-over-the-corpus)
counted 195 programs on 2026-10-16, split 162 / 13 / 20. What moved since:

- **The selection grew by 15 programs.** Route 1 grew by 10, `features/12`
  among them, and route 2 by 6.
- **`features/12_expect_false` moved from route 3 to route 1.** It is flagged,
  and its root is dead. Since the switch (`fbcae3b2`), a dead root decides a
  flagged program too, by L0: a fork only adds.
- **The twelve monotone programs past the budget are the same twelve**:
  - the six `square-*` cubes;
  - `syntax/arg-kinds`;
  - `zebra`, `zebra2`, `zebra2-obligations`;
  - `zebra2-minus-15` and its `-obligations` twin.
- **The fifteen monotone programs route 1 describes past the oracle's limit
  are the same fifteen** as
  [§ 5 there](../p1e2.1a_the_solution_region/route1_reach.md#5-past-the-oracles-limit-the-fifteen).
- **Every monotone program at `n ≤ 16` is still route 1's**: 156 of 156.

**The slow entries** (`corpus/corpus.toml`, `slow = true`) are the same ten.
Two are flagged: `features/01` and `lattice/06_friends_tables`. Of the
eight monotone ones, five are among the twelve, the zebra family. The other
three finish at the default:

- `features/02_star_in_identifiers`, n = 15;
- `lattice/07_two_pairs_two_tables`, n = 16;
- `zebra2-hints`, n = 36.

So route 1 runs past its budget on **fifteen programs**: the twelve, plus
those three for completeness.

## 2. Route 1 past its budget

### 2.1 The fifteen

At the ceiling, one process each:

| program | n | layers | visited | alive | \|N\| | k_c | k_s | facts beyond root | wall | peak RSS |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `lattice/07_two_pairs_two_tables` | 16 | 0–4 | 177 | 113 | 64 | 8 | 8 | 448 | 6.3 ms | 7 MiB |
| `zebra2-hints` | 36 | 0–13 | 8 389 | 8 316 | 73 | 8 316 | 6 | 309 545 | 0.50 s | 31 MiB |
| `features/02_star_in_identifiers` | 15 | 0–15 | 32 768 | 32 768 | 0 | 32 768 | 728 | 470 616 | 0.40 s | 18 MiB |
| `zebra2` | 56 | 0–18 | 270 579 | 270 213 | 366 | 261 982 | 1 | 15 094 278 | 17.86 s | 138 MiB |
| `zebra2-obligations` | 56 | 0–18 | 270 579 | 270 213 | 366 | 261 982 | 1 | 15 094 278 | 18.47 s | 138 MiB |
| `zebra` | 56 | 0–18 | 270 912 | 270 454 | 458 | 261 982 | 1 | 30 251 060 | 26.08 s | 145 MiB |
| `syntax/arg-kinds` | 19 | 0–19 | 524 288 | 524 288 | 0 | 524 288 | 524 288 | 4 980 736 | 1.81 s | 328 MiB |
| `square-bwd/floors` | 20 | 0–20 | 1 048 576 | 1 048 576 | 0 | 1 048 576 | 27 559 | 18 468 768 | 11.42 s | 446 MiB |
| `square-bwd/houses` | 20 | 0–20 | 1 048 576 | 1 048 576 | 0 | 1 048 576 | 27 559 | 18 468 768 | 12.86 s | 448 MiB |
| `square-bwd/meetings` | 20 | 0–20 | 1 048 576 | 1 048 576 | 0 | 1 048 576 | 27 559 | 18 468 768 | 13.35 s | 446 MiB |
| `square-fwd/floors` | 20 | 0–20 | 1 048 576 | 1 048 576 | 0 | 1 048 576 | 27 559 | 18 468 768 | 13.22 s | 447 MiB |
| `square-fwd/houses` | 20 | 0–20 | 1 048 576 | 1 048 576 | 0 | 1 048 576 | 27 559 | 18 468 768 | 12.37 s | 447 MiB |
| `square-fwd/meetings` | 20 | 0–20 | 1 048 576 | 1 048 576 | 0 | 1 048 576 | 27 559 | 18 468 768 | 13.09 s | 447 MiB |
| `zebra2-minus-15` *(not finished)* | 96 | 0–6; 7 not visited | 4 125 316 | 4 097 512 | ≥ 27 804 | ≥ 2 840 307 | ≥ 32 | 378 138 052 | 494.97 s | 3.59 GiB |
| `zebra2-minus-15-obligations` *(not finished)* | 96 | 0–6; 7 not visited | 4 125 316 | 4 097 512 | ≥ 27 804 | ≥ 2 840 307 | ≥ 32 | 378 138 052 | 493.49 s | 3.59 GiB |

*Wall* is route 1's, in process, at `--jobs 16`: root, every layer's
saturations and the description. *Peak RSS* is the whole process's. *facts
beyond root* is summed over the visited fixpoints, with ⊤'s added on a run
that declined, which is describe's `work` row. A `≥` is a count of what the
run met, labelled as a floor. It is not a total, and it is not extrapolated.

- **Thirteen finish.** The zebra family's three finish exactly as
  [`route1_reach.md` § 6](../p1e2.1a_the_solution_region/route1_reach.md#6-where-route-1-declines-and-zebras-region-counted)
  counted them on 2026-10-16, to the commitment and layer for layer. That
  includes `zebra`'s 270 912 visits, its k_c of 261 982 and its one state.
- **The seven everything-alive cubes finish.** Each layer is exactly the
  binomial, C(20, k) or C(19, k). Every node is alive, discharged and in R,
  so the cube is all of R, k_c = 2ⁿ, and N is empty. The *"presumably"* of 2026-10-16 is gone.
- **The `zebra2-minus-15` pair does not finish** (§ 2.3). Each reached the
  ceiling's 2²² visits before layer 7, in 8.3 minutes and 3.6 GiB.
- **Memory is a ceiling too.** Route 1 keeps every point it visits: its set,
  its closure, its owed instances and its state's key. That costs about
  0.9 KiB a point on the n = 96 pair and 0.4 KiB on the cubes.

### 2.2 What the finished runs' layers hold

`zebra`'s layers are in [`route1_reach.md` § 6](../p1e2.1a_the_solution_region/route1_reach.md#6-where-route-1-declines-and-zebras-region-counted)
and are not repeated here. § 3 prints them again, beside their states.

- **`zebra2` and `zebra2-obligations` agree layer for layer**, in every
  column. The twin deletes the `(hrule …)` and the query's `:hrules` clause,
  and neither reaches a node.
- **They agree with `zebra` in R, but not in what they visit.** R is the same
  at every size, and so is the maximal line's 18-fact commitment. The dead
  commitments differ at sizes 1–3: `zebra2` has 92 fewer minimal dead sets.
  The owing ones differ at sizes 1–8. Each node of `zebra2` derives half
  `zebra`'s facts.

### 2.3 `zebra2-minus-15` and its twin: not finished

n = 96, the puzzle with clue 15 removed. Its last complete layer is **6**.
The ceiling stopped it before layer 7, whose size the join had already
computed. The twin, `zebra2-minus-15-obligations`, agrees with it layer for
layer and is not printed twice.

| size | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| visited | 1 | 96 | 4 560 | 50 223 | 237 420 | 930 831 | 2 902 185 | *7 214 983: the join, not visited* |
| dead | 0 | 0 | 1 428 | 13 269 | 11 799 | 1 300 | 8 |  |
| owing | 1 | 96 | 2 911 | 26 684 | 117 658 | 343 542 | 766 313 |  |
| in R | 0 | 0 | 221 | 10 270 | 107 963 | 585 989 | 2 135 864 |  |

**Growth of the last layers**, as ratios of the counts above:

| from layer → to layer | 2 → 3 | 3 → 4 | 4 → 5 | 5 → 6 | 6 → 7 |
|---|---:|---:|---:|---:|---:|
| visited | 11.0 | 4.73 | 3.92 | 3.12 | 2.49, the join |
| in R | 46.5 | 10.5 | 5.43 | 3.64 | — |
| owing | 9.17 | 4.41 | 2.92 | 2.23 | — |
| dead | 9.29 | 0.89 | 0.11 | 0.006 | — |

- **What is certified.** Every count in layers 0–6 is exact. Layer 7's
  7 214 983 is exact too, as the number of 7-commitments whose every
  6-subset is alive. The run did not saturate them, so how many are alive is
  not known. Over sizes 0–6, R holds **2 840 307** commitments. The alive set
  holds 4 097 512. Of those, 704 443 are R's commitments of at most 5
  candidates, as `route1_reach.md` § 6 found.
- **What is not.** k_c, the lines, N past size 6 and every total. The growth
  ratios fall at every layer, but how far the layers run is not known, and
  this page prints no count for it.
- **Dead nearly stops.** N gains 8 members at size 6, against 13 269 at
  size 3. So almost every 6-commitment whose 5-subsets are all alive is alive
  itself. The layers are wide because the alive set is, not because N cuts
  them.
- Layer 7 alone holds more commitments than layers 0–6 together (4 125 316).
  Under this ceiling it was not visited.

## 3. States beside commitments

The states come from the same run as the commitments. Every visited point
carries its **class**: the first visited point whose fixpoint is the same
(`downset`'s `Point::class`, which route 1 counts k_s by). Per layer the
instrument counts three things:

- **alive states**: the distinct classes among the layer's alive points;
- **states in R**: the distinct classes among its members of R;
- **first met**: the alive points that open a class, i.e. the alive states
  no smaller commitment reached.

Summed over the layers, *first met* is the number of distinct alive states.
A state can recur in many layers, so the per-layer counts do not add up.

### 3.1 `zebra.ein`: 27 alive states, all of them met by layer 2

| size | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| visited | 1 | 56 | 703 | 1 303 | 4 085 | 10 187 | 20 433 | 33 415 | 44 749 | 49 060 | 43 890 | 31 848 | 18 566 | 8 568 | 3 060 | 816 | 153 | 18 | 1 |
| in R | 0 | 4 | 121 | 771 | 3 020 | 8 546 | 18 557 | 31 823 | 43 758 | 48 620 | 43 758 | 31 824 | 18 564 | 8 568 | 3 060 | 816 | 153 | 18 | 1 |
| **alive states** | 1 | 17 | 25 | 18 | 14 | 11 | 8 | 6 | 5 | 3 | 3 | 3 | 3 | 1 | 1 | 1 | 1 | 1 | 1 |
| **states in R** | 0 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 |
| **first met** | 1 | 17 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

- **Every alive state has a generator of at most 2 candidates.** Root's state
  is first met at layer 0, 17 states at layer 1 and 9 at layer 2. The
  270 126 alive commitments of sizes 3–18 reach no state that those 27 had
  not already reached.
- **Each layer is a handful of states.** Layer 9's 49 060 commitments are
  3 alive states. From layer 13 up, every commitment is the solution's state.
- **`zebra2` is the same shape with 20.** It meets 1, 14 and 5 states at
  layers 0–2 and none after.

### 3.2 `zebra2-minus-15`: 4 709 alive states through layer 6, none new at layer 6

| size | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---:|---:|---:|---:|---:|---:|---:|
| alive | 1 | 96 | 3 132 | 36 954 | 225 621 | 929 531 | 2 902 177 |
| in R | 0 | 0 | 221 | 10 270 | 107 963 | 585 989 | 2 135 864 |
| **alive states** | 1 | 61 | 1 046 | 3 357 | 4 170 | 3 631 | 2 823 |
| **states in R** | 0 | 0 | 28 | 32 | 32 | 32 | 32 |
| **first met** | 1 | 61 | 1 003 | 2 420 | 1 146 | 78 | 0 |

- **The 32 states of R are all met by layer 3**, and every layer from 3 to 6
  holds all 32. That is as many as the 32 models M1d's tree found. Route 1
  does not certify that there are no more: it did not finish.
- **New alive states rise, then stop.** They go 61, 1 003, 2 420, then 1 146
  and 78, and then **0 at layer 6**. Layer 6's 2 902 177 alive commitments
  are 2 823 states, every one already met at size ≤ 5.
- **What that licenses, and what it does not.** On a monotone program, a
  state's minimal generators can have any size. So a 7-candidate generator of
  a new state is not excluded by layer 6 meeting none. *4 709 alive states
  have a generator of at most 6 candidates* is a count. *The alive set has
  4 709 states* is not, and this page does not print it.
- Against the commitments: 4 097 512 alive commitments, and **870 times
  fewer** alive states over the same sizes.

### 3.3 The cubes: states fall behind commitments from layer 2

| size | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| visited | 1 | 20 | 190 | 1 140 | 4 845 | 15 504 | 38 760 | 77 520 | 125 970 | 167 960 | 184 756 | 167 960 | 125 970 | 77 520 | 38 760 | 15 504 | 4 845 | 1 140 | 190 | 20 | 1 |
| **alive states** | 1 | 20 | 188 | 1 063 | 3 738 | 8 472 | 13 350 | 15 765 | 14 999 | 12 300 | 9 186 | 6 385 | 4 080 | 2 381 | 1 320 | 738 | 396 | 176 | 56 | 11 | 1 |
| **first met** | 1 | 20 | 185 | 987 | 3 089 | 5 794 | 6 931 | 5 561 | 3 130 | 1 314 | 429 | 102 | 15 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

- On the six `square-*` programs, the distinct states per layer peak at
  layer 7 with 15 765, against layer 10's commitment peak of 184 756. New
  states stop after layer 13, so every state of R has a generator of at
  most 13 candidates.
- **`arg-kinds` has no such gap.** Every layer's states equal its
  commitments, C(19, k), and every state is first met in its own layer.
- **`features/02_star_in_identifiers`** (n = 15, also R = the cube) sits
  between the two: 728 states for 32 768 commitments, new ones stopping
  after layer 9.

### 3.4 The state route's input, counted

[`walk_and_region.md` § 6](../p1e2.1a_the_solution_region/walk_and_region.md#6-what-this-means-for-question-3)
bounded a route over states by *alive states × n* saturations. S1e2.1c.2
words that route as *"every alive state visited once, its one-up extensions
saturated"*. Its saturations are then root plus the alive states' distinct
one-up extensions: each alive state's candidates plus one candidate it does
not hold. That is a count the instrument takes (`extensions`):

| program | n | route 1 visits | alive states | × n, § 6's bound | root + extensions, counted | route 1 ÷ state route |
|---|---:|---:|---:|---:|---:|---:|
| `zebra` | 56 | 270 912 | 27 | 1 512 | 1 375 | 197.0 |
| `zebra2`, `zebra2-obligations` | 56 | 270 579 | 20 | 1 120 | 1 016 | 266.3 |
| `zebra2-hints` | 36 | 8 389 | 6 | 216 | 191 | 43.9 |
| `branching/08_hypothesis_relation_whitelist` | 38 | 5 946 | 34 | 1 292 | 1 099 | 5.4 |
| `branching/02_one_dead_one_alive` | 48 | 2 164 | 20 | 960 | 867 | 2.5 |
| `features/02_star_in_identifiers` | 15 | 32 768 | 728 | 10 920 | 2 253 | 14.5 |
| `square-*` (six) | 20 | 1 048 576 | 27 559 | 551 180 | 151 141 | 6.9 |
| `branching/06`, `/07` | 204 | 19 192 | 74 | 15 096 | 14 595 | 1.3 |
| `syntax/constraint-scopes` | 12 | 4 096 | 4 096 | 49 152 | 4 096 | 1.0 |
| `syntax/arg-kinds` | 19 | 524 288 | 524 288 | 9 961 472 | 524 288 | 1.0 |
| `type-exclusivity/pets` | 15 | 11 351 | 11 339 | 170 085 | 26 192 | 0.4 |
| `lattice/07_two_pairs_two_tables` | 16 | 177 | 113 | 1 808 | 1 073 | 0.2 |
| `lattice/06_friends_tables_everyone_sits` | 19 | 345 | 277 | 5 263 | 3 045 | 0.1 |
| `zebra2-minus-15`, its twin *(not finished)* | 96 | ≥ 4 125 316 | ≥ 4 709 | ≥ 452 064 | ≥ 411 210 | — |

- **Where states collapse, the route wins by two orders of magnitude.** On
  the zebra family it saturates 1 016–1 375 nodes where route 1 saturates
  270 579–270 912.
- **Where states do not collapse, it breaks even or loses.**
  - With N empty and every commitment its own state, it breaks even, at 2ⁿ
    either way: `arg-kinds`, `constraint-scopes`.
  - With N non-empty, it loses. `type-exclusivity/pets` costs 2.3 times
    route 1, `lattice/07_two_pairs_two_tables` 6.1 times, and
    `lattice/06_friends_tables_everyone_sits` 8.8 times.

  The reason is that an extension is saturated even when it contains a dead
  set. Route 1's join never generates such a commitment (L1), and the count
  above does not use L1. S1e2.1c.2's *"wins where commitments collapse into
  few states, and loses where every commitment is its own state"* is measured
  here, crossover included. What pruning by the dead sets found so far would
  save is the route's to measure.
- `zebra2-minus-15`'s row counts what its 4 709 states met by layer 6 extend
  to. It is a floor, because the alive set past layer 6 is not counted.
