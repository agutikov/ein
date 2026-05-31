# Data model — the store

`KnowledgeBase` is the registry that owns the entity dataclasses
from [`01_entities.md`](01_entities.md), plus the reverse indexes
and the derived-view machinery (layer views, hypothesis forks,
derivation DAGs).

**Sources of truth:**
[`src/ein_bot/kb/store.py`](../../../../src/ein_bot/kb/store.py),
[`src/ein_bot/kb/views.py`](../../../../src/ein_bot/kb/views.py),
[`src/ein_bot/kb/provenance.py`](../../../../src/ein_bot/kb/provenance.py),
[`src/ein_bot/kb/from_ir.py`](../../../../src/ein_bot/kb/from_ir.py).

---

## 1. Registries

The KB owns five dicts keyed by entity name plus one fact list:

```python
class KnowledgeBase:
    types:     dict[str, Type]
    instances: dict[str, Instance]
    relations: dict[str, Relation]   # both declared and open-world
    rules:     dict[str, Rule]
    facts:     list[Fact]             # all layers; `layer` is per-fact
    query:     Query | None           # optional, from (query …)
    classes:   EqClasses              # union-find placeholder
```

Lookups are O(1) for entities, O(|facts|) for the fact list. Per-
relation and per-instance fact lookups go through the reverse
indexes below — O(1) by name.

## 2. Reverse indexes

Six dicts precomputed at load and incrementally maintained on
single-fact additions:

| index                       | maps                                          | populated by      |
|-----------------------------|-----------------------------------------------|-------------------|
| `_types_by_parent`          | parent name → tuple of subtypes               | full rebuild      |
| `_instances_by_type`        | type name → tuple of instances                 | full rebuild      |
| `_facts_by_relation`        | relation name → tuple of facts                 | full + incremental |
| `_facts_by_instance`        | instance name → tuple of facts mentioning it   | full + incremental |
| `_rules_by_relation`        | relation name → tuple of rules over it          | full rebuild      |
| `_rules_by_type`            | type name → tuple of rules touching it          | full rebuild      |
| `_rule_apps_by_rule`        | rule name → tuple of property-application facts | full + incremental |
| `_rule_apps_on_relation`    | relation name → tuple of property facts on it   | full + incremental |

Entities expose these via `@property` accessors (e.g.
`type.instances`, `relation.facts`) — the dicts themselves are
internal.

## 3. Loading from IR — `KnowledgeBase.from_ir(forms)`

The loader walks parsed IR forms in a fixed order:

1. **Pass 1** (ontology block, schema):
   - `(type …)` → `Type` entity.
   - `(relation …)` → `Relation` entity
     (`declared=True`).
   - `(instance …)` → `Instance` entity (auto-vivifies the type if
     absent).
2. **Pass 2** (rules block):
   - `(rule …)` → `Rule` entity with `match` / `assert_` `Pattern`
     objects.
3. **Pass 3** (facts):
   - Ontology-block fact children → `Fact(layer=ONTOLOGY, …)`.
   - `(facts …)` children → `Fact(layer=FACT, …)`.
   - `(reasoning …)` children → `Fact(layer=REASONING, …)`.
   - Any fact whose head is *not* a declared relation auto-vivifies
     a `Relation(declared=False, …)`.
4. **Query**: last `(query …)` block, if any.
5. **Indexes**: `rebuild_indexes()`.
6. **Cycle check**: `detect_provenance_cycles()` over the loaded
   facts; raises `KBLoadError` on circular `:using` chains.

The loader is **open-world tolerant**: undeclared types and
relations auto-vivify rather than fail. Errors accumulate and raise
once at the end with all problems concatenated.

## 4. Layer views — `FactView`

Four methods on `KnowledgeBase` return read-only filtered views over
the fact list:

```python
kb.ontology()    -> FactView   # ONTOLOGY-layer facts
kb.fact_layer()  -> FactView   # FACT-layer facts
kb.reasoning()   -> FactView   # REASONING-layer facts
kb.all_layers()  -> FactView   # every fact
```

`fact_layer()` is named that way — not `facts()` — because `kb.facts`
is the registry list attribute (Python disallows shadowing).

`FactView` is a frozen dataclass wrapping a `tuple[Fact, ...]`. It
supports the sequence protocol (`__iter__` / `__len__` /
`__contains__` / `__bool__`) and four filter methods that return
**iterators**:

- `view.relation(name)` — facts whose head matches `name`.
- `view.about(instance | name)` — facts mentioning an instance.
- `view.by_source(source)` — facts with the given `:source`
  annotation.
- `view.by_rule(rule_name)` — facts with the given rule provenance.

A `view.matching(pattern)` stub exists as a P1.3 seam (raises
`NotImplementedError` until the matcher arrives).

## 5. Fork — `kb.fork()`

A `fork` is a hypothesis branch (per
[`../01-ein-graph/01_kb.md` §6](../01-ein-graph/01_kb.md)). The
implementation:

```python
def fork(self) -> KnowledgeBase:
    new = KnowledgeBase()
    # Shared by reference (immutable post-load):
    new.types      = self.types
    new.instances  = self.instances
    new.relations  = self.relations
    new.rules      = self.rules
    new.query      = self.query
    # Equality classes: forked (its own state, seeded from parent's).
    new.classes._parent = dict(self.classes._parent)
    # Facts list + reverse indexes: shallow-copied so appends to the
    # fork don't touch the parent.
    new.facts = list(self.facts)
    new._facts_by_relation     = dict(self._facts_by_relation)
    new._facts_by_instance     = dict(self._facts_by_instance)
    new._rule_apps_by_rule     = dict(self._rule_apps_by_rule)
    new._rule_apps_on_relation = dict(self._rule_apps_on_relation)
    # … (similar for other indexes)
    return new
```

