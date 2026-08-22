# P1a.7 — Parallelism

**Milestone:** [M1a — Rust port](../README.md)
**Status:** **resumed 2026-08-22**, three stages in —
[S1a.7.1](s1a.7.1_sync_shared_state.md) is **closed** and
[S1a.7.2](s1a.7.2_parallel_enterings.md)'s first task is shipped.
[S1a.7.0](s1a.7.0_speculation_audit.md) shipped 2026-08-20 — a stage the plan
did not have, because the phase's central risk was measurable *before* any of
it was built. The phase then **paused for two days** at the user's direction
while [P1a.8](../p1a.8_binary_container/README.md)–[P1a.10](../p1a.10_single_implementation/README.md)
ran, and the pause is why § The acceptance, restated exists: P1a.10 retired the
instrument four of the five remaining stages wrote their acceptance in terms
of. On resumption that restatement came first, then
[S1a.7.1](s1a.7.1_sync_shared_state.md), which measured *its* premise before
building and lost **three of its eight** tasks to the result — the interner's
lock, the fact store's concurrent append and the multi-threaded stress, none of
them declined, each of them removed by a number.

**Nothing is half-built.** No engine code is in an intermediate state and no
scaffolding is waiting to be removed. What has shipped is three instruments
(`spec_audit`, `shared_state_probe`, `flatten_probe`), two measurement
documents, seven invariance tests, a batch-synchronous integration mode that is
worth 2.8× at `--jobs 1` on its own, a core-set-aware bench harness, the
fan-out predicate and the assertion that holds it — and
**four** engine changes that are unconditional improvements rather than
parallel scaffolding: the engine's own names interned once instead of per
firing, a load pass that does what the compiler would otherwise do mid-search,
the per-worker provenance region (`features/01 -e` from 684–708 MB to
85–91 MB), and the layer barrier's root coalesce (`branching/07 -e` **3.17×**).

**Not one thread yet, and that is deliberate.** Everything above is either a
measurement, a sequential improvement the measurements found on the way, or an
invariant the parallel path will depend on and that is cheaper to establish
while nothing depends on it. The first `std::thread::spawn` in the repo will be
[S1a.7.2](s1a.7.2_parallel_enterings.md) T1a.7.2.1's, against a baseline two
stages of measurement have already made 3.17× faster on the workload the
scaling target is quoted against — and against a predicate that already says,
in the code and in the gate, which layers it may have.

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

**What was still undecided when the phase resumed** was written down rather
than left in someone's head, which was the point of pausing after an
instrument stage rather than in the middle of a refactor:
[S1a.7.2](s1a.7.2_parallel_enterings.md)'s four options for layer 1 — with
**(d) batch-synchronous integration the measured one** and the barrier
re-check of recorded solution nodes the piece it still owed — and the
fail-fast × speculation interaction, which design/08 never named.

