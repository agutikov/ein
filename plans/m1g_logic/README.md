# M1g — Logic in Ein: from truth tables to proof systems, first-order logic and Peano arithmetic

**Status:** planned 2026-11-11, on the user's instruction of that day ([§ The instruction](#the-instruction)).
Not started. It carries no estimate (the user's ruling of 2026-11-08).
**Depends on:** [M1f](../m1f_hypothesis_and_documentation/README.md), complete (the user, 2026-11-11: *"first
we will complete M1f"*). Specifically:
[S1f.6.16](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.16_binary_arithmetic.md), binary
arithmetic, and [S1f.6.17](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md),
gates and truth tables, which are this milestone's starting point and stay stages of P1f.6; `std.arith`, the
decimal calculator ([P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md)), which P1g.5
holds Peano's numbers to; and the language as P1f.5 and P1f.7 ruled it, the value rule above all
([`defined_behaviour.md` § 4.22](../../docs/kernel/defined_behaviour.md)).
**Blocks:** nothing.
**Promotes:** [F1b](../followups/f1b_logical_formulation.md), the logical formulation, whose *What promotion
would look like* names a milestone with phases PFL.1 to PFL.4. This is that milestone
([§ What it pulls in](#what-it-pulls-in)).

## The instruction

The user, 2026-11-11:

> new more stages: arithmetic with binary numbers, I suspect simpler rules; binary logic, truth tables, can be
> there any applications for it? new phase: from binary logic to preposition logic, what can be represented
> already and how? pull in related ideas/ and followups/, how preposition logic formulas can be represented?
> rules of inference? what are hypothesis in this case? different axiom sets  e.g. Hilbert, natural deduction
> (introduction and elimination of ...); First order logic, what we can implement already and how, what is
> nearly possible, what is required.   Natural numbers, Peano, induction. What make a new stage, what a new
> phase and what a new mile stone, M1g, M1h ?

(*preposition logic* is propositional logic; the quote is as written.)

The coordinator proposed one milestone, M1g, with five phases, and a second, M1h, only if the survey justified
an engine milestone. The user, the same day:

> 1 - go ahead, 2 - first we will complete M1f, 3 - no M1h need, M1g already contains everything I request;
> Naming contraint for binary logic - e.g. for "and" element, function can use _and, _and_, __and, AND, And,
> l_and, And2 - whatever

So:

- **M1g is one milestone and holds everything asked for,** the engine work the logic turns out to need
  included. There is no M1h. The engine work is a phase of its own, [P1g.6](p1g.6_engine_requirements/README.md).
- **It runs after M1f is complete.**
- **Binary arithmetic and binary logic stay stages of M1f's P1f.6,** S1f.6.16 and S1f.6.17, and are where M1g
  starts from.
- **Connective names are free.** The engine's words `and`, `or`, `eq`, `neq`, `absent` and `false` head no
  value under the value rule, and `not` heads a stored negation, which is the engine's and not a formula's. So
  a formula spells its connectives otherwise. **This plan writes them capitalised,** `And`, `Or`, `Not`, `Imp`,
  `Iff`, with `Top` and `Bot` for the two constants: a name is case-sensitive, and a capital is the lightest
  mark. It is one convention for the whole milestone and it is revisable
  ([Q-M1g.1](open_questions.md#q-m1g1--the-spelling-of-the-connectives)); S1f.6.17 picks the gates' spelling
  first, and M1g follows it if it differs.

## What this is

A study, in runnable programs, of what Ein's saturation and hypothesis lattice do with logic. Ein has two
mechanisms: **saturation**, which closes a state under rules, and **the lattice of commitments**, whose alive
and discharged part is the answer, the region R
([`solution_semantics.md`](../../docs/kernel/inference/solution_semantics.md)). Logic has several things
that look like each: derivation and valuation, proof and model, assumption and case. The milestone writes each
part of elementary logic as an Ein program, computes its answers, holds them to an independent check, and says
which of Ein's two mechanisms did the work.

It works at **two levels**, and keeping them apart is half of what it has to say
([S1g.1.2](p1g.1_survey_and_design/s1g.1.2_two_levels.md)):

- **Ein as the logic.** A rule of Ein *is* a rule of inference, a universally quantified implication applied
  forward; `absent` is negation as failure; an obligation is an existential that wants a witness; a hypothesis
  is a case. What fragment of first-order logic this is, is F1b's question and
  [P1g.4](p1g.4_first_order_logic/README.md)'s.
- **A logic written in Ein.** A formula is a nested fact, `(Imp p (Imp q p))`, a value like any other. A
  truth table, a proof and a proof system are then facts and rules *about* formulas, and Ein's own rules are
  the metalanguage. [P1g.2](p1g.2_propositional_logic/README.md), [P1g.3](p1g.3_proof_systems/README.md) and
  [P1g.5](p1g.5_peano_and_induction/README.md) are mostly this.

## What this is not

- **Not a theorem prover that competes with Lean, Coq or a SAT solver.** A truth table of n letters is 2ⁿ
  states of R, and the region limit is where it ends. The milestone measures how small *small* is and says so;
  [M10](../m10_external_benchmarks/README.md) is where Ein is compared with other tools.
- **Not engine arithmetic.** [Q-M1e2.24](../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein)
  stands: no numbers, no ranges, no arithmetic predicate. Peano's numbers are nested facts, `z` and `(s N)`,
  as the calculator's are `(n PREFIX DIGIT)`.
- **Not new syntax by default.** Every program in the plan is written in the grammar as it ships. Where the
  logic wants something the grammar refuses, that is a row of P1g.6 and a question for the user, never a
  silent change.
- **Not a formal semantics of Ein proved correct.** P1g.4 *states* the fragment and tests each row with a
  program; it proves no meta-theorem. [F1](../followups/f1_categorical_formulation.md), the categorical
  reading, stays parked.

## The phases

| ID | title | stages | ends with |
|---|---|---:|---|
| [P1g.1](p1g.1_survey_and_design/README.md) | Survey and design: what Ein represents today, on real programs | 4 | a page of programs with computed answers, the two levels set apart, the conventions, and the list of engine requirements P1g.6 schedules. **Stops for the user**; nothing ships before the user reads it |
| [P1g.2](p1g.2_propositional_logic/README.md) | Propositional logic: formulas, truth tables, valuations as hypotheses, normal forms | 4 | a formula evaluated by saturation alone; its truth table as R; tautology, satisfiability and equivalence as questions about R; NNF, CNF and DNF by rewriting; each held to a check written in Rust on every formula up to a bound |
| [P1g.3](p1g.3_proof_systems/README.md) | Proof systems: Hilbert's axioms, natural deduction, and what a proof is in Ein | 5 (1 optional) | a written proof checked by saturation; two Hilbert axiom sets compared; natural deduction with assumptions and discharge; soundness and completeness checked against P1g.2's truth tables up to a bound; and a plain statement of what a hypothesis is in each |
| [P1g.4](p1g.4_first_order_logic/README.md) | First-order logic: the fragment Ein is, relation algebra, and formulas with quantifiers as data | 5 | F1b's PFL.1 to PFL.4: the fragment as a table with a program per row, the relation algebra inventory, object-level quantifiers over a finite structure, the quantifier rules, and the boundary of what Ein cannot decide |
| [P1g.5](p1g.5_peano_and_induction/README.md) | Natural numbers: Peano's axioms, recursion, induction, and Gödel made concrete | 4 | unary numbers and the axioms as rules; addition and multiplication by recursion, held to `std.arith`; induction as a schema against bounded checking; and what of F26's Gödel section a program can show |
| [P1g.6](p1g.6_engine_requirements/README.md) | The engine requirements the logic turns out to have | 5, provisional | each requirement P1g.1 lists, scheduled, ruled or given an owner; seeded today with the candidates already known |

## The order

1. **P1g.1 first, and it stops.** Its page is read by the user before any other phase writes a file outside
   `plans/` and `build/` (the user's rule: real examples first).
2. **P1g.2,** which every later phase checks against: a truth table is the semantics a proof system is sound
   and complete for.
3. **P1g.3 and P1g.4's first two stages** may run side by side. S1g.4.1 and S1g.4.2 read Ein's own rule
   language and need nothing of P1g.3.
4. **P1g.4's object-level stages** (S1g.4.3, S1g.4.4) after P1g.3, whose proof checker they extend.
5. **P1g.5** after S1g.4.4, since an induction proof is a first-order proof; its first two stages need only
   P1g.1 and may run earlier.
6. **P1g.6 is not last.** Each of its stages runs when the phase that needs it reaches the need, and its
   stage list is settled at P1g.1's stop. A requirement no later phase needs is ruled or given an owner at the
   close, and is not built for its own sake.

## The user's questions, answered in outline

The **expected answer** column is the coordinator's, written before any stage has run. Each is **untested**
unless it cites a probe of [§ First probes](#first-probes-2026-11-11), and a probe is one run of one scratch
program, re-taken by the stage named.

| the question | answered by | expected answer |
|---|---|---|
| *from binary logic to preposition logic, what can be represented already and how?* | [S1g.1.1](p1g.1_survey_and_design/s1g.1.1_what_ein_represents_today.md); built by [P1g.2](p1g.2_propositional_logic/README.md) | Much of it, today, with no engine change. A formula is a nested fact; its value under a valuation is derived by rules over four truth tables; the valuations are hypotheses. Probe 1 |
| *how preposition logic formulas can be represented?* | [S1g.1.3](p1g.1_survey_and_design/s1g.1.3_conventions.md), [S1g.2.1](p1g.2_propositional_logic/s1g.2.1_formulas_and_evaluation.md) | `(Imp (And p q) p)`: a propositional letter is a name, a connective an ordinary relation with a capitalised name, a compound formula a nested fact. A formula is a value under the value rule because `And` is an ordinary relation. Whether formulas should also be circuits with named wires, as S1f.6.17 has them, is compared there |
| *rules of inference?* | [S1g.1.2](p1g.1_survey_and_design/s1g.1.2_two_levels.md), [P1g.3](p1g.3_proof_systems/README.md) | Two answers. An Ein rule already is one: premises, a conclusion, applied forward to a fixpoint. And a rule of the logic under study, modus ponens over formula objects, is an Ein rule whose premises are facts about formulas, `(holds (Imp ?a ?b))` and `(holds ?a)` giving `(holds ?b)`. Probes 2 and 4 |
| *what are hypothesis in this case?* | [S1g.1.2](p1g.1_survey_and_design/s1g.1.2_two_levels.md), [S1g.2.2](p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md), [S1g.3.3](p1g.3_proof_systems/s1g.3.3_natural_deduction.md) | It depends on the setting, and the three are not the same thing. **Semantics:** a hypothesis is one letter's value, and R is the set of valuations. **Natural deduction:** an assumption maps to a commitment, and a refuted one to a dead node with its core; but Ein *reports* the refutation and does not conclude `Not A` from it (probe 4), so discharge is the open part. **Hilbert's systems** have no hypotheses at all; a derivation *from* premises has them as root's facts |
| *different axiom sets e.g. Hilbert* | [S1g.3.1](p1g.3_proof_systems/s1g.3.1_hilbert_systems.md), [S1g.3.2](p1g.3_proof_systems/s1g.3.2_finding_a_proof.md) | An axiom schema is a rule that recognises its instances; a written proof is checked by saturation (probe 2). *Finding* a proof is harder: a schema's free variable is refused in a conclusion ([§ 4.13](../../docs/kernel/defined_behaviour.md)), so forward saturation needs a universe of formulas, and floods on it. To be measured |
| *natural deduction (introduction and elimination of ...)* | [S1g.3.3](p1g.3_proof_systems/s1g.3.3_natural_deduction.md) | Elimination rules are forward rules and fit saturation. Introduction rules that discharge an assumption (`Imp`'s, `Not`'s) reason across two states, which no Ein rule does today. Either the context is data, `(proves CTX A)`, or the lattice is read from outside. To be tried both ways |
| *First order logic, what we can implement already and how* | [S1g.4.1](p1g.4_first_order_logic/s1g.4.1_the_fragment.md), [S1g.4.2](p1g.4_first_order_logic/s1g.4.2_relation_algebra.md) | Ein's rules are Horn clauses with constructors, plus stored negation, negation as failure, integrity constraints, existentials as obligations over the names the program has, and disjunction only as search. The table with a program per row is the stage |
| *what is nearly possible* | [S1g.4.3](p1g.4_first_order_logic/s1g.4.3_formulas_with_quantifiers.md), [S1g.4.4](p1g.4_first_order_logic/s1g.4.4_quantifier_rules.md) | Quantified formulas as data, evaluated over a finite structure; Skolem terms as nested facts (probe 5); substitution as a rewriting theory |
| *what is required* | [S1g.4.5](p1g.4_first_order_logic/s1g.4.5_the_boundary.md), [P1g.6](p1g.6_engine_requirements/README.md) | A word for a saturation that does not stop (probe 3), fresh names, a refutation a rule can read, rules as data. The list is P1g.1's to settle |
| *Natural numbers, Peano, induction* | [P1g.5](p1g.5_peano_and_induction/README.md) | Numbers and the recursion equations compute today (probe 6). Induction is not a fact a state can hold: it is a schema over formulas with a variable, checked as a proof step, or replaced by checking up to a bound |
| *What make a new stage, what a new phase and what a new mile stone, M1g, M1h?* | [§ Stage, phase, milestone](#stage-phase-milestone) | Answered below, and ruled: one milestone |

## First probes (2026-11-11)

Six scratch programs the planning agent ran once while writing this tree, on the release binary built at
`8860d616`, with `ein solve FILE` unless said otherwise. They are **not** results: no file is banked, no test
holds them, and S1g.1.1 re-takes each as a kept program. They are here so that the plan's *expected answers*
are told apart from its guesses. Each program is reproduced, as a sketch, in the stage that re-takes it.

| # | the program | what `ein solve` said | re-taken by |
|---|---|---|---|
| 1 | Formulas as nested facts over `And`, `Or`, `Imp`, `Not`; four truth tables; evaluation on request; two letters whose values an hrule proposes, an obligation that each has a value, and a restriction that none has two; Peirce's law `(Imp (Imp (Imp p q) p) p)` asked for | k_s 4, the four valuations; the query's 4 states fall in **1 projection class, `?v = 1`**: a tautology, read off R. 11 saturations | [S1g.2.2](p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md) |
| 1b | The same with `(And p (Not p))` required to be 1 | k_s 0, *empty: every alive node owes*; 4 candidates, 2 dead alone. Unsatisfiable shows as an empty R whose alive nodes cannot pay `p`'s value, not as *every node is dead* | [S1g.2.2](p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md) |
| 2 | A written five-line Hilbert proof of `(Imp p p)` from K and S by modus ponens, each line a fact, three checking rules | all five lines `checked`; k_s 1, R {∅}, no candidate, 1 saturation: proof checking is saturation | [S1g.3.1](p1g.3_proof_systems/s1g.3.1_hilbert_systems.md) |
| 3 | `(nat z)` and a rule `(nat ?n)` gives `(nat (s ?n))`, under `ein saturate` | did not stop: 8 388 505 facts when a 20 s timeout ended it, and no word for it | [S1g.6.1](p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md) |
| 4 | Two premises, `(Imp (Not p) q)` and `(Imp (Not p) (Not q))`; modus ponens; a rule that `A` and `(Not A)` are `(false)`; `(Not p)` assumable as a hypothesis | 1 candidate, **1 dead alone**, N has 1 minimal dead set, R {∅}. The assumption is refuted and the refutation reported; root does **not** hold `(Not (Not p))` | [S1g.3.3](p1g.3_proof_systems/s1g.3.3_natural_deduction.md) |
| 4b | The same with the assumption written as an activator's argument, `(solve :hrules (suppose ((Not p))))` | a parse error at the inner `(`: an activator's argument in `:hrules` is a name, never a nested fact | [S1g.6.4](p1g.6_engine_requirements/s1g.6.4_witnesses_and_fresh_names.md) |
| 5 | `(person Ann)` and a rule concluding `(parent (father-of ?x) ?x)` | `(parent (father-of Ann) Ann)`: a Skolem term as a nested fact, one level | [S1g.4.4](p1g.4_first_order_logic/s1g.4.4_quantifier_rules.md) |
| 6 | Unary numbers; `(plus X Y)` asked, answered by `(plus-of X Y R)` through two recursion rules | 2 + 3 = `(s (s (s (s (s z)))))`, R {∅} | [S1g.5.1](p1g.5_peano_and_induction/s1g.5.1_unary_numbers_and_the_axioms.md) |

**The sketches in the stage files** were each run once the same way, joined to what they say they rest on,
so that every sketch is at least a program the binary loads. Beyond the six probes: evaluation under a
written valuation needs its relations closed, or the blind enumerator proposes 62 candidates
([S1g.2.1](p1g.2_propositional_logic/s1g.2.1_formulas_and_evaluation.md)); negation normal form on request
answered one formula ([S1g.2.3](p1g.2_propositional_logic/s1g.2.3_normal_forms.md)); a knights-and-knaves
puzzle's claim held ([S1g.2.4](p1g.2_propositional_logic/s1g.2.4_applications_and_scale.md)); an axiom
schema written as a bare forward rule is refused at load
([S1g.3.2](p1g.3_proof_systems/s1g.3.2_finding_a_proof.md)); implication introduction over contexts as data
proved `(Imp p p)` and not `(Imp p (Imp q p))`
([S1g.3.3](p1g.3_proof_systems/s1g.3.3_natural_deduction.md)); a `forall` detector derived symmetry from a
written relation ([S1g.4.2](p1g.4_first_order_logic/s1g.4.2_relation_algebra.md)); and a search over the
numbers killed root at its witness ([S1g.5.4](p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md)).
Each stage file says what its sketch did and did not compute. The same caution applies: one run each, nothing
banked.

## What it pulls in

Each idea and followup the instruction's *"pull in related ideas/ and followups/"* reaches, with what of it
goes where. A followup that is pulled in keeps its file; only F1b is promoted whole.

| source | what of it | goes to |
|---|---|---|
| [F1b](../followups/f1b_logical_formulation.md), logical formulation | **promoted here, 2026-11-11.** PFL.1, the fragment as a BNF over kernel forms | [S1g.4.1](p1g.4_first_order_logic/s1g.4.1_the_fragment.md) |
| | PFL.2, the propositional rules of inference against the Wikipedia table | [S1g.3.4](p1g.3_proof_systems/s1g.3.4_soundness_and_completeness.md), with [S1g.1.2](p1g.1_survey_and_design/s1g.1.2_two_levels.md) for which are Ein rules outright |
| | PFL.3, relation algebra, and its TODO (stdlib rules for the properties; *are rules able to detect properties from facts?*) | [S1g.4.2](p1g.4_first_order_logic/s1g.4.2_relation_algebra.md) |
| | PFL.4, the fragment Ein cannot decide | [S1g.4.5](p1g.4_first_order_logic/s1g.4.5_the_boundary.md) |
| [F26](../followups/f26_consistency_and_completeness.md), consistency and completeness | promotion 1, the glossary of senses; 2, the decidability table; 4, the chase reading of obligations | [S1g.4.5](p1g.4_first_order_logic/s1g.4.5_the_boundary.md) |
| | promotion 3, the engine's word for *did not stop* | [S1g.6.1](p1g.6_engine_requirements/s1g.6.1_a_saturation_that_does_not_stop.md) |
| | § Does Gödel's theorem say anything about an Ein program, and promotion 5, a self-referential program | [S1g.5.4](p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md) |
| | promotion 6, the common-sense table as benchmark classes | stays F26's, for M2; S1g.4.5 links it |
| [F5](../followups/f5_rules_as_data.md), rules as data | rules as values a rule can read: what a proof system written in Ein would need to talk about Ein's own rules | [S1g.6.5](p1g.6_engine_requirements/s1g.6.5_rules_as_data_and_refutations.md), as a requirement to state and price; F5's self-modification stays F5's |
| [F7](../followups/f7_rule_induction.md), rule taxonomy and induction | the 4-class taxonomy as a column of the fragment table; B′, properties detected from facts, as F1b's TODO | [S1g.4.1](p1g.4_first_order_logic/s1g.4.1_the_fragment.md), [S1g.4.2](p1g.4_first_order_logic/s1g.4.2_relation_algebra.md); induction of rules stays F7's |
| [F15](../followups/f15_math_formulae/ideas.md), rules exported as mathematical formulas | the three forms of a rule, logical, relational and algebraic, and the lowering between them | [S1g.4.2](p1g.4_first_order_logic/s1g.4.2_relation_algebra.md) writes them by hand for the stdlib; [S1g.6.3](p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md) is the exporter |
| [F17](../followups/f17_formal_verification/ideas.md), formal verification | its Ein section's reading of Ein as a theory layer before a prover, and rewriting logic (K) as the nearest relative | [S1g.4.1](p1g.4_first_order_logic/s1g.4.1_the_fragment.md)'s comparison column; nothing else is taken |
| [F20](../followups/f20_rule_kinds_as_sentence_moods.md), rule kinds as sentence moods | assertion, restriction and obligation as three logical forms: a Horn clause, an integrity constraint, an existential | [S1g.4.1](p1g.4_first_order_logic/s1g.4.1_the_fragment.md) |
| [F21](../followups/f21_non_monotone_programs_decided.md), non-monotone programs | negation as failure against classical negation; a cycle through `absent` as a program with no stable reading | [S1g.4.1](p1g.4_first_order_logic/s1g.4.1_the_fragment.md) cites it; M1g decides none of F21's decisions and writes its own theories monotone |
| [F8](../followups/f8_FCA_RCA_odis_tptp/ideas.md), FCA, RCA, TPTP | TPTP as the place first-order problems come from | [S1g.4.5](p1g.4_first_order_logic/s1g.4.5_the_boundary.md): which TPTP classes the fragment could state at all; running them is [M10](../m10_external_benchmarks/README.md)'s |
| [F12](../followups/f12_rules_and_relations/ideas.md), rules and relations | *a rule's match defines a relation, and its assert links two*: the reading of a rule as an inclusion between relations | [S1g.4.2](p1g.4_first_order_logic/s1g.4.2_relation_algebra.md) |
| [F1](../followups/f1_categorical_formulation.md), [idea 07](../ideas/07-categorical-formulation.md), categorical formulation | relation algebra as the shared ground between the two readings | linked from S1g.4.2; **stays parked** |
| [Idea 06](../ideas/06-inference-rules-completeness.md), completeness of inference rules | its four senses of *enough*, and its row *hypothesis + contradiction: assume A, derive ⊥, so not A* | [S1g.3.3](p1g.3_proof_systems/s1g.3.3_natural_deduction.md) (the row), [S1g.3.4](p1g.3_proof_systems/s1g.3.4_soundness_and_completeness.md) (completeness of a rule set, in the one case where it has a definition) |
| [Idea 09](../ideas/09-puzzles-beyond-zebra.md), puzzles beyond Zebra | knights and knaves as formulas; the liar | [S1g.2.4](p1g.2_propositional_logic/s1g.2.4_applications_and_scale.md); the liar in [S1g.5.4](p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md) |
| [Idea 02](../ideas/02-graph-as-formal-substrate.md), the graph as a formal substrate | a formula and a proof as graphs the views can draw | [S1g.6.3](p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md) |
| [Idea 03](../ideas/03-three-task-classes.md), three task classes | solve, gaps, contradictions read as valid or satisfiable, contingent, unsatisfiable | [S1g.2.2](p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md) |

## Stage, phase, milestone

The instruction's last question, and how this plan answered it, under the schema of
[`plans/README.md`](../README.md):

- **A stage** is one question with one deliverable that a subagent can finish and a test can hold: binary
  arithmetic is the calculator's circuits over other tables, so it is a stage of the calculator's phase, and
  so is binary logic, which rebuilds the calculator's own gate.
- **A phase** is a subject with its own acceptance and its own stop for the user: propositional logic, proof
  systems, first-order logic and Peano arithmetic each have a design that could be ruled differently without
  touching the others.
- **A milestone** is a body of work with one reason to exist and a dependency of its own. Logic is not the
  calculator's subject and not M1f's, it waits for all of M1f, and it promotes a followup that asked for a
  milestone. So it is M1g. A second milestone for the engine work would have separated each requirement from
  the program that shows why it is needed; the user ruled it out, and P1g.6 keeps them together.

## Acceptance

- **P1g.1's page was read and ruled on by the user** before any file outside `plans/` and `build/` was written.
- **Every question of the instruction has a page that answers it with programs,** each program's answer
  computed by the binary and not written by hand, and each number carrying the command that re-takes it, its
  date, or the test that pins it.
- **Search-free where it should be, held by a test:** evaluation of a formula under a given valuation, proof
  checking, normal forms, and Peano's recursion each have no candidate, k_s 1 and R {∅}, as the calculator's
  programs do (`ein-cli/tests/arith_search_free.rs` is the model).
- **Right, held against an independent check written in Rust:** every truth value, every tautology verdict,
  every normal form and every checked proof agrees with a check that shares no code with the Ein theory, on
  every formula up to a stated bound.
- **Each proof system is sound and complete up to a bound,** against P1g.2's truth tables, or the page names
  the formula on which it is not and why.
- **The fragment table exists** (F1b's PFL.1), with a program per row, and the boundary page (PFL.4) names
  what Ein cannot decide, with a program that shows each limit.
- **What a hypothesis is, in each setting, is stated on one page,** each statement with its program.
- **Every engine requirement P1g.1 listed is built, ruled or owned.** A change to the engine follows the
  repo's discipline: a wrong answer is a register row in `corpus/wrong_answers.toml` before it is fixed; a
  change to the grammar waits for the user's ruling; config never changes the answer; nothing adds engine
  arithmetic.
- **What ships passes the gates it joins:** a stdlib module has a program per rule under `tests/stdlib/`, an
  `:expect` in each, and the mutation sweep; every relation and rule carries a `:why`; every new `.ein` file
  has a corpus entry.
- **F1b is closed as promoted,** and each row of [§ What it pulls in](#what-it-pulls-in) is done or says why
  not.

## Risks, each with what checks it

- **The object level is quadratic by nature.** Rules about formulas join facts on nested values, which the
  fact index does not key on (E-3 of the calculator's
  [`findings.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)). The calculator avoided
  it by a rule of the theory; a logic's natural rules do not. Every stage measures its batch cost on the
  matcher's counters, and S1g.6.2 is where the index is taken up.
- **2ⁿ.** A truth table by hypotheses is R of 2ⁿ states, and satisfiability by region is a search with no
  clause learning beyond minimal dead sets. S1g.2.4 measures where it ends and the page says it without
  apology: it is the semantics made visible, not a solver.
- **Forward saturation floods on schemata.** A Hilbert system closed forward over even a small universe of
  formulas derives every theorem in it. S1g.3.2 measures it and falls back on requests.
- **A saturation that does not stop** is one rule away in every phase past P1g.2 (probe 3), and the engine
  has no word for it. Until S1g.6.1, every theory is written demand-driven, as the calculator is, and each
  stage's test runs under a timeout that fails loudly.
- **The two levels blur.** A page that says *Ein proves* when a theory written in Ein derived a fact about a
  formula misleads. S1g.1.2 fixes the vocabulary, and every later page uses it.
- **Reductio is assumed and not tested.** The mapping from an assumption to a commitment, and from a
  contradiction under it to a dead state and its core, is the plan's most attractive claim; probe 4 already
  shows its limit. S1g.3.3 tests it and stops for the user.
- **Scope.** Each of sequent calculus, resolution, tableaux, modal logic, intuitionistic semantics and
  Gödel numbering is a subject. The first three are one optional stage; the rest are named where they come up
  and not planned.

## Connections

- **[M2](../m2_nl_to_ir/README.md):** a formula language with checked semantics is a *formal target* a
  formalizer can be measured against, with ground truth no puzzle has: validity is decidable by table. And
  [§ The reasoning as a program](../m2_nl_to_ir/README.md#the-reasoning-as-a-program-the-users-idea-2026-11-11)
  (the user's idea of 2026-11-11) is, at this milestone's object level, literal: a proof written as an Ein
  program that the kernel checks line by line (probe 2).
- **[M10](../m10_external_benchmarks/README.md):** the same problems for Z3, CVC5, Prolog, Clingo and Lean.
  P1g.2's satisfiability programs and P1g.4's fragment say which TPTP and SAT classes Ein can state; M10 runs
  them.
- **[M1d](../../docs/history/m1d_satisfiability/README.md):** *from saturation to satisfiability* is this
  milestone's distinction between derivation and model, drawn for puzzles.
- **[P1f.6](../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md):** the request protocol, the
  search-free test, the harness against Rust's integers, and
  [S1f.6.10](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.10_beyond_arithmetic.md)'s bounded
  theorem checking, which P1g.5 continues.
- **[`docs/lib/03`](../../docs/lib/03-theorem-proving-formal-methods.md)** and
  **[`docs/lib/02`](../../docs/lib/02-solvers-csp-sat-smt.md):** the external systems each phase compares
  itself with.
- **[`open_questions.md`](open_questions.md):** Q-M1g.1 to .12.
