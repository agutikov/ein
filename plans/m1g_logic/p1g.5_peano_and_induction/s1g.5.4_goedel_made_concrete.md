# S1g.5.4 — Gödel made concrete: what the theory decides, a search that does not end, the liar, and what self-reference needs

**Phase:** [P1g.5](README.md)
**Status:** planned 2026-11-11, not started. A report with programs; it builds no theory.
**Depends on:** [S1g.5.1](s1g.5.1_unary_numbers_and_the_axioms.md) to [S1g.5.3](s1g.5.3_induction.md);
[S1g.4.5](../p1g.4_first_order_logic/s1g.4.5_the_boundary.md)'s decidability table and glossary.
**Pulls in:** [F26](../../followups/f26_consistency_and_completeness.md)'s § *Does Gödel's theorem say
anything about an Ein program?*, its five bullets, and promotion 5 (*a worked self-referential program, once
rules are data: the liar and a Gödel-style sentence, and what R is for each*);
[idea 09](../../ideas/09-puzzles-beyond-zebra.md)'s liar and barber.

## What it asks

F26 was filed on the user's question about *"connections to consistency and completeness in common sense
(Godel and futher)"*, and its Gödel section is marked a sketch, *checked by nothing*. With Peano's
arithmetic written in Ein there are programs to check it against. The stage takes F26's five bullets one at a
time and, for each, either shows it with a program or says it stays the textbook's statement about a system
Ein only resembles.

It is a report, and its discipline is the hard part: the incompleteness theorems are about first-order
theories with a recursive axiom set and full quantification. Almost nothing in this milestone is literally
such a theory, and a page that slides from *an Ein program* to *Peano arithmetic* to *a proof system written
in Ein* without saying which is meant would be wrong in the way popular accounts of Gödel are wrong.

## The five bullets, and what a program can show of each

**1. *A zebra program is far below it.*** Its names are finite, H is finite, every question is decided.
Shown by pointing at [S1g.4.5](../p1g.4_first_order_logic/s1g.4.5_the_boundary.md)'s first row. And the
sense in which such a program is *incomplete* (R has several states) set beside Gödel's sense, with
[S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md)'s contingent formula: a
propositional theory that does not decide `p` is incomplete, decidably and unremarkably so. This is the
bullet that keeps the rest honest: *incomplete* is not the surprising word, *essentially* incomplete is.

**2. *Nested facts change the class.*** Consistency becomes undecidable. S1g.4.5 builds the machine; this
stage adds the arithmetic face of it: a program that **searches the numbers for a witness and derives
`(false)` on finding one**, driven by a successor-generating rule. A sketch, to be tried
under a bound; `is-witness` and `tried-without` stand for whatever property the instance tests, each to be
computed on request by S1g.5.1's theory. Run once with the property written as facts (0 and 1 lack it, 2 has
it): root was dead, *refuted by found* (`ein solve`, release binary at `8860d616`, 2026-11-11):

