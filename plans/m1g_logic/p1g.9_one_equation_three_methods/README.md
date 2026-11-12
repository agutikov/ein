# P1g.9 — One equation, three methods: arithmetic read backwards, search, and algebra

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-12, on the user's reply of that day in round 2 of the calculator's questions
(`dc88520b`). Not started. No stage file yet: the phase is this README, and its stages are sections below.
The number 9 is the next free one after P1g.7 and P1g.8.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s stop; of M1f, the calculator's tables and
the probes banked under `examples/beyond_arith/`
([S1f.6.10](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.10_beyond_arithmetic.md)), and
[P1f.8's S1f.8.4](../../m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/s1f.8.4_linear_equations.md),
whose derivation of a linear equation is the third method as far as school algebra takes it.
**Read with:** [P1g.7](../p1g.7_unsatisfiable_against_owing/README.md), which rules what an unpayable debt is
called; this phase shows the same equation dead by one method and open by another, and decides nothing of
that.

## Where it comes from

The calculator's questions 24 and 37
([`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q24)) asked the same
thing twice. The user, 2026-11-12:

> solution of `x × 7 = 862` belongs yet to arithmetic or already to algebra? this question open vs. false is
> deep.

> Q24 already asked this, Is it possible at all to solve it by saturation without algebra rules that allows
> infer x*7=862 -> x=862/7 ? Again is it arithmetics or algebra?

[Round 2, items 24, 28 and 37](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q24)
answered in outline, *the question, "which x", is algebra's: it has an unknown. The methods differ*, set
out three methods, and proposed that P1f.8 compare them. The user's reply there (`dc88520b`):

> This Arithmetic read backwards vs Search vs Algebra comparison is somethinig like number theory and goes
> into separate phase in M1g. p1f.8 remain incomplete naive school algebra.

So the comparison is **this phase, in M1g,** and
[P1f.8](../../m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/README.md) stays what the
reply calls it, *incomplete naive school algebra*: it solves a linear equation the way a pupil does and
does not carry the comparison. The plan first put the comparison into P1f.8's S1f.8.4 (`d24e2656`); it was
moved here on this reply.

## The three methods

From round 2, with the figures of question 24 and of
[`beyond_arithmetic.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/beyond_arithmetic.md):

| | 1. arithmetic read backwards | 2. search | 3. algebra |
|---|---|---|---|
| what it is | the multiplication table is a relation, `(md A B CARRY-IN DIGIT CARRY-OUT)`, and a rule asks it *which A, with 7, gives a units digit 2*; the school multiplication run from the answer to the operand, a firing a column, with no rewriting and no search | each digit of x a hypothesis that owes one of ten values; the engine tries assignments | a rule rewrites the equation, x × 7 = 862 ⇒ x = 862 ÷ 7 with remainder zero, and the rest is arithmetic |
| built | run on the calculator's prototype, eleven rules; banked under `examples/beyond_arith/` | banked under `examples/beyond_arith/` | nothing of it was built when the question was asked; P1f.8's S1f.8.4 builds the linear case |
| `x × 7 = 861` | solved by saturation alone | solved; `--single` takes 10n + 1 saturations for n digits a column at a time | to be computed |
| `x × 7 = 862` | the last column cannot close: **a dead root** with a three-row core | **`Open`**: root is alive and owes the digits, every complete way of paying is dead, R is empty; its argument is 28 dead sets in the dump | to be computed: the division leaves a remainder |

The user's remark on question 24, that *"we would not generate hypothesis like (let x <number>) for every
numbers already present in KB"*, is why method 2 guesses digits, ten candidates a place, and never numbers
from the KB.

## Why it is M1g's, and *"something like number theory"*

The question *which x has x × 7 = 862* is a statement about the numbers with an unknown in it, and its
answer *none* is a small theorem: 7 does not divide 862. The three methods are three ways of establishing
such a statement, and M1g is where Ein's ways of establishing things are set side by side: saturation
against the lattice ([§ What this is](../README.md#what-this-is)), a derivation against a model. Method 1
is a decision by computation over the tables; method 2 is a search whose failure is an argument made of
dead sets; method 3 is a derivation in which each line follows from the last by a rule about equations,
which is the equational proof [P1g.3](../p1g.3_proof_systems/README.md) takes from P1f.8. Divisibility,
remainders and the congruences the columns compute are number theory's first objects, and S1f.6.10 already
ran primes, casting out nines and gcd by saturation; [P1g.5](../p1g.5_peano_and_induction/README.md) is
where *for every number* lives.

## The stages

### S1g.9.1 — The three methods on one set of equations

`x × 7 = 861`, `x × 7 = 862`, `x + 4567 = 127645`, and `3x + 4 = 19` with its degenerate neighbours (no
root, every number a root, a zero coefficient), each stated for each method that can state it, as kept
programs with answers computed by the binary and every surface each prints. The page says, for each
method, which equations it can state at all: method 1, one unknown and a one-digit multiplier or an addend
(a longer multiplier is division, which the calculator has); method 2, any equation an evaluator can
check, over a bounded domain; method 3, the shapes its rewriting rules know.

### S1g.9.2 — What *no solution* is under each

The user's *"open vs. false"*, shown and not decided: the dead root and its core of table rows; `Open` and
its dead sets; the derivation's remainder, which P1f.8's S1f.8.4 puts to the user as a result fact, a flag
or a dead root. One page, the three side by side on the same equation, with what each explanation is and
what a reader can check of it. The word for an unpayable debt is
[P1g.7's](../p1g.7_unsatisfiable_against_owing/README.md) ruling, and R is the same empty set under either
word.

### S1g.9.3 — What each costs, and where arithmetic ends

Saturations, firings and time for each method as the equation grows in digits, through
`utils/bench_env.sh`. And the user's question in its own terms, answered on the programs and not by
definition: method 1 uses no rule that mentions an unknown as such, only a table read in another
direction; method 3 is the first that rewrites a statement *about* x. Where a problem needs the third (two
unknowns, an unknown used twice) is measured, as S1f.8.4 measures when a word problem needs algebra.

### S1g.9.4 — Inverse requests, proposed or not

The calculator's question 37 proposed `(solve-add Y S)` and `(solve-mul M P)` in the stdlib; round 2 says
they are **not** added now, and the two banked examples stay examples. If the comparison shows method 1
earning a place, this stage puts a proposal to the user with the comparison as its evidence, and a module
follows only on a ruling. If it does not, the page says so.

### S1g.9.5 — As far as *"something like number theory"* goes

What of S1f.6.10's number-theory probes (divisibility, gcd, congruences, primes) reads as an inverse
question under the three methods, on one or two examples, and where the phase stops: a statement over
every number is P1g.5's, by induction or by a bounded check.

## What it is not

- **Not a change to P1f.8.** That phase remains *"incomplete naive school algebra"*; nothing here asks it
  for more.
- **Not the ruling on open against false.** P1g.7's.
- **Not a solver of equations in general,** and not engine arithmetic.
- **Not built before the user has seen the programs:** the phase follows P1g.1's stop, and any module is
  proposed on computed examples first.

## Acceptance

- The equations of S1g.9.1 are kept programs, each method's answer computed and held by a claim, and held
  to an independent check.
- The three readings of *no solution* are on one page, on the same equation.
- The cost table carries each figure's command and date.
- The question *arithmetic or algebra* has a page that answers it on the programs.
- Inverse requests are proposed to the user or declined, in writing.
- Nothing in the engine moved; if a probe meets a wrong answer it is a register row before its fix.

## The discipline

The [milestone's](../README.md#acceptance), and
[P1g.6 § The discipline](../p1g.6_engine_requirements/README.md#the-discipline) where the engine is met.