**Cost:** O(|facts|) for the shallow copies — bounded by Zebra-
scale at ~50-200 facts. If hypothesis branching becomes a hot path
(P1.5 profiling), revisit with a copy-on-write index wrapper.

**Caveat about entity back-pointers:** shared entities keep their
`_kb` pointing at the **original** KB, not the fork. So
`norwegian.facts` (entity API) returns the root KB's facts, *not*
the fork's view. Fork-scoped queries go through the explicit view
API: `fork.all_layers().about(norwegian)`. This is intentional —
the entity API tells you *root* state, the view API tells you
*branch* state.

## 6. Encoding-agnostic logical views

[Memory: project — IR encoding choice deferred](../../../../README.md)
keeps both `zebra.ein` (classic) and `zebra2.ein` (unified is-a)
valid through every M1 stage. Downstream code must NOT assume
`kb.types` is populated.

Two helpers in `views.py` paper over the difference:

```python
logical_types(kb)     -> tuple[Type | str, ...]
logical_instances(kb) -> tuple[Instance | str, ...]
```

- **Classic encoding** (zebra.ein): returns `kb.types.values()` /
  `kb.instances.values()` — typed entities.
- **Unified-is-a encoding** (zebra2.ein): `kb.types` and
  `kb.instances` are empty. The helpers walk `is-a` facts —
  RHS atoms of `(is-a Child Parent)` are the *logical types*; LHS
  atoms that never appear as RHS are the *logical instances* (leaves
  of the is-a forest).

The return is a **union** type (`Type | str` / `Instance | str`)
because in the unified case no entity exists — only a name. Two
companion helpers, `type_name(x)` and `instance_name(x)`, paper over
the union for callers that just want a string.

The drift-detection tests in
`tests/kb/test_layers.py::TestEncodingDriftDetection` assert that
both encodings produce the same logical-leaf set (the only
structural difference is zebra2's catch-all `T` root).

## 7. Provenance + derivation DAG

Per [`01_entities.md` §3](01_entities.md), every fact has a
`Provenance` record. Two `KnowledgeBase` methods walk it:

### 7.1 `kb.derivation_dag(fact) -> DerivationDAG`

BFS from `fact` through `provenance.premises_raw`, resolving each id
via `kb._fact_by_id(rel, args)` and recursing into `rule`-kind facts
only. `source`- and `hypothesis`-kind facts terminate the recursion.

The result is a `DerivationDAG` frozen dataclass:

```python
DerivationDAG(
    root:  Fact,
    nodes: tuple[Fact, ...],
    edges: tuple[tuple[Fact, Fact], ...],   # (premise, conclusion)
)
```

with `.sources` returning the terminal frontier (source + hypothesis
kinds) and `.to_dot()` producing a Graphviz `digraph` string —
boxes for rule-derived, ellipses for source/hypothesis.

Cycles in user-authored provenance are caught at load time
(`detect_provenance_cycles`) and raise `KBLoadError`. Cycles that
arise during derivation (rule misuse) are *broken* by the BFS
visited-set — the revisited fact appears as a node but isn't
re-expanded.

### 7.2 `kb.unsat_core(conflicting) -> set[Fact]`

For each fact in `conflicting`, walks its `derivation_dag` and
accumulates the source-kind terminals. The union is the **minimal
source-frontier** that derives the conflict — the input to the
*contradictions* task class
([`docs/ideas/03-three-task-classes.md`](../../../ideas/03-three-task-classes.md)).

```python
core = kb.unsat_core([conflicting_fact_1, conflicting_fact_2])
# core: set of source-kind Facts; their (rel, args) + source annotation
# tell the user "these were the load-bearing premises".
```

## 8. Equality classes — placeholder

`kb.classes: EqClasses` is a small union-find over instance names:

```python
kb.classes.find(name)       -> root name
kb.classes.union(a, b)      -> merge classes
kb.classes.equivalent(a, b) -> bool
kb.classes.classes()         -> dict[root, list[members]]
```

M1 ships the union-find but doesn't yet wire it to rule firings.
The seam exists for a future e-graph promotion (F4 Q30); equality
saturation can slot in without rework on the rest of the KB.

## 9. Mutation API

Loaders (and the inference engine, P1.3) mutate the KB through:

- `kb.add_type(t)` — idempotent by name.
- `kb.add_instance(inst)` — idempotent by name.
- `kb.add_relation(r)` — idempotent by name; *declared* upgrades
  beat *open-world* placeholders.
- `kb.add_rule(rule)` — idempotent by name.
- `kb.add_fact(f)` — dedupe by `(relation_name, args)`; first-seen
  layer wins.
- `kb._index_fact(f)` — incremental index update; call after a
  single-fact `add_fact` to avoid a full `rebuild_indexes`.
- `kb.rebuild_indexes()` — full rebuild from registries + fact list.

The engine doesn't *remove* facts (the graph is monotonic — see
[`../01-ein-graph/02_rules.md` §1](../01-ein-graph/02_rules.md));
retraction happens via *forking* (the speculative branch is
discarded) rather than mutation.

## 10. Connections

- [`01_entities.md`](01_entities.md) — the dataclasses this store
  owns.
- [`../01-ein-graph/`](../01-ein-graph/) — the conceptual model.
- [`../03-ein-lang/`](../03-ein-lang/) — the surface syntax the
  loader parses.
- [`../../inference/`](../../inference/) — the P1.3 stub that will
  produce reasoning-layer facts via rule firings.
- Plan: [`plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/`](../../../../plans/m1_core_graph_reasoning/p1.2_typed_hypergraph/).
