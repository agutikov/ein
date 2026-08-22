# S1a.9.0 — The slow corpus, re-priced

**Phase:** P1a.9 (Release)
**Estimate:** 3 days
**Depends on:** nothing — it runs first, before anything is packaged
**Numbered `.0`** for the same reason
[S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md) is: measure the
premise before spending the phase. That stage inverted `design/08` §2 in one
day of measurement; this one starts with the measurement already taken.

**Status: shipped 2026-08-22.** The record is
[`corpus_cost.md`](corpus_cost.md); the instrument that produced it is
[`utils/corpus_cost.py`](../../../utils/corpus_cost.py), the eighteenth script
in `utils/` and the ninth in the M1a measurement set.

| finding | number |
|---|---|
| entries flagged `slow`, before → after | **17 → 3** — twelve were never slow by the new rule, two stopped being when a run that asked nothing was dropped |
| the flagship, `zebra2.ein`, flagged slow at | **16 ms** (260 ms for all sixteen of its cells) |
| `EIN_CORPUS_SLOW=1`, before → after | **307.2 s → 21.1 s** of `cargo test`, 660 → 641 cells, green both times |
| the default sweep, before → after | 1.70 s → **2.97 s**, 542 → **622** cells — thirteen entries came back |
| share of the old slow tier spent on **two** cells | **90 %** — `square-unique/{corner-house,cul-de-sac} :: render lattice`, 219 s of lattice DOT for two demos of one rule firing |
| entries whose `solve` and `solve -e` cost the same | **ten**, ratio 0.98–1.24× — no node is ever complete, so the fast path has nothing to stop at |
| what the three "cannot finish" cells actually do | **not a timeout — the OOM killer**, at 14.3 GB and 57–152 s. The note that blamed CPython was wrong about the reason as well as the engine |
| runs dropped / restored | **16 cells dropped**, 0 restored — `features/05_stdlib_domain_elim` finishes in 3.0 s now and keeps its exclusion, because affordable is not the test |
| notes naming CPython | **6 → 0** |
| the corpus's worst case | `features/04_open :: render lattice`, **10.2 s** — 640× `zebra2`'s whole `solve`, and 18.3 MB of DOT |
| handed to M1d | [Q-M1d.6](../../m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false) — `Contradiction` said with `exhausted=False`, and the `-m` sweep that shows it |

## Context

[`corpus/corpus.toml`](../../../corpus/corpus.toml) marks **17 of 128** entries
`slow = true`, and five of them additionally drop `solve` / `solve -e` from
their `runs` column. Every one of those exclusions carries the same reason,
verbatim:

> *no solve, solve -e: outlives a 150 s budget under CPython, and a run nobody
> can finish is not coverage.*

**Under CPython.** The flag and the exclusions were priced against an engine
that left the tree at
[S1a.10.5](../p1a.10_single_implementation/s1a.10.5_removal.md), on a budget
set for it. ein.rs solves `zebra -e` in 47.5 ms where the measurement that set
these notes was ~165× slower
([P1a.6](../p1a.6_performance/README.md)). Nobody has asked since whether the
list still describes anything.

It does not, and the shape of what it describes instead is the stage.

## What is already measured — 2026-08-21

`ein.rs/target/release/ein`, cold process, median of the runs noted, 30 s
timeout. Process startup floor on this machine is **1.68 ms**
(`ein --help`), so every figure below is solve time.

| entry | `solve` | `solve -e` | ratio | `-e` declared |
|---|---:|---:|---:|:--:|
| `branching/06_lookahead_on` | 7 ms | 276 ms | 40× | yes |
| `branching/07_lookahead_off` | 1 114 ms | 1 125 ms | **1.0×** | yes |
| `features/01_not_and_absent` | 2 228 ms | 2 266 ms | **1.0×** | yes |
| `features/02_star_in_identifiers` | 293 ms | 290 ms | **1.0×** | yes |
| `features/04_open` | **136 s** | > 30 s | — | no |
| `features/05_stdlib_domain_elim` | 3 607 ms | 3 574 ms | **1.0×** | no |
| `saturation/square-bwd/{floors,houses,meetings}` | 345–350 ms | 340–350 ms | **1.0×** | yes |
| `saturation/square-fwd/{floors,houses,meetings}` | 348–364 ms | 343–351 ms | **1.0×** | yes |
| `saturation/square-unique/corner-house` | > 30 s | > 30 s | — | no |
| `saturation/square-unique/cul-de-sac` | > 30 s | > 30 s | — | no |
| `zebra` | 23 ms | 70 ms | 3.1× | yes |
| `zebra2-minus-15` | 38 ms | > 30 s | — | no |
| `zebra2` | **11 ms** | 42 ms | 3.8× | yes |

