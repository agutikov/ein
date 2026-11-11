# S1g.1.4 — The engine requirements, and the design page the user reads

**Phase:** [P1g.1](README.md)
**Status:** planned 2026-11-11, not started.
**Depends on:** [S1g.1.1](s1g.1.1_what_ein_represents_today.md), [S1g.1.2](s1g.1.2_two_levels.md),
[S1g.1.3](s1g.1.3_conventions.md).
**Stops for the user.** This is the phase's stop, and the milestone's first.

## What it asks

> First order logic, what we can implement already and how, what is nearly possible, what is required.

(the user, 2026-11-11). The survey's third pile is *what is required*. This stage turns it into a list the
user can rule on, restates [P1g.6](../p1g.6_engine_requirements/README.md)'s provisional stage list to it, and
assembles the page.

## The requirements list

Each row of the list has the same columns, so that one can be compared with another:

- **what cannot be done,** in one sentence, in the logic's words and in Ein's;
- **the witness:** the program of S1g.1.1 that shows it, with what the binary said;
- **who needs it:** the stages of P1g.2 to P1g.5 that cannot be done, or can only be done by a detour, without
  it; and the detour, if there is one;
- **the smallest change** that would allow it, and what kind of change it is: a fix (the engine does
  something wrong today, so a register row first), a read-out change, a new behaviour of the engine, or a
  change to the grammar (the user's ruling first);
- **what it could move:** which existing answers, goldens and pages, by a first sweep where that is cheap;
- **the recommendation:** build it in P1g.6, take the detour and park the row, or leave it to an existing
  followup that already owns it.

The list starts from P1g.6's seeded candidates, C-1 to C-11
([its table](../p1g.6_engine_requirements/README.md#the-candidates-as-seeded-2026-11-11)), each of which this
stage confirms with a witness, corrects or drops, and adds what the survey found.

**The discipline every row is held to,** stated once on the page: a wrong answer is a register row before it
is fixed; a grammar change needs the user's ruling; config never changes the answer; no native numbers
([Q-M1e2.24](../../m1e2_correctness/open_questions.md#q-m1e224--are-there-numbers-in-ein) stands), and
nothing here adds engine arithmetic.

## The design page

`p1g.1_survey_and_design/design.md`, in the order a reader needs:

1. **The answers in one screen:** the instruction's questions, each with a two-line answer and a link to its
   program.
2. **The programs,** S1g.1.1's sixteen, each with its run.
3. **The two levels,** S1g.1.2's three tables.
4. **The conventions,** S1g.1.3's choices and recommendations.
5. **Possible, nearly possible, required:** the three piles.
6. **The requirements list,** as above.
7. **What the survey changes in the plan:** each of P1g.2 to P1g.6, with the stages it confirms, re-scopes or
   drops.
8. **The questions for the user,** each with a recommendation: Q-M1g.2 and .3 from S1g.1.3;
   [Q-M1g.5](../open_questions.md#q-m1g5--is-a-refuted-assumption-a-fact) to
   [.8](../open_questions.md#q-m1g8--fresh-names) from the requirements, each asked only as far as the survey
   can already show its options on programs;
   [Q-M1g.10](../open_questions.md#q-m1g10--the-optional-proof-systems) and
   [.11](../open_questions.md#q-m1g11--how-far-induction-goes), the scope questions.
9. **§ The rulings,** empty until the user answers, then their words verbatim and dated, as
   [the calculator's](../../m1f_hypothesis_and_documentation/p1f.6_calculator/design.md#the-rulings-2026-11-11)
   are.

## Tasks

1. **T1g.1.4.1 — the requirements list,** every row with its witness.
2. **T1g.1.4.2 — P1g.6 restated:** its README's table and its stage files edited to the list. A candidate the
   survey could not witness is struck with the reason; a new one gets a stage file or a row of an existing
   one.
3. **T1g.1.4.3 — `design.md`,** assembled, every number with its command and date.
4. **T1g.1.4.4 — the stop.** The page shown to the user; the rulings recorded; the milestone's phases
   re-planned to them in the same commit as the record, the way P1f.6 was re-planned on 2026-11-11.

## Acceptance

- `design.md` exists and is complete to its section 8; every program's answer is computed.
- Every requirement has a witness program, or is struck.
- P1g.6's table and stage files agree with the list.
- The user's rulings are recorded verbatim, and no later phase has started before them.
- The commit is docs only.

## What it may find

- That the list is short. Then P1g.6 shrinks, and the milestone is mostly theories written in Ein. That is a
  result, and the page says so.
- That one requirement gates most of the milestone (the likeliest: a saturation that does not stop, or the
  nested-fact index). Then the order in [M1g § The order](../README.md#the-order) changes, and that
  requirement's stage runs first.
