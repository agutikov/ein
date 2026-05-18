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
| Q34 | Algebraic properties beyond symmetric/transitive (reflexivity, antisymmetry, …) |
| Q35 | Variable typing via `:match (is-a ?var Type)` — pattern, not syntax          |
| Q36 | Relation inheritance / rule polymorphism — `(subtype-of instance-of subtype-of)` |

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

## Algebraic properties beyond symmetric/transitive (Q34)

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

**Promotion trigger**: a puzzle/domain where a property beyond
`symmetric`/`transitive` drives the solution. Likely candidates:
proof-graph theorem proving (reflexive equality / congruence),
Sudoku variants with explicit `≠` (irreflexive), scheduling with
precedence (antisymmetric `≤`), 2-D spatial puzzles (Q32) where
adjacency is more than 1-D `next-to`.

Connection: [idea 06](../../docs/ideas/06-inference-rules-completeness.md)
§rule families, [M1 P1.3 S1.3.1](../m1_core_graph_reasoning/p1.3_inference_rules/)
(rule presentation language), [`docs/ir.md` §3 predicate
registry](../../docs/ir.md).

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

---

## How to promote

Any item that gains a concrete deliverable — a milestone scope, a
worked test puzzle, a measurable target — graduates to its own
followup file (rename `qNN_<topic>.md` and link from the F4 index).
When two related items appear in the same scope, bundle into a new
milestone folder under `plans/m_followups_<topic>/`.
