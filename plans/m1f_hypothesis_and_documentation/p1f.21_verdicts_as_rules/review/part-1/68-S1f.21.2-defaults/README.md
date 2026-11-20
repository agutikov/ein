# 68 · The defaults S1f.21.2 took

[← part 1](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.2_the_pages.md#record)

S1f.21.2 is done: the pages say what part 1 built. It changed no engine code and no corpus program. You said to
run the phase on defaults and review the questions after. These are the five it took.

## What you should know first

- **The pages now say that a stored negation denies nothing by itself**: a candidate is denied, and a state dead,
  only where a rule refutes the pair, which a program has by importing `std.contradiction`. That was untrue on
  ten pages, some of them the second part's (default 5).
- **The user's page on constraints had a wrong answer** once the contradiction was a rule: its program gained the
  import (default 3), and the page shows what it answers without the line.
- **Nine decisions are written and three rewritten in place** under `docs/decisions/`; the record lists each.

## The five

| # | question | the default taken | the other reading |
|---|---|---|---|
| 1 | [Q-M1f.601](../../../../open_questions.md#q-m1f601--which-of-the-first-parts-readings-get-a-decision-file) | A reading about Ein that no decision carried got a file under its id: Q-M1f.552, .553, .555, .556, .559, .580, .589, .590 and .593. One whose content another decision states got none, that decision saying it; one about the corpus's bookkeeping got none. | a file for every id |
| 2 | [Q-M1f.602](../../../../open_questions.md#q-m1f602--the-reasoning-pages-notation-for-the-engines-word) | On the reasoning pages the word is written as a conclusion in the place of literals, the rules that state it are one set, *dead* is a match of one of them, and *denies* is a stored side beside a rule that refutes the pair. | none was weighed; the notation is the pages' and can be renamed |
| 3 | [Q-M1f.603](../../../../open_questions.md#q-m1f603--the-constraints-pages-running-program-imports-the-contradiction) | The user's page on constraints runs one program through. Its answer and its dead candidate rest on a negation being a constraint, so the program imports `std.contradiction`; without the line it has three candidates and a third state, and the page says so. | the program left without the import, and the page's answer rewritten to the one with no contradiction |
| 4 | [Q-M1f.604](../../../../open_questions.md#q-m1f604--how-the-words-a-module-reads-and-does-not-declare-are-listed) | Each list on the standard library's pages is taken from a run: the relations a program that is one bare import brings into being, with the rules that read each. | lists written by reading the modules |
| 5 | [Q-M1f.605](../../../../open_questions.md#q-m1f605--sentences-of-the-second-parts-pages-changed-by-the-first-parts) | Five sentences on the pages about hypotheses and the search, which are the second part's, said that a stored negation denies a candidate by itself. Each was untrue once the contradiction was a rule, and was changed; nothing else of those pages was touched. | left untrue until the second part's pages are written |

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 68. Blank keeps the five as written.
