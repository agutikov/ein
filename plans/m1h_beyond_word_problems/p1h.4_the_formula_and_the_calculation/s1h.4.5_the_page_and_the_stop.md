# S1h.4.5 — The page and the stop

**Phase:** [P1h.4](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. **It is the phase's stop:** no engine source, no grammar file, no
printed line and nothing outside `plans/` and `build/` changes before the user has read its page and ruled.
**Depends on:** [S1h.4.1](s1h.4.1_the_two_parts_and_a_well_formed_formula.md) to
[S1h.4.4](s1h.4.4_the_alternatives_not_taken.md), whose sections it assembles.

## What it rests on

The user's reply that made the subject a phase, 2026-11-12: *"How we can divide 2 parts? inference of a
formula and calculations […] Yes, this is a phase in M1h"*. The user's rule for a change to the language or to
what a surface prints (2026-10-16): real programs, with their computed answers, shown before it is
implemented. And the milestone's acceptance for this phase: *a solved problem's formula is an object a reader
can see apart from its calculation, and the page says what Ein can and cannot say about a solution.*

## What it produces

`formula_and_calculation.md` in this directory, whole: the four sections brought to one page, the proposals,
and the questions. The commit at the stop is pages only.

## Tasks

1. **T1h.4.5.1 — the page assembled.** One reading order: the two parts and the line; the formula as an
   object, two variants; the pupil's page, three forms; the alternatives not taken and *answer correct,
   solution not*. Every program re-run on the binary of the day, every number with its command and date.
2. **T1h.4.5.2 — the reply's question, answered at the top.** *"How we can divide 2 parts?"*, in a
   paragraph, from S1h.4.1's line and S1h.4.2's formula: what the first part delivers, what the second
   assumes, and the condition *well-formed* as the table of checks with what is missing.
3. **T1h.4.5.3 — the proposals, each on real programs.**
   - **the formula:** the variant recommended, *before* or *instead of*, as the banked two-step problem
     rewritten with it and its read-out beside today's; or *neither*, with the reason;
   - **the pupil's page:** the form recommended of the three, as the page it writes for the set of problems;
     what it would change in the engine, the grammar or neither;
   - **what is not proposed:** anything for *why not*, unless S1h.4.4 found a construction on the right side
     of the user's line.
   A recommendation the runs do not support is not made.
4. **T1h.4.5.4 — the questions:** [Q-M1h.16](../open_questions.md#q-m1h16--the-pupils-page), the pupil's
   page, **unless P1g.10's S1g.10.4 has ruled its question f**, in which case the ruling is quoted and the
   question is not asked again;
   [Q-M1h.17](../open_questions.md#q-m1h17--the-formula-as-an-object), the formula as an object;
   [Q-M1h.2](../open_questions.md#q-m1h2--whether-it-follows-p1g10), if the formula's data is still
   provisional because P1g.10 has not run. And any question the four stages raised, filed under the next
   free id first.
5. **T1h.4.5.5 — the findings shown.** Every row of this phase's `findings.md`, each with its witness; and
   the notes to P1g.10 from S1h.4.2's first task, as a list that phase can take.
6. **T1h.4.5.6 — the stop.** The user reads the page. The rulings are recorded on it in the user's words and
   in [`open_questions.md`](../open_questions.md); a ruling on Q-M1h.16 is recorded in P1g.10's README
   against its question f as well, by a pointer.
7. **T1h.4.5.7 — the re-plan.** [S1h.4.6](s1h.4.6_the_pupils_page_built.md) and
   [S1h.4.7](s1h.4.7_the_formula_banked_and_the_close.md) are restated to the rulings and given their tasks.

## Acceptance

- The page exists; every program on it was run by the binary; every number carries its command and date.
- The reply's question has its answer at the top, with the table of checks.
- Each proposal is shown on real programs, or the page says none is made and why.
- The user has ruled, or has said which questions wait; the page and `open_questions.md` record it in the
  user's words.
- No file under `ein.rs/`, `docs/kernel/`, `examples/`, `stdlib/` or `tests/` moved.

## What it must not do

- **Not change the trace, the grammar, the word theory or the calculator.**
- **Not take a default** on a question the user has not answered.
- **Not ask the user about a theory of solutions.** The user ruled it out of scope; the page says where the
  line fell and stops.

## The standing rules

Questions go to [`open_questions.md`](../open_questions.md) under the next free id; findings to this phase's
`findings.md`. A ruling is recorded as the user gave it and may be reopened by a later finding. The link check
is run before the commit, which changes pages only.
