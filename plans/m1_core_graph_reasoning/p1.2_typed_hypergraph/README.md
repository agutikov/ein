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

## Load-bearing decisions

These are stated up front so downstream stages and reviewers know
what the data model commits to (rationale in S1.2.1):

1. **The graph is the data model — entity API is derived.** Objects,
   types, AND relations are all nodes; non-binary facts are Levi-
   bipartite hyperedge nodes. The compact rendering
   `(Red)-is-a->(Color)` is one *view*; canonical is `(Red)<-1-(is-a)-2->(Color)`.
   Cross-references like `Instance.type` are cached lookups, not a
   competing schema.
2. **Graph is static; rules + inference are the engine.** Reasoning
   happens by rule firings (P1.3) over the KB, not by mutating the
   data model.
3. **Rules can be higher-order.** Parameters can range over
   relations (`(rule symmetric (?rel) :match (?rel ?a ?b))`) — `?rel`
   binds to a Relation node. M1 covers first-order *and* the
   relation-parameter form (already in zebra.ein); deeper
   higher-order forms (rules over rules, predicate over predicate)
   ride [F4 Q36](../../followups/f4_cross_cutting.md#relation-inheritance--rule-polymorphism-q36).
4. **Types and relations are first-class node kinds** — not derived
   labels on facts.
5. **Rules are graph rewrites** — `:match` / `:assert` are typed
   `Pattern` objects (structural only here; matching lives in P1.3).
6. **No syntactic typed-vars (`?a:T`)** — variables are typed by
   premises in `:match` (e.g. `(is-a ?a T)`). See
   [F4 Q35](../../followups/f4_cross_cutting.md#variable-typing-via-match-is-a-var-type-q35).
7. **Compound / virtual node kinds — open class.** M1 ships the base
   set (Type, Instance, Relation, Rule, Fact). Future higher-order
   rules may need sets of relations, projections over argument-slot N,
   top/bottom of a relation subgraph, criterion-selected groups.
   Architecture must accept them without rework — see
   [M1 Q26](../open_questions.md#q26--compound--virtual-node-kinds-for-higher-order-rules).
8. **Ontology IR sub-head split** is a *syntax* question parked at
   [M1 Q22](../open_questions.md). The data model is robust to either
   form (loader normalises).
9. **What has types** — only `Instance` (one type each). Vars and
   relations are typed *structurally* (via patterns, via signature)
   not as a slot on the entity.

## Connections

- [Idea 02](../../../docs/ideas/02-graph-as-formal-substrate.md) —
  why the graph is primary, not the solver.
- [Idea 05](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#the-three-implicit-layers) —
  the three layers worth naming explicitly.
- [Idea 06 row 2](../../../docs/ideas/06-inference-rules-completeness.md) —
  equality-class hooks reserved (e-graph promotion later).
- [Idea 03](../../../docs/ideas/03-three-task-classes.md) — provenance is
  the substrate for the contradictions task class.
- [F4 Q34 / Q35 / Q36](../../followups/f4_cross_cutting.md) —
  algebraic-property cartesian product, typed vars, rule polymorphism;
  the data model is the seam those followups extend.
