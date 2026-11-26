# L-M2.29 — The model alone solves the Zebra puzzle in its thinking, names it, and prints no answer

**Status:** open
**Found:** by S2.2.2's probe (`model-alone.on.zebra`), read again by
[the analysis of P2.2 against its goals](../../p2.2_formalizer/report/ANALYSIS.md#10-the-model-solves-the-puzzle-and-cannot-state-it),
2026-11-26; filed at the phase's close, 2026-11-27
**Owner:** [S2.2c.2](../../p2.2c_decoding_and_model_alone/s2.2c.2_the_model_alone.md), which measures the model
alone; [Q-M2.39](../../open_questions/milestone/Q-M2.39-what-is-the-minimal-goal-s-instance-if-the-model-alone-solves-the-zebra-text.md),
the user's

## What

**The minimal goal's other side rests on the decoding.** The model alone on the Zebra text, thinking on, greedy,
reached the right grid in its thinking by about its 5 900th token, then went to double-check, opened a second
branch and repeated a cycle to its budget: no answer given. *The model alone does not solve it* is so far *the model
alone, decoded greedily, prints no answer*. Three things beside it:

- **It recognised the puzzle.** Its thinking opens *Here's a thinking process that leads to the solution of the
  Einstein's Riddle (Zebra Puzzle)*, and the five kinds it names (*pet*, *cigarette*, *nationality* among them) are
  in no sentence of the text.
- **The two modes never met.** Thinking on, it reads *the blue house* as a colour of one of five houses and uses
  *next to* both ways; writing the program, thinking off, the same weights wrote `BlueHouse` and
  `(lives-in Norwegian BlueHouse)`. No run has the model reason and then write a program.
- **It was run on this one text**, not on its two edits and not on a puzzle it cannot have seen.

## Evidence

[The probe's report § 4](../../p2.2_formalizer/model_runtime/README.md#4-the-model-alone-on-the-zebra-text) and
the record `model-alone.on.zebra` of `model_runtime/probe.jsonl.gz`;
[the analysis § 10.1](../../p2.2_formalizer/report/ANALYSIS.md#101-what-the-records-show).

## What it costs

Under the model's own sampler the model may solve the puzzle in text, and the minimal goal as worded would then
have no other side on this text; and a text the model knows by name is a poor place to look for what Ein adds.
Neither is measured.
