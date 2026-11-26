# E-124 — One state may answer a question of one value with a row for each pass of a rule that reads what it concludes

**Kind:** limit
**Status:** open
**Filed:** by S2.2.4, 2026-11-26, on `7aaefe6f`
**Owner:** none; [Q-M2.10](../../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md) and S2.5.3's answer rule read it
**Since P2.2's close (2026-11-27):** none in Ein; [Q-M2.33](../../open_questions/answer/Q-M2.33-is-a-class-with-several-rows-an-answer-of-several-values-or-the-undetermined-text.md), [S2.2d.4](../../p2.2d_formalizer_corrected/s2.2d.4_check_findings.md), [S2.5.3](../../p2.5_harness/s2.5.3_metrics.md)

## What

**The model's word problems write a change with no moment**: `(has Bob (n z 5) Apple)` and `(gave Bob (n z 2)
Alice)`, and a rule that reads `has` and concludes `has`, *"what a giver has after giving is what the giver had less
what was given"*. With no moment to keep the before apart from the after, the rule reads its own conclusion and fires
again: 5 less 2 is 3, 3 less 2 is 1, and 1 less 2 is no `diff-of` with a remainder of `z`, so it stops. **The program
is monotone, R is one state, `--single` says `Solution`, and the answer query asked of one value, *how many apples
does Bob have?*, has three rows in that one state**: *Bob has (n z 1) apples. Bob has (n z 3) apples. Bob has (n z 5)
apples.* Ein is right: a query lists every binding the state holds, and *one* counts states, not rows. What it does
not say is that the question was of one value and the state holds three: no line names the rule that fed itself.

## Evidence

S2.2.4's record, [`../../steps/`](../../p2.2_formalizer/steps/README.md), the condition `steps`: `bob1` (three rows: 5, 3, 1),
`bystander` (five: Carol's 9, 7, 5, 3, 1, the rule's `?t` reading the kind as a moment), `before` (three), each
`--single` *one*; `einaf/tests/fixtures/steps/bob1/`, the first, replayed in EinAf's gate step.

## What it costs the application

An outcome read off the kernel's word is *one* and wrong. The answer is S2.5.3's reader's, which reads the answer
query's classes and their rows; a program whose answer query has more than one row in the one state is an answer
of several values to a question of one, and the loop's repair is the facts' moments, which no diagnostic points at.
