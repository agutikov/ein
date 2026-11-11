# P1g.2 — Propositional logic: formulas, truth tables, valuations as hypotheses, normal forms

**Milestone:** [M1g](../README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [P1g.1](../p1g.1_survey_and_design/README.md)'s rulings, for the conventions; of M1f,
[S1f.6.17](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.17_binary_logic.md)'s gate tables,
which the connectives' tables are or are held equal to.
**Blocks:** [P1g.3](../p1g.3_proof_systems/README.md), whose soundness and completeness are stated against
this phase's truth tables.

## What it is

The semantics of propositional logic, written in Ein and computed by its two mechanisms, each doing the part
that is its own:

- **Saturation evaluates.** A formula is a nested fact. Under a given valuation its value is derived by rules
  over the connectives' truth tables, on request, with no search: H empty, k_s 1, R {∅}, as an arithmetic
  program is ([S1g.2.1](s1g.2.1_formulas_and_evaluation.md)).
- **The lattice quantifies over valuations.** Make each letter's value a hypothesis, and R is the set of
  valuations. Then the classical notions are questions about R, in the read-out's own words
  ([S1g.2.2](s1g.2.2_valuations_as_hypotheses.md)):

  | the notion | as a question about R |
  |---|---|
  | the truth table of F | a query's projection classes over all of R |
  | F is a tautology | F has the value 1 in every state: one class |
  | F is satisfiable | require F; R is not empty, and its states are the satisfying valuations |
  | F is unsatisfiable | require F; R is empty, and N's cores say why |
  | F and G are equivalent | no state tells them apart: requiring that they differ empties R |
  | Γ entails F | require Γ and that F is 0; R is empty |

- **Rules rewrite.** Normal forms (negation normal form, conjunctive, disjunctive) are rewriting on request,
  a third use of saturation, each result held equivalent to its source by the table above
  ([S1g.2.3](s1g.2.3_normal_forms.md)).

And **a formula is a circuit**: S1f.6.17's gate-built adder and this phase's formulas are one thing in two
shapes, and S1g.2.1 builds the bridge both ways.

A first probe of the second point ran on 2026-11-11 ([M1g § First probes](../README.md#first-probes-2026-11-11),
1 and 1b): Peirce's law over two letters gave four states and one projection class, `?v = 1`. The phase
re-takes it and builds on what the user rules at P1g.1's stop.

## The stages

| ID | title | depends on | status |
|---|---|---|---|
| [S1g.2.1](s1g.2.1_formulas_and_evaluation.md) | Formulas and evaluation: the connectives' tables, the value of a formula on request, and the bridge to circuits | P1g.1 | planned |
| [S1g.2.2](s1g.2.2_valuations_as_hypotheses.md) | Valuations as hypotheses: the truth table as R; tautology, satisfiability, equivalence and entailment as questions about R | S1g.2.1 | planned; its first task **stops for the user** |
| [S1g.2.3](s1g.2.3_normal_forms.md) | Normal forms as rewriting: NNF, CNF, DNF, and a connective defined by others | S1g.2.1; S1g.2.2 for the checks | planned |
| [S1g.2.4](s1g.2.4_applications_and_scale.md) | Applications and scale: puzzles of truth and lies as formulas, Ein's own three states as a third truth value, and how many letters is too many | S1g.2.2 | planned |

## Acceptance

- **Search-free evaluation, held by a test:** on every program that evaluates a formula under a written
  valuation, no candidate, k_s 1, R {∅}, and the theory writes no `absent` and no `not` premise.
- **Right, against an independent check written in Rust:** for every formula up to the bound S1g.2.1 names,
  over its letters, the value under every valuation, the tautology verdict, the satisfying set, and each
  normal form's equivalence to its source agree with a Rust evaluator that shares nothing with the theory.
  The harness is a table of checks that P1g.3 adds rows to, as the calculator's `OPS` is.
- **The truth table is R, and the oracle agrees:** on every program of S1g.2.2 within the oracle's limit,
  `ein oracle` gives the same R as `ein solve`.
- **Each classical notion has a program and a claim** that `ein test` holds, and a page that says, for each,
  how it reads in the read-out, including the cases where the read-out's words surprise (probe 1b).
- **The gate-built circuit and the formula agree,** on S1f.6.17's adder, held by a test.
- **The cost is measured, not asserted:** matcher candidates per request (the calculator's
  `arith_batch.rs` is the model) for evaluation, and saturations and states for each lattice program, each
  with its command.

## Risks

- **A join on a formula scans.** The natural evaluation rule looks up `(value-of ?a ?x)` with `?a` a nested
  fact and `?x` free, which E-3 says walks the relation. S1g.2.1 measures it and writes the continuation form
  beside it.
- **The obligation and the hrule say the same thing twice.** *Every letter has a value* is an obligation,
  and the hrule proposes each value; the obligations rung could propose them from the obligation alone
  (`(bijective color-loc)` does for the zebra). S1g.2.2 tries the program with no hrule.
- **An unsatisfiable requirement does not read as a contradiction.** Probe 1b's read-out says *empty: every
  alive node owes*. It is right, and a reader expecting *every node is dead* will misread it. The page of
  S1g.2.2 has to teach the read-out, and
  [`read_out.md`](../../../docs/kernel/inference/read_out.md) is the reference.
