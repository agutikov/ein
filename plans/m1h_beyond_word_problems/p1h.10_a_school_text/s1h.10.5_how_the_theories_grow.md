# S1h.10.5 — How the theories grow: the five counts, per group and cumulatively, and the close

**Phase:** [P1h.10](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. It is past the phase's two stops: **its tasks are
written at [S1h.10.2](s1h.10.2_fifty_questions_in_five_groups.md)'s re-plan.** Its counting script is written
early, since [S1h.10.4](s1h.10.4_five_theories_or_one_theory_and_five_ontologies.md) uses it on the first two
groups. Its table is **shown** to the user.
**Depends on:** S1h.10.4's theories.

## What it rests on

The instruction, 2026-11-12:

> I expect theories complexity would grow (measuring complexity is itself a question, but let's now just
> count rules, rules size, facts, objects and names).

The milestone README: *Measure how the theories grow. … The five counts are taken for each theory by a
script, per group and cumulatively, and reported as a table; a better measure of complexity is noted as a
question and not pursued.*

So three things are said: there is an **expectation**, that complexity grows with the groups; there is a
**measure for now**, five counts; and the measure itself is *"a question"*, left open on purpose.

## What it ships

- **A script that takes the five counts** of a theory from its files, read as text. Each count needs a
  definition before it is a number, and the instruction gives the five names and no definitions:
  - **rules:** the `(rule …)` and `(hrule …)` forms a theory declares;
  - **rules size:** not defined by the instruction
    ([Q-M1h.20](../open_questions.md#q-m1h20--what-rules-size-counts)). The script takes more than one count
    side by side, the premises and conclusions of a rule and the names and variables it writes, so that the
    table can be read without a choice;
  - **facts:** the facts a theory states, told apart from the text's own, which are counted once, as the
    baseline;
  - **objects** and **names:** which of the names a theory writes is an *object* is a definition the stage
    proposes with the script and prints on the page (the candidates: a name that is an argument of a fact; a
    name the ontology declares an instance) beside the plain count of distinct names.
  It reads programs with [`utils/rule_census.py`](../../../utils/rule_census.py)'s reader or its like, runs
  no engine, and is a script of `utils/` with its line in `utils/README.md` (and the count of scripts in
  `AGENTS.md`'s `utils/` paragraph restated in place).
- **The table:** one row a group, the five counts **per group** (what that group's theory adds) and
  **cumulatively** (everything the group's questions are answered with), beside how many of the group's
  questions are answered as expected, and beside the baseline, the text's facts alone. Both shapes of
  Q-M1h.3 where both were written.
- **What the table says of the expectation,** in a paragraph: whether the counts grow with the groups, by
  how much, and where a group cost less than the one below it, if one did.
- **What the counts do not measure,** noted and not pursued: what the script cannot see (two writers, two
  counts; a rule's depth of nesting; how many facts a theory derives as against states), and one or two
  better measures as questions, with [M12](../../m12_rules_as_data/README.md)'s census of rule shapes named
  as the nearest instrument.

## The close

- **The phase's report,** one page: the text, the groups, how many questions each theory answered, the table,
  and the first thing the milestone README says the phase would show, with the evidence: *a question about a
  feeling or a motive needs facts no sentence states*. The facts each theory had to supply, listed by group.
- **Handed on by name:** to [M2](../../m2_nl_to_ir/README.md), the pairs of sentence and facts and the fifty
  questions with expected answers, as a small benchmark with graded difficulty, with where the files are; to
  [F26](../../followups/f26_consistency_and_completeness.md), what the upper groups met of its table of
  common sense.
- Every row of this phase's `findings.md` has a ruling, a fix or an owner; the phase's README and the
  milestone's phase table are restated to what was built.

## Acceptance

Written with the tasks. At the least: the script's counts for one theory are checked by hand against its
file; every definition is on the page beside its column; the table has a row for each of the five groups,
per group and cumulatively; the user has seen it.

## What it must not do

- **Not design a measure of complexity.** *"let's now just count"*.
- **Not rewrite a theory to move a count.**
- **Not conclude past the data:** one text, one writer of theories, fifty questions.
