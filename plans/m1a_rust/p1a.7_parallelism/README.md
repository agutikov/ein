# P1a.7 — Parallelism

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **paused 2026-08-20**, one stage in, at the user's direction.
[S1a.7.0](s1a.7.0_speculation_audit.md) shipped — a stage the plan did not
have: the phase's central risk is measurable *before* any of it is built, so
it was measured first. What it found is in [scaling.md](scaling.md) and it
moves two of the four stages after it. **Nothing is half-built**: S1a.7.0's
deliverables are an instrument, a corpus sweep and two invariance tests, all
committed and green, and no engine code is in an intermediate state. See
§ Paused — what a resumer needs to know.

## Paused — what a resumer needs to know

**Where it stopped.** After [S1a.7.0](s1a.7.0_speculation_audit.md), before
[S1a.7.1](s1a.7.1_sync_shared_state.md). The natural resumption point is
S1a.7.1 unchanged: `Terms` is still threaded as `&mut` through ~78 sites and
that is still the phase's first real work.

**What shipped and stays useful regardless.** All of it is independent of the
parallel path and none of it is scaffolding to be removed:

- `ein-infer/src/spec_audit.rs` + [`utils/spec_audit.py`](../../../utils/spec_audit.py)
  — the speculation audit, `--features spec-audit`;
- `SolveOptions::integrate_every` — the batch-synchronous integration mode,
  and the shape [P1d.1](../../m1d_satisfiability/p1d.1_exhaustive_search/README.md)'s conflict
  mining will need when it decides where harvested clauses land;
- `ein-infer/tests/search_invariants.rs` — six tests asserting that the answer
  depends on neither the entering order nor the integration time. These are
  **not** parallelism tests; they are properties of the search, and they hold
  whether or not this phase ever resumes;
- `ein-infer/examples/defer_probe.rs` — which found that a root write seals a
  layer every fork then walks, worth **2.8×** on `branching/07 -e` at
  `--jobs 1`. That is a single-threaded finding sitting in a parallelism
  phase, and it does not need the phase to be cashed.

