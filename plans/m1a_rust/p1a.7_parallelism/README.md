# P1a.7 — Parallelism

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **resumed 2026-08-22**, two stages in.
[S1a.7.0](s1a.7.0_speculation_audit.md) shipped 2026-08-20 — a stage the plan
did not have, because the phase's central risk was measurable *before* any of
it was built. The phase then **paused for two days** at the user's direction
while [P1a.8](../p1a.8_binary_container/README.md)–[P1a.10](../p1a.10_single_implementation/README.md)
ran, and the pause is why § The acceptance, restated exists: P1a.10 retired the
instrument four of the five remaining stages wrote their acceptance in terms
of. On resumption that restatement came first, then
[S1a.7.1](s1a.7.1_sync_shared_state.md), which measured *its* premise before
building and lost two of its six original tasks to the result.

**Nothing is half-built.** No engine code is in an intermediate state and no
scaffolding is waiting to be removed. What has shipped is two instruments
(`spec_audit`, `shared_state_probe`), two measurement documents, three
invariance tests, a batch-synchronous integration mode that is worth 2.8× at
`--jobs 1` on its own, a core-set-aware bench harness — and two engine changes
that are unconditional improvements rather than parallel scaffolding: the
engine's own names interned once instead of per firing, and a load pass that
does what the compiler would otherwise do mid-search.

## Resumed — what the interval changed

**The pause cost the phase its vocabulary and nothing else.** P1a.10 deleted
`ein-conformance`, `ein-oracle` and the `T0…T3` tiers, and this phase's
acceptance was written in them: "`--jobs {1,2,4,8,16}` T3-identical on the
whole corpus", "a 10 000-run randomised stress of `--jobs 8` vs `--jobs 1`",
and every tier reference in [S1a.7.2](s1a.7.2_parallel_enterings.md) through
[S1a.7.5](s1a.7.5_jobs_contract.md). The *invariants* were never affected —
`--jobs N` agreeing with `--jobs 1` compares one engine against itself and
never needed an oracle — so the restatement below is a change of instrument,
not of promise, and in one place it is a **stronger** promise than the tier
was: see the third row.

**The pause also gave the phase a better comparison than it had.** The tiers
were a *tolerance ladder* — T0 the verdict, T3 the bytes — designed for two
implementations that were allowed to differ in narration.
[`ein-parity`](../../../ein.rs/crates/ein-parity/src/lib.rs) is what survived
that argument, and it is a *cut* rather than a ladder: the verdict, the model,
the unsat core and **every search counter** are compared exactly, and what is
admitted is narration — a firing count, an event ordinal, a dying fork's
stopping point. [S1a.10.1](../p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md)
then showed the cut is not about two engines at all: permute one engine's id
space and the *same* renderings move — **51 of 3 160 permuted (file, op)
pairs** as of 2026-08-22, 31 of them a dying fork's stopping point and 20 the
derivation it narrates, **0 answers**. That is exactly the comparison
`--jobs N` needs, already built, already corpus-wide, and already priced.

**What is still undecided is unchanged**, and both items are written down
rather than left in someone's head, which was the point of pausing after an
instrument stage rather than in the middle of a refactor:
[S1a.7.2](s1a.7.2_parallel_enterings.md)'s four options for layer 1 — with
**(d) batch-synchronous integration the measured one** and the barrier
re-check of recorded solution nodes the piece it still owes — and the
fail-fast × speculation interaction, which design/08 never named.

## The acceptance, restated

One row per criterion. ✅ is an instrument that exists and runs in
`cargo test --workspace`; the rest are this phase's to build, and the point of
the table is that each now names *what* to build instead of naming a harness.

| the criterion, as it was written | what it named | what asserts it now |
|---|---|---|
| `--jobs {1,2,4,8,16}` **T3-identical** on the whole corpus | `ein-conformance --tier T3`, two processes diffed per corpus cell | **`ein-render/tests/jobs_invariance.rs`** — a third sweep over [`corpus_ops`](../../../ein.rs/crates/ein-render/tests/corpus_ops/mod.rs), in `id_order_invariance`'s shape: the manifest's 128 files × 45 ops, run at `--jobs 1` and at `--jobs N` in one process, compared through `ein-parity`'s cut |
| — its counter half | T1: every `enterings_*`, `saturate_count`, `nogoods_*`, the NAF and hypgen counters | `ein-cli/tests/summary_properties.rs` ✅, whose thirteen identities already run over every `solve` cell — extended to run the sweep at `--jobs N`. The cut above holds the counters *exactly*, so this is the belt to that braces |
| — its process half | T0/T3: exit code, stdout, `--json-summary` | `ein-cli/tests/corpus_cli.rs` ✅ — every declared cell as a process against a banked exit table — extended with a `--jobs` axis |
| — its byte half | T3: the rendered surfaces | the goldens ✅ (`golden_events`, `golden_trace`, `golden_dot`, `golden_dump`) re-run at `--jobs N`. These are **stricter** than the cut and are where a narration change is *supposed* to be visible |
| a **10 000-run randomised stress** of `--jobs 8` vs `--jobs 1` | the harness driving two processes per run | [`utils/fuzz_ein.py`](../../../utils/fuzz_ein.py) ✅, which kept its generator and lost its differ at [S1a.10.4](../p1a.10_single_implementation/s1a.10.4_utils.md) — a sixth property, *the same program at `--jobs 8` answers as it does at `--jobs 1`*, joining the five one engine can already check |
| — and the id-space arm nobody asked for | — | `id_order_invariance` ✅ with `EIN_ID_FILES` already points that sweep at generated input; `--jobs` composes with it, and the composition is the interesting run |

