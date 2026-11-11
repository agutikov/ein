# S1g.4.2 — Relation algebra: the stdlib's signature against Tarski's, a rule as an inclusion between relations, and properties detected from facts

**Phase:** [P1g.4](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.4.1](s1g.4.1_the_fragment.md).
**Pulls in:** [F1b](../../followups/f1b_logical_formulation.md)'s PFL.3 and its TODO;
[F15](../../followups/f15_math_formulae/ideas.md), rules exported as mathematical formulas;
[F12](../../followups/f12_rules_and_relations/ideas.md), a rule's match as a relation;
[F7](../../followups/f7_rule_induction.md)'s B′, properties from facts;
[F1](../../followups/f1_categorical_formulation.md) and [idea 07](../../ideas/07-categorical-formulation.md)
only as a link: relation algebra is where the logical and the categorical reading meet.

## What it asks

F1b's PFL.3: *"relation-algebra operations: declare `compose`, `intersect`, `union`, `complement` as activator
families; prove (or disprove) closure under composition for the rule set."* Most of the declaring is done:
[`std.algebra`](../../../stdlib/algebra.ein) says it ships Tarski's signature ⟨∪, ∩, ¬, 0, 1, ;, ⁻¹, 1'⟩ as
generic rules. So the stage is an **audit and a reading**, not a build: what is there, what each rule means
as a formula, what the algebra's laws say of the rules, and F1b's two open TODO lines.

## The four parts

**1. The inventory.** Each of Tarski's operations and constants, and each property of a single relation
(reflexive, irreflexive, symmetric, antisymmetric, asymmetric, transitive, connex, functional, injective,
total, surjective, difunctional, and the ones F1b's TODO names as missing: *coreflexive, etc.*), against the
stdlib activator that gives it, read off
[`07_stdlib_api.md`](../../../docs/kernel/ir/03-ein-lang/07_stdlib_api.md) and the module. For each: whether
it **derives** edges (a closure), **checks** (a restriction, `(false)` on violation), or **owes** (an
obligation); whether it is intrinsic or ranges over a typed universe; and whether it is sound open-world or
only for a closed relation. Gaps are listed, and F1b's *"converse - what? rule?"* is answered from the
module's own text. A gap is filled only if a later stage needs it; a new stdlib rule brings its test
program, as the gate requires.

**2. Each rule as a formula, in three forms** (F15). For every rule of `std.algebra`, by hand:

- the **logical** form, ∀-quantified: `R(x,y) ∧ R(y,z) → R(x,z)`;
- the **relational** form, an inclusion in the algebra: `R ; R ⊆ R`;
- the **algebraic** property it expresses, where it has a name: transitivity; or, in F15's generalisation,
  compatibility of an operation with an order.

F12's reading is the bridge between the first two: *a rule's `:match` defines a relation over its variables,
and its `:assert` says that relation is included in another*. The stage states it precisely for the
three-variable case and marks where it fails (a rule with `absent`; a rule that concludes `(false)`, which is
an inclusion in the empty relation; an obligation). Tarski and Givant's theorem is the frame the module's
header already cites: the algebra's equations express exactly first-order logic with three variables, so the
rules with more (`co-located`'s four, `adjacent-via`'s five) have no relational form, and the table shows
which stdlib rules those are. The table is the specification an automatic exporter would be tested against;
the exporter itself is [S1g.6.3](../p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md)'s.

**3. The laws.** PFL.3's *"prove (or disprove) closure under composition for the rule set"*, made concrete
and bounded:

- **The algebra's axioms as tests.** Each axiom of a relation algebra (associativity of `;`, the involution
  laws of converse, distribution over union, the Schröder equivalences, the identity laws) as a program: on
  small relations written as facts, both sides are built by the stdlib's activators and a claim says they
  are the same relation. On **every** pair or triple of relations over a two- or three-element set, generated
  by a Rust harness that also computes both sides itself. A bounded check of the axioms, so a check of the
  *rules*, not a proof about the algebra.
- **Closure of the rule set:** if two activators are composed (the relation one derives feeds another), is
  the result something a single activator states? The module's reflective lemmas (`converse-pair-symmetric`,
  `compose-contravariant`, `join-converse`) are instances. The stage lists which compositions the lemmas
  cover and which they do not, and does not try to prove completeness.
- **The closed-world caveat, tested:** `complement`, `difference` and `connex` read by failure. One program
  each showing the answer where the operand is determined by saturation, and one where it is decided by a
  hypothesis, with what *forbid*'s refusal
  ([§ 4.7](../../../docs/kernel/defined_behaviour.md)) says of the second.

**4. Properties detected from facts.** F1b's TODO: *"properties detection - are rules able to detect
properties from facts?"* A rule that derives `(symmetric R)` from R's extent has to say *every* edge has its
mirror: a universal over a closed extent, so `forall` over R, sound only once R is closed and complete. The
stage writes the detectors for the properties of part 1. A sketch, to be replaced by computed programs: run
once on a relation of two mirrored edges it derived `looks-symmetric`, and with one edge removed it derived
nothing; the read-out's `lattice` row said *monotone (the static check)*, the program having no hypothesis
relation (`ein solve`, release binary at `8860d616`, 2026-11-11).

```lisp
;; sketch: symmetry detected from a written relation's extent
(import std.macro :symbols (forall))
(relation looks-symmetric Relation :why "every {?1} edge has its mirror")
(relation detect Relation          :why "the properties of {?1} are asked for")
(rule detect-symmetric (?R)
  :match  (and (detect ?R)
               (forall ?a (?R ?a ?b) (?R ?b ?a)))
  :assert (looks-symmetric ?R)
  :why    "every {?R} edge has its mirror")
```

and establishes what the loader and the engine make of them: whether `forall` binds as the detector needs
for each property (it did for this one); whether the rule is flagged non-monotone once a commitment can
change the relation it reads, as it should be, and what *forbid* refuses if `looks-symmetric` feeds a refutation; and what the answer means when R is a
hypothesis relation (the property holds in some states of R and not others, which is a question about a
query's classes and not about the program). Then the honest summary for F7's B′: detection is *checking a
universal on a finished relation*, which Ein does by failure; *inducing* a property that then generalises to
unseen facts is induction of a rule and stays [F7](../../followups/f7_rule_induction.md)'s.

## Tasks

1. **T1g.4.2.1 — the inventory table,** with gaps.
2. **T1g.4.2.2 — the three-forms table** for every rule of `std.algebra`, and F12's reading stated.
3. **T1g.4.2.3 — the axioms as bounded tests,** the Rust harness and the programs.
4. **T1g.4.2.4 — the detectors,** tried; what loads, what is flagged, what is refused.
5. **T1g.4.2.5 — the page,** and the answers to F1b's TODO lines written into its banner.

## Acceptance

- Every operation and property has a row, with the stdlib rule or *absent*.
- Every `std.algebra` rule has its three forms, or the reason it has no relational one.
- Each algebra axiom holds on every relation tuple up to the stated size, by the Rust check; the count and
  the command are on the page.
- The detection question has an answer with programs: which properties, under which closure, with which
  refusals.
- No stdlib rule is added without its test program; no existing answer moves, or the move is named before
  any golden is blessed.

## What it may find

- **A stdlib rule that fails an axiom's test.** That is a wrong answer of the stdlib: a register row, and
  the fix with its test.
- **That `forall` cannot state a detector** as written (its guard's variables). Then the detector is written
  with `absent` directly, and the macro's limit is a row of the fragment table.
