# L-M2.28 — The prompts' first example is the Zebra puzzle's theory in miniature

**Status:** open
**Found:** by [the analysis of P2.2 against its goals](../../p2.2_formalizer/report/ANALYSIS.md#5-where-this-reading-differs-from-the-reports),
2026-11-26, reading the three prompt examples; filed at the phase's close, 2026-11-27
**Owner:** [S2.3.4](../../p2.3_benchmark/s2.3.4_splits.md), which takes the list of what a prompt quotes and names
the families an example shows; [S2.2d.6](../../p2.2d_formalizer_corrected/s2.2d.6_run_again_and_nearest_miss.md),
which writes the prompts' next version

## What

S2.2.4's default 7 calls its three examples *families no text of the phase has*, and in the same sentence `shops`
*an attribute puzzle over the Zebra puzzle's theory*; `buying` and `ages` are word problems over the calculator.
`einaf/prompts/examples/shops/` is the Zebra puzzle's theory written the hand's way: *the blue stall* is `Blue`, a
`Color`; the passive stays; *next to* is a relation of its own; the query has two variables; step 3 ends on the
obligations and no `solve` form. **No solution leaks**, so the phase's acceptance item holds; but the one shot on
the Zebra text is not a run on an unseen kind of text, and it missed all the same: the model's steps 2 and 3
repeat that example's comments nearly word for word and its selections with the names changed, and every reading
decision the example also shows went wrong on the new sentences.

## Evidence

`einaf/prompts/examples/shops/step1.ein` to `step3.ein` beside
`plans/m2_nl_to_ir/p2.2_formalizer/steps/runs/steps/zebra/step2.out.ein` and `step3.out.ein`;
[S2.2.4's record](../../p2.2_formalizer/s2.2.4_passes.md#defaults-taken), default 7.

## What it costs

A number of the attribute family taken with these prompts is a number with a worked example of its theory in the
prompt, and a number of the word problems likewise; the Level B run says so a family. It also says what the model
does with an example: it follows a shape and does not carry over a reading.