**The one place the restatement promises more.** T3 was *bytes*, and a byte
comparison cannot say which difference is allowed — it only says there is one.
The cut says which: the verdict, the model, the unsat core and every counter
are exact, and the narration renderings are named, closed, and counted under
`EIN_PARITY_STRICT=1` — 51 of 3 160 today. So "T3-identical" becomes "**identical except
where a permuted id space already moves, and no wider**", which is a criterion
a reviewer can check rather than a diff a reviewer can only read.

**And the one place it promises the same thing more cheaply.** T3 ran two
processes per corpus cell — 738 s of engine time for a full sweep. `corpus_ops`
runs in-process; `id_order_invariance` does the whole corpus twice — 3 160 permutations over
128 files × 45 ops — in **10.9 s**.
The nightly cadence [S1a.7.5](s1a.7.5_jobs_contract.md) T1a.7.5.3 asks for was
a consequence of the harness's cost, and may not survive contact with an
instrument that is 2 000× cheaper.

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

Use the cores without giving up the gate. `--jobs 1` stays the default;
`--jobs N` is **the same computation** — same verdict, same models, same
unsat core, same counters, and the same renderings up to the narration
[`ein-parity`](../../../ein.rs/crates/ein-parity/src/lib.rs) already admits —
via speculate-and-validate; and `--unordered` is an explicit opt-out for
throughput.

> Until 2026-08-22 this read "stays T3 … is **also** T3". The tier is gone with
> the harness; the promise is § The acceptance, restated, and it is the same
> promise made against an instrument that exists.

Design: [design/08](../design/08_parallelism.md).

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.7.0](s1a.7.0_speculation_audit.md) ✅ | The speculation audit | 1 d |
| [S1a.7.1](s1a.7.1_sync_shared_state.md) ◑ | Making the shared state `Sync` — **T1a.7.1.0/.1/.3 done**, [shared_state.md](shared_state.md) | 3 d |
| [S1a.7.2](s1a.7.2_parallel_enterings.md) | Level 1: parallel enterings | 4 d |
| [S1a.7.3](s1a.7.3_parallel_boundary.md) | Level 3: the parallel boundary round | 2 d |
| [S1a.7.4](s1a.7.4_parallel_enqueue.md) | Level 2: the parallel enqueue pass | 2 d |
| [S1a.7.5](s1a.7.5_jobs_contract.md) | The `--jobs` contract | 2 d |

## Acceptance for the phase

The first two items are restated in § The acceptance, restated, which names
the instrument for each half; they are repeated here in one line so the list
stays readable.

- `--jobs {1,2,4,8,16}` **is the same computation as `--jobs 1`** on the whole
  corpus — exact on the answer and every counter, and no wider in narration
  than a permuted id space already is.
- A 10 000-run randomised stress of `--jobs 8` vs `--jobs 1` with no
  divergence, as a sixth property of the fuzzer rather than a harness run.
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
  **The instrument for that shipped 2026-08-22**:
  [`utils/bench_env.sh`](../../../utils/bench_env.sh) `--cores P:8` / `PT:8` /
  `E:8` resolves and *reports* the set — `cpu0,2,4,6,8,10,12,14 — 8 cpu(s), 8
  physical core(s), all P` against `PT:8`'s `cpu0..7 — 8 cpu(s), 4 physical
  core(s)` — and refuses a spec the machine cannot fill rather than quietly
  giving fewer.
- Re-validation rate reported **per workload** and, on each, either ≤ a few
  percent or paid for by a mechanism whose cost is measured. **Restated
  2026-08-20**: the corpus average is 0.1 % and the puzzles that matter are
  36–50 %, so an average would have passed this criterion without meeting it
  (Q-M1a.7, [scaling.md §3](scaling.md#3-the-audit)).
- **The shared state carries no lock the measurement did not ask for.**
  [shared_state.md](shared_state.md): the interner and the integer pool are
  shared by `&` because they do not grow during a search
  (`ein-infer/tests/interning.rs`), and the fact store assigns **41 to 417 ids
  per search** against 5.8–26 M reads, so whatever it gets must leave the read
  path alone.
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
- [shared_state.md](shared_state.md) — S1a.7.1's, in scaling.md's shape:
  what a worker shares, and how hard each of the four structures is hit
- [`utils/spec_audit.py`](../../../utils/spec_audit.py) — S1a.7.0's sweep;
  `ein-infer/src/spec_audit.rs` is the instrument it drives
- `ein-infer/examples/shared_state_probe.rs` — S1a.7.1's, and the four
  `fact_*` counters it reads
