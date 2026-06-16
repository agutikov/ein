# Glossary

Definitions for terms used throughout the kernel documentation.
Entries are grouped roughly by topic; cross-references point at the
authoritative discussion.

> **Scope.** This is a *kernel-internal* glossary — terms Ein
> uses with a specific technical meaning. Broader external concepts
> (LLM, CSP, SMT solver, …) live in
> [`docs/lib/`](../lib/).

---

## Graph model

### Atom
A *name* — a lexical token that identifies a node (`Norwegian`,
`House-1`, `Red`, `co-located`, `rule`, `not`, `T`). The atom is the
name; the node it denotes is the thing named. Two occurrences of the
same atom denote the same node. Distinguish from **Object** (the node
itself). See
[`ir/01-ein-graph/03_ein_model.md` §2](ir/01-ein-graph/03_ein_model.md).

### Object
A graph node *named by an atom* and representing a concrete entity in
a puzzle (Norwegian, House-1, Red) — a vertex with no outbound arrows
and in-arrows from facts. Drawn as an ellipse in the compact view.
The atom is its name; the object is the node. See
[`ir/01-ein-graph/01_kb.md` §1](ir/01-ein-graph/01_kb.md).

### Type
A graph node classifying objects (Nationality, House, Color). Drawn
as a box. Types are themselves graph nodes — they participate in
relations like any object. See [`ir/01-ein-graph/01_kb.md` §1](ir/01-ein-graph/01_kb.md).

### Relation
Three overlapping uses worth disambiguating:
1. A **relation declaration** — a graph node naming a named
   predicate (`co-located`, `is-a`). First-class: relations are
   nodes, not edge labels.
2. A **relation instance** = a **fact** — the proposition that this
   relation holds between specific arguments.
3. The colloquial "any multi-name node `(A B C)`" — any node with
   two or more outgoing slot-edges.
See [`ir/01-ein-graph/01_kb.md` §1](ir/01-ein-graph/01_kb.md).

### Fact
A hyperedge node — an instance of a relation applied to specific
arguments. The unit of *proposition* in the KB. Drawn as an octagon
in the detailed (Levi-bipartite) view; as a single labelled arrow
in the compact view for binary facts. See
[`ir/01-ein-graph/01_kb.md` §1](ir/01-ein-graph/01_kb.md).

### Rule
A graph rewriting rule with `:match` LHS / `:assert` RHS / optional
`:where` guard. Three families (T1/T2/T3) by what the LHS quantifies
over. See [`ir/01-ein-graph/02_rules.md`](ir/01-ein-graph/02_rules.md).

### Hyperedge
An edge with arity ≠ 2 — connects three or more participants. DOT
has no native hyperedges; we encode them Levi-bipartite (one octagon
node per fact + n slot-labelled edges to participants). See
[`ir/01-ein-graph/01_kb.md` §2.2](ir/01-ein-graph/01_kb.md).

### Levi-bipartite
The canonical encoding of a hypergraph as an ordinary graph: every
hyperedge becomes a node, with edges to each of its participants.
Named after Friedrich Wilhelm Levi. In Ein, **the** canonical
form of every fact, regardless of arity. See
[`ir/01-ein-graph/01_kb.md` §2.2](ir/01-ein-graph/01_kb.md).

### Layer (ontology / fact / reasoning)
Three knowledge populations the KB stratifies facts into. The layer
is **per fact** (P1.7c — no block wrappers): an explicit `:layer` wins,
else it is derived from provenance.
- **ontology** — implicit assumptions (schema, instance enumeration,
  rule-application meta-facts). Derived when a fact carries neither
  `:source` nor `:rule`/`:using`.
- **fact** — explicit puzzle statements, each with a `:source`
  annotation (which derives this layer).
- **reasoning** — derived facts produced by rule firings or
  hypotheses; carry `:rule`/`:using` (which derives this layer).
