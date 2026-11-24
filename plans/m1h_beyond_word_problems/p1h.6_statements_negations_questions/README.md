# P1h.6 — Statements, negations, questions: an open fact read as a question

**Milestone:** [M1h](../README.md)
**Status:** created 2026-11-18 on the user's instruction (below). **Not started**; no stage file yet, the phase is this
README and the note it promotes.
**Depends on:** M1f's P1f.21 for what an open fact is, **closed** on 2026-11-25 and its rulings decisions: `(open F)`
is a hypothesis, one fact whoever writes it, and `(open R)` what a state owes, held at a state's fixpoint; a rule may
read one, and its program is flagged
([Q-M1f.549](../../../docs/decisions/hypotheses/Q-M1f.549-a-hypothesis-is-a-fact-open-f-whoever-writes.md),
[Q-M1f.566](../../../docs/decisions/hypotheses/Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md),
[Q-M1f.569](../../../docs/decisions/hypotheses/Q-M1f.569-a-rule-may-read-an-open-fact-and-its-program.md)); its
working page on questions as facts, `open_facts.md` § 10.8, is in that phase's deleted folder
(`git log --diff-filter=D -- plans/m1f_hypothesis_and_documentation/p1f.21_verdicts_as_rules`); the hypothesis space's
structure as M1f's P1f.10 built it (`ein show hyps`, `ein render h-space`,
[`05-hypothesis-space.md`](../../../docs/ein/reasoning/02-kb-evolution/05-hypothesis-space.md)).

## The instruction

The user, 2026-11-18, verbatim:

> M1h - pull F20 there with one more idea of converting open facts into questions/queries by replacing one argument
> with variable, using this approch for hypothesis space analysis

**F20** is the user's note of 2026-10-16, that Ein's three rule kinds are the three sentence forms: a rule that
concludes a fact is a **statement**, one that concludes a negation or `(false)` a **negation** (a restriction), an
obligation a **question**; R is *the ways to answer the program's questions without contradicting its negations,
closed under its statements*. It was a followup; it is this phase's source, moved here whole:
[`f20_rule_kinds_as_sentence_moods.md`](f20_rule_kinds_as_sentence_moods.md).

## The new idea: an open fact is a question with one argument taken out

An open fact is a fact not yet decided, a candidate of H, `(owns Norwegian Zebra)`. **Replacing one of its arguments
with a variable makes it a question**, a query whose answers are the facts that fill the place:

| open fact | the place taken out | the question, as a query | in words |
|---|---|---|---|
| `(owns Norwegian Zebra)` | the 2nd argument | `(query :goal (owns Norwegian ?x))` | *what does the Norwegian own?* |
| `(owns Norwegian Zebra)` | the 1st argument | `(query :goal (owns ?x Zebra))` | *who owns the zebra?*, the puzzle's own question |
| `(owns Norwegian Zebra)` | the relation | `(?r Norwegian Zebra)`, which Ein's goal refuses unless the relation is bound | *how are the Norwegian and the zebra related?* |

**For the hypothesis space**, each candidate stands under one question per place, and each question gathers the
candidates that agree everywhere but in that place: its possible answers. So H read as questions is:

- **the groups**: a question's answers are a group of candidates (P1f.10's groups, a cover of H), named by the
  question that gathers them, not computed blind;
- **their standing at a state**: a question with no answer left is unanswerable (the state dies, or owes), one with
  exactly one left is forced, one with several is open; at root, at each state of R, and across R (a question R
  answers one way in every state is decided by the program);
- **obligations**: an obligation's alternatives are the answers of one question (F20), so *owes n* is *n questions
  unanswered*, and the question form names which;
- **an explanation**: an open state reads as the questions it has not answered (F20's way back, idea 08).

## What the phase would do

Stages, planned when the user starts the phase:

1. **The survey, on real programs** (zebra2 and its obligation twin, a seats program, a calculator program, a
   beliefs program): for every candidate of H and every place, the question, as Ein queries run by `ein solve`;
   the questions' answer sets beside `ein render h-space`'s groups and P1f.10's structure; each question's standing
   at root and across R; what the question form shows that the groups do not, and what it costs (H × places).
   Stops for the user.
2. **The design**, on the user's rulings: the surface (beside `ein show hyps` and `ein render h-space`), the question
   of a relation's place, F20's open question below.
3. **The build**, and the close.

**F20's open question**, carried: does every interrogative sentence of a puzzle become an obligation, or only those
the solution must answer? A query's `:goal` asks without requiring; an obligation requires.

## Rules of the phase

M1h's ([`../README.md`](../README.md)): a phase begins with real programs or a survey and stops for the user before
anything ships; no estimate is written.
