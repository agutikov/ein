# S1a.4.3 — Apriori candidate generation and the no-good store

**Phase:** P1a.4 (Search layer)
**Estimate:** 3 days
**Depends on:** [S1a.4.1](s1a.4.1_hypothesis_generation.md)
**Implements:** `ein/inference/{apriori,nogoods}.py`,
[design/07](../design/07_search_layer.md) §4

## Context

The lattice's arithmetic: layer *k+1* candidates from the layer-*k*
frontier by the textbook Apriori prefix-join, filtered against the alive
set and the learned no-good clauses (the downward-closure prune). Pure
set arithmetic — no KB inspection, no saturation — which makes it the
easiest part of the search layer to port and the easiest to get subtly
wrong in *ordering*.

With `FactId`s this becomes integer work, and the clause store gets a
bitmask fast path that matters exactly in the regime where it is hot:
with `enable_singleton_writeback` off, exhaustive zebra2 explodes from
101 to 3 336+ enterings and the clause set grows with it.

## Acceptance

- Candidate lists identical, in order, at every layer of every corpus
  entry (compared through the `enter` event sequence).
- `nogoods_emitted` / `nogoods_subsumed` identical.
- The clause store is subsumption-minimal after every emission, matching
  ein.py's set exactly (compared as a set of sorted `FactId` tuples,
  translated back to fact s-expressions for the diff).
- `order_candidates` in both modes: `lex` and `score-sum` (with
  `hypgen_scoring="popularity"`, where the score actually differentiates)
  produce identical orders.
- A stress fixture with `enable_singleton_writeback=false` and a
  `--max-enterings` budget reproduces the same candidate stream up to the
  cut.

## Tasks

### Task T1a.4.3.1 — `CanonicalSetId`

A sorted, deduped `SmallVec<[FactId; 4]>`. `canonicalise` =
sort + dedup. **Ordering:** ein.py sorts `(relation, args)` tuples, i.e.
by *content*; ein.rs must use the semantic comparator, not `FactId`
order ([design/02](../design/02_determinism_and_order.md) §3b,
[design/08](../design/08_parallelism.md) §1). This is the single most
likely place for an interner-order leak.

### Task T1a.4.3.2 — Prefix join

`apriori_prefix_join`: sort `a_prev`; for each `s` at index `i`, take
`prefix = s[:-1]` and scan `sorted_prev[i+1:]`, **breaking** on the first
`t` whose prefix differs, emitting `(*prefix, s[-1], t[-1])` when
`s[-1] < t[-1]`. The `break` is load-bearing for both cost and order.

### Task T1a.4.3.3 — `filter_candidate`

Drop if any element is not in `alive` (a bitset test), or if any clause
is a subset of the candidate. The "every (k−1)-subset ∈ a_prev" check is
covered by construction and is deliberately *not* re-verified.

### Task T1a.4.3.4 — The clause store

Two representations behind one interface:

- **≤ 64 alive** (zebra2's regime): an alive-index bitmask per clause;
  subset test is `clause & cand == clause`. The index is rebuilt when
  `alive` changes, i.e. once per layer.
- **> 64 alive**: sorted `Box<[FactId]>` with a merge-intersection.

Both must produce identical emitted/subsumed counts; add a property test
that runs a randomised clause workload through both and compares.

### Task T1a.4.3.5 — `emit_nogood`

Subsumption on emit: a new clause removes every stored strict superset
and is itself dropped if a stored clause subsumes it. `min_size=1` so
layer-1 singleton deaths land (Q1.5b.5.c). Gated by
`enable_path_nogoods`; when off, `_nogoods` stays empty and
`filter_candidate`'s clause check is a no-op — verify that path too,
since it changes entering counts.

### Task T1a.4.3.6 — `layer_1` and `order_candidates`

`layer_1(alive)` = every singleton, sorted. `order_candidates`:
`"lex"` → sorted; `"score-sum"` → `sorted(key=(-score_sum, canonical))`,
**stable**, where the per-element score comes from
`hypgen.score_hypothesis` and an element not yet in the KB is scored
through a synthetic fact (relation + args only — provenance is
immaterial to the scorer). Unknown mode → the same `ValueError` text.

## Notes

- `filter_candidate` runs at *layer generation* time, so no-goods emitted
  mid-layer do not prune within that layer. That asymmetry is what makes
  [design/08](../design/08_parallelism.md) §2's case 1 free, and it is
  worth a comment in both implementations.
- Do not add a "smarter" candidate order here.
  [F9](../../followups/f9_e_catalog.md) measured that whole cluster inert
  against a complete cardinality-BFS; the arithmetic does not change
  because the language did.