See [`ir/01-ein-graph/01_kb.md` §3](ir/01-ein-graph/01_kb.md).

### Provenance
A per-fact record of *where the fact came from*. Four kinds: `source`
(from IR), `rule` (from a firing), `hypothesis` (speculative branch),
`rejected` (retracted). See
[`ir/02-data-model/01_entities.md` §3](ir/02-data-model/01_entities.md).

### Derivation DAG
The transitive closure of `rule`-kind provenance — from a derived
fact, the directed acyclic graph of premise facts back to source-
kind terminals. See [`ir/02-data-model/02_store.md` §7](ir/02-data-model/02_store.md).

### Unsat core
The minimal source-kind frontier across a set of conflicting facts —
the "given" premises that, together, derive the conflict. Output of
the *contradictions* task class (idea 03). See
[`ir/02-data-model/02_store.md` §7.2](ir/02-data-model/02_store.md).

### Fork
A hypothesis branch — a `KnowledgeBase` that shares ontology and
fact-layer entities with its parent by reference, but isolates
reasoning-layer additions. See
[`ir/02-data-model/02_store.md` §5](ir/02-data-model/02_store.md).

### Equality class
A union-find class of objects the engine has concluded are *the same*.
M1 ships the union-find but doesn't yet act on it automatically;
reserved for an e-graph promotion (F4 Q30).

---

## Rule families

### T1 rule (first-order)
A rule whose LHS quantifies only over object variables; relation
names are literal. Cheap to match (one indexed lookup per relation).
See [`ir/01-ein-graph/02_rules.md` §2.1](ir/01-ein-graph/02_rules.md).

