# S1g.3.1 — Hilbert systems: a proof as data, a checker by saturation, and two axiom sets compared

**Phase:** [P1g.3](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.2](../p1g.2_propositional_logic/README.md);
[S1g.2.3](../p1g.2_propositional_logic/s1g.2.3_normal_forms.md)'s rewriting into a basis.

## What it asks

*"different axiom sets e.g. Hilbert"* (the user, 2026-11-11). A Hilbert system is a set of axiom schemata and
one rule, modus ponens. The stage writes one in Ein, checks written proofs with it, then writes a second
axiom set and compares the two.

## A proof, and its checker

A proof is a list of lines; a line states a formula and gives its reason. The checker has a rule per axiom
schema, which recognises an instance by its shape, and one for modus ponens. A sketch, to be replaced by
programs whose answers are computed. It is probe 2's program: on the release binary at `8860d616`
(2026-11-11) it loaded and derived `checked` for all five lines, with no candidate and one saturation.

```lisp
;; sketch: the classical proof of p -> p from K and S, written out and checked
(relation Imp Formula Formula   :why "if {?1} then {?2}")
(relation step Line Formula Reason :why "line {?1} states {?2} by {?3}")
(relation ax-k                  :why "axiom K")
(relation ax-s                  :why "axiom S")
(relation mp Line Line          :why "modus ponens from lines {?1} and {?2}")
(relation checked Line Formula  :why "line {?1} proves {?2}")

(rule check-k ()                           ; A -> (B -> A)
  :match  (step ?n (Imp ?a (Imp ?b ?a)) (ax-k))
  :assert (checked ?n (Imp ?a (Imp ?b ?a)))
  :why    "line {?n} is an instance of K")
(rule check-s ()                           ; (A -> (B -> C)) -> ((A -> B) -> (A -> C))
  :match  (step ?n (Imp (Imp ?a (Imp ?b ?c)) (Imp (Imp ?a ?b) (Imp ?a ?c))) (ax-s))
  :assert (checked ?n (Imp (Imp ?a (Imp ?b ?c)) (Imp (Imp ?a ?b) (Imp ?a ?c))))
  :why    "line {?n} is an instance of S")
(rule check-mp ()
  :match  (and (step ?n ?b (mp ?i ?j)) (checked ?i (Imp ?a ?b)) (checked ?j ?a))
  :assert (checked ?n ?b)
  :why    "line {?n} follows from lines {?i} and {?j} by modus ponens")

(step l1 (Imp (Imp p (Imp (Imp p p) p)) (Imp (Imp p (Imp p p)) (Imp p p))) (ax-s))
(step l2 (Imp p (Imp (Imp p p) p)) (ax-k))
(step l3 (Imp (Imp p (Imp p p)) (Imp p p)) (mp l1 l2))
(step l4 (Imp p (Imp p p)) (ax-k))
(step l5 (Imp p p) (mp l3 l4))

(solve :hypothesis-relations ())
(query :goal (checked l5 ?f) :expect (model (checked l5 (Imp p p))))
```

What the stage settles about it:

- **A schema is recognised by pattern.** The repeated variable `?a` in `check-k`'s pattern is the schema's
  side condition, checked by the matcher; nothing else is needed. That is the whole of *how an axiom schema
  is represented*: a nested pattern with metavariables.
- **The proof's shape.** Lines as facts keyed by a name, as above, so a line can cite another; or one nested
  value, a proof term, `(mp (ax-s …) (ax-k …))`, which is a value a rule can pass and the Curry–Howard
  reading of the same proof (K and S are the combinators). Both to be built on the same proofs and compared:
  which the trace explains better, which the fact index serves (lines join on names; a proof term on nested
  values).
- **A wrong proof.** A line that cites the wrong lines or misstates its formula is simply not `checked`. The
  stage adds what says so in words: a rule that derives `(unchecked ?n)` needs a negative test, which the
  theory does not write; a claim that lists every `checked` line fails instead, and `ein test` reports it.
  Whether a *reason why it failed* can be derived monotonically (the first premise that is missing) is to be
  tried.
- **Derivations from premises:** a line whose reason is `(premise)` is checked when its formula is a
  `(given F)` fact. Then *Γ proves F* is a program with Γ at root.
- **The order of lines does not matter** to saturation: a line citing a later line is accepted, and a cycle
  of lines citing each other is not (nothing grounds it). The page says so; it is a difference from the
  textbook definition that costs nothing in soundness.

## Two axiom sets

- **Łukasiewicz's three,** over `Imp` and `Not`: K, S, and contraposition
  `(Imp (Imp (Not a) (Not b)) (Imp b a))`.
- **A second set, to be chosen at the stage's start** with the reason for the choice. The candidates: the
  Hilbert–Ackermann or Kleene sets with an axiom pair per connective for `And`, `Or` (no rewriting into a
  basis, more axioms, proofs that read closer to natural deduction); Mendelson's third axiom in place of
  contraposition; or Meredith's single axiom (the extreme: one schema, long proofs). The intuitionistic
  fragment, K and S and the connective axioms without any classical axiom, comes free as a subset, and
  Peirce's law is the formula that separates it: provable in one, not in the other, a tautology in both.
- **The comparison,** measured on a shared list of theorems (identity, the syllogism, double negation both
  ways, De Morgan, excluded middle, Peirce's law): the proof's length in each system, the rules of the
  checker, the firings of a check, and what each needs defined away first.

Where the proofs come from: textbook proofs written by hand for the short ones; for the rest, generated by
the Rust harness (which may translate a natural-deduction or a tableau proof by the deduction theorem) and
checked by Ein. Ein checks; it is not asked to find, here.

## Tasks

1. **T1g.3.1.1 — the proof format and the checker** for Łukasiewicz's set, lines as facts; the proof-term
   form beside it.
2. **T1g.3.1.2 — the banked proofs,** each a program with a claim; a program per checker rule.
3. **T1g.3.1.3 — rejection:** every single-line corruption of each banked proof (a wrong citation, a wrong
   formula, a wrong reason), generated, fails its claim. Held by a test.
4. **T1g.3.1.4 — the second axiom set,** its checker and the same theorems.
5. **T1g.3.1.5 — the comparison page,** and the search-free test's rows.

## Acceptance

- Every banked proof is checked by saturation alone: no candidate, R {∅}, held by the search-free test.
- Every single-line corruption is rejected, held by a test.
- The Rust check: every checked line's formula is a tautology (it calls P1g.2's evaluator), on every banked
  and generated proof.
- Two axiom sets, the same theorems, and the comparison with its numbers and commands.

## What it may find

- **The checker's joins are on lines' names,** which the index serves, so a long proof should check in
  proportion to its length; if it does not, the witness is S1g.6.2's.
- **A schema with a side condition a pattern cannot state.** None of the propositional schemata has one; the
  quantifier axioms of [S1g.4.4](../p1g.4_first_order_logic/s1g.4.4_quantifier_rules.md) do (*x not free
  in A*), and this stage notes where the checker will have to grow.
