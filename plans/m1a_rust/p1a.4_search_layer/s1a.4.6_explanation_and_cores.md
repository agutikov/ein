# S1a.4.6 — Explanation and unsat cores

**Phase:** P1a.4 (Search layer)
**Estimate:** 3 days
**Depends on:** [S1a.4.5](s1a.4.5_solve_loop.md),
[S1a.2.4](../p1a.2_kb_core/s1a.2.4_provenance.md)
**Implements:** `ein/inference/{explain,frontier}.py`,
`KnowledgeBase.unsat_core`, [design/07](../design/07_search_layer.md) §7

## Context

Three related searches over the AND/OR justification graph, with three
different questions:

| function | asks |
|---|---|
| `kb.unsat_core(conflicting)` | the **union** of source-frontier terminals reachable from the conflicts |
| `frontier.smallest_contradiction_frontier` | the **minimum-cardinality** frontier of *one* witness, choosing one justification per fact |
| `explain.explain` | a minimal explanation of a fact, under a budget |

S1.21.7 made a fact an OR-node over its recorded derivations, which is
what lets these choose rather than union. It also made the justification
graph genuinely **cyclic** — the symmetric mirror has `(R a b)` and
`(R b a)` justifying each other — which the search handles by taking a
least fixpoint from the sources up, so a fact is never grounded in
itself.

This is the part of the engine where a "cleaner" rewrite most easily
changes the answer. Port it literally.

## Acceptance

- For every corpus contradiction, `unsat_core`,
  `smallest_contradiction_frontier` and `explain` return the **same fact
  sets in the same order** as ein.py.
- `zebra2-bad` reports a 1-fact core (the injected culprit), not 38.
- With `record_alternative_justifications=false`,
  `smallest_contradiction_frontier` falls back to the recorded-primary
  walk and both sides agree on that path too.
- `ExplanationBudget` cuts at the same point and reports the same partial
  result.
- `Explanation.__len__` / iteration order identical (they reach the
  trace).
- `validate_proof_for_explanation` accepts/rejects the same proofs with
  the same messages.

## Tasks

### Task T1a.4.6.1 — The shared walk

`walk_premises(fact, resolve, keep, visited, justifications)` as a BFS
over `FactId` with a `BitSet` visited set. One shared `visited` memoises
across all conflicting facts — that sharing is observable through the
returned core, not just a speedup.

`kb.unsat_core` on top of it: `keep` = provenance is `None` or kind is
`source` / `hypothesis`. Primary-only by default; `all_justifications`
gives the larger soundness envelope, and the default is deliberate
(unioning over alternatives makes the core monotonically *larger*, which
is the opposite of legible).

### Task T1a.4.6.2 — `_Graph` and environments

`_build_graph`: the AND/OR graph rooted at the target, with `rank`
(topological-ish depth) and `key(env) = (len(env), sorted ranks)`. Note
the two `sorted(..., key=repr)` sites — `sorted({*g.just, *g.seed},
key=repr)` and `sorted(set(envs), key=g.key)` — which need
`python_repr` ([design/02](../design/02_determinism_and_order.md) §7).

### Task T1a.4.6.3 — `_propagate`, `_fold`, `_minimise`

The least-fixpoint propagation from sources upward
(`for fid in sorted(wave, key=rank)`), the per-node environment fold
across AND-nodes, and the minimisation that drops dominated
environments. Port the loop structure, the wave ordering, and the
domination test exactly; each of them decides which of several equally
small explanations is returned.

### Task T1a.4.6.4 — `explain` and the fallback

`explain(kb, fact, budget)` with `ExplanationBudget` (node / environment
/ time caps) and `_recorded_fallback` when the budget is exhausted —
including its key `(len(core), " ".join(sorted(repr(f) for f in core)))`.

### Task T1a.4.6.5 — `smallest_contradiction_frontier`

The minimum-cardinality frontier over one witness, choosing one
justification per fact rather than unioning. Used by
`try_commitment_set` on both dead paths and by `_contradiction` for a
root-level clash.

### Task T1a.4.6.6 — `minimal_contradiction_frontier` and the contract

The `explain`-side entry and
`monotonic/contract.validate_proof_for_explanation` (which checks a
`LatticeProof` is usable for explanation — subset relations against
`learned_nogoods`, the `kb_index` walk).

## Notes

- **NAF and cores.** `absent_premises` is recorded on provenance but no
  walk interprets it, and that is intentional — `absent_semantics.md`'s
  corollary C3 says deletion-based core minimisation is unsound under
  NAF, because dropping a fact can flip an absence and fabricate a
  contradiction the full KB never had. Do not "improve" the walks to
  honour negative premises here; that is a semantics change and it needs
  its own decision.
- Diff this stage's output on *every* corpus entry that produces a
  contradiction, not just the zebra ones. The small `examples/branching`
  and `examples/lattice` fixtures are where a tie-break difference is
  visible at a glance.
