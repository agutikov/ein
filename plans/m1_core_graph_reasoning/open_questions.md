# M1 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

## Index

| Q   | Title                                                                                | Resolved in        |
|-----|--------------------------------------------------------------------------------------|--------------------|
| Q4  | Rule presentation language — Python functions, graph-rewrite DSL, or Horn clauses?    | P1.3 S1.3.1        |
| Q5  | What "enough" means for the rule set                                                  | P1.3 S1.3.2 + P1.7 |
| Q6  | Symmetry breaking — before or after trace generation?                                 | P1.5 S1.5.3        |
| Q15 | Forward-chaining queue vs priority by cheapness vs random rule ordering               | P1.3 S1.3.3        |
| Q16 | Are rules universal, or per-puzzle?                                                   | P1.3 + P1.7        |
| Q17 | Spatial-relation formalisation — Allen, RCC, or ad-hoc 1-D interval?                  | S1.1.1 (2026-05-18) — declarative square rule |
| Q18 | Provenance granularity — per-edge, per-step, or per-derivation-DAG?                   | P1.2 S1.2.3        |
| Q19 | Hypothesis branching — eager (every choice) vs lazy (only when saturation stalls)?    | P1.5 S1.5.1        |
| Q20 | Trace reordering — engine order, planner-pass reorder, or human-template fitting?     | P1.6 S1.6.4        |
| Q21 | IR ↔ DOT structural isomorphism — bidirectional, layout-free                          | S1.1.1 T1.1.1.6    |
| Q22 | Ontology IR — three sub-heads (relation-decls / relation-defs / types-and-objects)?   | P1.1 (revisit) or P1.2 |
| Q23 | What carries an explicit type slot — only `Instance`, or also vars / relations?       | P1.2 S1.2.1 (decided) |
| Q24 | `:where` clause in `sibling-exclusive` — what does it mean, and should it stay?       | P1.3 S1.3.1        |
| Q25 | Cardinality + ordinality rules with vars — IR shape, graph representation             | P1.3 S1.3.2        |
| Q26 | Compound / virtual node kinds for higher-order rules (sets, projections, groups, top/bottom) | P1.2 leaves the seam; concrete kinds parked for followups |
| Q27 | Relation declaration body form — bundle properties inside `(relation R (T T) (transitive …))` vs separate property-application facts? | M1 ships form (b); form (a) parked as future sugar |
| Q28 | Empty parens `()` — placeholder, ⊥, ⊤, or forbidden?                  | M1 grammar parses as `@empty` no-op; engine semantics TBD |
| Q29 | Pattern: walker vs cached executable program — compile unit?           | P1.3 S1.3.1 (resolved 2026-05-20) — option B, per-(rule, activator-binding) compile |
| Q30 | Universal rule library + import mechanism — does `examples/rules.ein` exist? | Deferred to P1.8 (2026-05-20); P1.3 assumes inline rules per puzzle |
| Q31 | `:why` template substitution language — `{?var}` or Python `{var}`?    | P1.3 S1.3.1 (resolved 2026-05-20) — `{?var}` (zebra.ein convention) |
| Q32 | `:where` semantics — sugar, top-level, or drop entirely?               | P1.3 S1.3.1 (resolved 2026-05-20) — drop; ship predicate registry |
| Q33 | Predicate primitives — minimal set for M1                              | P1.3 S1.3.1 (resolved 2026-05-20) — `eq` + `neq` only; rest → followups |
| Q39 | Module path — `src/ein_bot/inference/` vs `src/ein_bot/rules/inference/`? | P1.3 (resolved 2026-05-20) — flat `inference/` |
| Q40 | `(hypothesis ?h)` / `(contradiction-under ?h)` rule premises — map to shipped `Provenance`? | P1.3 S1.3.2 + P1.5 (resolved 2026-05-20) — Option A: synthetic facts + `Fact.args` widening |
| Q41 | Rule priority — scale and placement                                    | P1.3 S1.3.3 (resolved 2026-05-20) — 100/200/300/900 banded, per-rule, lower-first |
| Q42 | P1.3 scope — path (a) narrow vs (b) all 10 with deferred                | P1.3 (resolved 2026-05-20 by Q33 cascade) — path (a), 6 rules |

---

## Q4 — Rule presentation language

Per [idea 06 §Open sub-questions point 1](../../docs/ideas/06-inference-rules-completeness.md).

**Options:**

