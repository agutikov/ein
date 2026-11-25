# E-42 — A negative answer has no sentence

**Kind:** gap
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none; [M1p](../../../m1p_ein_to_natural_language/README.md)'s ground

## What is missing

**A negative answer has no sentence.** A query's `:goal-text` is filled once a row, so *no*, *unknown* and *not enough information* print nothing unless the program derives a fact to say them, as [`gap.ein`](../../../../examples/word_problems/gap.ein) does; and an empty R has no state to answer from

## Evidence

[`can_question.ein`](../../analysis/probes/p2.3/can_question.ein), [`notgoal.ein`](../../analysis/probes/p2.3/notgoal.ein); `ein solve examples/word_problems/gap.ein --single`

## What it costs the application

the answer is not always the program's own output: EinAf says the outcome in a fixed text ([the frame's default 15](../../README.md#the-defaults-the-re-plan-took))
