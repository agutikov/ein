# P1g.3 — Proof systems: Hilbert's axioms, natural deduction, and what a proof is in Ein

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.2](../p1g.2_propositional_logic/README.md): the formula's shape, the truth tables every
system here is checked against, and the rewriting into a basis.
**Pulls in:** [F1b](../../followups/f1b_logical_formulation.md)'s PFL.2 (*"encode the propositional-calculus
rules of inference as ein activators / rules; check completeness against the Wikipedia table"*);
[idea 06](../../ideas/06-inference-rules-completeness.md)'s question of when a rule set is *enough*, in the
one setting where the question has a textbook answer.

## The instruction's part

> rules of inference? what are hypothesis in this case? different axiom sets  e.g. Hilbert, natural deduction
> (introduction and elimination of ...)

## What it is

P1g.2 computed truth. This phase computes **provability**, by rules that never consult a truth table, and
then checks the two against each other. For each proof system it asks three separate things, which have
three different answers in Ein:

1. **How a proof is represented and checked.** A proof is data the program writes: lines, each a formula and
   a reason. Checking it is saturation over rules that recognise each reason. Search-free, like the
   calculator: probe 2 checked a five-line Hilbert proof of `(Imp p p)` in one saturation
   ([M1g § First probes](../README.md#first-probes-2026-11-11)).
2. **How a proof is found.** Forward saturation derives every theorem it can reach, and a schema reaches
   everything, so it floods; and an axiom schema cannot even be written as a bare forward rule, since a
   conclusion's variable must be bound
   ([`defined_behaviour.md` § 4.13](../../../docs/kernel/defined_behaviour.md)). The candidates are the
   calculator's **request protocol** turned to goals (*prove this*, answered by a proof), and **search with
   hypotheses**, where a guess is a line or a cut formula. Which works, and how far, is to be tried.
3. **Whether the system is sound and complete,** checked and not proved: on every formula up to a bound,
   *provable* and *tautology* agree ([S1g.3.4](s1g.3.4_soundness_and_completeness.md)).

And it states **what a hypothesis is**, which differs by system:

- **In a Hilbert system, nothing is.** There are axioms and modus ponens. A derivation *from premises* has
  its premises as facts at root. Ein's lattice has no part in checking one.
- **In natural deduction, an assumption is.** It is opened, used, and discharged by an introduction rule.
  The plan's claim, **to be tested and not assumed**, is that an assumption maps to a commitment of Ein's
  lattice, and reductio (assume A, derive a contradiction, conclude `Not A`) to a dead state and its core.
  Probe 4 already bounds the claim: the engine *reports* the dead assumption and its core, and concludes
  nothing from it at root. So the mapping gives reductio's premise and not its conclusion, and
  [S1g.3.3](s1g.3.3_natural_deduction.md) is where the rest is worked out, with
  [Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact) for the user.
- **In proof search, a guess is,** in any system.

## The stages

| ID | title | depends on | status |
|---|---|---|---|
| [S1g.3.1](s1g.3.1_hilbert_systems.md) | Hilbert systems: a proof as data, a checker by saturation, and two axiom sets compared | P1g.2 | planned |
| [S1g.3.2](s1g.3.2_finding_a_proof.md) | Finding a proof: the flood measured, goals as requests, guesses as hypotheses, and the deduction theorem | S1g.3.1 | planned; a report first |
| [S1g.3.3](s1g.3.3_natural_deduction.md) | Natural deduction: introduction and elimination for each connective, assumptions, discharge, and reductio against Ein's dead state | S1g.3.1 | planned; **stops for the user** |
| [S1g.3.4](s1g.3.4_soundness_and_completeness.md) | Soundness and completeness, checked against the truth tables on every formula up to a bound; and the classical rules of inference as derived rules | S1g.3.1 to .3 | planned |
| [S1g.3.5](s1g.3.5_other_calculi.md) | **Optional:** sequent calculus, resolution, analytic tableaux | S1g.3.3; S1g.2.3's clauses | planned; runs only for the systems the user picks |

## The order

S1g.3.1 first, since every later stage reuses its proof format and checker. S1g.3.2 and S1g.3.3 beside each
other. S1g.3.4 once both have a notion of *provable*. S1g.3.5 last and only on the user's word
([Q-M1g.10](../open_questions.md#q-m1g10--the-optional-proof-systems)).

## Acceptance

- **For each system, a written proof is checked by saturation alone,** held by the search-free test, and a
  wrong proof is rejected: a line that does not follow is not `checked`, and the program's claim fails. The
  mutation idea of `utils/stdlib_mutants.py`, applied to proofs: every single-line corruption of each banked
  proof is rejected.
- **Two Hilbert axiom sets,** Łukasiewicz's three and one other, each with the same theorems proved, and a
  page that compares them on what was measured (proof lengths, rules, what each needs defined away).
- **Natural deduction has a rule pair per connective,** each with a program, and the page says, for each
  rule, whether it is an Ein rule outright, a rule over contexts as data, or a reading of the lattice.
- **Sound and complete up to the bound,** or the formula named on which a system is not, with the reason.
  The Rust check's rows for this phase: every checked proof's conclusion is a tautology (soundness of the
  checker), and every tautology up to the bound has a proof the theory finds or the harness supplies
  (completeness, as far as the search reaches, with the reach stated).
- **What a hypothesis is in each system** is stated on the phase's page with the program that shows it.
- Every number carries its command.

## Risks

- **Proof search is the hard half and may mostly fail.** The phase is planned so that checking, comparing
  and the soundness half do not depend on it: a proof can be supplied by the Rust harness, which may use any
  algorithm, and Ein checks it. What Ein finds alone is reported as a reach, not promised.
- **Contexts as data need sets.** A natural-deduction judgment has a set of open assumptions. As a nested
  list it has an order and repetitions; two derivations of one judgment then differ as values. The stage
  that meets it says which structural rules it had to write.
- **The attractive mapping is partly false.** If an assumption is a commitment, discharge crosses states,
  and no rule does. The phase says so plainly and prices the alternatives rather than presenting the part
  that works as the whole.
