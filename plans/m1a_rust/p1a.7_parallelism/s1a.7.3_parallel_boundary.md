# S1a.7.3 — Level 3: the parallel boundary round

**Phase:** P1a.7 (Parallelism)
**Estimate:** 2 days
**Status:** ✗ **DECLINED 2026-08-23** — the premise was measured before the
stage was built and does not hold
([scaling.md §9](scaling.md#9-levels-2-and-3-measured-before-they-are-built)).
Nothing here is deferred: the mechanism is not built, not gated off, and not
half-present. The document is kept because **the reasoning is the deliverable**
— what would re-open it is named below, and it is a workload rather than an
argument.
**Depends on:** [S1a.7.1](s1a.7.1_sync_shared_state.md)
**Implements:** [design/08](../design/08_parallelism.md) §4

## The premise, measured

> **Taken 2026-08-23, before any of the mechanism was built**, in the form
> [S1a.7.0](s1a.7.0_speculation_audit.md) and
> [S1a.7.1](s1a.7.1_sync_shared_state.md) established for this phase.
> § Context below is the stage as written and its opening sentence is from an
> engine that no longer exists.

**Three of the four workloads of the phase's measurement set never park a
single candidate.** `branching/06 -e`, `branching/07 -e` and
`saturation/square-bwd/houses -e` run 4 775, 10 932 and 21 700 boundary rounds
between them and every one returns at `admit_from_boundary`'s
`parked.is_empty()` line. `admit_from_boundary` is **0.0 %** of each of those
three profiles, and that zero is structural rather than a sampling artefact.

On the fourth, `features/01_not_and_absent -e` — the corpus's NAF fixture, and
the workload this stage would most expect to serve — it is **3.2 %**, and a
round judges a **median of one** parked candidate and never more than five.
T1a.7.3.1 fans out a round's `first_failing` queries *in chunks of `jobs`*;
there is nothing to put in the chunk.

The site is 18.9–30.2 % on the two zebras, whose rounds judge a median of 3 and
6 — still short of one chunk at `--jobs 8`, and
[§5.4](scaling.md#5-what-this-chooses) already moved them out of the scaling
set for being 30 and 47 ms runs whose layer 1 cannot be fanned out at all.

**And a round is shorter than a barrier.** 0.18 µs on `features/01 -e`, 2.8 and
4.8 µs on the zebras, against ~10 µs for a barrier on the pool — which
[§8a](scaling.md#8a-t1a724--the-early-stop-and-the-batch-that-was-flat) priced
while measuring something else.

**Why the premise moved, and it is not that the plan was wrong.**
[S1a.6.12](../p1a.6_performance/s1a.6.12_boundary_and_snapshot.md) gave the
boundary epoch invalidation: a round now re-judges only the candidates whose
watched relations moved. 3 216 rounds of `zebra -e` visit 248 043 of the
947 758 candidates a copying round would have handled, and stop early on the
admission that ends each one. **That is the bulk this stage proposed to spread
over cores, and an earlier stage already deleted it** — incrementality and
parallelism compete for the same work.

**Declined on these numbers** (2026-08-23), the way
[S1a.7.1](s1a.7.1_sync_shared_state.md) lost three of its eight tasks: removed
by a measurement, not by a preference. § Notes below pre-authorised the softer
half of this ("gated off by default with the number recorded"); the numbers say
the gate would never open, and a mechanism whose gate never opens is a
mechanism with tests, a config field and no caller.

**What would re-open it** is a workload that parks in bulk — the predicate is
"does a boundary round have `jobs` candidates to judge", not "is the boundary
expensive". [M1c P1c.2](../../m1c_external_validation/p1c.2_external_benchmarks/README.md)'s
external benchmark corpus is where one would come from, and re-taking the two
tables above is a morning's work at that point.

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

> **Restated 2026-08-22.** Every "T*n*-identical" below named
> `ein-conformance`, which [P1a.10](../p1a.10_single_implementation/README.md)
> retired with the second engine. The successor per half is the phase
> [README § The acceptance, restated](README.md#the-acceptance-restated); the
> promise is unchanged and in one place stronger, because the cut names which
> differences are admitted where a byte diff could only say that there was one.

- `park` / `admit` / `retire` event sequences identical to `--jobs 1` on
  the whole corpus — `corpus_ops`' `Op::Saturate` is the whole verbose stream
  and is compared **byte for byte** (`ein-parity` does not cut it: design/01
  §5's relaxation never touched the event stream), so this half needs no new
  instrument beyond the `--jobs` axis.
- `naf_rounds` / `naf_admitted` / `naf_retired` identical — the four NAF
  counters are already row 1.4 of the [oracle
  ledger](../p1a.10_single_implementation/oracle_ledger.md) and already
  asserted by `summary_properties.rs`.
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
