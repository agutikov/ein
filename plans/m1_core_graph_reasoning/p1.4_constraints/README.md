# P1.4 — Contradiction detection

**Estimate:** ~1-2 days (down from the original ~1 week —
see [S1.4.0 review](s1.4.0_review.md)).
**Depends on:** P1.2 (graph + provenance), P1.3 (saturator —
shipped 2026-05-21).
**Blocks:** P1.5 (hypothesis loop calls the detector to decide
branch retraction).

## Goal

Implement a small, focused **contradiction detector**: scan the
KB for `(X, (not X))` pairs and return them as `Contradiction`
records the hypothesis loop (P1.5) and the trace renderer (P1.6)
consume.

The phase was substantially **rescoped 2026-05-21** by the
[S1.4.0 review](s1.4.0_review.md) against the shipped P1.3:

- **Spatial constraints** (S1.4.2, ~3-4 days) → **dropped**.
  Q17 (resolved 2026-05-18) chose the declarative graph-only
  formulation; the `right-of` / `next-to` relations + the
  `square-fwd` / `square-bwd` / `square-unique` rules handle the
  Zebra spatial cases. Disjunctive non-adjacent cases ("Ivory left
  of Green") migrate to P1.5 hypothesis branching.

- **Structural constraints** (S1.4.1's original scope) → **largely
  obsolete**. `type-exclusivity` (T2 on `co-located`) produces the
  `(not (co-located A B))` facts the original `single-attribute` /
  `all-different` constraints would have asserted. The detector is
  what consumes those facts when they conflict with positive
  assertions.

- **`Constraint` entity / `verify_incremental` API / 1-D position
  lattice** → all dropped. The detector is a pure scan over the
  existing fact set; no new entity types or indexes needed.

The acceptance criterion shrinks to: **a clean detector + P1.5 can
call it after each saturation cycle to decide whether to retract
the current branch**.

## Stages

| ID      | Title                                                          | Duration |
|---------|----------------------------------------------------------------|----------|
| S1.4.0  | Review of P1.4 against shipped P1.3 + resolution plan          | meta — closed 2026-05-21 |
| S1.4.1  | Contradiction detector                                         | 1-2 days |

S1.4.2 (spatial constraints) was deleted by the S1.4.0 review;
see [§D of S1.4.0](s1.4.0_review.md#d-where-the-spatial-cases-now-live)
for where each of its use cases now lives.

## Acceptance

- `ContradictionDetector(kb).detect()` returns a tuple of
  `Contradiction(positive_fact, negative_fact, layer, branch)`
  records, one per `(X, (not X))` pair in the same layer.
- An empty tuple is the no-conflict verdict; a non-empty tuple is
  consumed by P1.5.
- Detection is O(|not-layer facts|) — fast for Zebra-scale
  (~120 negative facts after saturation).
- `pytest tests/inference/test_contradiction.py` covers:
  - empty KB → empty result;
  - KB with a positive only → empty result;
  - KB with a `(not X)` only (no matching positive) → empty;
  - KB with `(X, (not X))` → one Contradiction record;
  - same shape across layers (FACT-layer X + REASONING-layer not-X
    → still a Contradiction, layer reported correctly);
  - the corner case where `X` itself is a nested-Fact arg of some
    other fact (Q40) — detector handles it without recursing into
    the nesting.

## Connections

- [S1.4.0 review](s1.4.0_review.md) — the audit that produced this
  shrunk scope.
- [`plans/ideas.md`](../../ideas.md) → "P1.4 constraints — collapse"
  (promoted / pruned).
- [P1.5 hypothesis loop](../p1.5_hypothesis_loop/) — calls
  `detector.detect()` at branch boundaries; emits the synthetic
  `(contradiction-under ?h)` fact that triggers the
  `hypothesis-contradiction` rule shipped in P1.3.
- [Q17 — spatial-relation formalisation](../open_questions.md#q17--spatial-relation-formalisation)
  — the resolution that obsoleted the original S1.4.2.
- [Idea 03 — three task classes](../../../docs/ideas/03-three-task-classes.md) —
  the "contradictions" task class consumes the detector's output
  directly.
- [Idea 05 §Open question](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#open-question-recorded-in-the-readme)
  — the PoC's spatial open question, resolved by Q17 + P1.3's
  square rules (not by P1.4 any more).
