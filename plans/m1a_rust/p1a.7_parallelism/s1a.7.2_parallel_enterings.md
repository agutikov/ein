# S1a.7.2 — Level 1: parallel enterings

**Phase:** P1a.7 (Parallelism)
**Estimate:** 4 days
**Depends on:** [S1a.7.1](s1a.7.1_sync_shared_state.md)
**Implements:** [design/08](../design/08_parallelism.md) §2

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

## Acceptance

- `--jobs {2,4,8,16}` T3-identical to `--jobs 1` on the whole corpus.
- The three validation cases each have a fixture that exercises them, and
  the fixture for case 3 (a layer-2 commitment whose fork reads a
  `(not h)` written mid-layer) is *constructed*, not hoped for.
- Re-validation rate reported per run and ≤ a few percent on the corpus;
  above that, refine the read-set before shipping (Q-M1a.7).
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
  read-set.
