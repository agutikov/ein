# S1g.4.1 — The fragment: Ein's rule language as first-order logic, a table with a program per row, and the BNF

**Phase:** [P1g.4](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md), above all
[S1g.1.2](../p1g.1_survey_and_design/s1g.1.2_two_levels.md)'s meta-level tables.
**Pulls in:** [F1b](../../followups/f1b_logical_formulation.md)'s PFL.1;
[F20](../../followups/f20_rule_kinds_as_sentence_moods.md)'s three rule kinds;
[F7](../../followups/f7_rule_induction.md)'s four-class taxonomy;
[F21](../../followups/f21_non_monotone_programs_decided.md) for `absent`;
[F17](../../followups/f17_formal_verification/ideas.md)'s comparison with rewriting logic.

## What it asks

F1b: *"which of ∀, ∃, ¬, ∧, ∨, →, ↔, =, ≠ are native; which are macros; which are uninhabited. State the
answer as a one-line BNF over the kernel forms."* And the instruction: *"First order logic, what we can
implement already and how"*. The stage answers by a table in which every row is a first-order construct, the
Ein form that is it or stands in for it, a program, and what the binary did.

## The table to fill

The **Ein** column is the coordinator's reading of the kernel pages
([`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md),
[`02_patterns.md`](../../../docs/kernel/ir/03-ein-lang/02_patterns.md),
[`defined_behaviour.md` § 4](../../../docs/kernel/defined_behaviour.md)); each row is **to be confirmed by
its program**.

| construct | where | Ein | status expected |
|---|---|---|---|
| ∀ | a rule's variables | every variable of a `:match` is universally quantified over values; the rule is ∀x̄ (body → head) | native |
| ∀ | inside a body | `(forall ?b G B)`, a macro for `(absent (and G (absent B)))`: guarded, and true by failure, so sound where G's and B's relations are closed | macro; closed-world |
| ∃ | inside a body | a variable that occurs only in the `:match` | native |
| ∃ | in a conclusion | not written. Either a **Skolem term**, a nested fact over the rule's variables, `(parent (father-of ?x) ?x)` (probe 5); or an **obligation**, `(open ?R)`, which builds no witness and makes the state owe one, paid by the search from the program's names | two stand-ins, different in kind |
| ∧ | body, head | `(and …)` in a `:match`; `(and …)` in an `:assert` | native |
| ∨ | body | `(or …)` in a `:match`; W39, open and F22's, is a known wrong answer on disjuncts that bind the same registers | native, with a register row |
| ∨ | head | not written. Each disjunct a candidate and an obligation that one holds: the lattice | by search only |
| → | the rule | `:match` to `:assert` | native, one level; an implication as a premise or a conclusion is not a form |
| ↔ | | two rules | derived |
| ¬ | stored | `(not P)`: a fact of its own, derived or written, matched as a pattern; `P` and `(not P)` together are a contradiction by the kernel's pair check | native: explicit negation |
| ¬ | by failure | `(absent P)`: a premise's test of the state at the closure boundary; refused where a refutation would rest on one a commitment can fill ([§ 4.7](../../../docs/kernel/defined_behaviour.md)) | native: non-monotone |
| ⊥ | head | `(false)` | native: an integrity constraint |
| =, ≠ | body, head | `eq`, `neq`: identity of values, computed, never stored; a false one concluded is `(false)` | native; no equational theory |
| function symbols | terms | a nested fact of an ordinary relation, under the value rule ([§ 4.22](../../../docs/kernel/defined_behaviour.md)): a free constructor | native since nested facts; uninterpreted |
| quantifying over relations | | a rule's relation parameter, bound by an activator fact to a relation's *name* | first-order over names |
| the closed world | | `(__closed__ R)`, and a claim that names a relation | per relation |
| a third truth value | | `(unknown P)`: neither `P` nor `(not P)` | macro |

Beside it, three more columns the stage adds:

- **F20's kind:** which rows are *assertions* (a Horn clause's head is a fact), *restrictions* (the head is
  `(false)` or a `not`), *obligations* (the head is `open`), and what each is called in logic: a definite
  clause, an integrity constraint or denial, an existential (tuple-generating) dependency.
- **F7's class,** for the rows that involve a relation parameter.
- **Compared with:** Datalog (no function symbols, no existentials), Datalog with stratified negation, Horn
  clauses with function symbols (Prolog's pure fragment, forward), existential rules and the chase, answer
  set programming (disjunction and negation as failure, stable models), rewriting logic (F17's K). One line
  a row saying *the same as*, *like, except*, or *not established*.

## What the stage has to establish beyond the table

1. **The BNF,** F1b's one line: the fragment's sentences generated from the kernel's forms, read off
   `00_ebnf.md`'s generated grammar and not written from memory.
2. **The semantics the table assumes,** in three sentences with their sources: a state is the least fixpoint
   of the rules over the commitment
   ([`solution_semantics.md` § 1.5](../../../docs/kernel/inference/solution_semantics.md)); the answer is
   the alive and discharged commitments; `absent` is read at the boundary. For a monotone program with no
   obligation, the coordinator expects *a state is the least Herbrand model of root plus the commitment*,
   and R's members are the consistent ones: **to be checked against the definitions, and against the oracle
   on a handful of programs.**
3. **Translation, one way.** A small set of first-order sentences in clausal form, each translated by hand
   into Ein under the table, run, and the answer compared with what the sentence's models are over the
   program's names: a Horn theory, one with a denial, one with an existential, one with a disjunctive head,
   one that is not Horn and has no translation. This is the instruction's *"what we can implement already
   and how"*, as five programs.
4. **Where Ein is not classical first-order logic at all:** no excluded middle on facts (S1g.1.2); a dead
   state's content; unique names; the universe H scoped by the program (`outside`); and that *entailment*
   has two readings in Ein, derived at root, or true in every state of R, which differ exactly when a
   disjunction or an existential is in play.

## Tasks

1. **T1g.4.1.1 — a program per row,** under the examples' logic directory, each with a claim; the table
   filled from their runs.
2. **T1g.4.1.2 — the BNF and the semantics paragraph,** each claim with its source or its test.
3. **T1g.4.1.3 — the five translations,** with their answers and the oracle's.
4. **T1g.4.1.4 — the page,** a current kernel page (Q-M1g.12 for where), in the kernel's voice, with the
   plan's history left here.

## Acceptance

- Every row of the table has a program whose claim `ein test` holds, and no row's status is written from
  expectation.
- The BNF is derived from the generated grammar, and a test or a `--check` holds the page to it if that is
  cheap; if not, the page carries its date.
- Each *compared with* cell either cites the definition compared with or says *not established*.
- F1b's PFL.1 is marked answered in its file's banner.

## What it may find

- **Rows the table lacks:** constructs of Ein with no first-order reading (a rule's `:why`; an activator as
  a second-order instantiation; `(solve …)`'s scoping), listed as such.
- **That two of Ein's negations interact in a way no row describes.** F21 owns non-monotone programs; the
  finding goes there with its program.
- **A wrong answer,** on `or` above all (W39's family). A register row first.
