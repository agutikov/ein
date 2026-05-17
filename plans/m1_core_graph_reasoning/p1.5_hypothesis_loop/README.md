# P1.5 — Hypothesis loop + ATMS-style branching

**Estimate:** ~2 weeks.
**Depends on:** P1.2 (`g.fork()`), P1.3 (saturator),
P1.4 (`verify`).
**Blocks:** P1.7 (Zebra integration).

## Goal

Implement the *outer* loop of the reasoner — the part that branches
when propagation stalls, tests each branch by re-saturating, and
either accepts a unique survivor or recurses (multilevel
hypothesis). This is the PoC's
[`algorithm sketch`](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#the-algorithm-as-printed-in-the-readme),
done right: lazy branching, backjump on contradiction, and a search
tree rendered as a first-class artefact.

Per [`docs/ideas/03-three-task-classes.md`](../../../docs/ideas/03-three-task-classes.md),
the same loop services all three task classes (solve / gaps /
contradictions) — the difference is what the loop *records* and
what it returns at quiescence.

## Stages

| ID      | Title                              | Duration |
|---------|------------------------------------|----------|
| S1.5.1  | Saturate-then-branch driver       | 4-5 days |
| S1.5.2  | Multilevel branching + search tree | 3-4 days |
| S1.5.3  | Symmetry detection placeholder    | 2-3 days |

## Acceptance

- `solve(g, registry)` returns `Solution | Ambiguity(branches) | Contradiction(unsat-core)`.
- The Zebra puzzle solves uniquely.
- Zebra-minus-one-condition produces an `Ambiguity` with at least
  two distinct branches.
- Search tree exposed as a tree-shaped IR + DOT diagram.
- Symmetry stub records `(symmetry-class …)` notes; trace planner
  can read them.

## Connections

- [Idea 05 §Hypothesis mechanism](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#hypothesis-mechanism).
- [Idea 06 row 5](../../../docs/ideas/06-inference-rules-completeness.md) —
  hypothesis-and-contradiction as a *rule*; we treat it specially.
- [Idea 03 §The implicit fourth class](../../../docs/ideas/03-three-task-classes.md) —
  *explanation* falls out of the hypothesis machinery + provenance.
- ATMS prior art:
  [docs/index/09-cognitive-architectures-neurosymbolic.md](../../../docs/index/09-cognitive-architectures-neurosymbolic.md).
