# P1.2 — Entity data model + three layers + provenance

**Estimate:** 2 weeks.
**Depends on:** P1.1 (IR locked).
**Blocks:** P1.3, P1.4, P1.5, P1.6.

## Goal

Build the **entity-typed knowledge model** the engine actually
reasons over: `Type`, `Instance`, `Relation`, `Rule`, `Fact`, with
**navigable cross-references** between them (the rules that apply
to a relation, the types a rule quantifies over, the facts about an
instance, …). Layer the entities across three knowledge populations
(ontology / fact / reasoning), and attach **per-fact provenance** so
a contradiction can name which premises clashed.

The PoC's `State` was a pair of dicts with no notion of Rule or
Relation. The S1.2.1 entity model is the public API; a hyperedge
store sits underneath only as far as facts have ≥ 2 args. The "graph"
framing from `docs/ideas/02` describes the *visualisation* of the
entity model, not its in-memory shape — see `docs/ir.md` §6.

Per [`docs/ideas/02-graph-as-formal-substrate.md`](../../../docs/ideas/02-graph-as-formal-substrate.md)
the knowledge base is *the* working memory, not a discarded
intermediate. Per [`docs/ideas/05-zebra-puzzle-graph-reasoner.md`](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#the-three-implicit-layers)
the three layers are made explicit so they can be queried
separately. Per [`docs/ideas/03-three-task-classes.md`](../../../docs/ideas/03-three-task-classes.md)
the provenance records are non-optional — they're what makes the
*contradictions* task class human-readable.

## Stages

| ID      | Title                                              | Duration |
|---------|----------------------------------------------------|----------|
| S1.2.1  | Data model — Rule · Relation · Type · Instance · Fact + cross-refs | 4-5 days |
| S1.2.2  | Three layers as views                              | 2-3 days |
| S1.2.3  | Per-fact provenance + derivation-DAG queries       | 3-4 days |
| S1.2.4  | Unified graph rendering of the KB                  | 3-4 days |

## Acceptance

- `from ein_bot.kb import KnowledgeBase`, populated by
  `KnowledgeBase.from_ir(parse_file("examples/zebra.ein"))`.
- Entity API: `kb.types`, `kb.instances`, `kb.relations`, `kb.rules`,
  `kb.facts`. Each entity exposes its cross-references as attributes
  (e.g. `relation.rules`, `rule.types`, `instance.facts`).
- Three views: `kb.ontology()`, `kb.facts()`, `kb.reasoning()` —
  each a thin filter over the same registry; `kb.fork()` only
  copies the REASONING slice.
- `kb.derivation_dag(fact)` returns the recoverable provenance DAG;
  `kb.unsat_core(facts)` returns the minimal source-fact frontier.
- DOT export of each view returns syntactically-valid Graphviz —
  per-form forward by `to_dot` (S1.1.4), the PoC-style **unified
  view** with fused entity identity by `kb.to_dot()` (S1.2.4),
  reverse by `from_dot` (this phase).

## Connections

- [Idea 02](../../../docs/ideas/02-graph-as-formal-substrate.md) —
  why the graph is primary, not the solver.
- [Idea 05](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#the-three-implicit-layers) —
  the three layers worth naming explicitly.
- [Idea 06 row 2](../../../docs/ideas/06-inference-rules-completeness.md) —
  equality-class hooks reserved (e-graph promotion later).
- [Idea 03](../../../docs/ideas/03-three-task-classes.md) — provenance is
  the substrate for the contradictions task class.
