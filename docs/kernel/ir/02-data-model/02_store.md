# Data model — the store

`KnowledgeBase` is the registry that owns the entity dataclasses
from [`01_entities.md`](01_entities.md), plus the reverse indexes
and the derived-view machinery (the fact view, hypothesis forks,
derivation DAGs).

**Sources of truth:**
[`src/ein/kb/store.py`](../../../../ein.py/src/ein/kb/store.py),
[`src/ein/kb/views.py`](../../../../ein.py/src/ein/kb/views.py),
[`src/ein/kb/provenance.py`](../../../../ein.py/src/ein/kb/provenance.py),
[`src/ein/kb/from_ir.py`](../../../../ein.py/src/ein/kb/from_ir.py).

---

## 1. Registries

The KB owns the entity-name dicts plus one fact list:

```python
class KnowledgeBase:
    relations: dict[str, Relation]   # both declared and open-world
    rules:     dict[str, Rule]
    hrules:    dict[str, Rule]        # hypothesis-generation rules
    facts:     list[Fact]             # every fact; origin is per-fact provenance
    names:     dict[str, NameRef]     # every distinct name + participation
    query:     Query | None           # optional, from (query …)
    classes:   EqClasses              # union-find placeholder
```

> **S1.7.23 — no `types` / `instances` registries.** The kernel keeps
> no type-system entity-view; a puzzle's membership facts are ordinary
> facts and the inheritance forest is just `is-a` facts. See
> [`01_entities.md` §1](01_entities.md).

Lookups are O(1) for entities, O(|facts|) for the fact list. Per-
relation fact lookups go through the reverse indexes below — O(1) by
name.

## 2. Reverse indexes

Precomputed at load and (for the fact / name indexes) incrementally
maintained on single-fact additions:

| index                       | maps                                          | populated by      |
|-----------------------------|-----------------------------------------------|-------------------|
| `_facts_by_relation`        | relation name → tuple of facts                 | full + incremental |
| `_rules_by_relation`        | relation name → tuple of rules over it          | full rebuild      |
| `_rule_apps_by_rule`        | rule name → tuple of property-application facts | full + incremental |
| `_rule_apps_on_relation`    | relation name → tuple of property facts on it   | full + incremental |
| `names`                     | name → `NameRef` (head / arg participation)    | full + incremental |

Entities expose these via `@property` accessors (e.g. `relation.facts`)
— the dicts themselves are internal. (S1.7.23 removed the
`_types_by_parent` / `_instances_by_type` / `_facts_by_instance` /
`_rules_by_type` indexes — they served only the deleted `Type` /
`Instance` accessors.)

## 3. Loading from IR — `KnowledgeBase.from_ir(forms)`

The loader walks the **flat sequence of forms** (P1.7c — no block
wrappers; see [`../03-ein-lang/06_reserved_names.md`](../03-ein-lang/06_reserved_names.md)),
routing each by its **head**:

1. **Declarators** (by head):
   - `(relation …)` → `Relation` entity (`declared=True`).
   - `(rule …)` / `(hrule …)` → `Rule` entity with `match` / `assert_`
     `Pattern` objects.
   - `(query …)` → the `Query` (last one wins).
2. **Facts** (any other head — `=`, `not`, or a generic `(NAME …)`):
   - membership facts are ordinary `Fact`s (no
     `Type` / `Instance` entities; they are plain facts on user-space
     relations, see [`01_entities.md` §1](01_entities.md)).
   - The fact's **origin is its provenance**: `:rule`/`:using` → a
     `rule`-kind record, `:source` → a `source`-kind record with the id,
     neither → a `source`-kind record with `source=None`. An explicit
     `:layer` is a **load error** (S1.22.1b — knowledge layers gone).
   - Any fact whose head is *not* a declared relation auto-vivifies
     a `Relation(declared=False, …)`.
3. **Indexes**: `rebuild_indexes()`.
4. **Cycle check**: `detect_provenance_cycles()` over the loaded
   facts; raises `KBLoadError` on circular `:using` chains.

The loader is **open-world tolerant**: undeclared types and
relations auto-vivify rather than fail. Errors accumulate and raise
once at the end with all problems concatenated.

## 4. Fact view — `FactView`

One method on `KnowledgeBase` returns a read-only filtered view over the
fact list:

```python
kb.all_facts()  -> FactView   # every fact
```

Named `all_facts()` — not `facts()` — because `kb.facts` is the registry
list attribute (Python disallows shadowing). The three *layer-scoped*
siblings (`ontology()` / `fact_layer()` / `reasoning()`) and
`facts_in_layer()` were deleted with the `Layer` enum in S1.22.1b: they
partitioned the list by a denormalised copy of the provenance, the
`by_source` / `by_rule` filters below select the same facts from the
provenance itself, and no engine code called them.

