# F4 — Cross-cutting ideas

Theme owner: parking lot.
Trigger: opportunistic — none of these blocks any milestone.

This file collects ideas that recur across multiple
[`docs/ideas/`](../../docs/ideas/) notes but don't belong to any
one of them. Each gets a short stanza; promote individual items
to their own followup file when they grow.

## Open questions (parked)

| Q   | Title                                                                       |
|-----|-----------------------------------------------------------------------------|
| Q14 | Rule learning source — hand-written, library, LLM-suggested, learned?       |
| Q30 | Equality saturation / e-graph promotion — when does Zebra-scale benefit?    |
| Q31 | LLM as policy over the reasoning graph (M1.P1.5 search-tree choice)         |
| Q32 | 2-D / N-D spatial — when do we need beyond M1.P1.4's 1-D position lattice?  |
| Q33 | Reasoning-graph differential rendering for live agents                      |
| Q34 | Algebraic properties beyond symmetric/transitive + 2^7 cartesian product   |
| Q35 | Variable typing via `:match (is-a ?var Type)` — pattern, not syntax          |
| Q36 | Relation inheritance / rule polymorphism — `(subtype-of instance-of subtype-of)` |
| Q37 | Induction — facts → rules over relations; rules learned from fact patterns  |
| Q38 | LLM as fact/relation/type/rule extractor — per-word/per-role question schemas |

---

## Rule learning from human walkthroughs (Q14)

