# S1g.4.3 — Formulas with quantifiers as data: terms, a finite structure, evaluation, and substitution

**Phase:** [P1g.4](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.2](../p1g.2_propositional_logic/README.md)'s formulas and evaluation;
[P1g.1](../p1g.1_survey_and_design/README.md)'s conventions.

## What it asks

The object level of first-order logic: a formula with variables and quantifiers as a value, what it means
over a structure, and the one operation every proof rule about quantifiers needs, substitution. The
instruction's *"what is nearly possible"* is expected to be mostly here.

## The representation

To be shown to the reader on computed programs before it is fixed (it follows S1g.1.3's conventions):

- **A term** is an object variable, a constant, or a function applied to terms. Constants and functions are
  the object language's and are spelt as ordinary nested facts, `(plus x (s z))`.
- **An object variable is a name,** `x`, declared by a fact `(var x)`. It is *not* one of Ein's `?x`: those
  belong to the metalanguage and range over formulas and terms. Keeping the two apart is the stage's first
  job, and the page says it in its first paragraph.
- **An atomic formula** is a predicate applied to terms, wrapped so that a rule can tell it from a
  connective without a negative test: `(At (loves x y))`, or a declared predicate, `(pred loves)`. To be
  chosen on examples.
- **A quantified formula** is `(All x F)` or `(Ex x F)`: the bound variable a name, the body a formula.
- **A structure** is given as facts: a finite domain, `(dom Ann)`, and each predicate's extent as facts
  about domain elements, `(true-of loves Ann Bob)`. An assignment of variables to elements is a nested list,
  `(bind x Ann REST)`, carried by the evaluation request.

## Evaluation over a finite structure

*Is F true in this structure under this assignment?*, on request, by saturation alone. A quantifier ranges
over the domain, and the domain is finite and written, so `(All x F)` is the conjunction of F's values with
x bound to each element. The theory writes no `absent`, so *for all* cannot be *there is no counterexample*:
it must **walk** the domain, as the calculator walks a number's digits. That needs the domain as a list the
walk can step through, written or built on request from the `dom` facts by an order the program states. A
sketch of the walk's middle, to be replaced by computed programs. It loads (release binary at `8860d616`,
2026-11-11) and has computed nothing: the ends of the walk and the atoms are not written.

```lisp
;; sketch only: (All x F) under assignment E is the conjunction over the domain's list
(relation All Var Formula                 :why "for every {?1}, {?2}")
(relation bind Var Element Assignment     :why "{?1} is {?2}, and {?3}")
(relation elems Element Elements          :why "{?1}, then {?2}")
(relation domain-list Elements            :why "the domain, listed: {?1}")
(relation sat Formula Assignment          :why "the value of {?1} under {?2} is asked for")
(relation sat-of Formula Assignment Bit   :why "{?1} under {?2} has the value {?3}")
(relation all-walk Var Formula Assignment Elements :why "{?2} is checked for {?1} over {?4}, under {?3}")
(relation all-walk-of Var Formula Assignment Elements Bit :why "over {?4}, every {?1} gives {?2} the value {?5}")
(relation t-and Bit Bit Bit               :why "{?1} and {?2} is {?3}")

(rule all-ask ()                           ; start the walk at the head of the domain's list
  :match  (and (sat (All ?x ?f) ?e) (domain-list ?d))
  :assert (all-walk ?x ?f ?e ?d)
  :why    "to value for every {?x}, {?f}: try each element")
(rule all-step-ask ()                      ; one element: ask for the body with x bound to it, and for the rest
  :match  (all-walk ?x ?f ?e (elems ?a ?rest))
  :assert (and (sat ?f (bind ?x ?a ?e)) (all-walk ?x ?f ?e ?rest))
  :why    "try {?x} as {?a}")
(rule all-step ()
  :match  (and (all-walk ?x ?f ?e (elems ?a ?rest))
               (sat-of ?f (bind ?x ?a ?e) ?v)
               (all-walk-of ?x ?f ?e ?rest ?w)
               (t-and ?v ?w ?u))
  :assert (all-walk-of ?x ?f ?e (elems ?a ?rest) ?u)
  :why    "with {?x} as {?a} it is {?v}, and for the rest {?w}")
```