- **A** — Python predicate over a `Graph` view. Easy; awful for
  trace generation (the rule "fires" with no structured premise list).
- **B** — Graph-rewrite DSL (LHS pattern → RHS pattern, à la DPO).
  Structural, formal, slightly heavy. Matches
  [idea 07 Reading C](../../docs/ideas/07-categorical-formulation.md).
- **C** — Horn-clause / Datalog encoding. Familiar declarative;
  loses the "graph rewrite" intuition.

**Working answer**: B — a small DSL where each rule is
`(rule <name> :match <pattern> :assert <conclusion> :why <reason-template>)`.
The original sketch carved out a Python-callable fallback for "cases
where a pattern can't express what we need", citing the spatial
constraint of P1.4 as the trigger; the declarative square rule
(see [Q17](#q17--spatial-relation-formalisation)) removes that
trigger. Keep the Python fallback latent as an *escape hatch* for
future problem classes, but Zebra ships without using it.
Locked in P1.3 S1.3.1.

## Q5 — What "enough" means

Per [idea 06 §What "enough" should mean](../../docs/ideas/06-inference-rules-completeness.md):
four non-equivalent answers — functional, propagation, explanation,
domain.

**Working answer**: M1's target is **explanation-complete on Zebra**
(every step of the human walkthrough in
[idea 08](../../docs/ideas/08-human-style-deductive-trace.md) maps to
a named rule firing). Propagation-completeness is desirable but not
required to ship. Domain-completeness is out of scope; revisit per
problem class.

## Q6 — Symmetry breaking

Per [idea 08 §Open questions point 3](../../docs/ideas/08-human-style-deductive-trace.md).
Two paths initially considered:
(a) pre-trace — engine enforces a canonicalisation (e.g. house
numbering); (b) post-trace — engine explores both, trace planner
annotates "doesn't matter which way numbered".

**Resolved 2026-05-21** (user direction during S1.5.0 review):
**neither (a) nor (b)** — the question doesn't belong to the
engine. By the time a `.ein` file declares `House-1`, …, `House-5`
plus `(right-of House-2 House-1)` etc., the orientation is *already
committed* by construction. The engine sees unique named instances
with one spatial layout; there is no second orientation to explore
or canonicalise.

The "WLOG numbering" remark in idea 08's target trace documents
the **ontology writer's** choice, not a derivation step. M1's
hypothesis loop (P1.5) does no symmetry handling. M2's NL → IR
pipeline inherits the decision: the NL parser maps positional
phrases ("the first house", "the leftmost") to named House
instances by adopting a convention.

S1.5.3 (the original "symmetry detection placeholder" stage) is
**dropped** from M1 — see [S1.5.0 review §D](../p1.5_hypothesis_loop/s1.5.0_review.md#d-symmetry--not-an-engine-concern-s153-dropped).

Future puzzles with *latent* symmetry that can't be committed at
ontology-write time (e.g. graph automorphism on an abstract group)
remain a research direction — beyond M1's scope.

## Q15 — Rule ordering

Per [idea 06 §Open sub-questions point 3](../../docs/ideas/06-inference-rules-completeness.md).

**Working answer**: forward-chaining queue with a static priority
class per rule (cheap propagation rules fire before expensive
search-like ones); revisit only if the trace shows obviously bad
ordering. Decided in P1.3 S1.3.3.

## Q16 — Universal vs per-puzzle rules

Per [idea 06 §Open sub-questions point 4](../../docs/ideas/06-inference-rules-completeness.md).

**Working answer**: ship with a *universal core* (composition,
equality, exclusivity, exhaustion, hypothesis-contradiction,
arc-consistency, cardinality, forced-unique). Problem-specific
rules — including the *square rule* (formerly the "spatial bundle"
loaded by Python on `:spatial-via`) — are now ordinary rule
declarations + property-application facts inside the puzzle's `.ein`
file. No bundle-loading mechanism; the IR is the only configuration
surface. New problem classes write their own rules in their own
files, same as Zebra. Decided in P1.3 + P1.7.

The `:spatial-via SpatialAttribute` kw-pair was the original
mechanism for the spatial bundle; resolved on 2026-05-18 by the
declarative answer to [Q17](#q17--spatial-relation-formalisation).

## Q17 — Spatial-relation formalisation

The PoC's 2021 open question
([idea 05 §Open question recorded in the README](../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#open-question-recorded-in-the-readme)).

**Options:**

- Allen interval algebra (13 named relations between intervals).
- RCC (region connection calculus).
- Ad-hoc 1-D position lattice with `pos(x) ∈ {1..N}` and integer
  arithmetic on `pos`.
- **Declarative graph-only:** materialise the spatial structure as
  pairwise `(right-of House_{i+1} House_i)` facts; reuse the PoC's
  *square rule* (now `square-fwd` / `square-bwd` rules + property
  facts) to project spatial relations across `co-located`. No
  integer arithmetic; no position lattice; no `:spatial-via` Python
  hardcode.

**Working answer**: the declarative graph-only formulation. Decided
2026-05-18 in this conversation; locks the answer for Zebra. Pulls
Q17 forward from P1.4 to S1.1.1.

Implementation already shipped in `examples/zebra.ein`:

- Drop the `SpatialAttribute` type and the `:spatial-via` kw-pair on
  `right-of` / `next-to`.
- Add `square-fwd` and `square-bwd` rules (gated by
  `(square-fwd ?R)` / `(square-bwd ?R)` rule-application facts on
  `right-of` and `next-to`).
- Materialise condition (1) "five houses in a row" as four
  `(right-of House_{i+1} House_i)` facts; `(implies right-of next-to)`
  + `(symmetric next-to)` derive the eight next-to pairs.

Trade-offs:

- *For:* IR-native, no Python fallback. The trace renderer reads each
  spatial step as a square-rule firing — fully visible to
  [idea 08](../../docs/ideas/08-human-style-deductive-trace.md).
  M3's SMT translation maps `square-fwd` / `square-bwd` to two
  universally-quantified clauses with no integer theory required.
- *Against:* problems with continuous or multi-dimensional spatial
  structure (Allen, RCC) will need a different formulation. Revisit
  for those problem classes; for Zebra-class puzzles the graph-only
  answer is sufficient.

P1.4 S1.4.2 keeps the *open question for richer problem classes*
(when does the graph-only answer break?) but no longer ships an
integer-arithmetic spatial encoding for Zebra.

## Q18 — Provenance granularity

Per [idea 03 §What "contradictions" specifically means](../../docs/ideas/03-three-task-classes.md)
and [idea 08](../../docs/ideas/08-human-style-deductive-trace.md).

**Working answer**: per-derived-edge provenance — each edge carries
a tuple `(rule, premise_edges, source_or_rule_id)`. The full
derivation DAG is recoverable by transitive closure.

**Decided** in P1.2 S1.2.3 (2026-05-19): implemented as the
`Provenance` dataclass (`kind in {'source', 'rule', 'hypothesis',
'rejected'}` + the discriminator-specific fields). Each `Fact`
carries an optional `provenance: Provenance | None`. The full DAG
is built by `KnowledgeBase.derivation_dag(fact)` via BFS over
`provenance.premises_raw`. `KnowledgeBase.unsat_core(facts)` returns
the minimal source-kind frontier — the input to the *contradictions*
task class.

IR-level round-trip of `:using` is deferred (the current grammar
doesn't accept headless lists of compact-form fact refs) — see
S1.2.3 T1.2.3.4.

## Q19 — Hypothesis branching strategy

PoC's algorithm branches whenever `solved(s)` returns false. Modern
CDCL solvers branch only when propagation saturates.

**Working answer**: lazy branching — saturate first with all
propagation rules; branch only when no rule fires and the graph is
still incomplete. Mirrors human walkthroughs. Decided in P1.5 S1.5.1.

## Q20 — Trace reordering

Per [idea 08 §Why this is a hard problem — Ordering](../../docs/ideas/08-human-style-deductive-trace.md).

**Working answer**: emit engine-order trace + a `--reorder` flag
that runs a planner pass clustering by entity. Defer full
human-template fitting to M2 (where an LLM surface generator is
available). Decided in P1.6 S1.6.4.

## Q21 — IR ↔ DOT structural isomorphism

Surfaced in S1.1.1 design (2026-05-18). The EBNF ↔ railroad-diagram
move: two surface representations of one structure. The IR is the
textual view; a documented subset of DOT is the graphical view.
Meaning lives in their shared structure, not in either surface.

**Options:**

- **A** — One-way: `ir → dot` only, DOT is for human inspection.
- **B** — One-way with a documented schema: `ir → dot` is total and
  deterministic in structure, `dot → ir` left aspirational.
- **C** — Bidirectional: both `ir → dot` and `dot → ir` are
  deliverables; the project's DOT *subset* and the IR are
  inter-convertible up to layout.

**Working answer**: C. `ir → dot` is mandatory and blocks S1.1.2;
`dot → ir` is required but does not block P1.1 — it lands in P1.2
alongside the typed-hypergraph data model. Only graph *structure* is
fixed by the schema; layout (positions, rank, unspecified style
choices) is free — `random_layout` is permitted.

Pulls forward:

- Each kernel form has a fixed DOT shape — §Rendering in `docs/ir.md`
  (S1.1.1 T1.1.1.6).
- Hyperedges render Levi/bipartite — anchors
  [Q1](../open_questions.md#q1)'s typed-hypergraph + equality-class-ID
  answer visually.
- Rule rendering: 3 modes (side-by-side / DPO span / overlay),
  configurable; `rules.ein` defaults to side-by-side, traces default
  to overlay.
- Trace rendering: 3 views (per-step / aggregate / derivation-DAG),
  configurable; default per-step (matches M1 acceptance §2).
- Branch rendering: tree-of-states for the search-tree view,
  sub-clusters per branch for per-state snapshots.

Decided in S1.1.1 T1.1.1.6.

## Q22 — Ontology IR sub-head split

Currently one `(ontology …)` block holds: type declarations,
instance declarations, relation signatures, rule-application facts
(`(symmetric R)` etc.), and structural facts (e.g. the four
`(right-of House_{i+1} House_i)` in zebra.ein). The KB loader
(P1.2 S1.2.1 T1.2.1.4) already routes these to different slots
internally.

The user proposed (2026-05-18) splitting the `(ontology …)` block
*syntactically* into three sub-heads:

```lisp
(ontology
  (relations
    (relation co-located (Attribute Attribute))
    (relation right-of   (Attribute Attribute))
    …)
  (definitions               ;; rules + rule-application facts
    (symmetric co-located)
    (transitive co-located)
    (square-fwd right-of)
    …)
  (types-and-objects
    (type Attribute) …
    (instance Norwegian Nationality) …
    (instance House-1 House) …))
```

Justification (user's): "rules are separate head — good. Ontology
can be split into: relations declarations, relations definitions
(via rules), types looks secondary as produced by a relation, so
types and objects go into 3rd separate ontology sub-head."

**Working answer:** *parked*. The data model is already split
internally; the syntactic re-org is a cosmetic IR change that does
not affect P1.2's data-model decisions. Revisit during P1.7
authoring when the larger Zebra IR is being read by multiple
people — readability may justify the split there.

**Counter-argument:** the current flat (ontology …) block reads
fine for Zebra-scale puzzles (one page). Premature factoring; wait
for an instance where it bites.

## Q23 — What carries an explicit type slot

Direct question raised 2026-05-18: which entities in the data model
have a `type` slot?

**Working answer** (locked in P1.2 S1.2.1):
- `Instance.type` — yes, exactly one. Every instance belongs to one
  declared type (no multi-typing in M1).
- `Var` — *no* explicit slot. Variables are typed *structurally* by
  the premises that bind them (`(is-a ?var T)` etc.); see
  [F4 Q35](../followups/f4_cross_cutting.md#variable-typing-via-match-is-a-var-type-q35).
- `Relation` — *no* explicit slot, but has a `signature: tuple[Type,
  …]` that says what types its arguments can be. Not the same as
  "the relation has a type" in the OOP sense.
- `Type` / `Rule` / `Fact` — no type slot. (Types form a hierarchy
  via `parent`; rules and facts are not themselves typed.)

This consciously aligns with the unified `is-a` model of
zebra2.ein: types are graph nodes, "having a type" is "being on the
receiving end of an `is-a` / `instance-of` edge". The slot exists
only as a fast-access cache on `Instance`.

**Multi-typing** (user's question): out of M1 scope. If an instance
must be of two types, declare an intermediate intersection type or
use additional `(is-a)` facts in the unified model. Revisit if the
need is real.

## Q24 — `:where` clause in `sibling-exclusive`

Surfaced 2026-05-18. The clause appears in some early rule sketches
but is *not* documented in the rule DSL spec (P1.3 S1.3.1). The
options:

- **A** — `:where` is a top-level rule clause holding side
  conditions evaluated against the matcher's bindings (similar to
  Prolog cut + guard). Currently used inline inside `(and …)`
  patterns; pulling it to top-level is a cosmetic move.
- **B** — `:where` is the same as `(and … :where p …)` inline form,
  just a sugar; nothing to add to the DSL.
- **C** — delete the keyword everywhere; require all side
  conditions to be ordinary pattern premises.

**Working answer:** B — it's syntactic sugar for the inline
`:where` predicate inside an `(and …)`. Document in P1.3 S1.3.1
DSL spec; do not promote to top-level keyword. Locks when the DSL
is written down.

## Q25 — Cardinality and ordinality rules with vars

The rule library has `global-cardinality` (`exactly-n X V N`,
T1.3.2.8) — counting facts with a value. The user (2026-05-18)
asked whether **ordinality rules** (less-than, position-of, between)
should be first-class IR forms or derived from `(rel ?r ?a ?b)`
patterns over a chosen relation.

**Options:**

- **A** — first-class `(less-than ?a ?b)` form + a dedicated rule
  family. Tight coupling to the position lattice (P1.4 S1.4.2).
- **B** — derived from `right-of` / `before` etc. via the existing
  square + transitivity rules. No new IR forms; everything reduces
  to the generic relation machinery.
- **C** — half-and-half: keep ordinality as a *named property* (a
  rule-application fact `(ordinal ?R)`) that activates a small
  bundle of derived rules.

**Working answer:** B for Zebra. Ordinality questions ("is house 3
to the right of house 1?") reduce to transitive closure over
`right-of`. C may be needed for Sudoku-style puzzles where ordinal
position matters arithmetically. Revisit in P1.3 S1.3.2 with the
demo-problem set — if the cardinality/ordinality demos need an
ordinal-specific rule, promote to a named family.

## Q26 — Compound / virtual node kinds for higher-order rules

User direction (2026-05-18, before P1.2 begins): the data model
should support not only the base kinds (Type, Instance, Relation,
Rule, Fact) but eventually a class of *compound* / *virtual* node
kinds that higher-order rules will need. Examples:

- **Relation-set-of-object** — the set of all relations involving a
  given object.
- **Slot-projection** — all objects connected to argument-slot N of
  a given relation (e.g. "all 'subjects' of `is-a`").
- **Top / bottom of relation subgraph** — the maximal / minimal
  element under a relation (when applicable, e.g. `is-a` lattice).
- **Criterion-selected group** — objects matching a graph pattern
  predicate (e.g. zebra.ein's three middle houses = those with
  exactly two `next-to` neighbours).
- "*Who knows what else…*" — open class.

These are not pre-stored entities; they are *computed-on-demand
subgraphs* materialised when a higher-order rule binds a parameter
to one.

**Options:**

- **A** — Implement nothing in M1. Concrete kinds land as needed in
  followups; P1.2 just doesn't foreclose them.
- **B** — Reserve a `VirtualNode` base class / kind tag in the data
  model. M1 doesn't subclass it; the type hierarchy exists for
  later extension.
- **C** — Implement one (say, slot-projection) as a worked example
  in M1; defer the rest.

**Working answer:** A — *don't foreclose*. P1.2 ships the base
entity kinds (Type, Instance, Relation, Rule, Fact); the store and
the index design must accept new kinds without rework, but no
specific kind is implemented in M1. When the first higher-order
rule actually *needs* one (likely in F4 promotion work), promote
the kind to a concrete entity at that time. Documented for design
review; concrete promotion happens followup-by-followup.

Connection: [feedback memory `graph-canonical`](.) — graph is the
canonical data model, entity API is a derived view; this open
question is the *future-proofing* corollary.

## Q27 — Relation body form

User direction (2026-05-19): two equivalent forms for declaring a
relation along with its algebraic properties:

```lisp
;; Form (a) — body bundled into the declaration
(relation is-a (A B) (transitive asymmetric sibling-exclusive))

;; Form (b) — declaration + separate property-application facts (M1)
(relation is-a (T T))
(transitive        is-a)
(asymmetric        is-a)
(sibling-exclusive is-a)
```

Semantically equivalent. Which form, and why? See
[`docs/kernel/ir/01-ein-graph/03_ein_model.md` §7.2](../../docs/kernel/ir/01-ein-graph/03_ein_model.md)
for the full trade-off table.

**Working answer:** ship form **(b)** in M1 (already the de-facto
form in zebra.ein / zebra2.ein). Form (a) is admitted as a *future
syntactic sugar* that desugars into form (b) at load time; it does
NOT need first-class engine support. The desugaring lands when
authoring readability becomes a pain point (probably P1.7+ when
non-zebra puzzles arrive).

Trade-off rationale (from `03_ein_model.md` §7.2):
- Form (b) makes property-application facts *first-class graph
  nodes*, so rules (T2 + F5) can match and modify them.
- Form (a) is more readable in the small but hides the property
  facts inside a nested form; rules wanting to match them would need
  unpacking.
- Choosing (b) aligns with the "everything is a node" principle
  ([feedback memory `graph-canonical`](.)).

## Q28 — Empty parens `()` node semantics

User direction (2026-05-19): the grammar accepts `()` as a parenth-
esised form; what does it *mean*?

Candidates:
- **Placeholder / hole** — "something belongs here but is
  undetermined". Useful in partially-derived facts.
- **Global singleton ⊥ (bottom)** — the impossible / empty type.
- **Global singleton ⊤ (top)** — the universal-supertype.
  (zebra2.ein uses an explicit `T` atom for this — `()` would be
  the unnamed equivalent.)
- **Forbidden** — `()` is a parse error; no hole notation.

**Working answer:** *parked*. The grammar's current behaviour
parses `()` as a synthetic `@empty`-headed `SForm` with no
attributes; the engine treats it as a no-op atom. No semantic
commitment in M1. Revisit when a concrete use case forces the
question — most likely the negative-fact representation in P1.5
(hypothesis loop) or the partially-derived-fact representation in
P1.6 (trace).

When a use case arrives, the *placeholder / hole* reading is the
most likely choice because:
- ⊥ and ⊤ are better served by explicit named atoms (`T` in
  zebra2.ein is already the convention).
- *Forbidden* would require grammar work and offers no upside.

## Q29 — Pattern: walker vs cached executable program

Surfaced 2026-05-19 during the P1.3 review. P1.2 ships
`kb.Pattern` as a structural lift of the rule's `:match` / `:assert`
IR clause — `expr: IRNode` plus pre-computed metadata (variables,
relation_names, type_names, has_instance_pattern). The original
P1.3 plan proposed a *separate* `Pattern` AST inside `rules/`.

**Options:**

- **A — Walker**: matcher walks `kb.Pattern.expr` directly,
  dispatching on `IRNode` subtype each step.
- **B — Cached executable program** in `inference/`: lazy compile
  to opcodes (`Scan`, `Join`, `Guard`), cached by `Pattern.expr`
  identity.
- **C — Extend `kb.Pattern`** with a `compiled` field.

**Resolved 2026-05-20** — option **B**, with the **compile unit
being per (rule, activator-binding) pair** (not per `Pattern`
alone). The activator fact (e.g. `(transitive co-located)`) binds
the rule's parameter list *before* matching begins; the compiler
substitutes the parameters and bakes concrete relation names into
the program. Cache key: `(rule.name, activator.args)`. Zebra.ein
has 6 activators → 6 compiled programs.

Option A rejected: a walker re-discovers the activator binding on
every match attempt — wrong granularity for activator-driven T2
polymorphism. Option C rejected: contaminates kb with engine state
(inverts the kb → inference dependency direction).

Full analysis:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q29](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).

## Q30 — Universal rule library + import mechanism

Surfaced 2026-05-19 during the P1.3 review.

**Options:**

- (a) `examples/rules.ein` is a universal library every puzzle
  imports.
- (b) rules are always inline in the puzzle's `.ein` file (current
  zebra.ein).
- (c) hybrid — a kernel core auto-loaded, puzzles can extend.

**Deferred to P1.8** (user direction 2026-05-20). The universal
rule library question entangles with an *import / include*
mechanism in the IR language (how one `.ein` file references rules
in another), which is too large for P1.3 to resolve and not on the
M1-acceptance critical path. P1.8 — a new M1 phase, name + slot
TBD — will design the library + import mechanism and decide
between (a) / (b) / (c).

For P1.3: assume zebra.ein-style **inline rules per puzzle**
(option b's de-facto form).

Full context:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q30](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).

## Q31 — `:why` template substitution language

Surfaced 2026-05-19 during the P1.3 review. Zebra.ein uses
`{?rel}` (ein-var style, `?` prefix). The original P1.3 plan used
`{r}` / `{p0}` (Python format).

**Resolved 2026-05-20** — adopt the `{?var}` notation. The `?` is
*part of the reference*, not a prefix that's stripped; `?var` is
simultaneously the node name and the var name. Inside the `:why`
string, `{?rel}` substitutes to the bound node name (e.g.
`co-located`), `{?a}` to the bound argument (e.g. `Norwegian`),
etc. The `?` is preserved through binding — there is no separate
"bare var name" form.

Grammar: the template tokeniser scans for `{` … `}` delimited
sequences whose body starts with `?` followed by a var-name. Any
`{x}` *without* a leading `?` is a literal `{x}` (not substituted).

Full context:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q31](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).

## Q32 — `:where` semantics

Surfaced 2026-05-19 during the P1.3 review. Q24's working answer
was "`:where` is sugar for inline `(and …)` premise"; zebra.ein's
`transitive` rule uses `:where (neq ?a ?c)` *as a top-level
keyword inside `(and …)`* — already a top-level construct.

**Resolved 2026-05-20** — **drop `:where`; ship a built-in
predicate registry** (`inference/predicates.py`). All four
candidate benefits of `:where` over canonical `(and …)` (guard-
vs-generator distinction, prevent auto-vivification, readability,
future-proofing) dissolve once a predicate registry exists; the
registry is strictly more general.

Concrete:
- `inference/predicates.py` — initial set `eq`, `neq` (per Q33).
- Loader (`kb/from_ir.py`) consults registry before auto-
  vivifying unknown heads.
- Matcher dispatches premises via `(head ∈ registry)` → guard,
  else fact-scan.
- Migration: rewrite zebra.ein's `transitive` rule from
  `:where (neq ?a ?c)` → positional `(neq ?a ?c)`.

Closes Q24 retroactively (the "sugar" framing was effectively
saying "drop the keyword once registry exists").

Full context:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q32](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).

## Q33 — Predicate primitives — minimal set for M1

Surfaced 2026-05-19 during the P1.3 review. The original P1.3 plan
listed `forall`, `exists`, `count`, `single`, `the`, `\` (set
diff), `in`, variadic `?xs...` as needed primitives for T1.3.2.3
/ .4 / .8 / .9.

**Resolved 2026-05-20** (user direction): M1 ships **only the
minimal predicate set** required for the current 6 rules.

### M1 built-in predicate registry

```python
predicates = {
    "eq":  lambda b, args: resolve(args[0], b) == resolve(args[1], b),
    "neq": lambda b, args: resolve(args[0], b) != resolve(args[1], b),
}
```

Two binary predicates. That's it.

### What's not a predicate (and why)

- `(rel ?a ?b)` pattern matching — matcher's core operation;
  existence check is pattern matching itself.
- `(not (rel ?a ?b))` — structural wrapper (matcher: negation as
  failure; asserter: negative fact assertion). Not a registry entry.
- `forall ?x P(?x)` — already `:match (P(?x))`; the clause is
  implicitly universally quantified.
- `exists ?x P(?x)` — already `:assert (P(?x))`; adding a fact IS
  the existential.
- `single ?x P(?x)` — derivable: `(and P(?x) (not (and P(?y) (neq ?y ?x))))`.

### Deferred to followups (not P1.4, not P1.7)

- Variadic args `?xs...`
- Numeric predicates: `lt`, `gt`, `le`, `ge`
- Set operations + aggregation: `count`, `member`, `in`, set diff
- Domain primitives: `dom`, `functional`

A future F-numbered followup adjacent to F4 owns these.

Full context:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q33](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).

## Q39 — Module path

Surfaced 2026-05-19 during the P1.3 review. The kernel docs at
`docs/kernel/inference/` point at `src/ein_bot/inference/`; the
original P1.3 plan used `src/ein_bot/rules/`.

**Resolved 2026-05-20** — **flat `src/ein_bot/inference/`**
(option A). All M1 engine artifacts (matcher, firing, engine
driver, predicate registry, compiler) sit at this one level. Tests
at `tests/inference/`.

KB and its `kb.rules` (rule *definitions*) stay in
`src/ein_bot/kb/` — rule definitions are *data*; matching /
saturation is *engine*.

Promotion to nested `rules/inference/` (option B) when a second
non-engine file appears at the rule-language layer — likely Q30 /
P1.8 (universal rule library lands `rules/library/…`) or F5
(rules-as-data lands rule synthesis machinery). Migration is a
mechanical `git mv` + import-fix sweep; no design rework.

Full context:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q39](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).

## Q40 — Hypothesis-rule premises

Surfaced 2026-05-19 during the P1.3 review. The original P1.3
plan's T1.3.2.5 sketched:

```lisp
(rule hypothesis-contradiction
  :match  (and (hypothesis ?h) (contradiction-under ?h))
  :assert (not ?h)
  ...)
```

The premises `(hypothesis ?h)` and `(contradiction-under ?h)`
don't correspond to any fact in the shipped data model — P1.2's
`Provenance` carries `kind='hypothesis' | 'rejected'` on the fact
itself, not as a separate fact.

**Options:**

- **A — Synthetic facts**: engine emits `(hypothesis <fact>)` and
  `(contradiction-under <fact>)` as auto-vivified relations; the
  matcher dispatches normally.
- **B — Engine procedure**: hypothesis-contradiction is a special
  engine procedure that doesn't go through the matcher.

**Resolved 2026-05-20** — option **A**. The kernel ein model
(`docs/kernel/ir/01-ein-graph/03_ein_model.md` §3) already says
facts can have **relational nodes** as args — `(hypothesis
(co-located Norwegian House-2))` is just a fact whose first arg
is a relational node. This is *not* a compound-node-kind promotion
(Q26 is about virtual / computed kinds; different concern); it's
the kernel's existing node-as-arg duality.

Required work:
- **`Fact.args` widening**: `tuple[str | int | Fact, ...]`. Small
  P1.2 patch.
- **Loader recursion** for nested SForms in args.
- **Matcher extension** for nested-fact patterns (Q29 compile pass
  treats SForm args as sub-patterns; runtime unifier recurses).
- **Engine emission**: P1.5's fork procedure emits `(hypothesis
  <fact>)`; the contradiction detector emits `(contradiction-under
  <fact>)`.

Pays F5 (rules-as-data) amortisation for free — fact-as-arg is the
precondition for rules-as-facts.

Promotes T1.3.2.5 (`hypothesis-contradiction`) back into the M1
core (was deferred under §F path (a) before Q40 resolution).

Full context:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q40](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).

## Q41 — Rule priority — scale and placement

Surfaced 2026-05-19 during the P1.3 review. Zebra.ein uses 1/3/5/8
priorities inside each rule definition. The original P1.3 plan
used 100/200/300/900 in a static config table.

**Resolved 2026-05-20** — adopt the **100/200/300/900 banded
scale** from the original P1.3 plan; keep zebra.ein's **per-rule
placement** (priority lives inside the `(rule …)` form, not in an
engine config table); **lower fires first**.

### Bands

| band       | priority | what fires here                                                  |
|------------|---------:|------------------------------------------------------------------|
| propagate  | 100      | cheap, structural propagation — symmetric, inverse, implies      |
| derive     | 200      | composition / join — transitive, square-fwd, square-bwd          |
| eliminate  | 300      | exclusion / narrowing — type-exclusivity                          |
| hypothesis | 900      | reserved for hypothesis-contradiction (P1.5)                      |

Within-band fine-grained ordering (e.g., 110 / 120) is open and
per-puzzle.

### Zebra.ein migration

`symmetric` 1→100, `implies` 3→100, `transitive` 5→200, `square-fwd`
8→200, `square-bwd` 8→200, `type-exclusivity` 10→300.

Full context:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q41](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).

## Q42 — P1.3 scope — path (a) vs (b)

Surfaced 2026-05-19 during the P1.3 review. Two paths:

- (a) Narrow P1.3 to the 6 zebra.ein rules + a seam for the other 4.
- (b) Keep all 10 with 4 marked `:blocked-by`.

**Resolved 2026-05-20 by Q33 cascade** — **path (a)**. Q33's
resolution (M1 ships only `eq` + `neq` predicates; all numeric /
set / variadic primitives → followups) makes the 4 deferred rules
structurally unshippable in M1. Path (b)'s `:blocked-by`
placeholders would be misleading — they'd point at followups, not
P1.4 / P1.7.

M1 core = 6 zebra.ein rules: `symmetric`, `transitive`, `implies`,
`square-fwd`, `square-bwd`, `type-exclusivity`. Plus
`hypothesis-contradiction` promoted in by Q40 = **7 rules total**.

The 4 deferred rules (T1.3.2.3 / .4 / .6 / .8 in the original
plan) land in a future followup phase adjacent to F4.

Full context:
[`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q42](p1.3_inference_rules/s1.3.0_review_and_revisions.md#g-consolidated-open-questions-for-p13).
