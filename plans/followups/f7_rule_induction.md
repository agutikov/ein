# F7 — Rule taxonomy + rule induction

The hand-authored rule library is the engine's largest hardcoded
input. Two related followups: (a) classify rules by their
*structural shape* and use the classification to mechanically lift
specific rules into more general ones, and (b) infer entire rules
from the relation algebra of the puzzle's ontology.

## Trigger

Surfaces when:

- The rule library grows past hand-management (≥ 15-20 rules, or a
  second puzzle joins Zebra), and bulk transformations / templates
  become cheaper than per-rule authoring.
- M2's NL → IR pipeline needs to *deduce* the rule library from
  text rather than read it off a curated `examples/rules.ein`.
- A specific puzzle exposes an activator-selection problem (see
  `(transitive is-a)` in zebra2 — logically valid but
  computationally devastating; see *§Rule-set sufficiency* below).

## The 4-class taxonomy

The M1 rule library exhibits four shapes, by what's parameterised
vs hardcoded in `:match` / `:assert`:

| class | description | example | params |
|------:|-------------|---------|--------|
| **(1)** Pure structure | No relation names anywhere — only variables.  Relation participation entirely via the activator binding. | `symmetric`, `implies` | `(?rel)` / `(?p ?q)` |
| **(2)** Predicate-using | Includes built-in predicates (`neq`, `not`, `eq`, `and`, `or`) but no hardcoded relation names. | `transitive` (uses `neq`) | `(?rel)` |
| **(3)** Relation-var + relation-const | Mixes a parameterised relation with one or more hardcoded relation names. | `square-fwd`, `square-bwd`, `square-unique` (parameterise `?R`, hardcode `co-located`) | `(?R)` / `(?R ?T)` |
| **(4)** No relation vars | Every relation in the body is a literal Atom; the rule is bound to a specific relation algebra. | The pre-T2 `type-exclusivity` (hardcoded `instance`, `co-located`) | `()` |

The M1 zebra.ein rewrite (2026-05-21) lifted the original
class-(4) `type-exclusivity` into class-(3) by introducing the
`?R` parameter and the activator `(type-exclusivity co-located)`.
That migration is the prototype for the *generalisation automation*
sub-track below.

### Cross-cutting: relation-var arity

Orthogonal to the 4 classes, count the relation vars in the rule
body:

- **One** — describes a property of a single relation
  (`symmetric` says R is symmetric; `transitive` says R chains).
- **Multiple** — describes interaction between relations
  (`implies` says one relation includes another; `square-fwd`
  says co-located commutes with strict-order R; the entire square
  family is really a categorical statement about how `co-located`
  behaves under a particular relation-algebra structure).

The square-fwd insight: it's *"co-located's behaviour under a
strict-order relation R"* — equivalently, that the equivalence
relation `co-located` is **invariant** under the action of R on
the underlying set. There's a category-theoretic statement
lurking; see F1 (categorical formulation).

## Sub-track A — generalisation automation

Move from class (4) → (3) → (2) → (1) mechanically:

- **(4) → (3)**: find hardcoded relation names that don't refer to
  built-in predicates; replace with a fresh `?R` parameter; add a
  per-puzzle activator that re-introduces the binding.
- **(3) → (2)**: harder — requires recognising when the "hardcoded
  relation" is really another rule-parameter slot, not a literal
  constant. Probably needs schema annotations.
- **(2) → (1)**: hardest — abstract over the built-in predicates,
  parameterising over the *predicate kind* (eq/neq/etc.) rather
  than the predicate identity. Borderline interpreter territory.

Each step is a small refactoring transformation on the rule's IR
AST plus a corresponding ontology change (new activator fact). The
M1 type-exclusivity migration is the worked example for (4) → (3).

## Sub-track B — rule induction from relations

If the ontology declares a relation with structural properties
(declared in IR or inferred from facts), the engine can *induce*
which rule activators apply. Example:

```lisp
(relation parent-of Person Person)
(asserted-property parent-of  transitive)
(asserted-property parent-of  asymmetric)
```

would induce:

```lisp
(transitive parent-of)
(asymmetric parent-of)
```

For richer cases, the property itself can be inferred from sample
facts (statistical induction — if `(R a b)` and `(R b c)` always
appear with `(R a c)`, R is *probably* transitive). This is the
**ontology induction** thread from
[`docs/ideas/04-nlp-to-graph-to-solver-pipeline.md`](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md);
it's a research direction, not a small refactor.

This sub-track is the natural endpoint of M2's NL → IR pipeline:
parsing produces relation declarations + sample facts; rule
induction fills in the activators; the engine takes it from
there. Without it, M2 cannot fully automate model construction
from natural language — a human still has to write activator
facts.

### Sub-sub-track B' — instance properties → type properties

