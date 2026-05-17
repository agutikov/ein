# P1.4 — Constraints: structural + spatial

**Estimate:** ~1 week.
**Depends on:** P1.2 (graph), P1.3 (rules, since some constraints
fire derived edges).
**Blocks:** P1.5 (hypothesis testing calls `verify`).

## Goal

Implement the constraint engine the PoC sketched. Two layers:

1. **Structural constraints** — the PoC's
   "single-type" and "number-of-attributes" constraints,
   formalised cleanly per
   [`docs/ideas/05-zebra-puzzle-graph-reasoner.md`](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#constraints-as-the-readme-puts-it).
2. **Spatial constraints** — close the PoC's
   [open question](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#open-question-recorded-in-the-readme)
   ("how do we define `Ivory house to the left of Green one`?")
   using a 1-D position lattice.

A *constraint* is a predicate the engine can call mid-saturation
and mid-hypothesis to declare a state *infeasible*. Constraints
have provenance (they reference the source IR form), so a violation
gives a useful contradiction.

## Stages

| ID      | Title                              | Duration |
|---------|------------------------------------|----------|
| S1.4.1  | Structural constraints             | 2-3 days |
| S1.4.2  | Spatial constraints (1-D lattice)  | 3-4 days |

## Acceptance

- `verify(graph) -> Verdict` returns `OK | Violation(constraint_id, witness_edges)`.
- All four PoC structural constraints exercised.
- Spatial constraints solve the "Ivory left of Green" case
  without sneaking it in as a one-off rule.
- Pytest suite covers each constraint kind + a contradiction
  recovery test.

## Connections

- [Idea 05 §Constraints](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#constraints-as-the-readme-puts-it).
- [Idea 06 row 8](../../../docs/ideas/06-inference-rules-completeness.md) —
  Allen-style spatial reasoning; we deliberately ship the simpler
  1-D lattice (see [M1 Q17](../../open_questions.md#q17--spatial-relation-formalisation)).