`FactView` is a frozen dataclass wrapping a `tuple[Fact, ...]`. It
supports the sequence protocol (`__iter__` / `__len__` /
`__contains__` / `__bool__`) and four filter methods that return
**iterators**:

- `view.relation(name)` — facts whose head matches `name`.
- `view.about(name)` — facts mentioning a name.
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
    new.relations  = self.relations
    new.rules      = self.rules
    new.hrules     = self.hrules
    new.query      = self.query
    # Equality classes: forked (its own state, seeded from parent's).
    new.classes._parent = dict(self.classes._parent)
    # Facts list + reverse indexes: shallow-copied so appends to the
    # fork don't touch the parent.
    new.facts = list(self.facts)
    new._facts_by_relation     = dict(self._facts_by_relation)
    new._rule_apps_by_rule     = dict(self._rule_apps_by_rule)
    new._rule_apps_on_relation = dict(self._rule_apps_on_relation)
    new.names                  = dict(self.names)
    # Recorded justifications (§7) follow the *facts* contract, not the
    # shared-by-reference one: a branch-local derivation may name
    # hypothesis premises the root never assumed.
    new._alt_justifications    = dict(self._alt_justifications)
    # … (`_rules_by_relation` shared by reference; immutable post-load)
    return new
```

**Cost:** O(|facts|) for the shallow copies — bounded by Zebra-
scale at ~50-200 facts. If hypothesis branching becomes a hot path
(P1.5 profiling), revisit with a copy-on-write index wrapper.

**Caveat about entity back-pointers:** shared entities keep their
`_kb` pointing at the **original** KB, not the fork. So a shared
`Relation`'s `.facts` (entity API) returns the root KB's facts, *not*
the fork's view. Fork-scoped queries go through the explicit view
API: `fork.all_facts().about(name)`. This is intentional —
the entity API tells you *root* state, the view API tells you
*branch* state.

## 6. Type / instance views — removed (S1.7.23)

This section used to document `logical_types` / `logical_instances`
(and `type_name` / `instance_name`) — the encoding-agnostic
`is-a`-bridge over the `kb.types` / `kb.instances` entity-view. **All
of it is gone** (S1.7.23):
the kernel imposes no type system, so there is no derived
types-and-instances view to maintain. A puzzle that wants a named-type
projection computes it with a user-space ein-lang rule over its own
inheritance relation; the renderer reads the puzzle's `is-a` facts
directly (`kb/render.py:_schema_nodes`).

## 7. Provenance + derivation DAG

Per [`01_entities.md` §3](01_entities.md), every fact has a
`Provenance` record — its **primary** justification. A fact the engine
derived more than one way carries the other derivations too, in a KB
side table reached through four methods:

```python
kb.justifications(fact)  -> tuple[Provenance, ...]  # the OR-node, primary first
kb.record_justification(fact, prov) -> bool         # engine-side; True if newly kept
kb.accepts_justification(fact, n_premises) -> bool  # O(1) hot-path pre-check
kb.has_alternative_justifications()  -> bool
```

So a fact is an OR-node and each justification an AND-node over its
`premises_raw` — the proof structure is an AND/OR **graph**, and the
two walkers below take an `all_justifications` keyword saying which
reading they want. Which records are keepable (rule-kind, at least one
premise, onto a non-terminal primary), the `(rule, premises_raw)`
dedup key and the `MAX_ALT_JUSTIFICATIONS` cap are in
[`01_entities.md` §3.1](01_entities.md); the store-side contracts are:

- **Where it is recorded.** `kb.add_and_index_fact` — the
  saturation-time add — returns the pre-existing fact on a dedup hit and records
  the incoming `provenance` as an alternative instead of dropping it.
  That covers the partially-novel multi-assert case; the bulk of
  re-derivations never reach it, because the saturator short-circuits
  a wholly-redundant firing before it builds any fact and calls
  `record_justification` itself, as does the `__symmetric__` native
  mirror.
- **Copy contract.** The table follows the *facts* contract —
  shallow-copied per `fork()` (§5) and per `snapshot()`, never shared
  by reference. A justification recorded inside a branch may name
  `hypothesis`-kind premises the root never assumed, and sharing would
  leak that phantom assumption into a root-level core.
- **Not rebuildable.** `rebuild_indexes()` deliberately leaves it
  alone. Every index in §2 is a projection of `kb.facts` and so is
  safe to drop and recompute; this is a record of derivations the
  engine *attempted*, which no amount of looking at the current fact
  set reconstructs.

A consumer writing its own walk gets the same choice explicitly:
`provenance.walk_premises(…, justifications=…)` and
`build_derivation_dag(…, justifications=…)` share the selector
`provenance.justifications_of()` — `None` (the default) means the
primary justification alone, `kb.justifications` makes the walk
OR-aware.

### 7.1 `kb.derivation_dag(fact, *, all_justifications=False) -> DerivationDAG`

BFS from `fact` through `provenance.premises_raw`, resolving each id
via `kb._fact_by_id(rel, args)` and recursing into `rule`-kind facts
only. `source`- and `hypothesis`-kind facts terminate the recursion.

The result is a `DerivationDAG` frozen dataclass:

```python
DerivationDAG(
    root:      Fact,
    nodes:     tuple[Fact, ...],
    edges:     tuple[tuple[Fact, Fact], ...],   # (premise, conclusion)
    and_nodes: tuple[tuple[Fact, tuple[Fact, ...]], ...],  # one per justification
)
```

with `.sources` returning the terminal frontier (source + hypothesis
kinds) and `.to_dot()` producing a Graphviz `digraph` string —
boxes for rule-derived, ellipses for source/hypothesis.

`all_justifications=True` expands every recorded derivation rather
than the primary one. `and_nodes` then carries the conjunction
structure `edges` cannot: one entry per justification, pairing a
conclusion with the premises of that *one* derivation — in a flat
edge set the in-edges of a node are the union over derivations, so
which subset constitutes one proof is unrecoverable. `.is_or_graph`
is True once some fact has more than one justification, and `.to_dot()`
then draws a small diamond per justification (premises feeding it, it
feeding the conclusion) so alternatives read as alternatives instead
of as one big conjunction. The default stays primary-only because the
DAG is a *display* object: one derivation per fact is what a reader
can follow.

Cycles in user-authored provenance are caught at load time
(`detect_provenance_cycles`, §3) and raise `KBLoadError`. That check
is primary-only and load-time-only **on purpose**: engine-*recorded*
provenance is legitimately cyclic — once re-derivations are recorded,
symmetric / transitive closure has `(R a b)` and `(R b a)` justifying
each other in any ordinary puzzle — so running it over a saturated KB
would reject well-founded KBs. Cycles met during a walk are *broken*
by the BFS visited-set — the revisited fact appears as a node but
isn't re-expanded; an OR-aware consumer has to handle them itself,
which `inference/explain.py` does by taking a least fixpoint from the
frontier upward.

### 7.2 `kb.unsat_core(conflicting, *, all_justifications=False) -> set[Fact]`

For each fact in `conflicting`, walks its premise closure (the shared
`provenance.walk_premises`, with one `visited` set memoising across
the conflicting facts) and accumulates the frontier terminals —
source-kind, hypothesis-kind, or un-provenanced givens. The union is
the **recorded source-frontier** that derives the conflict (per the
recorded derivations — by default the primary justification of each
fact; not a subset-minimal MUS) — the input to the *contradictions*
task class
([`docs/ideas/03-three-task-classes.md`](../../../../plans/ideas/03-three-task-classes.md)).

```python
core = kb.unsat_core([conflicting_fact_1, conflicting_fact_2])
# core: set of source-kind Facts; their (rel, args) + source annotation
# tell the user "these were the load-bearing premises".
```

`all_justifications=True` walks every recorded derivation instead, so
the frontier is the union over the whole AND/OR closure. The default
is primary-only and stays that way deliberately: unioning over
alternatives makes a core monotonically **larger**, the opposite of a
legible explanation (and the union across *witnesses* already
over-states a conflict — one cause fanning out into many witnesses).
The OR-aware union's use is as a soundness envelope — no explanation
of these conflicts can name a fact outside it.

Neither reading is minimal — this method unions, it never chooses:
across the conflicting facts always, and across each fact's
justifications under the opt-in. For the smallest answer use
[`inference/frontier.py`](../../../../ein.py/src/ein/inference/frontier.py)'s
`smallest_contradiction_frontier` — a minimum-cardinality AND/OR
search over every recorded derivation (provenance-based, NAF-safe,
budgeted); **not** a subset-minimal MUS. It picks one justification
per fact and one witness to explain, so its result is always a subset
of the union core and is independent of the order in which the rules
fired; it is what a `k = 0` verdict's `unsat_core` is built from (per
dead commitment — with an exhausted lattice the verdict still unions
across the dead commitments, since no single one explains the unsat).
Two further caveats: the alternatives searched are only the firings
the saturator attempted, capped per fact, so "minimal" stays relative
to the rule set and the saturation strategy; and the search is
budgeted — call `inference.explain.minimal_contradiction_frontier`
directly to read `Explanation.exhausted`, which reports whether a cap
was hit (a truncated search is still sound).

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
  provenance wins.
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
  produce derived facts via rule firings.
- Plan: `M1 P1.2`.
