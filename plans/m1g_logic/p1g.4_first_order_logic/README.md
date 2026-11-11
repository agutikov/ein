# P1g.4 — First-order logic: the fragment Ein is, relation algebra, and formulas with quantifiers as data

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s rulings. S1g.4.1 and S1g.4.2 need nothing
more; S1g.4.3 and S1g.4.4 extend [P1g.2](../p1g.2_propositional_logic/README.md)'s formulas and
[P1g.3](../p1g.3_proof_systems/README.md)'s checkers.
**Promotes:** [F1b](../../followups/f1b_logical_formulation.md). This phase is its four phases:

| F1b | here |
|---|---|
| PFL.1, *pin the fragment: which of ∀, ∃, ¬, ∧, ∨, →, ↔, =, ≠ are native; which are macros; which are uninhabited. State the answer as a one-line BNF over the kernel forms* | [S1g.4.1](s1g.4.1_the_fragment.md) |
| PFL.2, the propositional rules of inference | [P1g.3](../p1g.3_proof_systems/README.md)'s [S1g.3.4](../p1g.3_proof_systems/s1g.3.4_soundness_and_completeness.md) |
| PFL.3, *relation-algebra operations […] prove (or disprove) closure under composition for the rule set* | [S1g.4.2](s1g.4.2_relation_algebra.md) |
| PFL.4, *identify the first-order fragment ein cannot decide; document the boundary* | [S1g.4.5](s1g.4.5_the_boundary.md) |

## The instruction's part

> First order logic, what we can implement already and how, what is nearly possible, what is required.

## What it is

The phase has the milestone's two levels in its two halves.

**The first half reads Ein itself** (S1g.4.1, S1g.4.2). An Ein program is already a first-order theory of a
particular kind, and F1b asked which. The coordinator's expectation, which the stage tests row by row:

- a **rule** is a universally quantified implication whose body is a conjunction, a Horn clause, applied
  forward to a least fixpoint;
- a **nested fact** is a term built by a constructor: a function symbol with no equations;
- **`not`** is a stored, explicit negation, a fact like any other that a rule must derive; **`absent`** is
  negation as failure, a test of the state, non-monotone
  ([`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md),
  [`monotonicity.md`](../../../docs/kernel/inference/monotonicity.md)); **`(false)`** is an integrity
  constraint's head;
- an **obligation**, `(open ?R)`, is an existential conclusion with no witness built: the state owes one,
  and the search proposes witnesses *from the names the program has*;
- a **disjunction** in a conclusion does not exist; its work is done by hypotheses and an obligation, so by
  search;
- **`eq` and `neq`** are identity of values, computed: unique names, and no theory of equality;
- the **`forall` macro** is a bounded universal by double negation as failure, sound for a closed relation;
- a **relation parameter** quantifies over relation *names*, which is first-order over a reified signature
  and not second-order.

**The second half writes first-order logic in Ein** (S1g.4.3, S1g.4.4): formulas with quantifiers as values,
evaluated over a finite structure; substitution; the quantifier rules of a proof system; witnesses. Here the
instruction's three piles are sharpest: evaluating over a finite structure is expected possible today;
checking a first-order proof nearly possible (it needs substitution and a freshness condition); *finding*
one, and anything that enumerates terms without bound, is where the engine's limits are.

**The last stage is the boundary** (S1g.4.5): what Ein decides, semi-decides and cannot state, as a table
with a program per row, together with the parts of
[F26](../../followups/f26_consistency_and_completeness.md) that become concrete once the fragment is pinned.

## The stages

| ID | title | depends on | status |
|---|---|---|---|
| [S1g.4.1](s1g.4.1_the_fragment.md) | The fragment: Ein's rule language as first-order logic, a table with a program per row, and the BNF (PFL.1) | P1g.1 | planned |
| [S1g.4.2](s1g.4.2_relation_algebra.md) | Relation algebra: the stdlib's signature against Tarski's, a rule as an inclusion between relations, and properties detected from facts (PFL.3) | S1g.4.1 | planned |
| [S1g.4.3](s1g.4.3_formulas_with_quantifiers.md) | Formulas with quantifiers as data: terms, a finite structure, evaluation, and substitution | P1g.2; P1g.1 | planned |
| [S1g.4.4](s1g.4.4_quantifier_rules.md) | The quantifier rules: instantiation, generalisation, witnesses and Skolem terms, in a checked proof | S1g.4.3; S1g.3.1, S1g.3.3 | planned; one question **for the user** |
| [S1g.4.5](s1g.4.5_the_boundary.md) | The boundary: what Ein decides, what it only semi-decides, and what it cannot state (PFL.4); the glossary of *consistent* and *complete* | S1g.4.1 to .4 | planned |

## Acceptance

- **The fragment table,** every row with a program whose answer is computed, and a BNF of the fragment over
  the kernel's forms. A current kernel page, wherever P1f.20 leaves the kernel tree
  ([Q-M1g.12](../open_questions.md#q-m1g12--where-the-milestones-kernel-pages-go)).
- **The relation-algebra inventory:** each of Tarski's operations and each property against the stdlib rule
  that gives it, with its formula in the three forms of F15, and the answer to F1b's two TODO questions.
- **A quantified formula is evaluated over a finite structure,** search-free, and agrees with a Rust model
  checker on every formula up to a bound over every structure up to a size.
- **Substitution is right:** capture is avoided or refused, held by the Rust check on generated cases.
- **A first-order proof is checked,** with the quantifier rules' side conditions, and corrupted proofs
  (above all, a violated freshness condition) are rejected.
- **The boundary page,** each limit with a program that shows it, and the decidability table.
- **F1b's file is closed as promoted,** each PFL answered or its remainder named.

## Risks

- **Stating the fragment wrongly.** A table that says *Ein is Datalog with X* is a claim about a literature.
  Each row's claim is limited to what its program shows, and the comparison column cites the definition it
  compares with. Where the coordinator does not know (how Ein's obligations relate to the chase's
  termination classes; whether Ein's answer on a non-monotone program is a stable model), the page says *not
  established* and names F21 or F26.
- **Substitution with binders is where object-level logics go wrong.** The Rust check generates the capture
  cases on purpose.
- **Unbounded terms.** Any rule that builds a term from a term may not stop (probe 3). Every theory here is
  demand-driven or bounded, and S1g.6.1 is the engine's side.
