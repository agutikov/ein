# S1g.3.3 — Natural deduction: introduction and elimination for each connective, assumptions, discharge, and reductio against Ein's dead state

**Phase:** [P1g.3](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.3.1](s1g.3.1_hilbert_systems.md), for the proof format and the checker's shape.
**Pulls in:** [idea 06](../../ideas/06-inference-rules-completeness.md)'s row *hypothesis + contradiction:
`assume A=x → derive ⊥ ⇒ A≠x`, natural deduction, ATMS, CDCL*;
[F26](../../followups/f26_consistency_and_completeness.md)'s first row (what a dead state's content is).
**Stops for the user:** its first task, the two readings below on real programs, with
[Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact).

## What it asks

> natural deduction (introduction and elimination of ...) […] what are hypothesis in this case?

Natural deduction has, for each connective, a rule that introduces it and one that eliminates it, and
**assumptions**: formulas supposed for the sake of argument and later discharged. Ein has hypotheses:
candidate facts committed in some states of a lattice. The stage tests how far the second is the first.

## The rules, sorted by what they need

The coordinator's sorting, each row **to be tried**:

| rule | shape | in Ein |
|---|---|---|
| `And` elimination, `Imp` elimination (modus ponens), `Iff` elimination, `Bot` elimination, double-negation elimination | from formulas that hold, a formula that holds | a forward rule over `(holds F)`. Saturation applies it |
| `And` introduction, `Or` introduction, `Iff` introduction | from formulas that hold, a *larger* formula | a forward rule would build every conjunction of everything: a flood. On request, `(want (And A B))` asks for A and for B: the goal's shape decides the step |
| `Imp` introduction, `Not` introduction (reductio), `Or` elimination (cases) | from a *derivation under an assumption*, a formula with the assumption **discharged** | the question of this stage |

The third group is where the two readings part.

## Reading 1: an assumption is a commitment

Make `(holds A)` a candidate. In the states where it is committed, the elimination rules derive what follows
from A. Then:

- **Reductio's premise is exactly a dead state.** A rule says `(holds F)` and `(holds (Not F))` give
  `(false)`. If committing A kills the state, the engine reports A among N's minimal dead sets, with the
  core: the premises that, with A, suffice for the contradiction. The sketch, to be replaced by programs
  whose answers are computed, is probe 4's; on the release binary at `8860d616` (2026-11-11) it answered
  *1 candidate, 1 dead alone*, N of one set, R {∅}.

  ```lisp
  ;; sketch: from (not p) -> q and (not p) -> (not q), assume (not p) and meet a contradiction
  (relation Imp Formula Formula :why "if {?1} then {?2}")
  (relation Not Formula         :why "not {?1}")
  (relation holds Formula       :why "{?1} holds")
  (relation assumable Formula   :why "{?1} may be assumed")
  (holds (Imp (Not p) q))
  (holds (Imp (Not p) (Not q)))
  (rule imp-elim ()
    :match  (and (holds (Imp ?a ?b)) (holds ?a)) :assert (holds ?b)
    :why    "{?a}, and if {?a} then {?b}; so {?b}")
  (rule clash ()
    :match  (and (holds ?a) (holds (Not ?a))) :assert (false)
    :why    "{?a} and its negation both hold")
  (assumable (Not p))
  (hrule suppose ()
    :match (assumable ?f) :assert (holds ?f)
    :why   "suppose {?f}")
  (solve :hrules (suppose))
  (query :goal (holds ?f))
  ```

- **Reductio's conclusion is not derived.** In that run root holds the two premises and nothing more: not
  `(holds (Not (Not p)))`, nor `(holds p)`. A state's facts are its own; no rule reads another state or N.
- **`Imp` introduction has the same gap.** In the state where A is committed, B holds; that
  `(holds (Imp A B))` at root follows is a fact about two states.
- **`Or` elimination is the lattice's own shape,** read from outside: commit each disjunct, and what holds
  in every alive state is the conclusion. A query's projection classes show it. No rule concludes it.

So reading 1 gives each discharging rule's **premise** as a property of the lattice that `ein solve`
reports, and none of their conclusions as facts. Three ways on from there, and the user's ruling chooses:

- **(a) It stays a report.** The page says: Ein's search *is* the assume-and-refute step of natural
  deduction, and its read-out is where the discharged conclusion is read. Nothing changes in the engine.
- **(b) A refuted singleton becomes a stored negation,** on a program's request. The deleted walk wrote such
  facts back into root; why that was removed, and whether a declared, scoped form of it would be sound for
  monotone programs, is to be established from M1e2's record before it is proposed.