Three groups, and only one of them is what the flag claims.

### A — four are not slow (`zebra2` is 11 ms)

`zebra2`, `zebra`, `branching/06_lookahead_on` and `zebra2-minus-15`'s fast
path all finish in 7–38 ms. `zebra2` carries `slow = true` and the longest
`runs` list in the manifest; at 11 ms the flag is simply false, and a flag that
is false on the corpus's flagship entry is a flag no reader trusts anywhere
else.

### B — ten are slow because the fast path buys nothing

Ten entries cost the **same** exhaustively as they do at `stop_after=1`
(ratio 1.0×). Every one of them returns the same thing:

```
07_lookahead_off  Contradiction k=0 exhausted=False layers=5 enterings=11 501
01_not_and_absent Contradiction k=0 exhausted=False layers=5 enterings=384 167
02_star…          Contradiction k=0 exhausted=False layers=5 enterings=4 943
square-fwd/houses Contradiction k=0 exhausted=False layers=5 enterings=21 699
```

`layers == 5` is `max_set_size`, the default `-m`. Sweeping it on
`02_star_in_identifiers` shows what is actually happening:

| `-m` | verdict | layers | enterings | wall |
|---:|---|---:|---:|---:|
| 2 | Contradiction, `exhausted=False` | 2 | 120 | 10 ms |
| 3 | Contradiction, `exhausted=False` | 3 | 575 | 33 ms |
| 4 | Contradiction, `exhausted=False` | 4 | 1 940 | 114 ms |
| 5 | Contradiction, `exhausted=False` | 5 | 4 943 | 293 ms |
| 6 | Contradiction, `exhausted=False` | 6 | 9 948 | 592 ms |
| 7 | Contradiction, `exhausted=False` | 7 | 16 383 | 983 ms |

`layers_explored` tracks `-m` exactly and the cost roughly doubles per layer.
**The search is not proving unsat — it is running out of commitment-set depth
and reporting `Contradiction` anyway**, and the whole cost of these ten
entries is the depth it walks before doing so.

Two things follow, and they are different in kind.

**The manifest's, and it is this stage's.** `02_star_in_identifiers` is a demo
of `*` in identifiers — two rules, four `is-a` edges, a transitive closure, and
`(query :goal (?R Rex Animal))` with a `;;TODO S1.5.9` beside it. There is no
hypothesis structure in it to search. Its `runs` column nevertheless asks for
`solve` **and** `solve -e`, so the engine enumerates a wide empty hypothesis
space to the cap and spends 293 ms establishing nothing, on a file whose
meaningful run — `saturate` — is near the startup floor. Several of the ten
are saturation demos in the same position.

**The engine's, and it is not this stage's.**
[`docs/api/inference.md`](../../../docs/api/inference.md) documents `k == 0` as
*"`Contradiction` — unsat (**when exhausted**)"*, and every one of these
reports `Contradiction` with `exhausted=False`. By the contract as written the
word is not earned: a depth-capped search that found no model is nearer to
`Aborted` — *"budget cut, not proven"* — than to a refutation. That is
[M1d](../../m1d_satisfiability/README.md)'s subject and
[Q-M1d.1](../../m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)'s
question; this stage **states and hands over** rather than fixes, because
changing a verdict word changes checked-in fixtures across the corpus and that
is a semantic decision, not a release chore.

### C — three still cannot finish, and CPython is no longer why

`features/04_open` takes **136 s**; `square-unique/corner-house` and
`cul-de-sac` outlive 30 s. These are the entries whose exclusion note blames a
CPython budget — and they are unfinishable on an engine two orders of magnitude
faster, which means **the note's reason was wrong even where its conclusion was
right**. `04_open`'s own note gets it right and should be the model for all
three: *"a feature demo of `open`, not of search — nothing bounds the
hypothesis space, so `solve` measures the blind enumerator."*

## Acceptance

- Every one of the 17 entries has a **measured** `solve` / `solve -e` figure
  against ein.rs, taken through [`utils/bench_env.sh`](../../../utils/bench_env.sh)
  so the machine state is on the record, and the numbers live where a reader
  will find them rather than in this file alone.
- **No surviving note explains a cost by naming CPython.** Each is rewritten to
  say what is true of ein.rs, or deleted.
- `slow = true` means something again: a stated threshold, applied uniformly,
  with the four group-A entries losing the flag. `zebra2` at 11 ms is not slow.
