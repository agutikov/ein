# S1g.4.4 — The quantifier rules: instantiation, generalisation, witnesses and Skolem terms, in a checked proof

**Phase:** [P1g.4](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.4.3](s1g.4.3_formulas_with_quantifiers.md)'s substitution;
[S1g.3.1](../p1g.3_proof_systems/s1g.3.1_hilbert_systems.md)'s and
[S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md)'s proof formats and checkers.
**For the user:** [Q-M1g.8](../open_questions.md#q-m1g8--fresh-names), fresh names, put with what this
stage found Skolem terms can and cannot do.

## What it asks

What it takes to **check a first-order proof** in Ein, and how far Ein can **make** the steps a quantifier
needs: instantiate a universal, supply a witness for an existential. The four quantifier rules of natural
deduction, and their Hilbert-style counterparts (two axiom schemata and generalisation), each have a side
condition, and the side conditions are the stage.

## The four rules, and what each needs

| rule | says | needs | expected |
|---|---|---|---|
| ∀ elimination | from `(All x F)`, `F[t/x]` for any term t | substitution; *t free for x in F*; and, forward, a choice of t | checking: possible. Applying it forward: t is unbound, the schema's wall again ([§ 4.13](../../../docs/kernel/defined_behaviour.md)); on request, or over the terms that occur in the proof so far |
| ∃ introduction | from `F[t/x]`, `(Ex x F)` | substitution, read backward: the line names t | checking: possible |
| ∀ introduction | from F proved for an *arbitrary* x, `(All x F)` | **freshness:** x not free in any open assumption | checking: a walk over the context with `free-in`; the negative, *not free in*, has to be derived positively (a `not-free-in` relation by recursion on the formula), since the theory writes no `absent` |
| ∃ elimination | from `(Ex x F)` and a derivation of G from `F[c/x]` with c **new**, G | freshness of c: not in G, not in the context; and a discharge, as `Or` elimination has | checking: as above, plus S1g.3.3's answer to discharge |

So a **checker** needs: substitution, `not-free-in` as a positive relation, and contexts as data. All three
are theories over nested facts, written demand-driven. Expected nearly possible today. The stage builds the
checker and the page reports which side condition was hardest.

## Witnesses, three ways

Where a *finder* or Ein's own rules need a witness, the options are the ones
[S1g.4.1](s1g.4.1_the_fragment.md)'s table names, here compared on one example each, computed:

**1. A Skolem term.** A rule that concludes an existential builds its witness from the rule's variables: a
nested fact. The sketch is probe 5's; on the release binary at `8860d616` (2026-11-11) it answered
`(parent (father-of Ann) Ann)`.

```lisp
;; sketch: every person has a father, the father a Skolem term
(relation father-of Person     :why "the father of {?1}")
(relation person Person        :why "{?1} is a person")
(relation parent Person Person :why "{?1} is a parent of {?2}")
(person Ann)
(rule has-father ()
  :match  (person ?x)
  :assert (parent (father-of ?x) ?x)
  :why    "{?x} has a father")
(solve :hypothesis-relations ())
(query :goal (parent ?f ?c) :expect (model (parent (father-of Ann) Ann)))
```

What it gives and what it does not, to be shown: it is sound for refutation and for deriving consequences
(Skolemisation preserves satisfiability); the witness is a new *value* and not a new *name*, so the
alive-set invariant holds and the blind enumerator does not propose facts about it (to be checked: what H
contains when a nested fact appears in a derived fact). And it is where saturation stops stopping: add *a
father is a person* and the terms grow without bound, `(father-of (father-of Ann))` and on, exactly probe
3. With the engine's word for that ([S1g.6.1](../p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md))
the program has an answer; without it, a timeout.

**2. An obligation.** `(open ?R)`: the state owes a witness, and the obligations rung proposes the existing
names as candidates. This is the existential of a *finite model*: the witness is one of the things the
program already has. Ann's father is then Ann, Bob or nobody the program names. The stage shows the same
sentence under 1 and under 2 and says what each R means: under 1, one state with an anonymous father; under
2, a state per candidate father, and `Open` if none can be. F26 asks whether this is a chase step; the two
readings side by side are the material [S1g.4.5](s1g.4.5_the_boundary.md) answers it from.

**3. A parameter supplied by the proof.** In a written proof the fresh c of ∃ elimination is a name the
proof's author chose, and the checker verifies it is fresh. No name is made. This is enough for checking,
and it is why checking is *nearly possible* while finding is *required*: only a finder has to invent c.

## Herbrand's theorem, as the bridge back to P1g.2

A universal sentence is unsatisfiable exactly when some finite set of its ground instances is
propositionally unsatisfiable. In Ein's terms, to be tried on two or three small sentences: Skolemise by
hand, let request-driven rules produce ground instances over the terms that occur, and hand the instances to
P1g.2's machinery, or let the direct program's `(false)` do it. This is what a forward rule engine does
anyway: Ein instantiates rules over the values in the state. The page states that correspondence, since it
is the most accurate one-sentence answer to *what first-order logic Ein already does*: **Ein grounds a
theory over the terms a state holds, and decides the propositional remainder by saturation and search.**
That sentence is the coordinator's and is the stage's to confirm or correct.

## Tasks

1. **T1g.4.4.1 — `not-free-in` and the context walks,** on request, search-free, held to a Rust
   implementation on generated cases.
2. **T1g.4.4.2 — the first-order proof checker,** natural deduction's four rules added to S1g.3.3's (or the
   Hilbert schemata and generalisation added to S1g.3.1's, if the user chose that system as the default); a
   program per rule; three or four textbook proofs banked (the exchange of two universals; `(Ex x (All y …))`
   implies `(All y (Ex x …))`; a syllogism with a quantifier).
3. **T1g.4.4.3 — rejection,** above all of the **classic wrong proofs**: generalising over a variable free
   in an assumption; using an eigenvariable that is not fresh; the converse of the quantifier exchange. Each
   banked as a program whose claim is that the last line is *not* checked.
4. **T1g.4.4.4 — the three witnesses** on one sentence, each computed, with what H and R are.
5. **T1g.4.4.5 — Herbrand,** two or three sentences, and the sentence above confirmed or corrected.
6. **T1g.4.4.6 — Q-M1g.8, for the user:** what Skolem terms sufficed for, and the first thing they did not.

## Acceptance

- Every banked correct proof is checked by saturation alone; every banked wrong proof is rejected, and each
  rejection is for the side condition it violates (the test names the line).
- The Rust check: every checked line of every generated proof is valid in every structure up to the size
  S1g.4.3's model checker reaches (soundness, sampled).
- The three witnesses each have a program and a paragraph saying what R means.
- The page states what is *possible today*, *nearly possible* and *required* for first-order proof, each
  with its program, and Q-M1g.8 is put to the user.

## What it may find

- **That H holds candidates about Skolem terms,** or none, in a way that changes R: read against
  [`solution_semantics.md` § 1.4](../../../docs/kernel/inference/solution_semantics.md), and a register row
  if it is wrong.
- **That equality is needed at once.** *Every person has exactly one father* needs the witness to be equal
  to something, and Ein's `eq` is identity of values: `(father-of Ann)` is never equal to `Bob`. A theory of
  equality (congruence, as [idea 06](../../ideas/06-inference-rules-completeness.md)'s second row names it)
  written as rules over an `equal` relation is the detour; it is tried on that one sentence and its cost
  stated.