- **(c) A rule reads the lattice.** A premise such as *in the state that commits A, B holds*. The largest
  change; [S1g.6.5](../p1g.6_engine_requirements/s1g.6.5_rules_as_data_and_refutations.md) prices it.

## Reading 2: the context is data

The object-level answer, with no engine change: a judgment carries its open assumptions,
`(proves CTX F)`, the context a nested list. Then every rule is an ordinary rule over judgments:

```lisp
;; sketch: implication introduction and the assumption rule over judgments
(relation ctx Formula Context   :why "{?1}, then {?2}")
(relation want Context Formula  :why "a proof of {?2} from {?1} is asked for")
(relation proves Context Formula :why "from {?1}, {?2} is proved")

(rule imp-intro-ask ()                     ; to prove A -> B, assume A and prove B
  :match  (want ?c (Imp ?a ?b))
  :assert (want (ctx ?a ?c) ?b)
  :why    "to prove if {?a} then {?b}, assume {?a} and prove {?b}")
(rule imp-intro ()
  :match  (and (want ?c (Imp ?a ?b)) (proves (ctx ?a ?c) ?b))
  :assert (proves ?c (Imp ?a ?b))
  :why    "{?b} was proved from {?a}, so if {?a} then {?b}; {?a} is discharged")
(rule assumption ()                        ; the newest assumption proves itself
  :match  (want (ctx ?a ?c) ?a)
  :assert (proves (ctx ?a ?c) ?a)
  :why    "{?a} is assumed")
```

Run once with `(want nil (Imp p p))`, `nil` a name for the empty context: it derived `(proves (ctx p nil) p)`
and `(proves nil (Imp p p))`, with no candidate; with `(Imp p (Imp q p))` it derived nothing, since the
assumption rule sees only the newest assumption (`ein solve`, release binary at `8860d616`, 2026-11-11).

To be worked out: looking an assumption up deeper in the context (a walk, as the calculator walks a
number); weakening and the order of assumptions (a context is a list, the logic wants a set); the
elimination rules, which are not goal-directed (which implication to eliminate is a guess: the same wall as
S1g.3.2's cut formula, softened here because a normal proof only eliminates subformulas of the context,
which the stage uses as the universe); and classical logic, by reductio as a rule over judgments,
`(proves (ctx (Not A) C) Bot)` giving `(proves C A)`. Expected: a search-free prover for the
implication-and-conjunction fragment on request, and a bounded one beyond.

## Tasks

1. **T1g.3.3.1 — the two readings on the same five theorems,** shown to the user: identity, the syllogism,
   the proof by contradiction above, a proof by cases, and Peirce's law (which needs a classical rule).
   For reading 1, each read-out in full, with N and the cores, and exactly what is and is not a fact at root.
   For reading 2, each derived judgment. With them, Q-M1g.5's three options, (b)'s history from M1e2, and a
   recommendation.
2. **T1g.3.3.2 — the proof format and checker** for natural deduction (a line's reason names its rule, its
   premises' lines, and the assumption it discharges), on S1g.3.1's model; a program per rule; rejection of
   corrupted proofs. A checker needs no search under either reading.
3. **T1g.3.3.3 — reading 2 built,** as ruled: the rule pairs, on request; the search-free test where it is
   search-free; the reach measured where it is not.
4. **T1g.3.3.4 — the core against the proof.** For each refuted assumption of reading 1: is the core the set
   of premises the natural-deduction proof uses? On the banked examples, compared by the Rust harness.
5. **T1g.3.3.5 — the page:** *what a hypothesis is in natural deduction, in Ein*, with the table of rules
   and, for each, which reading carries it.

## Acceptance

- The user has ruled on Q-M1g.5 with the five theorems in front of them.
- Every rule of the system has a program; every banked proof is checked by saturation alone; every
  single-line corruption is rejected.
- The page states, per rule: an Ein rule outright, a rule over contexts, or a reading of the lattice; and for
  reductio, exactly what the engine derives, reports and does not conclude, each with its program.
- Soundness of what was built: every `proves` judgment is valid (the context entails the formula) by P1g.2's
  evaluator, on every program in the stage, checked in Rust.

## What it may find

- **That a dead state's core is a good proof of the negation,** or that it is not minimal or not canonical
  (F22). Either is reported with examples.
- **That intuitionistic and classical logic separate cleanly** in reading 2 (drop one rule) and not at all
  in reading 1 (the lattice refutes semantically). That would be the phase's most interesting sentence, and
  it is to be shown, not assumed.
- **A wrong answer in N or a core** on a program with nested facts as candidates. A register row first.
