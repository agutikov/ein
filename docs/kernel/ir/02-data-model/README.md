# Data model — the in-memory representation

How the graph from [`../01-ein-graph/`](../01-ein-graph/) is held in
memory: the entity kinds, the knowledge-base store that owns them, the
indexes that make cross-references O(1), the fact view, the
hypothesis-fork mechanic, and the provenance + derivation-DAG machinery.

## Files

- [`01_entities.md`](01_entities.md) — the entity kinds
  (`Relation`, `Rule`, `Fact` + the `NameRef` index) plus `Pattern`,
  and `Provenance`. Identity rules. Cross-reference
  accessors. Pattern's structural-only view of `:match` / `:assert`.
  (S1.7.23 — no `Type` / `Instance` classes; the kernel imposes no
  type system.)
- [`02_store.md`](02_store.md) — the store's registries + indexes; the
  IR loader; the fact view; `fork()` for hypothesis branches; the
  derivation DAG / unsat core; equality classes placeholder.
- [`03_implementation.md`](03_implementation.md) — the code-level companion
  (**dev-facing**): the module map, the identity mechanics, and the
  concrete collection shapes + complexity.

## Reading order

Read `01_entities.md` first to understand the node-kind classes and
how identity works (name vs `(rel, args)`, what's excluded from
equality). Then `02_store.md` for how they're aggregated, indexed,
and viewed. `03_implementation.md` is the implementer's deep-dive (module
map + mechanics + complexity); skip it if you only need the abstract model.

## Where this maps to code

[`ein-core`](../../../../ein.rs/crates/ein-core/src/) — `intern.rs`,
`value.rs`, `facts.rs`, `terms.rs`, `entities.rs`, `program.rs`, `kb.rs`,
`prov.rs` — with the loader in `ein-ir` (`from_ir.rs`, `imports.rs`) and the
DOT view in `ein-render` (`kb_dot.rs`). The module-by-module map with roles is
[`03_implementation.md` §1](03_implementation.md).

## Stability

Stable through M1. F4 promotion seams (compound node kinds, e-graph)
are noted explicitly in `01_entities.md` / `02_store.md`; M1 doesn't
implement them but the architecture stays open.

The IR encoding choice (one generic link relation vs typed attribute relations,
`is-a`) was **resolved in P1.7**: the canonical encoding is `is-a`
(S1.7.6),
and the kernel keeps no type/instance entity-view at all
(S1.7.23) —
both forms are just facts.
