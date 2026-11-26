# E-42 — A negative answer has no sentence

**Kind:** gap
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none; [M1p](../../../m1p_ein_to_natural_language/README.md)'s ground

## What is missing

**A negative answer has no sentence.** A query's `:goal-text` is filled once a row, so *no*, *unknown* and *not enough information* print nothing unless the program derives a fact to say them, as [`gap.ein`](../../../../examples/word_problems/gap.ein) does; and an empty R has no state to answer from

## Evidence

[`can_question.ein`](../../analysis/probes/p2.3/can_question.ein), [`notgoal.ein`](../../analysis/probes/p2.3/notgoal.ein); `ein solve examples/word_problems/gap.ein --single`


Confirmed and sharpened by S2.2.5, 2026-11-26: a line is printed per row and for nothing else. An answer query with
one class and no row (`einaf/tests/fixtures/answer/norow.ein`), two classes or more (`several.ein`), an empty R, dead
(`inconsistent.ein`) or owing (`incomplete.ein`), print no line of the program's; `einaf from-text` prints one of
EinAf's fixed texts for each ([from a text to a short answer § 2](../../../../docs/einaf/08-from-text.md#2-the-answer)),
and every_outcome_has_its_line_and_einaf_from_text_replay_prints_it` holds the table.

## What it costs the application

the answer is not always the program's own output: EinAf says the outcome in a fixed text ([the frame's default 15](../../README.md#the-defaults-the-re-plan-took))
