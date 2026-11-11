# S1f.8.9 — Computation problems in geometry, the diagram gap detected and named, and the phase's report

**Phase:** [P1f.8](README.md)
**Status:** planned 2026-11-11. Not started. Its report is **shown** to the user; it asks nothing new.
**Depends on:** [S1f.8.7](s1f.8.7_theorems_as_rules.md), the rule base and its metric layer;
[S1f.8.8](s1f.8.8_constructions_contradiction_cases.md), cases; [S1f.8.4](s1f.8.4_linear_equations.md) for
problems with an unknown; [S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md), whose pattern it follows
and whose ruled gap mechanism it uses.

## The instruction's part

> middle school geometry contains proofs, but I'm not sure it is 100% complete formal proof system

Most of a school geometry lesson is not proof. It is *find the angle*, *find the perimeter*, and the proof is in
the justification of each step. And the user's doubt is at its sharpest in exactly those steps a pupil does not
justify, because the picture shows them.

## 1. Computation problems

A geometry computation is a word problem whose words are a figure. The pattern is
[S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md)'s: facts of the problem; a theory that turns them
into the calculator's requests, a rule a reading, each with its `:why`; the calculator's answers read back as
facts about the figure. The theory here is S1f.8.7's rule base at the metric level.

| problem | what it shows |
|---|---|
| *Two angles of a triangle are 50° and 60°. Find the third.* | the angle sum as two requests; probe G-4 |
| *An isosceles triangle has an angle of 40° at the apex. Find the base angles.* | a theorem (equal base angles) and then an equation, `2b + 40 = 180`: the rule base hands S1f.8.4 an equation |
| *One angle of a triangle is twice another, and the third is 60°.* | an unknown named by the problem; a linear equation over the figure |
| *A rectangle is 12 by 25. Find its perimeter and its area.* | [`examples/arith/mul.ein`](../../../examples/arith/mul.ein)'s program with the words as facts; a unit carried and named |
| *Lines a and b are parallel; a transversal makes an angle of 70° with a. Find the eight angles.* | several theorems chained; each answer's trace names its theorems |
| *A triangle has angles of 100° and 90°.* | a figure that cannot exist: what the program says, as S1f.6.20's *gave 7 of 5* |
| *An angle of a triangle is 50°. Find the others.* | a missing quantity, named |

