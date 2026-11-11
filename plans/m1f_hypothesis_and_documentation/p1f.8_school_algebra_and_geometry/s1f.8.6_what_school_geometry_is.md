# S1f.8.6 — What school geometry is, honestly, and how a figure is written

**Phase:** [P1f.8](README.md)
**Status:** planned 2026-11-11. Not started. **Stops for the user:** the vocabulary and the reading of the
literature, before [S1f.8.7](s1f.8.7_theorems_as_rules.md) writes a rule base on them.
**Depends on:** [S1f.8.1](s1f.8.1_survey_and_design.md)'s rulings. It needs no algebra;
[S1f.6.6](../p1f.6_calculator/s1f.6.6_division.md) for the measure layer.

## The instruction's part

> middle school geometry contains proofs, but I'm not sure it is 100% complete formal proof system (it starts in
> 5th grade)

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
  [S1f.8.8](s1f.8.8_constructions_contradiction_cases.md) meets.

**So the phase's question is not *is school geometry complete* but:** which steps of a school proof are formal
derivations Ein can check or find, and at which steps does the proof lean on the picture. The answer is a table
over real proofs, and [S1f.8.9](s1f.8.9_computation_and_the_diagram_gap.md)'s report is where it ends up.
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
    built in, the same want as S1f.8.3's monomial order; the order is then data, `(before A B)` for the
    figure's points, written by the problem or derived from one list;
  - *rules written to match every spelling.* Each theorem then has many rules or a disjunction.
- **Equality relations are equivalences.** Symmetry and transitivity of `seg-eq` and `ang-eq`. `std.algebra`
  (the stdlib's relation-algebra module) has `symmetric` and `transitive` declarations; whether they apply to a
  relation whose arguments are nested values is to be tried. Transitive closure over n equal segments is n²
  facts, and the usual remedy, a representative per class, again wants an order.
- **Non-degeneracy.** `(triangle A B C)` says three points are distinct and not collinear. School omits it;
  a rule without it proves things about a triangle with two vertices in one place.

The stage measures the three ways on G-1's theorem and on one with more points (the parallelogram of S1f.8.7),
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
probe G-1 declared it (one run, 2026-11-11; the program loaded and answered, see S1f.8.7), with the figure of a
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
6. **The stop:** the page, the proposal, questions g and (with S1f.8.3) d.

## Acceptance

- The literature page exists, each claim with its source, and each *to be checked* above is resolved or still
  marked.
- Three school proofs are tabulated step by step.
- The vocabulary is proposed with measurements for the three bookkeeping schemes, each a program the binary ran.
- The user has ruled on the vocabulary and the levels.
- Nothing outside `plans/`, `build/` and `docs/lib/` moved.

## What it may find

- **That an order on names is wanted twice** (canonical spellings here, monomial order in S1f.8.3). Two wants
  make a finding; it is still not asked of the engine without the user.
- **That the stdlib's `symmetric` and `transitive` do not take nested arguments as hoped.** Then it is a row of
  [`findings.md`](findings.md), and the phase writes its own closure rules.
- **That the reading of the literature changes the plan:** if a deductive database's standard rule set is
  published in a form that can be transcribed, S1f.8.7 starts from it and says so, in place of a rule base
  made up from a textbook's theorem list.
