# S1a.7.3 — Level 3: the parallel boundary round

**Phase:** P1a.7 (Parallelism)
**Estimate:** 2 days
**Depends on:** [S1a.7.1](s1a.7.1_sync_shared_state.md)
**Implements:** [design/08](../design/08_parallelism.md) §4

## Context

`_admit_from_boundary` is 72 % of an exhaustive solve under ein.py, and
even after [S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md)'s
semi-naive re-evaluation it is likely to remain the largest single site.
It is also, structurally, the easiest thing in the engine to
parallelise: guard evaluation is a **read-only query against a quiesced
KB**, and the world cannot change during a round because at most one
candidate is admitted and admitting ends the round.

The determinism comes from where the *choice* is made: tasks compute
per-candidate verdicts concurrently; an ordered scan afterwards picks
the first passing candidate in priority/FIFO order. Which task finishes
first is irrelevant.

## Acceptance

- `park` / `admit` / `retire` event sequences identical to `--jobs 1` on
  the whole corpus.
- `naf_rounds` / `naf_admitted` / `naf_retired` identical.
- Speedup measured on the boundary specifically (the `boundary` criterion
  bench), not just on end-to-end wall-clock.
- Wasted evaluations (candidates judged after the eventual winner)
  reported and bounded by the chunk size.

## Tasks

### Task T1a.7.3.1 — Chunked evaluation

Take the parked candidates in priority/FIFO order, filtered to the dirty
set ([S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md)
T1a.3.4.7). Evaluate them in chunks of `jobs`: run the chunk's
`first_failing` queries concurrently, then scan the chunk's results in
order. If a chunk contains a pass, admit the earliest passer and stop; if
not, apply the chunk's retirements and stamps and move to the next chunk.

Chunking (rather than fanning out the whole parked set) bounds the waste
without changing the outcome.

### Task T1a.7.3.2 — Shared per-round memo

The per-round guard memo
([S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md)
T1a.3.4.6) becomes concurrent: `(guard, projected_env) → verdict` in a
sharded map, computed at most once per round. Two candidates sharing a
guard and a projected environment then cost one query even across
threads.

Watch for the classic trap: a memo that a thread populates *after*
another thread already computed the same entry is fine (same answer,
wasted work), but a memo that returns a *stale* entry across a round
boundary is not. Key the memo by round number and clear it, or rebuild
it per round.

### Task T1a.7.3.3 — Read-only assertions

Guard evaluation must not write. Enforce structurally: the evaluation
path takes `&Kb`, and in debug builds a fact-count check before and after
each round asserts nothing was added.

### Task T1a.7.3.4 — Retirement and stamps

Retirements (an anti-monotone guard that failed) and watch-version
stamps are per-candidate side effects. Collect them per task and apply
them **in candidate order** during the ordered scan, so the parked set's
state evolves exactly as it does sequentially.

### Task T1a.7.3.5 — Interaction with level 1

A boundary round runs *inside* a fork's saturation, which is itself
running on a worker under
[S1a.7.2](s1a.7.2_parallel_enterings.md). Nested parallelism needs a
policy: either a shared global pool with work-stealing (rayon's default,
and fine — the tasks are independent), or level-3 parallelism disabled
when level 1 is already saturating the cores. Measure both; the simple
answer is usually "one pool, let it steal", with level 3's chunk size
tuned down when many forks are live.

## Notes

- The soundness reason for admitting one candidate per round is
  unchanged and unaffected by parallelism: batching would let one
  admission invalidate another's already-taken verdict, which on
  `p ← absent q; q ← absent p` derives both. Parallel *evaluation* is
  fine; parallel *admission* is not.
- If this stage does not show a speedup, check whether
  [S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md)
  already made rounds so cheap that the per-round fan-out overhead
  dominates. That would be a good outcome, and the stage should then be
  gated off by default with the number recorded.
