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

**Status:** done 2026-11-03, T1e2.1c.1.1–.4, one commit each. **The second
half**, § 9, is
[S1e2.1c.2](s1e2.1c.2_the_lines_by_duality.md)'s measurement of the state
route against this page's table, taken the same day.

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

# § 1, § 5, § 8: every corpus file and register witness, at the default budget (2^19 facts)
utils/bench_env.sh --cores PT $R --budget 18446744073709551615 --facts 524288 --jobs 16 > default.jsonl

# § 2, § 3: past the budget, one process per program, under the cap
systemd-run --user --scope -q -p MemoryMax=11G -p MemorySwapMax=0 \
  python3 peak.py OUT.jsonl -- timeout 900 \
  utils/bench_env.sh --cores PT $R --budget 4194304 --jobs 16 FILE

# § 4: two saturations, ∅ and ⊤, on every program
utils/bench_env.sh --cores PT $R --budget 1 --jobs 1 > two.jsonl

# § 5: describe by the binary, at the smallest budget that describes
utils/bench_env.sh --cores PT ein.rs/target/release/ein solve FILE --region-budget F -j 16 -s

# § 6: the flagged programs past the limit, as a bare run prints them
ein.rs/target/release/ein solve FILE -s
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
snmalloc, so § 5 gives the binary's wall beside the instrument's where it
matters. One JSON line per program, and the tables were read off the lines.

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
Two are flagged: `features/01` and `lattice/06_friends_tables` (§ 6). Of the
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
  so the cube is all of R, k_c = 2ⁿ, and N is empty. § 4 decides the same in
  two saturations. The *"presumably"* of 2026-10-16 is gone.
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

## 4. The everything-alive cubes, decided