**What the interval changes, and this is the part that will be missed.**
[P1a.10](../p1a.10_single_implementation/README.md) retires the conformance
harness — and **this phase's acceptance is written in terms of it**: "`--jobs
{1,2,4,8,16}` T3-identical on the whole corpus", "a 10 000-run randomised
stress of `--jobs 8` vs `--jobs 1`", and every T-tier reference in
[S1a.7.2](s1a.7.2_parallel_enterings.md) through
[S1a.7.5](s1a.7.5_jobs_contract.md). After P1a.10 those criteria name an
instrument that does not exist. A resumed P1a.7 **restates its acceptance
against whatever [S1a.10.1](../p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md)
banked** before it writes any code. The invariants themselves are unaffected —
`--jobs N` agreeing with `--jobs 1` is a comparison of one engine against
itself and never needed an oracle — but the *vocabulary* they are written in
goes away with it.

[P1d.1](../../m1d_satisfiability/p1d.1_exhaustive_search/README.md) touches it from the other
side: its conflict-mining dive writes learned clauses mid-layer, which is
exactly the hazard [S1a.7.0](s1a.7.0_speculation_audit.md) measured
(§ finding 4). Whichever of the two lands first owes the other the rule about
when a clause becomes visible; the phases should not discover it twice.

**What is still undecided.** [S1a.7.2](s1a.7.2_parallel_enterings.md)'s four
options for layer 1 — continue-and-validate, `--jobs`-scoped divergence,
sequential layer 1, or batch-synchronous integration — with **(d) the measured
one** and the barrier re-check of recorded solution nodes the piece it still
owes. And the fail-fast × speculation interaction, which design/08 never
named. Neither has been decided; both are written up rather than left in
someone's head, which is the point of pausing after an instrument stage rather
than in the middle of a refactor.
**Estimate:** 2.5 weeks (14 days of stages — S1a.7.0 added one worth 1 d)
**Depends on:** [P1a.6](../p1a.6_performance/README.md) — parallelise a
fast engine, not a slow one, or the speedup measures the wrong thing.

## What the phase start found

[S1a.7.0](s1a.7.0_speculation_audit.md) ran the sequential engine over the
corpus and, beside every entering, the *same* entering against `R0` — root as
it stood when the layer opened, which is what a worker would have forked.
**1 078 704 enterings speculated and compared.** Four results, and the phase
is re-shaped by the middle two:

- **The control held.** 1 078 154 case-1 speculations — both arms forking the
  same root — and **zero differences**. That is the corpus-scale form of the
  property level 1 rests on: `try_commitment_set` is pure with respect to
  root. It was a one-fixture assertion until now.
- **[design/08](../design/08_parallelism.md) §2's case-1 claim is inverted.**
  It says case 1 "is the whole of layer 1". But layer *L* enters commitment
  **sets of size L**, so a death licenses a clause of width *L* — and only a
  width-1 clause is a *fact* root can hold. Hence **layer 1 is the only layer
  that adds a fact to root mid-layer**, and every layer ≥ 2 is case 1 with no
  validator at all. (The no-good store is written mid-layer at every level and
  is not a hazard: no fork reads it while saturating.) The doc is corrected,
  with the guard in both engines and the `writeback`-by-layer counts as the
  evidence.
- **The re-validation rate is 0.1 % corpus-wide and 36–50 % on the puzzles the
  milestone's targets name** — 97.2 % on `zebra2-hints -e`. The acceptance
  criterion below said "≤ a few percent" and would have *passed on the
  average* while being wrong on every recognisable workload. It is restated
  per workload.
- **The speculation is wrong, not merely stale.** On **35** enterings it
  returns `alive` where the sequential engine returns `dead-post`: a mid-layer
  `(not h)` is a premise of `std.elim`, whose `domain-elimination` *asserts a
  positive* once every other value is excluded — not bookkeeping. And
  with `enable_fail_fast_fork` **off**, `core`-moved collapses exactly onto
  `kind`-moved (35 = 35), which names a question design/08 never asked —
  fail-fast × speculation — and hands it to
  [S1a.7.2](s1a.7.2_parallel_enterings.md).

And one thing the plan had right for the wrong workload: the entries with a
search big enough to need cores put **98.2–99.9 % of their enterings past
layer 1**, where the speculation is exact by construction. `zebra2 -e` puts
44.6 %. See § Acceptance for what that does to the scaling target.

### And the shape a parallel layer actually has

design/08 §2 speculates and then *repairs*. The stage also built and measured
the other shape — **test a batch against one KB, integrate what the batch
learned at a barrier** (`SolveOptions::integrate_every`) — because it is what a
parallel layer does whether or not anybody designs it, and because the two
properties it needs are testable without a thread. Both are now tests
(`ein-infer/tests/search_invariants.rs`, 6 of them, 2.15 s):

- **the answer does not depend on the entering order** — `lex`, `score-sum` and
  two seeded shuffles, 16 files, same verdict and same model set;
- **the answer does not depend on when the layer integrates** — a barrier every
  4 enterings and one per layer, same files, plus two five-layer searches of
  5 173 and 11 501 enterings — **and the two compose**, which is the case a
  parallel layer is.

The argument is [design/08 §2a](../design/08_parallelism.md#2a-deferred-integration--the-batch-synchronous-layer),
with the commutation identity `sat(B ∪ W ∪ c) = sat(sat(B ∪ c) ∪ W)` and the
asymmetry that matters: `dead` is monotone, so **a death found under deferred
integration is a real death and a *solution* is provisional**. The barrier
re-check that turns the measured model-set equality into a constructed one is
one re-entry per recorded solution node, and it is S1a.7.2's to build.

What it costs, all cells answer-identical:

| workload | sequential | whole-layer barrier |
|---|---:|---:|
| `zebra2 -e` | 101 enterings, 37 ms | **617**, 163 ms |
| `branching/06 -e` (0 writebacks) | 5 173, 263 ms | **5 173**, 259 ms |
| `branching/07 -e` (162 writebacks) | 11 501, **1 135 ms**, root depth **164** | **11 501**, **406 ms**, root depth **3** |

The last row is the finding nobody was looking for. **Every root write seals
another layer and every fork inherits the whole stack** — so coalescing a
layer's writes takes `branching/07 -e` from 1 135 ms to 406 ms **at
`--jobs 1`**, for the same enterings and the same answer. The mode is not a tax
on the workloads that want cores; on the deepest of them it is a 2.8× discount.

## Goal

Use the cores without giving up the byte gate. `--jobs 1` stays the
default and stays T3; `--jobs N` is **also** T3 — same verdict, same
models, same counters, same stdout — via speculate-and-validate; and
`--unordered` is an explicit opt-out for throughput.

Design: [design/08](../design/08_parallelism.md).

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.7.0](s1a.7.0_speculation_audit.md) ✅ | The speculation audit | 1 d |
| [S1a.7.1](s1a.7.1_sync_shared_state.md) | Making the shared state `Sync` | 3 d |
| [S1a.7.2](s1a.7.2_parallel_enterings.md) | Level 1: parallel enterings | 4 d |
| [S1a.7.3](s1a.7.3_parallel_boundary.md) | Level 3: the parallel boundary round | 2 d |
| [S1a.7.4](s1a.7.4_parallel_enqueue.md) | Level 2: the parallel enqueue pass | 2 d |
| [S1a.7.5](s1a.7.5_jobs_contract.md) | The `--jobs` contract | 2 d |

## Acceptance for the phase

- `--jobs {1,2,4,8,16}` T3-identical on the whole corpus.
- A 10 000-run randomised stress of `--jobs 8` vs `--jobs 1` with no
  divergence.
- **≥ 6× on 8 P-cores** on the phase's measurement set — `branching/06 -e`,
  `branching/07 -e`, `saturation/square-bwd/houses -e`, `features/01 -e`:
  0.2–1.9 s runs with ≥ 98 % of their enterings past layer 1. **Restated
  2026-08-20 by [S1a.7.0](s1a.7.0_speculation_audit.md)**; it read "≥ 6× on 8
  cores for exhaustive zebra2's Phase 2 wall-clock", which was written when
  that run was 4.5 s. It is 31 ms today, 42.3 % of its firings are in the
  exactly-parallel part, and no engine change shows 6× on that. The two zebras
  stay as *parity* cells, which is what they have always been good for. And
  "8 cores" now has to name its core set: this machine has 8 P-cores at
  5.6 GHz and 16 E-cores at 4.1 GHz, and a scaling number that does not say
  which it used is not a measurement
  ([scaling.md](scaling.md#p1a7--the-scaling-measurements) preamble).
- Re-validation rate reported **per workload** and, on each, either ≤ a few
  percent or paid for by a mechanism whose cost is measured. **Restated
  2026-08-20**: the corpus average is 0.1 % and the puzzles that matter are
  36–50 %, so an average would have passed this criterion without meeting it
  (Q-M1a.7, [scaling.md §3](scaling.md#3-the-audit)).
- **Layers ≥ 2 run with no validator, and the engine asserts why** — no root
  write may occur between a layer opening and closing above layer 1. Today
  that is true because the writeback is singleton-only; it is an invariant the
  parallel path depends on and therefore one a debug assertion has to hold.
- TSan and `loom` clean on the shared structures.

## Risks

- **The validation argument is the whole phase.** `sat(base ∪ W ∪ c) =
  sat(sat(base ∪ c) ∪ W)` holds because the KB is append-only and
  saturation is a least fixpoint. Write it down next to the code, with
  the fixture that would break if it were false (a layer-2 commitment
  whose fork reads a `(not h)` written mid-layer).
  **S1a.7.0 sharpened this and found the fixture is a layer-*1* one.** A
  layer-2 commitment cannot read a mid-layer write, because there are none
  above layer 1; the case-3 fixture is a layer-1 candidate that survives
  without `W` and dies with it, and 35 of them already exist in the corpus.
  The identity is also not the whole argument: it is about *fixpoints*, and
  `enable_fail_fast_fork` means a dying fork never reaches one — so the
  continuation recovers `kind` but recovers `core` only where the fork ran to
  quiescence ([scaling.md §3](scaling.md#3-the-audit)).
- **Memory scales with jobs.** N live forks = N deltas over one shared
  base; measure peak RSS at `--jobs 16` on the worst corpus entry
  (`enable_singleton_writeback=false`, 3 336+ enterings). **And the base is
  not the constant here.** [S1a.6.4](../p1a.6_performance/s1a.6.4_hypgen_and_lattice.md)
  measured the corpus's slowest `solve` cells, which no P1a.6 target covers:
  `features/01_not_and_absent -e` peaks at **724 MB** at `--jobs 1`, and an
  uncapped `saturation/square-unique/terminus.ein -e` reaches **12.3 GB** and
  was OOM-killed on the dev machine — ~1 KB per entering, growing linearly,
  over ~12 M enterings. That is the *search*'s state, not a fork's delta, and
  it is what a job count multiplies against a machine's RAM
  ([baseline.md §15](../p1a.6_performance/baseline.md#15-s1a64--the-per-call-setup-and-the-enumerator-the-targets-never-run)).
- **Speculative waste at `stop_after`.** Bounded by the job count, but
  measure it: a `-n 1` solve that speculates 16 enterings to use 1 is
  fine; one that speculates 16 layers is not.

## Cross-links

- [design/08 — Parallelism](../design/08_parallelism.md)
- [design/03 §5 — `Arc<KbCore> + Delta`](../design/03_data_model.md)
- [scaling.md](scaling.md) — the phase's measurements, in
  [baseline.md](../p1a.6_performance/baseline.md)'s shape
- [`utils/spec_audit.py`](../../../utils/spec_audit.py) — S1a.7.0's sweep;
  `ein-infer/src/spec_audit.rs` is the instrument it drives
