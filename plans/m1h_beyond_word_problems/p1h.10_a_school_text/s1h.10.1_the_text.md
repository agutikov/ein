# S1h.10.1 — The text: a short list, with each candidate's length, language and whether it may be reproduced

**Phase:** [P1h.10](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. **It stops for the user:** the choice of the text is the user's,
and nothing of the phase is written past this page before it is made.
**Depends on:** nothing.

## What it rests on

The instruction, 2026-11-12:

> p1h.10 - pick a prose text for like end of elementry school (3-5th grade) 50-100 sentences, very famous
> everybody would remember from school.

The milestone README: *Pick the text: prose, of the end of elementary school (3rd to 5th grade), 50 to 100
sentences, "very famous everybody would remember from school". The choice is the user's; the phase proposes a
short list with, for each, its length, its language and whether its text may be reproduced in the repository,
and stops.* And its open question 4: *the user is bilingual and the repo's documents are in English; a text
"everybody would remember from school" depends on whose school.*

## What it produces

`the_text.md` in this directory: a short list of candidate texts as one table, a paragraph a candidate, and
the two questions for the user. No program, and nothing outside `plans/`.

## Tasks

1. **T1h.10.1.1 — the criteria, from the instruction's words,** each made checkable before any text is
   looked at:
   - ***prose*:** not verse, and not a play;
   - ***end of elementary school (3-5th grade)*:** read in those grades, with the reader or curriculum that
     shows it named;
   - ***50-100 sentences*:** counted, by a rule the page states (what ends a sentence; how direct speech is
     counted), on a named edition or translation; a text outside the range is listed only with the count
     and the word *outside*, and an excerpt of a longer text is listed as an excerpt;
   - ***very famous everybody would remember from school*:** which school. Since
     [Q-M1h.4](../open_questions.md#q-m1h4--the-text-and-its-language) is open, the list draws from each
     school tradition the user may mean, the Russian and an English-language one at the least, and says for
     each candidate where it is read.
2. **T1h.10.1.2 — the candidates.** A short list, each with: title, author, year; the language of the
   original and of the edition counted; the count of sentences; the grade and the school; **whether its text
   may be reproduced in the repository,** for the original and for the translation counted, as far as the
   stage can establish it and with the source of the statement (the stage is not a lawyer and says so; where
   it cannot establish it, the cell says *not established*).
3. **T1h.10.1.3 — what each candidate would carry.** The instruction's second sentence asks for questions
   up to *"feelings, motivation explanation, call of duty, moral dilemmas, what auther wanted to say"*. For
   each candidate, in a few lines: who acts in it and how many events it has; whether it has a stated order
   of events and stated causes; whether a character's feeling or motive is stated or only shown; whether it
   holds a choice between duties; whether a reader is usually asked what the author meant. A candidate that
   cannot carry the upper groups is kept on the list and marked.
4. **T1h.10.1.4 — the language, as a fact about Ein.** What the choice of language costs in a program: a
   `:source` string holds any Unicode and a name is ASCII, so a text that is not in English stands beside
   its facts with its people and things under transliterated names, and every `:why` and `:goal-text` is
   written in one language, which the page proposes (the repo's documents are in English). The check of
   2026-11-13 is re-taken on one sentence of each candidate's original.
5. **T1h.10.1.5 — where it would live.**
   [Q-M1h.18](../open_questions.md#q-m1h18--where-the-text-its-facts-and-the-theories-live): the candidates
   for a home (a directory under `examples/` named for the text; `tests/` for the questions; the stdlib for
   nothing yet), and what the repository holds of the text itself under each answer to T2's last column.
6. **T1h.10.1.6 — the page and the stop.** The table, the paragraphs, and the two questions. **The stage
   stops here.** The user's choice is recorded on the page in the user's words and against Q-M1h.4 and
   Q-M1h.18 in [`open_questions.md`](../open_questions.md).

## Acceptance

- Every candidate on the list has every column of T2 filled, or the words *not established*.
- Every count of sentences names the edition counted and was taken by the stated rule.
- The list draws from more than one school tradition, or the page says why it could not.
- The user has chosen a text, or has asked for another list; the page records it.
- Nothing outside `plans/` moved.

## What it must not do

- **Not choose the text.** A recommendation is allowed, with its reason; the choice is the user's.
- **Not reproduce a candidate's text on the page** beyond a sentence or two of quotation, before T2's column
  for it says it may be.
- **Not write a fact, a question or a relation.** Those are the next stages', after the choice.
- **Not send anything about the user or the project** in a request to an outside service while looking for
  texts, editions or their status.

## The standing rules

Questions go to [`open_questions.md`](../open_questions.md) under the next free id; findings to `findings.md`
in this directory, opened at the first. A ruling is recorded as the user gave it. The link check is run before
the commit, which changes pages only.
