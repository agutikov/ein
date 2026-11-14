# S1g.10.3 — Simplification as rewriting to a polynomial normal form; equality of expressions; identities

**Phase:** [P1g.10](README.md) of [M1g](../README.md). Until 2026-11-13 this stage was **S1f.8.3** of M1f's P1f.8
([README § The move](README.md#the-move-of-2026-11-13)); nothing of its content was redesigned.
**Status:** planned 2026-11-11. Not started.
**Depends on:** [S1g.10.2](s1g.10.2_expressions_and_evaluation.md), the term language and the evaluator, which is
this stage's check; [S1f.6.7](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.7_negative_numbers.md), signed coefficients;
[S1f.6.8](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.8_rational_numbers.md), fractional ones. A first cut over natural-number
coefficients and no subtraction may run on S1f.6.6 alone, and says what it leaves out.

## The instruction's part

> variables, expressions simplifications

## What it is

A pupil is told *simplify*, and the word hides a definition: there is a form every expression can be brought to,
and two expressions are equal exactly when they are brought to the same one. For polynomials that form exists
and is computable. The stage writes it, as a theory that rewrites **on request** and **in one direction**, and
then uses it three times: to simplify, to decide equality, and to prove an identity.

### The normal form

- **One variable.** A sum of monomials `c·xᵏ` in descending degree, no two of one degree, no zero coefficient;
  the zero polynomial is the number zero. Written as the calculator writes a number, a nest with the leading
  term outermost or innermost (the choice is measured, as *reading order* was for digits), each coefficient in
  its own normal form: an integer simplified (S1f.6.7), a fraction in lowest terms (S1f.6.8).
- **Several variables.** A monomial is a coefficient and a product of powers, its variables in one fixed order;
  the monomials in one fixed order (by total degree, then lexicographically: the graded order a schoolbook uses
  without naming it). **Both orders need an order on names,** and Ein has none built in: no premise compares two
  names. So the order is data: either the program states it, `(var-order a b)`, or the theory carries a table
  for the letters it admits, as the calculator carries `cl` for digits. This is question d of the phase's
  [README](README.md#the-questions-for-the-user), and the stage shows both on the same identity.
- **What is not in it:** a fraction of polynomials is brought to one polynomial over another and no further
  (no cancelling of common factors, which is polynomial gcd); a root; a power with a letter in the exponent.

### How it rewrites

The risk is the phase's first ([README § Risks](README.md#risks-each-with-what-checks-it)): a rule set that
rewrites both ways floods under forward saturation. `a + b → b + a` alone, run forward, derives every
rearrangement. So:

- **Demand-driven.** `(simplify E)` is a request; `(simpler E F)` its answer, F in normal form. Nothing is
  derived about an expression nobody asked about. Sub-requests are made by rules, as the calculator's circuits
  make theirs.
- **Not rewriting at all, where it can be avoided.** The plan's expectation, to be tried against a rule-by-rule
  rewriter: *convert, then compute.* Each leaf becomes a polynomial in normal form (a number, or one monomial);
  `Plus` becomes a **merge** of two sorted polynomials, which is the calculator's addition walk with monomials
  for columns; `Times` becomes the school multiplication of polynomials, a row per monomial, each row merged
  into the sum; `Neg` and `Minus` negate coefficients; `Pow` with a written exponent is repeated `Times`. This
  is a circuit: it terminates because it walks the tree once, and it is confluent because it is a function.
- **A rewriter, beside it, for the trace.** *Expand the bracket*, *collect like terms*, *put in order*, a rule a
  school step, each with a `:why`. This one is what a pupil writes, and it is the one whose termination and
  confluence are real questions. The stage states the measure that decreases for each rule (brackets, then
  inversions of the order, then count of terms), holds on every expression up to the bound that it reaches the
  circuit's normal form, and reports a critical pair where two rules apply to one expression: forward
  saturation takes **both**, so a non-confluent rule set shows as two `simpler` answers for one request, which a
  test can see. That is a property of saturation worth stating: it does not choose a strategy, it explores all
  of them, so confluence is observable and non-termination is a flood.

**Whether simplification is search-free** is then answered separately for the two: the circuit is expected to
be (no candidate, R {∅}); the rewriter is search-free too, since exploring every rewrite order is saturation and
not the lattice, and its cost is the number of intermediate expressions, to be measured.

### Equality, and identities

- **`(same E F)`** is asked, and answered by `(same-as E F yes)` or `(same-as E F no)` from the two normal
  forms. The *no* is derived without negation in the way the calculator's comparison is, by walking the two
  normal forms side by side to the first difference; the stage does not use `(neq ?p ?q)` on two nested values
  until it has measured what that costs and whether the value rule and S1f.5.7's `neq` read nested values as
  wanted (to be tried).
- **An identity** is `(same E F)` answered *yes*: `(a + b)² = a² + 2ab + b²`, the difference of squares, the
  cube of a sum, the distributive law itself. It is **proved for all values**, since equal normal forms are
  equal polynomials. It is not a proof by equational steps: there are no lines. What the program gives a reader
  is the two computations of the normal forms. A line-by-line equational proof, checked, is P1g.3's
  ([README § What is left to M1g's other phases](README.md#what-is-left-to-m1gs-other-phases)).
- **The honest caveat, stated on the page:** *equal as polynomials* and *equal at every value* agree over the
  rationals; the stage says so and cites it, and does not prove it. The Rust check compares both ways on the
  bound: normal forms equal, and values equal on enough points.

## Sketch

**Sketch only, to be replaced by programs whose answers are computed.** The planning agent ran this once on
2026-11-11 (probe A-6): the binary answered `(simpler (Plus (term (n z 2) x) (term (n z 3) x)) (term (n z 5) x))`,
no candidate, k_s 1. It is one rewriter rule, *collect like terms*, in a spelling older than S1g.10.2's term
language (`(term C x)` for `c·x`), and it handles exactly the expression shown.

```lisp
(import std.math.decimal-arith :symbols (decimal-arith))

(relation term Number Name   :why "{?1} of {?2}")
(relation Plus Expr Expr     :why "{?1} plus {?2}")
(relation simplify Expr      :why "asked: {?1} made simpler")
(relation simpler Expr Expr  :why "{?1} is {?2}")

;; "Simplify 2x + 3x."
(simplify (Plus (term (n z 2) x) (term (n z 3) x)))

(rule like-terms ()
  :match  (simplify (Plus (term ?a ?v) (term ?b ?v)))
  :assert (add ?a ?b)
  :why    "both terms are of {?v}: add {?a} and {?b}")
(rule like-terms-added ()
  :match  (and (simplify (Plus (term ?a ?v) (term ?b ?v))) (sum-of ?a ?b ?s))
  :assert (simpler (Plus (term ?a ?v) (term ?b ?v)) (term ?s ?v))
  :why    "{?a} of {?v} and {?b} of {?v} are {?s} of {?v}")

(solve :hypothesis-relations ())
(query :goal (simpler ?e ?r)
       :expect (model (simpler (Plus (term (n z 2) x) (term (n z 3) x)) (term (n z 5) x))))
```

A normal form as planned, **not run**, for `3x² + 2x + 5`, most significant term outermost as the calculator's
accumulator is; the spelling is S1g.10.1's to fix:

```lisp
(poly x (mono (n z 3) (n z 2)                 ; 3 x^2
        (mono (n z 2) (n z 1)                 ; 2 x^1
        (mono (n z 5) z                       ; 5 x^0
        end))))
```

## Tasks

1. **The normal form,** for one variable, then for several, with its orders as ruled.
2. **The circuit:** conversion, merge, product, negation, powers; tagged as the calculator's circuits are if
   S1g.10.2's measurement says tags are needed.
3. **The rewriter:** the school steps as rules, the decreasing measure stated for each, and the comparison with
   the circuit on every expression up to the bound.
4. **Equality and identities:** `same`, and the identities of the school course as corpus programs, each with
   an `:expect`.
5. **The tests:** normal forms against a Rust polynomial check that shares no code with the theory; the
   search-free test; every rewriter answer equal to the circuit's and **one** per request; a timeout that fails
   loudly on every program; the batch measure.
6. **The formulas beside the rules.** Each rewriter rule carries the identity it applies, `a(b + c) = ab + ac`,
   in its `:why` or in a comment beside it (a rule takes no `:source`), so that [F15](../../followups/f15_math_formulae/ideas.md) has rules whose formula is
   known.

## Acceptance

- Every expression of the term language up to the bound has **one** normal form, computed by saturation alone,
  agreeing with the Rust check.
- `(same E F)` answers *yes* exactly where the check says the polynomials are equal, and *no* otherwise, with
  no `absent` in the theory (or the page says where one was needed and what *forbid* makes of it).
- The identities `(a ± b)²`, `(a + b)(a − b)`, `(a + b)³` and the distributive law are each a program whose
  claim holds.
- The rewriter terminates and agrees with the circuit on the bound, or the page names the expression on which
  it does not.
- The page says what the normal form does **not** do, each with a program: `(x² − 1)/(x − 1)`, a root, a letter
  in an exponent.

## What it may find

- **That an order on names is wanted in the engine.** It is not asked for: a table or a declared order is the
  plan. If both turn out heavy, the row goes to [`findings.md`](findings.md) and the user decides.
- **That the rewriter floods on a small expression.** Then the page shows the flood with its count, keeps the
  circuit as what ships, and shows the school steps another way: as the circuit's own steps with school `:why`s.
- **That a comparison of two nested values without negation is long.** The calculator's comparison was a table
  and a walk; a comparison of polynomials is a walk of walks.