- **Every dropped run is re-justified or restored.** For each of the five
  entries missing `solve` / `solve -e`, either the run is added back with its
  measured cost, or the exclusion is kept with a reason that is about the
  *puzzle* (an unbounded hypothesis space) and not about an interpreter.
- The group-B finding is **written down where the search owns it** — an entry
  in [M1d](../../m1d_satisfiability/README.md) — with the `-m` sweep as its
  reproducer. This stage does not change a verdict word.
- `EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test corpus_cli` still passes,
  and its wall-clock before and after is recorded. If restoring runs makes it
  materially slower, that is a number in the record, not a reason to skip them.

## Tasks

### Task T1a.9.0.1 — Measure all 17, properly

The table above is a laptop reading taken while planning: one repetition, a
30 s ceiling, no core pinning. Re-take it under `bench_env.sh` with the
repetition count and the statistic the rest of the milestone uses
([`criterion_table.py`](../../../utils/criterion_table.py)'s precedent — a mean
without a deviation is not a measurement), and with a ceiling high enough to
turn the three `> 30 s` cells into numbers or into a stated timeout.

### Task T1a.9.0.2 — Define what `slow` means

Today it is a flag with no threshold, which is why it drifted. Pick one — a
wall-clock bound on the slowest declared run is the obvious candidate — write
it into [`corpus/README.md`](../../../corpus/README.md) beside the table of
readers, and make the manifest's own invariant test check the flag against it
so the list cannot silently rot a second time. The check belongs with the
completeness check that already fails on an unlisted file.

### Task T1a.9.0.3 — Re-justify the five exclusions

`features/04_open`, `features/05_stdlib_domain_elim`,
`square-unique/{corner-house,cul-de-sac}`, `zebra2-minus-15`. Each keeps its
exclusion or loses it on the measurement, and each ends with a note in
`04_open`'s style — what about the *puzzle* makes the run meaningless or
unbounded. `zebra2-minus-15` is the one likely to differ from the others: it is
[M1d](../../m1d_satisfiability/README.md)'s named subject and genuinely
under-determined at 32 models, so "large rather than pathological" is already
the right note and needs only its interpreter clause removed.

### Task T1a.9.0.4 — The `runs` column against what the fixture is for

The group-B saturation demos are asked to `solve`. Decide per entry whether
`solve` is coverage or noise: a fixture that demonstrates the lexer, the
closure idiom or a stdlib rule set is exercised by `saturate`, and adding
`solve` to it buys a `Contradiction` nobody reads at 100–1 000× the cost.
Where a run is dropped, say so in the note — a `runs` column that shrinks
without a reason is how coverage disappears.

**Do not drop a run merely because it is slow.** The distinction this task
turns on is whether the run *asks the fixture's question*, and
[`corpus/README.md`](../../../corpus/README.md) should carry that sentence.

### Task T1a.9.0.5 — Hand the verdict question to M1d

Write the group-B finding into M1d as an entry of its own: the ten entries, the
`-m` sweep, the `Contradiction` / `exhausted=False` / `layers == max_set_size`
signature, and the contract line in `docs/api/inference.md` it sits against.
The reproducer is three commands and belongs in the entry.

M1d already asks *why an under-determined puzzle does not finish*; this is the
adjacent question — **what the engine says when it stops without an answer**,
and whether `Contradiction` may be said with `exhausted=False`.

## Notes

- **This stage is allowed to find nothing that needs fixing.** If the honest
  outcome is "the flags were stale, four are removed, the rest are correct and
  here are their numbers", that is a complete stage. The value is the list
  meaning something, not a defect count —
  [backlog lists work, not verdicts](../p1a.10_single_implementation/s1a.10.4_utils.md).
- **The 136 s figure deserves its own line in the record.** `04_open` is the
  slowest single thing in the repository by two orders of magnitude, on an
  engine that solves the milestone's headline puzzle in 11 ms. It is a demo of
  `open` and the cost is a property of an unbounded hypothesis space rather
  than a regression — but "the engine's worst case on the corpus is 12 000× its
  flagship" is a sentence
  [M1d](../../m1d_satisfiability/README.md) should own, and nowhere in the tree
  currently says it.
- **The measurement is the deliverable, and it survives its conclusions.**
  Whatever is decided about flags and `runs`, the 17 figures are the first
  ein.rs-native pricing of the corpus's tail and belong in the record with the
  date and the machine, the way
  [`baseline.md`](../p1a.6_performance/baseline.md) keeps its arc.
