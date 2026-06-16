# Categorical formulation of the puzzle

## The observation

The triangle inference rule of the `Ein` design is *literally*
categorical composition. Once you see that, you naturally ask
whether the whole puzzle has a categorical formulation: what are
objects? morphisms? categories? functors?

## User's own words

> The transitive rule is literally the same thing as composition in
> CT. Can this problem be formulated in CT, and how? What would be the
> objects, the morphisms, the categories, the functors etc.

## The mapping (sketched, not committed)

There are at least three plausible categorical readings — none of
them is *the* answer yet.

### Reading A — graph as a free category
- **Objects**: entities (`Norwegian`, `House-1`, `Blue`, `Tea`, …).
- **Morphisms**: relations (`lives_in`, `next_to`, `has_color`,
  `drinks`, …) plus all paths obtained by composing them.
- **Composition**: chaining relations.
- **Identity**: trivial `self_is_self` on every entity.

The triangle rule = the requirement that composites be themselves
named morphisms (or at least computable). The square rule is
*non-trivial* in this reading because spatial constraints are
two-sided.

### Reading B — objects = types, morphisms = relations
- **Objects**: domains / types (`House`, `Color`, `Nationality`,
  `Pet`, `Drink`).
- **Morphisms**: typed relations (`House —has_color→ Color`,
  `Nationality —lives_in→ House`, …).
- **Composition**: typed relation chaining.

This is the "schema" view: each puzzle has a small category of
types, and each puzzle instance is a functor *into* `Set` (or a
finite `FinSet`).

### Reading C — puzzle instance as a functor; constraints as commuting diagrams
- **A puzzle instance** is a functor `F : Schema → FinSet` (or into
  some target category encoding the value domains).
- **A constraint** "the Brit lives in the red house" becomes the
  commuting diagram requiring `F(lives_in) ∘ F(is_brit) = F(is_red)`.
- **Solving the puzzle** = finding a functor satisfying all the
  commuting-diagram constraints.

This is the most idiomatic CT reading: the puzzle is the search for
a model of a sketch.

## Where category theory becomes genuinely heavy

- **Global constraints** (`allDifferent`, "exactly one X per Y")
  want *limits / pullbacks / monomorphisms*. Their direct categorical
  expression is correct but cumbersome.
- **Higher rewriting** — applying inference rules as a process —
  wants *adhesive categories* and *DPO/SPO graph rewriting*.
- **Multiple-translation pipelines** (`NL → graph → SMT → proof`)
  want *functors between categories of representations*, with
  *natural transformations* between alternative such functors.

## Pragmatic note

CT here is best treated as a **semantics / meta-language**, not a
runtime substrate:

- it explains *why* the pieces compose,
- it justifies named transformations between representations,
- it does *not* execute anything faster than a typed-hypergraph
  engine would.

A reasonable role: design-time formal sanity check, with the actual
engine living in [02-graph-as-formal-substrate.md](02-graph-as-formal-substrate.md).

## Open questions

1. **Which reading should the design commit to?** B is most
   tractable; C is most idiomatic; A is closest to current `Ein`.
2. **Does the categorical view buy anything operational?** Examples:
   automatic detection that two rule sets are equivalent (functorial
   isomorphism); transferring a proof from one schema to another
   (natural transformation); composing puzzle solvers
   (limit of categories).
3. **What about higher categories?** 2-morphisms could model
   rewrites-of-rewrites — natural for the rule-evolution direction
   (and the self-modifying language idea in
   [01-self-modifying-constraint-language.md](01-self-modifying-constraint-language.md)).
4. **Is graph rewriting (DPO/SPO) the right operational read-off?**
   Probably yes, but the cost-benefit vs simpler typed pattern
   matching needs measuring on real rules.

## Connections (context, not answers)

- CT primitives, diagram languages, monoidal/string-diagram
  variants: [05-category-theory.md](../lib/05-category-theory.md).
- DPO/SPO graph rewriting:
  [05-category-theory.md](../lib/05-category-theory.md) §5,
  [06-graphs-rewrite-systems.md](../lib/06-graphs-rewrite-systems.md) §4.
- Catlab.jl as a possible host for a categorical IR:
  [04-programming-languages.md](../lib/04-programming-languages.md) §5.
- The user's intuition that "graphs are everywhere" — supplies the
  raw material for any categorical reading:
  [06-graphs-rewrite-systems.md](../lib/06-graphs-rewrite-systems.md).
