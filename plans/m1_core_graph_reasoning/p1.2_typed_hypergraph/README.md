# P1.2 — Typed-hypergraph core

**Estimate:** 2 weeks.
**Depends on:** P1.1 (IR locked).
**Blocks:** P1.3, P1.4, P1.5, P1.6.

## Goal

Implement the graph the PoC's `State` was a half-hearted draft of:
a **typed hypergraph**, organised into three layers (ontology /
fact / reasoning), with **per-edge provenance** so a contradiction
can name which premises clashed.

Per [`docs/ideas/02-graph-as-formal-substrate.md`](../../../docs/ideas/02-graph-as-formal-substrate.md)
the graph is *the* working memory, not a discarded intermediate.
Per [`docs/ideas/05-zebra-puzzle-graph-reasoner.md`](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#the-three-implicit-layers)
the three layers are made explicit so they can be queried separately.
Per [`docs/ideas/03-three-task-classes.md`](../../../docs/ideas/03-three-task-classes.md)
the provenance edges are non-optional — they're what makes the
*contradictions* task class human-readable.

## Stages

| ID      | Title                            | Duration |
|---------|----------------------------------|----------|
| S1.2.1  | Data model (typed hypergraph)    | 4-5 days |
| S1.2.2  | Three layers as views            | 2-3 days |
| S1.2.3  | Provenance edges + DAG queries   | 3-4 days |

## Acceptance

- `from ein_bot.graph import Graph`, with mutation API
  `add_type / add_instance / add_edge / add_hedge`, query API
  `nodes(type=…) / edges(rel=…) / hedges(rel=…)`, and
  `provenance(edge_id) -> DerivationDAG`.
- Three views: `g.ontology()`, `g.facts()`, `g.reasoning()` —
  each a thin filter over the same store.
- Loading `examples/zebra.ein` populates the graph; counts match
  the PoC fixture (60 objects, 100 edges).
- DOT export of each view returns a syntactically-valid Graphviz
  digraph.

## Connections

- [Idea 02](../../../docs/ideas/02-graph-as-formal-substrate.md) —
  why the graph is primary, not the solver.
- [Idea 05](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#the-three-implicit-layers) —
  the three layers worth naming explicitly.
- [Idea 06 row 2](../../../docs/ideas/06-inference-rules-completeness.md) —
  equality-class hooks reserved (e-graph promotion later).
- [Idea 03](../../../docs/ideas/03-three-task-classes.md) — provenance is
  the substrate for the contradictions task class.
