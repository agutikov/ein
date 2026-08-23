# S1a.7.4 — Level 2: the parallel enqueue pass

**Phase:** P1a.7 (Parallelism)
**Estimate:** 2 days
**Status:** ✗ **DECLINED 2026-08-23** — the premise was measured before the
stage was built: right about the share, wrong about the width
([scaling.md §9](scaling.md#9-levels-2-and-3-measured-before-they-are-built)).
Nothing here is deferred; the document is kept for the reasoning and for the
predicate that would re-open it.
**Depends on:** [S1a.7.1](s1a.7.1_sync_shared_state.md)
**Implements:** [design/08](../design/08_parallelism.md) §3

## The premise, measured

> **Taken 2026-08-23, before any of the mechanism was built**, in the form
> [S1a.7.0](s1a.7.0_speculation_audit.md) and
> [S1a.7.1](s1a.7.1_sync_shared_state.md) established for this phase.

**This stage's self-assessment is wrong in the direction that flatters it.**
§ Context calls it "the smallest of the three engine-level parallel wins";
`enqueue_pass` is **10.6–31.2 % of a solve on every workload measured**,
including all four of the phase's measurement set, where
[S1a.7.3](s1a.7.3_parallel_boundary.md)'s site is 0.0 % on three of them. By
share, the two stages should swap places.

**By fan-out width they should both go.** T1a.7.4.1 schedules one task per plan
(full pass) or per `(delta fact, plan)` pair (delta pass), and the two counters
added for this question — `enqueue_pass`, `enqueue_task`, and `enqueue_task_full`
to keep the two kinds apart — say what one pass has to hand out:

| workload | passes | tasks | of which full | **tasks per pass** | share |
|---|---:|---:|---:|---:|---:|
| `zebra2 -e` | 1 945 | 89 042 | 125 | **45.7** | 25.3 % |
| `zebra -e` | 6 587 | 71 695 | 32 | **10.9** | 31.2 % |
| `branching/07 -e` | 66 946 | 204 220 | 4 | **3.1** | 20.7 % |
| `branching/06 -e` | 27 481 | 80 480 | 4 | **2.9** | 12.8 % |
| `sq-bwd/houses -e` | 118 790 | 197 811 | 1 | **1.7** | 30.2 % |
| `features/01 -e` | 665 801 | 900 254 | 2 | **1.4** | 10.6 % |

The `full` column is 1–4 outside the zebras — one per compiled plan, once — so
those means *are* the delta-pass width. **1.4 to 3.1 tasks on the measurement
set**, against a fan-out of 8.

**And a pass is shorter than a barrier**: 0.26 µs on `features/01 -e`, 0.64 on
`houses -e`, 0.87 on `branching/07 -e`, against ~10 µs for a barrier on the
pool ([§8a](scaling.md#8a-t1a724--the-early-stop-and-the-batch-that-was-flat)
priced that while measuring something else). T1a.7.4.4's threshold would
therefore be met on the two zebras and nowhere else — which is § Context's own
escape hatch, "if the threshold ends up *never*, that is a legitimate result",
arriving with a number.

**Why the premise moved.** The stage was written against a *full* re-match per
firing. [S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md)'s
semi-naive re-evaluation replaced it with delta seeding, on the finding that
"91 % of matcher output was re-discovery a full re-match would recompute" —
and that 91 % is exactly the bulk T1a.7.4.1 proposed to spread over cores.
**Incrementality and parallelism compete for the same work**, and here the
incremental version shipped three phases ago.

**Declined on these numbers** (2026-08-23), beside S1a.7.3's. What would re-open it is a program whose delta passes are wide — the
zebras' 45.7 shows the shape exists — and the predicate to re-take is `tasks
per pass ≥ jobs`, not `share ≥ x %`.

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

> **Restated 2026-08-22.** Every "T*n*-identical" below named
> `ein-conformance`, which [P1a.10](../p1a.10_single_implementation/README.md)
> retired with the second engine. The successor per half is the phase
> [README § The acceptance, restated](README.md#the-acceptance-restated); the
> promise is unchanged and in one place stronger, because the cut names which
> differences are admitted where a byte diff could only say that there was one.

- `enqueue` event sequence identical to `--jobs 1` (same order, same
  tiebreakers, same park/queue routing) — as [S1a.7.3](s1a.7.3_parallel_boundary.md),
  through `Op::Saturate`'s uncut stream.
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
