# E-108 — A clue with its arguments swapped is answered *one*, with another grid and, in the text's words, the same answer

**Kind:** limit
**Status:** open
**Filed:** by S2.2.0a, 2026-11-26, on `4bdbccaf`
**Owner:** none
**Since P2.2's close (2026-11-27):** none in Ein; [S2.4.4](../../p2.4_loop/s2.4.4_faithfulness.md) is what sees it

## What

**A clue with its arguments swapped is answered *one*, with another grid and, in the text's words, the same
answer.** Sentence (6) written the other way round, *the ivory house is immediately to the right of the green
house*, gives a puzzle with one solution, so every encoding answers `one` under every question, with no message: the
zebra in the fourth house instead of the fifth. `zebra.ein` and `zebra2.ein`, whose answer sentence names the
houses, print *"… the Japanese owns zebra in House-4"*; the four literal encodings, whose questions ask *who* and not
*where*, print *"The Norwegian drinks water. The Japanese owns the zebra."*, the right answer's sentence word for
word. Nothing a kernel can do tells a satisfiable wrong reading from the right one by its answer.

## Evidence

[`../zebra_formulations/probes/damaged/`](../../p2.2_formalizer/zebra_formulations/probes/damaged/), `*.swap-6.ein`: `einaf outcome
FILE` is `one` under each reading; `ein solve FILE --single` prints the sentence above.

## What it costs the application

The swap is the formalizer's likeliest slip on a sentence of direction, and it is invisible to the outcome, to the
answer and, in a literal encoding, even to the answer's text. Only a check of the facts against the sentences (a
faithfulness judge) sees it.
