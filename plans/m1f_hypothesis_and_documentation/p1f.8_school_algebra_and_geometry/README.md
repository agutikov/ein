# P1f.8 — Middle-school algebra and geometry: expressions, equations, and the proofs a pupil writes

**Milestone:** [M1f](../README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day ([§ The instruction](#the-instruction)).
Not started. It carries no estimate (the user's ruling of 2026-11-08). Its first stage **stops for the user**
with real programs computed on a prototype, before anything ships.
**Depends on:** [P1f.6](../p1f.6_calculator/README.md), the calculator, and runs after it. Specifically:

- **the four operations and comparison,** `std.arith`
  ([S1f.6.3](../p1f.6_calculator/s1f.6.3_addition.md) to [S1f.6.6](../p1f.6_calculator/s1f.6.6_division.md)),
  done 2026-11-11: every number here is the calculator's, `z` or `(n PREFIX DIGIT)`;
- **[S1f.6.7](../p1f.6_calculator/s1f.6.7_negative_numbers.md), negative numbers:** `x + 5 = 2` has no answer
  without them, and a polynomial's coefficients are signed;
- **[S1f.6.8](../p1f.6_calculator/s1f.6.8_rational_numbers.md), rational numbers:** `3x = 7`, and the gcd chain
  that puts a fraction in lowest terms;
- **[S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md), word problems:** its two kinds of gap, its
  readings as hypotheses and its *the problem is a model* are the pattern this phase's word problems and
  geometry computations follow;
- **[S1f.6.14](../p1f.6_calculator/s1f.6.14_expressions.md)'s ruling,** on whether Ein gets an expression layer.
  This phase cannot be written without expressions as *data*, so it is where that question's evidence comes
  from, and the ruling is taken at [S1f.8.1](s1f.8.1_survey_and_design.md)'s stop if it has not been taken
  before ([§ Expressions, and S1f.6.14](#expressions-and-s1f614));
- **[S1f.6.10](../p1f.6_calculator/s1f.6.10_beyond_arithmetic.md)'s inverse questions,** which hand over here:
  that stage asks what a table run backwards and a digit guessed make of `x + 4567 = 127645`; this phase asks
  what a pupil's derivation makes of it.

**Blocks:** nothing. [M1g](../../m1g_logic/README.md) follows all of M1f, so it follows this; its proof-system
and induction phases are written so as not to need it, and
[§ What is left to M1g](#what-is-left-to-m1g) says where the line is.

**The number.** 8 was one of M1f's unassigned phase numbers
([M1f § What this is](../README.md#what-this-is) listed *1–4, 8–9 and 11–19* until this phase took it). It is taken because it
is free and sits between the calculator, P1f.6, and the hypothesis structure, P1f.10; a phase's number is not
its place in the order.

## The instruction

The user, 2026-11-11:

> I forgot another idea - between calculator and proof systems - middle school algebra and geometry: variables,
> expressions simplifications, solving equasions; middle school geometry contains proofs, but I'm not sure it is
> 100% complete formal proof system (it starts in 5th grade) - so just separate phase in M1f

So: one phase, in M1f, between the calculator and M1g's proof systems. Two halves. **Algebra:** variables,
simplifying expressions, solving equations. **Geometry:** the proofs school geometry contains, and the honest
question the user raises, whether school geometry is a complete formal proof system at all.

## What it is

The calculator computes with numbers. M1g studies proof as an object. School mathematics is what lies between:
the first place a pupil computes with **something that is not a number** (a letter, an expression, a figure), and
the first place a pupil is asked to **justify** each step. The phase writes that mathematics as Ein programs and
asks of each piece which of Ein's two mechanisms does the work.

| the school activity | what it is in Ein, as planned | mechanism | stage |
|---|---|---|---|
| an expression with a letter in it | a nested fact over the calculator's numbers and names; the letter is a **name**, never an Ein `?x` | data | [S1f.8.2](s1f.8.2_expressions_and_evaluation.md) |
| *find the value of 3x + 4 when x = 5* | substitution, then the calculator, on request | saturation, search-free | S1f.8.2 |
| *simplify*, *expand the brackets*, *collect like terms* | rewriting on request toward a polynomial normal form | saturation, search-free; to be shown | [S1f.8.3](s1f.8.3_simplification_and_identities.md) |
| *prove the identity (a+b)² = a² + 2ab + b²* | both sides have one normal form | saturation | S1f.8.3 |
| *solve 3x + 4 = 19* | a derivation, a rule a step, each with its `:why`: the school solution is the trace | saturation, search-free | [S1f.8.4](s1f.8.4_linear_equations.md) |
| the same over a small finite domain | the unknown's value as a hypothesis, the equation a restriction | the lattice | S1f.8.4 |
| a system of two equations, a quadratic | substitution, elimination; factoring where the discriminant is a perfect square | saturation; a quadratic's two roots are two facts, not two states | [S1f.8.5](s1f.8.5_systems_and_quadratics.md) |
| a figure and what is given about it | points as names, segments, angles and triangles as nested facts, relations between them | data | [S1f.8.6](s1f.8.6_what_school_geometry_is.md) |
| *prove that the base angles are equal* | forward saturation over a rule base of theorems, each with its `:why`: the trace is the two-column proof | saturation | [S1f.8.7](s1f.8.7_theorems_as_rules.md) |
| *draw the median*, *suppose the lines meet*, *consider two cases* | a construction, an assumption, a case: each a hypothesis, each with a limit that is measured | the lattice | [S1f.8.8](s1f.8.8_constructions_contradiction_cases.md) |
| *find the third angle*, *find the perimeter* | the word-problem pattern over the calculator | saturation | [S1f.8.9](s1f.8.9_computation_and_the_diagram_gap.md) |
| *it is clear from the picture that D lies between B and C* | a fact the figure must state. Where it is missing, the gap is detected and named, never assumed | detection | S1f.8.9 |

## What it is not

- **Not a computer algebra system.** No factoring of general polynomials, no symbolic integration, no
  simplification heuristics. A normal form for polynomials with rational coefficients, linear equations, two
  equations in two unknowns, and quadratics with a rational root: the school course to about the 8th grade, and
  where it stops is stated with a program.
- **Not a complete geometry prover.** A rule base of a few dozen school theorems, saturated forward. It will prove
  some school theorems and not others, and the phase's report says which and why. Algebraic methods that decide
  large classes of geometry statements (coordinates and polynomial ideals: Wu's method, Gröbner bases, the area
  method) are named in [S1f.8.6](s1f.8.6_what_school_geometry_is.md) and not built: they are not how a pupil
  proves anything, and they need arithmetic Ein does not have.
- **Not engine arithmetic.** [Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)
  stands: no numbers, no ranges, no arithmetic predicate. Every number is the calculator's nested fact, and every
  computation a request the imported theory answers.
- **Not a language of function calls.** The user, 2026-11-11: *"Ein is not Lisp, is it?"*. An expression here is a
  thing the program *talks about*, as a formula is in M1g; it is not a call that returns.
- **Not new syntax.** Every program is written in the grammar as it ships. A grammar change needs the user's
  ruling on real programs shown first, and none is planned.
- **Not natural-language parsing.** A problem's English is a `:source` string and a comment; its facts are
  written by hand. The sentence beside the facts is recorded, since that pair is what
  [M2](../../m2_nl_to_ir/README.md) would have to produce.
- **Not proof theory.** What a proof *is*, soundness, completeness, natural deduction and induction are M1g's.
  This phase writes proofs the way a pupil does and reports what kind of object each turned out to be.

## Expressions, and S1f.6.14

The calculator's rulings left one thing open: the expression layer, `(calc E)` and `(let NAME E)`
([`design.md` § The rulings](../p1f.6_calculator/design.md#the-rulings-2026-11-11)). The user found it
questionable, and [S1f.6.14](../p1f.6_calculator/s1f.6.14_expressions.md) waits. Algebra meets the same shape
from the other side and the two must not be confused:

- **S1f.6.14's question** is whether a program may *compute* by writing a tree: is `(calc (add N1 (div N2 N3)))`
  a second way to say what request facts and rules already say?
- **This phase's need** is that a program *reasons about* a tree: `3x + 4` has no value, it has a *shape*, and
  simplifying it or solving for `x` reads that shape. It is data, in the way a formula is data in
  [M1g](../../m1g_logic/README.md) (*a logic written in Ein*, its second level).

The spelling shows the difference and avoids a clash. `add`, `sub`, `mul` and `div` are the calculator's request
relations and mean *asked* once the theory is imported flat
([`stdlib/arith.ein`](../../../stdlib/arith.ein) § The names this module occupies). An expression's heads are
other ordinary relations. This plan writes them capitalised, `Plus`, `Minus`, `Times`, `Over`, `Neg`, `Pow`, as
M1g writes its connectives `And` and `Imp`
([Q-M1g.1](../../m1g_logic/open_questions.md#q-m1g1--the-spelling-of-the-connectives)); the user ruled for the
gates that any spelling will do, and one convention across the two milestones is the reason for this one. It is
revisable at S1f.8.1's stop.

**What S1f.8.1 puts to the user is therefore one question with two parts,** on programs it has computed:
whether expression *data* of this kind is accepted (the phase cannot run otherwise), and whether, once an
evaluator over that data exists as an imported module ([S1f.8.2](s1f.8.2_expressions_and_evaluation.md)), it
**is** the answer to S1f.6.14: a program that wants a tree writes `(evaluate E)` over data, and the calculator's
core stays requests and results. If S1f.6.14 has been ruled before this phase starts, the phase follows that
ruling and asks only the first part.

## What a variable is

Three things are called a variable and the phase keeps them apart, on one page of
[S1f.8.2](s1f.8.2_expressions_and_evaluation.md):

| | written | what it is |
|---|---|---|
| **Ein's pattern variable** | `?x` | a slot in a rule's pattern. It is bound by a match and exists in no state. It ranges over values |
| **an object-level variable** | the name `x`, inside `(var x)` | a name of the program, like `Bob`. It can be assigned a value, `(assign x N)`, or be the unknown of an equation |
| **an unknown** | `(sought x)` | an object-level variable the problem asks for. Its value is a derived fact, or a hypothesis |

A rule that says *like terms add* is `?`-quantified over object-level variables: `(term ?a ?v)` matches `2x` with
`?v` bound to the name `x`. That is the same two levels M1g's
[S1g.1.2](../../m1g_logic/p1g.1_survey_and_design/s1g.1.2_two_levels.md) sets apart for formulas.
(The word `unknown` itself is taken: it is a macro of `std.macro`, and a fact `(unknown x)` is refused at load,
[§ First probes](#first-probes-2026-11-11), A-5.)

## The stages

| ID | title | depends on | stops for the user |
|---|---|---|---|
| [S1f.8.1](s1f.8.1_survey_and_design.md) | The survey and the design: both halves on a prototype, every answer computed, the conventions and the questions | S1f.6.6; S1f.6.7 and S1f.6.8 for the programs that need a sign or a fraction | **yes: the phase's stop.** Nothing outside `plans/` and `build/` is written before the user has read its page |
| [S1f.8.2](s1f.8.2_expressions_and_evaluation.md) | Expressions as data, what a variable is, and evaluation under an assignment | S1f.8.1's rulings; S1f.6.14's ruling or its taking here | no |
| [S1f.8.3](s1f.8.3_simplification_and_identities.md) | Simplification as rewriting to a polynomial normal form; equality of expressions; identities proved by normal forms | S1f.8.2; S1f.6.7, S1f.6.8 | no; one choice **for the user** (the normal form's order, with S1f.8.1) |
| [S1f.8.4](s1f.8.4_linear_equations.md) | Linear equations in one unknown: the pupil's derivation, no solution and every number, derivation against search, and word problems that need an unknown | S1f.8.3; S1f.6.20 | **yes, once:** what *no solution* and *every number* are (a flag, a dead root, a result fact) |
| [S1f.8.5](s1f.8.5_systems_and_quadratics.md) | Systems of two linear equations; quadratics with a rational root; inequalities, optional | S1f.8.4 | no; the optional part runs on the user's word |
| [S1f.8.6](s1f.8.6_what_school_geometry_is.md) | What school geometry is, honestly, and how a figure is written: the vocabulary, the bookkeeping, measures against synthetic relations | S1f.8.1's rulings; S1f.6.6 for measures | **yes:** the vocabulary and the reading of the literature, before the rule base is written |
| [S1f.8.7](s1f.8.7_theorems_as_rules.md) | Theorems as rules, proofs as saturation: a rule base, five school theorems, and the trace as a two-column proof | S1f.8.6 | no |
| [S1f.8.8](s1f.8.8_constructions_contradiction_cases.md) | Auxiliary constructions, proof by contradiction and case splits: three uses of a hypothesis, and what each cannot do today | S1f.8.7 | **yes:** how a construction is written, and whether a refuted assumption concludes anything, with M1g's [Q-M1g.5](../../m1g_logic/open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact) |
| [S1f.8.9](s1f.8.9_computation_and_the_diagram_gap.md) | Computation problems in geometry, the diagram gap detected and named, and the phase's report: which fragment of school mathematics is a formal derivation in Ein | S1f.8.7, S1f.8.8; S1f.8.4 for problems with an unknown | no; the report is **shown** |

No stage carries an estimate. Engine findings go to [`findings.md`](findings.md).

## The order

1. **S1f.8.1 first, and it stops.** Both halves are on its page, so the user sees the geometry's vocabulary and
   the algebra's expressions together and can rule the shared conventions once.
2. **The algebra, S1f.8.2 to S1f.8.5, in order.** Each uses the one before: evaluation needs expressions, a
   normal form needs evaluation as its check, an equation is solved on normal forms, and a system is two
   equations.
3. **The geometry, S1f.8.6 to S1f.8.8, in order, beside the algebra.** S1f.8.6 and S1f.8.7 use no algebra and
   may run as soon as S1f.8.1's rulings are in. They share no file with S1f.8.2 to S1f.8.5.
4. **S1f.8.9 last,** since a geometry computation with an unknown (*one angle is twice another*) is S1f.8.4's
   equation over S1f.8.7's facts, and the report reads every stage.

Against the rest of M1f: after P1f.6's extensions and word problems, since it imports them. It shares no file
with [P1f.10](../p1f.10_hypothesis_structure/README.md) or [P1f.20](../p1f.20_docs_refactor/README.md) but the
kernel pages P1f.20 moves, which it does not edit.

## First probes (2026-11-11)

Scratch programs the planning agent ran once while writing this tree, on the release binary found in
`ein.rs/target/release/` that day (built 2026-11-11 17:59; not rebuilt), with `ein solve FILE` unless said
otherwise. They are **not** results: nothing is banked, no test holds them, and each is **re-taken** by the stage
named, as a kept program. They are here so that the plan's expectations are told apart from its guesses. Each
program is reproduced as a sketch in the stage that re-takes it.

| # | the program | what the binary said | re-taken by |
|---|---|---|---|
| A-1 | `3x + 4 = 19` as `(linear x 3 4 19)`, three rules of the program over `std.arith`: take the constant from both sides, divide by the coefficient, read the quotient | `(value x (n z 5))`; no candidate, k_s 1, R {∅}, 1 saturation; the claim holds under `ein test`. `--trace` writes **123 steps, of which 3 are the pupil's** (steps 2, 19 and 123); the other 120 are the calculator's columns | [S1f.8.4](s1f.8.4_linear_equations.md) |
| A-2 | the same with 18 for 19, and a rule that reads a remainder `(n ?p ?r)` | `(no-root x)` derived with no negation: a non-zero normal-form number always has the shape `(n P D)`, so *the remainder is not zero* is a pattern | S1f.8.4 |
| A-3 | the same with the coefficient `z`: `0·x + 4 = 19` | **root is dead:** the naive solver asks `(div 15 z)`, and the calculator's ruled division by zero derives `(false)`. `refuted by division-by-zero-is-false`; the core is the equation and three table rows. A solver must compare the coefficient with zero before it divides | S1f.8.4 |
| A-4 | `3x + 4 = 19` by search: an hrule proposes `(value x (n z D))` for each of ten digits, a rule kills two values of one unknown, a rule kills a value whose left side is not 19 | 10 candidates, **9 dead alone**, N 9, k_s 2: ∅ and `{(value x (n z 5))}`, 11 saturations. ∅ is in R because nothing says the unknown has a value; `:over maximal` reads the one answer | S1f.8.4 |
| A-5 | a fact `(unknown x)` | refused at load: `unknown` is a macro of `std.macro`, and a macro's call is no fact ([`defined_behaviour.md` § 4.24](../../../docs/kernel/defined_behaviour.md)). The plan writes `sought` | [S1f.8.2](s1f.8.2_expressions_and_evaluation.md) |
| A-6 | `(simplify (Plus (term 2 x) (term 3 x)))`, two rules: ask the calculator for the sum of the coefficients, read it | `(simpler … (term (n z 5) x))`; search-free | [S1f.8.3](s1f.8.3_simplification_and_identities.md) |
| A-7 | `(evaluate (Plus (Times (num 3) (var x)) (num 4)))` with `(assign x 5)`, eight rules: requests go down the tree, values come up through `mul` and `add` | `(value-of … (n (n z 1) 9))`; search-free. Its rules join an expression with its parts' values, so they break the calculator's rule of the theory | S1f.8.2 |
| A-8 | A-1 with the line `(solve :hypothesis-relations ())` taken out | **exit 1**, one line: *a state's commitments pass 2^128: the state route cannot count them*. The program's own relations are candidates for the blind enumerator over every name the tables hold. Every program of this phase states its lattice | [S1f.8.1](s1f.8.1_survey_and_design.md); [`findings.md`](findings.md) F-1 |
| G-1 | An isosceles triangle, `AB = AC`; `SAS` and *corresponding angles of congruent triangles* as rules; three bookkeeping rules | `(ang-eq (ang A B C) (ang A C B))`, by Pappus's proof (the triangle is congruent to itself read the other way round); no candidate, k_s 1, 1 saturation, 10 firings. Without the `(solve …)` line: 24 candidates, k_s at least 72 012, 118 847 saturations | [S1f.8.7](s1f.8.7_theorems_as_rules.md) |
| G-2 | The same theorem by the school's proof: an hrule proposes the construction `(drawn (mid B C))`, then `SSS` | 1 candidate, 0 dead, k_s 2. The theorem holds in the state that drew the midpoint and **not at root**: the query over all of R has two classes, one *undetermined*; over the maximal line it has one, the theorem | [S1f.8.8](s1f.8.8_constructions_contradiction_cases.md) |
| G-3 | Two lines perpendicular to a third; an hrule supposes they meet; a rule makes that `(false)` | 2 candidates, **2 dead alone**, N 2, R {∅}. The supposition is refuted and reported; root does **not** hold `(parallel a b)`. M1g's probe 4 again, on a school theorem | S1f.8.8 |
| G-4 | A triangle with angles of 50 and 60 degrees; three rules: add the two, take the sum from 180, read the third | `(measure (ang B C A) (n (n z 7) 0))`; search-free | [S1f.8.9](s1f.8.9_computation_and_the_diagram_gap.md) |
| G-5 | (seen in G-1's `--trace`) a `:why` that prints a nested value | the value prints as written, *(seg A C) is as long as (seg A B)*, not by its own relation's `:why`, *the segment AC* | [`findings.md`](findings.md) F-3 |

## Acceptance

- **S1f.8.1's page was read and ruled on by the user** before any file outside `plans/` and `build/` was written.
- **Each row of [§ What it is](#what-it-is) has a program whose answer the binary computed,** or the page that
  owns it says *this could not be done* and why, with the program that shows it.
- **Search-free where it should be, held by a test:** evaluation, simplification to normal form, the derivation
  of a linear equation's root, and the forward proof of each target theorem have no candidate, k_s 1 and R {∅},
  as the calculator's programs do (`ein-cli/tests/arith_search_free.rs` is the model). Where a stage uses the
  lattice (search over a domain, a construction, an assumption, a case), the page says so and what it cost.
- **Right, held against an independent check written in Rust:** every value, every normal form and every root
  agrees with a check that shares no code with the Ein theory, on every expression and equation up to a stated
  bound; two expressions have one normal form exactly when the check says they are equal as polynomials.
- **The trace is the school solution:** for one linear equation and one geometry theorem at least, the steps a
  pupil would write are read off the run, each a rule with its `:why` rendered, and a test holds them. What stands
  between today's trace and that reading (A-1's 3 steps in 123) is a row of [`findings.md`](findings.md) with a
  ruling, a fix or an owner.
- **Five school theorems,** each with what the program proved, by which rules, and what it needed written that a
  pupil reads off the picture; and at least one theorem the rule base **cannot** prove, with the reason.
- **No gap is silently filled.** A degenerate equation, a missing quantity and a missing fact of the figure are
  each reported by name; none is assumed.
- **The report** ([S1f.8.9](s1f.8.9_computation_and_the_diagram_gap.md)) states which fragment of school algebra
  and geometry is a formal derivation in Ein today, and hands the rest to M1g by name.
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
  result fact), and **directed** (every rule moves toward the normal form, never both ways). S1f.8.3 states the
  measure that decreases, and a test runs each theory under a timeout that fails loudly, since the engine has
  no word for a saturation that does not stop (M1g's C-1).
- **The natural rules join on nested values.** *Value the two parts, then add* (A-7) joins an expression with
  its parts' values: the join the calculator's rule of the theory forbids, quadratic in a batch (E-3 of the
  calculator's [`findings.md`](../p1f.6_calculator/findings.md)). One problem is small and does not notice. The
  phase **measures** each theory in a batch on the matcher's counters, and either rewrites it with tags, as the
  calculator did, or says that it is quadratic and why that is accepted. It does not build the index: that is
  M1g's [S1g.6.2](../../m1g_logic/p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md).
- **Bookkeeping swamps the mathematics.** `AB` is `BA`, the angle `BAC` is `CAB`, a triangle has six readings,
  equality is symmetric and transitive. G-1 needed three such rules for one theorem. Written as forward rules
  they multiply every fact; S1f.8.6 chooses between closure rules, a canonical spelling and a symmetric
  declaration, on measurements.
- **The trace buries the solution.** 3 steps in 123 (A-1). A school solution is the problem theory's steps with
  the arithmetic folded. Whether a trace can be read at one theory's level is the first row of
  [`findings.md`](findings.md), and the phase's headline acceptance depends on it.
- **H is not empty by default** (A-8, G-1). Every program states its lattice or closes its relations, and the
  search-free test holds it.
- **A construction is not an assumption.** A hypothesis is the only way Ein adds something a rule did not derive,
  and G-2 shows its price: the theorem is in a state, not at root. If S1f.8.8 finds no honest reading, the phase
  says that school proofs with a construction are found by Ein and *certified* only by re-running with the
  construction written as a fact.
- **The geometry is unsound without the picture.** A rule that adds two angles is true only if a ray lies inside
  an angle. A rule base that leaves the side condition out proves false things about some figures. S1f.8.7 writes
  every such condition as a premise, and S1f.8.9's negative tests are figures on which the unconditioned rule
  would be wrong.
- **Scope.** Each of trigonometry, circles, similarity, areas, coordinates and inequalities is a subject. Circles,
  similarity and areas are named where they come up and are not planned; inequalities are one optional part.

## The questions for the user

They are asked at the stops named, on computed programs. They are not filed as `Q-M1f.<n>` today, since P1f.6 is
filing its own in [`open_questions.md`](../open_questions.md); the phase files them, with the ids then free, when
it starts.

| | the question | asked at | the plan's default |
|---|---|---|---|
| a | **Is expression data accepted, and is an evaluator over it the answer to S1f.6.14?** ([§ Expressions, and S1f.6.14](#expressions-and-s1f614)) | S1f.8.1 | data yes; `(evaluate E)` as an imported module is S1f.6.14's (b)-like answer, and no `calc` in the calculator's core |
| b | **The spelling of expression heads,** `Plus` `Times` …, shared with M1g's connectives | S1f.8.1 | capitalised |
| c | **Where the theories live:** `stdlib/` (`std.algebra` is taken by the relation-algebra module, so another name), or `examples/`, as S1f.6.20 also asks | S1f.8.1 | `examples/` first; a module moves to `stdlib/` when a second program imports it |
| d | **The normal form's order** of monomials and of variables | S1f.8.1, for S1f.8.3 | descending degree, variables in the order of the name rule's text |
| e | **What an equation with no solution, or with every number as one, is:** a result fact (`(no-root x)`, `(every-number x)`), a flag and then `(false)` as division by zero is, or a dead root | S1f.8.4 | a result fact. An equation with no root is an *answer*, and a program that states one is not contradictory; `(false)` is for a problem whose own facts conflict |
| f | **Is the school solution a view of the trace?** A read-out that shows one theory's steps and folds an imported theory's | S1f.8.1 (shown), ruled at S1f.8.4 | a finding with an owner; the phase proposes and does not build a `--trace` option without a ruling |
| g | **The geometry's vocabulary and its level:** synthetic relations only, or measures as the calculator's numbers too | S1f.8.6 | both, kept apart: a synthetic rule base, and a measure layer that imports it and the calculator |
| h | **A construction:** a hypothesis, a Skolem-style value on request, or a fact the problem writes | S1f.8.8 | on request as a value where a rule can name it (`(mid B C)`); the *choice* of construction as a hypothesis, reported as search |
| i | **Does a refuted assumption conclude anything at root?** The same as [Q-M1g.5](../../m1g_logic/open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact); asked once, there or here, whichever stage runs first | S1f.8.8 | not built here; the limit is shown and the candidate named |
| j | **Inequalities:** in or out | S1f.8.5 | out unless asked |

## What is left to M1g

So that the two do not overlap:

- **Equational reasoning as a proof system.** S1f.8.3 proves an identity by computing two normal forms. That is
  a decision procedure, not a proof. A proof by *equational steps*, each an axiom of a commutative ring applied at
  a position, checked line by line, is a proof system and belongs with
  [P1g.3](../../m1g_logic/p1g.3_proof_systems/README.md)'s checker. This phase hands it the normal-form
  procedure as the semantics such a system would be checked against.
- **What a proof is.** S1f.8.7's proofs are derivations Ein finds; a proof as *data* that Ein checks is
  [S1g.3.1](../../m1g_logic/p1g.3_proof_systems/s1g.3.1_hilbert_systems.md)'s.
- **Assumption and discharge.** S1f.8.8 shows reductio's premise (a dead commitment and its core) on school
  theorems. The rule that concludes the negation is
  [S1g.3.3](../../m1g_logic/p1g.3_proof_systems/s1g.3.3_natural_deduction.md)'s.
- **Witnesses and fresh names.** S1f.8.8 uses a nested value for a constructed point and reports where that is
  not enough. Names made at run time are M1g's candidate C-8,
  [S1g.6.4](../../m1g_logic/p1g.6_engine_requirements/s1g.6.4_witnesses_and_fresh_names.md), with
  [Q-M1g.8](../../m1g_logic/open_questions.md#q-m1g8--fresh-names); this phase names it and does not build it.
- **For all numbers.** An identity is proved for all values by normal forms, which is algebra and not induction.
  A statement whose proof needs induction (a sum formula, divisibility for every n) is
  [P1g.5](../../m1g_logic/p1g.5_peano_and_induction/README.md)'s.
- **Whether a system is complete.** S1f.8.6 reports what the literature says of Euclid, Hilbert and Tarski, and
  S1f.8.9 reports what the phase's rule base proved. Neither proves a meta-theorem;
  [F26](../../followups/f26_consistency_and_completeness.md)'s glossary of the senses of *complete* is where the
  word is sorted out, and [S1g.4.5](../../m1g_logic/p1g.4_first_order_logic/s1g.4.5_the_boundary.md) takes it.

## Connections

- **[P1f.6](../p1f.6_calculator/README.md):** the numbers, the request protocol, the search-free test, the
  harness against Rust's integers, the rule of the theory and its reason, the ruling on division by zero, and
  [`findings.md`](../p1f.6_calculator/findings.md)'s E-3 and E-12, which this phase's rules meet head on.
  [S1f.6.10](../p1f.6_calculator/s1f.6.10_beyond_arithmetic.md) hands over its equations;
  [S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md) its gaps and readings;
  [S1f.6.14](../p1f.6_calculator/s1f.6.14_expressions.md) its question.
- **[M1g](../../m1g_logic/README.md):** [§ What is left to M1g](#what-is-left-to-m1g). In the other direction,
  this phase is M1g's first source of proofs that are not about formulas: S1g.3.3's reductio and S1g.4.4's
  witnesses can be tried on S1f.8.8's theorems.
- **[M2](../../m2_nl_to_ir/README.md):** each solved problem is an instance of
  [§ The reasoning as a program](../../m2_nl_to_ir/README.md#the-reasoning-as-a-program-the-users-idea-2026-11-11):
  the school solution as a self-explained Ein program, every step checked by the kernel. A school problem book
  is a benchmark with gold answers and gold *reasoning*, and the gap problems are instances whose right answer
  is *the figure does not say*.
- **[F26](../../followups/f26_consistency_and_completeness.md):** the senses of *complete*. School geometry is
  the plainest case of a system that is used as if complete and is not, and the diagram gap is F26's
  *under-determined by a reading* in a new domain.
- **[F15](../../followups/f15_math_formulae/ideas.md):** rules exported as formulas. A geometry rule has a
  school statement, a logical form and a picture; an algebra rule *is* a formula, `a(b + c) = ab + ac`. This
  phase's rule bases are the first where the exported formula is the thing the rule was written from, and
  S1f.8.3 and S1f.8.7 record the formula beside each rule, in its `:why` or a comment, so that F15 has a corpus.
- **[Idea 09](../../ideas/09-puzzles-beyond-zebra.md):** puzzles beyond Zebra; it sets geometry-heavy puzzles
  (tangram, Soma) aside as out of reach. Nothing here changes that: this is deductive geometry over named
  points, not spatial search.
- **[Idea 08](../../ideas/08-human-style-deductive-trace.md):** the human-style trace. The acceptance line *the
  trace is the school solution* is that idea with a ground truth: a textbook's worked solution.
- **[`docs/lib/03`](../../../docs/lib/03-theorem-proving-formal-methods.md):** the external systems
  S1f.8.6 compares with.