A finer-grained generalisation, distinct from inducing rule
activators on relations: lift properties from **instances** to the
**type**. Surfaced 2026-05-24 from TODO.md's "P1.8 ideas" entry.

Three concrete patterns:

- **No facts at all** — if no `(R a b)` exists for any
  `a : A, b : B`, induce `(no-relation R A B)` *at the type
  level* — i.e. R doesn't connect A-instances to B-instances in
  this domain. Stronger than per-fact NAF: encodes the absence as
  a positive type-level claim.
- **All facts present** — if `(R a b)` holds for *every* pair
  `(a, b) ∈ A × B`, induce `(total-relation R A B)` (or the
  generalised form). The type-level analog of pointwise
  enumeration.
- **Partial facts** — if some `a : A` have `(R a _)` and some
  don't, **A splits into two sub-types** under R's domain. Induce
  `(subtype A_with_R A)` and `(subtype A_without_R A)`; assign
  instances accordingly. The clustering signal IS the rule shape
  for a new sub-typing.

The third case is the most interesting: it suggests **ontology
refinement** driven by observed irregularity. The pre-induction
ontology says "A-instances are interchangeable"; the post-induction
ontology has discovered they're not (under R), and adds the
sub-types that match the data.

Cross-cuts:

- Composes with sub-track B (property induction at the relation
  level) — instance-property induction can *feed* relation-property
  induction once a partial pattern is enriched enough.
- Composes with [Idea 04](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md)
  ontology deduction — the NL pipeline likely produces partial
  facts; instance-property induction is how the engine *responds*
  to that partiality with structure.
- Composes with [P1.8 Theme C — negative-fact volume](../m1_core_graph_reasoning/p1.8_ein_lang_modules/README.md)
  — `(no-relation R A B)` at type level *replaces* the
  `|A| × |B|` negative-fact volume that would otherwise be
  materialised.

Promotion ordering: probably comes **after** sub-tracks A + B
have shipped (the type-level induction needs the relation-level
property language to land its conclusions into).

## Sub-track C — rule-set sufficiency

A logically-correct rule activation can still be operationally
wasteful. Example from zebra2.ein (2026-05-21):
`(transitive is-a)` is sound (is-a is transitive in any sensible
ontology), but it derives ~30 ancestor edges per leaf, which
then makes `(sibling-exclusive is-a)` produce ~1330 spurious
negative facts (Norwegian and Red "sibling under Attribute" via
transitive closure). Dropping `(transitive is-a)` is *operationally*
correct without changing the logical conclusions of interest.

A rule-selection engine would:

1. Observe what the puzzle's `(query …)` actually asks.
2. Compute the minimal set of activators that derives the goal.
3. Skip activators whose conclusions are unused by the query
   (and not consumed by downstream rules).

This is a *goal-driven* alternative to the current eager
saturation. Probably needs the trace renderer (P1.6) to surface
which firings actually mattered.

Cross-cutting: a "rule-set sufficiency" check could also run
post-hoc on a saturated KB, flagging activators whose firings
were all redundant or unconsumed — a static-analysis pass on the
rule library.

## Relation to other followups

- [F1 — Categorical formulation](f1_categorical_formulation.md) —
  the rule-shape taxonomy maps to categorical structure
  (signatures, functors, natural transformations). F1 makes the
  mapping formal; F7 supplies the operational motivation.
- [F2 — Self-modifying constraint language](f2_self_modifying_language.md)
  — rung 1 of self-modification; F7 is upstream of it (the
  classification tells the language *what* to modify).
- [F4 — Cross-cutting](f4_cross_cutting.md) — Q37 (induction —
  rules from facts) and Q38 (LLM as fact/relation/type/rule
  extractor) overlap with sub-track B.
- [F5 — Rules as data](f5_rules_as_data.md) — rung 2 of
  self-modification; F5 mechanises *how* rules get rewritten, F7
  motivates *which* rewrites are worthwhile.
- [Idea 04 — NL → IR](../../docs/ideas/04-nlp-to-graph-to-solver-pipeline.md)
  — the M2 pipeline that consumes B.
- [Idea 07 — Categorical formulation](../../docs/ideas/07-categorical-formulation.md)
  — the formal underpinning for the *invariance-under-R* reading
  of square-fwd / square-unique.
- [Idea 10 — Generic self-modification](../../docs/ideas/10-generic-self-modification.md)
  — the umbrella for F2 / F5 / F6 and now F7.

## Connection to M2

If M2's goal is "fully automated model construction for NL problem
text", sub-track B (rule induction) is on the M2 critical path —
not a post-M3 followup. Without it, the NL pipeline produces
relations + facts but no activators; the engine sits idle. M2's
plan should call out F7 sub-track B as a deliverable, not a
parking-lot item.
