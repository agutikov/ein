# P1a.4 — Search layer

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **shipped** 2026-08-18 — all six stages, acceptance below.
**Estimate:** 4 weeks (19 days of stages)
**Depends on:** [P1a.3](../p1a.3_deductive_core/README.md)
**Blocks:** [P1a.5](../p1a.5_presentation/README.md)

## Goal

Everything above the fixpoint: hypothesis generation, one-step
lookahead, the Apriori commitment lattice, no-good learning, the
commitment primitive, and the three-phase `solve` loop with its verdict
synthesis. At the end of this phase `ein.rs solve <file>` returns the
right answer with the right counters on every corpus entry — **T1
corpus-wide**, T2 on the branching fixtures.

Design: [design/07](../design/07_search_layer.md).

## Stages

| stage | title | est. | shipped |
|---|---|---|---|
| [S1a.4.1](s1a.4.1_hypothesis_generation.md) | Hypothesis generation | 4 d | ✅ |
| [S1a.4.2](s1a.4.2_lookahead_and_closure.md) | Lookahead, closure marking, NAF dependency map | 3 d | ✅ |
| [S1a.4.3](s1a.4.3_apriori_and_nogoods.md) | Apriori candidate generation and the no-good store | 3 d | ✅ |
| [S1a.4.4](s1a.4.4_commitment_primitive.md) | The commitment primitive | 2 d | ✅ |
| [S1a.4.5](s1a.4.5_solve_loop.md) | The solve loop and verdict synthesis | 4 d | ✅ |
| [S1a.4.6](s1a.4.6_explanation_and_cores.md) | Explanation and unsat cores | 3 d | ✅ |

**They did not ship in that order, and the plan's own dependencies are
why.** S1a.4.4's `try_commitment_set` puts
`smallest_contradiction_frontier` in the `enter` event and in
`CommitmentSetResult`, and that function is S1a.4.6's — which in turn
declared a dependency on S1a.4.5, which depends on S1a.4.4. The cycle is
only in the *acceptances*: S1a.4.6's machinery has none, so it shipped
third-from-last and the order was **1 → 2 → 3 → 6 → 4 → 5**. Two thirds of
S1a.4.2 came forward into S1a.4.1 for the same kind of reason: the filter
pipeline's acceptance is not checkable without the lookahead, which
accounts for 547 of the corpus's 4 479 raw candidates.

## Acceptance for the phase

All met. **239 tests** in `cargo test --workspace` (207 at P1a.3's
close), of which **42** are differential against `ein.py` — 13 of them
this phase's, in `hypgen_parity.rs`.