**Units and what the model leaves out.** Degrees and a length's unit are names the problem states; an area's
unit is a product the theory does not compute and says so. S1f.6.20's *the problem is a model*
([its section](../p1f.6_calculator/s1f.6.20_word_problems.md#the-problem-is-a-model-and-says-where-it-ends))
applies with a new entry in the list of what the model leaves out: **the theory does not know where the points
are.** It has no coordinates. Everything it knows of position is the facts of incidence and order it was given.

## 2. The diagram gap

*D is on BC. The angle BAD is 30° and DAC is 40°. Find BAC.* A pupil answers 70° at a glance. The answer is
right only if D is **between** B and C. If D is on the line beyond B, the angle is 10°; beyond C, no such figure
exists, since 30 is less than 40. The problem's text does not say; the picture does.

The gaps of this kind, as the planning agent expects them, to be confirmed on S1f.8.6's three proofs taken
apart:

| what the picture shows | where a rule needs it |
|---|---|
| a point lies **between** two others | adding segments; adding angles; *the angle ABM is the angle ABC* (probe G-2's cheat) |
| two points are on the **same side** of a line, or on opposite sides | alternate against corresponding angles; whether a quadrilateral is convex |
| two lines **meet**, and where | the parallelogram's diagonals; any *let O be the intersection* |
| a point is **inside** a triangle or an angle | every proof that cuts a figure in two |
| three points are **not collinear**, two points are distinct | every rule about a triangle |

**The detection.** [S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md) detects a missing quantity: the
theory's own rules ask for something the problem does not give. This is the same, with a relation of position
in place of a number. Each rule of S1f.8.7 that has a side condition is the place: where every other premise
holds and the side condition is neither stated nor derived, the theory reports *this step needs to know whether
D is between B and C*. The mechanism is whichever S1f.6.20's stop ruled (an obligation, so the state owes; a
derived `needs` fact by `absent`; or the reading as a hypothesis); this stage does not choose another. It tries
the ruled one on position and reports what differs:

- **as a missing fact:** the answer is withheld and the gap named. The pupil's 70° is not given;
- **as a missing reading, by hypotheses** (the user's lean at S1f.6.20: *"readings as hypotheses"*): *D between
  B and C*, *C between B and D*, *B between D and C* are three candidates, one of which holds; R has the
  surviving positions and the query's projection classes are the answers: 70° under the first, 10° under the
  third, each named with its position, and the second dead, with a core that says why (the part would exceed
  the whole). This is S1f.8.8's case split, arrived at from the other side;
- **never silently assumed.** No rule of the base has a default position.

Position facts are **non-monotone to detect** (a `needs` fact rests on an `absent`), and S1f.6.20's file says
what that costs: the module that detects is kept apart from the rule base, which stays monotone, as the
word-problem module is kept apart from the calculator.

**What the theory will not detect,** said once on the page: a gap *outside* the model. It reports a position
its own rules ask for. It does not know that a figure drawn to scale would have shown the triangle cannot
close.

## 3. The report

`p1f.8_school_algebra_and_geometry/report.md`, the phase's answer to the instruction, read off the stages and
not written from memory. Each row cites the program and the command.

- **The fragment map.** For each school activity of the phase's
  [README § What it is](README.md#what-it-is): done by saturation alone; done by the lattice, at what cost; done
  only with something written that a pupil would not write; not done, and why.
- **The user's doubt, answered for what was tried.** Over the proofs the phase wrote: how many steps were a
  rule applied, how many a computation, how many a fact of the picture that had to be stated. Beside it, what
  S1f.8.6 found in the literature about the systems that *are* complete. The report does not claim more than
  that table: a count over a handful of proofs is not a theorem about school geometry.
- **The trace as the school solution:** what was achieved, shown on one equation and one theorem beside the
  textbook's text.
- **What the two halves share:** an order on names wanted twice, bookkeeping of spellings, the request
  protocol, joins on nested values, a reading of the trace at one theory's level.
- **The handover to M1g,** by name: equational proof to P1g.3; assumption and discharge to S1g.3.3; witnesses
  and fresh names to S1g.6.4 with this phase's constructions as evidence; induction to P1g.5; the senses of
  *complete* to S1g.4.5 and [F26](../../followups/f26_consistency_and_completeness.md). Each with the program of
  this phase that shows the need.
- **For [M2](../../m2_nl_to_ir/README.md):** the problems as pairs, the sentence and the facts; the solved
  programs as instances of
  [the reasoning as a program](../../m2_nl_to_ir/README.md#the-reasoning-as-a-program-the-users-idea-2026-11-11);
  the gap problems as instances whose right answer is *the problem does not say*.
- **[`findings.md`](findings.md), closed:** every row fixed, ruled or owned.

## Sketch

**Sketch only, to be replaced by programs whose answers are computed.** Probe G-4, as the planning agent ran it
once on 2026-11-11: the binary answered `(measure (ang B C A) (n (n z 7) 0))`, the claim held, no candidate,
k_s 1.

```lisp
(import std.arith :symbols (arith))

(relation triangle Point Point Point :why "{?1}{?2}{?3} is a triangle")
(relation ang Point Point Point      :why "the angle {?1}{?2}{?3}, at {?2}")
(relation measure Angle Number       :why "{?1} measures {?2} degrees")

;; "Two angles of a triangle are 50 and 60 degrees. Find the third."
(triangle A B C)
(measure (ang C A B) (n (n z 5) 0))
(measure (ang A B C) (n (n z 6) 0))

(rule two-angles-known ()
  :match  (and (triangle ?a ?b ?c) (measure (ang ?c ?a ?b) ?x) (measure (ang ?a ?b ?c) ?y))
  :assert (add ?x ?y)
  :why    "two angles of {?a}{?b}{?c} are known, {?x} and {?y}: add them")
(rule rest-of-180 ()
  :match  (and (triangle ?a ?b ?c) (measure (ang ?c ?a ?b) ?x) (measure (ang ?a ?b ?c) ?y)
               (sum-of ?x ?y ?s))
  :assert (sub (n (n (n z 1) 8) 0) ?s)
  :why    "the angles of a triangle add up to 180: take {?s} from 180")
(rule third-angle ()
  :match  (and (triangle ?a ?b ?c) (measure (ang ?c ?a ?b) ?x) (measure (ang ?a ?b ?c) ?y)
               (sum-of ?x ?y ?s) (diff-of (n (n (n z 1) 8) 0) ?s ?t z))
  :assert (measure (ang ?b ?c ?a) ?t)
  :why    "so the third angle, at {?c}, is {?t}")

(solve :hypothesis-relations ())
(query :goal (measure (ang B C A) ?t)
       :expect (model (measure (ang B C A) (n (n z 7) 0))))
```

What the sketch leaves for the stage: it knows the two angles only in the spellings and the order written (the
bookkeeping of S1f.8.6); with angles of 100 and 90 the difference is *short by 10* and the third rule does not
fire, so the impossible triangle is answered by silence, which is what the stage must replace with a statement.

The diagram gap, **not run**: the facts of the problem and the two readings the theory is to tell apart.

```lisp
;; "D is on BC. The angle BAD is 30 and DAC is 40. Find BAC."
(collinear B D C)
(measure (ang B A D) (n (n z 3) 0))
(measure (ang D A C) (n (n z 4) 0))
;; not stated: (between B D C).  With it, BAC is 70. With (between D B C), BAC is 10.
;; With (between B C D) there is no such figure: BAD would be BAC and 40 more, and it is 30.
```

## Tasks

1. **The computation problems,** each a program over the rule base and the calculator, each sentence beside its
   facts.
2. **The gap table,** from the proofs and problems actually written.
3. **The detection,** by S1f.6.20's ruled mechanism, as a module apart; the angle problem under both readings.
4. **The negative figures** S1f.8.7 asked for, completed: for each rule with a position premise, a figure on
   which the rule without that premise would derive something false.
5. **The tests:** every answer against the harness's integers; each gap problem held to its named gap; the
   impossible figure held to its stated outcome; search-free where no reading is searched.
6. **The report,** and [`findings.md`](findings.md) closed.
7. **The examples** as corpus entries, each with an `:expect`.

## Acceptance

- Each computation problem answers, and its trace names the theorems it used; the engine is unchanged by the
  stage.
- **No answer rests on a position the problem did not state.** The angle problem without *between* gives no
  single answer: it names the gap, or gives both answers each under its named position.
- Every rule with a position premise has a negative figure, held by a test.
- The report exists, every row of it with its program and command, and it says in plain words what of the
  user's doubt the phase could answer and what it could not.
- `findings.md` has no row without a ruling, a fix or an owner.

## What it may find

- **That position facts are most of a figure.** If a proof of five lines needs fifteen facts of order and side
  to be sound, the report says so with the count: that *is* the measure of how far school geometry is from a
  formal system, on the proofs tried.
- **That detection by `absent` meets *forbid*.** S1f.6.20 will have met it first; this stage follows its
  outcome.
- **That a derived position is possible in some cases** (a midpoint is between its ends; the foot of an
  altitude in an acute triangle is between the vertices). Each such rule is a theorem with its own conditions,
  written where the textbook proves it and not before.
