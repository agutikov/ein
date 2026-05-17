# P3.4 — Explanation recovery: SMT model → IR trace

**Estimate:** 3-4 days.
**Depends on:** P3.3.
**Blocks:** P3.5.

## Goal

Per [`idea 02`](../../../docs/ideas/02-graph-as-formal-substrate.md),
*"the philosophical difference is who owns the proof"* — even when
SMT did the heavy lifting, the *trace* lives on the IR. This phase
lifts SMT outputs back into IR-shaped derivations the M1 trace
renderer can consume.

## Stages

| ID      | Title                              | Duration |
|---------|------------------------------------|----------|
| S3.4.1  | Lift model + unsat core to IR     | 3-4 days |

## Acceptance

- An SMT solve produces the same trace artefact shape as a graph
  solve (same `(trace …)` IR form).
- The trace marks SMT-derived facts as
  `Provenance(kind='smt', backend=..., smt2-fragment=...)`.
- Unsat core IR-edges link back to source facts via the M1
  derivation DAG.

## Connections

- [Idea 02 §Where it lands](../../../docs/ideas/02-graph-as-formal-substrate.md#where-it-lands-compared-to-graph--ir-for-solver) —
  "the philosophical difference is who owns the proof".
- [Idea 03](../../../docs/ideas/03-three-task-classes.md) — the trace
  is what makes the contradictions class useful.
