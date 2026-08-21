# Data model — entities

How the graph from [`../01-ein-graph/`](../01-ein-graph/) is held in
memory. Frozen Python dataclasses with identity by name, attached to
the owning :class:`KnowledgeBase` via a `_kb` back-pointer for
cross-reference lookups.

**Source of truth:**
[`ein-core/entities.rs`](../../../../ein.rs/crates/ein-core/src/entities.rs).
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
> as the heads/args of the puzzle's own membership facts (`is-a`, or
> whatever relation it declares); the participation of every name is captured by
> the lightweight `NameRef` index (§1.4). A puzzle that wants a
> named-type projection computes it with a user-space ein-lang rule
> over its own inheritance relation. See
> S1.7.23.

Metadata fields (`loc`, `provenance`, `_kb`, `raw`) are
**excluded** from identity — two facts with the same `(rel, args)`
but different provenance are *the same fact* for dedup
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

**Relations are objects — with one qualification.** Auto-vivification is
what makes "a relation is just another node" operational rather than
aspirational: declarations are mirrored as ordinary `(relation R A B)`
facts, property tags take relation names as *arguments*, and any
undeclared fact head gets a `Relation` entity so it can participate in
the cross-reference indexes. The single place the kernel treats a
relation name as *not* an object is `NameRef.category` (§1.4): hypgen's
candidate-object pool keeps only `category == "object"` names, so the
blind enumerator never guesses *about* a relation. That is a
**tractability device, not a semantic stratification** — nothing stops a
rule from asserting `(is-a co-located EquivalenceRelation)`, and nothing
in the store treats such a fact differently. Since S1.22.4 each
declaration also stores the arity-1 fact `(relation R)`, so "R is a
relation" is itself an ordinary matchable proposition about a relation
node. See
[`../03-ein-lang/08_self_describing.md`](../03-ein-lang/08_self_describing.md)
for what userspace builds on top of it.

The `signature` holds the argument-position type **names** (opaque
atoms — S1.7.23 keeps no `Type` entities to resolve them to; hypgen
uses them only as object-exclusion metadata). It may be **empty** — a
bare `(relation R)` declares a relation node with no declared arg types,
and is thereby not a hypothesis target (S1.22.4). It is **not** a kernel
type annotation: rules read the atoms as types, the kernel reads only
the tuple's shape. Definitive account of that split:
[`../03-ein-lang/01_grammar.md` §what the signature means](../03-ein-lang/01_grammar.md#what-the-signature-means--userspace-types-kernel-structure).

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
)
```

The pattern object is **structural-only** for M1 — the matching
semantics (binding, unification, backtracking) lives in P1.3 with
the inference engine. The Pattern serves as the type-checker's view
of a clause and as the data the `Rule.relations` cross-reference
walks. (`type_names` / `has_instance_pattern` are **gone** — S1.7.23
removed their `Rule.types` / `_rules_by_type` consumers with the `Type`
entity, and S1.22.1 removed the now-dead fields.)

A small example — the LHS `(and (?rel ?a ?b) (?rel ?b ?c))` of the
`transitive` rule:

| field                  | value                          |
|------------------------|--------------------------------|
| `variables`            | `('rel', 'a', 'b', 'c')`       |
| `relation_names`       | `()` — `?rel` is a Var, not literal |
| `type_names`           | `()` — no instance premise     |
| `has_instance_pattern` | `False`                        |

Contrast with `type-exclusivity`'s LHS `(and (is-a ?a ?T) (is-a ?b ?T))`:

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
[`explain.rs`](../../../../ein.rs/crates/ein-infer/src/explain.rs)
searches the AND/OR graph for a minimum-cardinality frontier under an
explicit budget — **not** a subset-minimal MUS, and minimal only over
the derivations the saturator actually recorded.

---

## 4. Origin predicates — `Fact.is_given` / `Fact.is_derived`

```python
f.is_given    # provenance.kind == 'source' and provenance.source is not None
f.is_derived  # provenance is not None and provenance.kind != 'source'
# neither → a background assumption (schema, is-a enumeration, tag)
```

The same `(rel, args)` exists at most once in the KB; its `provenance`
records where it came from, and these two read that. They are for
*presentation* — the engine treats every fact alike.

> **S1.22.1b** replaced a stored `Layer` enum
> (`ONTOLOGY`/`FACT`/`REASONING`) with these. The enum was a
> denormalised copy of `Provenance` — measured over every
> `examples/**/*.ein`, the only facts it disagreed with were the ones
> carrying an explicit `:layer` override — and the contradiction
> detector read it as an epistemic guard, which silently accepted a
> puzzle whose own clues contradicted each other. `Fact.layer` and
> `:layer` are both gone; the loader rejects the annotation.

---

## 5. Entity attachment — the back-pointer, and why it is gone

Entities are **immutable**, which is what makes them hashable and their
identity stable. Nothing about the KB that owns them is part of that identity:
two entities of the same kind with the same name (or `(rel, args)` for facts)
are equal across KBs, which is what makes sharing them by reference across a
fork sound.

> **Historical.** The Python implementation reached the owning KB through a
> `_kb` back-pointer wired onto each frozen dataclass after construction and
> excluded from equality, so `Relation.facts` could answer with no argument —
> and answered *for the root* when asked on a fork, a caveat the store page
> had to document. ein.rs passes the KB explicitly instead, so there is
> nothing to attach, detach, or exclude, and the caveat has no subject. See
> [`03_implementation.md` §2](03_implementation.md).

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
provenance, and any owning-KB reference never affect equality. **Facts are
also totally *ordered*** wherever the engine has to sort them, by a rule that
is stated rather than inherited:
[`defined_behaviour.md` §2.1](../../defined_behaviour.md). For
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
  indexes, fork(), the fact view, the justification table, derivation
  DAG.
- [`../01-ein-graph/01_kb.md`](../01-ein-graph/01_kb.md) — the
  conceptual model these dataclasses implement.
- [`../03-ein-lang/`](../03-ein-lang/) — the surface syntax that
  produces these entities at load.
- [`../../inference/`](../../inference/) — P1.3 engine that *adds*
  derived facts via rule firings.
