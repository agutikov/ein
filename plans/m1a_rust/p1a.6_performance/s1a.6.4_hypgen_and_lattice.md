# S1a.6.4 — Hypgen and lattice hot paths

**Phase:** P1a.6 (Performance)
**Estimate:** 3 days
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** refinements to
[design/07](../design/07_search_layer.md) §§2, 4

## Context

The search layer was ported for fidelity, not speed: hypgen still
enumerates ~18 k candidates per full call, `complete()` still asks the
generator for its first element, and the alive set is still recomputed
from scratch per layer. All of that is correct and all of it is
constant-factor work that integers make cheap — but only if the
allocations go away too.

The constraint that shapes every task here: **`HypGenStats` attribution
is a T1 observable**, so nothing may change which filter drops a
candidate, or in what order.

## Acceptance

- T1 identical: every `HypGenStats` key, every entering counter.
- T2 identical on the branching fixtures.
- Measured: candidate-enumeration cost per `complete()` call and per
  `open_hypotheses()` call, before and after.
- Zero allocations for a rejected candidate.

## Tasks

### Task T1a.6.4.1 — Intern-on-probe

Compute a candidate's row key and run the two bit tests
(`negated_fact`, `fact_already_exists`) *before* interning or
materialising anything. Only a candidate that survives them becomes a
`FactId`. This is the difference between 18 k interns and ~100 per call.

Care: the `seen_in_call` dedup also wants a key. Use the row key's hash
plus an equality check against a small open-addressed table, rather than
interning to get a `FactId` to dedup on.

### Task T1a.6.4.2 — Hoisted candidate-object list

Land the hoist from
[S1a.4.1](../p1a.4_search_layer/s1a.4.1_hypothesis_generation.md)
T1a.4.1.1 if it was deferred, and cache the derived sets — the type-role
atoms (from relation signatures) and the reserved names — on the
`Program`, since they cannot change after load.

### Task T1a.6.4.3 — Relation/slot precomputation

The `(relation, slot)` enumeration order and the per-relation skip
verdicts (`closed`, whitelist, blacklist) are recomputable only when the
`(__closed__ R)` extent or the query changes. Cache them with a version
counter, rebuilt on change. Must preserve the *pre-candidate counter*
bumps exactly — a skip that is cached still has to be counted.

### Task T1a.6.4.4 — No-good bitmask

Land the ≤64-alive bitmask representation from
[S1a.4.3](../p1a.4_search_layer/s1a.4.3_apriori_and_nogoods.md) if it
was deferred, and measure it in the regime where it matters:
`enable_singleton_writeback=false` on zebra2, where the search explodes
to 3 336+ enterings and the clause set grows with it.

### Task T1a.6.4.5 — Incremental alive maintenance

`_compute_alive` re-runs the whole generator per layer. The alive set
changes only by: a candidate becoming a fact, a `(not h)` writeback, or a
forced-positive promotion — all of which are single-fact events the
engine already knows about. Maintaining the set incrementally is
plausible, but it is the riskiest task here: the generator's output is
defined by the *pipeline*, not by a set difference, and the lookahead
filter is not monotone in an obvious way.

Do it **only** if the profile still shows `_compute_alive` after the
tasks above, and gate it behind a debug assertion that recomputes from
scratch and compares, run in every conformance build.

### Task T1a.6.4.6 — `complete()` fast path

`complete()` needs only *one* surviving candidate. Order the enumeration
so the cheapest-to-survive candidates come first — **no**: that changes
which candidate is found, and `complete` is a boolean, so it would not
change the answer... but it *would* change the kill-cache writes the
lookahead makes along the way, which are root-visible facts. So: leave
the order alone, and make the per-candidate cost lower instead. Recorded
here because it is a tempting change that is wrong.

## Notes

- The profile's `hypgen/branch` row in `utils/profile_solve.py` is
  inflated by nested `saturate` calls; use the Rust profile's own
  attribution, not that rollup.
- If hypgen does not appear in [S1a.6.1](s1a.6.1_profile_baseline.md)'s
  top five, skip tasks 3–5 and say so in the stage log.
