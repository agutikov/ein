# S1f.8.4 — Linear equations in one unknown: the pupil's derivation, no solution and every number, derivation against search, and word problems that need an unknown

**Phase:** [P1f.8](README.md)
**Status:** planned 2026-11-11. Not started. **Stops for the user once,** before the degenerate cases are built:
what an equation with no solution, or with every number as one, *is*.
**Depends on:** [S1f.8.3](s1f.8.3_simplification_and_identities.md), the normal form each side is brought to;
[S1f.6.7](../p1f.6_calculator/s1f.6.7_negative_numbers.md) and
[S1f.6.8](../p1f.6_calculator/s1f.6.8_rational_numbers.md) for roots that are not natural numbers;
[S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md) for the word problems, whose theory this stage
extends and does not rewrite. It takes over the equations of
[S1f.6.10](../p1f.6_calculator/s1f.6.10_beyond_arithmetic.md), items 1 and 2.

## The instruction's part

> solving equasions

## What it is

### The derivation a pupil writes

*Do the same thing to both sides.* An equation is a fact, `(equation L R)`, over S1f.8.2's expressions. Solving
it is a chain of equations, each derived from the last by one rule with its `:why`:

| the school step | the rule, in outline |
|---|---|
| *expand the brackets and collect like terms on each side* | S1f.8.3's normal form of L and of R |
| *move the terms with x to the left and the numbers to the right* | subtract R's x-term and L's constant from both sides |
| *divide both sides by the coefficient of x* | the calculator's division, or S1f.6.8's fraction |
| *check: put the root back* | S1f.8.2's evaluation of both sides at the root, compared |

The trace of the run is then the solution as a textbook prints it, **if the trace can be read at the problem
theory's level**: probe A-1's trace is 123 steps of which 3 are these ([`findings.md`](findings.md) F-2). The
stage shows the three ways S1f.8.1 proposed and builds the one the user rules; until then its test holds the
three steps *among* the trace's.

It is **search-free** (A-1: no candidate, k_s 1, one saturation), and that is the point to make on the page: a
linear equation is not a puzzle. Every step is forced.

**The check is part of the solution, and it is a different thing.** The derivation shows that *if* x is a root
then x is 5. The check shows that 5 *is* a root. School teaches both and so does the program: the root is
substituted, both sides evaluated, and `(checked x)` derived. For a linear equation the steps are reversible
and the check never fails; S1f.8.5's quadratics and an equation with x in a denominator are where it does.

### No solution, and every number

After the terms are moved the equation is `a·x = d`:

| | the school says | what the theory must do | probe |
|---|---|---|---|
| a ≠ 0, a divides d | *x = d/a* | derive the root | A-1 |
| a ≠ 0, a does not divide d | *x = d/a*, a fraction; or, over the whole numbers, *no whole root* | with S1f.6.8, the root is a rational. Without it, the remainder is read by its shape, `(n ?p ?r)`, with no negation | A-2 |
| d would be negative | *x is negative* | with S1f.6.7, a root. Without it, `diff-of`'s second number says *short by*, and that is reported | — |
| a = 0, d ≠ 0 | *0 = 5: no solution* | **not divide.** The naive solver asks `(div d z)`, and the calculator's ruled division by zero kills root (A-3). So the theory compares a with zero first, as [`stdlib/arith.ein`](../../../stdlib/arith.ein) tells a program that must not die to do, and answers *no root* | A-3 |
| a = 0, d = 0 | *0 = 0: every number is a solution* | answer *every number*, and derive no value: there is no one value to derive | — |

