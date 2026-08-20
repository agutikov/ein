# S1a.7.2 — Level 1: parallel enterings

**Phase:** P1a.7 (Parallelism)
**Estimate:** 4 days
**Depends on:** [S1a.7.1](s1a.7.1_sync_shared_state.md),
[S1a.7.0](s1a.7.0_speculation_audit.md)
**Implements:** [design/08](../design/08_parallelism.md) §2
**Decides:** whether `--jobs N` keeps T3 through layer 1, and how
`enable_fail_fast_fork` interacts with a continued fork

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
`enterings_dead_post` — a T1 failure. The fix is speculate-and-validate,
and its correctness rests on an identity the engine already depends on:

> `sat(base ∪ W ∪ c) = sat(sat(base ∪ c) ∪ W)`
>
> because the KB is append-only and saturation is a least fixpoint.

The same identity is behind `is_stalled()`'s re-enqueue after external
writes and behind fail-fast's "inconsistent at firing *n* ⇒ inconsistent
at the fixpoint".

## What the audit changed

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
  below.
- **(d) is the measured option, and it is the one to beat.**
  `SolveOptions::integrate_every` already exists
  ([S1a.7.0](s1a.7.0_speculation_audit.md) T1a.7.0.5): test a batch of
  candidates against one KB, integrate what the batch learned at a barrier.
  Answer-identical over 16 files under 4 orders and 3 policies, **free** on the
  workloads that want cores (5 173 → 5 173 and 11 501 → 11 501 enterings), and
  **2.8× faster** on `branching/07 -e` at `--jobs 1`, because coalescing a
  layer's root writes takes root's layer stack from depth 164 to 3 and every
  fork walks that stack. What it costs is the prune it defers — 6.1× the
  enterings on `zebra2 -e`, recovered to 1.1× by batching at 20. What it does
  **not** yet have is the piece this stage owes it: by
  [design/08 §2a](../design/08_parallelism.md#2a-deferred-integration--the-batch-synchronous-layer),
  a death found under deferral is real but an **alive** verdict is provisional,
  and the one provisional verdict that reaches the answer is a recorded
  solution node. **Re-check those at the barrier** — one re-entry per solution,
  and the model set is exact by construction rather than by measurement.
  `stop_after` is the case that needs it most, and the case the tests do not
  cover.

## Acceptance

- `--jobs {2,4,8,16}` T3-identical to `--jobs 1` on the whole corpus.
- **Layers ≥ 2 take the no-validator path, and a debug assertion holds the
  invariant that lets them** — no root write between a layer opening and
  closing, above layer 1.
- The three validation cases each have a fixture that exercises them, and
  the fixture for case 3 is *constructed*, not hoped for. **It is a layer-1
  fixture** (S1a.7.0: a layer-2 commitment cannot read a mid-layer write,
  because there are none), and 35 real ones already exist in the corpus —
  `solve -e examples/zebra.ein` layer 1 entering 11 is the worked example in
  [scaling.md §3](scaling.md#the-speculation-is-wrong-not-merely-stale).
- Re-validation rate reported **per run**, against S1a.7.0's numbers as the
  before-column, so a mechanism that changes it is visible (Q-M1a.7).
- The fail-fast interaction is **decided in writing** with its cost measured,
  not left to the implementation.
- If the batch-synchronous route is taken: the **barrier re-check of recorded
  solution nodes** is implemented, and `stop_after` is covered by a test — the
  one case [S1a.7.0](s1a.7.0_speculation_audit.md)'s invariance tests
  deliberately do not claim.
- Speculative waste at `stop_after` bounded by the job count and
  measured.
- Peak RSS at `--jobs 16` on the worst corpus entry recorded.

## Tasks

### Task T1a.7.2.1 — Snapshot and fan out

At layer start, take `R0 = Arc::clone(root_core)` — free
([design/03](../design/03_data_model.md) §5). Run
`try_commitment_set(R0, c)` for every candidate on the pool, collecting
into an **index-ordered** vector (`collect_into_vec`, not an unordered
reduce).

This needs [S1a.4.4](../p1a.4_search_layer/s1a.4.4_commitment_primitive.md)
T1a.4.4.5's seam: the primitive takes `&Arc<KbCore>` and returns
everything, writing nothing to root.

### Task T1a.7.2.2 — Ordered commit

Walk candidates in canonical order and commit each result: bump the
stats counters, emit the no-good, apply the singleton writeback (adding
to the write set `W`), call the dumper hooks, record solution nodes,
check `stop_after`. Counters and events therefore appear in exactly the
sequential order.

### Task T1a.7.2.3 — Validation

Per [design/08](../design/08_parallelism.md) §2:

1. `W = ∅` → accept as computed. *This is all of layer 1*, where a
   learned clause can only concern the candidate that just died.
2. `c` intersects `{h : (not h) ∈ W}` → emit `dead-pre` directly with the
   frontier from the clash, exactly as the primitive's pre-check would.
3. otherwise → **continue** the fork's saturation with `W` as the delta
   (semi-naive seeding, [design/06](../design/06_saturation.md)). If
   nothing new is derived and no contradiction appears, the speculative
   result stands; if something is derived, the continuation *is* the
   corrected result.

Case 3 requires keeping the fork's saturator alive (queues, `_seen`,
`_fired`) until its result is committed. Budget that memory: it is the
main cost of the scheme.

### Task T1a.7.2.4 — Early stop

`stop_after` must cut at the same candidate. Commit in order, break
there, and cancel outstanding speculative work. Measure the waste: a
`-n 1` solve that speculates `jobs` enterings to use one is fine; one
that speculates a whole layer is not — chunk the fan-out when
`stop_after` is small.

### Task T1a.7.2.5 — Diagnostics

Report, under the existing `--stats`-adjacent surface: worker count,
speculative enterings computed vs committed, case-2 and case-3 counts,
and continuation firings. Without these, a regression in the validation
rate is invisible.

### Task T1a.7.2.6 — The stress test

10 000 randomised `--jobs 8` runs across the corpus, T3-diffed against
`--jobs 1`, run nightly. Include the `enable_singleton_writeback=false`
entry — that is where `W` is largest and case 3 is most likely.

## Notes

- Write the `sat(base ∪ W ∪ c) = sat(sat(base ∪ c) ∪ W)` argument in the
  code next to the validator, with the fixture that would break if it
  were false. A parallel scheme whose correctness lives only in a plan
  document decays.
- If the re-validation rate is high, the *first* refinement is a
  per-fork read-set of relations touched during saturation (cheap to
  record in the matcher's candidate lookup), not a finer-grained fact
  read-set. **S1a.7.0 makes this unpromising**: `W`'s facts are all
  `(not …)`, every zebra-family rule set reads `not`, and the 35 wrong
  speculations are wrong *because* their forks would have consumed a `W`
  fact. A relation-level read-set would clear almost none of them, and one
  that cleared any of the 35 would be unsound.
