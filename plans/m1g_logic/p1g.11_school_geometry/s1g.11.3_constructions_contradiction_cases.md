# S1g.11.3 — Auxiliary constructions, proof by contradiction and case splits: three uses of a hypothesis, and what each cannot do today

**Phase:** [P1g.11](README.md) of [M1g](../README.md). Until 2026-11-13 this stage was **S1f.8.8** of M1f's P1f.8
([README § The move](README.md#the-move-of-2026-11-13)); nothing of its content was redesigned.
**Status:** planned 2026-11-11. Not started. **Stops for the user:** how a construction is written, and whether
a refuted assumption concludes anything.
**Depends on:** [S1g.11.2](s1g.11.2_theorems_as_rules.md): the rule base, and the theorems it could not prove,
which are this stage's material.

## The instruction's part

> middle school geometry contains proofs, but I'm not sure it is 100% complete formal proof system

Three moves of a school proof are not *apply a theorem to what is there*. Each adds something no rule derived:
a point, an assumption, a case. Ein has one mechanism for adding what no rule derived, the hypothesis, and the
stage finds out how well it fits each. The planning agent's probes say: partly, with a different limit each
time.

## 1. Auxiliary constructions

*Draw the median. Extend the side. Through C draw a line parallel to AB.* Many school proofs start so, and
finding the construction is the hard part of the proof.

**The constraint.** Ein never makes a name at run time: no rule asserts a name the program lacks (the M1
alive-set invariant), and the universe H is built from the program's names
([`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md), §§ 1 and 2). A constructed
point has no name. Three ways to have one anyway, each tried:

| | how | what it is | expected limit |
|---|---|---|---|
| **(a) a value** | the point is a nested fact over the points it is made from: `(mid B C)`, `(foot A (line B C))`, `(meet (line A C) (line B D))` | a Skolem-style term: a new value, no new name, legal today (M1g's probe 5) | **a rule that constructs unasked floods:** the midpoint of every segment, then segments to those midpoints, then their midpoints. Constructions must be on request, `(construct (mid B C))`, and then something has to ask. And one point gets two values: the centroid is the meeting of any two medians, three spellings, the bookkeeping of S1g.11.1 once more |
| **(b) a hypothesis** | an hrule proposes the construction; the lattice explores which constructions to add | *which construction* is the search, as a model proposes them for AlphaGeometry's engine ([S1g.11.1](s1g.11.1_what_school_geometry_is.md), to be checked) | **the theorem is in a state, not at root** (probe G-2, [`findings.md`](findings.md) F-4): R has the state with the construction and the state without, and a query over all of R reports the theorem as one class of two. And the candidates multiply: n constructions are 2ⁿ states, none dead, since a construction contradicts nothing |
| **(c) a fact** | the problem writes the construction: *let M be the midpoint of BC* | the proof is then checked, not found: the pupil's first line is given | none in the engine; it concedes the hard part |

**The conceptual point, to be stated on the page.** A hypothesis in Ein is a guess **about the world**: it may be
false, and a state is a world in which it holds. A construction is not a guess. The midpoint of BC exists
whether or not it is drawn, and what follows from it is true of the figure as given. So (b) uses the lattice
for something it does not mean, and the read-out says so honestly by reporting two classes. Three readings of
G-2's answer are compared:

- **read the maximal line.** Where every candidate is a construction and none restricts, the maximal state is
  the figure with everything drawn, and what holds there holds. `:over maximal` reads the theorem (G-2). It is
  sound only while no hypothesis in the program is a real guess, and nothing checks that;
- **re-run with the construction as a fact** (c), found by (b): search proposes, a second program certifies.
  Two runs, each honest;
- **(a) on request, with the request a hypothesis:** the candidate is `(construct (mid B C))`, and the facts
  about the point are derived in its state. The same shape as (b), with a cleaner statement.

**What the phase would need and does not build:** a way to say *this may be added and changes nothing that
was true*: a conservative extension. In M1g's terms it is the witness of an existential that is known to hold
(*every segment has a midpoint*), which is candidate C-8 of
[P1g.6](../p1g.6_engine_requirements/README.md),
[S1g.6.4](../p1g.6_engine_requirements/s1g.6.4_witnesses_and_fresh_names.md), with
[Q-M1g.8](../open_questions.md#q-m1g8--fresh-names) and
[S1g.4.4](../p1g.4_first_order_logic/s1g.4.4_quantifier_rules.md)'s witnesses. This stage supplies
that candidate with its first non-logical evidence and stops there.

**Which constructions to propose.** A bounded menu per figure: the midpoint of a side, the foot of an altitude,
a parallel through a vertex, a side extended by its own length. The stage measures how many states that is on
the five targets, and where the region limit ends it. It does not write a strategy.

## 2. Proof by contradiction

*Two lines perpendicular to a third are parallel. Suppose they meet at P. Then from P there are two
perpendiculars to the third line, which cannot be. So they do not meet.*

**The mapping:** the assumption is a candidate; the contradiction makes its commitment dead; the core says
which facts refuted it, and `refuted by` names the rule. Probe G-3: two candidates (the supposition in its two
spellings), **both dead alone**, N of two minimal dead sets, R {∅}.

**The limit:** root does not hold the conclusion. *They do not meet* is not a fact; it is the read-out's
statement that a candidate is dead. A rule cannot read it, so nothing can be built on it: *parallel, hence the
alternate angles are equal* does not follow in the program. This is M1g's probe 4 and
[Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact), found again on a school
theorem, and it is listed as F-5.

What this stage does with it, short of M1g:

- shows the dead commitment, its core and its `refuted by` as **the proof by contradiction, written by the
  engine**: the core is the list of facts the contradiction used, which is what a pupil's proof cites;
- shows the **two-run** form: the first program refutes the assumption; the second states the negation as a
  fact with a `:source` naming the refutation, and goes on. Honest, and manual;
- shows the alternative that needs no hypothesis: **the contrapositive as a rule.** *Two perpendiculars to one
  line are parallel* is itself a theorem of the rule base, and a school proof by contradiction is usually how
  the *rule* was justified, not how it is used. So the stage separates *using* a theorem proved by
  contradiction (a forward rule, no hypothesis) from *proving* it (the lattice), and says that the second is
  where Ein's limit is;
- states what [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md), natural deduction's
  negation introduction, will formalise, and asks the user Q-M1g.5 here only if this stage runs first.

A caution the page carries: under S1e2.4.1's *forbid*, a refutation may not rest on an `absent` over a relation
a commitment can still extend. A geometry rule base written monotone, as the calculator is, never meets it; a
*not parallel* written as `(absent (parallel …))` would.

## 3. Case splits

*The foot of the altitude lies between B and C, or at B, or outside. Consider each.* Or: *the angle is acute,
right or obtuse.*

**The mapping:** each case is a candidate; an obligation says one of them holds, a restriction that no two do;
R is then the cases that survive, and a conclusion that holds in **every** state of R holds. A query's
projection classes show it: one class means every case agrees.

**The limit is the same as the construction's, and milder:** the conclusion is per state, and *holds in every
state* is read off the read-out, not derived. The stage shows a school theorem proved by cases (to be chosen
from the textbook: the inscribed angle theorem is the classic and needs circles, which the phase does not
have; a triangle's altitude and the three positions of its foot is the plan) with one projection class, and a
false *theorem* with two.

**And cases are where the diagram gap comes from.** A school proof draws one picture and so proves one case.
The well-known false proof that every triangle is isosceles is a correct derivation from a wrongly drawn
figure: the picture puts a point inside the triangle that lies outside. Written as an Ein program **with the
position as a case**, the fallacy should show as a dead case or as two classes. This is the stage's show-piece
if it works, and it is **to be tried**: the planning agent has not run it.

## Sketch

**Sketches only, to be replaced by programs whose answers are computed.** Both were run once by the planning
agent on 2026-11-11.

Probe G-2, the construction as a hypothesis, over S1g.11.1's vocabulary plus `mid`, `drawn` and `on`. The binary
answered: 1 candidate, 0 dead, k_s 2; the query over all of R has **2 projection classes**, state 1 *the goal has
no row: undetermined here* and state 2 `?other = (ang A C B)`; over the maximal line, 1 class, the theorem.

```lisp
(relation mid Point Point   :why "the midpoint of {?1}{?2}")
(relation drawn Point       :why "{?1} is drawn")
(relation on Point Point Point :why "{?1} lies on {?2}{?3}, between them")

(triangle A B C)
(seg-eq (seg A B) (seg A C))

(hrule draw-midpoint-of-base ()
  :match  (triangle ?a ?b ?c)
  :assert (drawn (mid ?b ?c))
  :why    "draw the midpoint of {?b}{?c}")
(rule midpoint-halves ()
  :match  (drawn (mid ?b ?c))
  :assert (and (seg-eq (seg ?b (mid ?b ?c)) (seg ?c (mid ?b ?c)))
               (on (mid ?b ?c) ?b ?c))
  :why    "a midpoint cuts its segment into two equal parts")
(rule sss ()
  :match  (and (triangle ?a ?b ?c) (on ?m ?b ?c)
               (seg-eq (seg ?a ?b) (seg ?a ?c))
               (seg-eq (seg ?b ?m) (seg ?c ?m)))
  :assert (cong (tri ?a ?b ?m) (tri ?a ?c ?m))
  :why    "three sides: {?a}{?b} = {?a}{?c}, {?b}{?m} = {?c}{?m}, and {?a}{?m} is shared")
(rule base-angles ()
  :match  (and (cong (tri ?a ?b ?m) (tri ?a ?c ?m)) (on ?m ?b ?c))
  :assert (ang-eq (ang ?a ?b ?c) (ang ?a ?c ?b))
  :why    "the angles at {?b} and {?c} correspond, and {?m} lies on {?b}{?c}, so they are the triangle's own")

(solve :hrules (draw-midpoint-of-base))
(query :goal (ang-eq (ang A B C) ?other))
(query :goal (ang-eq (ang A B C) ?other) :over maximal)
```

The sketch cheats, and the stage must not: `sss` is written for this figure (the shared side is not a premise),
and `base-angles` folds in the step *the angle ABM is the angle ABC because M is on BC*, which is a fact about
the picture. Written with S1g.11.2's general rules, the same proof needs that step as a rule with its
betweenness premise.

Probe G-3, the assumption refuted. The binary answered: 2 candidates, 2 dead alone, N 2, R {∅}; the query
`(parallel ?x ?y)` has no row.

```lisp
(relation perp Line Line     :why "{?1} is perpendicular to {?2}")
(relation meet Line Line     :why "{?1} and {?2} meet")
(relation parallel Line Line :why "{?1} and {?2} are parallel")

(perp a c)
(perp b c)

(hrule suppose-they-meet ()
  :match  (and (perp ?a ?c) (perp ?b ?c) (neq ?a ?b))
  :assert (meet ?a ?b)
  :why    "suppose {?a} and {?b} meet")
(rule two-perpendiculars-from-a-point ()
  :match  (and (meet ?a ?b) (perp ?a ?c) (perp ?b ?c) (neq ?a ?b))
  :assert (false)
  :why    "{?a} and {?b} would be two perpendiculars to {?c} from one point, and there is only one")

(solve :hrules (suppose-they-meet))
(query :goal (parallel ?x ?y))
```

## Tasks

1. **Constructions,** the three ways on S1g.11.2's targets 1 and 5 and on the theorem that stage could not
   prove; the menu, and the states it costs; the three readings of the answer compared.
2. **Contradiction:** one school theorem refuted as a commitment, with its core and `refuted by` read as the
   proof; the two-run form; the contrapositive as a rule.
3. **Cases:** one theorem by cases with one projection class; the false proof that every triangle is isosceles,
   tried.
4. **The stop:** the page; question h; question i if M1g has not asked it; the candidates named for P1g.6 with
   this stage's programs as evidence, written into that phase's seed table only on the coordinator's word
   (since 2026-11-13 [S1g.6.7](../p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md) lists the
   geometry's candidates in one place, T-1, T-6 to T-8 and T-10 this stage's).
5. **What ships,** as ruled: the programs as corpus entries with an `:expect` each, a claim about the region
   where the result is a state's and not root's (`(solve … :expect (region …))`).
6. **The tests:** each program's R against `ein oracle`; the dead commitments' cores named; the timeout.

## Acceptance

- Each of the three moves has at least one school proof as a program, with the read-out shown and the limit
  stated in a sentence that a program supports.
- For constructions: the three ways are on the page with what each cost, and the page says which the phase
  uses and why the theorem is, or is not, root's.
- For contradiction: the core of the refuted assumption is shown beside the textbook proof's citations.
- For cases: one true theorem with one class, and one false claim with more than one or with a dead case.
- **Nothing is built in the engine.** Each thing the stage wanted and lacked is named with M1g's candidate, or
  is a row of [`findings.md`](findings.md).
- The user has ruled on how a construction is written.

## What it may find

- **That (a) and (b) together are enough** for the school proofs tried, with `:over maximal` as the reading.
  Then the page says exactly when that reading is sound, and proposes the check nothing does today: that no
  candidate of the program restricts.
- **That an hrule cannot be told to propose a nested construction for only the segments a request names.** The
  parse error of M1g's probe 4b (an activator's argument in `:hrules` is a name, never a nested fact) is the
  neighbour; to be tried.
- **That the false isosceles proof does not show its flaw,** because the flaw is in a betweenness fact the
  program was simply told. Then the result is the plain one: Ein checks the derivation and not the figure, and
  the figure is the problem's to state. That is [S1g.11.4](s1g.11.4_computation_and_the_diagram_gap.md)'s subject.