| item | result |
|---|---|
| **T1 corpus-wide** — every counter in [design/01](../design/01_parity_contract.md) §2 | `solve-shape` over every corpus entry in three regimes: `fast` (5 174 enterings), `exhaustive` (1 618), `shuffled` (5 207). **65 files, 0 differences** in each |
| **T2** on `branching/**`, `lattice/**`, `domain_elim/**` — identical `hyp` / `enter` / `nogood` / `writeback` sequences | the `hyp` sweep (4 489 candidates over 66 files) and the `solve` sweeps' event block, **with its `n`** — which counts every event including the ~58 000 saturator ones the text filters out |
| The three acceptance fixtures reproduce, with the same models | `crates/ein-infer/tests/acceptance.rs`, ported from `ein.py/acceptance/`. Both encodings reach the published 25-cell grid, `zebra2-minus-15` reads as `Ambiguity`, `zebra2-bad` as `Contradiction` naming the injected fact. **0.87 s**, against minutes under PyPy — so they run with the ordinary suite instead of a separate gate |
| `--shuffle --seed N` produces the same traversal | **yes** — [Q-M1a.5](../open_questions.md#q-m1a5--reproducing-cpythons-shuffle) resolved as (a), CPython's Mersenne Twister ported and checked both by table and on every corpus entry |
| [`features.md`](../../../docs/kernel/inference/features.md)'s lever matrix regenerated | **every entering count reproduces**, and `enable_singleton_writeback` is still the one load-bearing lever — below |

### The lever matrix, re-measured

`cargo run --release -p ein-infer --example lever_matrix`, `zebra2.ein`,
`max_enterings = 20 000`:

| lever off | enterings (py / rs) | ×base (py / rs) |
|---|---|---|
| `enable_singleton_writeback` | **3 336+ / 3 831** | ≥23× *(Aborted)* / **57.7×** |
| `enable_fail_fast_fork` | 101 / 101 | 1.9× / **3.0×** |
| `lattice_order="score-sum"` | 134 / 134 | 1.0× / 1.2× |
| `enable_pre_branch_lookahead` | 111 / 111 | 0.9× / 1.0× |
| every other lever | 101 / 101 | 1.0× / 1.0× |

Two readings, and neither is the wall clock:

- **ein.rs can answer a cell ein.py could only time out on.** The Python
  matrix aborts a runaway on a 90-second budget, so
  `enable_singleton_writeback`-off was "3 336+, still climbing". Here it
  *finishes*: **3 831 enterings in 11.3 s**, which confirms the ≥ 3 336
  and replaces an inequality with a number.
- **`enable_fail_fast_fork` got more valuable, not less.** It avoids
  *saturation* work, and saturation is now 31× cheaper relative to
  everything around it — so the lever's share of what is left went up,
  1.9× → 3.0×. design/07 predicted the reverse for the lookahead (whose
  sign may flip once matching is cheap) and said nothing about this one;
  it is [P1a.6](../p1a.6_performance/README.md)'s to re-read.

### The bench set has no pending rows left

| bench | ein.py | ein.rs | |
|---|---:|---:|---|
| `solve_fast` zebra2 (11 enterings) | 1.22 s | **43.0 ms** | 28× |
| `solve_exhaustive` zebra2 (101 enterings) | 5.00 s | **194 ms** | 26× |
| `solve zebra` fast (13 enterings) | 6.99 s | **119 ms** | 59× |
| `solve zebra` exhaustive (111 enterings) | 30.4 s | **587 ms** | 52× |
| one hypgen pass, `zebra2` hrule + lookahead | 18.3 ms | **656 µs** | 28× |
| one hypgen pass, `terminus` blind + lookahead | 7.49 ms | **122 µs** | 61× |

Both sides report the same `k`, the same enterings and the same
alive/dead split on every row. **These are the search, not the run**: the
milestone's baseline is end-to-end and attributes 200 ms of parse and
430 ms of load to it, and those have their own benches at 1 003× and
607×, so folding them in would flatter this row with two others' results.
Against PyPy's 4.07 s end-to-end for `solve zebra2 -e`, the port's
~195 ms is **21×** — the milestone's ≥ 20× target, met.

The per-commit conformance tier, re-run at close: **473 cells, 0
differences, T3, 348.5 s of engine time** (455 at P1a.3's close; the two
new fixtures add 18). `./run_tests.sh`: 1 500 + 5 passed.

### The instruments this phase needed

Six ops on `utils/ir_oracle.py`, each following
[S1a.2.3](../p1a.2_kb_core/s1a.2.3_loader.md)'s shape — both
implementations render the same text and the texts are diffed:

- **`hyp-shape`** (± `closed`) — every candidate with the *name of the
  filter that dropped it*, plus the `--hyp-stats` report. Two regimes,
  because closing a relation removes its candidates *before* the
  whitelist is consulted, so `no_hypothesis_relation` is only reachable
  in one of them.
- **`naf-map`** — the static stratification proxy, over a saturated cache.
- **`lattice-shape`** — the Apriori join, both ordering modes, and a
  fixed no-good recipe chosen to make all three subsumption outcomes
  happen.
- **`commit-shape`** (± `fail-fast`) — every entering's kind, firings,
  fork size, core, hypothesis writes and their provenance.
- **`explain-shape`** (± `alts`) — the three searches over the AND/OR
  graph, on contradictions *and* on a sample of derived facts, twice,
  once under a deliberately tight budget.
- **`solve-shape`** (three modes) — the whole loop.

## What the port had to look at rather than transcribe

- **`score_hypothesis` does not read the config the generator reads.**
  ein.py falls back to `"most-constrained"` when `kb.config is None`,
  while the dataclass default has been `"popularity"` since S1.5a.7 — two
  different fallbacks for one field. No corpus file distinguishes them, so
  it is a unit test.
- **A whitelist of names that resolve to nothing is still a whitelist.**
  Deciding "is `:hypothesis-relations` empty?" on how many names
  *interned* rather than how many were *declared* would turn a whitelist
  of one misspelled relation — which excludes everything — into no
  whitelist at all.
- **design/07's "intern-on-demand" split does not exist here.**
  `FactStore::intern` *is* `probe` plus a push on a miss, and the push is
  bounded by the distinct candidate space rather than by the call count.
- **The prefix join's `break` is a cost win and only a cost win**, where
  [S1a.4.3](s1a.4.3_apriori_and_nogoods.md) calls it "load-bearing for
  both cost and order". Replacing it with a `continue` is byte-identical
  on all 65 files.
- **Two interner-order leaks**, both in tie-breaks that ein.py makes by
  *content*: `layer_1`'s `sorted(alive)` (33 of 65 files) and
  `record_node`'s lex-smallest-commitment rule (1 file, and only the
  exhaustive regime has two paths to compete).