### T2 rule (relation-polymorphic / higher-order)
A rule with at least one relation variable in its LHS or RHS. Fires
only when activated by a **property fact** (a fact whose head is the
rule's name). Most of M1's rule library is T2. See
[`ir/01-ein-graph/02_rules.md` §2.2](ir/01-ein-graph/02_rules.md).

### T3 rule (structural / aggregate)
A rule whose LHS uses an aggregate predicate (count, uniqueness,
position) — a property of the whole graph, not a local subgraph
match. Bridges graph rewriting and CSP arc-consistency. See
[`ir/01-ein-graph/02_rules.md` §2.3](ir/01-ein-graph/02_rules.md).

### Property fact
A fact whose head matches a rule's name; its arguments supply the
rule's parameter bindings. Example: `(symmetric co-located)` is a
property fact that activates the `symmetric` rule on the `co-located`
relation. See [`ir/01-ein-graph/02_rules.md` §2.2](ir/01-ein-graph/02_rules.md).

### Kernel meta-primitive
A shape-pinned reserved word in the IR grammar: `instance`, `not`,
`and`, `or`, `neq`, `=`. Wrong arity is a parse error, not a
validator error. See
[`ir/03-ein-lang/01_grammar.md`](ir/03-ein-lang/01_grammar.md).

### Saturation
The fixed-point of rule firing — applying every rule until no new
fact is produced. The default M1 strategy is lazy: saturate before
branching. See [`inference/`](inference/) (P1.3 stub).

---

## Algebraic properties of relations

These are predicates over a relation; they activate or constrain T2
rules. See [F4 Q34](../../plans/followups/f4_cross_cutting.md) for
the full 2⁷ cartesian product discussion.

### Symmetric
`R(a,b) ⇒ R(b,a)`. Example: `co-located`, `next-to`.

### Transitive
`R(a,b) ∧ R(b,c) ⇒ R(a,c)`. Example: `co-located`, `is-a`,
`ancestor-of`.

### Reflexive
`R(x,x)` holds for every `x` in the relation's domain. Example:
`co-located` mathematically (every node is co-located with itself);
Ein's M1 doesn't materialise the self-edges — see
[zebra.ein §Future-work](../../examples/zebra.ein).

### Asymmetric
`R(a,b) ⇒ ¬R(b,a)` for `a ≠ b`. Example: `is-a` (Norwegian is-a
Nationality, but Nationality is-not-a Norwegian), `right-of`.

### Antisymmetric
`R(a,b) ∧ R(b,a) ⇒ a = b`. Strictly weaker than asymmetric;
the canonical partial-order property.

### Irreflexive
`¬R(x,x)` for every `x`. Used with strict orders and explicit `≠`.

---

## Categorical / theoretical

### Homoiconic
A language whose source code is itself a value in the language —
Lisp's `(list 1 2 3)` is both syntax and a list literal. ein-lang is
homoiconic: rules, facts, and traces all share one S-expression
grammar, so the engine can read its own traces. See
[`ir/03-ein-lang/05_inspirations.md`](ir/03-ein-lang/05_inspirations.md).

### DPO (double-pushout)
A categorical formulation of graph rewriting where a rule is a span
`L ← K → R` (matched subgraph K preserved, L-deletions and R-additions
happen via pushouts). Ein's pattern language is positive
conjunctive (no deletions), so the K = L case applies; the DPO
machinery is reserved for the F1 categorical-formulation followup.
See [`ir/03-ein-lang/04_dot_rendering.md` §Rule rendering mode (b)](ir/03-ein-lang/04_dot_rendering.md).

### E-graph (equality graph)
A data structure that maintains equivalence classes of terms with
shared sub-terms compressed — the canonical substrate for *equality
saturation*. Ein ships a union-find placeholder; full e-graph
is F4 Q30. See [`docs/lib/06-graphs-rewrite-systems.md`](../lib/06-graphs-rewrite-systems.md).

### Equality saturation
Apply all known equality rewrites without committing to a normal
form; an e-graph compresses redundant terms. Powerful for
verification and superoptimisation. F4 promotion target. See
[`docs/lib/06-graphs-rewrite-systems.md`](../lib/06-graphs-rewrite-systems.md).

### ATMS (Assumption-based Truth Maintenance System)
A truth-maintenance variant where every fact carries the set of
assumptions it depends on. Ein's per-fact provenance (S1.2.3)
is the ATMS-style substrate; the trace renderer + hypothesis loop
read it. See [`docs/lib/09-cognitive-architectures-neurosymbolic.md`](../lib/09-cognitive-architectures-neurosymbolic.md).

### Functoriality
A categorical property: a rule R is *functorial* in a relation P if R
is preserved under morphisms in P's category. Some rules are
functorial (symmetric closure preservation); others aren't
(transitivity-of-subtype is NOT functorial along an instance-of
inclusion). See [F4 Q36](../../plans/followups/f4_cross_cutting.md).

---

## Process / project terms

### Encoding-agnostic
A piece of code that works equally well over `zebra.ein` (classic
`(type …)` / `(instance …)` declarations) and `zebra2.ein` (unified
`is-a` fact graph), without committing to either encoding. P1.7
resolved the encoding question — `is-a` is canonical
([S1.7.6](../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.6_kernel_minimization.md)),
and S1.7.23 removed the kernel's type/instance entity-view entirely
(the `logical_types` / `logical_instances` bridge is gone), so both
forms are just facts and any type projection is a user-space rule.

### Open-world
The KB loader tolerates references to undeclared types and relations
— they auto-vivify with a `declared=False` flag rather than fail. See
[`ir/02-data-model/01_entities.md` §1.3](ir/02-data-model/01_entities.md).

### Three task classes
Solve / find-gaps / find-contradictions — the three modes a
constraint engine should support. From
[`docs/ideas/03-three-task-classes.md`](../../plans/ideas/03-three-task-classes.md).

### Trace fidelity
The acceptance criterion that every reasoning step in the engine's
output has a recoverable, named cause — no opaque Python firings, no
"because solver said so". The M1 acceptance gate. See
[`docs/ideas/08-human-style-deductive-trace.md`](../../plans/ideas/08-human-style-deductive-trace.md).
