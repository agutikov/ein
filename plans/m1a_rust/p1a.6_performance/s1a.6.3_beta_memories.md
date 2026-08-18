# S1a.6.3 — Beta-memories (F11 D1)

**Phase:** P1a.6 (Performance)
**Estimate:** 4 days
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** [design/05](../design/05_matcher.md) §7
**Closes or updates:** [F11](../../followups/f11_deductive_layer_perf.md)
D1, Q-M1a.10

## Context

The named next rung on the Datalog ladder. The engine has walked
naive → semi-naive (participation index = alpha-memory; D2 delta-driven
enqueue; D5 seeded delta join), and the one thing D5 still recomputes is
the **intermediate join result**. A beta-memory materialises
`(plan, prefix-of-steps) → binding tuples` and extends it incrementally.

F11 parks it on a specific objection: *"a beta-memory is per-KB state,
and this engine forks KBs constantly; a memory that must be copied per
fork can lose more than it saves"* — with P1.8a's D3 (cross-fork carry,
built and reverted the same day) as the cautionary precedent.

[design/03](../design/03_data_model.md) §5's layered KB is the answer to
that objection, which is why this stage exists here rather than staying
parked. It is still a **gated** stage: it ships only if it is T2-green
and measurably better on *both* puzzles.

**What it is actually for, sharpened by
[baseline.md §9](baseline.md#9-the-fork-entry-re-derivation).** Every
entering re-derives the root's whole fixpoint — **94.6 %** of `zebra -e`'s
fork firings and **95.6 %** of `zebra2 -e`'s are redundant — and
`try_commitment_set` is 95.0 % of `zebra -e`. The cheapest way to remove
that is to stop *narrating* it, which is
[S1a.6.9](s1a.6.9_fork_entry_delta.md) and is **observable**
([Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)).
This stage is the *invisible* way to remove it: T1a.6.3.2's **root
memories** are exactly "compute the root's matches once and replay them
into every fork", producing the same firings in the same order from a
table instead of from a rescan. So the target is not "make matching
faster" — it is **make the 95 % that is re-derivation nearly free**, and
the § 9 tables are what the before/after is read against.

## Acceptance

- **T2 identical** on the whole corpus — same firing sequence, same
  order. A beta-memory that changes which of two equally-valid matches
  is found first is a divergence, not an optimisation.
- Measurably better on `zebra2 -e` **and** `zebra -e` (they have very
  different rule shapes: 19 plans vs 6). `zebra -e` is the one that
  decides — it is the workload that misses its milestone target, and the
  one whose fork re-derivation records **no** alternative justifications
  at all (`alt` = 0), so the memory has nothing to reproduce there beyond
  the match sequence itself.
- Memory: per-fork delta memory bounded and reported; peak RSS at
  `--jobs 1` no worse than 1.5× the pre-stage build.
- If either condition fails: **revert**, and update F11 with the numbers
  and the reason. That is a successful outcome for this stage.

## Tasks

### Task T1a.6.3.1 — The ordering argument, first

Before writing the memory, write down why enumerating from it reproduces
the nested-loop order, and what would break it. Sketch:

- a plan is left-deep; each `Scan`/`Join` iterates its extent in append
  order;
- a prefix memory stores tuples in *discovery* order, which for a
  left-deep plan over append-ordered extents is exactly the order the
  nested loops produce;
- appending on delta preserves that, because a delta fact is appended to
  the extent and its extensions are discovered after every earlier
  tuple's.

Then test it: a randomised differential harness that runs the same plan
with and without memories over randomised fact insertion orders and
compares the emitted match sequences.

### Task T1a.6.3.2 — Root memories

Build during root saturation, store in `KbCore`, share read-only by
`Arc` across every fork. Keyed `(PlanId, prefix_len)`; value is a packed
tuple array (registers bound at that prefix) plus the premise `FactId`s
needed to rebuild provenance.

### Task T1a.6.3.3 — Fork delta memories

Per-fork, holding only partial joins involving at least one fork-local
fact. Enumeration walks root-then-delta. A fork is dropped wholesale, so
there is no invalidation: the root memory is never invalidated within a
solve (append-only), and the delta memory dies with the fork.

### Task T1a.6.3.4 — Which prefixes to materialise

Not every prefix pays. Start with "the longest prefix whose tuple count
is below a threshold", measure, and consider a per-plan policy chosen at
compile time from the plan's shape (chain vs star). Keep the policy
explicit and dumpable — a memory whose population is mysterious is
impossible to debug.

### Task T1a.6.3.5 — Interaction with the semi-naive boundary

Guard sub-plans are also matched ([S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md)),
and after that stage they are seeded at deltas too. Decide explicitly
whether guards get memories; the safe default is **no** at first, then
measure.

### Task T1a.6.3.6 — Feature-gate and measure

Ship behind a `beta-memories` feature so the A/B is one build flag, and
report the matrix (on/off × both puzzles × fast/exhaustive) before
deciding the default.

## Notes

- The ordering argument in T1a.6.3.1 is the same argument
  [S1a.6.9](s1a.6.9_fork_entry_delta.md) needs for its delta seeding, from
  the other side: there, the claim is that every match over root-only
  facts already fired at root; here, that replaying them from a memory
  reproduces their order. Write it once and cite it twice.
- Q-M1a.10 asks whether this is still the largest lever *after* the
  register matcher and the semi-naive boundary. It may not be — those two
  removed the costs that made partial-join recomputation expensive.
  [S1a.6.1](s1a.6.1_profile_baseline.md)'s table decides whether this
  stage runs at all.
- D2 (worst-case-optimal join) stays out of scope: its trigger is a
  cyclic step graph whose match cost dominates, and ein's rule bodies are
  acyclic chains and stars where a left-deep plan is already optimal.
  Re-check the trigger here and record the answer; do not implement it.