## Where the risks landed

- **"`explain` is where a 'cleaner' port silently changes the answer."**
  The environment representation *did* change — an environment is a
  sorted rank vector here, not a `frozenset` — and the risk was managed
  the way it asked: the loop structure, the wave ordering and the
  domination test are shape-for-shape, and the diff runs on every corpus
  entry rather than on the zebra ones. Three of four mutations move
  files; the fourth needed the instrument to reach for it (below).
- **"hypgen's stats are attribution, not accounting."** Swapping the
  first two filters — same survivors, different counters — moves 2 files.
  The risk was that such a change looks like an optimisation; it is
  caught in the same breath as a wrong answer.

## What the corpus could not tell, and what was done about it

Four paths were mutation-tested and found untested. Two got fixtures,
one got a purpose-built instrument line, and one is recorded:

- **The lookahead's D3 guard, both halves.** Disabling either left all 64
  files green. `examples/branching/13_lookahead_naf_world.ein` and
  `14_lookahead_unjudgeable.ein` now pin them; both flip **Solution →
  Contradiction** when their guard is removed, because a wrong kill is
  cached as `(not h)` and poisons every later branch. `14` needs *two*
  levels of absent nesting — at one level the two cheap checks are
  already exact, so a `forall`-shaped fixture would pass either way.
- **`_recorded_fallback`'s tie-break.** It only decides when two targets
  tie on core size, and `zebra2-bad`'s four size-1 cores are won by the
  same witness whichever way it is broken. The instrument calls the
  fallback once on the **reversed** witness list, which separates them.
- **`branch=0` on a hypothesis write.** T1a.4.4.2 says changing it
  changes provenance output; with the instrument printing only
  kind/firings/core it moved nothing, and with provenance in the line it
  moves 47 files.

## Cross-links

- [design/07 — Search layer](../design/07_search_layer.md)
- [`algorithm_layer_n.md`](../../../docs/kernel/inference/algorithm_layer_n.md)
- [D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)
  — `sorted(alive)` raises in ein.py where ein.rs answers
- [Q-M1a.4](../open_questions.md#q-m1a4--sorted-over-mixed-type-fact-args)
  (resolved — D2) · [Q-M1a.5](../open_questions.md#q-m1a5--reproducing-cpythons-shuffle)
  (resolved — MT19937 ported)
- [F9 ledger](../../followups/f9_e_catalog.md) — the rejected
  search-layer optimisations; do not re-derive them in Rust.
