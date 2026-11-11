# S1g.1.2 — Two levels: Ein's rules as rules of inference, a logic written in Ein, and what a hypothesis is in each

**Phase:** [P1g.1](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.1.1](s1g.1.1_what_ein_represents_today.md), whose programs are its examples.
**Pulls in:** [F1b](../../followups/f1b_logical_formulation.md)'s PFL.2 in outline (which rules of inference
are *direct* in Ein and which need machinery); [idea 06](../../ideas/06-inference-rules-completeness.md)'s
row *hypothesis + contradiction*; [F20](../../followups/f20_rule_kinds_as_sentence_moods.md)'s three rule
kinds; [F26](../../followups/f26_consistency_and_completeness.md)'s table of what Ein means by its words.

## What it asks

Two of the instruction's questions have two answers each, and a page that gives one of them as *the* answer
will mislead:

> rules of inference? what are hypothesis in this case?

A rule of inference can be **an Ein rule** (the metalanguage's own step) or **a rule of the logic under
study**, written as an Ein rule about formula objects. A hypothesis can be **a candidate fact of Ein's
lattice** or **an assumption inside a proof**, and whether the second maps onto the first is something to
test. The stage sets out the vocabulary every later page of the milestone uses.

## The two levels, side by side

The coordinator's reading, each cell to be backed by one of S1g.1.1's programs or corrected:

| | **Ein as the logic** (the meta level) | **a logic written in Ein** (the object level) |
|---|---|---|
| a proposition | a ground fact, `(owns Ann zebra)` | a value, `(Imp p q)`, held inside a fact: `(holds (Imp p q))` |
| true | stored in the state | `(value-of F 1)` under the state's valuation, or `(holds F)`, or `(checked LINE F)`: the theory says which |
| false | `(not F)` stored; or, under a closed relation, absent | `(value-of F 0)`; `(holds (Not F))` |
| a rule of inference | `(rule …)`: premises matched, conclusion asserted, to a fixpoint | a fact about formulas derived by an Ein rule: `(holds (Imp ?a ?b))` and `(holds ?a)` give `(holds ?b)` |
| the variables of a rule | Ein's `?x`, ranging over values | Ein's `?a`, ranging over *formulas*: a schema's metavariables |
| a contradiction | `(false)`: the state is dead, with a core | the theory's choice: a rule that `(holds A)` and `(holds (Not A))` give `(false)` makes it Ein's too |
| an axiom | a fact the program writes | a formula the theory accepts: an instance of a schema, recognised by a rule |
| a proof | the derivation the engine recorded: `--trace`, a fact's justification | a value or a set of facts a program writes, which rules check |
| a hypothesis | a candidate of H, committed in some states of the lattice | an assumption of a derivation; or one letter's value |
| a model | a state of R | a valuation, which may itself be a state of R ([S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md)) |

## Which classical rules of inference are Ein rules outright

F1b's PFL.2 asked which rules of the propositional calculus are *"direct rule-graph rewrites in ein and which
need machinery the kernel doesn't have"*. At the meta level, over facts, the coordinator's expectation, **to
be tried, one program a row**:

| the rule | at the meta level |
|---|---|
| modus ponens | what firing a rule *is* |
| conjunction introduction and elimination | `(and …)` in an `:assert` and in a `:match` |
| hypothetical syllogism | two rules chained by saturation; never a derived *rule*, since rules are not data ([F5](../../followups/f5_rules_as_data.md)) |
| modus tollens | not automatic: from a rule `A` gives `B` and a stored `(not B)`, nothing derives `(not A)`. A second rule must say it, or the search refutes `A` as a hypothesis |
| disjunction introduction | no disjunctive fact exists to introduce |
| disjunction elimination, case analysis | the lattice: each disjunct a candidate, an obligation that one holds; what is common to every state of R is the conclusion, read by a query's projection classes |
| disjunctive syllogism | elimination: `std.elim`'s rules, with a closed domain |
| reductio | a dead commitment and its core; reported, not concluded (probe 4, [Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact)) |
| excluded middle | not a rule: a fact may be neither stored nor negated, the third state `(unknown P)` reads |

The last two rows are where Ein's own logic is visibly **not classical two-valued logic**, and the page says
so with a program for each.

## What a hypothesis is, setting by setting

The table the milestone's acceptance asks for. Each row names the stage that tests it.

| setting | the hypothesis is | R is | a dead node is | tested by |
|---|---|---|---|---|
| a puzzle (Zebra) | a candidate fact of the answer | the solutions | a combination the clues refute | the corpus |
| a circuit's truth table | an input's value | the rows of the table | nothing, unless an output is required | S1f.6.17 |
| a formula's semantics | a letter's value | the valuations, or those that satisfy a requirement | a valuation that falsifies it | [S1g.2.2](../p1g.2_propositional_logic/s1g.2.2_valuations_as_hypotheses.md) |
| a Hilbert derivation | nothing: the system has axioms and one rule; a premise is a fact at root | {∅} | none | [S1g.3.1](../p1g.3_proof_systems/s1g.3.1_hilbert_systems.md) |
| proof search | a guess at a cut formula or at the next line | the proofs found | a line that does not check | [S1g.3.2](../p1g.3_proof_systems/s1g.3.2_finding_a_proof.md), to be tried |
| natural deduction | an assumption, open until discharged | the consistent sets of assumptions | an assumption set that derives a contradiction: reductio's premise | [S1g.3.3](../p1g.3_proof_systems/s1g.3.3_natural_deduction.md) |
| a tableau | a branch's choice at a disjunction | the open branches: the countermodels | a closed branch | [S1g.3.5](../p1g.3_proof_systems/s1g.3.5_other_calculi.md), optional |
| a first-order model | a ground atom of the structure | the structures that satisfy the theory, over the names given | a structure that violates an axiom | [S1g.4.3](../p1g.4_first_order_logic/s1g.4.3_formulas_with_quantifiers.md) |
| induction | the induction hypothesis: a formula assumed for `n` | to be found out | to be found out | [S1g.5.3](../p1g.5_peano_and_induction/s1g.5.3_induction.md) |

## Tasks

1. **T1g.1.2.1 — the two-level table,** each cell with the program of S1g.1.1 that shows it, or a new one.
2. **T1g.1.2.2 — the classical rules,** one small program a row over plain facts (no formula objects), with
   its answer: derived, not derived, or read off R.
3. **T1g.1.2.3 — the hypothesis table,** the first three rows with their programs now; the later rows carry
   the expectation and the stage that will replace it.
4. **T1g.1.2.4 — the vocabulary.** A short list of words the milestone's pages use in one sense only:
   *derives* (saturation, either level), *holds* (the object theory's relation), *valid* (true in every state
   of R), *refuted* (dead), *proves* (only of an object-level proof that was checked). *Ein proves* is not
   written of a derivation.

## Acceptance

- The three tables, in `design.md`, each row backed by a program and its computed answer or marked *to be
  tried by* a named stage.
- The two rows where Ein's meta level is not classical each have a program a reader can run.
- The vocabulary list, which later stages cite.

## What it may find

- That the meta level is closer to some named logic than the coordinator knows how to say: intuitionistic in
  having no excluded middle, paraconsistent in what a dead state still holds (F26's first row), a logic
  program's in its negation as failure. S1g.4.1 names the fragment; this stage only reports what the
  programs show.