To be settled: the end of the list (a name for the empty list, with value 1); looking a variable up in an
assignment (shadowing: the newest binding wins, which a walk from the front gives for free); `Ex` by the
`Or` table; a predicate's value read from the structure's facts, where a missing fact means false, which
again cannot be a negative test, so each predicate's extent is written with both values or the structure is
a table. And the cost: a formula with k nested quantifiers over a domain of d elements asks d^k
sub-requests. Measured.

**The structure as a hypothesis.** With a predicate's extent proposed by an hrule, R is the set of
*structures over the given names* that satisfy a required sentence: a finite model finder, by the same
construction as [S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md)'s truth table,
with 2^(d^arity) states a predicate. Shown on one small sentence (a relation that is irreflexive and total
on three elements, say) and then compared with **the same sentence written directly as Ein rules**, which is
what a zebra program is. The comparison is the point: the object-level detour evaluates, by a walk, what
Ein's own matcher does natively, and the page counts the difference. This is the first-order row of
S1g.1.2's hypothesis table.

## Substitution

`F[t/x]`, the term t for the free occurrences of x in F, on request: `(subst F x T)` answered by
`(subst-of F x T G)`. Recursion on F's shape; the cases that matter are the binders:

- under `(All x …)` with the same x: nothing is substituted;
- under `(All y …)` where y occurs in t: **capture.** The textbook renames y to a fresh variable. Ein makes
  no name ([Q-M1g.8](../open_questions.md#q-m1g8--fresh-names)). The options, each to be tried: **refuse**
  (derive a flag, as division by zero does, and no result; the proof rules then require *t free for x in
  F*, which is how many textbooks state them anyway); rename to a **built** variable, a nested fact such as
  `(prime y)`, which is a value and so a legal variable if variables may be values; or avoid names
  altogether with **de Bruijn indices**, variables as unary numbers counting binders, which removes capture
  and makes every formula harder to read.
- A variable *is not* another variable: the rule needs `(neq ?x ?y)` on object variables, which are names,
  so the guard is the engine's own.

Free variables, `(free-in x F)`, and *closed formula* are the companion requests.

## Tasks

1. **T1g.4.3.1 — the representation shown** on five formulas (a universal, an existential, an alternation,
   a shadowed variable, one with a function term), with the alternatives, and one chosen.
2. **T1g.4.3.2 — evaluation,** the walk, the search-free test, and a **Rust model checker** as the
   independent check: every formula up to a bound, over every structure up to a size, under every
   assignment.
3. **T1g.4.3.3 — the structure as a hypothesis,** one sentence, against the direct program, counted.
4. **T1g.4.3.4 — substitution and free variables,** with the three answers to capture tried, and the Rust
   check generating the capture cases on purpose.
5. **T1g.4.3.5 — the cost table,** candidates per request and firings against quantifier depth and domain
   size.

## Acceptance

- A quantified formula is evaluated over a finite structure by saturation alone, held by the search-free
  test, and agrees with the Rust model checker on every case up to the stated bounds.
- Substitution agrees with the Rust implementation on every generated case; a capturing substitution is
  never silently wrong: it is refused, renamed correctly, or impossible by construction, and the page says
  which was chosen and why.
- The model-finder program and its direct twin give the same R, and the oracle agrees where it reaches.
- Every number has its command.

## What it may find

- **That writing every predicate's false cases is unbearable,** and the theory wants one `absent` over a
  closed extent. Then the stage says what that costs: the theory is no longer monotone, and which of its
  programs *forbid* refuses.
- **That assignments as nested lists make every lookup a scan** of nested values. The numbers are S1g.6.2's.
- **The nesting limit:** de Bruijn indices and long assignments nest deeply; E-4's 1 000 is far away for
  formulas a person writes, and the page states the margin.