> **Both closed 2026-08-22, and neither by choosing between the options.** The
> layer-1 question is answered by *(c)*, sequential layer 1 — one more pass over
> the event stream found the writebacks are 248 of 248 in layer 1 and that `W`
> grows to the second-to-last candidate, so there is no head/tail split to take
> and no fanned-out layer has a `W` at all. That deletes the validator, and with
> it the fail-fast interaction, which was a question about *continued* forks.
> (d) is not taken and its finding is: the 2.8× was the layer stack, and
> T1a.7.2.0 takes it directly for 3.17×.
> [S1a.7.2 § The decision](s1a.7.2_parallel_enterings.md#the-decision--a-layer-is-fanned-out-iff-it-cannot-write-to-root).

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
  [S1a.7.2](s1a.7.2_parallel_enterings.md). **Answered 2026-08-22 by removal**:
  a layer that can write to root is not fanned out, so there are no continued
  forks for fail-fast to interact with. The 35 keep their value as the evidence
  that no read-set filter would have been sound.

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
one re-entry per recorded solution node — **and it is not built, because the
mode it protects is not taken**: deferral moves `enterings_total` (101 → 521
here), and a search counter differing by job count is what the acceptance
forbids. Under immediate integration nothing is provisional.

What it costs, all cells answer-identical:

| workload | sequential | whole-layer barrier |
|---|---:|---:|
| `zebra2 -e` | 101 enterings, 37 ms | **521**, 163 ms |
| `branching/06 -e` (0 writebacks) | 5 173, 263 ms | **5 173**, 259 ms |
| `branching/07 -e` (162 writebacks) | 11 501, **1 135 ms**, root depth **164** | **11 501**, **406 ms**, root depth **3** |

The last row is the finding nobody was looking for. **Every root write seals
another layer and every fork inherits the whole stack** — so coalescing a
layer's writes takes `branching/07 -e` from 1 135 ms to 406 ms **at
`--jobs 1`**, for the same enterings and the same answer. The mode is not a tax
on the workloads that want cores; on the deepest of them it is a 2.8× discount.

**And the discount outlived the mode.** The entering count is identical there,
so all 2.8× is the depth column and none of it is the deferral. S1a.7.2 takes
it by flattening root at the layer barrier — integration still immediate, no
prune deferred, no counter moved — as its **first** task, T1a.7.2.0.

> **Shipped 2026-08-22, and it is 3.17×.** `SolveOptions::coalesce_root_at`,
> `Kb::flatten()` at the layer barrier once root is three layers deep. Above
> the 2.8× predicted, because the barrier also takes `compute_alive`'s probes
> and `promote_forced_positives`' re-saturation off the deep stack. Entering
> counts identical over the 49 non-slow corpus files that reach a `solve -e`
> verdict; the whole gate green with **no `EIN_BLESS`**; peak RSS within
> 0.3 MB.
> [scaling.md §6](scaling.md#6-t1a720--the-layer-stack-coalesced-at-the-barrier).

## Goal

Use the cores without giving up the gate. `--jobs 1` stays the default;
`--jobs N` is **the same computation** — same verdict, same models, same
unsat core, same counters, and the same renderings up to the narration
[`ein-parity`](../../../ein.rs/crates/ein-parity/src/lib.rs) already admits.
`--unordered` is an explicit opt-out for throughput.

> It read "via speculate-and-validate" until 2026-08-22. There is no
> validation: [S1a.7.2](s1a.7.2_parallel_enterings.md) fans out **only layers
> that cannot write to root**, which is every layer above the first, and runs
> layer 1 sequentially — 0.016 % of the corpus's enterings, and 0.24 % of the
> firings of the one measurement-set workload that writes back at all.

> Until 2026-08-22 this read "stays T3 … is **also** T3". The tier is gone with
> the harness; the promise is § The acceptance, restated, and it is the same
> promise made against an instrument that exists.

Design: [design/08](../design/08_parallelism.md).

## Stages

| stage | title | est. |
|---|---|---|
| [S1a.7.0](s1a.7.0_speculation_audit.md) ✅ | The speculation audit | 1 d |
| [S1a.7.1](s1a.7.1_sync_shared_state.md) ✅ | Making the shared state `Sync` — **closed 2026-08-22**, three of eight tasks deleted by measurement and no lock built, [shared_state.md](shared_state.md) | 3 d → 4.5 d |
| [S1a.7.2](s1a.7.2_parallel_enterings.md) ◑ | Level 1: parallel enterings — **its layer-1 question is decided** (2026-08-22, on paper: a layer is fanned out iff it cannot write to root), which deleted the validator, the fail-fast ruling and two acceptance items; **T1a.7.2.0 shipped**, 3.17× at `--jobs 1`, and **T1a.7.2.8** with it — the predicate now exists, and asserts | 4 d → 3 d |
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
  (`ein-infer/tests/interning.rs`); the fact store is shared by `&` too,
  because **zero enterings append to it on four of the six workloads** and 7
  of 111 on the worst, all in the head of a layer. And the **provenance arena** — the structure design/08 §6 has no row
  for, written by 100 % of enterings, 2 135 093 records and 205 MB on
  `features/01 -e` — is **per-worker**, because none of those records is
  referenced when the solve ends. That claim is asserted from the read side in
  every debug build and from the holding side by
  `ein-infer/tests/provenance.rs`, and it is where the phase's "memory scales
  with jobs" risk actually was.
- **Layers ≥ 2 run with no validator, and the engine asserts why** — no root
  write may occur between a layer opening and closing above layer 1. Today
  that is true because the writeback is singleton-only; it is an invariant the
  parallel path depends on and therefore one a debug assertion has to hold.
  ✅ **T1a.7.2.8, 2026-08-22**: `Run::fan_out_this_layer` is the predicate and
  carries the argument, `phase2` holds root's fact count across a fanned-out
  layer, and `search_invariants`'s
  `only_layer_one_writes_a_fact_to_root_mid_layer` asks the same question from
  outside the engine over 20 files — including the four that write back, which
  it asserts **do** grow in layer 1, so the claim cannot pass by nothing
  happening.
- TSan and `loom` clean on the shared structures. **`loom` has nothing to
  model**, and that is [S1a.7.1](s1a.7.1_sync_shared_state.md)'s finding rather
  than an evasion: every structure design/08 §6 named ends up `&`-shared or
  per-worker, so there is no protocol. TSan still applies, and applies to the
  fan-out, which is where the first thread is.
- **The determinism lint is green**, with 39 reviewed annotations — twelve of
  them T1a.7.1.5's classification of the engine's identity-order *sorts*, which
  live in the lint's allow-list because they answer the lint's question. It was
  red until 2026-08-22, on six lines T1a.7.1.7 had added.
- **The layer barrier's root coalesce is answer- and traversal-neutral.**
  T1a.7.2.0: entering counts identical over the 49 non-slow corpus files that
  reach a `solve -e` verdict, in every threshold setting, and the gate green
  with no `EIN_BLESS`
  ([scaling.md §6](scaling.md#6-t1a720--the-layer-stack-coalesced-at-the-barrier)).

## Risks

- ~~**The validation argument is the whole phase.**~~ **Retired 2026-08-22 —
  there is no validation.** It was going to be: `sat(base ∪ W ∪ c) =
  sat(sat(base ∪ c) ∪ W)`, written next to the code with the fixture that
  would break if it were false. S1a.7.0 sharpened it and found the fixture is
  a layer-*1* one — 35 already exist in the corpus — and that the identity is
  about *fixpoints*, which `enable_fail_fast_fork` means a dying fork never
  reaches. [S1a.7.2](s1a.7.2_parallel_enterings.md) then removed the question
  instead of answering it: **a layer is fanned out iff it cannot write a fact
  to root**, so no fanned-out layer has a `W` and nothing is repaired. What
  replaces this risk is a narrower one — **the predicate could stop being
  true**, and a mechanism that wrote to root mid-layer above layer 1 would
  change nothing visible until a fork read it. That is why T1a.7.2.8 is a debug
  assertion and not a comment
  ([scaling.md §3a](scaling.md#3a-where-the-writebacks-are-inside-layer-1--and-the-split-that-is-not-there):
  248 of 248 writebacks in layer 1, over 8 158 205 enterings and five layers).
- **Amdahl on the zebra family, and only there.** Serialising layer 1 costs
  0.24 % of `branching/07 -e`'s Phase-2 firings and nothing at all on the
  other three workloads of the measurement set — but **40–94 %** on the zebras,
  whose layer 1 holds half their firings. No job count makes an exhaustive
  zebra fast, which [scaling.md §5.4](scaling.md#5-what-this-chooses) had
  already concluded from the other side when it moved the scaling target. The
  risk is not the number; it is quoting a zebra speedup as the phase's.
- **Memory scales with jobs.** N live forks = N deltas over one shared
  base; measure peak RSS at `--jobs 16` on the worst corpus entry
  (`enable_singleton_writeback=false`, 3 336+ enterings). **And the base is
  not the constant here.** [S1a.6.4](../p1a.6_performance/s1a.6.4_hypgen_and_lattice.md)
  measured the corpus's slowest `solve` cells, which no P1a.6 target covers:
  `features/01_not_and_absent -e` peaked at **724 MB** at `--jobs 1`, and an
  uncapped `saturation/square-unique/terminus.ein -e` reaches **12.3 GB** and
  was OOM-killed on the dev machine — ~1 KB per entering, growing linearly,
  over ~12 M enterings. That is the *search*'s state, not a fork's delta, and
  it is what a job count multiplies against a machine's RAM
  ([baseline.md §15](../p1a.6_performance/baseline.md#15-s1a64--the-per-call-setup-and-the-enumerator-the-targets-never-run)).
  **And most of it was one structure.** [T1a.7.1.7](s1a.7.1_sync_shared_state.md#task-t1a717--the-provenance-arena)
  found that `features/01 -e`'s peak was overwhelmingly a provenance arena
  nothing reclaimed until the run ended — 2 135 093 records, twelve of them
  live — and the per-worker region it built takes the same file from
  **684–708 MB to 85–91 MB** at `--jobs 1`. So the "~1 KB per entering" figure
  is a pre-S1a.7.1 number and `terminus.ein` is worth re-measuring before it
  is used to size anything ([shared_state.md §2c](shared_state.md#2c-what-the-region-did--the-after-column)).
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
