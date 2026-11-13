# S1h.10.4 — Five theories, or one theory and five ontologies: each group's questions answered

**Phase:** [P1h.10](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. It is past the phase's two stops: **its tasks are
written at [S1h.10.2](s1h.10.2_fifty_questions_in_five_groups.md)'s re-plan.** It **stops for the user once,**
inside it, on [Q-M1h.3](../open_questions.md#q-m1h3--five-theories-or-one-theory-and-five-ontologies): the two
shapes on the first two groups, shown, before the other three groups are written.
**Depends on:** [S1h.10.3](s1h.10.3_the_text_and_the_questions_formalized.md): the text as facts, the
questions as queries, the expected answers on the page.

## What it rests on

The instruction, 2026-11-12: *"and then write 5 theories (or one theory and 5 ontologies) that allow answering
those groups of questions. I expect theories complexity would grow"*. The milestone README: *Write five
theories, "or one theory and 5 ontologies", each allowing the questions of its group to be answered. Which of
the two shapes is a question the phase answers by trying.*

Two readings are open in those words, and the stage does not close either by itself:

- **the two shapes.** *Five theories*: a module of rules a group, each over the one before or beside it.
  *One theory and five ontologies*: one set of rules, and for each group a body of types, instances and
  stated relations between them (the glossary's *ontology layer*,
  [`plans/README.md`](../../README.md#glossary)) that lets that group's questions be answered. Q-M1h.3;
- **whether a group's theory may use the groups below it.** *"complexity would grow"* reads as cumulative,
  and the milestone README has the counts taken *per group and cumulatively*; the stage builds each group
  over the ones below and counts both ways, so that neither reading is lost.

## What it ships

- **Both shapes, on the first two groups.** The same questions answered twice: as two modules of rules, and
  as one module of rules with two ontologies. Each with how many of the group's questions it answers as
  expected, and [S1h.10.5](s1h.10.5_how_the_theories_grow.md)'s five counts taken for each. **Shown to the
  user,** with what the stage saw of each shape: which grew faster, which was easier to say a thing in,
  which could not say something the other could. The user says which shape the other three groups take, or
  that both continue.
- **The other three groups,** in the shape ruled, each group's theory written against its questions, in the
  groups' order.
- **For every group, on the page:** the questions answered as expected, held by `ein test` with each
  question's `:expect` now in its program; the questions not answered, each with what answering it would
  need; and the facts the theory states that no sentence of the text states, listed, since they are the
  measured thing.

## What it carries, whatever the shape

- **A fact the theory supplies is told from the text's.** It sits in the theory's module or the group's
  ontology, with a `:source` that says it is common knowledge, or a reader's, and not a sentence; written as
  [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md)'s rulings have an assumption written, if that
  phase has ruled. The slice of an answer then shows which of them the answer rests on
  ([P1h.2](../p1h.2_slicing/README.md)).
- **An answer the theory gives that was not expected is not hidden.** A query with more rows than its
  expected answer, or other rows, is on the page as it is; an expected answer is changed only as
  S1h.10.3 allows, with the old one kept.
- **No theory reads another group's expected answers.** A theory is rules and facts about the story's
  world, and a rule that names a question is not one.
- **Held-back questions,** if the user asked for them at S1h.10.2's stop: answered by the finished theory,
  counted apart.
- **Every program states its lattice; every run carries `-T`;** each theory is put through
  `ein solve --monotone`, and a flagged one is shown with its site.
- **Timeline, causes, gaps:** from [P1h.1](../p1h.1_temporal_and_causal_theories/README.md)'s modules where
  they have shipped, and the page says what was imported; written locally and counted where they have not.
- **The repo's rules for what is tracked:** a `:why` on every relation and rule; a corpus entry a file; the
  stdlib's gates in full if, and only if, the user rules a theory into the stdlib.

## Acceptance

Written with the tasks. At the least: both shapes exist for the first two groups and the user has seen them;
for each of the five groups the page says how many questions are answered as expected and lists the rest;
every claim in a tracked program holds under `ein test`; where a tracked file outside `plans/` changed,
`./run_tests.sh` green on the commit.

## What it must not do

- **Not answer a question by a fact that is the answer.** A theory supplies knowledge about the world of
  the story, from which the answer follows; a fact that states the expected answer outright is counted as
  such and shown, never passed off as derived.
- **Not tune a theory to its counts.** The counts are taken of the theory as it was written to answer the
  questions.
- **Not generalise past the text** in what it claims: one text, fifty questions.
- **Not choose the shape for the user** past the first two groups.
