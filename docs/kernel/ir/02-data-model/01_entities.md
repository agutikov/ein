# Data model — entities

How the graph from [`../01-ein-graph/`](../01-ein-graph/) is held in
memory. Frozen Python dataclasses with identity by name, attached to
the owning :class:`KnowledgeBase` via a `_kb` back-pointer for
cross-reference lookups.

**Source of truth:**
[`src/ein/kb/entities.py`](../../../../ein.py/src/ein/kb/entities.py).
This document explains the mapping graph-node ↔ Python class; the
code is authoritative for field shapes.

---

## 1. The three entity classes

Each kind of graph node from
[`../01-ein-graph/01_kb.md` §1](../01-ein-graph/01_kb.md) that the
engine reasons over has a corresponding frozen dataclass. Identity
follows the table:

| graph node kind | Python class | identity (`__eq__` / `__hash__`) |
|-----------------|--------------|----------------------------------|
| Relation        | `Relation`   | `name` + `signature`              |
| Rule            | `Rule`       | `name`                           |
| Fact            | `Fact`       | `(relation_name, args)`           |

> **S1.7.23 — no `Type` / `Instance` entity classes.** The kernel
> imposes no type system: there is no derived types-and-instances
> entity-view (and no `kb.types` / `kb.instances` registries). Object
> and type nodes are just **names** — they appear as `Fact` args and
> as the heads/args of the puzzle's own `is-a` / `(type …)` /
> `(instance …)` facts; the participation of every name is captured by
> the lightweight `NameRef` index (§1.4). A puzzle that wants a
> named-type projection computes it with a user-space ein-lang rule
> over its own inheritance relation. See
> [S1.7.23](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.23_retire_kernel_type_system.md).

Metadata fields (`loc`, `provenance`, `layer`, `_kb`, `raw`) are
**excluded** from identity — two facts with the same `(rel, args)`
but different layers or sources are *the same fact* for dedup
purposes (see [`../01-ein-graph/01_kb.md` §4](../01-ein-graph/01_kb.md)).

### 1.1 `Relation`

A relation declaration. Note: relations are **first-class nodes**
in the graph — they participate in facts (`(symmetric co-located)`)
as themselves, not as edge labels.

```python
Relation(
    name:       str,
    signature:  tuple[str, ...],   # argument-position type names
    declared:   bool,               # True for explicit `(relation …)` decls;
                                    # False for open-world auto-vivified relations
    loc:        Loc | None,
    _kb:        KnowledgeBase | None,
)
```

The `declared` flag distinguishes explicitly-declared relations from
**open-world auto-vivified** ones — relations whose names appear as
fact heads without an accompanying `(relation …)` declaration
(typically the property tags `symmetric`, `transitive`, etc., which
are *also* rule names). Both flavours are graph nodes; the flag is
metadata for the schema-validator.

The `signature` holds the argument-position type **names** (opaque
atoms — S1.7.23 keeps no `Type` entities to resolve them to; hypgen
uses them only as object-exclusion metadata).

Cross-references:
- `rel.facts` → `tuple[Fact, ...]` — all facts whose head is this
  relation's name.
- `rel.properties` → `tuple[Fact, ...]` — rule-application facts
  targeting this relation (T2 activators from
  [`../01-ein-graph/02_rules.md` §2.2](../01-ein-graph/02_rules.md)).
- `rel.rules` → `tuple[Rule, ...]` — rules naming this relation, OR
  whose name appears as a property-fact head on this relation.
- `rel.rule` → `Rule | None` — if this relation's *name* matches a
  rule (e.g. `symmetric`), the corresponding rule. Non-None for
  property-tag carriers.

### 1.2 `Rule`

A graph rewriting rule. See
[`../01-ein-graph/02_rules.md`](../01-ein-graph/02_rules.md) for
the three types.

```python
Rule(
    name:     str,
    params:   tuple[str, ...],     # ?vars in the parameter list
    match:    Pattern | None,      # LHS
    assert_:  Pattern | None,      # RHS  (underscore avoids `assert` kw)
    why:      str,                 # trace-message template
    priority: int | None,
    loc:      Loc | None,
    _kb:      KnowledgeBase | None,
)
```

