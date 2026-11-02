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

**Status:** T1e2.1c.1.1 written 2026-11-03: §§ 1–2. The states, the cubes and
the cost table follow with T1e2.1c.1.2–.4.

**This stage measures.** No golden, `:expect` or register row moved. The
instrument is example code that no route reads:
[`route1_reach`](../../../ein.rs/crates/ein-infer/examples/route1_reach.rs)
gained ⊤'s status and the facts summed (T1e2.1c.1.1), each read off what the
route already computes.

```sh
./build.sh                                                   # the release ein, snmalloc linked
(cd ein.rs && cargo build --release -p ein-infer --example route1_reach)   # the instrument, system allocator
R=ein.rs/target/release/examples/route1_reach

# § 1: every corpus file and register witness, at the default budget (2^19 facts)
utils/bench_env.sh --cores PT $R --budget 18446744073709551615 --facts 524288 --jobs 16 > default.jsonl

# § 2: past the budget, one process per program, under the cap
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
and are not repeated here.

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
