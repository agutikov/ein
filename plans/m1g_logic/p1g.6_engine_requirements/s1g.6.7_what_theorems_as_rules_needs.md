# S1g.6.7 — What *theorems as rules* needs of the engine: the candidates, each a question with its witness

**Phase:** [P1g.6](README.md) — candidates T-1 to T-10 below, six of them existing candidates of the phase seen
from school geometry (C-1, C-2, C-6, C-8, C-9, C-11) and four with no id yet
**Status:** planned 2026-11-13, on the user's note of that day. Not started. Provisional until P1g.1's stop, as
the phase's other stages are. **It lists candidates and designs nothing.**
**Depends on:** the computed programs of
[S1g.11.1](../p1g.11_school_geometry/s1g.11.1_what_school_geometry_is.md) (the geometry's survey, problems 10
to 14) and [S1g.11.2](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md) (the rule base and its five
targets), which are its evidence. Nothing here runs before them.
**Needed by:** [P1g.11](../p1g.11_school_geometry/README.md), school geometry, and S1g.11.2 first.

## Where it comes from

On 2026-11-13 the user split M1f's P1f.8 into two phases of M1g and said of the stage that is now S1g.11.2,
*theorems as rules*:

> plans/m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/s1f.8.7_theorems_as_rules.md
> probably will require new features in the engine

That stage, and the two after it, already name the places where the plan expects the language as it ships to be
short, each with the detour it takes: in *What it may find*, in the risks of the phase, and in three rows of
its `findings.md`. This file gathers them in one place, so that the engine-requirements phase holds the
geometry's candidates beside the logic's. It adds no candidate the plans do not already state.

## The rule this stage is held to

**Candidates only.** No row below is a requirement, a design or a promise. Each is a question, and beside it
the program that would answer it.

**What is needed is found by S1g.11.1's and S1g.11.2's computed programs first.** The user's standing rule
(2026-10-16) is real programs, their answers computed, before anything is designed. So a row becomes a
requirement only when a kept program of P1g.11 has shown that the detour is not enough, with what the detour
cost. A row the programs do not bear out is struck, with the run that struck it.

