# P3.2 — Theory + sort selection

**Estimate:** 3-4 days.
**Depends on:** P3.1.
**Blocks:** P3.3.

## Goal

The user's open question: *"How to select theory? How to select
types mapping?"* This phase is the *decision policy* for the
translator — given an IR ontology, pick the SMT theories and sort
encodings to use.

## Stages

| ID      | Title                              | Duration |
|---------|------------------------------------|----------|
| S3.2.1  | Sort policy + ontology audit       | 3-4 days |

## Acceptance

- A `docs/decisions/M3-sort-policy.md` documenting the rules.
- The translator's `pick_sort(ir_type)` is implemented + tested.
- Audit script: for each IR file in `examples/`, print the sort
  decisions made. Catches surprises.

## Connections

- [Idea 04 §Open questions](../../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).
- [Idea 02](../../ideas/02-graph-as-formal-substrate.md) —
  sort choices affect *whether* SMT is faster than the graph engine.
