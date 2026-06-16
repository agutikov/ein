# Data model — Python implementation map

The **file-by-file** developer reference for the `kb/` package: the module
layout, the frozen-dataclass attachment mechanics, and the concrete collection
shapes + complexity. The *idiomatic* level — what each entity carries and how
the store behaves abstractly — is [`01_entities.md`](01_entities.md) (entities)
and [`02_store.md`](02_store.md) (store); this page is the code-level companion
(the data-model analog of [`../../inference/python_impl.md`](../../inference/python_impl.md)).

> **Audience: engine contributors.** Puzzle authors want
> [`../03-ein-lang/`](../03-ein-lang/); a non-Python reimplementer wants the
> abstract shapes in `01_entities` / `02_store` and can skip this page.

## 1. `kb/` module map

Source root: [`ein.py/src/ein/kb/`](../../../../ein.py/src/ein/kb/).

| module | role |
|--------|------|
| [`entities.py`](../../../../ein.py/src/ein/kb/entities.py) | the frozen entity dataclasses (`Relation`, `Rule`, `Fact`, `NameRef`) + `Layer`; `_attach` / `_detach`; `KERNEL_META_RELATIONS` |
| [`pattern.py`](../../../../ein.py/src/ein/kb/pattern.py) | `Pattern` — the structural-only view of a rule's `:match` / `:assert` (variables, relation names; no matching) |
| [`provenance.py`](../../../../ein.py/src/ein/kb/provenance.py) | `Provenance` (4 kinds) + its constructors; `DerivationDAG` + `.to_dot()`; `FactId` |
| [`store.py`](../../../../ein.py/src/ein/kb/store.py) | `KnowledgeBase` — registries, the reverse indexes, `fork()`, `derivation_dag` / `unsat_core`, the mutation API; `EqClasses`; `Query` |
| [`views.py`](../../../../ein.py/src/ein/kb/views.py) | `FactView` — read-only layer views + the `relation` / `about` / `by_source` / `by_rule` filters |
| [`from_ir.py`](../../../../ein.py/src/ein/kb/from_ir.py) | `KnowledgeBase.from_ir` — the flat-form loader (route by head, per-fact layer, open-world auto-vivify, cycle check) |
| [`imports.py`](../../../../ein.py/src/ein/kb/imports.py) | the module-import resolver (`std.<path>` → `stdlib/<path>.ein`; `:as` / `:symbols` + auto-closure, P1.8) |
| [`render.py`](../../../../ein.py/src/ein/kb/render.py) | `KnowledgeBase.to_dot` — the schema/fact DOT renderer (`_schema_nodes` reads `is-a` / `(type …)` / `(instance …)` facts directly) |

## 2. Dataclass attachment mechanics

All entity classes are **frozen** (hashable, identity-stable). The
owning-KB back-pointer is wired after construction:

- `_kb` is set via `object.__setattr__` (the frozen dataclass forbids normal
  assignment) by `_attach(entity, kb)`; `_detach` clears it.
- `_kb` is **excluded** from `__eq__` / `__hash__` / `__repr__`, so two
  entities of the same kind with the same identity tuple are equal *across*
  KBs — the property that makes `kb.fork()`'s by-reference sharing sound.
- Identity tuples: `Relation` = `(name, signature)`, `Rule` = `(name,)`,
  `Fact` = `(relation_name, args)` (recursive — `args` may hold nested
  `Fact`s), `Provenance` = all fields except `loc`. Metadata (`loc`, `layer`,
  `provenance`, `_kb`, `raw`) never affects equality.

The per-entity attachment detail is [`01_entities.md` §5](01_entities.md);
this is the consolidated view.

## 3. Collections & complexity

What backs each store member, and the cost of using it:

| member | shape | lookup | maintenance |
|--------|-------|--------|-------------|
| `relations` / `rules` / `hrules` / `names` | `dict[str, …]` | O(1) by name | idempotent `add_*` |
| `facts` | `list[Fact]` | O(\|facts\|) scan | append-only; dedupe by `(rel, args)`, first-seen layer wins |
| `_facts_by_relation` | `dict[str, tuple[Fact,…]]` | O(1) by relation | full @ load + incremental on `add_fact` |
| `_rule_apps_by_rule` / `_rule_apps_on_relation` | `dict[str, tuple[Fact,…]]` | O(1) | full + incremental |
| `_rules_by_relation` | `dict[str, tuple[Rule,…]]` | O(1) | full rebuild (immutable post-load) |
| `classes` (`EqClasses`) | union-find | ~O(α) | M1 placeholder; not wired to firings |

- **Incremental indexing:** `kb._index_fact(f)` updates the fact/name indexes
  after a single `add_fact`, avoiding a full `rebuild_indexes()`.
- **Fork cost:** `kb.fork()` is O(\|facts\|) — it shares `relations` / `rules` /
  `hrules` / `query` / `_rules_by_relation` by reference (immutable post-load)
  and shallow-copies the fact list + the four mutable indexes (see
  [`02_store.md` §5](02_store.md)). Bounded by Zebra scale (~50–200 facts); a
  copy-on-write index wrapper is the noted seam if branching becomes hot.
- **Reverse-index removal (S1.7.23):** `_types_by_parent` /
  `_instances_by_type` / `_facts_by_instance` / `_rules_by_type` are gone with
  the `Type` / `Instance` entity-view; named-type projection is now a
  user-space rule.

## See also

- [`01_entities.md`](01_entities.md) / [`02_store.md`](02_store.md) — the
  idiomatic entity + store reference this maps to code.
- [`../../inference/python_impl.md`](../../inference/python_impl.md) — the
  engine's module map (same code-level treatment for `inference/`).
- [`../../architecture.md`](../../architecture.md) — where `kb/` sits in the
  package dependency map.
