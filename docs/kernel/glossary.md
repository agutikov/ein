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
A record of *where a fact came from*. Four kinds: `source` (from IR),
`rule` (from a firing), `hypothesis` (speculative branch), `rejected`
(retracted). Provenance is per **derivation**, not per fact:
`Fact.provenance` is the primary justification and
`kb.justifications(fact)` returns every recorded one, so a fact is an
OR-node over AND-nodes — the proof structure is an AND/OR graph.
Frontier terminals take no alternatives: a given stays given. See
[`ir/02-data-model/01_entities.md` §3](ir/02-data-model/01_entities.md).

### Derivation DAG
The transitive closure of `rule`-kind provenance — from a derived
fact, the graph of premise facts back to the frontier terminals
(`source` / `hypothesis` / un-provenanced). `kb.derivation_dag(fact)`
follows the primary justification only; `all_justifications=True`
follows every recorded one, giving an AND/OR graph whose
per-justification premise groups are in `and_nodes` (`is_or_graph`;
`to_dot()` then draws a diamond per justification). Cycles are broken
at re-visit — the revisited fact is a node but is not re-expanded. See
[`ir/02-data-model/02_store.md` §7](ir/02-data-model/02_store.md).

### Unsat core
The frontier of given facts across a set of conflicting facts, per
the recorded derivations — the "given" premises that together derive
the conflict. Output of the *contradictions* task class (idea 03).
`kb.unsat_core(conflicting)` unions the frontier of each conflicting
fact's primary derivation; `all_justifications=True` unions over every
recorded derivation, which is monotonically *larger* — a soundness
envelope ("no explanation names a fact outside this"), not a better
explanation. Not a subset-minimal MUS; for a minimum-cardinality
answer see **Smallest contradiction frontier**. See
[`ir/02-data-model/02_store.md` §7.2](ir/02-data-model/02_store.md).

### Smallest contradiction frontier
The smallest set of given facts from which **one** recorded
contradiction follows — a minimum-cardinality AND/OR search over every
recorded derivation (provenance-based, NAF-safe, budgeted); **not** a
subset-minimal MUS. Independent of the order in which the rules
fired, and what the `k = 0` verdict reports as its unsat core (unioned
across dead commitments when no single dead one explains the unsat).
Three caveats: no proper subset is checked for satisfiability; the
alternatives searched are only the firings the saturator attempted,
capped per fact (`store.MAX_ALT_JUSTIFICATIONS`), so minimality is
relative to the rule set and the saturation strategy; and the search
is budgeted — `Explanation.exhausted` reports whether it completed,
and a truncated search is still sound. Computed by
[`frontier.smallest_contradiction_frontier`](../../ein.py/src/ein/inference/frontier.py)
over the **ATMS label** search in
[`explain.py`](../../ein.py/src/ein/inference/explain.py). See
[`inference/architecture_and_algorithms.md` §O6](inference/architecture_and_algorithms.md).

### Fork
A hypothesis branch — a `KnowledgeBase` that shares ontology and
fact-layer entities with its parent by reference, but isolates
reasoning-layer additions. See
[`ir/02-data-model/02_store.md` §5](ir/02-data-model/02_store.md).

### World
One `KnowledgeBase` instance under saturation: the root, or a fork
carrying a commitment set (`KB_C = fork(root) ∪ C`). Append-only
within a `saturate()` run; related to other worlds only by fork —
an `absent` query answered in one world is meaningless in every
other. The unit of evaluation for NAF and the search layer. See
[`inference/absent_semantics.md` §Worlds](inference/absent_semantics.md#worlds).

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

### Absent / negation-as-failure (NAF)
`(absent P)` in a `:match` — a **query** "the current world, at this
firing's fire time, holds no fact matching P" (¬∃ over P's unbound
vars; membership, not derivability). Never a ground atom: world-
relative, decided when the firing commits, never revisited after.
Not closed-world, not stratified NAF — on non-stratified rulesets
the result is defined by priority + FIFO order. Normative page:
[`inference/absent_semantics.md`](inference/absent_semantics.md);
operational how:
[`inference/README.md` §NAF](inference/README.md#naf-semantics--fire-time-re-evaluation-s15a1).

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
A truth-maintenance variant where every fact carries its **label** —
the sets of assumptions it depends on — maintained incrementally.
Ein's provenance (S1.2.3) records *justifications*, not labels: a fact
is an OR-node over the derivations the engine recorded, which is the
ATMS-style justification graph the trace renderer + hypothesis loop
read. A provenance record is not itself a label; labels are computed
on demand (see **ATMS label**). See
[`docs/lib/09-cognitive-architectures-neurosymbolic.md`](../lib/09-cognitive-architectures-neurosymbolic.md).

### ATMS label
The subset-minimal **environments** — sets of frontier facts — from
which a fact follows. Ein computes labels on demand rather than
storing them:
[`explain.py`](../../ein.py/src/ein/inference/explain.py) propagates
them by least fixpoint from the frontier upward over the AND/OR
provenance graph (`explain(kb, targets) -> Explanation`), under an
`ExplanationBudget`. Starting from empty labels that only grow makes a
**cyclic** justification graph safe by construction — symmetric and
transitive closure routinely make `(R a b)` and `(R b a)` justify each
other — since a fact can never ground itself. The engine-facing use is
the **Smallest contradiction frontier**.

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
