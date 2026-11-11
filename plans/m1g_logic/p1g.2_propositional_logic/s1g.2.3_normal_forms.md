# S1g.2.3 — Normal forms as rewriting: NNF, CNF, DNF, and a connective defined by others

**Phase:** [P1g.2](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.2.1](s1g.2.1_formulas_and_evaluation.md) for formulas;
[S1g.2.2](s1g.2.2_valuations_as_hypotheses.md) for the equivalence check.

## What it asks

Whether Ein's rules rewrite a formula into a normal form, by saturation alone, and what the rewriting looks
like when the thing rewritten is a value and not the state. It is the third use of saturation in the phase:
after evaluating and before proving. It is needed twice later: a proof system is stated over a small basis of
connectives, so the full set has to be *defined away* ([S1g.3.1](../p1g.3_proof_systems/s1g.3.1_hilbert_systems.md)),
and resolution works on clauses ([S1g.3.5](../p1g.3_proof_systems/s1g.3.5_other_calculi.md)).

## The shape

Ein does not rewrite a fact in place: a state only grows. So a rewriting is a **function computed on
request**, like an arithmetic operation: `(nnf F)` answered by `(nnf-of F G)`, the rules recursing on F's
shape. A sketch, to be replaced by programs whose answers are computed. Run once, with two rules for letters added
(a letter and a negated letter are their own normal form): `(nnf (Not (And (Not (Not p)) q)))` was answered
`(Or (Not p) (Not q))`, no candidate (`ein solve`, release binary at `8860d616`, 2026-11-11). The rest of
the stage is to be tried:

```lisp
;; sketch: negation normal form on request, two of its rule pairs
(relation nnf Formula            :why "the negation normal form of {?1} is asked for")
(relation nnf-of Formula Formula :why "the negation normal form of {?1} is {?2}")

(rule nnf-double-negation ()               ; not not A: ask for A's
  :match  (nnf (Not (Not ?a)))
  :assert (nnf ?a)
  :why    "not not {?a} is {?a}")
(rule nnf-double-negation-answer ()
  :match  (and (nnf (Not (Not ?a))) (nnf-of ?a ?g))
  :assert (nnf-of (Not (Not ?a)) ?g)
  :why    "not not {?a} has {?a}'s normal form, {?g}")
(rule nnf-de-morgan-and ()                 ; not (A and B): ask for not A, not B
  :match  (nnf (Not (And ?a ?b)))
  :assert (and (nnf (Not ?a)) (nnf (Not ?b)))
  :why    "not ({?a} and {?b}) is (not {?a}) or (not {?b})")
(rule nnf-de-morgan-and-answer ()
  :match  (and (nnf (Not (And ?a ?b))) (nnf-of (Not ?a) ?g) (nnf-of (Not ?b) ?h))
  :assert (nnf-of (Not (And ?a ?b)) (Or ?g ?h))
  :why    "so its normal form is {?g} or {?h}")
```

What has to be worked out:

- **Telling a letter from a compound without a negative test.** `(nnf ?p)` for a letter answers itself; the
  rule needs `(atom ?p)` as a premise, which is why letters are declared
  ([S1g.1.3](../p1g.1_survey_and_design/s1g.1.3_conventions.md), choice 4).
- **A rule per shape.** NNF needs a rule pair for each connective under a negation and each at the top. The
  families differ by a connective and its dual, and one parametrised rule over a table `(dual And Or)` may
  replace them: to be tried.
- **CNF by distribution** grows a formula exponentially in the worst case; the stage measures the growth on
  a family that shows it and states the bound it tests to. The Tseitin transformation avoids the growth by
  introducing a new letter per subformula, which needs a name the program does not have: the subformula
  itself as the letter's name is the candidate ([Q-M1g.8](../open_questions.md#q-m1g8--fresh-names)).
- **A canonical form needs an order.** A DNF as a set of terms is unique only up to order and repetition.
  As a nested fact it has one order. Whether two equivalent formulas get the *same* normal form (so that
  equivalence is syntactic identity) needs sorting, hence a comparison of names, which Ein has no order
  predicate for. Expected: the theory gives *a* normal form, not *the* normal form, unless the letters'
  order is written as facts. To be tried.
- **Clauses as facts.** For resolution, a CNF is better as a set of facts, `(clause C)` and
  `(lit C p)` or `(lit C (Not p))`, than as one value: the stage writes the conversion and S1g.3.5 uses it.

## Tasks

1. **T1g.2.3.1 — define away:** `Iff`, `Imp`, `Or` in terms of a basis, as a request, for each basis P1g.3's
   systems use; held equivalent.
2. **T1g.2.3.2 — NNF,** with its program per rule.
3. **T1g.2.3.3 — CNF and DNF by distribution,** with the growth measured; and the clause-set form.
4. **T1g.2.3.4 — the checks.** In the Rust harness: each result is in the form it claims (a syntactic check
   in Rust) and equivalent to its source (the evaluator), for every formula up to the bound. In Ein: for a
   sample, S1g.2.2's equivalence program on source and result.
5. **T1g.2.3.5 — search-free and measured:** the search-free test's new rows; firings per formula size; the
   batch cost.

## Acceptance

- Each rewriting is a request answered exactly once, by saturation alone, held by the search-free test.
- Each result is in its form and equivalent to its source, on every formula up to the bound, held by the
  Rust check.
- The page states the growth of CNF with its numbers and command, and says whether the normal form is
  canonical.

## What it may find

- That a rewriting theory is where the cost of nested values is felt most: every rule matches a deep
  pattern and builds a new value. The numbers go to
  [S1g.6.2](../p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md).
- That two requests reach one subformula by different routes and each answers it: results must agree, and
  *answered exactly once* is the test that says so, as it is for the calculator.