```lisp
;; sketch: look at 0, 1, 2, … for a number with a property; root dies on the first
(relation s Nat           :why "the successor of {?1}")
(relation try Nat         :why "{?1} is tried")
(relation is-witness Nat  :why "{?1} has the property")
(relation tried-without Nat :why "{?1} was tried and lacks the property")
(try z)
(rule next ()                              ; a failure asks for the next number
  :match  (tried-without ?n)
  :assert (try (s ?n))
  :why    "{?n} is not it; try the next")
(rule found ()
  :match  (and (try ?n) (is-witness ?n))
  :assert (false)
  :why    "{?n} has the property")
```

  With a property that some number has, root is dead and the core names the witness: a Σ₁ sentence, true,
  and **proved by running**. With a property none has, the program does not stop, and no finite run says so.
  That asymmetry is the content of *Q proves every true Σ₁ sentence and is incomplete beyond them*, and it is
  the same asymmetry as F26's *the calculator computes, and does not quantify*. Instances, in rising
  interest: an even number that is odd (none: provably, by a two-line induction the search cannot find); a
  counterexample to a small identity; a number whose Collatz trajectory, bounded in steps, does not reach 1
  (S1f.6.10's item 5); an even number above 2 that is not a sum of two primes. The last is a real open
  problem written as an Ein program whose consistency nobody knows, which is the plainest way to show the
  user what *consistency is undecidable* means for Ein.

**3. *The calculator computes, and does not quantify.*** Shown by S1g.5.3's closing table: ground equations
by saturation; a universal only as a checked object-level proof. The stage adds which named theory each
piece corresponds to, carefully: S1g.5.1's rules derive the true ground equations and inequations of the
language of arithmetic, which is Robinson's Q's strength on closed quantifier-free sentences; S1g.5.3's
checker with the induction rule is a proof checker *for* first-order Peano arithmetic, as far as it was
built. Ein's own rule language is neither.

**4. *Self-reference needs rules as data.*** Two programs to try, and one to describe:

- **The liar as a puzzle** (idea 09). At the object level, with P1g.2's formulas: a letter `L` required to
  be equivalent to its own negation, `(must-hold (Iff L (Not L)))`. Expected: R empty, both values dead,
  the cores two lines long. That is not a paradox, only an unsatisfiable requirement, and the page says why:
  the *sentence* does not refer to itself; the modeller wrote an equation.
- **The liar in Ein's own negation as failure:** a rule that concludes `p` when `p` is absent. A program
  with a cycle through `absent` and no stable reading. What the engine does with it today (the strata
  diagnostic since S1f.5.15; *forbid*'s refusal; the flagged-lattice description) is shown as it is, and
  [F21](../../followups/f21_non_monotone_programs_decided.md) owns what it should do.
- **A Gödel sentence** for the proof checker of S1g.5.3 would say *I have no checked proof*. To write it one
  must (a) code formulas and proofs as numbers of the object theory, or accept terms in place of numbers,
  which the object language here could do since formulas are already terms; (b) define *is a proof of* as
  an object-level formula, which means writing the checker *again*, inside the object language it checks;
  and (c) build the fixed point by the diagonal lemma. None of it needs a new engine feature; all of it is a
  large construction with a known outcome. **Described, not built.** What *would* need the engine is the
  other reading: an Ein rule that speaks of Ein's own rules or of its own derivability, which is
  [F5](../../followups/f5_rules_as_data.md), and
  [S1g.6.5](../p1g.6_engine_requirements/s1g.6.5_rules_as_data_and_refutations.md) is where that requirement
  is priced.

**5. *The second theorem, and what a certificate is.*** No program. One paragraph relating three things a
reader may run together: `exhausted` (every commitment over H classified: a statement about one run); a
green gate (the engine agrees with its tests and its oracle); and consistency of a theory. With a pointer to
[`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md) for the second.

## Tasks

1. **T1g.5.4.1 — the witness search,** on four properties, each run under a bound that fails loudly (or
   under S1g.6.1's word, once it exists): the two that halt with their cores; the two that do not, with what
   the run said.
2. **T1g.5.4.2 — the two liars,** each with every artefact of its run.
3. **T1g.5.4.3 — the Gödel sentence, described:** the three steps, each with what in the milestone it would
   reuse and what it would have to add, and an honest size.
4. **T1g.5.4.4 — the page,** `goedel.md` in this directory, bullet by bullet, each sentence marked *shown by
   a program here* or *the textbook's*; and F26's file annotated with what was established.

## Acceptance

- Each of F26's five bullets has a section that says what was shown, by which program, and what was not.
- No sentence attributes a property of first-order Peano arithmetic to an Ein program without saying which
  object is meant.
- The non-terminating programs are banked as examples only in a form the gate can run (bounded), with the
  unbounded form on the page.
- The user is shown the page; what, if anything, becomes further work is theirs to say.

## What it may find

- **That the engine's behaviour on the `absent` liar is a wrong answer** under some command. A register row,
  owned by F21.
- **That the witness search is a natural program people will write,** in which case the word for *did not
  stop* is not a curiosity but the most-used read-out of this family, which raises S1g.6.1's priority.
