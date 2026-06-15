# M3 — SMT integration

**Estimate:** ~1 month (~4 weeks).
**Status:** planned.
**Depends on:** [M1](../m1_core_graph_reasoning/README.md) (IR + engine).
**Blocks:** none.

## Goal

Per [`docs/ideas/02-graph-as-formal-substrate.md`](../../docs/ideas/02-graph-as-formal-substrate.md),
the graph engine is *primary*; solvers are *optional accelerators
for slices the graph engine declares hard*. This milestone wires the
slice handoff:

```
IR → graph engine (M1) → "this sub-problem is solver-shaped" → SMT-LIB → solver → IR model
                                                                                ▼
                                                                graph engine accepts the model
                                                                  → keeps proof on the IR
```

Three task classes per
[`docs/ideas/03-three-task-classes.md`](../../docs/ideas/03-three-task-classes.md)
all get solver back-ends:

- **A. Solve** — `(check-sat) + model extraction`.
- **B. Gaps** — model enumeration / backbone analysis.
- **C. Contradictions** — unsat core / MUS + provenance lift.

## Phases

| ID    | Title                              | Duration | Folder |
|-------|------------------------------------|----------|--------|
| P3.1  | IR → SMT-LIB translation           | 1-2 wk   | [`p3.1_ir_to_smt_lib/`](p3.1_ir_to_smt_lib/) |
| P3.2  | Theory + sort selection            | 3-4 days | [`p3.2_theory_and_sort_selection/`](p3.2_theory_and_sort_selection/) |
| P3.3  | Three task classes on SMT          | 1 wk     | [`p3.3_task_classes_on_smt/`](p3.3_task_classes_on_smt/) |
| P3.4  | Explanation recovery               | 3-4 days | [`p3.4_explanation_recovery/`](p3.4_explanation_recovery/) |
| P3.5  | Hybrid orchestration               | 3-4 days | [`p3.5_hybrid_orchestration/`](p3.5_hybrid_orchestration/) |

## Acceptance

- `ein solve zebra.ein --backend=smt:z3` returns the canonical
  answer.
- `--mode=gaps` returns the expected diverging variables.
- `--mode=contradictions` returns a minimal unsat core whose
  source-edges match the M1 engine's pure-graph contradiction.
- `--backend=hybrid` (default) drives the graph engine and only
  hands sub-problems to SMT when the engine flags `(hard-slice …)`
  on a subgraph.
- The trace records every solver invocation with its smt2 text +
  the model/unsat-core lifted back into IR.

## Open questions

See [`open_questions.md`](open_questions.md) for M3-scoped.

## Connections

- [Idea 02](../../docs/ideas/02-graph-as-formal-substrate.md) —
  solver-as-accelerator philosophy.
- [Idea 03](../../docs/ideas/03-three-task-classes.md) — the table
  in §Why distinguishing these matters drives P3.3.
- [Idea 04](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md) —
  IR → solver as one branch of the pipeline.
- The existing `smt/CVC4` submodule is a candidate backend; Z3 is
  primary.
