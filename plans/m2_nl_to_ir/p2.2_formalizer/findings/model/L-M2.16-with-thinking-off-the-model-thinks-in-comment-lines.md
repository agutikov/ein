# L-M2.16 — With thinking off, the model thinks in comment lines, and its loops run to the budget there

**Status:** open
**Found:** by S2.2.4, 2026-11-26, in its runs of the steps with the model (`7aaefe6f`)
**Owner:** [P2.2b](../../../p2.2b_guided_thinking/README.md), which gives the thinking a place of its own; and
[Q-M2.31](../../open_questions/model/Q-M2.31-does-a-step-s-grammar-bound-the-comment-lines-the-model-writes.md)

## What

Every step's grammar allows a comment line between two forms, before the first and after the last (S2.2.3's
default 15): the model opens its blocks and its parts with a section header, as its examples do. **With thinking
off, the model uses them to think**: it writes what it is about to do, weighs the catalogue's entries, restates the
text, in comment lines between its forms; and **where greedy decoding loops, it loops there, to the budget**, the
grammar taking every line. Of S2.2.4's runs under the steps' grammars: `youngest`'s step 2, 12 288 tokens, 1 254
lines, 1 236 of them comments (*"Cy is younger than Bob. Ann is older than or equal to Cy. …"*); `unused`'s step 2
(656 comment lines of 736: *"We must write the rule in Step 3. So the selection will be empty …"*) and its step 3
(793 of 793, no form). A step that ran to its budget in comments ends with the forms it wrote before them, or none.

## Evidence

[S2.2.4's record](../../steps/README.md), the condition `steps`: `runs/steps/youngest/step2.out.ein`,
`runs/steps/unused/step2.out.ein`, `runs/steps/unused/step3.out.ein` (packed under `runs/`), and the tables'
generated tokens and stops.

## What it costs

A step's call costs the whole budget, about four minutes on this machine, and the record holds the model's reasoning
as comments of the program. It is also a thinking the prompt never asked for, in the one place a grammar of Ein lets
it write prose: what P2.2b's guided thinking asks for in a place of its own.
