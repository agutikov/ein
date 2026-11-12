# S1g.11.1 — What school geometry is, honestly, and how a figure is written

**Phase:** [P1g.11](README.md) of [M1g](../README.md). Until 2026-11-13 this stage was **S1f.8.6** of M1f's P1f.8
([README § The move](README.md#the-move-of-2026-11-13)); nothing of its content was redesigned.
**Status:** planned 2026-11-11. Not started. **Stops for the user, and since 2026-11-13 it is the phase's
stop:** the geometry's survey on a prototype, the vocabulary and the reading of the literature, before
[S1g.11.2](s1g.11.2_theorems_as_rules.md) writes a rule base on them. Nothing outside `plans/`, `build/` and
`docs/lib/` is written before the user has read its page.
**Depends on:** [S1g.10.1](../p1g.10_school_algebra/s1g.10.1_survey_and_design.md)'s rulings on what the two
phases share (where the theories live, how a program states its lattice). It needs no algebra;
[S1f.6.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.6_division.md) for the measure layer.

## The instruction's part

> middle school geometry contains proofs, but I'm not sure it is 100% complete formal proof system (it starts in
> 5th grade)

## The survey of the geometry, moved here from S1f.8.1 (2026-11-13)

Until the split, [S1f.8.1](../p1g.10_school_algebra/s1g.10.1_survey_and_design.md) surveyed both halves of
P1f.8 on one prototype and one page, and was the one stop before anything shipped. Its geometry half is folded
into this stage, which already stopped for the user, so that P1g.11 has a stop of its own. What came over, as
it was written there, with nothing added:

**The page.** `p1g.11_school_geometry/design.md`, as the calculator's
[`design.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/design.md) is: target programs written
for a prototype under `build/s1g.11/`, each run on the release binary, each shown with every artefact a run
writes (`ein solve`, `ein saturate --dump`, `ein test`, `--trace`, `--dump-answer`), and after them the
conventions proposed and the questions, each with a recommendation that a run supports. The commit at the stop
is docs only (with the entries this stage adds to `docs/lib/`). The user's standing rule for a new subject
(2026-10-16) is its reason: real programs first, their answers computed, before anything is implemented.

**The target programs,** under the numbers they had in S1f.8.1's table. Each is a real school problem, with the
sentence beside the facts; a problem that turns out not to be writable stays on the page with what happened.

| # | the problem | what it shows | probe |
|---|---|---|---|
| 10 | *In a triangle ABC, AB = AC. Prove that the angles at B and C are equal.* | a proof by saturation; the trace as a two-column proof | G-1 |
| 11 | The same, by the textbook's proof: draw the median | a construction, and where the theorem then lives | G-2 |
| 12 | *Two lines perpendicular to a third are parallel.* | a proof by contradiction, as far as it goes | G-3 |
| 13 | *Two angles of a triangle are 50° and 60°. Find the third.* | a computation in a figure | G-4 |
| 14 | *D is on BC. The angle BAD is 30° and DAC is 40°. Find BAC.* with *D is on BC* stated, and then left out | the diagram gap: the sum is right only if D is between, and the program says what it was not told | — |

**The probes re-taken.** Every geometry probe of [README § First probes](README.md#first-probes-2026-11-11),
G-1 to G-5, as a kept program: those were one run each, and here each is a file under `build/s1g.11/` with the
command that runs it.

**What the page proposes, of S1f.8.1's seven items,** each on the programs above:

- *how a program states its lattice* (item 3): G-1 without `(solve :hypothesis-relations ())` is a search over
  24 candidates. The remedy is the one S1g.10.1 shows on A-1 and rules; this page shows it on G-1;
- *the geometry's vocabulary* (item 5): S1f.8.1 showed it in outline and left it to this stage, so it is now
  shown once, here, in full ([§ How a figure is written](#how-a-figure-is-written));
- *where the theories live* and what they are called (item 6, question c of
  [P1g.10's README](../p1g.10_school_algebra/README.md#the-questions-for-the-user)): ruled at S1g.10.1's stop
  for both phases, and followed here;
- *what is measured for each theory* (item 7): firings and depth for one problem, and the matcher's candidates
  for a batch, as the calculator's stages record them.

Items 1, 2 and 4 (expressions, the names, the trace as the school solution) are the algebra's and stayed in
S1g.10.1. The geometry's form of item 4, the two-column proof, is
[S1g.11.2](s1g.11.2_theorems_as_rules.md#the-two-column-proof)'s.

**The findings re-taken:** F-3, F-4 and F-5 of [`findings.md`](findings.md), each confirmed as a finding with a
kept witness, or struck.

**What the programs show of the engine.** The user, 2026-11-13, on the stage that follows this one:
*"probably will require new features in the engine"*. Problems 10 to 14 are the first computed evidence for or
against each candidate of
[S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md), and the page says, per
candidate, what the run showed. It designs nothing.

## The user's doubt, and what is known about it

The doubt is right, and it has a history. **What follows is the planning agent's summary from memory, hedged
where it is unsure; every sentence of it is to be checked against sources by this stage's first task, and the
page it writes cites each.**

- **Euclid's *Elements* is not a complete formal system.** Its proofs use facts the postulates do not give, read
  off the diagram. The gaps usually named: **order** (which point is between which; that a line through a
  triangle's side leaves through another, stated as an axiom by Pasch in the 1880s); **continuity** (the very
  first proposition takes for granted that two circles meet); **superposition** (moving a triangle onto another,
  used for the side-angle-side criterion and justified by no postulate).
- **Hilbert's *Grundlagen der Geometrie* (1899)** repaired this with explicit groups of axioms: incidence, order,
  congruence, parallels, continuity. Side-angle-side is there an axiom. Its continuity axioms are not
  first-order, so *complete* in the sense of *proves every true first-order sentence* is not the right word for
  it; what is usually said is that it is categorical (to be checked).
- **Tarski's elementary geometry** is a first-order theory over points alone, with two relations, betweenness
  and equidistance. It is, as far as the planning agent knows, **complete and decidable**, by Tarski's decision
  method for real closed fields. *To be checked,* with the cost: the decision procedure is not something a pupil
  or a forward rule base does.
- **School courses follow neither.** They are Euclid-like, informal, and lean on the figure. Some school axiom
  systems build the real numbers in (Birkhoff's ruler and protractor postulates, and the textbook systems
  descended from them; the planning agent believes the school textbooks the user is likely to have met, by
  Pogorelov, Atanasyan, and Kolmogorov, each state axioms of their own and that the first two are metric in
  this sense: *to be checked*). A school proof's step is typically one of: a named theorem applied; a
  computation with measures; and *it is seen from the figure that …*.
- **A formal account of the diagram exists.** Avigad, Dean and Mumma's system for the *Elements* treats what is
  read off a diagram as inferences of their own kind, with rules (to be checked, and read: it is the nearest
  thing in the literature to this phase's *diagram gap*).
- **Machine proving of school geometry** has two families. **Algebraic:** coordinates and polynomial algebra
  (Wu's method, Gröbner bases), and the area method; strong, and not how a pupil proves. **Synthetic:** a
  **deductive database**, forward saturation of a fixed rule base over the figure's facts to a fixpoint (Chou,
  Gao and Zhang, about 2000); the symbolic engine of AlphaGeometry (2024) is, as the planning agent understands
  it, of this kind, joined with algebraic reasoning over angles and ratios, and a language model that proposes
  auxiliary constructions. *To be checked against the papers.* If it is so, **Ein's saturation is that method**,
  and the division of labour there, saturation for deduction and something else for constructions, is the one
  [S1g.11.3](s1g.11.3_constructions_contradiction_cases.md) meets.

**So the phase's question is not *is school geometry complete* but:** which steps of a school proof are formal
derivations Ein can check or find, and at which steps does the proof lean on the picture. The answer is a table
over real proofs, and [S1g.11.4](s1g.11.4_computation_and_the_diagram_gap.md)'s report is where it ends up.
The senses of *complete* themselves are [F26](../../followups/f26_consistency_and_completeness.md)'s.

## How a figure is written

The figure is facts. Points are names. Everything else is a choice, and the stage makes each on a measurement.

**Objects as values.** `(seg A B)`, `(ang A B C)` (the angle at B), `(tri A B C)`, `(line A B)`: nested facts of
ordinary relations, values under the value rule
([`defined_behaviour.md` § 4.22](../../../docs/kernel/defined_behaviour.md)). A named line, `l`, is a name.

**Relations.** As planned: `(on P l)` incidence; `(collinear A B C)`; `(between A B C)`, B between A and C;
`(parallel l m)`, `(perp l m)`; `(seg-eq S T)`, equal length; `(ang-eq X Y)`; `(cong T U)`, congruent with
vertices in corresponding order; `(midpoint M A B)`; `(right X)`, a right angle; `(triangle A B C)`, the three
points are not collinear.

**The bookkeeping, which is the real work of this stage.** Probe G-1 proved one theorem and needed three rules
that are not geometry:

- **One object, several spellings.** `(seg A B)` and `(seg B A)` are two values and one segment. `(ang B A C)`
  and `(ang C A B)` likewise. A triangle has six. Three ways:
  - *closure rules,* as G-1 had: derive every respelling of every fact. Simple and multiplies the state: a
    `seg-eq` fact has 4 spellings, with symmetry 8; a `cong` fact 6, with symmetry 12;
  - *a canonical spelling,* the points in a fixed order. That needs an order on names, which Ein does not have
    built in, the same want as S1g.10.3's monomial order; the order is then data, `(before A B)` for the
    figure's points, written by the problem or derived from one list;
  - *rules written to match every spelling.* Each theorem then has many rules or a disjunction.
- **Equality relations are equivalences.** Symmetry and transitivity of `seg-eq` and `ang-eq`. `std.algebra`
  (the stdlib's relation-algebra module) has `symmetric` and `transitive` declarations; whether they apply to a
  relation whose arguments are nested values is to be tried. Transitive closure over n equal segments is n²
  facts, and the usual remedy, a representative per class, again wants an order.
- **Non-degeneracy.** `(triangle A B C)` says three points are distinct and not collinear. School omits it;
  a rule without it proves things about a triangle with two vertices in one place.

The stage measures the three ways on G-1's theorem and on one with more points (the parallelogram of S1g.11.2),
by facts in the state and by the matcher's candidates, and proposes one.

**Measures.** Two levels, kept apart as the calculator and the word-problem theory are:

- **synthetic:** `ang-eq`, `seg-eq`, `right`. No numbers. Most congruence proofs live here;
- **metric:** `(measure X N)`, `(length S N)`, N one of the calculator's numbers, in degrees and in a unit the
  problem names. Angle sums and perimeters live here. A rule joins the two: equal measures give `ang-eq`, and
  `ang-eq` carries a known measure across.

*The angles of a triangle add up to two right angles* can be said at either level. Synthetically it needs
angle addition as a relation, `(ang-sum X Y Z)`, and that is where betweenness enters: X and Y add to Z only if
a ray is inside Z. Whether the synthetic level gets angle addition at all, or sums are metric only, is put to
the user (question g of the [README](README.md#the-questions-for-the-user)).

**How the nested values read.** F-3 of [`findings.md`](findings.md): a `:why` prints `(seg A C)` as written, not
*the segment AC*. A two-column proof whose statements read *(ang A B C) equals (ang A C B)* is not yet a school
proof. The stage establishes what [`09_natural_language.md`](../../../docs/kernel/ir/03-ein-lang/09_natural_language.md)
says of a nested value inside a template, and files the row as a request or as a wrong answer accordingly.

## Sketch

**Sketch only, to be replaced by programs whose answers are computed.** The vocabulary as the planning agent's
probe G-1 declared it (one run, 2026-11-11; the program loaded and answered, see S1g.11.2), with the figure of a
school problem: *in the triangle ABC, AB = AC*.

```lisp
(relation seg Point Point         :why "the segment {?1}{?2}")
(relation ang Point Point Point   :why "the angle {?1}{?2}{?3}, at {?2}")
(relation tri Point Point Point   :why "the triangle {?1}{?2}{?3}")
(relation triangle Point Point Point :why "{?1}{?2}{?3} is a triangle")
(relation seg-eq Segment Segment  :why "{?1} is as long as {?2}")
(relation ang-eq Angle Angle      :why "{?1} equals {?2}")
(relation cong Triangle Triangle  :why "{?1} is congruent to {?2}, vertex for vertex")

(triangle A B C)
(seg-eq (seg A B) (seg A C))

;; the three rules that are bookkeeping and not geometry
(rule relabel ()
  :match  (triangle ?a ?b ?c)
  :assert (triangle ?a ?c ?b)
  :why    "a triangle read the other way round is the same triangle")
(rule seg-eq-sym ()
  :match  (seg-eq ?s ?t)
  :assert (seg-eq ?t ?s)
  :why    "if {?s} is as long as {?t}, {?t} is as long as {?s}")
(rule angle-read-back ()
  :match  (triangle ?a ?b ?c)
  :assert (ang-eq (ang ?b ?a ?c) (ang ?c ?a ?b))
  :why    "an angle is the same angle read from its other side")
```

`relabel` here gives two of a triangle's six readings, which was enough for that proof and is not the closure.

## Tasks

0. **The prototype and the survey's page** ([§ The survey of the geometry](#the-survey-of-the-geometry-moved-here-from-s1f81-2026-11-13)):
   problems 10 to 14 as programs under `build/s1g.11/`, probes G-1 to G-5 re-taken, the measurements, F-3 to
   F-5 re-taken, and what the runs show of S1g.6.7's candidates. (S1f.8.1's T1, the geometry's part.)
1. **The reading of the literature,** each claim of [§ The user's doubt](#the-users-doubt-and-what-is-known-about-it)
   checked against a source and corrected, as a page of the phase, with entries added to
   [`docs/lib/03`](../../../docs/lib/03-theorem-proving-formal-methods.md) for the systems it compares with
   (the deductive-database provers, the formal systems for the *Elements*, Tarski's theory). One paragraph on
   what the school textbooks' own axioms are, for two or three textbooks.
2. **Three school proofs, taken apart step by step:** each step classed as *a theorem applied*, *a computation*,
   or *read off the figure*. Real proofs from a textbook, cited. This table is the stage's answer to the user's
   doubt in miniature, before any rule exists.
3. **The vocabulary,** with the bookkeeping measured three ways, and the proposal.
4. **The two levels,** with the rules that join them.
5. **F-3** established and filed.
6. **The stop:** the page, the proposal, question g, and the order on names it wants, read with question d of
   [P1g.10](../p1g.10_school_algebra/README.md#the-questions-for-the-user) (the normal form's order, S1g.10.3's;
   it was *questions g and (with S1f.8.3) d* before the split). The user reads the page; the rulings are
   recorded on it in the user's words. (S1f.8.1's T2, the geometry's part.)
7. **The re-plan:** this phase's README and stage files are restated to the rulings, and its questions are
   filed in [`open_questions.md`](../open_questions.md) with ids then free. (S1f.8.1's T3, the geometry's part.)

## Acceptance

- The survey's page exists, every program on it was run by the binary, and every number on it carries its
  command and date.
- Every geometry probe of the README's table, G-1 to G-5, is re-taken, and each row says *confirmed* or how it
  differed.
- Each of F-3, F-4 and F-5 is confirmed as a finding with a kept witness, or struck.
- The literature page exists, each claim with its source, and each *to be checked* above is resolved or still
  marked.
- Three school proofs are tabulated step by step.
- The vocabulary is proposed with measurements for the three bookkeeping schemes, each a program the binary ran.
- The user has ruled on the vocabulary and the levels, or has said which questions wait.
- Nothing outside `plans/`, `build/` and `docs/lib/` moved.

## What it may find

- **That an order on names is wanted twice** (canonical spellings here, monomial order in S1g.10.3). Two wants
  make a finding; it is still not asked of the engine without the user.
- **That the stdlib's `symmetric` and `transitive` do not take nested arguments as hoped.** Then it is a row of
  [`findings.md`](findings.md), and the phase writes its own closure rules.
- **That a prototype needs the calculator's extensions before a problem can be shown** (S1f.8.1's second
  *may find*): problem 13 needs none; a measure problem with a sign or a fraction waits, and the page then goes
  to the user in two rounds.
- **That the reading of the literature changes the plan:** if a deductive database's standard rule set is
  published in a form that can be transcribed, S1g.11.2 starts from it and says so, in place of a rule base
  made up from a textbook's theorem list.
