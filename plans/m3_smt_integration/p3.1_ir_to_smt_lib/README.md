# P3.1 — IR → SMT-LIB translation

**Estimate:** 1-2 weeks.
**Depends on:** M1 closed.
**Blocks:** P3.3-P3.5.

## Goal

Translate the M1 IR into SMT-LIB 2 such that an off-the-shelf solver
(Z3 first) can `(check-sat)` it. The translator is *structural* —
each IR form maps to a small SMT-LIB template; no clever
re-encoding.

## Stages

| ID      | Title                              | Duration |
|---------|------------------------------------|----------|
| S3.1.1  | Backend interface + Z3 binding     | 3-4 days |
| S3.1.2  | Translator: ontology + facts → smt2 | 4-5 days |

## Acceptance

- `python -m ein.smt.translate examples/zebra.ein` emits a
  syntactically-valid `.smt2` file accepted by `z3 -smt2`.
- `z3 -smt2 zebra.smt2` returns `sat` with a model assigning every
  goal variable.
- `ein solve zebra.ein --backend=smt:z3` reports the canonical
  Zebra answer (Japanese keeps zebra; Norwegian drinks water).

## Connections

- [Idea 04 §Sketch of the pipeline](../../ideas/04-nlp-to-graph-to-solver-pipeline.md#sketch-of-the-pipeline) —
  the solver back-end branch.
- [Idea 02](../../ideas/02-graph-as-formal-substrate.md) —
  solver as accelerator, not authority.
