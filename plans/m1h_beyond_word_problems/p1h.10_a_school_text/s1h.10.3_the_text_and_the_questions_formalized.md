# S1h.10.3 — The text and the questions formalized: sentences as facts by hand, questions as queries, the expected answers written before any theory

**Phase:** [P1h.10](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. It is past the phase's two stops: **its tasks are
written at [S1h.10.2](s1h.10.2_fifty_questions_in_five_groups.md)'s re-plan,** since every one of them
depends on the text and the questions the user accepts. What is below is what the instruction and the
milestone README already say of it.
**Depends on:** the text and the questions as accepted; the ruling on
[Q-M1h.18](../open_questions.md#q-m1h18--where-the-text-its-facts-and-the-theories-live) (where the files
live); and, where they exist, [P1h.1](../p1h.1_temporal_and_causal_theories/README.md)'s modules and
[P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md)'s rulings
([README § The order](README.md#the-order)).

## What it rests on

The instruction, 2026-11-12: *"And then formalize the text and questions"*. The milestone README: *Formalize
the text and the questions: the sentences as facts, by hand, each with its sentence as `:source`; each
question as a query, with the expected answer written before the theory exists.*

## What it ships

- **The text as facts.** Every numbered sentence written by hand as one fact or several, each with that
  sentence as its `:source`; a sentence that carries no fact is listed with the reason. The relations the
  facts use, each declared with a `:why` through which a fact reads back as a sentence close to the text's.
- **The questions as queries.** Each of the fifty a `(query …)` with its `:goal` and a `:goal-text` that
  reads as the answer's sentence.
- **The expected answers, first.** Each question's expected answer as the facts its query should match,
  written from S1h.10.2's answer in words **before any theory is written**, and committed before the first
  commit of a theory, so that the order is in `git log`. Where they are held until a theory makes them hold
  is settled at the re-plan: a query's `:expect` fails `ein test` until something derives its facts, so a
  claim goes into a tracked program in the commit that banks the theory behind it, and until then the
  expected facts stand on the page.
- **The pairs, for M2.** A table of sentence against facts, which is what
  [M2](../../m2_nl_to_ir/README.md) would have to produce from the text.

## What it carries, whatever the text

- **A program that loads and guesses nothing by accident.** The text's relations are closed, or the program
  states its lattice, as the word theory's programs do; a bare `ein solve` of the text's facts is one state.
- **The questions the facts alone answer, counted.** With no theory at all, a query of the first group may
  already have its row. The count a group, with facts alone, is the baseline of
  [S1h.10.5](s1h.10.5_how_the_theories_grow.md)'s table.
- **What is not in the text, kept apart.** A fact no sentence states is not written here. It is a theory's,
  in the next stage, and is told from the text's facts by its module and its `:source`.
- **The repo's rules for what is tracked:** a corpus entry for every `.ein` file under `examples/`, `tests/`
  or `stdlib/`; a `:why` on every relation; no claim in the corpus that the run does not hold.

## Acceptance

Written with the tasks. At the least: every sentence has its facts or its reason; every question is a query;
the commit of the expected answers is earlier than any theory's; the facts alone load and are one state;
where a tracked file outside `plans/` changed, `./run_tests.sh` green on the commit.

## What it must not do

- **Not write a rule.** A relation and its facts only; the rules are the theories'.
- **Not parse the text by a program.** The facts are written by hand; a parser is M2's.
- **Not change an expected answer after a theory has been run against it** without saying so on the page,
  with the old answer kept beside the new.
- **Not add a relation the grammar or the value rule refuses and work round it in silence:** a refusal is a
  finding, shown.
