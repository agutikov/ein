# S1h.10.1 — The text: a short list, with each candidate's length, language and whether it may be reproduced

**Phase:** [P1h.10](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13; **done 2026-11-13, by the user's choice, made before the short list was
written:** *Winnie-the-Pooh* by A. A. Milne, 1926, in English, in Project Gutenberg's edition, and of it
**Chapter II** ([§ The choice, 2026-11-13](#the-choice-2026-11-13); Q-M1h.4 and
[Q-M1h.21](../open_questions.md#q-m1h21--which-part-of-the-book)). Not answered at this stop, and carried to
[S1h.10.3](s1h.10.3_the_text_and_the_questions_formalized.md), which needs it: where the text and its
programs live and whether the repository holds the chapter's text
([Q-M1h.18](../open_questions.md#q-m1h18--where-the-text-its-facts-and-the-theories-live)).
[S1h.10.2](s1h.10.2_fifty_questions_in_five_groups.md) can run.
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

## The choice, 2026-11-13

**The user's words**, in two messages of 2026-11-13, the second while the first was being recorded:

> Winnie-the-Pooh — A. A. Milne (1926) plans/m1h_beyond_word_problems/p1h.10_a_school_text/s1h.10.1_the_text.md

> https://www.gutenberg.org/cache/epub/67098/pg67098-images.html

So the text is ***Winnie-the-Pooh*, by A. A. Milne, first published in 1926**, and the edition is Project
Gutenberg's eBook #67098, released 2022-01-03, its HTML file at the address the user gave (its header:
*Language: English*; *Original publication: Canada: McClelland & Stewart, Ltd., 1926*). That answers
[Q-M1h.4](../open_questions.md#q-m1h4--the-text-and-its-language): the text, and its language, **English, in
Milne's own words**. Boris Zakhoder's Russian *Винни-Пух и все-все-все* is a retelling and another text; it was
not named and is not this phase's.

**What of this stage was not done, and why.** The short list (T1h.10.1.2), what each candidate would carry
(T1h.10.1.3) and the language as a fact about Ein for each candidate (T1h.10.1.4) were for a choice the user
has made without them; they are not written. What the choice leaves open, and the facts it rests on, are
below.

### How long it is, by chapter

The instruction asks for *"50-100 sentences"*. The book is ten chapters, each longer than that but one.
Counted on the edition the user named (`python3 -I build/m1h/pooh/count_html.py build/m1h/pooh/pooh.html`,
the file fetched from the user's address on 2026-11-13, sha256 beginning `0f2cd0f4e0defa4a`). **The rule:**
a chapter's prose is every paragraph of it, its title (a heading) and its songs (the `poetry` blocks, counted
apart in lines) and the captions of its drawings left out; a sentence ends at `.`, `!` or `?`, after any
closing quotes or brackets, where whitespace and a capital, an opening quote or a bracket follow; *Mr.* and
*Mrs.* end none. So direct speech that ends in `!` or `?` and goes on with a lower-case word (*"…?" said
Pooh*) is one sentence with what follows. The rule is a script's and approximate; a hand count of the chapter
chosen is S1h.10.3's.

| chapter | title | sentences | words | lines of song |
|---|---|---:|---:|---:|
| I | In which we are introduced to Winnie-the-Pooh and some bees, and the stories begin | 169 | 2 075 | 16 |
| II | In which Pooh goes visiting and gets into a tight place | 110 | 1 438 | 6 |
| **III** | **In which Pooh and Piglet go hunting and nearly catch a Woozle** | **88** | **1 219** | **0** |
| IV | In which Eeyore loses a tail and Pooh finds one | 102 | 1 396 | 5 |
| V | In which Piglet meets a Heffalump | 178 | 2 624 | 8 |
| VI | In which Eeyore has a birthday and gets two presents | 281 | 2 550 | 12 |
| VII | In which Kanga and Baby Roo come to the forest, and Piglet has a bath | 238 | 2 566 | 18 |
| VIII | In which Christopher Robin leads an expotition to the North Pole | 270 | 2 859 | 18 |
| IX | In which Piglet is entirely surrounded by water | 168 | 2 639 | 0 |
| X | In which Christopher Robin gives Pooh a party, and we say good-bye | 139 | 1 774 | 40 |

The same rule on the edition's plain-text file (`pg67098.txt`, `build/m1h/pooh/count.py`) gives the same
counts for I, II, III, V and X and differs by one to sixteen on the other five, where the two files lay
songs and paragraphs out differently. Chapter III is the one chapter inside 50 to 100; IV and II are just past it.

### Which part: Chapter II

The book is longer than the instruction's 50 to 100 sentences, so the phase takes a part of it
([Q-M1h.21](../open_questions.md#q-m1h21--which-part-of-the-book)). The user, 2026-11-13, a third message:

> pick chapter 2

**Chapter II, *In which Pooh goes visiting and gets into a tight place*:** 110 sentences and 1 438 words by
the rule above, and six lines of song. It is ten sentences past the instruction's *100*, and the choice is the
user's with it. Pooh, calling on Rabbit at eleven o'clock, is offered honey or condensed milk and says
*"Both"*; on the way out he sticks in the door of Rabbit's house; Christopher Robin says he must stay *"About
a week"* until he is thin enough, reads him *"a Sustaining Book, such as would help and comfort a Wedged Bear
in Great Tightness"*, and at the end of the week they all pull him out. The phase's questions are written on
this chapter.

### Whether its text may be reproduced in the repository

As far as this stage can establish it, and it is not a lawyer:

- **In the United States the book is in the public domain since 1 January 2022**, a work first published in
  1926 ([Duke's Center for the Study of the Public Domain, Public Domain Day 2022](https://web.law.duke.edu/cspd/publicdomainday/2022/bcvpd/);
  [Smithsonian Magazine](https://www.smithsonianmag.com/smart-news/whats-entering-the-public-domain-on-january-1-winnie-pooh-ernest-hemingway-180979299/)).
  Project Gutenberg, which publishes what is in the public domain in the United States, released it on
  2022-01-03.
- **Where copyright runs for the author's life and seventy years, the United Kingdom among those places,** it
  is not yet: A. A. Milne died on 31 January 1956 (Wikipedia), so the term runs to 31 December 2026 and the
  text is free there from 1 January 2027. That date is derived from the rule and the date of death, not taken
  from a source that states it.
- **E. H. Shepard's drawings are a separate work.** None of them is reproduced, whatever is ruled.
- **Project Gutenberg's own terms** govern a copy of its file as it distributes it (its licence and its
  name); the stage records that and decides nothing.

Whether the repository holds Chapter II's text, or sentence numbers against the edition and short
quotations, is [Q-M1h.18](../open_questions.md#q-m1h18--where-the-text-its-facts-and-the-theories-live), still
the user's. Nothing of the text is in the repository today: the two files counted are under `build/`, which
is not tracked.

