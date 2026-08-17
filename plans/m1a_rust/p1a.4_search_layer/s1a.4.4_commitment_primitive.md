# S1a.4.4 — The commitment primitive

**Phase:** P1a.4 (Search layer)
**Estimate:** 2 days
**Depends on:** [S1a.4.3](s1a.4.3_apriori_and_nogoods.md),
[P1a.3](../p1a.3_deductive_core/README.md)
**Implements:** `ein/inference/{commitment,frontier}.py`,
[design/07](../design/07_search_layer.md) §5

## Context

`try_commitment_set(root_kb, C)` is the one primitive the whole search
layer is built on: fork root, write every hypothesis in `C`, detect,
saturate, detect again. It is pure with respect to root — every
consequence stays in the fork (P1.21 R2) — and it is the unit that
[P1a.7](../p1a.7_parallelism/README.md) parallelises, so its purity is
not a nicety.

It is also where `enable_fail_fast_fork` lives: stopping a dying fork's
saturation at the firing that kills it rather than running to quiescence.
That is the one pure speed lever in the engine — same verdict, same
enterings, same deaths, same clauses — worth 1.9–2.4× on exhaustive
zebra2, because ~88 % of a dying fork's saturation happens after the
clash.

## Acceptance

- `enter` events identical for every commitment on every corpus entry:
  `kind` (`alive` / `dead-pre` / `dead-post`), firing count, and the
  unsat core as a fact set.
- With `enable_fail_fast_fork=false`, the firing prefix becomes the full
  run and `result.kb` the complete dead state — that path is exercised
  too, and its event trace also matches.
- `hypothesis_facts` are the writes for `C` only, not the saturator's
  additions.
- Two calls on the same root produce independent results sharing no
  mutable state (a test that mutates one fork and re-runs the other).

## Tasks

### Task T1a.4.4.1 — `CommitmentSetResult`

`commitment` · `kb` · `firings` · `kind` · `unsat_core` ·
`hypothesis_facts`. In Rust the `kb` is an owned `Kb` (base `Arc` +
delta), so returning it is free.

### Task T1a.4.4.2 — The primitive

Fork; write each `(rn, args)` with `Provenance::hypothesis(branch=0)`
through the index-maintaining add; **pre-saturation detect** (this
catches negatives that landed at root between candidate generation and
this fork — including the ones a mid-layer singleton writeback produced);
saturate; **post-saturation detect**; otherwise alive.

The `branch=0` is not a placeholder to improve: the branch id is
per-commitment context the lattice search does not use, and changing it
changes provenance output.

### Task T1a.4.4.3 — Fail-fast saturation

`_saturate_until_dead`: drive the saturator, append each firing, skip
redundant ones (they wrote nothing, so the KB cannot have changed), and
for each derived fact call the incremental `contradicts` — two bit tests
in ein.rs ([design/06](../design/06_saturation.md) §6). Return the prefix
including the killing firing and abandon the iterator there.

Soundness is the append-only argument: a KB inconsistent at firing *n* is
inconsistent at the fixpoint. Keep that sentence in the code.

### Task T1a.4.4.4 — Cores

`smallest_contradiction_frontier(kb, witnesses)` — the
minimum-cardinality source frontier of **one** witness across every
recorded derivation, not the union over all witnesses (`zebra2-bad`:
1 fact, the injected culprit, rather than 38). Lands here because
`try_commitment_set` is its main caller; the search machinery it shares
with `explain` is [S1a.4.6](s1a.4.6_explanation_and_cores.md).

### Task T1a.4.4.5 — Parallel readiness

Make the primitive take `&Arc<KbCore>` rather than `&mut Kb` and return
everything it produces, with **no** root writes inside — the writes
(`emit_nogood`, `(not h)` writeback, stats) belong to the caller's commit
step. This is the seam
[design/08](../design/08_parallelism.md) §2 needs, and building it now
costs nothing.

## Notes

- The `saturator_steps` parameter (a per-call firing cap) exists and is
  `None` in the shipping path; port it, it is used by tests.
- `enable_fail_fast_fork=false` is not dead configuration: a DAG builder
  that merges dead commitments by `state_key` needs the fixpoint, because
  two orientations of a symmetric dead commitment share a fixpoint
  without sharing a fail-fast prefix.
