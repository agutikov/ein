# S1a.4.6 — Explanation and unsat cores

**Phase:** P1a.4 (Search layer)
**Status:** **shipped** 2026-08-18, **before** S1a.4.4 and S1a.4.5 — see below.
**Estimate:** 3 days
**Depends on:** ~~[S1a.4.5](s1a.4.5_solve_loop.md)~~ (see below),
[S1a.2.4](../p1a.2_kb_core/s1a.2.4_provenance.md)

> **The stated order had a cycle, and this stage is the way out of it.**
> [S1a.4.4](s1a.4.4_commitment_primitive.md)'s T1a.4.4.4 needs
> `smallest_contradiction_frontier` — it is in the `enter` event and in
> `CommitmentSetResult`, so without it a commitment's *observable* is
> incomplete — and that function lives here (T1a.4.6.5). The dependency this
> doc declared on S1a.4.5 is only its **acceptance**: diffing over
> *per-commitment* contradictions needs a search to produce them. So the
> machinery lands first and the acceptance completes in two parts.
>
> What the corpus can already exercise, and does: **root** contradictions
> (2 files), every recorded derivation of every fact, and both budget
> regimes. What waits for S1a.4.5 is `validate_proof_for_explanation`
> (T1a.4.6.6's second half), which needs a `LatticeProof`.
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

The `explain-shape` diff, run in both `record_alternative_justifications`
regimes: **66 files, 178 explained facts, 0 differences** each.

| item | result |
|---|---|
| `unsat_core`, `smallest_contradiction_frontier` and `explain` agree | the `CORE` / `SCF` / `CONTRA` lines. Only **2** corpus files have a root contradiction, which is why the op also explains a deterministic sample of *derived* facts (every 5th by content order, capped at 12) — that is where the label propagation is actually exercised |
| `zebra2-bad` reports a 1-fact core, not 38 | `CORE 39`, `SCF 1 [(color-loc Green House-1)]` — the injected culprit |
| `record_alternative_justifications=false` | the second sweep; it is a different code path, not a smaller one |
| `ExplanationBudget` cuts at the same point and reports the same partial result | every target is explained twice, once under a deliberately tight budget. It cuts on **11** targets and reaches `_recorded_fallback` on **8** |
| `Explanation.__len__` / iteration order | `len` is in every line. The *frontier's* order is **not** compared and cannot be: ein.py's is a `frozenset`, not reproducible even run to run, so both sides sort — which is what every display site does |
| `validate_proof_for_explanation` | **moves to [S1a.4.5](s1a.4.5_solve_loop.md)** — it takes a `LatticeProof` |

Mutation-checked: ranking by `FactId` instead of `repr` order moves 7
files, dropping `_minimise`'s superset test moves 3, and leaving the
propagation wave unsorted moves 4.

### The one tie-break the corpus cannot separate

`_recorded_fallback`'s key is `(len(core), " ".join(sorted(repr(f))))`,
and its second half only decides when two targets tie on core *size*.
`zebra2-bad` has four size-1 cores — and the `repr`-smallest of them is
also the first the detector found, so plain first-wins gives the same
answer and the key's second half is invisible. Reaching it needed the
instrument to ask for it: it calls the fallback once on the **reversed**
witness list, which separates the two orders. Dropping the key half then
moves 1 file, where before it moved none.

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
