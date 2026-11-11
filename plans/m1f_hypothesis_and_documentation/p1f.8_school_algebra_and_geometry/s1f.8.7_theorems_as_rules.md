# S1f.8.7 — Theorems as rules, proofs as saturation: a rule base, five school theorems, and the trace as a two-column proof

**Phase:** [P1f.8](README.md)
**Status:** planned 2026-11-11. Not started.
**Depends on:** [S1f.8.6](s1f.8.6_what_school_geometry_is.md): the vocabulary, the bookkeeping scheme, the two
levels, as ruled.

## The instruction's part

> middle school geometry contains proofs

## What it is

A school course proves a theorem once and then **uses it as a step**. That is what an Ein rule is: premises, a
conclusion, a `:why`. So the rule base is the course's list of theorems and axioms, each a rule; the figure of a
problem is facts; and a proof is what saturation derives. The trace of the conclusion, back to the givens, is a
**two-column proof**: statement and reason, each line a fact and the rule that derived it.

This is the deductive-database method as [S1f.8.6](s1f.8.6_what_school_geometry_is.md) reports it from the
literature (to be checked there). Ein's version differs in three ways the page states: the rule base is a
program a reader can open, each rule with its sentence; the proof is not searched for, it is whatever the
fixpoint holds; and what is *not* derived is as much a result as what is.

### The rule base

As planned; the list is settled after S1f.8.6's three proofs have been taken apart:

| family | rules |
|---|---|
| congruence criteria | side-angle-side, angle-side-angle, side-side-side; for right triangles, hypotenuse and a leg |
| what congruence gives | corresponding sides are equal; corresponding angles are equal |
| isosceles triangles | equal sides give equal base angles, and the converse |
| angles at a point | vertical angles are equal; adjacent angles on a line are supplementary |
| parallels | a transversal's alternate angles are equal, corresponding angles are equal, and the converses; two lines parallel to a third |
| triangles | the angle sum; the exterior angle equals the two remote interior angles |
| quadrilaterals | a parallelogram's definition, its opposite sides and angles, its diagonals |
| midpoints | a midpoint halves; the midline of a triangle |

**A theorem that the base both contains and can prove.** *Base angles of an isosceles triangle* is a rule a
later proof uses, and it is also a target. The stage keeps the two apart by layers: a target is proved by a
program that imports **only the rules that come before it in the course**, so that nothing is proved from
itself. The order of the layers is the textbook's, and the page shows it as a dependency graph of theorems,
which is the school course's own structure made checkable.

**Side conditions are premises.** Every rule that is true only for some figures says so: `(triangle A B C)` for
non-collinear, `(between …)` where a sum of angles or segments is taken, `(neq ?l ?m)` where two lines must
differ. A rule without its condition is unsound, and S1f.8.9's negative figures are the test.

### The targets

Five school theorems. For each the page gives: the statement as the textbook has it; the figure's facts; the
rules available; what the program derived; the two-column proof read off the trace; **what had to be written
that a pupil reads off the picture**; and what it could not do.

| # | the theorem | the school proof | expected, and what is to be tried |
|---|---|---|---|
| 1 | The base angles of an isosceles triangle are equal | by the median or the bisector, then a congruence; or Pappus's, the triangle congruent to its own mirror reading | **Pappus's is found by saturation with no construction** (probe G-1: SAS and corresponding angles, 10 firings, no candidate). The textbook's own proof needs a constructed point: [S1f.8.8](s1f.8.8_constructions_contradiction_cases.md) |
| 2 | Vertical angles are equal | each is supplementary to the same angle | needs *supplementary* as a relation and *two angles supplementary to one are equal*: the first use of the synthetic angle sum, or of measures with an unknown. To be tried at both levels |
| 3 | The diagonals of a parallelogram bisect each other | alternate angles at the parallels, then angle-side-angle on the two triangles at the crossing | the figure must state that the diagonals **meet**, at a point O **between** each pair of opposite vertices. A pupil sees it. This is the diagram gap inside a proof |
| 4 | An exterior angle of a triangle equals the sum of the two remote interior angles | the angle sum, and a straight angle | a sum, so the metric level or `ang-sum`; with letters for the angles it is S1f.8.4's algebra over S1f.8.3's normal forms: `a + b + c = 180` and `c + d = 180` give `d = a + b`. The first place the two halves of the phase meet |
| 5 | The midline of a triangle is parallel to the third side and half of it | extend the midline to twice its length, a parallelogram appears | needs a construction, and *half* is metric or a ratio. Expected to be the hardest; it may be the theorem the base cannot prove without S1f.8.8, and then it is the stage's named failure |

And **at least one theorem the base cannot prove,** found and not chosen: something from the same textbook
chapters that the saturation does not reach, with the reason (a missing rule, a construction, a fact of the
figure, a case split).

### The two-column proof