`match` / `assert_` are `Pattern` objects — the structural view of
the IR clause (see §2 below). The matching semantics is in P1.3;
here we only carry the structure.

Cross-references:
- `rule.relations` → `tuple[Relation, ...]` — relations mentioned by
  literal name in `match` or `assert_`.
- `rule.applications` → `tuple[Fact, ...]` — property-facts whose
  head is this rule's name (T2 activations). For example, the
  `symmetric` rule's `applications` includes `(symmetric co-located)`
  and `(symmetric next-to)`.

### 1.3 `Fact`

A hyperedge — an instance of a relation applied to specific
arguments. The proposition.

```python
Fact(
    relation_name: str,
    args:          tuple[str | int | Fact, ...],   # argument identities (admits relational-node args)
    layer:         Layer,                          # ONTOLOGY | FACT | REASONING
    provenance:    Provenance | None,              # PRIMARY justification (§3.1)
    raw:           IRNode | None,                  # original IR node (metadata)
    loc:           Loc | None,
    _kb:           KnowledgeBase | None,
)
```

Arguments admit three shapes, matching the kernel ein model's
**named** vs **relational** node duality
([`../01-ein-graph/03_ein_model.md` §3](../01-ein-graph/03_ein_model.md)):

- `str` — a named node (an object name or a Relation name).
- `int` — a numeric literal.
- `Fact` — a **relational node** embedded as an argument
  (e.g. `(hypothesis (co-located Norwegian House-2))`). The nested
  `Fact` participates in identity recursively: two outer facts are
  equal iff their `(relation_name, args)` tuples compare equal
  element-wise, with nested `Fact` instances cascading via their
  own `__eq__`.

Resolution to typed entities happens on demand via
`fact.arg_entities`; nested `Fact` args are returned as-is.

Cross-references:
- `f.relation` → `Relation | None` — the relation entity this fact
  instantiates.
- `f.arg_entities` → `tuple[Relation | Fact | str | int, ...]` —
  resolve each name arg to its `Relation` entity when declared;
  object names stay raw strings (S1.7.23 — no `Type` / `Instance`
  entities), nested `Fact` args pass through.
- `f.is_rule_application` → `bool` — True iff the fact's head matches
  a declared rule name (i.e., the fact is a property activator).
