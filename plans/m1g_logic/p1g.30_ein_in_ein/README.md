# P1g.30 — Ein in Ein: a theory of Ein written in Ein

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-14, on the user's words of that day ([§ The instruction](#the-instruction)). Not
started. **The milestone's last phase.** No stage file yet: the phase is this README, its stages are sections
below, and they are written out as files when [P1g.20](../p1g.20_theorems_proved_saved_reused/README.md) has run.
**Depends on:** P1g.20 (a proof system and theorems saved and reused, which this phase encodes);
[P1g.3](../p1g.3_proof_systems/README.md) and [P1g.4](../p1g.4_first_order_logic/README.md) (formulas as data,
the quantifier rules, and [S1g.4.1](../p1g.4_first_order_logic/README.md)'s statement of the fragment Ein's own
rules are); [P1g.5](../p1g.5_peano_and_induction/README.md)'s
[S1g.5.4](../p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md), *Gödel made concrete*, which asks what
self-reference needs and stops there: this phase writes it; whatever the second group built of rules as data
([S1g.6.5](../p1g.6_engine_requirements/s1g.6.5_rules_as_data_and_refutations.md)).
**Pulls in:** [F26](../../followups/f26_consistency_and_completeness.md)'s promotion 5, *a worked
self-referential program*; [M12](../../m12_rules_as_data/README.md), rules as data;
[idea 10](../../ideas/10-generic-self-modification.md)'s ladder and [idea 01](../../ideas/01-self-modifying-constraint-language.md)'s
grammar that describes itself; [idea 09](../../ideas/09-puzzles-beyond-zebra.md)'s liar.

## The instruction

The user, 2026-11-14:

> M1g another note: theory of Ein itself, another attempt to formalize theory of Ein in Ein. Like EBNF syntext
> defined in EBNF (this is not fully correct example, just to show an idea). Or like Godel numeration. So, one
> step towards Godel, Tarski, Self-reference, Self-reflection, Introspection and Metacircularity. Add separate
> phase about it in the very end of M1g

and, placing it: *"materialize both phases, let theorem-reuse be p1g.20 and this this self-theory p1g.30"*.

## What it is

The milestone formalizes theories in Ein; this phase takes **Ein itself** as the theory. Three levels, each held
to the real thing rather than proved about it ([M1g § What this is not](../README.md#what-this-is-not): no formal
semantics of Ein proved correct):

- **Syntax.** An Ein program as Ein facts, its forms, rules, premises and conclusions quoted as nested facts,
  and Ein's grammar written as Ein rules that decide whether a quoted program is well-formed: the grammar
  describing the language it is written in, as EBNF can describe EBNF.
- **Coding.** A program as a code, Gödel's numbering made concrete: Ein has no numbers, but the calculator's
  numerals are digit atoms, so a code can be a decimal numeral; beside it the quotation, a term that is its own
  code. What each buys, and the two directions held as theories.
- **Semantics.** Saturation written in Ein: a rule's match and assert over quoted rules and quoted facts, one
  step and then the fixpoint, a metacircular saturator in the sense of an evaluator written in the language it
  evaluates; then as much of the answer R (commitments, alive, dead, discharged) as the quoted semantics reaches.

On those three, the step the user names: **self-reference** (a program that speaks of its own code),
**Gödel** (a sentence of the encoded proof system that says it is unprovable there), **Tarski** (a truth predicate
for quoted facts, and where its undefinability bites), **introspection** (a program reading its own derivation)
and **reflection** (the step from reading rules to adding them, which is M12's rung, named and not built here).

## The stages, provisional

1. **S1g.30.1 — Survey and design; stops for the user.** The literature, each read for what an Ein program can
   show of it: EBNF in EBNF and grammars of grammars; McCarthy's `eval` and the metacircular evaluator;
   reflective towers (Smith's 3-Lisp); quines and Kleene's recursion theorem; Gödel numbering and the diagonal
   lemma; Tarski's undefinability; Löb's theorem; Hofstadter. The quotation chosen (which facts a quoted program
   is), the levels named apart (Ein running a program; a program describing Ein; Ein running that description),
   and the page's vocabulary fixed, as S1g.1.2 fixes the milestone's.
2. **S1g.30.2 — Ein's syntax in Ein.** Every corpus program quoted as facts (a generator writes the quotation);
   the grammar as Ein rules over the quotation, deciding well-formed or not, with the reason; **held to the
   parser and the loader on the whole corpus**: the Ein grammar's verdict on every quoted program equals the
   real one, refusals included, and a quoted mutation of a program is refused by both or by neither.
3. **S1g.30.3 — Coding.** Code and decode as Ein theories, a program to its numeral and back, the round trip
   held on the corpus; the quotation beside it; what a code makes writable that a quotation does not (arithmetic
   on programs, as Gödel's proof needs) and its cost.
4. **S1g.30.4 — Saturation in Ein.** The metacircular saturator: matching, binding and asserting over quoted
   rules and facts, one step, then the fixpoint; **held to the engine**, the quoted fixpoint equal to the
   engine's on a named set of programs (the positive, one-stratum ones first; then `absent` and its strata;
   then the hypothesis layer as far as it goes), with where it stops and why.
5. **S1g.30.5 — Self-reference and its limits.** A quine (a program whose saturation derives its own code); the
   diagonal lemma made concrete over a bound; the liar as a program (a fact that says its own quotation is not
   derived) and what R is for it, with [F21](../../followups/f21_non_monotone_programs_decided.md); Gödel's
   sentence for P1g.20's encoded proof system; a truth predicate for quoted facts and the place Tarski's theorem
   stops it. **S1g.5.4's discipline:** every claim shown by a program or marked as the textbook's, not Ein's.
6. **S1g.30.6 — Introspection and reflection, a report.** A program reading its own derivation (the event
   protocol and provenance, quoted as facts); the step from reading a rule to adding one, set against M12's
   rung 2 and idea 10's ladder; what the engine would need for each, listed as requirements for M12 and the
   followups, since this phase comes after the milestone's second group.

## Acceptance

- **Every corpus program quoted,** and the Ein grammar's verdict on each equal to the parser's and the loader's.
- **Code and decode round-trip** on the corpus.
- **The saturator written in Ein reaches the engine's fixpoint** on a named set of programs, and the page says
  exactly where it stops.
- **A quine runs,** and the self-reference page shows the liar, a Gödel sentence and Tarski's limit each by a
  program, or marks it as the textbook's.
- **What reflection needs is written down** as requirements with an owner.

## Risks

- **Size.** A quoted program is many times its text, and a saturator running a saturator multiplies the cost;
  each stage states its bound and fails loudly past it.
- **The levels blur, three of them now.** Ein running a program, a program describing Ein, and Ein running that
  description; a page that says *Ein proves* of the third misleads. S1g.30.1 names them and every stage keeps to
  the names.
- **Self-reference meets non-monotonicity.** The liar is a program with no stable reading; what R is for it is
  F21's question, and this phase shows it rather than rules on it.

## What it must not do

- **Not change Ein's rules at run time:** a program that adds rules to itself is M12's rung, described here, not
  built.
- **Not claim Ein incomplete or inconsistent** on the strength of an analogy: S1g.5.4's discipline holds.
