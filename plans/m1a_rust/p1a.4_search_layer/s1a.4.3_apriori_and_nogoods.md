# S1a.4.3 — Apriori candidate generation and the no-good store

**Phase:** P1a.4 (Search layer)
**Status:** **shipped** 2026-08-18 — acceptance below, with two items that
[S1a.4.5](s1a.4.5_solve_loop.md) is the first stage able to run.
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

The `lattice-shape` diff runs the whole arithmetic over a **real** alive
set — the open hypotheses of a saturated root, capped at 12 by content
order so the layers stay bounded — and compares layers 1–3, both
ordering modes, every `nogood` event and the resulting store.
**65 files, 2 043 clause emissions, 0 differences**, plus one accepted
divergence below.

| item | result |
|---|---|
| Candidate lists identical, in order, at every layer | `LAYER1` / `LAYER2` / `LAYER3` compared as text; the layer-*k* order is what `layer_1`'s comparator decides, and an id-ordered `layer_1` moves **33 of 65** files |
| The clause store subsumption-minimal after every emission | the `STORE` block: a sorted list of sorted clauses. Dropping the superset removal moves **43 files**; taking `min_size` as a constant 2 moves **47** |
| `order_candidates` in both modes | both are in the text, and `score_sum_orders_differently_from_lex_somewhere_in_the_corpus` asserts the two are not the same check — under `most-constrained` every score is `0.0` and `score-sum` *is* `lex`, which is the path 53 corpus files (no `(config …)` block) take. It differentiates on **7** |
| `nogoods_emitted` / `nogoods_subsumed` identical | **moves to [S1a.4.5](s1a.4.5_solve_loop.md)** — they are `MonotonicStats` fields. What is pinned here is every `nogood` *event*, which is what they count |
| The `enable_singleton_writeback=false` stress fixture | **moves to [S1a.4.5](s1a.4.5_solve_loop.md)** — it needs a solve to have a candidate stream to cut |

### D2 — where the two implementations must differ

[Q-M1a.4](../open_questions.md#q-m1a4--sorted-over-mixed-type-fact-args)
was marked *blocking P1a.4* and this is the stage that reaches it:
`layer_1`'s `sorted(alive)` is the one comparison in the engine ein.py
cannot always make. Exactly one corpus file diverges, exactly the
predicted one, and the port answers
`[{(seat Ann 1)}, {(seat Ann left)}]` where ein.py raises `TypeError`.
Recorded as [D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)
and **asserted** by the sweep — a file on the ledger that stopped
diverging fails as loudly as one that started.

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
`s[-1] < t[-1]`.

> **"The `break` is load-bearing for both cost and order" — only cost.**
> Every set in a layer has the same size, so sorting by the full tuple
> sorts by the prefix as its primary key, and once a prefix differs no
> later entry can match it again. Replacing the `break` with a
> `continue` is byte-identical on all 65 files, which is how this was
> settled rather than by re-reading the loop. It stays because ein.py
> has it and because the scan is quadratic without it.

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

> **Only the sorted representation landed, deliberately.** The bitmask's
> stated payoff is the `enable_singleton_writeback=false` regime, where
> exhaustive `zebra2` grows from 101 enterings to 3 336+ — and nothing
> can run an exhaustive solve until
> [S1a.4.5](s1a.4.5_solve_loop.md). Landing a second representation
> before the measurement that justifies it is the mistake Win B already
> made once ([Q-M1a.17](../open_questions.md)), so the trigger condition
> goes to [P1a.6](../p1a.6_performance/README.md) with the numbers
> attached instead. The interface is one function
> (`apriori::is_subset`), so the second representation is a swap when it
> is earned.

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
