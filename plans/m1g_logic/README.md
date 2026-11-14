# M1g — Logic in Ein: from truth tables to proof systems, first-order logic and Peano arithmetic

**Status:** planned 2026-11-11, on the user's instruction of that day ([§ The instruction](#the-instruction)).
Not started. It carries no estimate (the user's ruling of 2026-11-08).
**Extended 2026-11-12** by the user's replies to the calculator's questions: two phases, P1g.7 and P1g.8, and
one stage of P1g.6, S1g.6.6, and, on the user's reply in round 2, a third phase, P1g.9 ([§ Additions of 2026-11-12](#additions-of-2026-11-12)).
**Extended 2026-11-13** on the user's instruction of that day: M1f's P1f.8, school algebra and geometry, split
into two phases and moved here as P1g.10 and P1g.11, and one more stage of P1g.6, S1g.6.7
([§ Additions of 2026-11-13](#additions-of-2026-11-13)).
**Depends on:** [M1f](../m1f_hypothesis_and_documentation/README.md), complete (the user, 2026-11-11: *"first
we will complete M1f"*). Specifically:
[S1f.6.16](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.16_binary_arithmetic.md), binary
arithmetic, and [S1f.6.17](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md),
gates and truth tables, which are this milestone's starting point and stay stages of P1f.6; `std.math.decimal-arith`, the
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
  included: no second milestone was made for that engine work, which is a phase of its own,
  [P1g.6](p1g.6_engine_requirements/README.md). (The id M1h, which the proposal of 2026-11-11 would have given
  such an engine milestone and the user declined, has been taken since for another subject: on 2026-11-12 the
  user created [M1h, *further work beyond word problems*](../m1h_beyond_word_problems/README.md). It is not an
  engine milestone for the logic, and the ruling above stands. Where M1h sits against this milestone in the
  order is not ruled; it is that README's open question.)
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
| [P1g.5](p1g.5_peano_and_induction/README.md) | Natural numbers: Peano's axioms, recursion, induction, and Gödel made concrete | 4 | unary numbers and the axioms as rules; addition and multiplication by recursion, held to `std.math.decimal-arith`; induction as a schema against bounded checking; and what of F26's Gödel section a program can show |
| [P1g.6](p1g.6_engine_requirements/README.md) | The engine requirements the logic turns out to have | 7, provisional (one added 2026-11-13) | each requirement P1g.1 lists, scheduled, ruled or given an owner; seeded today with the candidates already known |
| [P1g.7](p1g.7_unsatisfiable_against_owing/README.md) | Unsatisfiable against owing: the open state's explanation, and what a finished search may call an unpayable debt (added 2026-11-12) | 4, in the README | the witnesses on one page; the open state's core; the user's ruling on whether a finished search may call an unpayable debt dead; R unmoved on every program either way |
| [P1g.8](p1g.8_a_theory_of_types/README.md) | A theory of types as a stdlib module of its own, and a smaller kind of number where a larger is expected (added 2026-11-12) | 4, in the README | the design on programs, **stopping for the user**; the module; promotion between the calculator's number kinds as its first use, in a module the core does not import |
| [P1g.9](p1g.9_one_equation_three_methods/README.md) | One equation, three methods: arithmetic read backwards, search, and algebra (added 2026-11-12) | 5, in the README | `x × 7 = 862` and its neighbours by each method as kept programs; what *no solution* is under each, side by side; the cost of each; the question *arithmetic or algebra* answered on the programs; inverse requests proposed or declined |
| [P1g.10](p1g.10_school_algebra/README.md) | School algebra: expressions, equations, and the derivation a pupil writes (moved here 2026-11-13; the algebra half of M1f's P1f.8) | 7 | expressions as data over the calculator's numbers, evaluated and brought to a polynomial normal form by saturation alone; identities proved by normal forms; a linear equation solved as the derivation a pupil writes, beside the same equation by search; systems and quadratics with a rational root; a program that states false facts; compound numbers without a hardcoded representation. Its first stage **stops for the user** |
| [P1g.11](p1g.11_school_geometry/README.md) | School geometry: a figure as facts, theorems as rules, and the proofs a pupil writes (moved here 2026-11-13; the geometry half of M1f's P1f.8) | 4 | a figure written as facts and the literature read honestly; a rule base of theorems saturated over a figure, the trace a two-column proof; constructions, proof by contradiction and case splits as three uses of a hypothesis, each with its measured limit; the facts a pupil reads off the picture detected as gaps; and the report on which fragment of school mathematics is a formal derivation in Ein. Its first stage **stops for the user**; what it needs of the engine is found on its programs and listed as candidates in S1g.6.7 |

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

7. **P1g.7** is read with P1g.2: [S1g.2.2](p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md)
   reads *unsatisfiable* off R, and P1g.7's first stage banks the witnesses that stage meets. Its ruling,
   S1g.7.3, may come at any time after, since R is the same set whichever way it goes.
8. **P1g.8** is independent of the logic phases and may run beside them after P1g.1's stop; its own first
   stage stops for the user.
9. **P1g.9** needs only P1g.1's stop and P1g.10's linear equations
   ([S1g.10.4](p1g.10_school_algebra/s1g.10.4_linear_equations.md)); it is read with P1g.7 and may run
   before that phase's ruling, since it shows the two readings and decides neither.
10. **P1g.10** after P1g.1's stop, beside P1g.2, of which it needs nothing: it imports the calculator, and its
    own first stage stops for the user. It is placed **before P1g.3** by the user's words of 2026-11-11, when
    it was planned, *"between calculator and proof systems"*; no stage of P1g.2 to P1g.5 waits for it. What
    waits for it: P1g.9, for S1g.10.4; the equational proof P1g.3 takes from
    [S1g.10.3](p1g.10_school_algebra/s1g.10.3_simplification_and_identities.md)'s normal forms, if that is
    written; and [P1g.8](p1g.8_a_theory_of_types/README.md) and [M12](../m12_rules_as_data/README.md), which
    read [S1g.10.7](p1g.10_school_algebra/s1g.10.7_compound_numbers_without_a_hardcoded_representation.md).
11. **P1g.11** from [S1g.10.1](p1g.10_school_algebra/s1g.10.1_survey_and_design.md)'s stop on, whose rulings
    on what the two phases share its first stage follows; its first three stages run beside the algebra, and
    its last, S1g.11.4, after S1g.10.4, since a figure's problem with an unknown is that stage's equation and
    its report reads both phases. Before P1g.3 for the same reason as P1g.10, and it meets that phase twice:
    [S1g.11.2](p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md)'s proofs are derivations found, where
    P1g.3's are proofs checked, and S1g.11.3 and S1g.3.3 ask
    [Q-M1g.5](open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact) once, whichever runs first. It meets
    **P1g.6** wherever its programs show the language short:
    [S1g.6.7](p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md) lists the candidates, each
    P1g.6 stage that owns one runs when the geometry's programs have shown it and the user has ruled, and
    the geometry takes its named detour until then.

12. **The main goal's decision** ([§ The main goal](#the-main-goal-proof-system-theory-and-what-a-theorem-is-2026-11-14))
    last, on the programs of P1g.3, P1g.5 and P1g.11; its phase is planned when P1g.3's first stage has run.

The order of items 10 and 11 was chosen in the split of 2026-11-13 from the dependencies and is not the
user's ruling ([P1g.10 § Choices made in the split](p1g.10_school_algebra/README.md#choices-made-in-the-split)).

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

## Additions of 2026-11-12

The user replied to the calculator's questions on 2026-11-12
([`questions.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md); what stayed open is
[`questions_round2.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md); the index
of every reply is
[`replies_2026-11-12.md`](../m1f_hypothesis_and_documentation/p1f.6_calculator/replies_2026-11-12.md)). Three
replies send work here.

**A types theory, and promotion of a smaller number kind** (reply 12):

> question about type system inside the theory; lets postpone it till M1g, types theory deserves it's own
> separate theory file in stdlib, and then we will try to use it to solve this; keep as is now

[P1g.8](p1g.8_a_theory_of_types/README.md): the types theory as a stdlib module of its own, and the question
it was postponed from, a natural number where an integer is expected and an integer where a rational is, as
its first use.

**Unsatisfiable against owing** (replies 24 and 28): *"this question open vs. false is deep. […] It looks
like an entire separate phase."* [P1g.7](p1g.7_unsatisfiable_against_owing/README.md): the open state's
core, the read-out row for N, and whether a finished search may call an unpayable debt dead. Probe 1b of
[§ First probes](#first-probes-2026-11-11) met it on the day this milestone was planned. R is the same set
either way, which is why it could wait and why it can be decided here.

**A cheaper search where each object takes exactly one value** (reply 25, and round 2's item 25: *both
hold*): [S1g.6.6](p1g.6_engine_requirements/s1g.6.6_exactly_one_value_per_object.md), candidate C-12 of
P1g.6, with the table of question 25 as its witness.

**One equation, three methods** (replies 24 and 37; the user's reply in round 2, `dc88520b`): *"This
Arithmetic read backwards vs Search vs Algebra comparison is somethinig like number theory and goes into
separate phase in M1g. p1f.8 remain incomplete naive school algebra."*
[P1g.9](p1g.9_one_equation_three_methods/README.md). (The phase the quote names, M1f's P1f.8, is this
milestone's [P1g.10](p1g.10_school_algebra/README.md) since 2026-11-13.)

**Findings the calculator's phase gave this milestone** (section J of `questions.md`, each line left empty
and so taking its recommendation): E-3, E-12, E-15, E-24, E-25 and E-26 to
[S1g.6.2](p1g.6_engine_requirements/s1g.6.2_nested_facts_at_scale.md); E-9, E-10 and E-13 to
[S1g.6.3](p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md). Whether a number may print as a number
(question 55, E-14) was left empty and has no default: S1g.6.3's first task, which already stops for the
user, is where it is asked again.

**Where the starting point lives.** S1f.6.16 and S1f.6.17 ship as `std.math.binary-arith` and, by the
user's reply to the layout question (`dc88520b`), **`std.bool.*`**, not `std.logic.*`
([S1f.6.31](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md#the-reply)).
`std.logic` is free, and [Q-M1g.2](open_questions.md#q-m1g2--where-the-logic-ships) is asked knowing it.

**[M1h](../m1h_beyond_word_problems/README.md)** was created the same day for the work beyond word
problems. Two of its early phases touch this milestone's subject from the side of natural-language
problems, slicing and *gaps, assumptions and readings*, and P1g.7's ruling is one both will read.

## Additions of 2026-11-13

The user, 2026-11-13:

> plans/m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry seems too complex having both algebra
> and geometry, let's split it into 2 phases and move into m1g;
> plans/m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/s1f.8.7_theorems_as_rules.md  probably
> will require new features in the engine

**Two phases moved in.** M1f's P1f.8, *middle-school algebra and geometry*, planned 2026-11-11 *"between
calculator and proof systems"* and until now a phase of M1f that ran before this milestone, is two phases
here: [P1g.10](p1g.10_school_algebra/README.md), school algebra, seven stages, and
[P1g.11](p1g.11_school_geometry/README.md), school geometry, four. The files were moved with `git mv` and
nothing of the content was redesigned. The table of old and new ids, and the choices the split forced, are in
[P1g.10 § The move](p1g.10_school_algebra/README.md#the-move-of-2026-11-13); the old path keeps a
[stub](../m1f_hypothesis_and_documentation/p1f.8_school_algebra_and_geometry/README.md). M1f's phase number 8
is used and vacated.

**What it changes for this milestone.** The title says *logic*, and the two phases are school mathematics;
they are here because the user put them here, and they fit where the instruction of 2026-11-11 put them,
between the calculator and the proof systems: a pupil's derivation and a pupil's proof are the material
P1g.3 formalises. The milestone's dependency on M1f is unchanged. The handovers that were *from P1f.8 to
M1g* are now between sibling phases, and each phase's README lists its own
([P1g.10](p1g.10_school_algebra/README.md#what-is-left-to-m1gs-other-phases),
[P1g.11](p1g.11_school_geometry/README.md#what-is-left-to-m1gs-other-phases)). Their questions, which were
to be filed as `Q-M1f.<n>`, will be filed in [`open_questions.md`](open_questions.md) as `Q-M1g.<n>`; none
had been filed.

**One stage of P1g.6,** on the instruction's second sentence:
[S1g.6.7](p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md), *what theorems as rules needs
of the engine*. It lists ten candidates, each a question with the program that would show it, all taken from
what the geometry's stage files already say: a point a construction makes, figure names that do not depend
on the order of their points, a proof read off a run and which justification it shows, a nested value read
as words, equality classes of segments and angles, a saturation that does not stop, a conclusion that is a
state's, a hypothesis about a nested value, joins on nested values, and a check that a reading over the
maximal line is sound. Six are P1g.6's existing candidates met on a figure. **It designs nothing:** what is
needed is found by S1g.11.1's and S1g.11.2's computed programs first, and an engine or grammar change needs
the user's ruling on them.

## The main goal: proof system, theory, and what a theorem is (2026-11-14)

The user, 2026-11-14:

> M1g - important notes: one of the main goals of the whole milestone would be to decide where is in Ein a
> Proof System (with formulas and proofs) and where is Theory (with axioms and theorems), what are they,
> especially finally what is theorem, how it can be searched, verified/proved, saved and reused. So it would
> became possible to build new therems and use them to prove other theorems. The way they currently named
> rules could be logic rules, e.g. like 12 natural deduction rules. But there is an idea of theorems as ein
> rules, but it is not clear how it could even work this way.

So the milestone does not end with its phases' pages alone: it ends with a **decision**, made on the programs
they leave, of five things ([Q-M1g.16](open_questions.md#q-m1g16--proof-system-theory-and-what-a-theorem-is)):

1. **Where the proof system is:** formulas and proofs, and the rules of inference over them. P1g.3 builds it
   at the object level: a formula a nested fact, a proof facts a saturation checks (probe 2), natural
   deduction's rules ([S1g.3.3](p1g.3_proof_systems/s1g.3.3_natural_deduction.md), whose table of every
   connective's introduction and elimination is the user's *"12 natural deduction rules"*). Ein's own rules can
   *be* the logic's rules, the forward ones outright; the rules that discharge an assumption are the part no
   Ein rule states today.
2. **Where the theory is:** axioms and theorems, as distinct from the rules of inference. Axioms are facts or
   schemas ([S1g.3.1](p1g.3_proof_systems/s1g.3.1_hilbert_systems.md)); Peano's are
   [P1g.5](p1g.5_peano_and_induction/README.md)'s; a school figure's are
   [P1g.11](p1g.11_school_geometry/README.md)'s.
3. **What a theorem is,** at the two levels this milestone keeps apart ([§ What this is](#what-this-is)): a
   fact about a formula, `(theorem F)` with its proof, at the object level; or a rule of Ein, a derived rule
   applied forward, at the meta level; or both, one made from the other. **The stdlib already has the second
   kind, unproved:** `std.algebra`'s `converse-pair-symmetric` is a theorem written as a rule, and M1f's
   [E-53](../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md) is what one costs when nothing
   proves it: it does not compose with the converse type checks, and a well-typed pair is refuted.
4. **How a theorem is searched, verified and proved:** proof search
   ([S1g.3.2](p1g.3_proof_systems/s1g.3.2_finding_a_proof.md)), proof checking by saturation (S1g.3.1), and,
   for a theorem written as a rule, a proof that the rule is admissible, which nothing in the plan does yet.
5. **How a theorem is saved and reused,** so that new theorems are proved with old ones: a module of proved
   theorems, imported as any module is (M1f's [S1f.6.35](../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.35_imports_by_full_name.md)
   makes an import one copy by full name); and **the bridge** the user names as unclear, *"theorems as ein
   rules, but it is not clear how it could even work"*: a proved object-level theorem with schematic letters
   turned into an Ein rule. That turn reads a formula as a rule, which is rules as data
   ([S1g.6.5](p1g.6_engine_requirements/s1g.6.5_rules_as_data_and_refutations.md),
   [M12](../m12_rules_as_data/README.md)), or a generator that writes the rule's text; and the geometry meets
   the same question from the other side, theorems used as rules
   ([S1g.11.2](p1g.11_school_geometry/s1g.11.2_theorems_as_rules.md),
   [S1g.6.7](p1g.6_engine_requirements/s1g.6.7_what_theorems_as_rules_needs.md)).

**Where it is decided.** Last, after P1g.3, P1g.5 and P1g.11 have run, on their programs; each of those phases
records what it shows of the five, and the phase that writes the decision is planned when P1g.3's first stage has
run (proposed: P1g.12, *theorems proved, saved and reused*). Nothing here changes an earlier phase's tasks.

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
- **P1g.10 and P1g.11 meet their own READMEs' acceptance** (moved here 2026-11-13 with it): each stops for
  the user first, and the report on which fragment of school mathematics is a formal derivation in Ein is
  written by S1g.11.4 over both.
- **Every engine requirement P1g.1 listed is built, ruled or owned,** and every candidate of S1g.6.7 is
  answered by a program of P1g.11: shown needed, shown not needed, or not reached. A change to the engine follows the
  repo's discipline: a wrong answer is a register row in `corpus/wrong_answers.toml` before it is fixed; a
  change to the grammar waits for the user's ruling; config never changes the answer; nothing adds engine
  arithmetic.
- **What ships passes the gates it joins:** a stdlib module has a program per rule under `tests/stdlib/`, an
  `:expect` in each, and the mutation sweep; every relation and rule carries a `:why`; every new `.ein` file
  has a corpus entry.
- **F1b is closed as promoted,** and each row of [§ What it pulls in](#what-it-pulls-in) is done or says why
  not.
- **The main goal is decided** ([§ The main goal](#the-main-goal-proof-system-theory-and-what-a-theorem-is-2026-11-14)):
  a page says where in Ein the proof system and the theory are and what a theorem is, and shows on programs one
  theorem searched, proved and checked, saved, and reused to prove another, or names exactly what stops each step;
  the user rules on it.

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
- **School geometry may need the engine to change** (the user, 2026-11-13, of S1g.11.2: *"probably will
  require new features in the engine"*). The plan assumes the language as it ships and names a detour at each
  place it expects to be short. S1g.6.7 writes the candidates down before the programs, S1g.11.1's survey and
  S1g.11.2's targets are the programs, and nothing is designed before they have run and the user has ruled.
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
- **[P1g.10](p1g.10_school_algebra/README.md) and [P1g.11](p1g.11_school_geometry/README.md)** (planned
  2026-11-11 as M1f's P1f.8, the user: *"between calculator and proof systems"*; split and moved here on
  2026-11-13): school algebra and school geometry, which until the move ran before this milestone as part of
  M1f and are now two of its phases. They hand over to the logic phases, by name
  ([P1g.10's § What is left to M1g's other phases](p1g.10_school_algebra/README.md#what-is-left-to-m1gs-other-phases),
  [P1g.11's](p1g.11_school_geometry/README.md#what-is-left-to-m1gs-other-phases)):
  equational proof, line by line, to P1g.3, with its polynomial normal form as the semantics to check against;
  school proofs by contradiction and by cases as non-logical material for S1g.3.3; auxiliary constructions as
  the first evidence for S1g.6.4's witnesses (C-8) from outside logic; and statements that need induction to
  P1g.5. **No stage of P1g.2 to P1g.5 depends on them:** the logic's programs are written without their
  theories, and Q-M1g.5 is asked once, by whichever of S1g.3.3 and S1g.11.3 runs first. P1g.9 does depend on
  S1g.10.4.
- **[`docs/lib/03`](../../docs/lib/03-theorem-proving-formal-methods.md)** and
  **[`docs/lib/02`](../../docs/lib/02-solvers-csp-sat-smt.md):** the external systems each phase compares
  itself with.
- **[M1h](../m1h_beyond_word_problems/README.md)** (planned 2026-11-12): temporal and causal theories,
  slicing, gaps and assumptions, and three applications. Its place in the order against this milestone is
  not ruled.
- **[`open_questions.md`](open_questions.md):** Q-M1g.1 to .15.
