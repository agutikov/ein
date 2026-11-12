# S1g.10.5 — Systems of two linear equations; quadratics with a rational root; inequalities, optional

**Phase:** [P1g.10](README.md) of [M1g](../README.md). Until 2026-11-13 this stage was **S1f.8.5** of M1f's P1f.8
([README § The move](README.md#the-move-of-2026-11-13)); nothing of its content was redesigned.
**Status:** planned 2026-11-11. Not started. The optional part runs only on the user's word.
**Depends on:** [S1g.10.4](s1g.10.4_linear_equations.md): the equation as a fact, the solver, the ruled outcomes
for *no solution* and *every number*; [S1g.10.2](s1g.10.2_expressions_and_evaluation.md)'s substitution;
[S1f.6.7](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.7_negative_numbers.md) and
[S1f.6.8](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.8_rational_numbers.md), without which almost no system and no quadratic has
an answer the theory can write.

## The instruction's part

> solving equasions

The school course goes on from one linear equation in two directions, and the stage follows both as far as
rational numbers reach.

## Systems of two linear equations

Two facts, `(equation L1 R1)` and `(equation L2 R2)`, in two unknowns, under a problem's name so that they are
read together. Both school methods, each a chain of rules with `:why`s:

- **Substitution:** solve the first for x as an expression in y (S1g.10.4's steps with a letter left on the
  right); substitute it into the second (S1g.10.2's `substitute`); that is one linear equation in y; solve it;
  put y back.
- **Elimination:** bring both to `a·x + b·y = c`; multiply each by the other's coefficient of x; subtract; the
  x-terms cancel in the normal form; solve for y; put y back.

**Both are derivations and neither searches,** but each has a *choice* a pupil makes and a rule cannot: which
unknown to eliminate, which equation to start from. Forward saturation takes every choice, so it derives the
solution by every route. Three things follow and each is checked:

- the routes must **agree** (a test: one value per unknown; a theory that derives two is wrong);
- the work is the sum of all routes, not one (measured);
- the **trace of the answer** follows one route, the one its first justification recorded, and which one that
  is depends on rule order (a core is an explanation, the user's ruling of 2026-11-08). The page shows the
  trace and says whose route it is.

Whether a program may *ask* for a method, `(solve-by elimination p1)`, so that the school solution shown is the
method the exercise names, is written as a request and tried.

**The degenerate systems** are S1g.10.4's two cases met in two dimensions: parallel lines, *no solution* (the
elimination ends in `0 = 5`); the same line twice, *infinitely many* (it ends in `0 = 0`), where the answer is
not *every pair* but the relation the pair satisfies, which the theory states as the equation it was left with.
They follow S1g.10.4's ruling.

**Search, for comparison:** both unknowns as digits, 100 pairs as candidates' combinations. The stage measures
it once and reports where it stops being reasonable; it is not built further.

## Quadratics

`a·x² + b·x + c = 0`, after S1g.10.3's normal form has brought it there.

**What rationals can do.** The formula needs the square root of the discriminant `D = b² − 4ac`, and Ein's
numbers are rationals at most. So:

| the discriminant | the school says | the theory |
|---|---|---|
| a perfect square (of a rational) | two rational roots, or one if D = 0 | derives them |
| positive, not a perfect square | two irrational roots, written with √ | says **the roots are not rational**, names D, and derives no number. Writing `(Sqrt D)` as an expression and computing with it is a quadratic number field, out of scope; the page says so with the program that stops |
| negative | no real root | says so |

**The square root is the new piece.** Is D a perfect square, and of what? Three ways, each tried on the
prototype and measured:

- **The school algorithm,** digit pairs from the left, a trial digit each step: a circuit, like long division,
  with the trial digit by bisection as S1f.6.6's is. Search-free. Expected to be the one that ships.
- **Run the multiplication table backwards:** `(product-of ?r ?r D)` cannot be asked, since the calculator
  answers requests and enumerates no numbers.
- **By hypotheses:** the root's digits as candidates, the square as the check. This is S1f.6.10's *unknown
  digits as hypotheses*, and the comparison with the circuit is the point.

**Factoring** is the same question asked another way: `x² + 5x + 6 = (x + 2)(x + 3)` is *find two numbers with
sum 5 and product 6*. For small integer coefficients a pupil finds them by trying the divisors of 6. That is a
finite search with a domain the problem supplies (the divisors of c, each a division with remainder zero), and
it is the phase's cleanest example of **a domain derived by saturation and then searched**: the divisor
candidates are facts a rule derived, and the hrule proposes among them. Whether an hrule can propose a
candidate whose content a rule derived at root is expected to work (M1g's probe 4 did the like) and is to be
tried here.

**Two roots are two facts, not two states.** `(root x 2)` and `(root x 3)` both hold at root: the equation has
both. That is not `(value x 2)` and `(value x 3)`, which S1g.10.4's search version makes contradictory. The page
sets the two readings side by side, since a word problem that leads to a quadratic (*a rectangle's sides differ
by 1 and its area is 6*) then needs the second step school teaches and often skips: **which root answers the
problem**, here the positive one, by a rule of the problem's theory with its `:why`.

**The check matters here.** Squaring both sides, or multiplying by an expression with x in it, can add a root
that is not one. The stage includes one such equation, `x/(x − 2) = 2/(x − 2)`, where the derivation gives
`x = 2` and the check fails on a division by zero: the calculator's ruling would kill root. The theory has to
evaluate the check where the denominator has been compared with zero first, and report *2 is not a root: it
makes a denominator zero*.

## Inequalities (optional)

`3x + 4 < 19`. Linear inequalities are S1g.10.4's derivation with one new rule, *dividing by a negative number
turns the sign*, and an answer that is not a value but a **ray**: `x < 5`. The theory would state the ray as a
fact, `(below x 5)`, and nothing enumerates its members. It is optional because the answer shape is new (a set,
described) and because the number line, intervals and their intersections are a subject. If the user wants it,
it is one task: a single linear inequality, and a pair joined by *and*.

## Sketch

**Sketch only, not run: to be replaced by programs whose answers are computed.** The shapes of a system and of a
quadratic's answer, in S1g.10.2's term language; the spellings are S1g.10.1's to fix, and negative numbers are
S1f.6.7's `(int P N)`.

```lisp
;; "x + y = 10 and x - y = 2."
(equation p1 (Plus  (var x) (var y)) (num (n (n z 1) 0)))
(equation p1 (Minus (var x) (var y)) (num (n z 2)))
(sought p1 x)
(sought p1 y)

;; what the theory is to derive, with the route in the trace:
;;   (value p1 x (n z 6))   (value p1 y (n z 4))

;; "x^2 - 5x + 6 = 0": both roots are facts at root.
(equation p2 (Plus (Minus (Pow (var x) (n z 2))
                          (Times (num (n z 5)) (var x)))
                   (num (n z 6)))
             (num z))
(sought p2 x)
;;   (discriminant p2 (n z 1))   (root p2 x (n z 2))   (root p2 x (n z 3))
```

## Tasks

1. **Systems:** both methods, the agreement test, the degenerate pairs, the request for a method.
2. **The square root:** the three ways on the prototype; the circuit that ships, as a module beside the
   calculator's (its place, `std.arith` or the phase's own module, is put to the user with S1g.10.1's question c).
3. **Quadratics:** the discriminant, the roots where it is a square, the stated outcome where it is not;
   factoring by divisors as a search over a derived domain.
4. **The check,** and the equation with an extraneous root.
5. **Inequalities,** if asked.
6. **The tests:** every system and every quadratic up to a bound of small coefficients against a Rust solver
   over exact fractions; one value per unknown; the search-free test for the derivations; the timeout.
7. **The examples,** corpus entries each with an `:expect`, among them two word problems: one that leads to a
   system, one to a quadratic with a root the problem rejects.

## Acceptance

- Two linear equations in two unknowns are solved by saturation alone, by both methods, with one value per
  unknown, agreeing with the Rust solver on the bound; the degenerate systems have the ruled outcomes.
- A quadratic whose discriminant is a perfect square has its roots derived; one whose discriminant is not has
  that said, and no number derived.
- The square root's three ways are on the page with their costs.
- An extraneous root is caught by the check and reported, and does not kill root.
- The page states where the school course goes on and the phase does not: irrational roots, complex roots,
  degree three.

## What it may find

- **That every route costs too much.** Saturation explores all of them. If two equations are affordable and
  three are not, that is the measured edge of *derivation without a strategy*, and it is a result for
  [P1f.10](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/README.md) and M1g's proof search to read, not a defect to fix here.
- **That the hrule over a derived domain does not do what is expected.** Then it is a finding with its witness,
  and factoring is shown by derivation alone: try each divisor by rules, which is a walk.
- **Engine findings,** to [`findings.md`](findings.md).