**The argument** is [`read_out.md` § 6.2](../p1e2.1a_the_solution_region/read_out.md#62-a-partial-run-c3)'s
X11, applied to every program and not just one. On a monotone program, dead
is upward-closed (L1), so alive is downward-closed: **⊤ alive makes every node
alive**. Discharged is upward-closed (L2), so **∅ discharged makes every node
discharged**. Both together put every node in R: R is the one sub-cube
[∅ … ⊤], k_c = 2ⁿ, N = ∅, and both lines are trivial. Two saturations decide
it, ∅'s (root) and ⊤'s, and the test is exact both ways:

- if ⊤ is dead, ⊤ is outside R;
- if ∅ owes or is dead, ∅ is outside R.

`route1_reach --budget 1` runs exactly those two saturations. It visits layer 0
and declines layer 1, and the route then saturates ⊤ once.

**Checked on every program, in both directions.** Over the 183 monotone
programs, the two saturations decide R = the cube on **85**. Those 85 are
exactly the programs whose k_c, counted by route 1 to the end (§ 1's sweep,
§ 2's runs), is 2ⁿ.
No program is decided without being the cube, and none is the cube without
being decided. That holds on the 181 that route 1 counts. On the
`zebra2-minus-15` pair, which it does not count, ⊤ is dead, so they are not
the cube, and nothing contradicts that. Of the 98 programs not decided, 60
fail on ∅, which owes or is dead, and 56 on ⊤, which is dead; 18 fail on both.

Of the 85, 60 have n = 0, where ∅ is ⊤ and one saturation decides, and 74 have
n ≤ 3. The other eleven are `features/03_forall` (n = 5), `syntax/equality`
(6), `syntax/constraint-scopes` (12), `features/02_star_in_identifiers` (15),
and **the seven past the oracle's limit**:

| program | n | ∅ | ⊤ | the two saturations | k_c, counted | k_s, counted | the count's cost |
|---|---:|---|---|---:|---:|---:|---|
| `square-bwd/floors` | 20 | in R | alive, discharged, 24 facts | 0.13 ms | 1 048 576 = 2²⁰ | **27 559** | 1 048 576 saturations · 18 468 768 facts · 11.4 s |
| `square-bwd/houses` | 20 | in R | alive, discharged, 24 facts | 0.13 ms | 2²⁰ | **27 559** | the same · 12.9 s |
| `square-bwd/meetings` | 20 | in R | alive, discharged, 24 facts | 0.13 ms | 2²⁰ | **27 559** | the same · 13.4 s |
| `square-fwd/floors` | 20 | in R | alive, discharged, 24 facts | 0.13 ms | 2²⁰ | **27 559** | the same · 13.2 s |
| `square-fwd/houses` | 20 | in R | alive, discharged, 24 facts | 0.13 ms | 2²⁰ | **27 559** | the same · 12.4 s |
| `square-fwd/meetings` | 20 | in R | alive, discharged, 24 facts | 0.13 ms | 2²⁰ | **27 559** | the same · 13.1 s |
| `syntax/arg-kinds` | 19 | in R | alive, discharged, 19 facts | 0.03 ms | 524 288 = 2¹⁹ | **524 288** | 524 288 saturations · 4 980 736 facts · 1.81 s |

The two saturations are `two.jsonl`'s `route1_reach --budget 1 --jobs 1`. The
count is route 1 to the end at the ceiling, `--jobs 16` (§ 2.1).

- **All seven are decided, and all seven are counted.** k_c = 2ⁿ by the two
  saturations, and route 1's count agrees on each. k_s needs the states, and
  the count finished on every one of them.
- **The six `square-*` programs are one region six times.** They agree in
  every column the instrument prints: k_s, the 29 711 generators, the
  23 896 merged states, the 151 140 extensions, the facts per layer. They
  differ only in their wall. So [`solve_interface.md`
  § 3.4](../p1e2.1a_the_solution_region/solve_interface.md#34-what-it-costs)'s
  27 559 for `square-bwd/floors` holds for all six.
- **On `arg-kinds` every commitment is its own state**: k_s = k_c = 2¹⁹, and no
  state has a second generator. A route over states saves nothing there (§ 3).

**What describe prints today.** At the default budget, describe stops on all
seven after layer 5 (`square-*`) or layer 6 (`arg-kinds`), having saturated
∅ and then ⊤ once. Both trivial lines are certified, as X11 shows. Yet it
prints k_c and k_s as lower bounds: *21 701* and *10 077* on the squares,
*43 797* and *43 797* on `arg-kinds`. The two saturations it has already made
decide k_c = 2ⁿ, and nothing reads them for that. That is
[S1e2.1c.2](s1e2.1c.2_the_lines_by_duality.md)'s case, *"described in a
handful of saturations, with k_s counted or the read-out saying it is not"*,
and its input is here. Printing it is not this stage's: the stage measures.

## 5. What a description costs as the alive set grows

**The table S1e2.1c.2 and S1e2.1c.4 are measured against.** Here are the
monotone programs with at least 100 alive commitments, and the twelve past
the budget, in order of |alive|. § 8 has every monotone program with
n ≥ 1, in the same columns.

| program | n | \|alive\| | alive states | extensions | k_c | k_s | min | max | generators | \|N\| | saturations | facts | wall | default |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `lattice/07_two_pairs_two_tables` | 16 | 113 | 113 | 1 072 | 8 | 8 | 8 | 8 | 8 | 64 | 177 | 448 | 6.3 ms | yes |
| `type-exclusivity/colors` | 8 | 169 | 169 | 246 | 169 | 169 | 1 | 9 | 169 | 4 | 173 | 644 | 828 µs | yes |
| `type-exclusivity/nationalities` | 8 | 169 | 169 | 246 | 169 | 169 | 1 | 9 | 169 | 4 | 173 | 644 | 766 µs | yes |
| `lattice/06_friends_tables_everyone_sits` | 19 | 277 | 277 | 3 044 | 8 | 8 | 8 | 8 | 8 | 68 | 345 | 969 | 1.3 ms | yes |
| `branching/02_one_dead_one_alive` | 48 | 2 062 | 20 | 866 | 2 062 | 20 | 1 | 4 | 195 | 102 | 2 164 | 19 502 | 16.1 ms | yes |
| `syntax/constraint-scopes` | 12 | 4 096 | 4 096 | 4 095 | 4 096 | 4 096 | 1 | 1 | 4 096 | 0 | 4 096 | 24 576 | 8.7 ms | yes |
| `branching/08_hypothesis_relation_whitelist` | 38 | 5 782 | 34 | 1 098 | 5 782 | 34 | 1 | 8 | 443 | 164 | 5 946 | 55 506 | 24.4 ms | yes |
| `zebra2-hints` | 36 | 8 316 | 6 | 190 | 8 316 | 6 | 1 | 2 | 21 | 73 | 8 389 | 309 545 | 0.50 s | yes |
| `type-exclusivity/pets` | 15 | 11 339 | 11 339 | 26 191 | 11 339 | 11 339 | 1 | 39 | 11 339 | 12 | 11 351 | 76 743 | 23.3 ms | yes |
| `branching/06_lookahead_on` | 204 | 18 622 | 74 | 14 594 | 18 622 | 74 | 1 | 22 | 1 227 | 570 | 19 192 | 180 962 | 88.6 ms | yes |
| `branching/07_lookahead_off` | 204 | 18 622 | 74 | 14 594 | 18 622 | 74 | 1 | 22 | 1 227 | 570 | 19 192 | 180 962 | 90.6 ms | yes |
| `features/02_star_in_identifiers` | 15 | 32 768 | 728 | 2 252 | 32 768 | 728 | 1 | 1 | 1 072 | 0 | 32 768 | 470 616 | 0.40 s | yes |
| `zebra2-obligations` | 56 | 270 213 | 20 | 1 015 | 261 982 | 1 | 63 | 1 | 63 | 366 | 270 579 | 15 094 278 | 18.47 s | partial, layers 0–4 |
| `zebra2` | 56 | 270 213 | 20 | 1 015 | 261 982 | 1 | 63 | 1 | 63 | 366 | 270 579 | 15 094 278 | 17.86 s | partial, layers 0–4 |
| `zebra` | 56 | 270 454 | 27 | 1 374 | 261 982 | 1 | 63 | 1 | 63 | 458 | 270 912 | 30 251 060 | 26.08 s | partial, layers 0–2 |
| `syntax/arg-kinds` | 19 | 524 288 | 524 288 | 524 287 | 524 288 | 524 288 | 1 | 1 | 524 288 | 0 | 524 288 | 4 980 736 | 1.81 s | partial, layers 0–6 |
| `square-bwd/floors` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 11.42 s | partial, layers 0–5 |
| `square-bwd/houses` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 12.86 s | partial, layers 0–5 |
| `square-bwd/meetings` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 13.35 s | partial, layers 0–5 |
| `square-fwd/floors` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 13.22 s | partial, layers 0–5 |
| `square-fwd/houses` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 12.37 s | partial, layers 0–5 |
| `square-fwd/meetings` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 13.09 s | partial, layers 0–5 |
| `zebra2-minus-15-obligations` *(not finished)* | 96 | ≥ 4 097 512 | ≥ 4 709 | ≥ 411 209 | ≥ 2 840 307 | ≥ 32 | — | — | — | ≥ 27 804 | 4 125 317 | 378 138 052 | 493.49 s | partial, layers 0–2 |
| `zebra2-minus-15` *(not finished)* | 96 | ≥ 4 097 512 | ≥ 4 709 | ≥ 411 209 | ≥ 2 840 307 | ≥ 32 | — | — | — | ≥ 27 804 | 4 125 317 | 378 138 052 | 494.97 s | partial, layers 0–2 |

The columns:

- **n**, **|alive|**, **alive states**: the down-set's commitments and their
  distinct states (§ 3).
- **extensions**: the alive states' distinct one-up extensions, the state
  route's work as S1e2.1c.2 words it (§ 3.4).
- **k_c**, **k_s**: R in commitments and in states.
- **min**, **max**: the two lines' sizes, each in commitments.
- **generators**: every state's minimal generators, summed. A state reached
  by one commitment counts 1.
- **|N|**: the minimal dead sets.
- **saturations**, **facts**, **wall**: describe's work, i.e. route 1 to the
  end at the ceiling, `--jobs 16`, in process. Saturations are the visited
  commitments, plus ⊤ on a run that declined. Facts are those beyond root,
  summed.
- **default**: whether the default budget of 2¹⁹ facts describes it, or the
  layers it stops after.

A `≥` is what a run that did not finish met, labelled as such and never a
total.

**What it shows:**

- **The cost follows |alive| times the size of a node, and neither n nor k_s.**
  - `branching/06` has n = 204 and costs 0.09 s, because its alive set is
    18 622 small nodes.
  - `zebra2-hints` has n = 36 and costs 0.50 s for 8 389 nodes, each of
    which derives 37 facts.
  - `zebra` describes one state for 30 251 060 facts.
  - `arg-kinds` describes 524 288 states for a sixth of that.
- **k_s and |alive| are independent.** Across the table, the ratio
  |alive| / alive states runs from 1 (`arg-kinds`, `pets`,
  `constraint-scopes`) to 13 511 (`zebra2`).
- **The default describes everything up to `features/02`**, at 470 616 facts.
  Each of the ten that finish past it needs at least 9.5 times the budget:
  `arg-kinds`, at 4 980 735 facts (§ 5.1).

### 5.1 The smallest budget that describes each of the twelve, and the binary

Route 1 checks its budget before each layer, against the facts spent plus a
projection of the next layer: its size times the facts per node of the layer
before ([`route1_reach.md` § 4](../p1e2.1a_the_solution_region/route1_reach.md#4-the-budget)).
Replaying that rule over a finished run's layers gives the **smallest
`--region-budget` under which describe finishes**. It is a count, and the
binary confirms it:

| program | `--region-budget` | describe by the binary | its `work` row | `-s` wall | process wall | peak RSS | the instrument's wall |
|---|---:|---|---|---:|---:|---:|---:|
| `zebra` | 30 251 060 | k_s 1, k_c 261 982, exhausted | 270 912 saturations · 30 251 060 facts beyond root · 19 layers | 19.16 s | 19.88 s | 217 MiB | 26.08 s |
| `zebra` (one fact under) | 30 251 059 | partial: stopped before layer 18, the last; k_s ≥ 1, k_c ≥ 261 981 | 270 912 saturations · 30 251 341 facts beyond root · 18 layers | 19.39 s | 20.10 s | 233 MiB |  |
| `zebra2` | 15 094 278 | k_s 1, k_c 261 982, exhausted | 270 579 saturations · 15 094 278 facts beyond root · 19 layers | 13.49 s | 13.97 s | 216 MiB | 17.86 s |
| `square-bwd/floors` | 18 468 767 | k_s 27 559, k_c 1 048 576, exhausted | 1 048 576 saturations · 18 468 768 facts beyond root · 21 layers | 11.90 s | 13.18 s | 481 MiB | 11.42 s |
| `syntax/arg-kinds` | 4 980 735 | k_s 524 288, k_c 524 288, exhausted | 524 288 saturations · 4 980 736 facts beyond root · 20 layers | 1.73 s | 5.74 s | 1.19 GiB | 1.81 s |

- **The replay is exact.** At the replayed budget the binary describes `zebra`.
  One fact under it, the binary stops before the last layer, whose one
  commitment is the solution's 18 facts. It then saturates ⊤ instead: 270 911
  visits plus ⊤, and ⊤'s 393 facts.
- **The binary's work row is the instrument's**, saturation for saturation
  and fact for fact.
- **The `-s` wall is the route's; the process wall is everything.** The
  binary links snmalloc. On the zebra family its `-s` wall is about three
  quarters of the instrument's, and on the cubes it is about the same. On
  `arg-kinds` the read-out costs more than the route.
  `ein solve … -t` splits it into 1.71 s of route, 2.55 s of read-out (each
  listed state's model, the queries) and 0.67 s of print. Describe's price
  past the route grows with k_s, and k_s is 2¹⁹ there.
- These walls are the ones a reader of `ein solve` pays at `-j 16`, taken
  2026-11-03 with the commands above.

## 6. The flagged programs past the limit

**Listed, not measured.** On a program the static check flags, what a node's
state is, and so what the alive set is, is open:
[F21's D2](../../followups/f21_non_monotone_programs_decided.md#d2--q-m1e22-what-a-nodes-state-is).
The search past the limit is
[D7](../../followups/f21_non_monotone_programs_decided.md#d7--the-non-monotone-search-past-the-oracles-limit).
So no alive set is counted below. Each row gives what a bare `ein solve FILE -s`
certifies today, at `d2b6fdf0`, with the warning P1e2.1b put on every surface
that reports R. The warning's three parts are as the binary prints them:

- **what is assumed**, the same line on every row: *every node is a fork of
  root (root's fixpoint, its commitment added, saturated); what a node is on
  this program is open: Q-M1e2.2, F21*;
- **what is at stake**, per site;
- **whether it bit**, in the `status` and `state` lines.

| program | n | route today | what it certifies | the warning: sites, at stake | `status` | `state` |
|---|---:|---|---|---|---|---|
| `features/01_not_and_absent` | 35 | 2, partial at 2¹⁶ | ∅ ∈ R; the minimal line {∅}; k_s, k_c ≥ 64 655 met; `exhausted no` | 2 sites (`unknown-feeling`): the state | not known — the cube is 2^35 nodes, past --region-limit 16 | stale facts — 24 754 of the 64 655 nodes the run computed hold one |
| `features/04_open` | 81 | 2, partial at 2¹⁶ | ∅ ∈ R; the minimal line {∅}; k_s, k_c ≥ 64 761 met | 1 site (`open-candidate`): the state | not known — the cube is 2^81 nodes | stale facts — 21 042 of the 64 761 nodes |
| `features/05_stdlib_domain_elim` | 56 | 2, partial at 2¹⁶ | ∅ ∈ R; the minimal line {∅}; k_s, k_c ≥ 7 176 met; N ≥ 21 | 4 sites (`domain-elimination`, `no-room-left`, `typecheck-arg-0`, `-1`): the state, and for three of them dead with it | not known — the cube is 2^56 nodes | no stale fact in the 65 156 nodes the run computed |
| `lattice/06_friends_tables` | 19 | 2, partial at 2¹⁶ | ∅ ∈ R; the minimal line {∅}; k_s, k_c ≥ 21 met; N ≥ 68 | 1 site (`group-seated-owed`, an obligation trigger): the status, owes | not known — the cube is 2^19 nodes | no stale fact in the 65 299 nodes |
| `square-unique/corner-house` | 118 | 2, partial at 2¹⁶ | ∅ ∈ R; the minimal line {∅}; k_s ≥ 65 214, k_c ≥ 65 389 met | 1 site (`square-unique`): the state | not known — the cube is 2^118 nodes | stale facts — 5 595 of the 65 389 nodes |
| `square-unique/cul-de-sac` | 114 | 2, partial at 2¹⁶ | ∅ ∈ R; the minimal line {∅}; k_s ≥ 64 817, k_c ≥ 64 923 met | 1 site (`square-unique`): the state | not known — the cube is 2^114 nodes | stale facts — 5 835 of the 64 923 nodes |
| `square-unique/terminus` | 153 | 2, partial at 2¹⁶ | ∅ ∈ R; the minimal line {∅}; k_s ≥ 64 514, k_c ≥ 65 029 met | 1 site (`square-unique`): the state | not known — the cube is 2^153 nodes | stale facts — 4 876 of the 65 029 nodes |
| `features/12_expect_false` | 20 | **1**, by L0, one saturation | all of R: root is dead, so R is empty, k_s = k_c = 0, `exhausted yes` | 2 sites (`forced`, `no-room`): the state, and dead with it | not known — root is dead, so every node is (L0), and the cube is 2^20 nodes | no stale fact in the 1 node the run computed |

- **Seven are past `--region-limit 16` with no licence**, the route-3 seven
  of § 1. Route 2 visits the cube in layer order up to 2¹⁶ nodes and stops.
  What it certifies without a licence is the minimal line, which is {∅}
  because root is alive and in R on all seven. Its `maximal line` row reads
  *not known — without a licence it needs the whole cube*. The counts it
  prints are of what it met, each labelled *a lower bound*.
- **`features/12` is the eighth** of
  [`route1_reach.md` § 1](../p1e2.1a_the_solution_region/route1_reach.md#1-the-three-routes-over-the-corpus)'s
  eight. It is past the limit, but its root is dead, so route 1 certifies
  R = ∅ in one saturation. It still carries the warning, whose `status` line
  says what L0 settles.
- **The warning bit on five**: a stale fact in 4 876–24 754 nodes of the
  cube's first 2¹⁶. On `features/05` and `lattice/06` it did not show within
  the nodes computed. On `lattice/06` the site puts the status at stake, not
  the state, which a stale fact cannot show.
- Each of the seven also prints one line to stderr: *note: FILE: partial —
  the question needs 2^n saturations, past --region-limit 16*. `features/12`
  prints none.

## 7. What it says for the stages that follow

**For [S1e2.1c.2](s1e2.1c.2_the_lines_by_duality.md), the state route by
duality:**

- **Its input is measured, not bounded.** § 3 and § 8 count every monotone
  program's alive states. § 3.4 counts its saturations as its acceptance
  words them.
- **The zebra family is its case.** It has 20–27 alive states, every one of
  them generated by at most 2 candidates, and 1 016–1 375 saturations where
  route 1 needs 270 579–270 912. Route 1 describes `zebra` only at
  30 251 060 facts, 2^24.85 (§ 5.1). Whether the state route fits 2¹⁹ facts
  depends on the facts a closed node derives, which this page does not
  measure. That is S1e2.1c.2's budget question, (a), and it now has route 1's
  side counted.
- **The crossover is measured.** The state route without L1 saturates dead
  extensions route 1's join never generates. It loses wherever states do not
  collapse:
  - `type-exclusivity/pets`: 26 192 against 11 351;
  - `lattice/06_friends_tables_everyone_sits`: 3 045 against 345;
  - `lattice/07_two_pairs_two_tables`: 1 073 against 177.

  It breaks even where every commitment is a state and N is empty
  (`arg-kinds`, `constraint-scopes`). So *"a route beside route 1, not in
  place of it"* is right, and the rule that picks describe's route needs a
  test that reads the program. Pruning extensions by the dead sets already
  found would move the crossover. How far, the route will measure.
- **The everything-alive cubes are decided in two saturations** on all 85
  monotone programs where R is the cube, and on no other (§ 4). Describe
  already makes both on the seven past the oracle's limit, and prints k_c as a
  lower bound. k_s still needs the states: 27 559 on each `square-*`, 151 141
  saturations by the state route against route 1's 1 048 576. On `arg-kinds`
  k_s = 2¹⁹, so a route that visits each state once visits 2¹⁹.
- **`zebra2-minus-15` is the open case.** Route 1 cannot finish it under this
  ceiling. Through size 6 it has 4 097 512 alive commitments and 4 709 alive
  states, and layer 6 met no new state (§ 3.2). Whether the state route
  finishes it is S1e2.1c.2's to measure. This page does not predict it.
- **Memory.** Route 1 keeps every visited point, at 0.4–0.9 KiB each (§ 2.1).
  A route that keeps states keeps 870 times fewer on `zebra2-minus-15`'s first
  six layers.

**For [S1e2.1c.4](s1e2.1c.4_the_flags_at_scale.md):** § 5 and § 8 are the
describe column it re-takes
[`solve_interface.md` § 3.4](../p1e2.1a_the_solution_region/solve_interface.md#34-what-it-costs)
against. Every other mode is its own.

**For F21:** nothing. § 6 lists the flagged programs past the limit with
what they get today, and measures nothing on them.

## 8. The whole table

Every monotone program with n ≥ 1, in § 5's columns, by |alive|, then n.
The 97 monotone programs with n = 0 are one saturation each, and R is ∅'s
state or empty:

| n = 0 | programs | R | saturations |
|---|---:|---|---:|
| ∅ alive and discharged | 60 | {∅}: k_c = k_s = 1 | 1 |
| ∅ alive, owing | 15 | empty | 1 |
| ∅ dead | 22 | empty | 1 |

Each of the 97 costs at most 5.9 ms.

| program | n | \|alive\| | alive states | extensions | k_c | k_s | min | max | generators | \|N\| | saturations | facts | wall | default |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `ein-bugs/alt-cap-core-reordered` | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 506 µs | yes |
| `ein-bugs/alt-cap-core` | 70 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 513 µs | yes |
| `features/09_adjacent_via_same_house` | 82 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 400 µs | yes |
| `ein-bugs/zebra2-hints` | 90 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 1 | 0 | 1.9 ms | yes |
| `ein-bugs/wrong-answers/forced-1` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 2 | 2 | 213 µs | yes |
| `ein-bugs/wrong-answers/forced-2-off` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | 217 µs | yes |
| `ein-bugs/wrong-answers/forced-2` | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | 251 µs | yes |
| `ein-bugs/alive-empty-phase1` | 2 | 1 | 1 | 2 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | 4 | 269 µs | yes |
| `ein-bugs/alive-set-fresh-name` | 2 | 1 | 1 | 2 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | 6 | 249 µs | yes |
| `ein-bugs/wrong-answers/forced-1-two` | 2 | 1 | 1 | 2 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | 4 | 258 µs | yes |
| `ein-bugs/wrong-answers/forced-2-two` | 2 | 1 | 1 | 2 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | 6 | 283 µs | yes |
| `ein-bugs/wrong-answers/fresh-name-default` | 2 | 1 | 1 | 2 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | 6 | 333 µs | yes |
| `ein-bugs/wrong-answers/fresh-name-nospawn` | 2 | 1 | 1 | 2 | 1 | 1 | 1 | 1 | 1 | 2 | 3 | 4 | 246 µs | yes |
| `ein-bugs/wrong-answers/owed-two-step` | 2 | 1 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 3 | 6 | 312 µs | yes |
| `ein-bugs/wrong-answers/tree-dead` | 2 | 1 | 1 | 2 | 0 | 0 | 0 | 0 | 0 | 2 | 3 | 4 | 304 µs | yes |
| `branching/01_saturate_only` | 16 | 1 | 1 | 16 | 1 | 1 | 1 | 1 | 1 | 16 | 17 | 56 | 1.3 ms | yes |
| `bijection/06_blind_enumeration` | 16 | 1 | 1 | 16 | 1 | 1 | 1 | 1 | 1 | 16 | 17 | 104 | 1.3 ms | yes |
| `domain_elim/b_only` | 1 | 2 | 2 | 1 | 2 | 2 | 1 | 1 | 2 | 0 | 2 | 1 | 365 µs | yes |
| `lattice/11_fifo_gap_static` | 1 | 2 | 2 | 1 | 2 | 2 | 1 | 1 | 2 | 0 | 2 | 1 | 270 µs | yes |
| `branching/09_hrule` | 7 | 2 | 2 | 13 | 2 | 2 | 1 | 1 | 2 | 6 | 8 | 34 | 740 µs | yes |
| `domain_elim/b_branch` | 7 | 2 | 2 | 13 | 2 | 2 | 1 | 1 | 2 | 6 | 8 | 13 | 344 µs | yes |
| `features/13_mixed_solution_and_open` | 2 | 3 | 3 | 3 | 1 | 1 | 1 | 1 | 1 | 1 | 4 | 7 | 348 µs | yes |
| `unsat-core-id-space` | 5 | 3 | 3 | 12 | 3 | 3 | 1 | 2 | 3 | 4 | 7 | 11 | 847 µs | yes |
| `ein-bugs/mixed-type-hypothesis` | 2 | 4 | 4 | 3 | 4 | 4 | 1 | 1 | 4 | 0 | 4 | 4 | 234 µs | yes |
| `ein-bugs/nested-fact-hypothesis` | 2 | 4 | 4 | 3 | 4 | 4 | 1 | 1 | 4 | 0 | 4 | 4 | 239 µs | yes |
| `ein-bugs/wrong-answers/arity3` | 2 | 4 | 4 | 3 | 4 | 4 | 1 | 1 | 4 | 0 | 4 | 4 | 221 µs | yes |
| `implies/org-chart` | 2 | 4 | 3 | 3 | 4 | 3 | 1 | 1 | 3 | 0 | 4 | 5 | 236 µs | yes |
| `implies/parent-to-ancestor` | 2 | 4 | 3 | 3 | 4 | 3 | 1 | 1 | 3 | 0 | 4 | 5 | 247 µs | yes |
| `implies/right-then-next` | 2 | 4 | 3 | 3 | 4 | 3 | 1 | 1 | 3 | 0 | 4 | 5 | 253 µs | yes |
| `ein-bugs/alive-set-fresh-name-declared` | 6 | 4 | 4 | 19 | 4 | 4 | 1 | 1 | 4 | 4 | 8 | 19 | 333 µs | yes |
| `branching/03_five_hyps_one_alive` | 132 | 4 | 2 | 262 | 4 | 2 | 1 | 1 | 3 | 130 | 134 | 442 | 2.0 ms | yes |
| `branching/15_lookahead_two_step_on` | 3 | 5 | 5 | 7 | 5 | 5 | 1 | 2 | 5 | 2 | 7 | 12 | 300 µs | yes |
| `branching/16_lookahead_two_step_off` | 3 | 5 | 5 | 7 | 5 | 5 | 1 | 2 | 5 | 2 | 7 | 12 | 328 µs | yes |
| `ein-bugs/complete-records-stale` | 3 | 5 | 5 | 7 | 5 | 5 | 1 | 2 | 5 | 2 | 7 | 11 | 292 µs | yes |
| `lattice/synth/one_model_per_component` | 4 | 5 | 5 | 10 | 4 | 4 | 4 | 4 | 4 | 6 | 11 | 22 | 329 µs | yes |
| `lattice/02_genuine_3set_death` | 3 | 7 | 7 | 7 | 7 | 7 | 1 | 3 | 7 | 1 | 8 | 13 | 320 µs | yes |
| `tests/region/01_states` | 3 | 7 | 7 | 7 | 7 | 7 | 1 | 3 | 7 | 1 | 8 | 13 | 374 µs | yes |
| `tests/region/08_query_classes` | 3 | 7 | 7 | 7 | 7 | 7 | 1 | 3 | 7 | 1 | 8 | 13 | 310 µs | yes |
| `features/11_expect_ambiguity` | 4 | 7 | 7 | 14 | 7 | 7 | 1 | 2 | 7 | 4 | 11 | 20 | 393 µs | yes |
| `lattice/05_owed_seats_bounded` | 4 | 7 | 7 | 14 | 2 | 2 | 2 | 2 | 2 | 4 | 11 | 20 | 458 µs | yes |
| `tests/region/09_query_over_maximal` | 4 | 7 | 7 | 14 | 7 | 7 | 1 | 2 | 7 | 4 | 11 | 20 | 370 µs | yes |
| `branching/12_typed_blind_solve` | 25 | 7 | 3 | 71 | 7 | 3 | 1 | 2 | 5 | 25 | 32 | 356 | 2.7 ms | yes |
| `lattice/03_state_hash_collision` | 3 | 8 | 5 | 7 | 8 | 5 | 1 | 1 | 5 | 0 | 8 | 16 | 298 µs | yes |
| `transitive/colocation-chain` | 3 | 8 | 4 | 6 | 8 | 4 | 1 | 1 | 5 | 0 | 8 | 17 | 330 µs | yes |
| `transitive/mealtimes` | 3 | 8 | 4 | 6 | 8 | 4 | 1 | 1 | 5 | 0 | 8 | 17 | 311 µs | yes |
| `transitive/taxonomy` | 3 | 8 | 4 | 6 | 8 | 4 | 1 | 1 | 5 | 0 | 8 | 17 | 417 µs | yes |
| `syntax/unary-and-nary` | 3 | 8 | 8 | 7 | 8 | 8 | 1 | 1 | 8 | 0 | 8 | 12 | 296 µs | yes |
| `tests/region/03_merged_or` | 3 | 8 | 4 | 6 | 8 | 4 | 1 | 1 | 5 | 0 | 8 | 17 | 337 µs | yes |
| `branching/05_mini_zebra` | 9 | 8 | 5 | 36 | 8 | 5 | 1 | 1 | 7 | 6 | 14 | 66 | 937 µs | yes |
| `lattice/01_subset_pruned` | 4 | 12 | 12 | 15 | 12 | 12 | 1 | 2 | 12 | 1 | 13 | 23 | 375 µs | yes |
| `ein-bugs/wrong-answers/owed-seats-renamed` | 4 | 16 | 16 | 15 | 7 | 7 | 2 | 1 | 7 | 0 | 16 | 32 | 386 µs | yes |
| `ein-bugs/wrong-answers/owed-seats` | 4 | 16 | 16 | 15 | 7 | 7 | 2 | 1 | 7 | 0 | 16 | 32 | 347 µs | yes |
| `ein-bugs/wrong-answers/uncovered-knows` | 4 | 16 | 16 | 15 | 7 | 7 | 2 | 1 | 7 | 0 | 16 | 32 | 394 µs | yes |
| `ein-bugs/wrong-answers/uncovered` | 4 | 16 | 16 | 15 | 7 | 7 | 2 | 1 | 7 | 0 | 16 | 32 | 406 µs | yes |
| `lattice/synth/merged_states` | 5 | 24 | 16 | 29 | 16 | 8 | 2 | 2 | 8 | 1 | 25 | 65 | 451 µs | yes |
| `tests/region/02_merged_single` | 5 | 24 | 16 | 29 | 16 | 8 | 2 | 2 | 8 | 1 | 25 | 65 | 501 µs | yes |
| `branching/04_two_levels` | 46 | 31 | 7 | 306 | 31 | 7 | 1 | 2 | 17 | 54 | 85 | 284 | 1.0 ms | yes |
| `features/03_forall` | 5 | 32 | 21 | 30 | 32 | 21 | 1 | 1 | 21 | 0 | 32 | 92 | 517 µs | yes |
| `lattice/synth/trivial_line` | 5 | 32 | 32 | 31 | 24 | 24 | 2 | 1 | 24 | 0 | 32 | 80 | 452 µs | yes |
| `syntax/equality` | 6 | 64 | 64 | 63 | 64 | 64 | 1 | 1 | 64 | 0 | 64 | 192 | 612 µs | yes |
| `branching/10_kill_cache_on` | 18 | 64 | 15 | 225 | 64 | 15 | 1 | 1 | 38 | 12 | 76 | 672 | 2.3 ms | yes |
| `branching/11_kill_cache_off` | 18 | 64 | 15 | 225 | 64 | 15 | 1 | 1 | 38 | 12 | 76 | 672 | 2.4 ms | yes |
| `lattice/07_two_pairs_two_tables` | 16 | 113 | 113 | 1 072 | 8 | 8 | 8 | 8 | 8 | 64 | 177 | 448 | 6.3 ms | yes |
| `type-exclusivity/colors` | 8 | 169 | 169 | 246 | 169 | 169 | 1 | 9 | 169 | 4 | 173 | 644 | 828 µs | yes |
| `type-exclusivity/nationalities` | 8 | 169 | 169 | 246 | 169 | 169 | 1 | 9 | 169 | 4 | 173 | 644 | 766 µs | yes |
| `lattice/06_friends_tables_everyone_sits` | 19 | 277 | 277 | 3 044 | 8 | 8 | 8 | 8 | 8 | 68 | 345 | 969 | 1.3 ms | yes |
| `branching/02_one_dead_one_alive` | 48 | 2 062 | 20 | 866 | 2 062 | 20 | 1 | 4 | 195 | 102 | 2 164 | 19 502 | 16.1 ms | yes |
| `syntax/constraint-scopes` | 12 | 4 096 | 4 096 | 4 095 | 4 096 | 4 096 | 1 | 1 | 4 096 | 0 | 4 096 | 24 576 | 8.7 ms | yes |
| `branching/08_hypothesis_relation_whitelist` | 38 | 5 782 | 34 | 1 098 | 5 782 | 34 | 1 | 8 | 443 | 164 | 5 946 | 55 506 | 24.4 ms | yes |
| `zebra2-hints` | 36 | 8 316 | 6 | 190 | 8 316 | 6 | 1 | 2 | 21 | 73 | 8 389 | 309 545 | 0.50 s | yes |
| `type-exclusivity/pets` | 15 | 11 339 | 11 339 | 26 191 | 11 339 | 11 339 | 1 | 39 | 11 339 | 12 | 11 351 | 76 743 | 23.3 ms | yes |
| `branching/06_lookahead_on` | 204 | 18 622 | 74 | 14 594 | 18 622 | 74 | 1 | 22 | 1 227 | 570 | 19 192 | 180 962 | 88.6 ms | yes |
| `branching/07_lookahead_off` | 204 | 18 622 | 74 | 14 594 | 18 622 | 74 | 1 | 22 | 1 227 | 570 | 19 192 | 180 962 | 90.6 ms | yes |
| `features/02_star_in_identifiers` | 15 | 32 768 | 728 | 2 252 | 32 768 | 728 | 1 | 1 | 1 072 | 0 | 32 768 | 470 616 | 0.40 s | yes |
| `zebra2-obligations` | 56 | 270 213 | 20 | 1 015 | 261 982 | 1 | 63 | 1 | 63 | 366 | 270 579 | 15 094 278 | 18.47 s | partial, layers 0–4 |
| `zebra2` | 56 | 270 213 | 20 | 1 015 | 261 982 | 1 | 63 | 1 | 63 | 366 | 270 579 | 15 094 278 | 17.86 s | partial, layers 0–4 |
| `zebra` | 56 | 270 454 | 27 | 1 374 | 261 982 | 1 | 63 | 1 | 63 | 458 | 270 912 | 30 251 060 | 26.08 s | partial, layers 0–2 |
| `syntax/arg-kinds` | 19 | 524 288 | 524 288 | 524 287 | 524 288 | 524 288 | 1 | 1 | 524 288 | 0 | 524 288 | 4 980 736 | 1.81 s | partial, layers 0–6 |
| `square-bwd/floors` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 11.42 s | partial, layers 0–5 |
| `square-bwd/houses` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 12.86 s | partial, layers 0–5 |
| `square-bwd/meetings` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 13.35 s | partial, layers 0–5 |
| `square-fwd/floors` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 13.22 s | partial, layers 0–5 |
| `square-fwd/houses` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 12.37 s | partial, layers 0–5 |
| `square-fwd/meetings` | 20 | 1 048 576 | 27 559 | 151 140 | 1 048 576 | 27 559 | 1 | 1 | 29 711 | 0 | 1 048 576 | 18 468 768 | 13.09 s | partial, layers 0–5 |
| `zebra2-minus-15-obligations` *(not finished)* | 96 | ≥ 4 097 512 | ≥ 4 709 | ≥ 411 209 | ≥ 2 840 307 | ≥ 32 | — | — | — | ≥ 27 804 | 4 125 317 | 378 138 052 | 493.49 s | partial, layers 0–2 |
| `zebra2-minus-15` *(not finished)* | 96 | ≥ 4 097 512 | ≥ 4 709 | ≥ 411 209 | ≥ 2 840 307 | ≥ 32 | — | — | — | ≥ 27 804 | 4 125 317 | 378 138 052 | 494.97 s | partial, layers 0–2 |

## 9. The state route beside route 1

**T1e2.1c.2.5's measurement**, for
[S1e2.1c.2](s1e2.1c.2_the_lines_by_duality.md), the second half of this page.
The state route ([`ein_infer::states`](../../../ein.rs/crates/ein-infer/src/states.rs))
visits every alive state once and saturates its one-up extensions. A dead
extension prunes its supersets (L1), and an extension that is a state already
is read and not saturated (L3). It reads R off the states: k_c by L3, each
state's minimal generators, both lines, N by duality, the components and the
sub-cubes. Since T1e2.1c.2.4 it is describe's route on every monotone program.
This section measures it against the route-1 table above, program by program.

```sh
./build.sh                                                   # the release ein, snmalloc linked
(cd ein.rs && cargo build --release -p ein-infer --example state_reach)
R=ein.rs/target/release/examples/state_reach
CAP="systemd-run --user --scope -q -p MemoryMax=11G -p MemorySwapMax=0"

# § 9.1, § 9.2: every monotone program but the zebra2-minus-15 pair, both routes, unbounded
$CAP python3 peak.py both.jsonl -- timeout 1800 \
  utils/bench_env.sh --cores PT $R --route1 --jobs 16 FILE…
# § 9.3: the pair, the state route alone, with R per size and the duality checked
$CAP python3 peak.py z.jsonl -- timeout 900 \
  utils/bench_env.sh --cores PT $R --jobs 16 --by-size 6 --duality examples/zebra2-minus-15.ein
# § 9.5: the binary, a bare describe, at -j 1 and -j 16
$CAP python3 peak.py out.txt -- timeout 900 \
  utils/bench_env.sh --cores PT ein.rs/target/release/ein solve FILE -s -t -j J [--region-budget F]
```

**Taken 2026-11-03 on `c5c45e37`**, release builds, with the machine state
`utils/bench_env.sh` printed: an i9-14900HX pinned with `--cores PT` to
cpu0–15 (8 physical P-cores), governor `powersave` (epp
`balance_performance`), turbo on, load average under 3. The ceiling is the
same as § 2's: `timeout 900` a run (1 800 for the sweep) and `MemoryMax=11G`.
No run reached either. `peak.py` and the instrument's JSON line are § 2's.
*Smallest budget* is the replay of § 5.1's rule over the state route's
layers: before a layer, the facts spent plus its saturations times the facts
per saturation so far.

### 9.1 Every monotone program: never more than route 1

Over the 181 monotone programs, both routes run to the end:

- **The state route saturates fewer nodes than route 1 on 32 and as many on
  149**, and derives fewer facts on the same 32 and as many on the rest. It
  never does more of either. Summed, that is **1 341 309 saturations and
  15 969 475 facts** against route 1's 7 732 817 and 177 557 787, and 6.3 s
  of wall against 131.5 s.
- **Where states do not collapse, it is route 1, node for node.** That is
  `pets`, `constraint-scopes` and `arg-kinds`. L1's pruning closes § 3.4's
  crossover: `pets`, `lattice/07_two_pairs_two_tables` and
  `lattice/06_friends_tables_everyone_sits` cost route 1's 11 351, 177 and 345
  saturations, where the unpruned count was 26 192, 1 073 and 3 045. Its wall
  there can be a few milliseconds more than route 1's, since it builds its
  description (`pets` 32.5 ms against 23.7 ms).
- **Where they collapse, it wins by the collapse**: 724 times on `zebra2`, 609
  on `zebra`, 108 on `zebra2-hints`, 20 on `branching/06` and `/07`, and 7.9
  on the `square-*` six.

The programs where the two differ, and every program route 1 needs more than
4 000 saturations for:

| program | n | alive states | saturations | facts | wall | route 1: saturations | facts | wall | route 1 ÷ state route, saturations |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `saturation/implies/org-chart.ein` | 2 | 3 | 3 | 3 | 0.3 ms | 4 | 5 | 0.3 ms | 1.3 |
| `saturation/implies/parent-to-ancestor.ein` | 2 | 3 | 3 | 3 | 0.3 ms | 4 | 5 | 0.3 ms | 1.3 |
| `saturation/implies/right-then-next.ein` | 2 | 3 | 3 | 3 | 0.2 ms | 4 | 5 | 0.3 ms | 1.3 |
| `lattice/03_state_hash_collision.ein` | 3 | 5 | 7 | 13 | 0.3 ms | 8 | 16 | 0.3 ms | 1.1 |
| `saturation/transitive/colocation-chain.ein` | 3 | 4 | 7 | 14 | 0.3 ms | 8 | 17 | 0.3 ms | 1.1 |
| `saturation/transitive/mealtimes.ein` | 3 | 4 | 7 | 14 | 0.3 ms | 8 | 17 | 0.3 ms | 1.1 |
| `saturation/transitive/taxonomy.ein` | 3 | 4 | 7 | 14 | 0.3 ms | 8 | 17 | 0.3 ms | 1.1 |
| `tests/region/03_merged_or.ein` | 3 | 4 | 7 | 14 | 0.3 ms | 8 | 17 | 0.3 ms | 1.1 |
| `branching/05_mini_zebra.ein` | 9 | 5 | 13 | 60 | 1.0 ms | 14 | 66 | 1.0 ms | 1.1 |
| `lattice/synth/merged_states.ein` | 5 | 16 | 18 | 46 | 0.4 ms | 25 | 65 | 0.4 ms | 1.4 |
| `tests/region/02_merged_single.ein` | 5 | 16 | 18 | 46 | 0.4 ms | 25 | 65 | 0.4 ms | 1.4 |
| `branching/12_typed_blind_solve.ein` | 25 | 3 | 30 | 348 | 2.9 ms | 32 | 356 | 2.8 ms | 1.1 |
| `features/03_forall.ein` | 5 | 21 | 22 | 56 | 0.5 ms | 32 | 92 | 0.5 ms | 1.5 |
| `branching/10_kill_cache_on.ein` | 18 | 15 | 58 | 480 | 2.3 ms | 76 | 672 | 2.4 ms | 1.3 |
| `branching/11_kill_cache_off.ein` | 18 | 15 | 58 | 480 | 2.2 ms | 76 | 672 | 2.3 ms | 1.3 |
| `branching/04_two_levels.ein` | 46 | 7 | 71 | 236 | 1.0 ms | 85 | 284 | 1.1 ms | 1.2 |
| `branching/03_five_hyps_one_alive.ein` | 132 | 2 | 133 | 440 | 1.9 ms | 134 | 442 | 2.0 ms | 1.0 |
| `branching/02_one_dead_one_alive.ein` | 48 | 20 | 193 | 1 272 | 4.3 ms | 2 164 | 19 502 | 19.4 ms | 11.2 |
| `syntax/constraint-scopes.ein` | 12 | 4 096 | 4 096 | 24 576 | 8.6 ms | 4 096 | 24 576 | 6.5 ms | 1.0 |
| `branching/08_hypothesis_relation_whitelist.ein` | 38 | 34 | 335 | 2 716 | 2.8 ms | 5 946 | 55 506 | 27.1 ms | 17.7 |
| `zebra2-hints.ein` | 36 | 6 | 78 | 6 733 | 79.4 ms | 8 389 | 309 545 | 506.9 ms | 107.6 |
| `saturation/type-exclusivity/pets.ein` | 15 | 11 339 | 11 351 | 76 743 | 32.5 ms | 11 351 | 76 743 | 23.7 ms | 1.0 |
| `branching/06_lookahead_on.ein` | 204 | 74 | 961 | 7 968 | 8.6 ms | 19 192 | 180 962 | 89.3 ms | 20.0 |
| `branching/07_lookahead_off.ein` | 204 | 74 | 961 | 7 968 | 8.7 ms | 19 192 | 180 962 | 97.6 ms | 20.0 |
| `features/02_star_in_identifiers.ein` | 15 | 728 | 1 736 | 21 364 | 20.4 ms | 32 768 | 470 616 | 316.0 ms | 18.9 |
| `zebra2-obligations.ein` | 56 | 20 | 374 | 42 705 | 185.2 ms | 270 579 | 15 094 278 | 18.16 s | 723.5 |
| `zebra2.ein` | 56 | 20 | 374 | 42 705 | 178.7 ms | 270 579 | 15 094 278 | 18.81 s | 723.5 |
| `zebra.ein` | 56 | 27 | 445 | 106 829 | 276.3 ms | 270 912 | 30 251 060 | 25.82 s | 608.8 |
| `syntax/arg-kinds.ein` | 19 | 524 288 | 524 288 | 4 980 736 | 1.81 s | 524 288 | 4 980 736 | 2.03 s | 1.0 |
| `saturation/square-bwd/floors.ein` | 20 | 27 559 | 132 383 | 1 773 548 | 571.9 ms | 1 048 576 | 18 468 768 | 10.50 s | 7.9 |
| `saturation/square-bwd/houses.ein` | 20 | 27 559 | 132 383 | 1 773 548 | 592.2 ms | 1 048 576 | 18 468 768 | 10.96 s | 7.9 |
| `saturation/square-bwd/meetings.ein` | 20 | 27 559 | 132 383 | 1 773 548 | 602.2 ms | 1 048 576 | 18 468 768 | 11.18 s | 7.9 |
| `saturation/square-fwd/floors.ein` | 20 | 27 559 | 132 383 | 1 773 548 | 553.8 ms | 1 048 576 | 18 468 768 | 11.59 s | 7.9 |
| `saturation/square-fwd/houses.ein` | 20 | 27 559 | 132 383 | 1 773 548 | 638.3 ms | 1 048 576 | 18 468 768 | 10.65 s | 7.9 |
| `saturation/square-fwd/meetings.ein` | 20 | 27 559 | 132 383 | 1 773 548 | 640.5 ms | 1 048 576 | 18 468 768 | 10.63 s | 7.9 |
| `zebra2-minus-15.ein` | 96 | 4 709 | 99 505 | 9 481 774 | 12.51 s | ≥ 4 125 316, not finished | ≥ 378 138 052 | 495 s | — |
| `zebra2-minus-15-obligations.ein` | 96 | 4 709 | 99 505 | 9 481 774 | 12.30 s | ≥ 4 125 316, not finished | ≥ 378 138 052 | 495 s | — |

`route_invariance::the_state_route_and_route_1_print_one_read_out` holds
both halves per commit, the read-out and *never more*, on the 169 programs
both routes finish at the default budget, and on 179 with `EIN_CORPUS_SLOW=1`
(390 s, release). On every monotone cube the oracle enumerates, the state
route's every field is the oracle's
(`the_state_route_is_the_oracle_on_every_monotone_program_it_reaches`, 156
cubes).

### 9.2 What it describes, and what the default budget reaches

R as the state route reads it, on every program past § 5's 32 768 facts and on
the zebra family and the pair. Every number is route 1's (§ 2.1, § 5), where
route 1 finishes. *description* is the read-off alone, single-threaded.

| program | k_s | k_c | min | max | generators | merged | \|N\| | maximal alive | components | description | smallest budget | default |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `ein-bugs/zebra2-hints.ein` | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0.0 ms | 0 | yes |
| `branching/05_mini_zebra.ein` | 5 | 8 | 1 | 1 | 7 | 1 | 6 | 1 | 1 | 0.0 ms | 60 | yes |
| `lattice/07_two_pairs_two_tables.ein` | 8 | 8 | 8 | 8 | 8 | 0 | 64 | 24 | 8 | 0.2 ms | 448 | yes |
| `zebra2-hints.ein` | 6 | 8 316 | 1 | 2 | 21 | 4 | 73 | 2 | 1 | 0.1 ms | 6 805 | yes |
| `features/02_star_in_identifiers.ein` | 728 | 32 768 | 1 | 1 | 1 072 | 688 | 0 | 1 | 1 | 1.3 ms | 21 364 | yes |
| `zebra2-obligations.ein` | 1 | 261 982 | 63 | 1 | 63 | 1 | 366 | 8 | 1 | 0.5 ms | 42 705 | yes |
| `zebra2.ein` | 1 | 261 982 | 63 | 1 | 63 | 1 | 366 | 8 | 1 | 0.5 ms | 42 705 | yes |
| `saturation/type-exclusivity/pets.ein` | 11 339 | 11 339 | 1 | 39 | 11 339 | 0 | 12 | 39 | 1 | 8.0 ms | 76 743 | yes |
| `zebra.ein` | 1 | 261 982 | 63 | 1 | 63 | 1 | 458 | 9 | 1 | 0.6 ms | 106 829 | yes |
| `saturation/square-bwd/floors.ein` | 27 559 | 1 048 576 | 1 | 1 | 29 711 | 23 896 | 0 | 1 | 1 | 57.1 ms | 1 773 548 | no |
| `saturation/square-bwd/houses.ein` | 27 559 | 1 048 576 | 1 | 1 | 29 711 | 23 896 | 0 | 1 | 1 | 54.9 ms | 1 773 548 | no |
| `saturation/square-bwd/meetings.ein` | 27 559 | 1 048 576 | 1 | 1 | 29 711 | 23 896 | 0 | 1 | 1 | 54.6 ms | 1 773 548 | no |
| `saturation/square-fwd/floors.ein` | 27 559 | 1 048 576 | 1 | 1 | 29 711 | 23 896 | 0 | 1 | 1 | 54.8 ms | 1 773 548 | no |
| `saturation/square-fwd/houses.ein` | 27 559 | 1 048 576 | 1 | 1 | 29 711 | 23 896 | 0 | 1 | 1 | 57.4 ms | 1 773 548 | no |
| `saturation/square-fwd/meetings.ein` | 27 559 | 1 048 576 | 1 | 1 | 29 711 | 23 896 | 0 | 1 | 1 | 55.3 ms | 1 773 548 | no |
| `syntax/arg-kinds.ein` | 524 288 | 524 288 | 1 | 1 | 524 288 | 0 | 0 | 1 | 1 | 927.1 ms | 4 980 736 | no |
| `zebra2-minus-15.ein` | 32 | 241 462 963 | 12 068 | 32 | 12 068 | 32 | 27 804 | 467 | 32 | 398.9 ms | 9 481 774 | no |
| `zebra2-minus-15-obligations.ein` | 32 | 241 462 963 | 12 068 | 32 | 12 068 | 32 | 27 804 | 467 | 32 | 401.7 ms | 9 481 774 | no |

- **The zebra family fits the default budget.** `zebra` needs 106 829 facts
  where route 1 needed 30 251 060. A bare `ein solve examples/zebra.ein`
  describes R exhausted: one state, 261 982 commitments, 63 minimal
  generators, |N| = 458.
- **The default still does not count every state of eight programs**: the six
  `square-*` cubes (1 773 548 facts), `arg-kinds` (4 980 736) and the pair
  (9 481 774). The first seven are everything-alive cubes, so R itself is
  decided (§ 9.4).
- **Not a corpus program is past 2²⁴ facts** on the state route. Route 1 needed
  2^24.85 for `zebra` alone, and did not finish the pair.

### 9.3 `zebra2-minus-15`, described

The open case of § 2.3, which route 1 could not finish at 2²² visits in 495 s.
**The state route finishes it**: 99 505 saturations and 9 481 774 facts,
12.5 s and a 90 MiB peak at `-j 16`, 23 layers by state size.

- **4 709 alive states**, the count § 3.2 met by layer 6 and could not call a
  total. None is generated only past layer 6.
- **R is 32 states and 241 462 963 commitments**, in 32 components, one state
  each. The minimal line is 12 068 commitments, every minimal generator of
  every state, the least of 2 candidates. The maximal line is the 32 states,
  as M1d's tree found.
- **|N| = 27 804**, route 1's floor exactly: N through size 6 was all of N.
  There are 467 maximal alive states, 435 of them owing.
- **Route 1 agrees where it finished.** R's commitments per size, enumerated
  from the generators, are § 2.3's layers 0–6, 0, 0, 221, 10 270, 107 963,
  585 989 and 2 135 864, and N's members per size are its dead column,
  1 428, 13 269, 11 799, 1 300 and 8.
  `the_state_route_is_route_1_on_zebra2_minus_15_through_layer_6` holds both
  (`EIN_CORPUS_SLOW=1`).
- **The duality holds the other way**: the complements of N's 467 minimal
  transversals are exactly the maximal alive states. That direction costs
  16.2 s of MMCS alone, more than the route (`state_reach --duality`).
- The twin, `zebra2-minus-15-obligations`, is the same in every column.

### 9.4 The everything-alive cubes, as describe prints them

§ 4's decision, in the read-out since T1e2.1c.2.4. On a run its budget stops,
the state route saturates ⊤ once, as route 1 did. With ∅ also in R, **R is
decided**: k_c = 2ⁿ, both trivial lines, one component, one sub-cube of
dimension n and N = 0, all printed exact. The `exhausted` row says that only
the states are not all counted, and `merged` says *not known*. At the default
budget:

| program | k_s printed | of | saturations | facts | `-t` route + read-out, `-j 1` |
|---|---:|---:|---:|---:|---:|
| `square-*` (six) | ≥ 17 449, complete up to states of 6 candidates | 27 559 | 42 711 | 426 508 | 0.48 s + 0.22 s |
| `syntax/arg-kinds` | ≥ 94 185, complete up to states of 6 candidates | 524 288 | 94 185 | 592 439 | 0.40 s + 0.36 s |

`k_s` is counted, every state, at the smallest budgets of § 9.2: 1 773 548
facts for a `square-*` cube, 4 980 736 for `arg-kinds`. Route 1 printed
k_c ≥ 21 701 and ≥ 43 797 at the default, and no N, components or sub-cube.

### 9.5 The binary: a bare `ein solve`

`ein solve FILE -s -t -j J [--region-budget F]`, the release binary with
snmalloc, one process per row. *route*, *read-out* and *print* are `-t`'s.
*peak* is the process's.

| program | `-j` | `--region-budget` | answer | work | route | read-out | print | peak |
|---|---:|---|---|---|---:|---:|---:|---:|
| `zebra` | 1 | default | exhausted, k_s 1 | 445 · 106 829 facts | 0.97 s | 1.9 ms | 0.1 ms | 14 MiB |
| `zebra` | 16 | default | exhausted, k_s 1 | 445 · 106 829 | 0.23 s | 2.0 ms | 0.1 ms | 82 MiB |
| `zebra2` | 1 | default | exhausted, k_s 1 | 374 · 42 705 | 0.54 s | 1.5 ms | 0.1 ms | 13 MiB |
| `zebra2-obligations` | 1 | default | exhausted, k_s 1 | 374 · 42 705 | 0.55 s | 1.6 ms | 0.1 ms | 12 MiB |
| `square-bwd/floors` | 1 | default | R decided, k_s ≥ 17 449 | 42 711 · 426 508 | 0.48 s | 0.22 s | 0.3 ms | 43 MiB |
| `square-bwd/floors` | 1 | 2²¹ | exhausted, k_s 27 559 | 132 383 · 1 773 548 | 2.26 s | 0.47 s | 0.11 s | 127 MiB |
| `square-bwd/floors` | 16 | 2²¹ | exhausted, k_s 27 559 | 132 383 · 1 773 548 | 0.72 s | 0.47 s | 0.11 s | 138 MiB |
| `syntax/arg-kinds` | 1 | default | R decided, k_s ≥ 94 185 | 94 185 · 592 439 | 0.40 s | 0.36 s | 1.2 ms | 119 MiB |
| `syntax/arg-kinds` | 1 | 2²³ | exhausted, k_s 2¹⁹ | 524 288 · 4 980 736 | 2.62 s | 2.50 s | 0.67 s | 1.12 GiB |
| `syntax/arg-kinds` | 16 | 2²³ | exhausted, k_s 2¹⁹ | 524 288 · 4 980 736 | 1.79 s | 2.50 s | 0.67 s | 1.18 GiB |
| `zebra2-minus-15` | 1 | default | partial, k_s ≥ 21 | 1 652 · 122 678 | 1.20 s | 20 ms | 0.2 ms | 16 MiB |
| `zebra2-minus-15` | 16 | default | partial, k_s ≥ 21 | 1 652 · 122 678 | 0.60 s | 20 ms | 0.2 ms | 69 MiB |
| `zebra2-minus-15` | 1 | 2²⁴ | exhausted, k_s 32 | 99 505 · 9 481 774 | 79.45 s | 29 ms | 6.3 ms | 47 MiB |
| `zebra2-minus-15` | 16 | 2²⁴ | exhausted, k_s 32 | 99 505 · 9 481 774 | 10.07 s | 29 ms | 6.2 ms | 97 MiB |

- **A bare describe of the zebra family costs under a second at `-j 1`**,
  where at S1e2.1c.1 it was partial and `zebra` exhausted only at 30 251 060
  facts, in 19.2 s at `-j 16` (§ 5.1).
- **`zebra2-minus-15` is cheap to stop and dear to finish.** At the default
  it stops at 122 678 facts, because its next layer, 6 739 extensions,
  projects 498 686 more. It then certifies less than route 1 did at the same
  budget: k_s ≥ 21 against ≥ 28, and N ≥ 294 against ≥ 1 428. Route 1 had
  spent 478 948. To finish it costs 79 s at `-j 1` and 10 s at `-j 16`.
- **`arg-kinds`' read-out still costs more than its route**, as § 5.1 found:
  2.50 s of read-out and 0.67 s of print against 1.79 s of route at `-j 16`,
  with a 1.18 GiB peak. The read-out reads each of its 2¹⁹ states' model, and
  that is not this stage's work, so it is noted and not fixed. Its route is
  route 1's, node for node.

### 9.6 What it says

- **For the budget question** (S1e2.1c.2's T1e2.1c.2.6, the user's): the
  default of 2¹⁹ facts already describes the zebra family. What it leaves
  partial is the seven cubes' state count and the pair. § 9.5 prices each
  step past it.
- **For [S1e2.1c.4](s1e2.1c.4_the_flags_at_scale.md)**: the modes are still
  route 1 with a cut, and describe now does less than they do on `zebra`. At
  the default budget `--single`, `--maximal` and `--minimal` stop partial
  after 4 225–4 343 saturations and 476 432–513 614 facts, where describe is
  exhausted in 445. At 2²⁰ they certify in 8 818–8 927 saturations
  (`ein solve examples/zebra.ein MODE --region-budget N -s`, 2026-11-03). So
  `zebra`'s *is it unique?* is answered by describe's `k_s` before `--single`
  can answer it.
- **For memory**: the state route keeps the states and the commitments it
  met, not every alive commitment. The pair's peak is 90 MiB where route 1's
  2²² visits held 3.59 GiB.