Per [idea 06 §Open sub-questions point 5](../../docs/ideas/06-inference-rules-completeness.md#open-sub-questions).
M1's rules are hand-written; the user asks whether they could be
*learned* by observing human walkthroughs.

- **What it would look like**: ingest a corpus of annotated human
  walkthroughs; for each step, find the smallest pattern that
  fires the same conclusion; aggregate; propose new
  `(rule …)` definitions.
- **Why hard**: human walkthroughs collapse multiple propagation
  steps into one ("then by exclusion …"); inverting that requires
  knowing the granularity the engine uses.
- **Why interesting**: M1's rule set may turn out to be *incomplete*
  for problem classes the user hasn't yet looked at; learned rules
  would close gaps automatically.

Connection: [idea 06](../../docs/ideas/06-inference-rules-completeness.md),
[idea 08](../../docs/ideas/08-human-style-deductive-trace.md).

## E-graph promotion (Q30)

M1.P1.2 ships *equality-class IDs* as a placeholder; F4 explores
whether promoting the reasoning layer to a full e-graph
(equality saturation) buys anything on real puzzles.

- **Plausibly yes** if a puzzle has rich equational reasoning
  ("the Brit's pet is the same as the Spaniard's dog").
- **Plausibly no** for Zebra — the equality structure is shallow.

Connection: [idea 02 §What "compute directly on the graph" can
mean](../../docs/ideas/02-graph-as-formal-substrate.md#what-compute-directly-on-the-graph-can-mean),
[`docs/index/06-graphs-rewrite-systems.md §egraph`](../../docs/index/06-graphs-rewrite-systems.md).

## LLM-as-policy in search-tree (Q31)

Per [idea 09 §LLMAsPolicy](../../docs/index/09-cognitive-architectures-neurosymbolic.md):
the LLM picks *which hypothesis branch to explore first* in
M1.P1.5. AlphaZero-style guided proof search.

- **Why**: human walkthroughs make "good" branching choices —
  the kind a small policy net could learn.
- **Why hard**: serialising the search state for the LLM is
  itself a research problem; mid-search LLM calls are expensive.

Connection: [`docs/index/11-search-optimization-algorithms.md`](../../docs/index/11-search-optimization-algorithms.md) §MCTS / AlphaZero.

## 2-D / N-D spatial (Q32)

M1.P1.4's 1-D position lattice covers Zebra. *Logic-grid* puzzles
in 2-D, *chess-style* puzzles, *graph-colouring* puzzles all need
more. F4 would extend `:positional` to multi-dimensional and
attach Allen-style 2-D relations.

Connection: [M1.P1.4 S1.4.2](../m1_core_graph_reasoning/p1.4_constraints/s1.4.2_spatial.md) leaves the replacement seam explicit.

## Reasoning-graph differential rendering (Q33)

Real-time UI showing the graph evolving step-by-step. Useful for
debugging the trace planner (M1.P1.6) and for live demos. Probably
a Cytoscape.js page driven by the trace IR.

Connection: M1.P1.6, the existing
[`docs/index/knowledge-graph.cy/`](../../docs/index/knowledge-graph.cy/)
Cytoscape view as a template.

## Algebraic properties beyond symmetric/transitive — and the 2^7 cartesian product (Q34)

### Properties beyond the M1 core

M1's `co-located` is mathematically reflexive (an equivalence
relation), but the `reflexive` rule was dropped from
[`examples/zebra.ein`](../../examples/zebra.ein) — every attribute
would generate a trivial self-edge `(co-located X X)` with no
inference payoff on Zebra-class puzzles. The engine can answer
"is X co-located with X?" at query time without materialising.

The general theme: algebraic properties that are trivial on Zebra
may become operationally important on richer problem classes:

- **Reflexivity** — congruence reasoning over proof terms
  (`(equal X X)` as a base case).
- **Antisymmetry** — partial orders (`x ≤ y ∧ y ≤ x ⇒ x = y`).
- **Irreflexivity** — strict orders / `≠` over a finite domain.
- **Totality / connex** — linear orders.
- **Asymmetry** — precedence constraints in scheduling.

When the engine encounters a problem whose solution walks one of
these, the rule library needs three pieces working together:

1. The property tag (e.g. `(reflexive co-located)` as a
   property-application fact).
2. The generic rule consuming the tag (the dropped
   `(rule reflexive (?rel) …)`).
3. A structural predicate like `(in-domain ?rel ?T)` that checks
   *signature homogeneity* — `(?rel ?a ?a)` only makes sense when
   `?a`'s type fits every position of `?rel`'s signature.

### The 2^7 cartesian product as a design tool

The project's existing seven rule-property tags
(`reflexive`, `symmetric`, `transitive`, `asymmetric`,
`exclusive`, `spatial-fwd`, `spatial-bwd`) generate 128 combinations.
Most are degenerate, inconsistent, or pointless, but a handful name
operationally important kinds of relation:

| profile                                       | structure produced              | example                                |
|-----------------------------------------------|---------------------------------|----------------------------------------|
| reflexive + symmetric + transitive            | equivalence classes             | `same-color-as`, `co-located` (treated as equivalence) |
| reflexive + transitive + antisymmetric        | partial order                   | `is-a`, `part-of`, `before-or-equal`  |
| transitive + asymmetric                       | strict order                    | `before`, `proper-subtype-of`, `ancestor-of` |
| symmetric + asymmetric                        | INCONSISTENT (degenerates to self-loops or empty) | — |
| spatial-fwd + spatial-bwd                     | bundled projection rule         | Zebra `right-of` over `co-located`    |
| transitive only                               | path / reachability closure     | generic graph-rewrite closure          |
| symmetric only                                | undirected graph                | `next-to`                              |
| reflexive only                                | self-edge closure (usually triv.)| — (trivial on most relations)         |

A clean way to expose this in the system: a `(relation-profile …)`
declaration that pre-classifies the 128 combinations into
`valid / inconsistent / degenerate / domain-specific / redundant`
buckets, used as a *design-time linter* rather than as runtime
logic.

```lisp
(relation-profile is-a
  :reflexive    true
  :transitive   true
  :antisymmetric true
  :role classification)

(relation-profile co-located
  :symmetric    true
  :transitive   true
  :role equivalence)

(relation-profile right-of
  :transitive   true
  :asymmetric   true
  :spatial-fwd  true
  :spatial-bwd  true
  :role spatial-order)
```

Note: `spatial-fwd` / `spatial-bwd` are not intrinsic algebraic
properties (unlike `symmetric` / `transitive`) — they are
*interaction rules* between a relation and `co-located`:

```text
R + co-located + R  ⇒  co-located
```

So they classify a *relation bundle*, not just a relation. This is
the conceptual content of the PoC's square rule, already
implemented in zebra.ein but worth naming.

### Minimal rule set — minimal for *what*?

The user (2026-05-18) asked whether one should search for the
*minimal* rule set. There is no single answer — different minima
optimise different things:

1. **Minimal semantic basis** — fewest primitives from which others
   derive. `equivalence = reflexive + symmetric + transitive`;
   `partial order = reflexive + transitive + antisymmetric`;
   `strict order = transitive + asymmetric`.
2. **Minimal for solving Zebra** — the smallest set of rules that
   reproduces the solution. The current `zebra.ein` set
   (`symmetric`, `transitive`, `implies`, `square-fwd`, `square-bwd`,
   `type-exclusivity`) is close — possibly missing one or two for
   richer puzzles.
3. **Minimal for explanation-completeness** — fewest rules such that
   every human-walkthrough step (idea 08) has a named firing. May be
   larger than (2) because human walkthroughs name moves that the
   engine could collapse.
4. **Minimal implementation complexity** — fewest AST nodes in the
   rule set. Favours generic over per-relation rules.
5. **Minimal trace complexity** — fewest derived intermediate facts.
   May conflict with (4) — generic rules can over-derive.

The M1 acceptance target is (3) for Zebra (explanation-complete on
the human walkthrough). The other minima are *measurement* targets
for benchmarking against alternative rule sets.

### Categorical / OOP-collapse framing

The deeper observation in the user's note (2026-05-18): with this
machinery, OOP-style inheritance, typing, constraint propagation,
and rule activation become *the same phenomenon* — edge propagation
over structured graphs. Inheritance is just transitive `is-a`
propagation; "having a type" is being on the receiving end of an
`is-a` edge; "behaviour" is additional edges derivable from the
type via inheritance rules.

| OOP concept       | graph-native form                          |
|-------------------|--------------------------------------------|
| class             | a graph node                               |
| inheritance       | transitive `is-a` propagation              |
| instance-of       | a kind of `is-a` (often collapsed)         |
| method / property | another relation; inheritable via the rule above |
| virtual method    | property lookup via traversal              |

This formalisation overlaps F1 (categorical formulation): types =
objects, relations = morphisms, inheritance = morphism composition
/ preorder, rules = rewrite morphisms. See
[F1](f1_categorical_formulation.md) for the categorical thread.

### Promotion trigger

A puzzle/domain where a property beyond `symmetric`/`transitive`
drives the solution. Likely candidates: proof-graph theorem proving
(reflexive equality / congruence), Sudoku variants with explicit
`≠` (irreflexive), scheduling with precedence (antisymmetric `≤`),
2-D spatial puzzles ([Q32](#2-d--n-d-spatial-q32)) where adjacency
is more than 1-D `next-to`.

Connection: [idea 06](../../docs/ideas/06-inference-rules-completeness.md)
§rule families, [M1 P1.3 S1.3.1](../m1_core_graph_reasoning/p1.3_inference_rules/)
(rule presentation language), [`docs/ir.md` §3 predicate
registry](../../docs/ir.md), [F1](f1_categorical_formulation.md).

## Variable typing via `:match (is-a ?var Type)` (Q35)

User direction (2026-05-18, this conversation): variables in rule
patterns should carry types when not deducible from context — but
**not via a new syntactic feature** (`?var:Type`). Instead, the type
constraint goes directly into the `:match` clause:

```lisp
;; instead of  :match (rel ?a:Attribute ?b:Attribute)
:match (and (rel ?a ?b) (is-a ?a Attribute) (is-a ?b Attribute))
```

Justification (user's): in the original zebra.ein model where
`instance` is a hardcoded kernel relation, typed-var syntax could
be sugar that desugars to `(instance ?a Type)`. But in the
unified zebra2.ein model with `is-a`, the type constraint *is*
just another premise in `:match` — adding dedicated syntax buys
nothing the pattern language doesn't already express.

**Type system scope** (parked): types for variables is the
beachhead; the same principle generalises to "types for relations,
rules, …" (a type system over the IR's meta-level — see Q36).

**Promotion trigger**: when (a) `:match` patterns become long enough
that explicit `(is-a ?var T)` premises clutter readability, or (b)
the engine wants to use types as a *search-pruning* signal (only
enumerate descendants of T as bindings for ?var). Pruning is the
real benefit; explicit-syntax sugar is *not* the motivation.

Connection: examples/zebra2.ein (the `is-a` formulation that makes
this natural), `docs/ir.md` §3 pattern sub-language.

## Relation inheritance / rule polymorphism (Q36)

User direction (2026-05-18, this conversation): if relations
themselves have *types* (treated as nodes in the is-a tree), then
*inheriting a relation means inheriting its rules*. Concretely:

```lisp
;; declares: the relation `instance-of` is a subtype of the
;; relation `subtype-of` — so any rule applied to `subtype-of`
;; (e.g. `(transitive subtype-of)`) also applies to `instance-of`.
(subtype-of instance-of subtype-of)
```

This is **relation polymorphism**: rule-applications propagate down
the is-a tree of relations.

### Where is our inheritance rule btw?

The user asked — and the answer is honest about a gap.

In **zebra.ein** (the `(instance …)` / `(type …)` split), the
*composition* rule "instance-of(a, T) ∧ subtype-of(T, S) ⊢
instance-of(a, S)" is **not declared anywhere**. The engine never
materialises `(instance Norwegian Attribute)` from `(instance
Norwegian Nationality) ∧ (type Nationality Attribute)`. The
`type-exclusivity` rule fires only on directly-declared instance
facts. This is a real correctness gap for puzzles where the
composition is load-bearing — Zebra happens not to need it.

In **zebra2.ein** (the unified `is-a`), the composition IS
declared via `(transitive is-a)`. Norwegian is-a Nationality, and
Nationality is-a Attribute, so Norwegian is-a Attribute by
transitivity. No separate composition rule needed.

### Does the relation-inheritance idea make sense?

Partially. The subtle bit:

- **subtype-of is transitive**: A ⊆ B ∧ B ⊆ C ⟹ A ⊆ C. ✓
- **instance-of is NOT transitive**: a ∈ T ∧ T ∈ U does NOT imply
  a ∈ U. (Norwegian is-an-instance-of Nationality, Nationality
  is-an-instance-of [meta-class], does not make Norwegian an
  instance of [meta-class].)
- **instance-of *composes* with subtype-of**: a ∈ T ∧ T ⊆ S ⟹
  a ∈ S. (This is a different rule shape — heterogeneous.)

So if we naively declared `(subtype-of instance-of subtype-of)`
and inherited *transitivity* from subtype-of to instance-of, we'd
make instance-of wrongly transitive. The simple "inherit all rules"
formulation is unsound.

A correct version would need:
- Per-rule annotation of which subtype-of relations preserve it
  (a "covariance" tag).
- Or rules expressed at the right level of generality so that the
  composition form falls out (which the unified `is-a` of zebra2
  already does, by collapsing the distinction).

**The cleanest answer in the current design**: collapse to one
inheritance relation (zebra2.ein's `is-a`) and avoid the
meta-inheritance question entirely. The interesting research
question — what does "inherit a rule by inheriting a relation"
mean formally — survives only in the *split* model and is a
meaningful piece of relation polymorphism / categorical reasoning
about rule preservation.

### Categorical framing

Rule polymorphism is closely related to **functoriality**: a rule R
applies to relation P, and there's a morphism f: P → Q in the
"relation category"; if R is preserved under f (R is *functorial*
in P), it also applies to Q. Some rules are functorial (e.g.
symmetric closure preservation); others aren't (transitivity of
subtype-of is NOT functorial along an instance-of inclusion).

Connection: [f1_categorical_formulation.md](f1_categorical_formulation.md),
[docs/ideas/07-categorical-formulation.md](../../docs/ideas/07-categorical-formulation.md).

**Promotion trigger**: a puzzle where the engine has to manage two
different inheritance-like relations *and* a non-trivial rule that
only applies to one of them. Until then, zebra2's unified `is-a`
sidesteps the issue.

## Induction — rules from facts (Q37)

User direction (2026-05-18): if rules are first-class graph objects
and facts are also graph objects, can the engine *learn* new rules
by spotting regularities in the fact set? Two specific phrasings:

1. **Facts → rules on relations** — observe that `(R a b)` and
   `(R b c)` and `(R a c)` co-occur many times → induce
   `(transitive R)` as a property-application fact.
2. **Rules induced from facts directly** — observe `(R a b) ∧ (S b
   c) ⇒ (T a c)` patterns occurring → propose the explicit rule.

This is a generalisation of [Q14](#rule-learning-from-human-walkthroughs-q14)
(rule learning from human walkthroughs): Q14 is supervised (the
human trace is the supervision); Q37 is unsupervised (just look at
the fact set for regularities).

**Why hard:** spurious patterns. Two facts that happen to be
symmetric in Zebra do not mean `(symmetric R)` is intended; the
user could trivially break the regularity by adding a third fact.
A confidence threshold + a human-in-the-loop ratification step
would be the natural shape.

**Why interesting:** complements F2 (self-modifying language) —
the LLM proposes; the induction engine validates against a corpus
of facts. The induced rule library is then a measurement target
for [Q34's "minimal rule set"](#algebraic-properties-beyond-symmetric-transitive--and-the-2-7-cartesian-product-q34).

Connection: [docs/ideas/01](../../docs/ideas/01-self-modifying-constraint-language.md),
[Q14](#rule-learning-from-human-walkthroughs-q14).

## LLM as fact/relation/type/rule extractor (Q38)

User direction (2026-05-18): "how can an LLM produce facts,
relations, types, and rules?" Proposal: drive the LLM with a
*per-word, per-role* schema of questions:

- For nouns (in subject role): is this an `Instance`? if so, what
  `Type` does it belong to? does the surrounding text introduce a
  new `Type`?
- For verbs (in predicate role): does this name a `Relation`? if
  so, what are its argument types? is it a known relation or a new
  one?
- For pre-/post-modifiers: do they introduce property facts on a
  relation (e.g. "always", "symmetric") or constrain a value?
- For determiners and quantifiers: do they correspond to
  cardinality constraints? exclusivity?

The output is a structured stream of `(type …)`, `(instance …)`,
`(relation …)`, `(rel …)`, `(rule …)` IR forms — a constrained
generation problem (cf. [idea 01](../../docs/ideas/01-self-modifying-constraint-language.md)).

This is the M2 territory ([NL → IR](../m2_nl_to_ir/README.md)) — at
F4 here only as the *cross-cutting research question*. The concrete
implementation lands in [M2 P2.4 (NL → IR pipeline)](../m2_nl_to_ir/p2.4_nl_to_ir_pipeline/);
the *schema of question lists per word-class / per-role* is the
parked design problem.

Why F4 not M2: the *catalogue* of questions to ask, and *how
ein-bot's IR vocabulary maps to natural-language constructions*, is
prior to picking an LLM or a GBNF — and outlives any specific
M2 implementation choice. It is the *rosetta stone* between the
two surfaces.

Connection: [M2 P2.4](../m2_nl_to_ir/p2.4_nl_to_ir_pipeline/),
[idea 01](../../docs/ideas/01-self-modifying-constraint-language.md),
[docs/index/10 NLP & semantic parsing](../../docs/index/10-nlp-semantic-parsing.md).

---

## How to promote

Any item that gains a concrete deliverable — a milestone scope, a
worked test puzzle, a measurable target — graduates to its own
followup file (rename `qNN_<topic>.md` and link from the F4 index).
When two related items appear in the same scope, bundle into a new
milestone folder under `plans/m_followups_<topic>/`.