**An engine or grammar change needs the user's ruling,** on those programs
([P1g.6 § The discipline](README.md#the-discipline)). What the user rules built is built by the stage of this
phase that owns the candidate, never by a stage of P1g.11.

## The candidates

*Where the plans say it* cites the page that already states the want. *The witness* is the program that would
show it; none has been run for this purpose. *Today's detour* is what the geometry's stages do in the language
as it ships. *Owner if needed* is the stage of this phase that would take it; **new** means P1g.6's table has
no candidate for it today, and it gets a C-id only if a program shows it.

| id | the question | where the plans say it | the witness that would show it | today's detour | owner if needed |
|---|---|---|---|---|---|
| T-1 | **A point a construction makes.** Is a nested value, `(mid B C)`, enough for *draw the median*, or does a construction need something the engine lacks: a name made at run time, or a way to say *this may be added and changes nothing that was true*? | [S1g.11.3 § 1](../p1g.11_school_geometry/s1g.11.3_constructions_contradiction_cases.md#1-auxiliary-constructions); F-4 of [P1g.11's `findings.md`](../p1g.11_school_geometry/findings.md); probe G-2 | problem 11 of S1g.11.1's survey (the isosceles theorem by the median); target 5 of S1g.11.2 (the midline), which the plan expects to fail without a construction | the point as a value on request; the choice of construction as a hypothesis, read `:over maximal`; or the construction written as a fact and the proof checked, not found | C-8, [S1g.6.4](s1g.6.4_witnesses_and_fresh_names.md), [Q-M1g.8](../open_questions.md#q-m1g8--fresh-names) |
| T-2 | **One figure, several spellings.** `(seg A B)` and `(seg B A)` are two values and one segment; an angle has two spellings and a triangle six. Do closure rules or a canonical spelling written by the program suffice, or is something wanted of the engine: an **order on names** as a computed predicate beside `eq` and `neq`, or a value read up to the order of its arguments? | [S1g.11.1 § How a figure is written](../p1g.11_school_geometry/s1g.11.1_what_school_geometry_is.md#how-a-figure-is-written) and its first *may find* (*an order on names is wanted twice*); [S1g.6.4](s1g.6.4_witnesses_and_fresh_names.md)'s last *may find* | the three bookkeeping schemes measured on G-1's theorem and on the parallelogram (S1g.11.1's task 3): facts in the state and the matcher's candidates under each | closure rules, as G-1 had; or the order written as facts by the problem, `(before A B)` | **new.** The order on names is named in S1g.6.4 as what may be *the real requirement behind C-8* |
| T-3 | **A proof read off a run.** A two-column proof is the derivation of one fact back to the givens, each fact once, each reason a rule's `:why`. Does the binary need a *proof of this fact* read-out, or does a script over `--trace` and a `--dump-answer` state file do? And **which justification is shown** where a fact has several: the first the store recorded, which may vary with rule order | [S1g.11.2 § The two-column proof](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md#the-two-column-proof); F-2 of [P1g.10's `findings.md`](../p1g.10_school_algebra/findings.md) and its question f, the same wish from the algebra | target 1's proof written from G-1's ten-step trace, by the script; the same target under two rule orders, to see whether two proofs print | a script under `utils/` or `build/`; the proof shown is one explanation and the page says so (the user's ruling of 2026-11-08: a core is an explanation; F22 makes them canonical) | **new** as a read-out; its ruling is question f's, asked at S1g.10.1 and S1g.10.4 |
| T-4 | **A nested value read as words.** A `:why` prints `(seg A C)` as written, not *the segment AC*, so a two-column proof's statements are not yet a school proof's | F-3 of P1g.11's `findings.md`; probe G-5; [S1g.11.1](../p1g.11_school_geometry/s1g.11.1_what_school_geometry_is.md)'s task 5 | G-1's `--trace` and a query's `:goal-text`, set against what [`09_natural_language.md`](../../../docs/kernel/ir/03-ein-lang/09_natural_language.md) states of a nested value inside a template | none: the statements print as Ein text | C-6, [S1g.6.3](s1g.6.3_seeing_nested_facts.md); a register row instead, if the page says the other behaviour |
| T-5 | **Equality classes of segments and angles.** `seg-eq` and `ang-eq` are equivalences: symmetry doubles each fact and transitivity over n equal segments is n² facts. Do the stdlib's `symmetric` and `transitive` take a relation whose arguments are nested values, and is the closure affordable, or is a representative per class wanted, which again wants an order on names? | [S1g.11.1 § How a figure is written](../p1g.11_school_geometry/s1g.11.1_what_school_geometry_is.md#how-a-figure-is-written) (*equality relations are equivalences*) and its second *may find*; [S1g.11.2](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md)'s first *may find* (*the state grows faster than the figure*) | `std.algebra`'s two declarations on `seg-eq` over nested arguments; the state's size and the matcher's candidates on target 3, the parallelogram, where the equal segments and angles are many | the phase's own closure rules | **new.** It shares T-2's order on names. Note that `eq` is computed and never stored (M1f S1f.5.7), so any engine notion of a class of equal values would meet that ruling |
| T-6 | **A saturation that does not stop.** A rule that constructs unasked floods: the midpoint of every segment, then segments to those midpoints, then their midpoints. Is writing every construction on request enough, under a test's timeout, or does the geometry need the engine's bound and word? | [S1g.11.3 § 1](../p1g.11_school_geometry/s1g.11.3_constructions_contradiction_cases.md#1-auxiliary-constructions), way (a)'s expected limit; M1g's probe 3 | way (a) written with no request, on one triangle, under `timeout`; then on request | every construction demand-driven, `(construct (mid B C))`; each test under a timeout that fails loudly | C-1, [S1g.6.1](s1g.6.1_a_saturation_that_does_not_stop.md), [Q-M1g.6](../open_questions.md#q-m1g6--the-word-for-a-saturation-that-does-not-stop) |
| T-7 | **A conclusion that is a state's, or no state's.** A refuted assumption is reported and root holds nothing of it; a theorem under a construction holds in the state that drew it; a proof by cases holds *in every state of R*, which is read off the read-out and not derived. Does a school proof need a rule to read another state? | [S1g.11.3 § 2 and § 3](../p1g.11_school_geometry/s1g.11.3_constructions_contradiction_cases.md); F-4 and F-5; probes G-2 and G-3 | problem 12 (two perpendiculars to a third); the altitude's foot by cases, one projection class; the false proof that every triangle is isosceles | the read-out is the proof: the dead commitment's core, `:over maximal`, one projection class; or two runs, search then check | C-11, [S1g.6.5](s1g.6.5_rules_as_data_and_refutations.md), [Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact), asked once by whichever of S1g.11.3 and S1g.3.3 runs first |
| T-8 | **A hypothesis about a nested value, named in the program's list.** Can an hrule be told to propose a construction for only the segments a request names, and can `(solve :hrules …)` name it? | [S1g.11.3](../p1g.11_school_geometry/s1g.11.3_constructions_contradiction_cases.md)'s second *may find*; M1g's probe 4b | G-2 with its construction named in the `(solve …)` list | a marker fact and an hrule with no parameter | C-9, [S1g.6.4](s1g.6.4_witnesses_and_fresh_names.md), [Q-M1g.7](../open_questions.md#q-m1g7--a-hypothesis-about-a-nested-fact) |
| T-9 | **Joins on nested values.** `sas` joins two `triangle` facts and three facts on nested values, five premises, on a state where every triangle has several readings. Is one figure small enough not to notice, and what does a batch of figures cost? | [S1g.11.2](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md)'s sketch and its second *may find*; E-3 of the calculator's findings | the matcher's candidates for each of the five targets, and for a batch of figures | none taken: it is measured and reported, and tags, the calculator's remedy, are tried if a target is slow | C-2, [S1g.6.2](s1g.6.2_nested_facts_at_scale.md) |
| T-10 | **A check that a reading is sound.** `:over maximal` reads a theorem off the state with everything drawn, which is right only while no candidate of the program restricts, *and nothing checks that*. Is such a check wanted, and is it the engine's? | [S1g.11.3 § 1](../p1g.11_school_geometry/s1g.11.3_constructions_contradiction_cases.md#1-auxiliary-constructions) (*read the maximal line*) and its first *may find* | G-2 beside a variant in which one candidate is a real guess, both read `:over maximal` | the page states when the reading is sound, case by case | **new.** It is T-1's question from the other side: a construction said to be conservative needs no such check |

**Not candidates,** though the geometry will meet them. *H is not empty by default* (G-1 is 24 candidates
without its `(solve …)` line): ruled, the author states the lattice. *Measures*: the calculator's numbers, and
nothing adds engine arithmetic. *The picture*: what a figure does not state is a gap the theory names
([S1g.11.4](../p1g.11_school_geometry/s1g.11.4_computation_and_the_diagram_gap.md)), and where its detection
by `absent` meets *forbid* it follows S1f.6.20's outcome. *A rule base in layers*, a target importing only the
theorems before it, and *the mutation sweep pointed at a rule base outside `stdlib/`*: the first is imports as
they ship, to be tried, and the second a script. Each stays with its owner.

## Tasks

1. **T1g.6.7.1 — the survey's evidence.** After
   [S1g.11.1](../p1g.11_school_geometry/s1g.11.1_what_school_geometry_is.md)'s stop: each row above set against
   the runs of problems 10 to 14 and of the three bookkeeping schemes. Per row: *shown*, with the program and
   what the detour cost; *not shown*; or *not reached*.
2. **T1g.6.7.2 — the rule base's evidence.** After
   [S1g.11.2](../p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md): the same against the five targets, the
   theorem the base could not prove and its two-column proofs. A want the programs met that is not in the table
   is added as a row, with its program.
3. **T1g.6.7.3 — the page for the user.** The rows the programs showed, each with its program, today's detour
   and the detour's measured cost, and the stage of this phase that would own it. No design: the design of a
   candidate the user wants looked at is its owner's first task, on the same programs.
4. **T1g.6.7.4 — the hand-over.** What the user rules is recorded in the user's words. A row with an existing
   candidate goes to its stage as evidence from outside logic, as
   [S1g.11.3](../p1g.11_school_geometry/s1g.11.3_constructions_contradiction_cases.md) already plans for C-8. A
   **new** row the user wants taken up gets a C-id in [P1g.6's table](README.md#the-candidates-as-seeded-2026-11-11)
   and a stage. A row the user rules out is recorded with the detour that stands.

## Acceptance

- Every row is in one of three states with a program beside it: shown needed, shown not needed, or not reached.
- No row was designed or built by this stage, and no engine source, grammar file or stdlib module moved for it.
- The user has the page, and each row the programs showed has a ruling, or is said to wait.
- At the phase's close each row is in one of the four states of [P1g.6 § Acceptance](README.md#acceptance):
  built by its owner, ruled not to be built, detoured, or owned by a followup.

## What it must not do

- **Not turn a candidate into a plan** before a program of P1g.11 has shown it.
- **Not hold P1g.11 back.** S1g.11.1 and S1g.11.2 write their programs in the language as it ships and take the
  detours named; this stage reads their runs and runs none of its own.
- **Not add engine arithmetic, a number token or a new predicate** on its own word. An order on names (T-2,
  T-5) would be a computed predicate beside `eq` and `neq`: a question for the user if and only if a stage
  cannot do without it, with the detour measured first, as S1g.6.4 already says.
- **Not change the grammar** (T-8) without the user's ruling on the programs written both ways.

## The discipline

[P1g.6 § The discipline](README.md#the-discipline): a wrong answer is a register row before it is fixed; a
change to the grammar needs the user's ruling; config never changes the answer; no native numbers; one tool,
consistent answers; minimum work per question; every sweep names the goldens it expects to move before
blessing.
