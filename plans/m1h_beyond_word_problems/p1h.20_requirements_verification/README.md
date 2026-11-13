# P1h.20 — Requirements verification: a survey first

**Milestone:** [M1h](../README.md)
**Status:** planned 2026-11-12 as a section of the milestone's README, on the user's instruction of that day
([§ The instruction](#the-instruction)); **written out as this directory on 2026-11-13,** on the user's
instruction of that day (*"materialize M1h phases"*). Not started. It carries no estimate (the user's ruling
of 2026-11-08). **A survey first, which stops for the user;** by the milestone README *the phase's plan past
the survey is written after the user has read it*, so this directory holds the survey's stage and one stage
that is that plan's place and has no content yet.
**Depends on:** nothing. The survey reads nothing of the earlier phases and may be run at any time the user
wants it ([M1h § The order](../README.md#the-order)).
**Blocks:** nothing. [P1h.30](../p1h.30_triz/README.md) is its neighbour.

The milestone README's section, [§ P1h.20](../README.md#p1h20--requirements-verification), is the phase's
source and is kept as written.

## The instruction

The user, 2026-11-12, in the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md), above the questions
on word problems. As typed:

> p1h.20 - requirements verification. How is it done in real world? e.g. in systems design, or market
> research, construction, renovation, hair styling, cooking, anywhere. First survey methods of requirement
> verification. I think there are many, different tables, analysis, answering pre-defined or template
> questions and other approaches. How it is done? Probably my modelling. Like threats modelling, failures
> modelling, and many different modellings.

## What it is

**The question** is the instruction's: *How is it done in real world?* **The first step** is the
instruction's too: *First survey methods of requirement verification.* Three things in it bound the survey:

- **the fields,** as far apart as the user lists them: *systems design, market research, construction,
  renovation, hair styling, cooking, anywhere*;
- **the methods:** *"different tables, analysis, answering pre-defined or template questions and other
  approaches"*;
- **the user's own conjecture,** that it is done by **modelling**: *"Probably my modelling. Like threats
  modelling, failures modelling, and many different modellings."* The survey tests the conjecture and does
  not assume it.

**What the survey has to return,** so that the next step can be planned from it (the milestone README): for
each method, what its input is (a list of requirements, a design, a model), what it produces (a table, a
verdict, a list of gaps or conflicts), and which part of it is *reasoning over stated facts*, which is the
part Ein could do. Its catalogue entries go to [`docs/lib/`](../../../docs/lib/README.md) beside the
external-tech notes, by that directory's conventions.

**Its natural meeting points with Ein,** *to be confirmed by the survey and not assumed*: the three task
classes ([idea 03](../../ideas/03-three-task-classes.md): solve, gaps, contradictions) read as *the
requirements are satisfiable, incomplete, or in conflict*;
[P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md)'s gaps and assumptions;
[P1h.2](../p1h.2_slicing/README.md)'s slice as *which requirements a design decision rests on*.

## What it is not

- **Not a plan past the survey.** Nothing is designed, and no program is written, before the user has read
  the survey.
- **Not a promise of programs.** *Whether P1h.20 and P1h.30 end in programs at all* is the milestone's open
  question ([Q-M1h.6](../open_questions.md#q-m1h6--whether-p1h20-and-p1h30-end-in-programs-at-all)); *in
  part* or *no* is a result.
- **Not a survey of requirements engineering as a whole.** How requirements are gathered, written and
  managed is in scope only where a method of *verifying* them needs it said.
- **Not a tool comparison.** A product is named where it is the usual carrier of a method, and no further.

## The stages

| ID | title | depends on | stops for the user |
|---|---|---|---|
| [S1h.20.1](s1h.20.1_the_survey.md) | The survey: how requirements are verified, field by field and method by method, and which part of each is reasoning over stated facts | nothing | **yes: the phase's stop.** The user reads the survey; Q-M1h.6, for this phase |
| [S1h.20.2](s1h.20.2_the_plan_after_the_survey.md) | The plan that follows the survey | the user's reading of S1h.20.1 | **written after the stop; no content before it** |

No stage carries an estimate. Findings go to `findings.md` in this directory, opened at the first one
([M1h § Stage, phase, milestone](../README.md#stage-phase-milestone)).

## Acceptance

From the milestone's: *the survey, read by the user, and the plan that followed it.*

- The survey exists as a page, with every method it describes carrying a source.
- For each method the page says its input, its output, and which part is reasoning over stated facts.
- Each field the user names has at least one method described, or the page says what was looked for and not
  found.
- The user's conjecture, *done by modelling*, is answered on the survey's evidence.
- Each of the three meeting points with Ein is marked confirmed, not confirmed or not reached, with the
  method that shows it.
- The catalogue entries are in `docs/lib/` by its conventions, and the link check is green.
- The user has read it and has answered Q-M1h.6 for this phase; S1h.20.2 is written to the answer.

## Risks, each with what checks it

- **The survey finds only what software engineering wrote down.** The instruction's fields run to *hair
  styling* and *cooking*, where the method may be a practice with no name. S1h.20.1 looks in each field
  separately and reports a practice in the field's own words before it maps it to anything.
- **Everything looks like a constraint problem to a reasoner.** The meeting points are *to be confirmed and
  not assumed*. The survey marks, for each method, the part that is **not** reasoning over stated facts
  (judgement, measurement, a trial, a conversation) as carefully as the part that is.
- **A catalogue of names.** A list of methods with a line each answers nothing. The acceptance asks for
  input, output and the reasoning part of each, and the page is organised by those.
- **Sources of uneven weight.** A standard, a textbook and a blog post are not the same. Each entry says
  what its source is.

## Connections

- **[P1h.30](../p1h.30_triz/README.md):** a contradiction between two requirements is where requirements
  verification ends and where, as that survey will have to confirm, TRIZ begins.
- **[P1h.2](../p1h.2_slicing/README.md), [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md):** the
  meeting points above.
- **[Idea 03](../../ideas/03-three-task-classes.md):** three task classes.
- **[`docs/lib/`](../../../docs/lib/README.md):** the catalogue the entries join;
  [`07`](../../../docs/lib/07-static-analysis-compilers.md) and
  [`03`](../../../docs/lib/03-theorem-proving-formal-methods.md) hold the formal-methods end of the subject
  already.
- **[F17](../../followups/f17_formal_verification/ideas.md):** formal verification, the followup; the
  survey says where a method of this phase is that subject and leaves it there.
- **[`open_questions.md`](../open_questions.md):** Q-M1h.6.
