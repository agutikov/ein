# S1a.4.4 — The commitment primitive

**Phase:** P1a.4 (Search layer)
**Status:** **shipped** 2026-08-18 — acceptance below.
**Estimate:** 2 days
**Depends on:** [S1a.4.3](s1a.4.3_apriori_and_nogoods.md),
[S1a.4.6](s1a.4.6_explanation_and_cores.md) (see T1a.4.4.4),
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

The `commit-shape` diff enters the layer-1 singletons and the first six
layer-2 sets of a real alive frontier — **65 files, 477 enterings, 0
differences** — in both fail-fast regimes.

| item | result |
|---|---|
| `kind`, firing count, unsat core identical for every commitment | the `ENTER` line, plus the fork's fact count and the hypothesis writes' *provenance*. Skipping the pre-saturation detect moves 1 file; returning the union core instead of the smallest frontier moves 9 |
| `enable_fail_fast_fork=false` exercised too | the second sweep. It is genuinely a second check: forcing the slow path in the *fast* regime moves **11 files** and nothing in the slow one |
| `hypothesis_facts` are the writes for `C` only | the `hyps=` column, against `facts=` for the fork's total |
| Two calls independent, sharing no mutable state | `REPEAT` compares the whole result of a re-entering, and `two_enterings_share_no_mutable_state` writes into the first fork *between* the two calls and checks the second is untouched — because `REPEAT` alone cannot tell isolation from coincidence |

`branch=0` is in the diff. T1a.4.4.2 says changing it changes provenance
output, and that was true only once the instrument printed provenance:
before it did, `branch=1` moved nothing; after, it moves **47 files**.

### The dependency the stage table did not have

T1a.4.4.4 needs `smallest_contradiction_frontier`, which is
[S1a.4.6](s1a.4.6_explanation_and_cores.md)'s T1a.4.6.5 — and S1a.4.6
declared a dependency on S1a.4.5, which depends on this stage. The cycle
is only in the *acceptances*: S1a.4.6's machinery has no such dependency,
so it shipped first. See its header.

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

> **The "no root writes" half landed; the `&Arc<KbCore>` half did not,
> and cannot yet.** `Kb::fork` takes `&mut self` on purpose — sealing the
> parent's top layer is what makes the two histories diverge
> ([design/03](../design/03_data_model.md) §5) — and the primitive also
> needs `&mut Terms`, because interning a hypothesis and building a
> conclusion both write the global tables. Turning either into a shared
> handle is a change to the *data model*, not to this function, and it is
> what [P1a.7](../p1a.7_parallelism/README.md) is for. What is true now
> is the part that mattered: nothing inside writes root, nothing inside
> emits a no-good or touches a counter, and
> `two_enterings_share_no_mutable_state` checks it by mutating one fork
> between two calls.

## Notes

- The `saturator_steps` parameter (a per-call firing cap) exists and is
  `None` in the shipping path; port it, it is used by tests.
- `enable_fail_fast_fork=false` is not dead configuration: a DAG builder
  that merges dead commitments by `state_key` needs the fixpoint, because
  two orientations of a symmetric dead commitment share a fixpoint
  without sharing a fail-fast prefix.
