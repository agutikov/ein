# L-M2.45 — The facts form is four times the text forms in the model's tokens, and every form fits step 1's prompt

**Status:** open
**Found:** by [S2.2f.2](../../s2.2f.2_the_analysis_written.md), 2026-11-27
**Owner:** [S2.2f.3](../../s2.2f.3_step1_with_the_analysis.md), whose arms are these forms; [Q-M2.51](../../open_questions/formalizer/Q-M2.51-does-every-fact-of-the-analysis-quote-its-sentence.md)

## What

On the Zebra puzzle, SPRING's graphs, in `qwen3.5:4b`'s tokens (`llama-tokenize` on the CPU): **PENMAN 1 249, a node a
line 1 197, the facts 5 689**; the block step 1's prompt holds, the form with its paragraph, 1 465, 1 440 and 5 959.
IBM's: 1 230, 1 448 (its words in brackets) and 7 735. The named entities beside add 100 to 300. Step 1's prompt on
the Zebra puzzle was 4 793 tokens with no analysis (the recorded run of the model, `zebra-contradiction`), so a text
form makes it about 6 300 and the facts about 10 800, of the 53 248 the context leaves beside the budget of 12 288
generated tokens: **every form fits**, so the stage's rule (the form the prompt's budget holds for the Zebra text) does
not choose, and the default is PENMAN by another reason (the stage's record, Defaults). The facts form's length is its
sentences: every fact quotes its sentence as `:source`, 2 354 of its 5 689 tokens (the same program with no `:source`
is 3 335).

## Evidence

`python3 plans/m2_nl_to_ir/p2.2f_text_analysis/tools/write_forms.py --model GGUF`, its last table, every text, every
parser, every form, with and without the entities; [the stage's record](../../s2.2f.2_the_analysis_written.md#record).

## What it costs

The facts arm gives the model four times the reading of the text arms for the same graph, most of it the sentences
again; whether that helps or drowns step 1 is S2.2f.3's to measure, and how much of it to keep is
[Q-M2.51](../../open_questions/formalizer/Q-M2.51-does-every-fact-of-the-analysis-quote-its-sentence.md).
