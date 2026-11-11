# S1g.3.4 — Soundness and completeness, checked against the truth tables on every formula up to a bound; and the classical rules of inference as derived rules

**Phase:** [P1g.3](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.3.1](s1g.3.1_hilbert_systems.md), [S1g.3.2](s1g.3.2_finding_a_proof.md),
[S1g.3.3](s1g.3.3_natural_deduction.md); [P1g.2](../p1g.2_propositional_logic/README.md)'s evaluator and its
Rust check.
**Pulls in:** [F1b](../../followups/f1b_logical_formulation.md)'s PFL.2, *"check completeness against the
Wikipedia table"*; [idea 06](../../ideas/06-inference-rules-completeness.md)'s four senses of *enough*;
[F26](../../followups/f26_consistency_and_completeness.md)'s *completeness of a proof system (Gödel 1929)*.

## What it asks

A proof system is **sound** if everything it proves is a tautology and **complete** if every tautology has a
proof. For propositional logic both are theorems, proved by induction on proofs and on formulas, outside any
program. The stage does what a program can: it **checks both on every formula up to a bound**, with truth
decided by P1g.2's tables and provability by P1g.3's systems, and says exactly which part of each theorem
that check is.

It is also the one place where idea 06's question, *is this rule set enough?*, has a definition and a known
answer, so it is where the milestone shows what answering it looks like.

## What is checked, and by whom

| the claim | checked how | who decides truth | who decides provability |
|---|---|---|---|
| the checker is sound | every proof the checker accepts ends in a tautology | the Rust evaluator, and Ein's evaluation | S1g.3.1's and S1g.3.3's checkers |
| each axiom is valid | each schema's instances over the universe are tautologies; in Ein, one program a schema, [S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md)'s tautology question on the schema with letters for its metavariables | Ein | — |
| modus ponens preserves truth | the formula `(Imp (And A (Imp A B)) B)` is a tautology, in Ein | Ein | — |
| the system is complete up to the bound | every tautology up to the bound has a proof the checker accepts | the Rust evaluator | the checker, on a proof **supplied** by the Rust harness (any algorithm: Kalmár's construction, or a translation from a tableau) |
| Ein finds the proofs | the same, with the proof found by S1g.3.2's or S1g.3.3's search | — | Ein alone; reported as a reach, the smallest tautology not found named |
| the two systems agree | a formula is provable in one within the bound exactly when in the other | — | both checkers |

**The induction step is the part no bounded check gives.** Rows two and three *are* the induction's base and
step for soundness, checked by Ein: every axiom valid, the rule truth-preserving. From them soundness for
proofs of every length follows by an induction made outside Ein, as the calculator's column lemma does for
every width ([S1f.6.10](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.10_beyond_arithmetic.md),
item 4). The page states exactly which part Ein checked. Whether that induction can itself be written and
checked in Ein is [S1g.5.3](../p1g.5_peano_and_induction/s1g.5.3_induction.md)'s question about a different
induction, and is not attempted here.

## The classical rules of inference, as derived rules

F1b's PFL.2 names the Wikipedia table of rules for the propositional calculus. At the object level each is a
**derived rule** of the Hilbert system and of natural deduction: a schema with premises, admissible because
its conclusion is provable from its premises. The stage takes the table row by row (modus ponens, modus
tollens, hypothetical syllogism, disjunctive syllogism, constructive and destructive dilemma, simplification,
conjunction, addition, absorption, the De Morgan pair, commutation, association, distribution, double
negation, transposition, material implication and equivalence, exportation, tautology) and for each gives:

- its formula, `(Imp (And premises…) conclusion)`, shown a tautology by Ein;
- a checked proof in each system, from the harness;
- and, from [S1g.1.2](../p1g.1_survey_and_design/s1g.1.2_two_levels.md), whether the same rule at the meta
  level, over Ein's facts, is an Ein rule outright, needs a second rule, or is a reading of the lattice.

That table, three columns a row, is PFL.2 answered.

## Completeness in idea 06's other senses

Idea 06 lists four meanings of *enough*. For a propositional proof system written in Ein, one line each, to
be filled from the stage's runs: **functional** (a proof exists: completeness above), **propagation-complete**
(what saturation alone derives without a guess: the elimination-only fragment, measured),
**explanation-complete** (does a checked proof read as the textbook's, in `--trace`), **domain-complete**
(which logics the rule sets cover: classical, and intuitionistic by omission).

## Tasks

1. **T1g.3.4.1 — the harness's rows:** the enumeration of formulas up to the bound
   ([Q-M1g.9](../open_questions.md#q-m1g9--the-bound-of-an-exhaustive-check-and-its-tier)), a proof generator
   in Rust for each system, and the comparisons of the table. The bound, the count of formulas and the run
   time are stated; the larger bound goes to the tier its cost puts it in.
2. **T1g.3.4.2 — the validity programs** in Ein: one per axiom schema and per rule, each with a claim.
3. **T1g.3.4.3 — the derived-rules table,** every row with its three columns.
4. **T1g.3.4.4 — the reach:** which tautologies up to the bound Ein finds proofs of alone, by each way of
   S1g.3.2 and S1g.3.3.
5. **T1g.3.4.5 — the page,** `soundness_and_completeness.md` in this directory.

## Acceptance

- Sound up to the bound: no accepted proof ends in a non-tautology, on every proof the harness generates and
  every corruption of it the checker still accepts (there should be none that changes the conclusion).
- Complete up to the bound: every tautology has an accepted proof, for each system; or the formula on which
  one fails is named with the reason.
- Each axiom and rule is shown valid by an Ein program whose claim `ein test` holds.
- The derived-rules table is complete against the Wikipedia list as of the day it is taken, with the date.
- The page says which part of each theorem was checked by Ein, which by Rust, and which is the induction
  made outside.

## What it may find

- **A bug in a checker,** which is the point of the corruption sweep and of deciding truth independently.
- **That the harness's proofs are too long to check** past a small bound (Kalmár's construction is
  exponential in the letters). Then the bound is what it is, and stated.
- **That the meta-level column has more *needs a second rule* than expected,** which sharpens S1g.4.1's
  account of what Ein's negation is.
