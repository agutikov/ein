# Self-describing Ein — the kernel's model, in ein-lang

> **Status: part operational, part design.** This file expresses Ein's
> own model — the four levels of
> [`../01-ein-graph/05_four_level_kb.md`](../01-ein-graph/05_four_level_kb.md) —
> *in ein-lang itself*, using **only forms the loader already accepts**
> (the closed declarator set + facts). Much of it is **operational
> today**: the `is-a` kind lattice (L0), relation signatures that rules
> read (L2), and the entire relation-property algebra
> (`symmetric` / `bijective` / `converse` / …) are Ein features *defined
> in Ein* and reasoned about *by Ein*. The kernel-purity passes
> ([S1.7.23](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.23_retire_kernel_type_system.md),
> [S1.7.24](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.24_dehardcode_symmetric.md))
> put them there on purpose. The one piece that stays **design-only** is
> L3-on-L3 — rules that match or generate rules — which is
> [F5](../../../../plans/followups/f5_rules_as_data.md)'s work.

## 0. The goal — EBNF written in EBNF

The question this file answers (author's framing, preserved):

> Investigate how to describe Ein IR concepts in ein-lang itself — *like
> an EBNF grammar defined in EBNF*. Only valid existing Ein syntax
> allowed. Basic kinds as `(object OBJECT) (object RELATION) …`,
> `(is-a RELATION OBJECT)`, `(relation rel RELATION OBJECT FACT)` … ; then
> develop the representation, **express a feature of Ein in Ein, and do
> inference / reasoning about Ein in Ein.**

The constraint — *only valid existing syntax* — is the whole point: a
self-description that needs a new metalanguage is no longer Ein
describing itself. It also corrects the first instinct above:
`(object …)` is **not** a declarator (§1), so the kinds are named the
way Ein names everything else — as atoms in an `is-a` lattice (§2). The
pay-off is the homoiconic root of
[`../01-ein-graph/03_ein_model.md` §1](../01-ein-graph/03_ein_model.md) —
*instance is an instance of instance* — which §2 reaches in real syntax.

## 1. Why `(meta …)` with keyword slots is the wrong tool

An earlier sketch declared the levels with a keyword sub-language —
`(level L0 :name objects :nodes atoms :inhabits L1)` and similar
`:shape` / `:declares` / `:slots-from` / `:acts-on` clauses. **None of
that is ein-lang.** Ein's surface is a *closed declarator set*
([`06_reserved_names.md`](06_reserved_names.md) §declarators) —
`relation` / `rule` / `hrule` / `query` / `config` / `macro` / `import` —
plus one rule: **any other head is a fact.** There is no `meta`, no
`level`, no free-form keyword schema.

So the sketch does not fail loudly — it loads as *something else*.
`(meta …)` is a head outside the set, hence a **fact** named `meta`;
each `(level L0 :name objects :nodes atoms …)` is a fact named `level`
with one positional arg `L0`, and the loader's `_fact_args` **silently
drops** every unrecognised `:keyword value` pair. The whole
`:name / :nodes / :inhabits / :shape / …` payload evaporates at load;
what survives is the inert skeleton
`(meta (level L0) (level L1) (level L2) (level L3))`. A schema that
vanishes when loaded describes nothing.

The lesson fixes the rules for the rest of this file: to describe Ein in
Ein you have exactly **(a) the closed declarators** and **(b) facts over
user-named relations**. That is the entire toolbox — and it is enough.

## 2. The four levels, in real ein-lang

### L0 — the kinds are atoms in an `is-a` lattice

Ein has no `(object X)` form because it needs none: an object *is* any
atom that fills an argument slot, and "X is one of these" is said with
`is-a` — itself an ordinary relation a puzzle declares
([`zebra2.ein`](../../../../examples/zebra2.ein) does exactly this):

```lisp
(relation is-a T T)          ;; the membership relation, declared like any other
```

The author's `(object RELATION)` / `(is-a RELATION OBJECT)` instinct is
right; only the spelling changes — drop the non-existent `object`
declarator, keep the `is-a` facts. Ein's own node-kinds become atoms
under a common top (`03_ein_model` §1: every kind *is an object* because
every node is a graph vertex):

```lisp
(is-a Object   T)
(is-a Relation Object)       ;; a relation node is an object   ← author's (is-a RELATION OBJECT)
(is-a Fact     Object)       ;; facts are first-class nodes
(is-a Rule     Object)       ;; rules are first-class nodes
(is-a Sort     Object)       ;; a "type" is just an object used in a signature / is-a slot
```

### the homoiconic line

`is-a` was declared `(relation is-a T T)`, so `is-a` is *itself* a
Relation — and that is sayable, because `is-a` is not reserved:

```lisp
(is-a is-a Relation)         ;; the membership relation classifies itself
```

With `(is-a Relation Object)` above, the transitive-closure rules
`(includes is-a is-a*)` + `(transitive is-a*)` — *also* ordinary ein
rules ([`std.algebra`](../../../../ein.py/src/ein/stdlib/algebra.ein)) —
derive `(is-a* is-a Object)`: the membership relation is, by its own
machinery, an object. That is `03_ein_model` §1's fixed point —
*instance is an instance of instance* — reached with nothing but
`relation`, `is-a` facts, and the stdlib closure rules. **Inference
about Ein, performed by Ein.**

### L1 / L2 / L3 — fact, relation, rule

| level | what it is | the real ein form |
|-------|------------|-------------------|
| **L1 fact** | a Relation applied to Objects | `(color-loc Red House-1)` |
| **L2 relation** | a Relation node + its argument Sorts | `(relation color-loc Color House)` |
| **L2 property** | a Property tag on a Relation | `(bijective color-loc)`, `(symmetric next-to)` |
| **L3 rule** | a `:match` → `:assert` rewrite | `(rule symmetric (?R) :match (?R ?a ?b) :assert (?R ?b ?a))` |

Crucially, **L2 is reflexively matchable**: `relation` is kept a plain
symbol precisely so a rule body can pattern-match a *declaration* —
`(relation ?R ?A ?B)` is a legal premise
([`06_reserved_names.md`](06_reserved_names.md) — "kept a plain SYMBOL so
rules can pattern-match `(relation ?R ?A ?B)`"). Relations are data the
rules read. That one fact makes §3 operational rather than aspirational.

## 3. What already self-describes — shipped in M1

Three of the four levels describe Ein features *in Ein* and let Ein
*reason about them*, today, with no F5 machinery:

**Relation signatures are data rules consume.** Because `(relation ?R ?A
?B)` matches, real rules read the schema and act on it — the bijection
typecheck reads each declaration's arg sorts
([`std.bijection`](../../../../ein.py/src/ein/stdlib/bijection.ein)
`typecheck-setup`, `functional-negative`), the relation-algebra converse
check reads *two* signatures to reject an ill-typed inverse
([`std.algebra`](../../../../ein.py/src/ein/stdlib/algebra.ein)
`converse-illtyped-dom`), and zebra2's `disjunctive-prune` reads
`(relation ?R2 ?A ?B)` to bound a partner's domain. The L2 layer is
introspected by L3 rules in the live engine.

**The property algebra *is* Ein-in-Ein.** `symmetric`, `transitive`,
`functional`, `injective`, `total`, `surjective`, `bijective`,
`converse`, `compose`, `meet`, `join`, `reflexive` … are **not kernel
primitives** — each is a plain fact whose meaning is supplied by an
ordinary ein rule:

```lisp
(symmetric next-to)                       ;; a fact …
(rule symmetric (?R)                      ;; … given meaning by a rule
  :match  (?R ?a ?b)
  :assert (?R ?b ?a))
```

`std.algebra` implements *"Tarski's ⟨∪, ∩, ¬, 0, 1, ; , ⁻¹, 1'⟩ … as
generic ein rules on the existing matcher"*; `std.bijection` adds the
closed-world inference (elimination, negative-completion, typecheck).
The kernel-purity passes were exactly the act of *moving Ein's type
system and relation properties out of the engine and into ein-lang*:
[S1.7.23](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.23_retire_kernel_type_system.md)
retired the built-in type system (`is-a` / `T` became ordinary data),
[S1.7.24](../../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.24_dehardcode_symmetric.md)
de-hardcoded `symmetric` (symmetry now lives entirely in the user rule).
Self-description of these features is not a future deliverable — it is
the current architecture.

**The quantifier sugar is Ein-in-Ein too.** `forall` and `open` are not
kernel forms; they are `(macro …)` declarations in
[`stdlib/macro.ein`](../../../../ein.py/src/ein/stdlib/macro.ein):

```lisp
(macro forall (?b ?G ?B) (absent (and ?G (absent ?B))))
(macro open   (?P)       (and (absent ?P) (absent (not ?P))))
```

## 4. The abstract syntax of Ein, as Ein relations (descriptive)

Everything in §3 is *operational* — the engine acts on it. The author's
"EBNF in EBNF" also has a purely **descriptive** reading: write Ein's
grammar productions as ein relation declarations. They name the shape of
each level; no M1 rule consumes them (they are documentation that
*parses*) — the honest analogue of a grammar that describes but does not
execute. `Sort`, `Pattern`, `Property` here are atoms (Objects, §2):

```lisp
(relation fact-of   Relation Object)   ;; a Fact applies a Relation to Object slots
(relation sig-of    Relation Sort)     ;; a Relation has argument Sorts
(relation prop-of   Property Relation) ;; a Property tags a Relation
(relation match-of  Rule Pattern)      ;; a Rule has a :match pattern …
(relation assert-of Rule Pattern)      ;; … and an :assert pattern
```

§5 (F5) is what turns this tier from descriptive into operational: once
rules can match `(rule …)` nodes, these productions become the schema a
validator checks against — or a rule-generator emits against.

## 5. The real frontier: rules as data (F5)

The boundary is sharp. **L0, L1, L2 reify; L3 does not.** A rule can
match a fact `(?R ?a ?b)`, a relation `(relation ?R ?A ?B)`, and a
property `(symmetric ?R)` — but **not** another rule: there is no
`(rule ?N …)` node a `:match` can bind, and a rule may not `:assert` a
new `(rule …)` or `(relation …)`. The downward-consuming stack
([`05_four_level_kb.md` §4](../01-ein-graph/05_four_level_kb.md)) is a
clean DAG *only* because M1 forbids the L3→L3 and L3→L2 arrows.

| concern | operational in M1 | F5 |
|---------|-------------------|----|
| L0 kinds as `is-a` atoms | ✅ ordinary facts | — |
| L2 signatures read by rules | ✅ `(relation ?R ?A ?B)` matches | — |
| L2 property algebra | ✅ facts + std.algebra / std.bijection rules | — |
| quantifier sugar | ✅ std.macro | — |
| rules match rules | ✗ | matcher extension |
| rules assert relations / rules | ✗ | engine + M1-invariant change |

That last pair is the genuine self-modifying core — rule induction
([F7](../../../../plans/followups/f7_rule_induction.md)) generating rules
against the §4 schema; the self-modifying-language loop
([F2 / idea 01](../../../../plans/ideas/01-self-modifying-constraint-language.md)).
M1's job here is done once it has (a) shown three levels already
self-describe and (b) written the L3 schema down precisely enough for F5
to target.

## Open questions

- **Does `relation` need a self-declaration?** `(relation relation …)`
  is rejected — the loader reserves the name `relation`
  ([`06_reserved_names.md`](06_reserved_names.md), `_reserved_names`).
  So `relation`'s own signature can only be *described* (§4), never
  declared in-language. Gap, or correct — the one bootstrap primitive a
  language cannot define in itself?
- **One top, or per-kind tops?** §2 hangs every kind under `Object` / `T`.
  The open class of virtual node kinds
  ([`../01-ein-graph/01_kb.md` §8](../01-ein-graph/01_kb.md)) may want
  more than one top. Decide when a puzzle forces it.
- **Reflexivity.** `(is-a is-a Relation)` is already a fixed-point edge;
  F5 rules that match rules will close more of these. The level model
  survives — "consumes any level, including its own"
  ([`05_four_level_kb.md` §4](../01-ein-graph/05_four_level_kb.md)).

## See also

- [`../01-ein-graph/05_four_level_kb.md`](../01-ein-graph/05_four_level_kb.md)
  — the L0–L3 schema this file expresses in ein-lang.
- [`../01-ein-graph/03_ein_model.md`](../01-ein-graph/03_ein_model.md)
  — the reflexive root (§1) that makes a self-describing schema coherent.
- [`06_reserved_names.md`](06_reserved_names.md) — the closed declarator
  set + the reserved-name guard that bound this file's toolbox.
- [`std.algebra`](../../../../ein.py/src/ein/stdlib/algebra.ein) /
  [`std.bijection`](../../../../ein.py/src/ein/stdlib/bijection.ein) /
  [`std.macro`](../../../../ein.py/src/ein/stdlib/macro.ein) — Ein
  features already written in Ein (§3).
- [F5 — rules as data](../../../../plans/followups/f5_rules_as_data.md)
  — the implementation half (§5).
- [F1b — logical formulation](../../../../plans/followups/f1b_logical_formulation.md)
  — the formal-fragment sibling.
