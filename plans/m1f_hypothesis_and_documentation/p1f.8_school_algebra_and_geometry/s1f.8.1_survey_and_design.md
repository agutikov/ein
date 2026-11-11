# S1f.8.1 — The survey and the design: both halves on a prototype, every answer computed

**Phase:** [P1f.8](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day. Not started. **It is the phase's stop:**
nothing outside `plans/` and `build/` is written before the user has read its page.
**Depends on:** [S1f.6.6](../p1f.6_calculator/s1f.6.6_division.md), the four operations. The programs that need
a sign or a fraction wait for [S1f.6.7](../p1f.6_calculator/s1f.6.7_negative_numbers.md) and
[S1f.6.8](../p1f.6_calculator/s1f.6.8_rational_numbers.md); the page marks each such program, and may be shown
in two rounds, the natural-number programs first, as the calculator's design was shown in three.

## The instruction

The user, 2026-11-11:

> I forgot another idea - between calculator and proof systems - middle school algebra and geometry: variables,
> expressions simplifications, solving equasions; middle school geometry contains proofs, but I'm not sure it is
> 100% complete formal proof system (it starts in 5th grade) - so just separate phase in M1f

And the user's standing rule for a new subject (2026-10-16): real programs first, their answers computed, before
anything is implemented.

## What it is

A page, `p1f.8_school_algebra_and_geometry/design.md`, as the calculator's
[`design.md`](../p1f.6_calculator/design.md) is: target programs written for a prototype under `build/s1f.8/`,
each run on the release binary, each shown with every artefact a run writes (`ein solve`, `ein saturate --dump`,
`ein test`, `--trace`, `--dump-answer`), and after them the conventions proposed and the questions, each with a
recommendation that a run supports. The commit at the stop is docs only.

It re-takes every probe of [README § First probes](README.md#first-probes-2026-11-11) as a kept program. Those
were one run each; here each is a file under `build/s1f.8/` with the command that runs it.

## The target programs

Each is a real school problem, with the sentence beside the facts. The table is the least the page shows; a
problem that turns out not to be writable stays on the page with what happened.

| # | the problem | what it shows | probe |
|---|---|---|---|
| 1 | *Find the value of 3x + 4 when x = 5.* | an expression as data; substitution; the calculator underneath | A-7 |
| 2 | *Simplify 2x + 3x.* then *3(x + 2) + 4x.* | like terms; a bracket expanded; the normal form `7x + 6` | A-6 |
| 3 | *Are 2(x + 3) and 2x + 6 the same?* | equality as one normal form | — |
| 4 | *Solve 3x + 4 = 19.* | the derivation, three steps, and its trace beside a textbook's worked solution | A-1 |
| 5 | *Solve 3x + 4 = 18*, *0·x + 4 = 19*, *0·x + 4 = 4*, *x + 5 = 2.* | no whole root; no root; every number; a negative root | A-2, A-3 |
| 6 | Problem 4 with x known to be a digit, solved by search | the same answer from the lattice; the two costs side by side | A-4 |
| 7 | *Bob has some apples. He gave 2 away and has 3. How many had he?* | [S1f.6.20](../p1f.6_calculator/s1f.6.20_word_problems.md)'s problem run backwards: the unknown is a start, not a result | — |
| 8 | *x + y = 10, x − y = 2.* | two equations, by elimination | — |
| 9 | *Prove (a + b)² = a² + 2ab + b².* | an identity by normal forms, two variables | — |
| 10 | *In a triangle ABC, AB = AC. Prove that the angles at B and C are equal.* | a proof by saturation; the trace as a two-column proof | G-1 |
| 11 | The same, by the textbook's proof: draw the median | a construction, and where the theorem then lives | G-2 |
| 12 | *Two lines perpendicular to a third are parallel.* | a proof by contradiction, as far as it goes | G-3 |
| 13 | *Two angles of a triangle are 50° and 60°. Find the third.* | a computation in a figure | G-4 |
| 14 | *D is on BC. The angle BAD is 30° and DAC is 40°. Find BAC.* with *D is on BC* stated, and then left out | the diagram gap: the sum is right only if D is between, and the program says what it was not told | — |

## What the page proposes, each on the programs above

1. **Expressions:** the shape of the data, the spelling of its heads, and how it stands to
   [S1f.6.14](../p1f.6_calculator/s1f.6.14_expressions.md)
   ([README § Expressions, and S1f.6.14](README.md#expressions-and-s1f614)). Shown: problem 1 written three
   ways, as expression data with an evaluator; as request facts chained by rules of the program, the
   calculator's core way; and, for comparison only, as round 2's `(calc E)`, from the calculator's prototype.
2. **The names.** Which names the theories occupy, beside the calculator's long list; that `unknown` is taken
   (A-5); the spelling shared with M1g.
3. **How a program states its lattice.** Every program of the phase either writes
   `(solve :hypothesis-relations ())` or closes its relations, since without one A-1 is refused (A-8) and G-1 is
   a search over 24 candidates. The page shows both remedies on one program and what each prints, and files F-1.
4. **The trace as the school solution.** Problem 4's `--trace` beside the textbook's three lines (F-2), with what
   could be done: a marker on the problem theory's rules, a filter by module, a second rendering. Proposed, not
   built.
5. **The geometry's vocabulary,** in outline; [S1f.8.6](s1f.8.6_what_school_geometry_is.md) settles it.
6. **Where the theories live** and what they are called.
7. **What is measured for each theory:** firings and depth for one problem, and the matcher's candidates for a
   batch, as the calculator's stages record them.

## Sketch

Problem 4 as probed (A-1). **Sketch only, to be replaced by programs whose answers are computed** by this stage;
the planning agent ran it once on 2026-11-11 and the binary answered `(value x (n z 5))`, no candidate, k_s 1.

```lisp
(import std.arith :symbols (arith))

;; "Solve 3x + 4 = 19."  a·x + b = c, written with the coefficient, the constant and the right side.
(relation linear Unknown Number Number Number
  :why "{?2} times {?1}, plus {?3}, equals {?4}")
(relation moved Unknown Number Number
  :why "{?2} times {?1} equals {?3}")
(relation value Unknown Number
  :why "{?1} is {?2}")

(linear x (n z 3) (n z 4) (n (n z 1) 9))

(rule take-constant ()
  :match  (linear ?x ?a ?b ?c)
  :assert (sub ?c ?b)
  :why    "take {?b} from both sides")
(rule constant-taken ()
  :match  (and (linear ?x ?a ?b ?c) (diff-of ?c ?b ?d z))
  :assert (and (moved ?x ?a ?d) (div ?d ?a))
  :why    "{?a} times {?x} is {?c} less {?b}, which is {?d}; divide both sides by {?a}")
(rule divided ()
  :match  (and (moved ?x ?a ?d) (quot-of ?d ?a ?q z))
  :assert (value ?x ?q)
  :why    "{?x} is {?d} divided by {?a}, which is {?q}")

(solve :hypothesis-relations ())
(query :goal (value x ?v) :goal-text "x = {?v}"
       :expect (model (value x (n z 5))))
```

It is deliberately the *wrong* design in one respect, which the stage shows: `(linear x 3 4 19)` is an equation
already in a fixed form, so nothing was simplified and the solver knows one shape. Problem 4 on the page starts
from `(equation (Plus (Times (num 3) (var x)) (num 4)) (num 19))`, and S1f.8.3's normal form is what brings it
to this shape.

## Tasks

1. **T1, the prototype and the page.** The fourteen programs, the conventions, the measurements, the findings
   re-taken (F-1 to F-6), and the questions a to d, f and g of
   [README § The questions for the user](README.md#the-questions-for-the-user) with their recommendations.
2. **T2, the stop.** The user reads the page. The rulings are recorded on it in the user's words, as
   [`design.md` § The rulings](../p1f.6_calculator/design.md#the-rulings-2026-11-11) records the calculator's.
3. **T3, the re-plan.** The phase's README and stage files are restated to the rulings, and the questions are
   filed in [`open_questions.md`](../open_questions.md) with ids then free.

## Acceptance

- The page exists, every program on it was run by the binary, and every number on it carries its command and
  date.
- Every probe of the README's table is re-taken, and each row says *confirmed* or how it differed.
- Each of F-1 to F-6 is confirmed as a finding with a kept witness, or struck.
- The user has ruled, or has said which questions wait.
- Nothing outside `plans/` and `build/` moved.

## What it may find

- **That expression data is not accepted.** Then the algebra half is replanned around fixed equation shapes
  (`linear`, `linear2`, `quadratic`) with no simplification, and the page says what is lost.
- **That a prototype needs the extensions before anything interesting can be shown.** Problems 5 (the last),
  8 and 9 need signs; the page then goes to the user in two rounds.