`--trace` gives the derivation of everything. A two-column proof is the derivation of **one** fact: its
justification, its premises' justifications, back to the givens, in order, each fact once. The stage establishes
what the binary offers for that today (the justification of a fact in a `--dump-answer` state file, the trace's
per-step premises, `ein render` of a derivation) and writes the proof from it by a script under `utils/` or
`build/` if nothing prints it directly. Whether a *proof of this fact* read-out is wanted in `ein` itself is put
to the user with [`findings.md`](findings.md)'s F-2, which is the same wish from the algebra side.

**A proof is an explanation, and may not be the shortest.** The first justification the store recorded is the
one printed (the user's ruling of 2026-11-08: a core is an explanation and may vary with rule order). Two rule
orders may print two proofs of one theorem. The page shows one, and says so.

## Sketch

**Sketch only, to be replaced by programs whose answers are computed.** Target 1 by Pappus's proof, over the
vocabulary and the three bookkeeping rules of [S1f.8.6](s1f.8.6_what_school_geometry_is.md#sketch). The planning
agent ran the whole once on 2026-11-11 (probe G-1): the binary answered `?other = (ang A C B)`, the claim held,
no candidate, k_s 1, 1 saturation; `--trace` wrote ten steps, three bookkeeping rules twice each, `sas` twice
and `corresponding-angles` twice (each for the triangle and for its mirror reading).

```lisp
(rule sas ()
  :match  (and (triangle ?a ?b ?c) (triangle ?d ?e ?f)
               (seg-eq (seg ?a ?b) (seg ?d ?e))
               (ang-eq (ang ?b ?a ?c) (ang ?e ?d ?f))
               (seg-eq (seg ?a ?c) (seg ?d ?f)))
  :assert (cong (tri ?a ?b ?c) (tri ?d ?e ?f))
  :why    "two sides and the angle between them: {?a}{?b} = {?d}{?e}, the angle at {?a} equals the angle at {?d}, {?a}{?c} = {?d}{?f}")
(rule corresponding-angles ()
  :match  (cong (tri ?a ?b ?c) (tri ?d ?e ?f))
  :assert (and (ang-eq (ang ?a ?b ?c) (ang ?d ?e ?f))
               (ang-eq (ang ?a ?c ?b) (ang ?d ?f ?e)))
  :why    "congruent triangles have equal angles at corresponding vertices")

(solve :hypothesis-relations ())
(query :goal (ang-eq (ang A B C) ?other)
       :goal-text "the angle at B equals {?other}"
       :expect (model (ang-eq (ang A B C) (ang A C B))))
```

What the run did not show, each to be tried: that `sas` stays cheap when the figure has more than three points
(it joins two `triangle` facts and three facts on nested values, five premises, on a state where every triangle
has several readings); and what the same program is without `(solve :hypothesis-relations ())`, which the
probe did show: 24 candidates and a search of 118 847 saturations for a lower bound of 72 012 states, where the
proof is one.

## Tasks

1. **The rule base,** in layers in the textbook's order, each rule with its `:why` as the theorem's sentence, and
   the textbook's statement and its place in a comment beside it (a rule takes no `:source`; only a fact does);
   each rule's side conditions written.
2. **The five targets,** each a program importing only earlier layers, each with an `:expect`.
3. **The theorem it cannot prove,** found by trying the chapter's theorems in order.
4. **The two-column proof** of each target, read off the run, with the script or the read-out that makes it.
5. **The tests:** each target's claim; the search-free test; **a negative figure for each rule,** one that
   satisfies every premise but a side condition, on which the conclusion must not be derived; each rule fired by
   some program (the stdlib's coverage rule, applied here wherever the base lives); the timeout; the state's
   size and the matcher's candidates for each target.
6. **The mutation idea, applied to the rule base:** a premise dropped from a rule should make some negative
   figure fail. `utils/stdlib_mutants.py` does this for `stdlib/`; if the base lives elsewhere the stage says
   how the same sweep is pointed at it.

## Acceptance

- Each target either has a program whose claim holds, with no candidate and one saturation, or is the named
  failure with its reason.
- No target is proved from a layer that contains it.
- Each proved target has a two-column proof read off the run, every reason a rule's `:why`, and a test holds its
  lines for at least one.
- Every rule has a negative figure on which it does not fire wrongly.
- The page lists, per target, **the facts that were written and that a pupil would have read off the picture**.

## What it may find

- **That the state grows faster than the figure.** Every spelling of every equality between every pair. The
  measure is taken per target, and if the parallelogram is already slow the bookkeeping scheme goes back to
  S1f.8.6's table.
- **That five premises joined on nested values are slow in a batch** (E-3). One figure is small; the measure is
  reported, and the index is M1g's S1g.6.2.
- **That saturation proves too much to read.** A fixpoint over a figure holds hundreds of true facts nobody
  asked for. That is why the proof is a slice of the trace and not the trace.
- **Engine findings,** to [`findings.md`](findings.md).