**The question for the user** (e of the [README](README.md#the-questions-for-the-user)): what the two last rows
*are*. The calculator's ruling was about a program that **asks for something that has no value**: *"program
contains division by zero, why we would want other queries results?"*, so a flag and then `(false)`. An
equation with no root is different in kind, and the plan's default says so: the question *which x* has the
answer *none*, a result fact `(no-root x)` beside which the rest of the program's queries still answer. A dead
root is right for something else, a **problem whose facts conflict**: *Bob has x apples, gave 2 away, has 3, and
had 4*. The page shows the same four programs under the three options (a result fact; a flag then `(false)`; a
dead root outright) with every surface each prints, as S1f.6.6's record does for division by zero, and the user
rules.

### Where search belongs

The same equation solved by the lattice: the unknown's value is a candidate, the equation a restriction. Probe
A-4, over ten digits: 10 candidates, 9 dead alone, k_s 2, 11 saturations, and ∅ in R, since nothing in that
program says the unknown *has* a value. The stage builds it properly and compares:

| | by derivation | by search over a finite domain |
|---|---|---|
| what is needed | the equation's shape must be one the rules know | a domain small enough to enumerate, and an evaluator; **any** equation the evaluator can check |
| what it costs | one saturation | a saturation per candidate at least; the region limit past a few dozen values |
| no solution | a result fact, or as ruled | every candidate dead; R is {∅}, or empty if an obligation says the unknown has a value. Each dead candidate has a core that says why it is not the root |
| every number | stated | every candidate alive: R is all of them |
| the explanation | the school solution | *tried each; this one fits* |

The page says plainly which problems want which: a linear equation is derivation; *find a digit d such that …*,
a cryptarithm and S1f.6.10's unknown digits are search; a quadratic over a small range could be either, and
S1f.8.5 compares them. **An obligation that the unknown has a value** (the M1d mechanism, as `std.slots` uses
it) is what makes *no solution* show as an empty R under search; the stage writes it and shows the read-out
with and without.

### Word problems that need an unknown

[S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md) reads *gave* as a subtraction from a known quantity.
*Bob has some apples. He gave 2 away and has 3. How many had he?* has the same words and the unknown at the
start. Two ways, both tried:

- **The word-problem theory emits an equation.** *Has some* introduces `(var apples-before)`; *gave 2* and *has
  3* give `(equation (Minus (var apples-before) (num 2)) (num 3))`; this stage solves it. The reading of the
  words is S1f.6.20's rules unchanged in spirit, with an expression where a number was.
- **The word-problem theory inverts the operation itself:** a rule *what was had before is what is had now plus
  what was given*. No algebra, a rule per direction, as a younger pupil does it.

The stage reports which problems the second way stops covering (two unknowns, an unknown used twice), and that
is the measured answer to *when does a problem need algebra*.

**The two kinds of gap, again.** A missing quantity: *Bob has some apples and gave some away; how many had he?*
One equation, two unknowns. The answer names the gap as S1f.6.20 does and, here, can say more: the relation
between the unknowns that *is* known, `before = now + given`. A missing reading: *gave away* a number against
*gave away all*; S1f.6.20's readings as hypotheses, unchanged.

## Sketch

**Sketch only, to be replaced by programs whose answers are computed.** Probe A-1's program is in
[S1f.8.1](s1f.8.1_survey_and_design.md#sketch). This is probe A-4, the same equation by search, as the planning
agent ran it once on 2026-11-11: 10 candidates, 9 dead alone, N 9, k_s 2, R is ∅ and
`{(value x (n z 5))}`, 11 saturations; `:over maximal` reads `?v = (n z 5)`.

```lisp
(import std.arith :symbols (arith))

(relation digit Digit          :why "{?1} is a digit")
(relation sought Unknown       :why "{?1} is asked for")
(relation value Unknown Number :why "{?1} is {?2}")

(digit 0) (digit 1) (digit 2) (digit 3) (digit 4)
(digit 5) (digit 6) (digit 7) (digit 8) (digit 9)
(sought x)

(hrule guess ()
  :match  (and (sought ?x) (digit ?d))
  :assert (value ?x (n z ?d))
  :why    "suppose {?x} is {?d}")
(rule one-value ()
  :match  (and (value ?x ?a) (value ?x ?b) (neq ?a ?b))
  :assert (false)
  :why    "{?x} cannot be both {?a} and {?b}")

;; 3x + 4 = 19, checked at a supposed value
(rule times ()
  :match  (value x ?v)
  :assert (mul (n z 3) ?v)
  :why    "three times {?v}")
(rule plus ()
  :match  (and (value x ?v) (product-of (n z 3) ?v ?p))
  :assert (add ?p (n z 4))
  :why    "plus four")
(rule check ()
  :match  (and (value x ?v) (product-of (n z 3) ?v ?p) (sum-of ?p (n z 4) ?s)
               (neq ?s (n (n z 1) 9)))
  :assert (false)
  :why    "3 x {?v} + 4 is {?s}, not 19")

(solve :hrules (guess))
(query :goal (value x ?v) :over maximal :goal-text "x = {?v}")
```

Two things in it are to be looked at and not copied. `(n z 0)` is a candidate, which is zero with a high zero
and not the normal form `z`: the theory reads an operand by value, and the candidate's spelling is still wrong.
And `(neq ?s (n (n z 1) 9))` compares two nested values by `neq`; it answered as wanted here, and whether a
comparison request, `(cmp ?s …)`, should stand in its place (two spellings of one number are `neq` and `eq` by
`cmp`) is exactly the kind of error this phase's tests have to catch.

## Tasks

1. **The derivation:** the equation as a fact, the four steps as rules over S1f.8.3's normal form, the check.
2. **The stop:** the four degenerate programs under the three options, every surface shown; the user's ruling
   recorded in the user's words.
3. **The degenerate cases,** as ruled.
4. **The search version,** with the obligation, and the comparison table filled with measurements.
5. **The trace as the school solution** (F-2): what is built is what the user ruled at S1f.8.1 or here.
6. **The word problems with an unknown,** both ways, over S1f.6.20's theory, with the gap problems.
7. **The tests:** every root against a Rust solver over exact fractions on every equation up to a bound of
   coefficients; every degenerate case; the search-free test for the derivation; the search version held to
   the derivation's root and to `ein oracle`; the three trace steps.
8. **The examples,** corpus entries each with an `:expect`.

## Acceptance

- A linear equation in one unknown, written with brackets and terms on both sides, is solved by saturation
  alone, and its root agrees with the Rust solver on the bound.
- Each degenerate case has the outcome the user ruled, held by a test, and no equation kills root through a
  division by zero the solver could have foreseen.
- The same root is found by search over a finite domain, the costs are on the page, and the page says which
  kind of problem wants which.
- One equation's school solution is read off the run, and a test holds its steps.
- The word problems with an unknown answer; each gap problem names its gap.

## What it may find

- **That *no negation* does not survive the degenerate cases.** *The coefficient is not zero* is a comparison,
  which the calculator derives outright (`cmp-of`); the plan expects no `absent`. If one is needed, the algebra
  module is non-monotone where the calculator is not, and *forbid* may refuse a refutation resting on it. Said
  on the page either way.
- **That the equation and its unknowns want a problem's name,** so that two equations in one program do not
  share an `x` (S1f.8.2's finding, met in earnest here and needed by S1f.8.5).
- **Engine findings,** to [`findings.md`](findings.md).
