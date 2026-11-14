# P1g.10 — School algebra: expressions, equations, and the derivation a pupil writes

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day ([§ The instruction](#the-instruction)),
as the algebra half of M1f's P1f.8. Not started. It carries no estimate (the user's ruling of 2026-11-08). Its
first stage **stops for the user** with real programs computed on a prototype, before anything ships.
**Extended 2026-11-12** by the user's replies to the calculator's questions
([§ Additions of 2026-11-12](#additions-of-2026-11-12)): the expression layer is ruled in and moved here, and two
stages are added.
**Split off and moved to M1g on 2026-11-13,** on the user's instruction of that day
([§ The move](#the-move-of-2026-11-13)). The geometry is [P1g.11](../p1g.11_school_geometry/README.md).
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s stop, as M1g's phases do, and
[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md), the calculator. Specifically:

- **the four operations and comparison,** `std.math.decimal-arith`
  ([S1f.6.3](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md) to
  [S1f.6.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.6_division.md)),
  done 2026-11-11: every number here is the calculator's, `z` or `(n PREFIX DIGIT)`;
- **[S1f.6.7](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.7_negative_numbers.md), negative
  numbers:** `x + 5 = 2` has no answer without them, and a polynomial's coefficients are signed;
- **[S1f.6.8](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.8_rational_numbers.md), rational
  numbers:** `3x = 7`, and the gcd chain that puts a fraction in lowest terms;
- **[S1f.6.20](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md), word
  problems:** its two kinds of gap, its readings as hypotheses and its *the problem is a model* are the pattern
  this phase's word problems follow;
- **[S1f.6.14](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.14_expressions.md)'s ruling,** on
  whether Ein gets an expression layer (**ruled 2026-11-12:** both `(calc E)` and `(let NAME E)`, in this phase;
  [§ Additions of 2026-11-12](#additions-of-2026-11-12)).
  This phase cannot be written without expressions as *data*, so it is where that question's evidence comes
  from, and the ruling is taken at [S1g.10.1](s1g.10.1_survey_and_design.md)'s stop if it has not been taken
  before ([§ Expressions, and S1f.6.14](#expressions-and-s1f614));
- **[S1f.6.10](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.10_beyond_arithmetic.md)'s inverse
  questions,** which hand over here: that stage asks what a table run backwards and a digit guessed make of
  `x + 4567 = 127645`; this phase asks what a pupil's derivation makes of it.

**Blocks:** [P1g.9](../p1g.9_one_equation_three_methods/README.md), which needs
[S1g.10.4](s1g.10.4_linear_equations.md)'s linear equations; and of
[P1g.11](../p1g.11_school_geometry/README.md), its first stage, which reads
[S1g.10.1](s1g.10.1_survey_and_design.md)'s rulings on what the two phases share, and its last, which needs
S1g.10.4 for a figure's problem with an unknown. M1g's proof-system and induction phases are written so as not
to need it, and [§ What is left to M1g's other phases](#what-is-left-to-m1gs-other-phases) says where the line is.

**The number.** 10 was the next free phase number of M1g on 2026-11-13. A phase's number is not its place in
the order ([M1g § The order](../README.md#the-order)). M1f's number 8, which this phase and P1g.11 shared as
P1f.8 from 2026-11-11, is recorded there as used and vacated, and is not reassigned.

## The move of 2026-11-13

The user, 2026-11-13:

> plans/m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry seems too complex having both algebra
> and geometry, let's split it into 2 phases and move into m1g;
> plans/m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/s1f.8.7_theorems_as_rules.md  probably
> will require new features in the engine

So P1f.8 is two phases of M1g: this one, the algebra, and [P1g.11](../p1g.11_school_geometry/README.md), the
geometry. The files were moved with `git mv`, so each stage's history follows it. **Nothing of the content was
redesigned:** every target program, probe, task, acceptance line, risk, question and connection of P1f.8 is in
exactly one of the two phases, or, where it covered both halves, in both. The old path keeps a
[stub](../../m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/README.md), since pages that
hold the user's replies as typed link to it. The second sentence of the instruction is recorded on
[S1g.11.2](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md#the-users-note-of-2026-11-13).

| old id (M1f's P1f.8) | new id | phase |
|---|---|---|
| S1f.8.1, the survey and the design | [S1g.10.1](s1g.10.1_survey_and_design.md), its algebra half; its geometry half is folded into S1g.11.1 | P1g.10 and P1g.11 |
| S1f.8.2, expressions and evaluation | [S1g.10.2](s1g.10.2_expressions_and_evaluation.md) | P1g.10 |
| S1f.8.3, simplification and identities | [S1g.10.3](s1g.10.3_simplification_and_identities.md) | P1g.10 |
| S1f.8.4, linear equations | [S1g.10.4](s1g.10.4_linear_equations.md) | P1g.10 |
| S1f.8.5, systems and quadratics | [S1g.10.5](s1g.10.5_systems_and_quadratics.md) | P1g.10 |
| S1f.8.10, a program that states false facts | [S1g.10.6](s1g.10.6_a_program_that_states_false_facts.md) | P1g.10 |
| S1f.8.11, compound numbers without a hardcoded representation | [S1g.10.7](s1g.10.7_compound_numbers_without_a_hardcoded_representation.md) | P1g.10 |
| S1f.8.6, what school geometry is | [S1g.11.1](../p1g.11_school_geometry/s1g.11.1_what_school_geometry_is.md) | P1g.11 |
| S1f.8.7, theorems as rules | [S1g.11.2](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md) | P1g.11 |
| S1f.8.8, constructions, contradiction, cases | [S1g.11.3](../p1g.11_school_geometry/s1g.11.3_constructions_contradiction_cases.md) | P1g.11 |
| S1f.8.9, computation and the diagram gap | [S1g.11.4](../p1g.11_school_geometry/s1g.11.4_computation_and_the_diagram_gap.md) | P1g.11 |

### Choices made in the split

The instruction gave the two phases and the milestone. What it did not say was chosen as follows, the same
list in both READMEs. None is a ruling, and each is revisable.

1. **The survey's geometry half is folded into S1g.11.1.** S1f.8.1 surveyed both halves and was the one stop.
   Its problems 1 to 9, probes A-1 to A-8, findings F-1, F-2 and F-6 and questions a to d and f stay in
   S1g.10.1; its problems 10 to 14, probes G-1 to G-5, findings F-3 to F-5 and question g went to S1g.11.1,
   which already stopped for the user and is now P1g.11's stop. The problems keep their numbers.
2. **The order.** P1g.10 after P1g.1's stop, beside P1g.2. P1g.11 from S1g.10.1's stop on, beside the algebra's
   later stages, and its last stage after S1g.10.4: the order P1f.8 had inside itself. Both are placed before
   P1g.3, by the user's words of 2026-11-11, *"between calculator and proof systems"*; no stage of P1g.2 to
   P1g.5 waits for them ([M1g § The order](../README.md#the-order)).
3. **What the two phases share is ruled once, at S1g.10.1's stop:** where the theories live (question c) and
   how a program states its lattice. S1g.11.1 keeps the dependency S1f.8.6 had on S1f.8.1's rulings.
4. **No letter or id inside the phase was renumbered.** Questions a to f and j are this phase's and g to i are
   P1g.11's; probes A-1 to A-8 are here and G-1 to G-5 there; findings F-1, F-2 and F-6 are in this phase's
   [`findings.md`](findings.md) and F-3 to F-5 in [P1g.11's](../p1g.11_school_geometry/findings.md), and the
   next free id, F-7, is taken from one sequence over the two files.
5. **The report stays one report,** written by S1g.11.4, the last stage of the two phases to run, at
   `p1g.11_school_geometry/report.md`. It reads every stage of both.
6. **The questions will be filed as `Q-M1g.<n>`,** in M1g's [`open_questions.md`](../open_questions.md), where
   the plan said `Q-M1f.<n>`. None had been filed, so no question is renumbered.
7. **Two design pages and two prototype directories** where the plan had one of each:
   `p1g.10_school_algebra/design.md` over `build/s1g.10/`, and `p1g.11_school_geometry/design.md` over
   `build/s1g.11/`.
8. **An item that covered both halves is carried by both READMEs:** the user's instruction of 2026-11-11, four
   rows of *what it is not*, the discipline's acceptance lines, the risks *H is not empty by default* and
   *joins on nested values*, each with its own half's witness, the note on the stdlib's names, and the
   connections to P1f.6, M2, F15 and idea 08.
9. **§ What is left to M1g is now § What is left to M1g's other phases.** The handovers are the same; the
   phases that receive them are now siblings.
10. **A seventh provisional stage of P1g.6,**
    [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md), holds the candidate engine
    features the user's note on S1g.11.2 points at, as questions with witness programs. It designs nothing.

## The instruction

The user, 2026-11-11:

> I forgot another idea - between calculator and proof systems - middle school algebra and geometry: variables,
> expressions simplifications, solving equasions; middle school geometry contains proofs, but I'm not sure it is
> 100% complete formal proof system (it starts in 5th grade) - so just separate phase in M1f

So, as planned that day: one phase, in M1f, between the calculator and M1g's proof systems, with two halves.
Since 2026-11-13 the halves are two phases of M1g ([§ The move](#the-move-of-2026-11-13)). **This phase is the
algebra:** variables, simplifying expressions, solving equations. The geometry, and the honest question the
user raises about it, is [P1g.11](../p1g.11_school_geometry/README.md)'s.

## What it is

The calculator computes with numbers. M1g's other phases study proof as an object. School mathematics is what
lies between: the first place a pupil computes with **something that is not a number** (a letter, an
expression, a figure), and the first place a pupil is asked to **justify** each step. The phase writes the
algebra of that mathematics as Ein programs and asks of each piece which of Ein's two mechanisms does the work.

| the school activity | what it is in Ein, as planned | mechanism | stage |
|---|---|---|---|
| an expression with a letter in it | a nested fact over the calculator's numbers and names; the letter is a **name**, never an Ein `?x` | data | [S1g.10.2](s1g.10.2_expressions_and_evaluation.md) |
| *find the value of 3x + 4 when x = 5* | substitution, then the calculator, on request | saturation, search-free | S1g.10.2 |
| *simplify*, *expand the brackets*, *collect like terms* | rewriting on request toward a polynomial normal form | saturation, search-free; to be shown | [S1g.10.3](s1g.10.3_simplification_and_identities.md) |
| *prove the identity (a+b)² = a² + 2ab + b²* | both sides have one normal form | saturation | S1g.10.3 |
| *solve 3x + 4 = 19* | a derivation, a rule a step, each with its `:why`: the school solution is the trace | saturation, search-free | [S1g.10.4](s1g.10.4_linear_equations.md) |
| the same over a small finite domain | the unknown's value as a hypothesis, the equation a restriction | the lattice | S1g.10.4 |
| a system of two equations, a quadratic | substitution, elimination; factoring where the discriminant is a perfect square | saturation; a quadratic's two roots are two facts, not two states | [S1g.10.5](s1g.10.5_systems_and_quadratics.md) |

The table's other five rows, a figure and what is given about it, a proof, a construction, a computation in a
figure and what is read off the picture, are [P1g.11's](../p1g.11_school_geometry/README.md#what-it-is).

## What it is not

- **Not a computer algebra system.** No factoring of general polynomials, no symbolic integration, no
  simplification heuristics. A normal form for polynomials with rational coefficients, linear equations, two
  equations in two unknowns, and quadratics with a rational root: the school course to about the 8th grade, and
  where it stops is stated with a program.
- **Not engine arithmetic.** [Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)
  stands: no numbers, no ranges, no arithmetic predicate. Every number is the calculator's nested fact, and every
  computation a request the imported theory answers.
- **Not a language of function calls.** The user, 2026-11-11: *"Ein is not Lisp, is it?"*. An expression here is a
  thing the program *talks about*, as a formula is in M1g's logic; it is not a call that returns.
- **Not new syntax.** Every program is written in the grammar as it ships. A grammar change needs the user's
  ruling on real programs shown first, and none is planned.
- **Not natural-language parsing.** A problem's English is a `:source` string and a comment; its facts are
  written by hand. The sentence beside the facts is recorded, since that pair is what
  [M2](../../m2_nl_to_ir/README.md) would have to produce.
- **Not proof theory.** What a proof *is*, soundness, completeness, natural deduction and induction are M1g's
  logic phases'. This phase writes derivations the way a pupil does and reports what kind of object each turned
  out to be.

## Expressions, and S1f.6.14

The calculator's rulings left one thing open: the expression layer, `(calc E)` and `(let NAME E)`
([`design.md` § The rulings](../../m1f_hypothesis_and_documentation/p1f.6_calculator/design.md#the-rulings-2026-11-11)).
The user found it questionable, and
[S1f.6.14](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.14_expressions.md) waits. Algebra meets
the same shape from the other side and the two must not be confused:

- **S1f.6.14's question** is whether a program may *compute* by writing a tree: is `(calc (add N1 (div N2 N3)))`
  a second way to say what request facts and rules already say?
- **This phase's need** is that a program *reasons about* a tree: `3x + 4` has no value, it has a *shape*, and
  simplifying it or solving for `x` reads that shape. It is data, in the way a formula is data in
  [M1g](../README.md) (*a logic written in Ein*, its second level).

The spelling shows the difference and avoids a clash. `add`, `sub`, `mul` and `div` are the calculator's request
relations and mean *asked* once the theory is imported flat
([`stdlib/math/decimal-arith.ein`](../../../stdlib/math/decimal-arith.ein) § The names this module occupies). An expression's heads are
other ordinary relations. This plan writes them capitalised, `Plus`, `Minus`, `Times`, `Over`, `Neg`, `Pow`, as
M1g writes its connectives `And` and `Imp`
([Q-M1g.1](../open_questions.md#q-m1g1--the-spelling-of-the-connectives)); the user ruled for the
gates that any spelling will do, and one convention across the milestone is the reason for this one. It is
revisable at S1g.10.1's stop.

**What S1g.10.1 puts to the user is therefore one question with two parts,** on programs it has computed:
whether expression *data* of this kind is accepted (the phase cannot run otherwise), and whether, once an
evaluator over that data exists as an imported module ([S1g.10.2](s1g.10.2_expressions_and_evaluation.md)), it
**is** the answer to S1f.6.14: a program that wants a tree writes `(evaluate E)` over data, and the calculator's
core stays requests and results. If S1f.6.14 has been ruled before this phase starts, the phase follows that
ruling and asks only the first part.

## What a variable is

Three things are called a variable and the phase keeps them apart, on one page of
[S1g.10.2](s1g.10.2_expressions_and_evaluation.md):

| | written | what it is |
|---|---|---|
| **Ein's pattern variable** | `?x` | a slot in a rule's pattern. It is bound by a match and exists in no state. It ranges over values |
| **an object-level variable** | the name `x`, inside `(var x)` | a name of the program, like `Bob`. It can be assigned a value, `(assign x N)`, or be the unknown of an equation |
| **an unknown** | `(sought x)` | an object-level variable the problem asks for. Its value is a derived fact, or a hypothesis |

A rule that says *like terms add* is `?`-quantified over object-level variables: `(term ?a ?v)` matches `2x` with
`?v` bound to the name `x`. That is the same two levels
[S1g.1.2](../p1g.1_survey_and_design/s1g.1.2_two_levels.md) sets apart for formulas.
(The word `unknown` itself is taken: it is a macro of `std.macro`, and a fact `(unknown x)` is refused at load,
[§ First probes](#first-probes-2026-11-11), A-5.)

## The stages

| ID | was | title | depends on | stops for the user |
|---|---|---|---|---|
| [S1g.10.1](s1g.10.1_survey_and_design.md) | S1f.8.1 | The survey and the design: the algebra on a prototype, every answer computed, the conventions and the questions | S1f.6.6; S1f.6.7 and S1f.6.8 for the programs that need a sign or a fraction | **yes: the phase's stop.** Nothing outside `plans/` and `build/` is written before the user has read its page |
| [S1g.10.2](s1g.10.2_expressions_and_evaluation.md) | S1f.8.2 | Expressions as data, what a variable is, and evaluation under an assignment | S1g.10.1's rulings; S1f.6.14's ruling or its taking here | no |
| [S1g.10.3](s1g.10.3_simplification_and_identities.md) | S1f.8.3 | Simplification as rewriting to a polynomial normal form; equality of expressions; identities proved by normal forms | S1g.10.2; S1f.6.7, S1f.6.8 | no; one choice **for the user** (the normal form's order, with S1g.10.1) |
| [S1g.10.4](s1g.10.4_linear_equations.md) | S1f.8.4 | Linear equations in one unknown: the pupil's derivation, no solution and every number, derivation against search, and word problems that need an unknown | S1g.10.3; S1f.6.20 | **yes, once:** what *no solution* and *every number* are (a flag, a dead root, a result fact) |
| [S1g.10.5](s1g.10.5_systems_and_quadratics.md) | S1f.8.5 | Systems of two linear equations; quadratics with a rational root; inequalities, optional | S1g.10.4 | no; the optional part runs on the user's word |
| [S1g.10.6](s1g.10.6_a_program_that_states_false_facts.md) | S1f.8.10 | A program that states false facts, `2 + 2 = 5`: what happens today, and whether an existing theory already answers it (the user, 2026-11-12) | the calculator as shipped | no; its page is **shown** |
| [S1g.10.7](s1g.10.7_compound_numbers_without_a_hardcoded_representation.md) | S1f.8.11 | Compound numbers without a hardcoded representation: integers and rationals over binary or decimal naturals, *"or even not numbers"* (the user, 2026-11-12) | `std.math.int`, `std.math.rat`; binary arithmetic in the stdlib, or its banked examples | **yes:** the directions on a prototype, before any shipped module moves |

No stage carries an estimate. Engine findings go to [`findings.md`](findings.md).

## The order

1. **S1g.10.1 first, and it stops.** As S1f.8.1 it showed both halves on one page, so that the user could rule
   the shared conventions once. It still rules them once: where the theories live and how a program states its
   lattice are settled here, and [S1g.11.1](../p1g.11_school_geometry/s1g.11.1_what_school_geometry_is.md),
   the geometry's stop, follows them.
2. **S1g.10.2 to S1g.10.5, in order.** Each uses the one before: evaluation needs expressions, a normal form
   needs evaluation as its check, an equation is solved on normal forms, and a system is two equations.
3. **The geometry runs beside them.** [P1g.11](../p1g.11_school_geometry/README.md)'s first three stages use no
   algebra and may run as soon as S1g.10.1's rulings are in. They share no file with S1g.10.2 to S1g.10.5. Its
   last stage, S1g.11.4, runs after S1g.10.4, since a geometry computation with an unknown (*one angle is twice
   another*) is S1g.10.4's equation over the geometry's facts, and its report reads every stage of both phases.
4. **With the additions.** S1g.10.6 is independent and may run at any time. S1g.10.7 runs after the calculator's
   binary build, beside the algebra.

Against the rest of M1g: after P1g.1's stop, as every phase of the milestone is, and after the calculator's
extensions and word problems, since it imports them; beside [P1g.2](../p1g.2_propositional_logic/README.md),
of which it needs nothing; and before [P1g.3](../p1g.3_proof_systems/README.md), by the user's words of
2026-11-11, *"between calculator and proof systems"*, though no stage of P1g.3 waits for it.
[P1g.9](../p1g.9_one_equation_three_methods/README.md) waits for S1g.10.4. It shares no file with M1f's
[P1f.10](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/README.md) or
[P1f.20](../../m1f_hypothesis_and_documentation/p1f.20_docs_refactor/README.md) but the kernel pages P1f.20
moves, which it does not edit.

## First probes (2026-11-11)

Scratch programs the planning agent ran once while writing this tree, on the release binary found in
`ein.rs/target/release/` that day (built 2026-11-11 17:59; not rebuilt), with `ein solve FILE` unless said
otherwise. They are **not** results: nothing is banked, no test holds them, and each is **re-taken** by the stage
named, as a kept program. They are here so that the plan's expectations are told apart from its guesses. Each
program is reproduced as a sketch in the stage that re-takes it. The geometry's probes, G-1 to G-5, are in
[P1g.11's table](../p1g.11_school_geometry/README.md#first-probes-2026-11-11).

| # | the program | what the binary said | re-taken by |
|---|---|---|---|
| A-1 | `3x + 4 = 19` as `(linear x 3 4 19)`, three rules of the program over `std.math.decimal-arith`: take the constant from both sides, divide by the coefficient, read the quotient | `(value x (n z 5))`; no candidate, k_s 1, R {∅}, 1 saturation; the claim holds under `ein test`. `--trace` writes **123 steps, of which 3 are the pupil's** (steps 2, 19 and 123); the other 120 are the calculator's columns | [S1g.10.4](s1g.10.4_linear_equations.md) |
| A-2 | the same with 18 for 19, and a rule that reads a remainder `(n ?p ?r)` | `(no-root x)` derived with no negation: a non-zero normal-form number always has the shape `(n P D)`, so *the remainder is not zero* is a pattern | S1g.10.4 |
| A-3 | the same with the coefficient `z`: `0·x + 4 = 19` | **root is dead:** the naive solver asks `(div 15 z)`, and the calculator's ruled division by zero derives `(false)`. `refuted by division-by-zero-is-false`; the core is the equation and three table rows. A solver must compare the coefficient with zero before it divides | S1g.10.4 |
| A-4 | `3x + 4 = 19` by search: an hrule proposes `(value x (n z D))` for each of ten digits, a rule kills two values of one unknown, a rule kills a value whose left side is not 19 | 10 candidates, **9 dead alone**, N 9, k_s 2: ∅ and `{(value x (n z 5))}`, 11 saturations. ∅ is in R because nothing says the unknown has a value; `:over maximal` reads the one answer | S1g.10.4 |
| A-5 | a fact `(unknown x)` | refused at load: `unknown` is a macro of `std.macro`, and a macro's call is no fact ([`defined_behaviour.md` § 4.24](../../../docs/kernel/defined_behaviour.md)). The plan writes `sought` | [S1g.10.2](s1g.10.2_expressions_and_evaluation.md) |
| A-6 | `(simplify (Plus (term 2 x) (term 3 x)))`, two rules: ask the calculator for the sum of the coefficients, read it | `(simpler … (term (n z 5) x))`; search-free | [S1g.10.3](s1g.10.3_simplification_and_identities.md) |
| A-7 | `(evaluate (Plus (Times (num 3) (var x)) (num 4)))` with `(assign x 5)`, eight rules: requests go down the tree, values come up through `mul` and `add` | `(value-of … (n (n z 1) 9))`; search-free. Its rules join an expression with its parts' values, so they break the calculator's rule of the theory | S1g.10.2 |
| A-8 | A-1 with the line `(solve :hypothesis-relations ())` taken out | **exit 1**, one line: *a state's commitments pass 2^128: the state route cannot count them*. The program's own relations are candidates for the blind enumerator over every name the tables hold. Every program of this phase states its lattice | [S1g.10.1](s1g.10.1_survey_and_design.md); [`findings.md`](findings.md) F-1 |

## Acceptance

- **S1g.10.1's page was read and ruled on by the user** before any file outside `plans/` and `build/` was written.
- **Each row of [§ What it is](#what-it-is) has a program whose answer the binary computed,** or the page that
  owns it says *this could not be done* and why, with the program that shows it.
- **Search-free where it should be, held by a test:** evaluation, simplification to normal form and the
  derivation of a linear equation's root have no candidate, k_s 1 and R {∅}, as the calculator's programs do
  (`ein-cli/tests/arith_search_free.rs` is the model). Where a stage uses the lattice (search over a domain),
  the page says so and what it cost. (The forward proof of each target theorem is
  [P1g.11's](../p1g.11_school_geometry/README.md#acceptance) half of this line.)
- **Right, held against an independent check written in Rust:** every value, every normal form and every root
  agrees with a check that shares no code with the Ein theory, on every expression and equation up to a stated
  bound; two expressions have one normal form exactly when the check says they are equal as polynomials.
- **The trace is the school solution:** for one linear equation at least, the steps a pupil would write are
  read off the run, each a rule with its `:why` rendered, and a test holds them. What stands between today's
  trace and that reading (A-1's 3 steps in 123) is a row of [`findings.md`](findings.md) with a ruling, a fix
  or an owner. (The same for one geometry theorem is P1g.11's half of this line.)
- **No gap is silently filled.** A degenerate equation is reported by name and is not assumed away. (A missing
  quantity and a missing fact of the figure are P1g.11's.)
- **The report** states which fragment of school algebra is a formal derivation in Ein today, and hands the
  rest to M1g's other phases by name. It is one report over both phases, written by
  [S1g.11.4](../p1g.11_school_geometry/s1g.11.4_computation_and_the_diagram_gap.md), and the line is held there.
- **The repo's discipline:** a wrong answer found in the engine is a row of
  [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml) before it is fixed; every other engine
  finding is a row of [`findings.md`](findings.md) with a ruling, a fix or an owner at the phase's close (the
  user's ruling of 2026-11-11: a phase keeps what it finds, and nothing goes to F25); every claim on a page
  carries its command, its date or the test that pins it; what ships passes the gates it joins (a corpus entry
  per `.ein`, an `:expect` in each test program, a `:why` on every relation and rule, the stdlib's coverage and
  mutation checks if a module goes to `stdlib/`).
- `./run_tests.sh` green at every stage boundary, and every golden a stage moves named in its record before it
  moves.

## Risks, each with what checks it

- **Rewriting floods.** `a + b = b + a` as a forward rule derives every rearrangement of every expression in the
  state. The calculator's answer is the phase's: **demand-driven** (a request, `(simplify E)`, answered by a
  result fact), and **directed** (every rule moves toward the normal form, never both ways). S1g.10.3 states the
  measure that decreases, and a test runs each theory under a timeout that fails loudly, since the engine has
  no word for a saturation that does not stop (M1g's C-1).
- **The natural rules join on nested values.** *Value the two parts, then add* (A-7) joins an expression with
  its parts' values: the join the calculator's rule of the theory forbids, quadratic in a batch (E-3 of the
  calculator's [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)). One
  problem is small and does not notice. The phase **measures** each theory in a batch on the matcher's
  counters, and either rewrites it with tags, as the calculator did, or says that it is quadratic and why that
  is accepted. It does not build the index: that is
  [S1g.6.2](../p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md).
- **The trace buries the solution.** 3 steps in 123 (A-1). A school solution is the problem theory's steps with
  the arithmetic folded. Whether a trace can be read at one theory's level is the second row of
  [`findings.md`](findings.md), F-2, and the phase's headline acceptance depends on it.
- **H is not empty by default** (A-8). Every program states its lattice or closes its relations, and the
  search-free test holds it.
- **Scope.** Inequalities are one optional part. (Trigonometry, circles, similarity, areas and coordinates are
  the geometry's scope, [P1g.11's](../p1g.11_school_geometry/README.md#risks-each-with-what-checks-it).)

## The questions for the user

They are asked at the stops named, on computed programs. They were not filed as `Q-M1f.<n>` when the phase was
planned, since P1f.6 was filing its own; since the move the phase files them in M1g's
[`open_questions.md`](../open_questions.md), as `Q-M1g.<n>` with the ids then free, when it starts. The letters
are P1f.8's and were not renumbered: g, h and i are [P1g.11's](../p1g.11_school_geometry/README.md#the-questions-for-the-user).

| | the question | asked at | the plan's default |
|---|---|---|---|
| a | *(its second half **ruled 2026-11-12**: the expression layer, `(calc E)` and `(let NAME E)`, is this phase's, [§ Additions](#additions-of-2026-11-12); how it is spelt and whether it is one thing with `(evaluate E)` is still asked)* **Is expression data accepted, and is an evaluator over it the answer to S1f.6.14?** ([§ Expressions, and S1f.6.14](#expressions-and-s1f614)) | S1g.10.1 | data yes; `(evaluate E)` as an imported module is S1f.6.14's (b)-like answer, and no `calc` in the calculator's core |
| b | **The spelling of expression heads,** `Plus` `Times` …, shared with M1g's connectives | S1g.10.1 | capitalised |
| c | **Where the theories live:** `stdlib/` (`std.algebra` is taken by the relation-algebra module, so another name), or `examples/`, as S1f.6.20 also asks. Asked once, for both phases | S1g.10.1 | `examples/` first; a module moves to `stdlib/` when a second program imports it |
| d | **The normal form's order** of monomials and of variables | S1g.10.1, for S1g.10.3 | descending degree, variables in the order of the name rule's text |
| e | **What an equation with no solution, or with every number as one, is:** a result fact (`(no-root x)`, `(every-number x)`), a flag and then `(false)` as division by zero is, or a dead root | S1g.10.4 | a result fact. An equation with no root is an *answer*, and a program that states one is not contradictory; `(false)` is for a problem whose own facts conflict |
| f | **Is the school solution a view of the trace?** A read-out that shows one theory's steps and folds an imported theory's | S1g.10.1 (shown), ruled at S1g.10.4 | a finding with an owner; the phase proposes and does not build a `--trace` option without a ruling |
| j | **Inequalities:** in or out | S1g.10.5 | out unless asked |

## Additions of 2026-11-12

The user replied to the calculator's questions on 2026-11-12
([`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md), with what stayed open in
[`questions_round2.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md); the index
of every reply is
[`replies_2026-11-12.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/replies_2026-11-12.md)). Four
of the replies sent work to P1f.8, and all four are the algebra's, so they are here. Where a reply says *p1f.8*
it is quoted as typed; the phase it names is this one since 2026-11-13.

**1. Expressions, `(calc E)` and `(let NAME E)`, moved here from S1f.6.14.** Reply 10:

> both go into p1f.8, where expressions naturally belong to, with check of used names, with check of recursive
> name usage (let a (+ a 1)) and other

Both are ruled in, in this phase and not in the calculator's core. They are
[S1g.10.2](s1g.10.2_expressions_and_evaluation.md#moved-here-from-s1f614-2026-11-12)'s, with the two checks the
reply names, a used name nothing states and a name defined through itself, and the *"and other"* the stage
proposes and shows first. [§ Expressions, and S1f.6.14](#expressions-and-s1f614) above was written while the
question waited; of its two parts, *whether a program may compute by writing a tree* is now answered yes, and
what is left for S1g.10.1's stop is the spelling and whether `calc` and `evaluate` are one layer.

**2. A program that states false facts.** Reply 7:

> (a), btw, `2 + 2 = 5` is interesting question - what happens to the program that states false facts? If it
> is not solved already by-product in existing theories - I expect it belongs to either p1f.8 or M1g

[S1g.10.6](s1g.10.6_a_program_that_states_false_facts.md): seven ways of writing *2 + 2 = 5*, each computed,
and the answer to the reply's *if*. What it finds that is a proof system's (a false axiom, *ex falso*) it
hands to [P1g.3](../p1g.3_proof_systems/README.md).

**3. Compound numbers without a hardcoded representation.** Reply 14, in full:

> a is ambiguous, add2 looks like add with 2 operands etc. Take (b), but ints and rats - different question.
> binary and decimal is numbers representation, integers, rationals, ... complex - are numbers types based on
> naturals of some representation. Add new stage into p1f.8 to investigate how to make compound numbers
> without hardcoded representation so can switch between binary and decimal, and maybe later use any new
> natural numbers there, or even not numbers...

[S1g.10.7](s1g.10.7_compound_numbers_without_a_hardcoded_representation.md), an investigation that stops for
the user. It runs once binary arithmetic is in the stdlib, or on its banked examples.

**4. Three methods for `x × 7 = 862`: not here.** The comparison of arithmetic read backwards, search and
algebra is [P1g.9](../p1g.9_one_equation_three_methods/README.md), by the user's reply in
round 2 (`dc88520b`): *"This Arithmetic read backwards vs Search vs Algebra comparison is somethinig like
number theory and goes into separate phase in M1g. p1f.8 remain incomplete naive school algebra."* The phase
those last words name is this one.

**The order, with the additions.** S1g.10.6 is independent and may run at any time. S1g.10.7 runs after the
calculator's binary build, beside the algebra.

**Names.** The calculator's modules are under `std.math` since [S1f.6.31](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md)
(2026-11-14): `std.math.decimal-arith`, `std.math.int` and `std.math.rat`, which this plan names; before that stage they
were `std.arith`, `std.int` and `std.rat`. The relations kept their names.

## What is left to M1g's other phases

So that the phases do not overlap (the section was *What is left to M1g* while the phase was M1f's; the
geometry's four entries are [P1g.11's](../p1g.11_school_geometry/README.md#what-is-left-to-m1gs-other-phases)):

- **Equational reasoning as a proof system.** S1g.10.3 proves an identity by computing two normal forms. That is
  a decision procedure, not a proof. A proof by *equational steps*, each an axiom of a commutative ring applied at
  a position, checked line by line, is a proof system and belongs with
  [P1g.3](../p1g.3_proof_systems/README.md)'s checker. This phase hands it the normal-form
  procedure as the semantics such a system would be checked against.
- **For all numbers.** An identity is proved for all values by normal forms, which is algebra and not induction.
  A statement whose proof needs induction (a sum formula, divisibility for every n) is
  [P1g.5](../p1g.5_peano_and_induction/README.md)'s.
- **One equation by three methods.** The comparison of arithmetic read backwards, search and algebra is
  [P1g.9](../p1g.9_one_equation_three_methods/README.md)'s, which reads S1g.10.4.

## Connections

- **[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md):** the numbers, the request
  protocol, the search-free test, the harness against Rust's integers, the rule of the theory and its reason,
  the ruling on division by zero, and
  [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)'s E-3 and E-12, which
  this phase's rules meet head on.
  [S1f.6.10](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.10_beyond_arithmetic.md) hands over
  its equations; [S1f.6.20](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md)
  its gaps and readings;
  [S1f.6.14](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.14_expressions.md) its question.
- **[P1g.11](../p1g.11_school_geometry/README.md):** the other half of what was P1f.8. The two meet where a
  figure's problem has an unknown (S1g.11.4 over S1g.10.4), where a theorem is a sum with letters for the
  angles (target 4 of S1g.11.2, over S1g.10.3's normal forms), in an order on names wanted twice (question d
  here, the canonical spelling there), and in the trace read at one theory's level (F-2 here, the two-column
  proof there).
- **M1g's other phases:** [§ What is left to M1g's other phases](#what-is-left-to-m1gs-other-phases);
  [P1g.8](../p1g.8_a_theory_of_types/README.md) reads S1g.10.7.
- **[M12](../../m12_rules_as_data/README.md):** rules as data cites S1g.10.7.
- **[M2](../../m2_nl_to_ir/README.md):** each solved problem is an instance of
  [§ The reasoning as a program](../../m2_nl_to_ir/README.md#the-reasoning-as-a-program-the-users-idea-2026-11-11):
  the school solution as a self-explained Ein program, every step checked by the kernel. A school problem book
  is a benchmark with gold answers and gold *reasoning*.
- **[F15](../../followups/f15_math_formulae/ideas.md):** rules exported as formulas. An algebra rule *is* a
  formula, `a(b + c) = ab + ac`. This phase's rule base is among the first where the exported formula is the
  thing the rule was written from, and S1g.10.3 records the formula beside each rule, in its `:why` or a
  comment, so that F15 has a corpus.
- **[Idea 08](../../ideas/08-human-style-deductive-trace.md):** the human-style trace. The acceptance line *the
  trace is the school solution* is that idea with a ground truth: a textbook's worked solution.
