# Data model — Python in-memory representation

How the graph from [`../01-ein-graph/`](../01-ein-graph/) is held in
memory: frozen dataclasses for the entity kinds, the
`KnowledgeBase` store that owns them, the reverse indexes that make
cross-references O(1), the layer views, the hypothesis-fork mechanic,
and the provenance + derivation-DAG machinery.

## Files

- [`01_entities.md`](01_entities.md) — the entity dataclasses
  (`Relation`, `Rule`, `Fact` + the `NameRef` index) plus `Pattern`,
  `Provenance`, and `Layer`. Identity rules. Cross-reference
  accessors. Pattern's structural-only view of `:match` / `:assert`.
  (S1.7.23 — no `Type` / `Instance` classes; the kernel imposes no
  type system.)
- [`02_store.md`](02_store.md) — `KnowledgeBase` registries + reverse
  indexes; the IR loader; layer views (`FactView`); `kb.fork()` for
  hypothesis branches; `derivation_dag` / `unsat_core`; equality
  classes placeholder.
- [`03_python_impl.md`](03_python_impl.md) — the code-level companion
  (**dev-facing**): the `kb/` module map, the frozen-dataclass
  attachment mechanics, and the concrete collection shapes + complexity.

## Reading order

Read `01_entities.md` first to understand the node-kind classes and
how identity works (name vs `(rel, args)`, what's excluded from
equality). Then `02_store.md` for how they're aggregated, indexed,
and viewed. `03_python_impl.md` is the implementer's deep-dive (module
map + mechanics + complexity); skip it if you only need the abstract model.

## Where this maps to code

The `kb/` package under
[`ein.py/src/ein/kb/`](../../../../ein.py/src/ein/kb/) — `entities.py`
(the dataclasses), `pattern.py`, `provenance.py`, `store.py`
(`KnowledgeBase`), `views.py`, `from_ir.py` (loader), `imports.py`,
`render.py`. The file-by-file map with roles is
[`03_python_impl.md` §1](03_python_impl.md).

## Stability

Stable through M1. F4 promotion seams (compound node kinds, e-graph)
are noted explicitly in `01_entities.md` / `02_store.md`; M1 doesn't
implement them but the architecture stays open.

The IR encoding choice (classic `(type …)`/`(instance …)` vs unified
`is-a`) was **resolved in P1.7**: the canonical encoding is `is-a`
([S1.7.6](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.6_kernel_minimization.md)),
and the kernel keeps no type/instance entity-view at all
([S1.7.23](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.23_retire_kernel_type_system.md)) —
both forms are just facts.
