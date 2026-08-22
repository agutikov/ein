# S1a.7.2 — Level 1: parallel enterings

**Phase:** P1a.7 (Parallelism)
**Estimate:** 4 days → **3 d.** The layer-1 question was decided on paper
before any of the mechanism was built (§ The decision), which deleted the
validator, the fail-fast ruling and two acceptance items. **.0, .8, .1 and .2
are shipped**: the layer stack coalesced (3.17× at `--jobs 1`), the predicate
asserted, the seam, and the fan-out with its ordered commit —
**3.16–4.30× on 8 P-cores**, the same computation on all 47 corpus entries and
byte-identical event streams. **.7** shipped with them — the layer's own serial
work, which the fan-out's measurements found and which turned out to be three
things. What is left is .4 (early stop), .5 (diagnostics) and .6 (the stress);
the target is **1.5×** away and that is now the fan-out's own efficiency rather
than a serial fraction
([scaling.md §8 § Where the other 1.5× is](scaling.md#where-the-other-15-is)).
**Depends on:** [S1a.7.1](s1a.7.1_sync_shared_state.md),
[S1a.7.0](s1a.7.0_speculation_audit.md)
**Implements:** [design/08](../design/08_parallelism.md) §2
**Decides:** whether `--jobs N` stays the same computation through layer 1,
and how `enable_fail_fast_fork` interacts with a continued fork —
**both settled 2026-08-22**, before any of the mechanism was built: it does,
and there are no continued forks (§ The decision)

> **Re-shaped 2026-08-20 by [S1a.7.0](s1a.7.0_speculation_audit.md)**, which
> measured this stage's premise before the stage started. Read
> [scaling.md §3](scaling.md#3-the-audit) first; the short version is in
> § What the audit changed, below.

## Context

The main event. Phase 2 evaluates 101 independent enterings on exhaustive
zebra2 — and 3 336+ with `enable_singleton_writeback` off. Each is a
fork, a saturation and two detections, with root never mutated by the
entering itself (P1.21 R2). Embarrassingly parallel, except for one
thing: the sequential loop *does* write to root mid-layer, via the
singleton `(not h)` writeback, and a later entering in the same layer
forks a root that now contains that negative.

So the naive parallel layer changes `enterings_dead_pre` /
`enterings_dead_post` — and a search counter is the one thing
[`ein-parity`](../../../ein.rs/crates/ein-parity/src/lib.rs)'s cut does **not**
admit, in either direction. The fix is speculate-and-validate,
and its correctness rests on an identity the engine already depends on:

> `sat(base ∪ W ∪ c) = sat(sat(base ∪ c) ∪ W)`
>
> because the KB is append-only and saturation is a least fixpoint.

The same identity is behind `is_stalled()`'s re-enqueue after external
writes and behind fail-fast's "inconsistent at firing *n* ⇒ inconsistent
at the fixpoint".

## What the audit changed

> **These are [S1a.7.0](s1a.7.0_speculation_audit.md)'s findings, taken
> 2026-08-20 before any mechanism existed, and they stand.** What has moved is
> the *options* the last two open — settled below in § The decision, by one
> more measurement over the same event stream.

**The stage splits at the layer boundary.**

- **Layers ≥ 2 need no validator at all.** The writeback fires only for a
  singleton *commitment*, so layer 1 is the only layer that writes to root
  mid-layer — and 98.2–99.9 % of the enterings of every workload big enough
  to want cores are above it. That half of the stage is a fan-out and an
  ordered commit, with nothing to validate and nothing to get wrong.
- **Layer 1 is the whole of the difficulty**, and it is worse than the design
  assumed: a writeback every ~1.8 enterings on the zebras, a case-3 rate of
  36–50 %, and **35 speculations that come back `alive` where the sequential
  engine says `dead-post`**. The mid-layer `(not h)` is a premise of the
  domain-elimination rules, so a read-set filter that waved those forks
  through would be unsound.
- **Case 2 never fires** — 0 in 1 078 704 enterings. Layer 1's candidates are
  distinct singletons, so `(not h_j)` cannot name a later candidate, and no
  layer above has a `W`. Implement it (a future `W` writer need not have that
  shape), do not tune it.
- **`enable_fail_fast_fork` × speculation is an open correctness question**
  the design does not mention. With fail-fast off, the speculation's `core`
  errors collapse exactly onto its `kind` errors (35 = 35); with it on, 40
  further cores differ because the two forks stopped at different firings of
  the same death. The identity above is about **fixpoints**, and a dying fork
  under fail-fast never reaches one. So the continuation recovers `kind` —
  monotone growth, `W` only adds — and recovers `core` only where the fork ran
  to quiescence. **This stage has to settle it**, and the three ways out are:
  (a) run a continued fork to quiescence rather than fail-fast, and pay the
  ~88 % of a dying fork's saturation that fail-fast exists to skip; (b) accept
  a `--jobs`-scoped divergence in `core` and narration, the way
  [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
  accepted one for the fork, with a ledger entry and goldens; (c) run layer 1
  sequentially, which is exact and costs 42–53 % of a zebra's firings and
  0.1–1.8 % of everything else's; or (d) **batch-synchronous integration**,
  below. → **(c)**, and the question dissolves with it: there are no continued
  forks, so nothing interacts with fail-fast.
- **(d) is the measured option, and it is the one to beat.**
  `SolveOptions::integrate_every` already exists
  ([S1a.7.0](s1a.7.0_speculation_audit.md) T1a.7.0.5): test a batch of
  candidates against one KB, integrate what the batch learned at a barrier.
  Answer-identical over 16 files under 4 orders and 3 policies, **free** on the
  workloads that want cores (5 173 → 5 173 and 11 501 → 11 501 enterings), and
  **2.8× faster** on `branching/07 -e` at `--jobs 1`, because coalescing a
  layer's root writes takes root's layer stack from depth 164 to 3 and every
  fork walks that stack. What it costs is the prune it defers — 5.2× the
  enterings on `zebra2 -e`, recovered to 1.1× by batching at 20. What it does
  **not** yet have is the piece this stage owes it: by
  [design/08 §2a](../design/08_parallelism.md#2a-deferred-integration--the-batch-synchronous-layer),
  a death found under deferral is real but an **alive** verdict is provisional,
  and the one provisional verdict that reaches the answer is a recorded
  solution node. **Re-check those at the barrier** — one re-entry per solution,
  and the model set is exact by construction rather than by measurement.
  `stop_after` is the case that needs it most, and the case the tests do not
  cover. → **Not taken**, because it moves `enterings_total` and the acceptance
  does not admit a search counter differing by job count. Its 2.8×, which is
  the layer stack rather than the deferral, is taken another way — T1a.7.2.0.

## The decision — a layer is fanned out iff it cannot write to root

> **Decided 2026-08-22**, before any of the mechanism was built, by
> [scaling.md §3a](scaling.md#3a-where-the-writebacks-are-inside-layer-1--and-the-split-that-is-not-there).
> The four routes above were (a) continue-and-validate, (b) a `--jobs`-scoped
> divergence, (c) sequential layer 1, (d) batch-synchronous integration. **It is
> (c)** — and the measurement that chose it also deleted the question the other
> three existed to answer.

> A layer is fanned out **iff it cannot write a fact to root**: every layer
> ≥ 2, always; and layer 1 when `enable_singleton_writeback` is off. Layer 1
> with the writeback on runs sequentially, exactly as it does today.

Three numbers make it, and the third is the one nobody had:

1. **Only layer 1 writes** — **248 of 248** writebacks corpus-wide, over
   8 158 205 enterings spanning five layers. design/08 §2 argues it from the
   clause width; this is it counted, on every `.ein` that produces events.
2. **Layer 1 is 0.016 % of those enterings** — 1 343 of 8 158 205.
3. **And there is no split inside it.** `W` grows until candidate **55 of 56**
   on both zebras, 35 of 36 on the hints fixture, and **204 of 204** on
   `branching/07 -e`. The tail a fan-out could take exactly is one candidate,
   one, one and zero.

That third one is the finding, because the expectation was the opposite.
[T1a.7.1.2](s1a.7.1_sync_shared_state.md#task-t1a712--fact-store) found a
layer's *fact-id* appends clustered in its head — largest within-layer index 6,
21, 83 — and "run the head, fan out the tail" is exactly what that licensed for
the fact store. It does not transfer, and the two are not one quantity seen
twice: an appending entering interns a proposition **inside**
`try_commitment_set`, while the singleton writeback is a **commit-time root
write** that happens after the entering returns. Their distributions are
opposite. Only the measurement says so, and it cost one pass over an event
stream the engine already emits.

**What (c) costs**, on the phase's own measurement set: `branching/07 -e`, 203
sequential enterings — **1.8 % of enterings and 0.24 % of Phase-2 firings**;
`branching/06 -e`, `sq-bwd/houses -e` and `features/01 -e`, **nothing at all**,
because none of the three writes back and their layer 1 is therefore fanned out
too. A 0.24 % serial fraction admits **417×** by Amdahl — against a **6×**
phase target, and the bound is not close to binding. It is a bound on *Phase 2*
only: root saturation is serial too and is not in this denominator, which is
what [S1a.7.3](s1a.7.3_parallel_boundary.md) and
[S1a.7.4](s1a.7.4_parallel_enqueue.md) are for. The zebra family pays 40–94 %
of its Phase-2 firings and is the *parity* cell set rather than the measurement
set — which [scaling.md §5.4](scaling.md#5-what-this-chooses) had already
concluded from the other side, before this question was asked.

### What the decision deletes

- **The validator, all three cases.** Case 1 *is* a fanned-out layer, by
  construction rather than by check; case 2 has **0** occurrences in 1 078 704
  and no path to one; and **case 3 cannot arise**, because no fanned-out layer
  has a `W`. T1a.7.2.3 is deleted rather than deferred, and with it the case-3
  fixture, the continuation's live-saturator memory — design/08 §2 calls it
  "the main cost of the scheme" — and the read-set refinement in the Notes.
- **The fail-fast question, which was the phase's hardest.** It was a question
  about *continued* forks: the identity `sat(B ∪ W ∪ c) = sat(sat(B ∪ c) ∪ W)`
  is about fixpoints, and `enable_fail_fast_fork` means a dying fork never
  reaches one. With no continuations there is nothing for it to interact with.
  Fail-fast keeps its 1.9–2.4× untouched, no fork's `core` is read off a
  different firing than the sequential engine's, and
  [Q-M1a.7](../open_questions.md#q-m1a7--may---jobs--1-move-counters)'s fourth
  point is answered **by removal** rather than by a ruling.
- **The re-validation rate as a per-run diagnostic.** It is 0 by construction.
  What replaces it is an assertion (T1a.7.2.8), not a counter — a number that
  can only be zero is not a measurement.
- **The barrier re-check of recorded solution nodes.** That obligation was
  (d)'s: under deferral an alive verdict is provisional, and a recorded
  solution is the one provisional verdict that reaches the answer. Under (c) no
  fanned-out fork ever sees a smaller root than the sequential engine would
  give it, so nothing is provisional and `stop_after` needs only the ordered
  commit it needed anyway.

### What it does not take — and what survives of it

**(d) is rejected as the parallelism mechanism**, and for a reason that has
nothing to do with its merits: it moves `enterings_total` — 101 → 521 under a
whole-layer barrier, 111 at batch 20, on `zebra2 -e` — and a search counter
that differs between `--jobs 1` and `--jobs N` is precisely what the restated
acceptance does not admit. Applying one batch policy at *every* job count
would satisfy invariance, but that is a **traversal change to the sequential
engine**: not this phase's to take, and 5.2× the enterings at batch = ∞.

**Its finding survives, and is worth more than the mechanism was.** Depth
164 → 3 is **2.8×** on `branching/07 -e` at `--jobs 1` *with an identical
entering count* — so all of it is the layer stack and none of it is deferral.
The route that takes the depth win without deferring a single prune is to
**flatten root at the layer barrier**:

- answer-neutral, and not on anyone's word — `Kb::depth()` reaches
  `defer_probe`, `layout_shape`, `alloc_cost` and `search_invariants` and
  never an output, and `Kb::check_layering` is the standing invariant that a
  flattened KB and a layered one hold the same thing;
- integration stays **immediate**, so every writeback prunes exactly when it
  does today and no entering count moves;
- it covers every layer above the first — 11 297 of `branching/07 -e`'s
  11 501 forks — leaving only layer 1's own 204 walking a stack that is still
  growing under them.

It is two lines, it is worth 2.8× at `--jobs 1`, and it is now **T1a.7.2.0** —
the stage's first act, before any fan-out, because a stage that lands its
parallelism first would be measuring speedup against a baseline it could have
fixed.

> **Shipped 2026-08-22, and it is 3.17×** rather than 2.8×: the barrier also
> takes `compute_alive`'s probes and `promote_forced_positives`' re-saturation
> off the deep stack, which the deferral experiment could not separate out.
> Everything else held — same enterings, same answer, no golden re-blessed.

**(b) is not needed and therefore not taken.** Q-M1a.7's recommendation — no
counter movement, `--unordered` as the opt-in escape — stands unchanged, and is
now cheap rather than merely preferred.

## Acceptance

> **Restated 2026-08-22, twice.** Once because every "T*n*-identical" below
> named `ein-conformance`, which
> [P1a.10](../p1a.10_single_implementation/README.md) retired with the second
> engine — the successor per half is the phase
> [README § The acceptance, restated](README.md#the-acceptance-restated). And
> once because § The decision removed four of these items rather than meeting
> them. A criterion for a mechanism that is not built is not a weaker
> criterion; it is a category error, and it is struck through rather than
> deleted so the reason survives.

- `--jobs {2,4,8,16}` is **the same computation as `--jobs 1`** on the whole
  corpus: `jobs_invariance` over `corpus_ops`, exact on the verdict, the model,
  the unsat core and every search counter, and no wider in narration than
  `id_order_invariance` already measures. **Unchanged, and now the cheap
  criterion rather than the hard one** — a fanned-out layer computes against
  exactly the root the sequential engine gives it, so there is no repair that
  could be subtly wrong. ◑ **The property holds and the instrument is not
  built**: 47 corpus entries agree on exit code, stdout and every
  `--json-summary` field, the verbose event stream is byte-identical, and
  `search_invariants.rs` compares the whole `MonotonicStats` over 16 files —
  but `jobs_invariance` over `corpus_ops` × 45 ops, which is what the phase
  README's table names, is still the sweep to write.
- **The fan-out predicate is asserted, not assumed.** A debug assertion that no
  root fact write happens between a fanned-out layer opening and closing —
  which is the invariant the whole decision rests on, and which
  [scaling.md §3a](scaling.md#3a-where-the-writebacks-are-inside-layer-1--and-the-split-that-is-not-there)
  measured at 248 of 248 rather than argued. T1a.7.2.8. ✅ **shipped
  2026-08-22**, on root's fact count rather than on `depth()`, with the
  assertion verified to fire and the same claim asked from outside the engine
  by two `search_invariants` tests.
- **The sequential path is bit-identical to today's.** Layer 1 with the
  writeback on is not re-implemented, it is not entered — `corpus_shapes.rs`'s
  renderings and the goldens are the check, and no `EIN_BLESS` may be needed.
- ~~The three validation cases each have a fixture that exercises them, and the
  fixture for case 3 is *constructed*, not hoped for.~~ **There are no
  validation cases.** Case 1 is a fanned-out layer by construction, case 2 has
  0 occurrences in 1 078 704 and no path to one, and case 3 cannot arise. The
  35 real case-3 enterings in the corpus keep their value as the *evidence* for
  the decision — they are why no read-set filter would have been sound — and
  `solve -e examples/zebra.ein` layer 1 entering 11 stays the worked example in
  [scaling.md §3](scaling.md#the-speculation-is-wrong-not-merely-stale).
- ~~Re-validation rate reported **per run**, against S1a.7.0's numbers as the
  before-column.~~ **It is 0 by construction.** A counter that can only read
  zero is not a measurement; the assertion above is what would catch the day it
  could not.
- **The fail-fast interaction is decided in writing with its cost measured.**
  ✅ — and the decision is that it **does not arise**: the interaction was
  between fail-fast and a *continued* fork, and there are no continuations.
  Route (a) would have paid ~88 % of a dying fork's saturation to restore the
  fixpoint the identity needs; (c) pays nothing and keeps fail-fast's 1.9–2.4×
  whole. [Q-M1a.7](../open_questions.md#q-m1a7--may---jobs--1-move-counters).
- ~~If the batch-synchronous route is taken: the barrier re-check of recorded
  solution nodes is implemented.~~ **It is not taken**, and under immediate
  integration no alive verdict is provisional, so there is nothing to re-check.
  `stop_after` is still covered by a test, because the ordered commit's cut is
  the one thing the fan-out could get wrong (T1a.7.2.4).
- **The layer-stack flatten is answer-neutral, or it is not taken.**
  `corpus_shapes` and `search_invariants` are the check, and the entering count
  of every corpus file is unchanged — that is what separates it from deferral,
  which changes both. T1a.7.2.0. ✅ **shipped 2026-08-22**: the whole gate green
  with **no `EIN_BLESS`**, and the entering count identical over the 49 non-slow
  corpus files that reach a `solve -e` verdict, in all four threshold settings.
  3.17× on `branching/07 -e`
  ([scaling.md §6](scaling.md#6-t1a720--the-layer-stack-coalesced-at-the-barrier)).
- Speculative waste at `stop_after` bounded by the job count and measured.
  ◑ **Bounded, not yet measured**: the batch is one round of workers whenever
  `stop_after` or `max_enterings` is set, so nothing past the cut is speculated
  beyond `jobs` enterings. `JobStats::speculated - committed` is the number and
  T1a.7.2.4 is where it gets taken.
- Peak RSS at `--jobs 16` on the worst corpus entry recorded. ✅ **`features/01
  -e`: 79.8 MB at `--jobs 1`, 82.8 at 8, 90.3 at 16.** **The baseline moved** —
  [T1a.7.1.7](s1a.7.1_sync_shared_state.md#task-t1a717--the-provenance-arena)
  took that file from 684–708 MB to 85–91 MB at `--jobs 1`, so what this
  measures is a fork's delta rather than an arena nobody reclaimed. It is also
  the item that **found a bug**: with a whole layer in flight the same file
  peaked at **1.9 GB**, which is why the batch is bounded
  ([scaling.md §8](scaling.md#the-batch-is-a-memory-decision-before-it-is-a-scheduling-one)).

## Tasks

### Task T1a.7.2.0 — The layer stack, coalesced at the barrier ✅

**Shipped 2026-08-22, and it is 3.17× rather than the 2.8× that was predicted.**
`SolveOptions::coalesce_root_at: Option<usize>` — the depth at which the layer
barrier rebuilds root as one layer — defaulting to `Some(3)`, plus
`ein-infer/examples/flatten_probe.rs`, two counters behind `--features
counters`, and two tests. Numbers and the threshold's measurement:
[scaling.md §6](scaling.md#6-t1a720--the-layer-stack-coalesced-at-the-barrier).

Four things are worth carrying forward from it.

- **It went in at the barrier, after `dumper.layer_end` and before
  `compute_alive`** — not at the next layer's start, which was the other
  candidate. The extra 0.4× over the prediction is that placement:
  `compute_alive` forks root and `promote_forced_positives` **re-saturates**
  it, and both run below the barrier on whatever stack the layer left. The
  deferral experiment could not separate that out, because deferring collapsed
  the stack before those ran too.
- **The threshold is 3 and it is measured, not chosen.** A fork seals root's
  top, so a layer with no mid-layer write leaves depth 2; 3 is the first depth a
  writeback can produce. Over the 49 non-slow corpus files that reach a
  `solve -e` verdict, `Some(3)` fires on **four** — the two zebras, the hints
  fixture and `branching/07 -e`, which are exactly the four that write back —
  once each, copying ≤ 533 facts. `Some(2)` fires on 33 for no measurable gain,
  which is what a `bool` would have shipped.
- **The entering count is identical in every setting on every one of the 49**,
  which is the whole difference between this and route (d). The two tests that
  hold it are `coalescing_at_the_barrier_collapses_roots_layer_stack` — which
  also asserts that with the barrier *off* root still ends deeper than 100, so
  it cannot pass vacuously the day the writebacks go — and
  `coalescing_costs_no_prune_where_deferring_costs_many`, the same claim on the
  two zebras where the deferral's price is visible.
- **No `EIN_BLESS`.** `corpus_shapes`'s 5 178 renderings, the four golden sets
  and `summary_properties`'s thirteen identities are all unchanged, which is the
  acceptance item's real content.

What follows is the task as written, kept because the reasoning is what a
reader needs and it survived contact with the measurement.

**First, and before any thread.** Every root write seals a layer and every fork
inherits the whole stack: `branching/07 -e`'s 162 mid-layer writebacks put root
at depth 164 and all 11 501 forks walk it. S1a.7.0's `defer_probe` measured
depth 164 → 3 as **1 135 → 406 ms** for the *same* 11 501 enterings and the
same answer — so the 2.8× is the stack, not the deferral that produced it.

Flatten root at the layer barrier — `Kb::flatten()` in `Run::integrate`, or at
`layer_end` — and the same collapse happens with integration still immediate:
every writeback prunes when it does today, no entering count moves, and the
zebras' 35 → 3 and 34 → 3 come along.

Answer-neutrality is not an assumption here: `Kb::depth()` reaches
`defer_probe`, `layout_shape`, `alloc_cost` and `search_invariants` and never
an output, and `Kb::check_layering` is the standing invariant that a flattened
KB and a layered one hold the same thing. The check is `corpus_shapes` plus
`search_invariants` with no re-bless.

Measure the cost as well as the win: `materialise()` is O(facts) per layer, and
a search whose layers are cheap and whose root is large could pay more than it
saves. If it does, the flatten is per-layer-conditional on `depth()` rather
than unconditional — and that is a threshold with a measurement behind it, not
a constant.

### Task T1a.7.2.1 — Snapshot and fan out ✅

**Shipped 2026-08-22, in two commits: the seam, then the threads.**
`SolveOptions::jobs`, `ein solve --jobs N`, a `rayon` pool built once per solve
behind `ein-infer`'s `parallel` feature, and `Run::fan_out` /
`Run::speculate` / `Run::commit_entering` — the split the whole stage is
about. Numbers: [scaling.md §7](scaling.md#7-t1a721--the-seam-and-what-it-costs)
(the seam) and [§8](scaling.md#8-t1a721--the-fan-out-and-the-three-things-it-costs)
(the fan-out).

**It is the same computation, and three instruments say so.** All 47 non-slow
corpus entries that reach a `solve -e` verdict agree at `--jobs 1` and
`--jobs 8` on exit code, stdout and every `--json-summary` field — 0
divergences; the `--events --events-level verbose` stream is **byte-identical**
at both job counts, `branching/06 -e`'s 2 200 561 lines included; and
`search_invariants.rs` compares the whole `MonotonicStats` over 16 files at
`--jobs {2,4,8}`.

**It is 3.16–4.30× on 8 P-cores, against a ≥ 6× target.** That is the stage's
honest number, and the first fan-out was 2.19–2.89× — what closed the rest is
four things the *measurement* found, none of them designed for and all of them
sequential improvements too (T1a.7.2.7 and § Where the other 1.5× is). What is
left is the fan-out's own **~5× on 8 cores**: on `sq-bwd/houses -e` the serial
terms are 8 ms of a 60 ms run, so Amdahl would allow 7.5× and the fan is what
does not deliver it. The profile has no lock in it and 11 % is the allocator,
so that is a question about what a fork allocates —
[P1a.6](../p1a.6_performance/README.md)-shaped, not P1a.7-shaped. None of it is
S1a.7.3's or S1a.7.4's either, since those parallelise Phase 1, which is not in
this denominator — and
[scaling.md §8 § Where the other 1.5× is](scaling.md#where-the-other-15-is) is
where they are measured.

Five things are worth carrying forward.

- **The `&mut Terms` question was answered by lending, not by rewriting 99
  signatures.** `ein_core::terms::Table<T>` is `Own(T)` until `Terms::share`
  and `Shared(Arc<T>)` until `Terms::reclaim`; a lent table answers a lookup
  and refuses an assignment. The alternative spelling — `Arc<T>` in both states
  — was built first and is **4 % slower**, because `Arc::get_mut` proves
  uniqueness with a locked read-modify-write on a path that runs 2 318 815
  times to assign 417 ids.
- **The corpus does hand enterings back**, and it is a correction to
  [shared_state.md §2a](shared_state.md#2a-and-a-total-is-the-wrong-shape-of-number-for-it):
  that table measured `try_commitment_set` only, and the `complete()` probe's
  blind enumerator *numbers the candidates it walks*.
  `lattice/02_genuine_3set_death` hands three back per run, the committing
  thread re-runs them, and every counter still matches.
- **The batch is a memory decision.** A whole layer in flight is **1.9 GB** on
  `features/01 -e` against 84 MB sequential — every speculated result holds a
  fork's KB and its record region until the commit reaches it — and it was
  *slower* than `--jobs 1`. Bounded at `jobs × 32` it is 89 MB and 2.4×, and
  peak RSS at `--jobs 16` is 90.3 MB against 79.8 sequential.
- **A bounded batch makes the barrier's cost the thing to watch**, and
  `std::thread::scope` fails it: ~96 000 spawns on `features/01 -e` and a 3×
  slowdown at `--jobs 2`. The threads have to live between batches, which is
  what the pool is for.
- **The event ordinal is assigned at the commit.** A worker builds its line
  with a hole where `n` goes; `Events::replay` fills it in commit order. That
  is what makes the byte-identical stream above possible, and merging raw bytes
  would not have.
- **In a fan-out, freeing is work too, and it belongs to whoever allocated.**
  The result a worker hands back should carry only what the far side reads —
  `Entered::kb` is `None` and `Entered::firings` empty unless a solution, a
  `store_lattice` or a dumper will look. That is worth 9–47 % of the speedup,
  and it is the reason `Dumper::reads_forks` exists.

What follows is the task as written.

At layer start, take `R0 = Arc::clone(root_core)` — free
([design/03](../design/03_data_model.md) §5) — and run
`try_commitment_set(R0, c)` for every candidate on the pool, collecting into an
**index-ordered** vector (`collect_into_vec`, not an unordered reduce).

**Only for a layer that cannot write to root**: layers ≥ 2 always, and layer 1
when `enable_singleton_writeback` is off. Layer 1 with the writeback on takes
today's loop unchanged — which is 203 of 11 501 enterings on `branching/07 -e`
and none at all on the other three workloads of the measurement set.

This needs [S1a.4.4](../p1a.4_search_layer/s1a.4.4_commitment_primitive.md)
T1a.4.4.5's seam: the primitive takes `&Arc<KbCore>` and returns everything,
writing nothing to root.

### Task T1a.7.2.2 — Ordered commit ✅

**Shipped with T1a.7.2.1**, because the fan-out is not testable without it.
`Run::commit_entering` walks candidates in canonical order and commits each
result: bumps the stats counters, emits the no-good, calls the dumper hooks,
records solution nodes, checks `stop_after`. Counters and events therefore
appear in exactly the sequential order, and the check is the byte-identical
event stream above.

There is no `W` to accumulate here — a fanned-out layer has no singleton
writeback by construction. What the commit does own besides the counters is the
**event sink** and the **record region**:

- `events::Buffer` was `Rc<RefCell<Vec<u8>>>` and a worker could not hold one
  ([T1a.7.1.4](s1a.7.1_sync_shared_state.md#task-t1a714--kbcore--program-audit)).
  It now has the shape the counters have — a per-worker buffer replayed here —
  and the piece that made it exact is that the **ordinal is assigned at the
  replay**: `n` belongs to the stream, not to the thread.
- A fork's provenance region **travels with its result** and is installed
  around that result's commit (`ProvArena::swap_fork`). Keeping the base with
  the records is what makes it safe: an id issued inside a fork means something
  only against the region that issued it, and there is no way to install one
  entering's records and read another's.

### ~~Task T1a.7.2.3 — Validation~~ — deleted, not deferred

design/08 §2's three cases were the price of fanning out a layer that writes to
root. No layer that writes to root is fanned out, so the price is not paid:
case 1 is the whole of a fanned-out layer *by construction*, case 2 was 0 in
1 078 704 enterings and cannot occur where there is no `W`, and case 3 needs a
`W` to continue against. The continuation's live saturator — design/08 §2's
"main cost of the scheme" — is not built, and neither is the read-set
refinement the Notes below reserved.

The **identity** survives the deletion and is worth keeping written down, since
two shipped mechanisms rest on it: `sat(B ∪ W ∪ c) = sat(sat(B ∪ c) ∪ W)` is
what licenses `is_stalled()`'s re-enqueue after an external write and
`Saturator::resume`'s fork-entry delta. It is stated with its proof in
[design/08 §2a](../design/08_parallelism.md#2a-deferred-integration--the-batch-synchronous-layer)
claim 3.

### Task T1a.7.2.7 — The layer's own serial work ✅

**Named and shipped by T1a.7.2.1's measurements**, which is why it has a number
the plan did not reserve: with the fan-out in and the commit's frees moved to
the workers, the largest serial term in Phase 2 was no longer the commit — it
was candidate generation, 39.5 ms of `branching/07 -e`'s 109 ms at `--jobs 8`.

Timing its parts split it into three, and **only one of the three is
parallelism**:

- **`filter_candidate` fans out** — the easiest fan-out in the engine: it asks
  whether every element is still alive and whether any learned clause is a
  subset of the candidate, reads both by `&`, and writes nothing. Its cost is
  `candidates × clauses`, which is why the file with the lookahead *off* pays
  47.7 ms of it. Order is kept by an indexed **mask** rather than by trusting a
  filtered collect: a layer's candidate order *is* the traversal. → 47.7 → 8.3 ms.
- **`order_candidates` took a slice and cloned it.** A layer arrives in the
  join's emission order, which is already `cmp_set` order, so the sort is a
  linear scan and the 26 ms was the copy it needed somewhere to put. By value.
- **`record_node` promoted before it deduped.** `branching/06 -e` calls it
  **1 221 times to keep 22 nodes**, and each call was doing
  `Kb::promote_provenance` and `Kb::snapshot` for a record the dedup then threw
  away. The key is a function of the fact list and the promotion rewrites
  justification tables, so computing the key first makes a losing node cost a
  sort. The comment that put the promotion first said it beat *threading the
  fork region's lifetime through the decision* — and T1a.7.2.1's region travels
  with the entering, so there is nothing to thread. → 3.13× → **3.72×**.

The acceptance is the fan-out's and for the same reason: none of it may move a
counter. A candidate list that differed by job count would be a traversal
change, and `jobs_does_not_move_the_answer_or_a_counter` is where that shows.

**Two of the three make `--jobs 1` faster too** — 2–3 % on the files they touch
— which is the pattern this whole stage has: the parallel run is an instrument
that finds sequential waste, because it is the one place a serial millisecond
cannot hide.

### Task T1a.7.2.4 — Early stop

`stop_after` must cut at the same candidate. Commit in order, break there, and
cancel outstanding speculative work. Measure the waste: a `-n 1` solve that
speculates `jobs` enterings to use one is fine; one that speculates a whole
layer is not — chunk the fan-out when `stop_after` is small.

This is now the **only** place the fan-out can differ from the sequential
engine, which is why it keeps its own test rather than leaning on
`jobs_invariance`: an early stop is the one case S1a.7.0's invariance tests
deliberately do not claim.

### Task T1a.7.2.5 — Diagnostics

Report, under the existing `--stats`-adjacent surface: worker count,
speculative enterings computed vs committed, and how many enterings ran on the
sequential path because their layer could write to root.

The case-2 / case-3 / continuation-firing counters this task used to name have
nothing to count. What replaced them is the last column: it is 0 on three of
the four measurement-set workloads and 203 on the fourth, and a build where it
grew would be a build where the fan-out predicate had changed.

### Task T1a.7.2.6 — The stress test

10 000 randomised `--jobs 8` runs across the corpus, diffed against
`--jobs 1` through `ein-parity`'s cut — a **sixth property** of
[`utils/fuzz_ein.py`](../../../utils/fuzz_ein.py), beside the five one engine
can already check, rather than a harness run. Include the
`enable_singleton_writeback=false` entry: with the writeback off, layer 1 is
fanned out too, so that entry is the one that exercises the predicate's *other*
branch — where the old plan wanted it for having the largest `W`.

### Task T1a.7.2.8 — The predicate, asserted ✅

**Shipped 2026-08-22, and before the fan-out rather than with it** — the
invariant the decision rests on is worth having under the sequential engine,
where nothing depends on it yet and a violation is therefore a *finding* rather
than a bug report.

`Run::fan_out_this_layer(layer)` is the predicate — `layer > 1 ||
!enable_singleton_writeback` — and it carries the reasoning the Notes below ask
for: why a dead commitment of width *L* licenses a fact only at *L = 1*, the
248-of-248 that makes it a measurement, and why the escape hatch is a
`SolverConfig` lever rather than `layer > 1`. `phase2` compares **root's fact
count** across a fanned-out layer in a `debug_assert!`.

Three things are worth carrying forward.

- **It is the fact count and not `depth()`.** The task asked for both. `depth()`
  moves by one whether or not anything was written, because the layer's *first*
  fork seals root's top — so an assertion on it would have had to admit a
  slack of one, and an assertion with slack is not the one you want between a
  future `W` writer and a silently wrong search.
- **The assertion is live, and that was checked rather than assumed.** With the
  predicate temporarily forced to `true` it fires on the first corpus file:
  *layer 1 was fanned out and root grew from 378 to 410 facts while it ran*. A
  debug assertion that can only ever hold is indistinguishable from a comment
  until somebody makes it fail on purpose.
- **The outside-the-engine form is a test, and it is the one that says the
  claim is not vacuous.** `search_invariants.rs`'s
  `only_layer_one_writes_a_fact_to_root_mid_layer` watches root's fact count at
  every `layer_start` / `layer_end` — a window that closes *before*
  `compute_alive`, the forced-positive cascade and the lookahead kill cache, so
  a non-zero delta is a mid-layer write and nothing else — over the 16 search
  files plus the four that write back, and then asserts that those four **do**
  grow in layer 1. `with_the_writeback_off_no_layer_writes_to_root` is the
  predicate's other branch.

## Notes

- **Write the reasoning next to the predicate, not in this file.** ✅ A
  parallel scheme whose correctness lives only in a plan document decays. What
  is beside `Run::fan_out_this_layer` is why layer 1 is the only layer that can
  write — the clause a dead commitment of width *L* licenses is a *fact* only
  at *L = 1* — with 248-of-248 as what makes it a measurement rather than an
  argument, and T1a.7.2.8's assertion as what makes it fail loudly.
- ~~If the re-validation rate is high, the first refinement is a per-fork
  read-set of relations touched during saturation.~~ **Not reachable now**, and
  it would not have worked: `W`'s facts are all `(not …)`, every zebra-family
  rule set reads `not`, and the 35 wrong speculations are wrong *because* their
  forks would have consumed a `W` fact. A relation-level read-set would have
  cleared almost none of them, and one that cleared any of the 35 would have
  been unsound. Kept because it is the refinement a reader will think of first.
- **The one thing to re-open this decision for** is a `W` that is not the
  singleton writeback. The predicate is "can this layer write a fact to root",
  not "is this layer 1"; if a future mechanism writes to root mid-layer at any
  depth, T1a.7.2.8 fires, and *then* design/08 §2's validator is the design
  that was measured and costed. It is deleted from the build, not from the
  record.
