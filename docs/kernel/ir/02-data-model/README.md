# Data model — Python in-memory representation

How the graph from [`../01-ein-graph/`](../01-ein-graph/) is held in
memory: frozen dataclasses for the five entity kinds, the
`KnowledgeBase` store that owns them, the reverse indexes that make
cross-references O(1), the layer views, the hypothesis-fork mechanic,
and the provenance + derivation-DAG machinery.

## Files

- [`01_entities.md`](01_entities.md) — the five entity dataclasses
  (`Type`, `Instance`, `Relation`, `Rule`, `Fact`) plus `Pattern`,
  `Provenance`, and `Layer`. Identity rules. Cross-reference
  accessors. Pattern's structural-only view of `:match` / `:assert`.
- [`02_store.md`](02_store.md) — `KnowledgeBase` registries + reverse
  indexes; the IR loader; layer views (`FactView`); `kb.fork()` for
  hypothesis branches; encoding-agnostic `logical_types` /
  `logical_instances`; `derivation_dag` / `unsat_core`; equality
  classes placeholder.

## Reading order

Read `01_entities.md` first to understand the node-kind classes and
how identity works (name vs `(rel, args)`, what's excluded from
equality). Then `02_store.md` for how they're aggregated, indexed,
and viewed.

## Where this maps to code

- `src/ein_bot/kb/entities.py` — Type/Instance/Relation/Rule/Fact/Layer.
- `src/ein_bot/kb/pattern.py` — Pattern.
- `src/ein_bot/kb/provenance.py` — Provenance, DerivationDAG.
- `src/ein_bot/kb/store.py` — KnowledgeBase, EqClasses, Query.
- `src/ein_bot/kb/views.py` — FactView, logical_types,
  logical_instances.
- `src/ein_bot/kb/from_ir.py` — IR → KB loader.

## Stability

Stable through M1. F4 promotion seams (compound node kinds, e-graph)
are noted explicitly in `01_entities.md` / `02_store.md`; M1 doesn't
implement them but the architecture stays open.

The IR encoding choice (classic vs unified is-a) is **deferred to
P1.7** — the data model handles both transparently; downstream code
uses `logical_types` / `logical_instances` to stay encoding-agnostic.