- `f.applied_rule` → `Rule | None` — the rule it activates.
- `f.source` / `f.rule_name` / `f.using` — backward-compat shorthand
  read through to the **primary** `provenance`. See
  [§3 below](#3-provenance) and [`02_store.md`](02_store.md).
- `f.premises` → `tuple[Fact, ...]` — for rule-kind provenance, the
  premise facts of that **primary** justification, resolved via the
  owning KB. A fact the engine derived a second way keeps that other
  derivation in the KB, not on the entity — `kb.justifications(f)`
  ([§3.1](#31-one-record-per-derivation--the-andor-proof-graph)).

### 1.4 `NameRef` — the global names index

Since S1.7.23 dropped the `Type` / `Instance` entity-view, the
encoding-agnostic record of "every distinct name and where it
participates" is a lightweight `NameRef` (one per name in `kb.names`),
not a typed entity:

```python
NameRef(
    name:     str,
    category: "object" | "relation" | "rule",  # the discriminator
    as_head:  tuple[Fact, ...],   # facts where the name is the head
    as_arg:   tuple[Fact, ...],   # facts where it appears as a string arg
)
```

`category` is `"relation"` for declared relations + the kernel-meta
heads (`relation` / `rule`), `"rule"` for rule names, and `"object"`
for everything else. `hypgen._candidate_objects` reads it (minus
signature-type names and the reserved primitives) to pick the blind
enumerator's guessable objects — replacing the old `is-a`-leaf /
`kb.instances` selection.

---

## 2. `Pattern` — structural view of `:match` / `:assert`

A `Pattern` lifts a rule's `:match` or `:assert` IR clause into a
typed object that knows three things about it without performing
matching:

```python
Pattern(
    expr:                  IRNode,           # the raw IR clause
    variables:             tuple[str, ...],  # ?vars bound by this pattern
    relation_names:        tuple[str, ...],  # relations named literally
    type_names:            tuple[str, ...],  # types touched via instance forms
    has_instance_pattern:  bool,             # any `(instance ?_ T)` premise
)
```

The pattern object is **structural-only** for M1 — the matching
semantics (binding, unification, backtracking) lives in P1.3 with
the inference engine. The Pattern serves as the type-checker's view
of a clause and as the data the `Rule.relations` cross-reference
walks. (`type_names` / `has_instance_pattern` are now **vestigial** —
S1.7.23 removed the `Rule.types` / `_rules_by_type` consumers with the
`Type` entity; the fields remain only as structural metadata.)

A small example — the LHS `(and (?rel ?a ?b) (?rel ?b ?c))` of the
`transitive` rule:

| field                  | value                          |
|------------------------|--------------------------------|
| `variables`            | `('rel', 'a', 'b', 'c')`       |
| `relation_names`       | `()` — `?rel` is a Var, not literal |
| `type_names`           | `()` — no instance premise     |
| `has_instance_pattern` | `False`                        |

Contrast with `type-exclusivity`'s LHS `(and (instance ?a ?T) (instance ?b ?T))`:

| field                  | value                          |
|------------------------|--------------------------------|
| `variables`            | `('a', 'T', 'b')`              |
| `relation_names`       | `('instance',)` — literal head |
| `type_names`           | `()` — `?T` is a Var, not literal |
| `has_instance_pattern` | `True`                         |

---

## 3. `Provenance` — where each fact came from

Per [`../01-ein-graph/01_kb.md` §5](../01-ein-graph/01_kb.md), every
fact carries a `Provenance` record of one of four kinds. The
dataclass:

```python
Provenance(
    kind:          str,        # 'source' | 'rule' | 'hypothesis' | 'rejected'
    # source-kind:
    source:        str | None,
    # rule-kind:
    rule:          str | None,
    premises_raw:  tuple[FactId, ...],         # (rel, args) tuples
    bindings:      tuple[tuple[str, str], ...], # (var, name) pairs
    # hypothesis-kind:
    branch:        int | None,
    loc:           Loc | None,                  # metadata
)
```

Convenience constructors:
- `Provenance.from_source(source, loc)` — for facts ingested at load.
- `Provenance.from_rule(rule, premises_raw, bindings, loc)` — for
  rule firings.
- `Provenance.from_hypothesis(branch, loc)` — for speculative facts.
- `Provenance.rejected(branch, loc)` — for facts in retracted
  branches.

Premises in a `rule`-kind record are stored as **fact-ids**
`(relation_name, args)` rather than direct references; this avoids
circular structural references. Resolution to live `Fact` objects
happens through the owning KB (`Fact.premises` property).

### 3.1 One record per derivation — the AND/OR proof graph

Provenance is per **derivation**, not per fact: `Fact.provenance` is
the primary justification and `kb.justifications(fact)` returns every
recorded one, so a fact is an OR-node over AND-nodes — the proof
structure is an AND/OR graph.

A `Fact` is one canonical object per `(relation_name, args)` identity
(§1.3), and several rules may derive it:

- `Fact.provenance` — the **primary** justification: the first
  derivation recorded for that identity. Re-derivation never changes
  it, so everything reading a fact's `:rule` / `:using` / premises
  keeps reading one stable story.
- `kb.justifications(fact) -> tuple[Provenance, ...]` — the whole
  OR-node, primary first, each entry an AND-node over its
  `premises_raw`. The alternatives live in a KB side table
  (`kb._alt_justifications`, keyed by fact-id), never on the entity:
  `Fact` is frozen and its one canonical object is shared by every
  index and every fork, so it cannot grow a field per re-derivation.
  See [`02_store.md` §7](02_store.md).

**Terminals take no alternatives.** Only rule-kind provenance with at
least one premise is recordable, and only onto a fact whose primary is
itself such a record:

- a `source` / `hypothesis` primary is the derivation **frontier** —
  what the engine treats as given. A clue that also happens to be
  re-derivable is still a clue: a given stays given.
- a rule-kind primary with an **empty** `premises_raw` is a synthetic
  engine writeback (`<forced-positive>` and friends, see
  [`../../inference/reserved_engine_strings.md`](../../inference/reserved_engine_strings.md))
  whose contract is that provenance walks *ground out* on it.

Two firings that consumed the same premises are the same
justification: the table dedups on the AND-node `(rule,
premises_raw)`. `bindings` is display metadata the trace reads off the
primary, so it is not part of that key — and an alternative recorded
on the saturator's redundant path carries none at all. The per-fact
list is capped at `store.MAX_ALT_JUSTIFICATIONS` (32) and kept sorted
by premise count, so the cap retains the **shortest** derivations —
the ones a minimum-cardinality explanation can use.

Recording is engine-side. The saturator's share of it — a redundant
firing (the bulk: ~194k of them on an exhaustive `zebra2` solve,
against 8 store-level dedup hits) and the `__symmetric__` native
mirror — is gated by
`SolverConfig.record_alternative_justifications`, default on, measured
at +2.5% median on that solve. With it off only the store's own dedup
seam still records, so facts are back to one justification apiece for
practical purposes and every walk below reads the primary.

The derivation DAG falls out by walking premises transitively —
primary-only (the default at every call site) for one derivation per
fact, `all_justifications=True` for the AND/OR graph; see
[`02_store.md` §7](02_store.md) for `derivation_dag` / `unsat_core`
and that opt-in. Choosing the best *combination* of justifications is
a different problem from walking one:
[`inference/explain.py`](../../../../ein.py/src/ein/inference/explain.py)
searches the AND/OR graph for a minimum-cardinality frontier under an
explicit budget — **not** a subset-minimal MUS, and minimal only over
the derivations the saturator actually recorded.

---

## 4. `Layer` — three knowledge populations

```python
class Layer(Enum):
    ONTOLOGY  = 'ontology'
    FACT      = 'fact'
    REASONING = 'reasoning'
```

The same `(rel, args)` exists at most once in the KB; the `layer`
records its origin. Loader populates ONTOLOGY and FACT; rule firings
(P1.3) populate REASONING. Hypothesis branches add facts in
REASONING with `kind='hypothesis'` provenance.

---

## 5. Entity attachment — frozen, but back-pointed

All entity dataclasses are **frozen** for hashability + identity
guarantees. The owning-KB back-pointer (`_kb`) is set by the
KnowledgeBase after construction via `object.__setattr__`. The
back-pointer is **excluded** from `__eq__` / `__hash__` / `__repr__`:
two entities of the same kind with the same name (or `(rel, args)`
for facts) are equal across KBs.

Detaching an entity (e.g., for serialisation) is supported via the
`_detach()` helper; after detach, all cross-reference properties
return empty tuples / `None`.

## 6. Identity rules — summary

```text
   Relation:   (name, signature)
   Rule:       (name,)
   Fact:       (relation_name, args)        — recursive: args may contain Fact
   Pattern:    (expr,)   — by structural IR equality
   Provenance: (all data fields except `loc`)
```
(S1.7.23 — no `Type` / `Instance` entities.)

Two entities are equal iff their identity tuples are equal — `loc`,
layer, provenance, and back-pointers never affect equality. For
nested-fact args, equality cascades pointwise: outer Facts are
equal iff their nested Fact args are equal (which they are iff
*their* `(relation_name, args)` match, recursively).

A fact's identity ignoring provenance is what makes the OR-node of
[§3.1](#31-one-record-per-derivation--the-andor-proof-graph) possible:
the same proposition derived twice is one `Fact` carrying two
justifications, not two facts. Note the two different notions of
"same justification" — `Provenance.__eq__` compares every data field
(`bindings` included), while the alternatives table dedups on the
coarser AND-node key `(rule, premises_raw)`, so two firings of one
rule over the same premises collapse however they were bound.

## See also

- [`02_store.md`](02_store.md) — the `KnowledgeBase` store, reverse
  indexes, fork(), layer views, the justification table, derivation
  DAG.
- [`../01-ein-graph/01_kb.md`](../01-ein-graph/01_kb.md) — the
  conceptual model these dataclasses implement.
- [`../03-ein-lang/`](../03-ein-lang/) — the surface syntax that
  produces these entities at load.
- [`../../inference/`](../../inference/) — P1.3 engine that *adds*
  reasoning-layer facts via rule firings.
