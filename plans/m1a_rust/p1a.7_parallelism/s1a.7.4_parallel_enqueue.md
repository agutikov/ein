# S1a.7.4 — Level 2: the parallel enqueue pass

**Phase:** P1a.7 (Parallelism)
**Estimate:** 2 days
**Depends on:** [S1a.7.1](s1a.7.1_sync_shared_state.md)
**Implements:** [design/08](../design/08_parallelism.md) §3

## Context

`_enqueue_pass` runs matchers and pushes candidates onto the queue. The
matching half is **read-only over the KB**; only the queue and the
`_seen` set are mutated. So the pass splits cleanly: match in parallel,
merge in canonical order.

Determinism comes from the merge. `_tiebreaker` is a monotone counter,
and nothing reads it during matching — so assigning tiebreakers *during
the merge*, in canonical order, reproduces the sequential sequence
exactly.

This is the smallest of the three engine-level parallel wins and the one
most likely to be dominated by overhead. It ships behind a work
threshold, and if the threshold ends up "never", that is a legitimate
result.

## Acceptance

- `enqueue` event sequence identical to `--jobs 1` (same order, same
  tiebreakers, same park/queue routing).
- A measured threshold above which the parallel pass wins, with the
  numbers; below it, the sequential path runs.
- No change to `_seen` semantics: the same candidates are deduped, and
  the counters that depend on it are identical.

## Tasks

### Task T1a.7.4.1 — Task decomposition

Two shapes, matching the two pass kinds:

- **full pass** — one task per plan, iterating the engine's plan list;
- **delta pass** — one task per `(delta fact, plan)` pair drawn from
  `pos_index`, plus the never-matched plans' full matches first.

Each task writes into its own buffer of `(bindings, premises, guards)`.

### Task T1a.7.4.2 — The canonical merge

Concatenate buffers in the sequential order: plan-list order for a full
pass; delta-fact order then plan order for a delta pass. Then, walking
that concatenation, apply `_seen` dedup, assign tiebreakers, and push to
`_queue` or `_parked`.

`_seen` stays single-threaded — it is consulted only during the merge —
which removes the need for any concurrent set here.

### Task T1a.7.4.3 — Buffer management

Per-worker reusable buffers (from
[S1a.6.2](../p1a.6_performance/s1a.6.2_memory_layout.md)'s arena pool)
so the fan-out does not trade matcher allocations for buffer
allocations. Bound the buffers: a plan that matches enormously should
stream rather than materialise, falling back to the sequential path.

### Task T1a.7.4.4 — The threshold

Estimate the pass's work before deciding (number of plans × candidate
bucket sizes for a full pass; delta size × plans for a delta pass), and
run sequentially below a measured cutoff. The estimate is cheap and its
value changes nothing observable, so it is free to tune.

### Task T1a.7.4.5 — Measurement

Bench the pass in isolation (root saturation of `zebra` — 502 facts, the
largest full pass in the corpus) and end-to-end. Report the fraction of
passes that took the parallel path; if it is tiny, gate the feature off
by default and say so.

## Notes

- Do **not** parallelise the firing loop itself. Firing mutates the KB,
  and the ordering it produces is the trace. The enqueue pass is
  parallelisable precisely because it is the read-only half.
- Nested under level 1 (a fork's saturation runs on a worker), this is
  third in priority for cores. If the pool is already saturated by
  enterings, the threshold should effectively disable it — measure
  rather than assume, but expect level 1 to matter far more.
