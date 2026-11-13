# P1h.30 — TRIZ: *can we formalize it in Ein?*

**Milestone:** [M1h](../README.md)
**Status:** planned 2026-11-12 as a section of the milestone's README, on the user's instruction of that day
([§ The instruction](#the-instruction)); **written out as this directory on 2026-11-13,** on the user's
instruction of that day (*"materialize M1h phases"*). Not started. It carries no estimate (the user's ruling
of 2026-11-08). **A survey with one small worked problem carried through by hand, read by the user before
anything is built.** By the milestone README *nothing of it is planned past the first step*, so this
directory holds the survey's stage and one stage that is the place of the plan that follows and has no
content yet.
**Depends on:** nothing. The survey reads nothing of the earlier phases and may be run at any time the user
wants it ([M1h § The order](../README.md#the-order)).
**Blocks:** nothing. [P1h.20](../p1h.20_requirements_verification/README.md) is its neighbour.

The milestone README's section, [§ P1h.30](../README.md#p1h30--triz), is the phase's source and is kept as
written.

## The instruction

The user, 2026-11-12, in the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md), above the questions
on word problems. As typed:

> p1h.30 - TRIZ/TIPS by Genrikh Saulovich Altshuller; can we formalize it in Ein?

## What it is

**The phase is that question.** TRIZ, the theory of inventive problem solving (TIPS in English) of Genrikh
Saulovich Altshuller, is asked about as a body of method to be written as an Ein theory.

The first step, and the only one planned, is the milestone README's: *a survey that says what TRIZ consists
of as "statable" things (its tables, its lists of principles, its notion of a contradiction between
requirements, its procedures) and, for each, whether it is facts, rules, a search, or something Ein has no
form for; with one small worked problem carried through by hand as the target. The user reads that before
anything is built.*

**Its neighbour is P1h.20:** *a contradiction between two requirements is where requirements verification
ends and where, as the survey will have to confirm, TRIZ begins.*

## What it is not

- **Not a plan past the survey.** No relation, rule or program is designed before the user has read it.
- **Not a promise of a formalization.** *"can we formalize it in Ein?"* may be answered *in part* or *no*,
  and that is a result
  ([Q-M1h.6](../open_questions.md#q-m1h6--whether-p1h20-and-p1h30-end-in-programs-at-all)).
- **Not an assessment of TRIZ as a method of invention.** Whether it works is not the question. What of it
  can be stated, and in what form, is.
- **Not a survey of every school that came after Altshuller.** The instruction names him; later variants are
  named where the survey meets them and described only where they changed what is statable.

## The stages

| ID | title | depends on | stops for the user |
|---|---|---|---|
| [S1h.30.1](s1h.30.1_the_survey_and_one_problem_by_hand.md) | The survey: what TRIZ consists of as statable things, what form each would have in Ein, and one small problem carried through by hand | nothing | **yes: the phase's stop.** The user reads it before anything is built; Q-M1h.6, for this phase |
| [S1h.30.2](s1h.30.2_the_plan_after_the_survey.md) | The plan that follows the survey | the user's reading of S1h.30.1 | **written after the stop; no content before it** |

No stage carries an estimate. Findings go to `findings.md` in this directory, opened at the first one
([M1h § Stage, phase, milestone](../README.md#stage-phase-milestone)).

## Acceptance

From the milestone's: *the survey, read by the user, and the plan that followed it.*

- The survey exists as a page, every part of TRIZ it describes carrying a source.
- Each statable thing is on the page with what it is in Ein's terms: facts, rules, a search, or *no form*,
  and for the last, what is missing.
- One small problem is carried through by hand, each step of the hand's work marked with the part of TRIZ it
  used and whether an Ein program could have taken it.
- The neighbourhood with P1h.20 is confirmed or corrected on the survey's evidence.
- The instruction's question has an answer on the page as far as a survey can give one: *yes, these parts*;
  *no, these*; *not known without trying, these*.
- The user has read it and has answered Q-M1h.6 for this phase; S1h.30.2 is written to the answer.

## Risks, each with what checks it

- **The sources are uneven.** TRIZ is described by Altshuller's own books, by translations of them, and by a
  large secondary literature that does not always agree with them. Each statement of the survey says which
  source it is from, and where the instruction's *"by Genrikh Saulovich Altshuller"* and a later source
  differ, Altshuller's is the one described.
- **A table is easy to state and may be the least of it.** Facts are the form Ein has most readily. The
  survey says for each table what a program would *do* with it once stated, and does not count a table
  stated as TRIZ formalized.
- **The worked problem is chosen to succeed.** It is taken from the literature with its published solution,
  and the page says where the hand's work used judgement no part of TRIZ states.
- **The tables may not be reproducible in the repository.** They are published works. The survey states for
  each, as [S1h.10.1](../p1h.10_a_school_text/s1h.10.1_the_text.md) does for a text, whether it may be, with
  the source of the statement, and reproduces none before that is established.

## Connections

- **[P1h.20](../p1h.20_requirements_verification/README.md):** the neighbour, as above.
- **[Idea 03](../../ideas/03-three-task-classes.md):** the third task class, contradictions; TRIZ starts
  from one and Ein reports one with its core.
- **[P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md):** an inventive problem is a problem whose
  statement leaves out what would resolve it; the survey says whether that reading holds.
- **[`docs/lib/`](../../../docs/lib/README.md):** the catalogue has no entry for TRIZ on 2026-11-13
  (`grep -ril triz docs/lib` finds none); the survey adds one, by that directory's conventions, as P1h.20's
  survey adds its methods.
- **[`open_questions.md`](../open_questions.md):** Q-M1h.6.
