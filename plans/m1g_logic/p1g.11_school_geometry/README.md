# P1g.11 — School geometry: a figure as facts, theorems as rules, and the proofs a pupil writes

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day ([§ The instruction](#the-instruction)),
as the geometry half of M1f's P1f.8. Not started. It carries no estimate (the user's ruling of 2026-11-08). Its
first stage **stops for the user** with real programs computed on a prototype, before anything ships.
**Split off and moved to M1g on 2026-11-13,** on the user's instruction of that day
([§ The move](#the-move-of-2026-11-13)), which also said of its second stage that it *"probably will require
new features in the engine"*. The algebra is [P1g.10](../p1g.10_school_algebra/README.md).
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s stop, as M1g's phases do;
[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md), the calculator; and
[P1g.10](../p1g.10_school_algebra/README.md) at two points. Specifically:

- **[S1g.10.1](../p1g.10_school_algebra/s1g.10.1_survey_and_design.md)'s rulings** on what the two phases share:
  where the theories live and how a program states its lattice. The first stage here,
  [S1g.11.1](s1g.11.1_what_school_geometry_is.md), follows them, as it did when both were stages of one phase;
- **[S1g.10.4](../p1g.10_school_algebra/s1g.10.4_linear_equations.md), linear equations,** for the last stage
  only: a figure's problem with an unknown is that stage's equation over this phase's facts;
- **the four operations and comparison,** `std.math.decimal-arith`
  ([S1f.6.3](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.3_addition.md) to
  [S1f.6.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.6_division.md)),
  done 2026-11-11: every measure here is the calculator's number, `z` or `(n PREFIX DIGIT)`;
- **[S1f.6.20](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md), word
  problems:** its two kinds of gap, its readings as hypotheses and its *the problem is a model* are the pattern
  this phase's geometry computations follow;
- **[P1g.6](../p1g.6_engine_requirements/README.md), the engine requirements,** for whatever the phase's
  computed programs show is needed and the user rules built. The candidates are collected in
  [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md). **The phase does not wait
  for any of them to start:** [S1g.11.1](s1g.11.1_what_school_geometry_is.md) and
  [S1g.11.2](s1g.11.2_theorems_as_rules.md) write their programs in the language as it ships, and what those
  programs lacked is what decides which candidate is a requirement.

**Blocks:** nothing. M1g's proof-system and induction phases are written so as not to need it, and
[§ What is left to M1g's other phases](#what-is-left-to-m1gs-other-phases) says where the line is.

**The number.** 11 was the next free phase number of M1g on 2026-11-13, after P1g.10. A phase's number is not
its place in the order ([M1g § The order](../README.md#the-order)). M1f's number 8, which this phase and P1g.10
shared as P1f.8 from 2026-11-11, is recorded there as used and vacated, and is not reassigned.

## The move of 2026-11-13

The user, 2026-11-13:

> plans/m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry seems too complex having both algebra
> and geometry, let's split it into 2 phases and move into m1g;
> plans/m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/s1f.8.7_theorems_as_rules.md  probably
> will require new features in the engine

So P1f.8 is two phases of M1g: [P1g.10](../p1g.10_school_algebra/README.md), the algebra, and this one, the
geometry. The files were moved with `git mv`, so each stage's history follows it. **Nothing of the content was
redesigned:** every target program, probe, task, acceptance line, risk, question and connection of P1f.8 is in
exactly one of the two phases, or, where it covered both halves, in both. The old path keeps a
[stub](../../m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/README.md), since pages that
hold the user's replies as typed link to it.

**The instruction's second sentence** is about the stage that is now
[S1g.11.2](s1g.11.2_theorems_as_rules.md#the-users-note-of-2026-11-13), *theorems as rules*, and is recorded
there. What it led to is one provisional stage file in P1g.6,
[S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md): the candidate engine features
that stage may need, each a question with the program that would show it. They are candidates, and the stage
designs nothing ([§ Risks](#risks-each-with-what-checks-it)).

| old id (M1f's P1f.8) | new id | phase |
|---|---|---|
| S1f.8.1, the survey and the design | [S1g.10.1](../p1g.10_school_algebra/s1g.10.1_survey_and_design.md), its algebra half; its geometry half is folded into [S1g.11.1](s1g.11.1_what_school_geometry_is.md#the-survey-of-the-geometry-moved-here-from-s1f81-2026-11-13) | P1g.10 and P1g.11 |
| S1f.8.2, expressions and evaluation | [S1g.10.2](../p1g.10_school_algebra/s1g.10.2_expressions_and_evaluation.md) | P1g.10 |
| S1f.8.3, simplification and identities | [S1g.10.3](../p1g.10_school_algebra/s1g.10.3_simplification_and_identities.md) | P1g.10 |
| S1f.8.4, linear equations | [S1g.10.4](../p1g.10_school_algebra/s1g.10.4_linear_equations.md) | P1g.10 |
| S1f.8.5, systems and quadratics | [S1g.10.5](../p1g.10_school_algebra/s1g.10.5_systems_and_quadratics.md) | P1g.10 |
| S1f.8.10, a program that states false facts | [S1g.10.6](../p1g.10_school_algebra/s1g.10.6_a_program_that_states_false_facts.md) | P1g.10 |
| S1f.8.11, compound numbers without a hardcoded representation | [S1g.10.7](../p1g.10_school_algebra/s1g.10.7_compound_numbers_without_a_hardcoded_representation.md) | P1g.10 |
| S1f.8.6, what school geometry is | [S1g.11.1](s1g.11.1_what_school_geometry_is.md) | P1g.11 |
| S1f.8.7, theorems as rules | [S1g.11.2](s1g.11.2_theorems_as_rules.md) | P1g.11 |
| S1f.8.8, constructions, contradiction, cases | [S1g.11.3](s1g.11.3_constructions_contradiction_cases.md) | P1g.11 |
| S1f.8.9, computation and the diagram gap | [S1g.11.4](s1g.11.4_computation_and_the_diagram_gap.md) | P1g.11 |

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
4. **No letter or id inside the phase was renumbered.** Questions a to f and j are P1g.10's and g to i are
   this phase's; probes A-1 to A-8 are there and G-1 to G-5 here; findings F-1, F-2 and F-6 are in
   [P1g.10's `findings.md`](../p1g.10_school_algebra/findings.md) and F-3 to F-5 in this phase's
   [`findings.md`](findings.md), and the next free id, F-7, is taken from one sequence over the two files.
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
geometry:** the proofs school geometry contains, and the honest question the user raises, whether school
geometry is a complete formal proof system at all. The algebra is
[P1g.10](../p1g.10_school_algebra/README.md)'s.

## What it is

The calculator computes with numbers. M1g's other phases study proof as an object. School mathematics is what
lies between: the first place a pupil computes with **something that is not a number** (a letter, an
expression, a figure), and the first place a pupil is asked to **justify** each step. The phase writes the
geometry of that mathematics as Ein programs and asks of each piece which of Ein's two mechanisms does the work.

| the school activity | what it is in Ein, as planned | mechanism | stage |
|---|---|---|---|
| a figure and what is given about it | points as names, segments, angles and triangles as nested facts, relations between them | data | [S1g.11.1](s1g.11.1_what_school_geometry_is.md) |
| *prove that the base angles are equal* | forward saturation over a rule base of theorems, each with its `:why`: the trace is the two-column proof | saturation | [S1g.11.2](s1g.11.2_theorems_as_rules.md) |
| *draw the median*, *suppose the lines meet*, *consider two cases* | a construction, an assumption, a case: each a hypothesis, each with a limit that is measured | the lattice | [S1g.11.3](s1g.11.3_constructions_contradiction_cases.md) |
| *find the third angle*, *find the perimeter* | the word-problem pattern over the calculator | saturation | [S1g.11.4](s1g.11.4_computation_and_the_diagram_gap.md) |
| *it is clear from the picture that D lies between B and C* | a fact the figure must state. Where it is missing, the gap is detected and named, never assumed | detection | S1g.11.4 |

The table's other seven rows, expressions, evaluation, simplification, identities, equations, their search and
systems, are [P1g.10's](../p1g.10_school_algebra/README.md#what-it-is).

## What it is not

- **Not a complete geometry prover.** A rule base of a few dozen school theorems, saturated forward. It will prove
  some school theorems and not others, and the phase's report says which and why. Algebraic methods that decide
  large classes of geometry statements (coordinates and polynomial ideals: Wu's method, Gröbner bases, the area
  method) are named in [S1g.11.1](s1g.11.1_what_school_geometry_is.md) and not built: they are not how a pupil
  proves anything, and they need arithmetic Ein does not have.
- **Not engine arithmetic.** [Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)
  stands: no numbers, no ranges, no arithmetic predicate. Every number is the calculator's nested fact, and every
  computation a request the imported theory answers.
- **Not new syntax.** Every program is written in the grammar as it ships. A grammar change needs the user's
  ruling on real programs shown first, and none is planned. The user's note of 2026-11-13 does not change this:
  [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md) lists candidates and plans
  no change.
- **Not natural-language parsing.** A problem's English is a `:source` string and a comment; its facts are
  written by hand. The sentence beside the facts is recorded, since that pair is what
  [M2](../../m2_nl_to_ir/README.md) would have to produce.
- **Not proof theory.** What a proof *is*, soundness, completeness, natural deduction and induction are M1g's
  logic phases'. This phase writes proofs the way a pupil does and reports what kind of object each turned out
  to be.

## The stages

| ID | was | title | depends on | stops for the user |
|---|---|---|---|---|
| [S1g.11.1](s1g.11.1_what_school_geometry_is.md) | S1f.8.6, with the geometry half of S1f.8.1 | What school geometry is, honestly, and how a figure is written: the survey on a prototype, the vocabulary, the bookkeeping, measures against synthetic relations | S1g.10.1's rulings on what the two phases share; S1f.6.6 for measures | **yes: the phase's stop.** The survey's programs, the vocabulary and the reading of the literature, before the rule base is written. Nothing outside `plans/`, `build/` and `docs/lib/` is written before the user has read its page |
| [S1g.11.2](s1g.11.2_theorems_as_rules.md) | S1f.8.7 | Theorems as rules, proofs as saturation: a rule base, five school theorems, and the trace as a two-column proof | S1g.11.1 | no. **The user, 2026-11-13: it *"probably will require new features in the engine"*;** its candidates are [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md)'s |
| [S1g.11.3](s1g.11.3_constructions_contradiction_cases.md) | S1f.8.8 | Auxiliary constructions, proof by contradiction and case splits: three uses of a hypothesis, and what each cannot do today | S1g.11.2 | **yes:** how a construction is written, and whether a refuted assumption concludes anything, with [Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact) |
| [S1g.11.4](s1g.11.4_computation_and_the_diagram_gap.md) | S1f.8.9 | Computation problems in geometry, the diagram gap detected and named, and the report: which fragment of school mathematics is a formal derivation in Ein | S1g.11.2, S1g.11.3; S1g.10.4 for problems with an unknown | no; the report is **shown** |

No stage carries an estimate. Engine findings go to [`findings.md`](findings.md).

## The order

1. **S1g.11.1 first, and it stops.** It runs once
   [S1g.10.1](../p1g.10_school_algebra/s1g.10.1_survey_and_design.md)'s rulings on the shared conventions are
   in. As S1f.8.1 planned it, the user saw the geometry's vocabulary and the algebra's expressions on one page;
   since the split there are two pages and two stops, and the shared conventions are ruled at the first.
2. **S1g.11.1 to S1g.11.3, in order, beside the algebra.** S1g.11.1 and S1g.11.2 use no algebra. They share no
   file with [P1g.10](../p1g.10_school_algebra/README.md)'s S1g.10.2 to S1g.10.5.
3. **S1g.11.4 last,** since a geometry computation with an unknown (*one angle is twice another*) is S1g.10.4's
   equation over S1g.11.2's facts, and the report reads every stage of both phases.
4. **P1g.6's stages, where the programs call for them.** A candidate of
   [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md) that S1g.11.1's or
   S1g.11.2's programs show is needed goes to the user with those programs; what the user rules built is built
   by the P1g.6 stage that owns it, and the geometry stage that needed it is finished after. Until then each
   stage takes the detour it names and says what the detour cost.

Against the rest of M1g: after P1g.1's stop, as every phase of the milestone is; before
[P1g.3](../p1g.3_proof_systems/README.md), by the user's words of 2026-11-11, *"between calculator and proof
systems"*, though no stage of P1g.3 waits for it. S1g.11.3 and
[S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md) ask Q-M1g.5 once, whichever runs first. It
shares no file with M1f's [P1f.10](../../m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/README.md)
or [P1f.20](../../m1f_hypothesis_and_documentation/p1f.20_docs_refactor/README.md) but the kernel pages P1f.20
moves, which it does not edit.

## First probes (2026-11-11)

Scratch programs the planning agent ran once while writing this tree, on the release binary found in
`ein.rs/target/release/` that day (built 2026-11-11 17:59; not rebuilt), with `ein solve FILE` unless said
otherwise. They are **not** results: nothing is banked, no test holds them, and each is **re-taken** by the stage
named, as a kept program. They are here so that the plan's expectations are told apart from its guesses. Each
program is reproduced as a sketch in the stage that re-takes it. The algebra's probes, A-1 to A-8, are in
[P1g.10's table](../p1g.10_school_algebra/README.md#first-probes-2026-11-11). All five below are also re-taken
by [S1g.11.1](s1g.11.1_what_school_geometry_is.md#the-survey-of-the-geometry-moved-here-from-s1f81-2026-11-13)'s
survey, as S1f.8.1 was to re-take them.

| # | the program | what the binary said | re-taken by |
|---|---|---|---|
| G-1 | An isosceles triangle, `AB = AC`; `SAS` and *corresponding angles of congruent triangles* as rules; three bookkeeping rules | `(ang-eq (ang A B C) (ang A C B))`, by Pappus's proof (the triangle is congruent to itself read the other way round); no candidate, k_s 1, 1 saturation, 10 firings. Without the `(solve …)` line: 24 candidates, k_s at least 72 012, 118 847 saturations | [S1g.11.2](s1g.11.2_theorems_as_rules.md) |
| G-2 | The same theorem by the school's proof: an hrule proposes the construction `(drawn (mid B C))`, then `SSS` | 1 candidate, 0 dead, k_s 2. The theorem holds in the state that drew the midpoint and **not at root**: the query over all of R has two classes, one *undetermined*; over the maximal line it has one, the theorem | [S1g.11.3](s1g.11.3_constructions_contradiction_cases.md) |
| G-3 | Two lines perpendicular to a third; an hrule supposes they meet; a rule makes that `(false)` | 2 candidates, **2 dead alone**, N 2, R {∅}. The supposition is refuted and reported; root does **not** hold `(parallel a b)`. M1g's probe 4 again, on a school theorem | S1g.11.3 |
| G-4 | A triangle with angles of 50 and 60 degrees; three rules: add the two, take the sum from 180, read the third | `(measure (ang B C A) (n (n z 7) 0))`; search-free | [S1g.11.4](s1g.11.4_computation_and_the_diagram_gap.md) |
| G-5 | (seen in G-1's `--trace`) a `:why` that prints a nested value | the value prints as written, *(seg A C) is as long as (seg A B)*, not by its own relation's `:why`, *the segment AC* | [`findings.md`](findings.md) F-3 |

## Acceptance

- **S1g.11.1's page was read and ruled on by the user** before any file outside `plans/`, `build/` and
  `docs/lib/` was written.
- **Each row of [§ What it is](#what-it-is) has a program whose answer the binary computed,** or the page that
  owns it says *this could not be done* and why, with the program that shows it.
- **Search-free where it should be, held by a test:** the forward proof of each target theorem has no
  candidate, k_s 1 and R {∅}, as the calculator's programs do (`ein-cli/tests/arith_search_free.rs` is the
  model). Where a stage uses the lattice (a construction, an assumption, a case), the page says so and what it
  cost. (Evaluation, simplification and a linear equation's root are
  [P1g.10's](../p1g.10_school_algebra/README.md#acceptance) half of this line.)
- **The trace is the school solution:** for one geometry theorem at least, the steps a pupil would write are
  read off the run, each a rule with its `:why` rendered, and a test holds them. What stands between today's
  trace and that reading is a row of a `findings.md` with a ruling, a fix or an owner: F-3 here, and F-2 of
  [P1g.10's](../p1g.10_school_algebra/findings.md). (The same for one linear equation is P1g.10's half of
  this line.)
- **Five school theorems,** each with what the program proved, by which rules, and what it needed written that a
  pupil reads off the picture; and at least one theorem the rule base **cannot** prove, with the reason.
- **No gap is silently filled.** A missing quantity and a missing fact of the figure are each reported by
  name; none is assumed. (A degenerate equation is P1g.10's.)
- **The report** ([S1g.11.4](s1g.11.4_computation_and_the_diagram_gap.md)) states which fragment of school algebra
  and geometry is a formal derivation in Ein today, and hands the rest to M1g's other phases by name. It is one
  report over both phases.
- **Every candidate of [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md) is
  answered by a program of this phase** at its close: shown needed, shown not needed, or not reached, with the
  program. (Added 2026-11-13 with the user's note; it asks for evidence and builds nothing.)
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

- **It may need the engine to change** (the user, 2026-11-13, of S1g.11.2: *"probably will require new features
  in the engine"*). The plan as written assumes the language as it ships and names a detour at each place it
  expects to be short: closure rules for spellings, a nested value for a constructed point, `:over maximal` for
  a theorem that is a state's, a script for the two-column proof. If the detours are not enough, the phase
  cannot finish on its own. What checks it: the candidates are written down before the programs,
  in [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md), each with the program
  that would show it; S1g.11.1's survey and S1g.11.2's targets are those programs; and nothing is designed or
  built before they have run and the user has ruled (the standing rule: real computed examples first, and an
  engine or grammar change needs the user's ruling).
- **Bookkeeping swamps the mathematics.** `AB` is `BA`, the angle `BAC` is `CAB`, a triangle has six readings,
  equality is symmetric and transitive. G-1 needed three such rules for one theorem. Written as forward rules
  they multiply every fact; S1g.11.1 chooses between closure rules, a canonical spelling and a symmetric
  declaration, on measurements.
- **The natural rules join on nested values.** `sas` joins two `triangle` facts and three facts on nested
  values, five premises, on a state where every triangle has several readings: the join the calculator's rule
  of the theory forbids, quadratic in a batch (E-3 of the calculator's
  [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)). One figure is small
  and does not notice. The phase **measures** each target on the matcher's counters and reports it. It does
  not build the index: that is [S1g.6.2](../p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md).
- **The trace buries the proof.** A fixpoint over a figure holds many true facts nobody asked for, so a
  two-column proof is a slice of the trace, the derivation of one fact, and not the trace
  ([S1g.11.2 § The two-column proof](s1g.11.2_theorems_as_rules.md#the-two-column-proof)). It is the geometry's
  form of P1g.10's risk *the trace buries the solution*, and of its F-2.
- **H is not empty by default** (G-1: 24 candidates and 118 847 saturations without the `(solve …)` line).
  Every program states its lattice or closes its relations, and the search-free test holds it.
- **A construction is not an assumption.** A hypothesis is the only way Ein adds something a rule did not derive,
  and G-2 shows its price: the theorem is in a state, not at root. If S1g.11.3 finds no honest reading, the phase
  says that school proofs with a construction are found by Ein and *certified* only by re-running with the
  construction written as a fact.
- **The geometry is unsound without the picture.** A rule that adds two angles is true only if a ray lies inside
  an angle. A rule base that leaves the side condition out proves false things about some figures. S1g.11.2 writes
  every such condition as a premise, and S1g.11.4's negative tests are figures on which the unconditioned rule
  would be wrong.
- **Scope.** Each of trigonometry, circles, similarity, areas and coordinates is a subject. Circles,
  similarity and areas are named where they come up and are not planned. (Inequalities are the algebra's,
  [P1g.10's](../p1g.10_school_algebra/README.md#risks-each-with-what-checks-it) optional part.)

## The questions for the user

They are asked at the stops named, on computed programs. They were not filed as `Q-M1f.<n>` when the phase was
planned, since P1f.6 was filing its own; since the move the phase files them in M1g's
[`open_questions.md`](../open_questions.md), as `Q-M1g.<n>` with the ids then free, when it starts. The letters
are P1f.8's and were not renumbered: a to f and j are
[P1g.10's](../p1g.10_school_algebra/README.md#the-questions-for-the-user).

| | the question | asked at | the plan's default |
|---|---|---|---|
| g | **The geometry's vocabulary and its level:** synthetic relations only, or measures as the calculator's numbers too | S1g.11.1 | both, kept apart: a synthetic rule base, and a measure layer that imports it and the calculator |
| h | **A construction:** a hypothesis, a Skolem-style value on request, or a fact the problem writes | S1g.11.3 | on request as a value where a rule can name it (`(mid B C)`); the *choice* of construction as a hypothesis, reported as search |
| i | **Does a refuted assumption conclude anything at root?** The same as [Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact); asked once, there or here, whichever stage runs first | S1g.11.3 | not built here; the limit is shown and the candidate named |

Three of P1g.10's questions are read here. **c**, where the theories live, is asked once at S1g.10.1 for both
phases. **d**, the normal form's order, meets S1g.11.1's canonical spelling in one want, an order on names.
**f**, whether the school solution is a view of the trace, is asked there with F-2, and the wish for a *proof
of this fact* read-out (S1g.11.2) is put to the user with it.

**Whether the engine changes** is not a question of this table. Each candidate of
[S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md) becomes a question for the
user only when a program of this phase has shown it, and is then asked by P1g.6 with that program.

## Names

The calculator's modules are under `std.math` since
[S1f.6.31](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md) (2026-11-14): the
decimal calculator is `std.math.decimal-arith`, `std.arith` before that stage. The relations kept their names. (From P1f.8's
§ Additions of 2026-11-12, whose four additions are otherwise all the algebra's,
[P1g.10's](../p1g.10_school_algebra/README.md#additions-of-2026-11-12).)

## What is left to M1g's other phases

So that the phases do not overlap (the section was *What is left to M1g* while the phase was M1f's; the
algebra's two entries are [P1g.10's](../p1g.10_school_algebra/README.md#what-is-left-to-m1gs-other-phases)):

- **What a proof is.** S1g.11.2's proofs are derivations Ein finds; a proof as *data* that Ein checks is
  [S1g.3.1](../p1g.3_proof_systems/s1g.3.1_hilbert_systems.md)'s.
- **Assumption and discharge.** S1g.11.3 shows reductio's premise (a dead commitment and its core) on school
  theorems. The rule that concludes the negation is
  [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md)'s.
- **Witnesses and fresh names.** S1g.11.3 uses a nested value for a constructed point and reports where that is
  not enough. Names made at run time are M1g's candidate C-8,
  [S1g.6.4](../p1g.6_engine_requirements/s1g.6.4_witnesses_and_fresh_names.md), with
  [Q-M1g.8](../open_questions.md#q-m1g8--fresh-names); this phase names it and does not build it.
- **Whether a system is complete.** S1g.11.1 reports what the literature says of Euclid, Hilbert and Tarski, and
  S1g.11.4 reports what the phase's rule base proved. Neither proves a meta-theorem;
  [F26](../../followups/f26_consistency_and_completeness.md)'s glossary of the senses of *complete* is where the
  word is sorted out, and [S1g.4.5](../p1g.4_first_order_logic/s1g.4.5_the_boundary.md) takes it.

## Connections

- **[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md):** the numbers, the request
  protocol, the search-free test, the harness against Rust's integers, the rule of the theory and its reason,
  and [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)'s E-3 and E-12,
  which this phase's rules meet head on.
  [S1f.6.20](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.20_word_problems.md) hands over its
  gaps and readings.
- **[P1g.10](../p1g.10_school_algebra/README.md):** the other half of what was P1f.8. The two meet where a
  figure's problem has an unknown (S1g.11.4 over S1g.10.4), where a theorem is a sum with letters for the
  angles (target 4 of S1g.11.2, over S1g.10.3's normal forms), in an order on names wanted twice (the
  canonical spelling here, question d there), and in the trace read at one theory's level (the two-column
  proof here, F-2 there).
- **[P1g.6](../p1g.6_engine_requirements/README.md):**
  [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md), the candidates of the
  user's note; and through it S1g.6.1 (a saturation that does not stop), S1g.6.2 (joins on nested values),
  S1g.6.3 (a nested value read as words), S1g.6.4 (a constructed point) and S1g.6.5 (a refutation a rule can
  read).
- **M1g's other phases:** [§ What is left to M1g's other phases](#what-is-left-to-m1gs-other-phases). In the
  other direction, this phase is M1g's first source of proofs that are not about formulas: S1g.3.3's reductio
  and S1g.4.4's witnesses can be tried on S1g.11.3's theorems.
- **[M2](../../m2_nl_to_ir/README.md):** each solved problem is an instance of
  [§ The reasoning as a program](../../m2_nl_to_ir/README.md#the-reasoning-as-a-program-the-users-idea-2026-11-11):
  the school solution as a self-explained Ein program, every step checked by the kernel. A school problem book
  is a benchmark with gold answers and gold *reasoning*, and the gap problems are instances whose right answer
  is *the figure does not say*.
- **[F26](../../followups/f26_consistency_and_completeness.md):** the senses of *complete*. School geometry is
  the plainest case of a system that is used as if complete and is not, and the diagram gap is F26's
  *under-determined by a reading* in a new domain.
- **[F15](../../followups/f15_math_formulae/ideas.md):** rules exported as formulas. A geometry rule has a
  school statement, a logical form and a picture. This phase's rule base is among the first where the exported
  formula is the thing the rule was written from, and S1g.11.2 records the formula beside each rule, in its
  `:why` or a comment, so that F15 has a corpus.
- **[Idea 09](../../ideas/09-puzzles-beyond-zebra.md):** puzzles beyond Zebra; it sets geometry-heavy puzzles
  (tangram, Soma) aside as out of reach. Nothing here changes that: this is deductive geometry over named
  points, not spatial search.
- **[Idea 08](../../ideas/08-human-style-deductive-trace.md):** the human-style trace. The acceptance line *the
  trace is the school solution* is that idea with a ground truth: a textbook's worked solution.
- **[`docs/lib/03`](../../../docs/lib/03-theorem-proving-formal-methods.md):** the external systems
  S1g.11.1 compares with.
