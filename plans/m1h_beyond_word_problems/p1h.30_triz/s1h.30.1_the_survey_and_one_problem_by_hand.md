# S1h.30.1 — The survey: what TRIZ consists of as statable things, and one small problem carried through by hand

**Phase:** [P1h.30](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A survey: it writes a page and no program. **It stops for the
user,** who reads it before anything is built.
**Depends on:** nothing. It may be run at any time the user wants it.

## What it rests on

The instruction, 2026-11-12:

> p1h.30 - TRIZ/TIPS by Genrikh Saulovich Altshuller; can we formalize it in Ein?

The milestone README: *Nothing of it is planned past the first step, which is a survey that says what TRIZ
consists of as "statable" things (its tables, its lists of principles, its notion of a contradiction between
requirements, its procedures) and, for each, whether it is facts, rules, a search, or something Ein has no
form for; with one small worked problem carried through by hand as the target. The user reads that before
anything is built. Its neighbour is P1h.20: a contradiction between two requirements is where requirements
verification ends and where, as the survey will have to confirm, TRIZ begins.*

## What it produces

`survey.md` in this directory: the parts of TRIZ as a table, a section a part, the worked problem, and an
answer to the instruction's question as far as a survey can give one. An entry in
[`docs/lib/`](../../../docs/lib/README.md) by that directory's conventions, in the file that is its most
natural home.

## Tasks

1. **T1h.30.1.1 — the sources.** Which of Altshuller's works the survey reads, in which language and
   edition, and which secondary sources, each with what it is (the author's own, a translation, a later
   school's account). The user reads Russian and English; where the original and a translation differ in a
   term, both are given. For each table or list the survey will describe: whether it may be reproduced in
   the repository, as far as the stage can establish it, with the source of the statement.
2. **T1h.30.1.2 — the statable things, each in its own section.** The four kinds the milestone README names,
   and whatever else the sources show that is neither:
   - **its tables:** what the rows and columns are, what a cell holds, how many there are;
   - **its lists of principles:** what a principle is, as a sentence, and how one is chosen;
   - **its notion of a contradiction between requirements:** what is said to contradict what, and the kinds
     the sources tell apart;
   - **its procedures:** the steps, what each takes in and gives out, and where a step is an instruction to
     a person to think.
   Each described from a source, with a small example of its use taken from that source.
3. **T1h.30.1.3 — for each, its form in Ein.** One of four, with the reason, and never more than the source
   supports:
   - **facts:** a table or a list a program could state as it stands;
   - **rules:** a step that derives something from stated things, with what it would read and conclude, in
     words;
   - **a search:** a choice among alternatives that the lattice of commitments could hold, with what the
     candidates would be and what would refute one;
   - **no form:** what Ein lacks for it, said plainly (an open vocabulary; a judgement of similarity; a
     generalisation from a concrete problem to an abstract one and back; a new object that no name of the
     program stands for).
   The page says for each table what a program would **do** with it once stated; a table stated is not
   TRIZ formalized.
4. **T1h.30.1.4 — one small worked problem, by hand.** A problem from the literature, with its published
   solution, small enough to follow on a page. Carried through as the sources' procedure has it, each step
   written down with: the part of TRIZ it used; what was stated at that point and what was concluded;
   whether an Ein program could have taken the step from what was stated (*yes, by a rule*; *yes, as a
   search over these candidates*; *no, and why*); and where the hand used judgement that no part of TRIZ
   states. **It is the target:** what a formalization would have to reproduce, and no program is written
   toward it here.
5. **T1h.30.1.5 — the neighbour.** *A contradiction between two requirements is where requirements
   verification ends and where, as the survey will have to confirm, TRIZ begins*: confirmed or corrected
   here, against T2's section on contradictions,
   and against [S1h.20.1](../p1h.20_requirements_verification/s1h.20.1_the_survey.md)'s survey if it has
   run. And what Ein already has at that point: a dead state with its core names the facts that conflict
   ([idea 03](../../ideas/03-three-task-classes.md)'s third class), which is where TRIZ would be handed the
   problem.
6. **T1h.30.1.6 — the answer, as far as a survey gives one.** *"can we formalize it in Ein?"*, part by
   part: *yes, as stated*; *in part, with this left out*; *no, for want of this*; *not known without
   trying*. And the question for the user,
   [Q-M1h.6](../open_questions.md#q-m1h6--whether-p1h20-and-p1h30-end-in-programs-at-all), put for this
   phase: does it go on to programs, on which parts, or stop here.
7. **T1h.30.1.7 — the page and the stop.** `survey.md`, every statement about TRIZ with its source. **The
   stage stops here.** The user's reading is recorded on the page in the user's words, and the answer to
   Q-M1h.6 for this phase in [`open_questions.md`](../open_questions.md).

## Acceptance

- Each of the four kinds of statable thing has a section with a source and an example from it.
- Each has one of the four forms, with the reason; every *no form* says what is missing.
- The worked problem is on the page step by step, each step marked as T4 asks.
- The instruction's question is answered part by part.
- No table or list is reproduced beyond what T1 established may be.
- No program was written, and nothing outside `plans/` and `docs/lib/` moved.
- The user has read the survey; the page records it.

## What it must not do

- **Not plan the phase past this page.** [S1h.30.2](s1h.30.2_the_plan_after_the_survey.md) is written after
  the user has read it.
- **Not write an Ein program,** a relation or a rule. T3's rules are described in words.
- **Not judge TRIZ.** The question is what of it can be stated.
- **Not let a secondary source stand for Altshuller** without saying so.
- **Not send anything about the user or the project** in a request to an outside service while surveying.

## The standing rules

Questions go to [`open_questions.md`](../open_questions.md) under the next free id; findings to `findings.md`
in this directory, opened at the first. A reading is recorded as the user gave it. The link check is run
before the commit, which changes pages only.
