# E-117 — A fence around a program is the parse's only error, each fence line named with the repair that deletes it

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by S2.2.2, 2026-11-26, on `1be58a36`
**Owner:** —

## What

**A model answers with its program in a Markdown fence**, ```` ```ein ```` above it and ```` ``` ```` below, and
the parse of the answer read whole refuses exactly the two fence lines and nothing else: each is a finding of the
kind `parse` at its line and column 1, *unexpected ```` ```ein ````: a name*, with the repair *Delete ```` ```ein
````*, and the parser's recovery reads every form between them as it would read the block alone. A backtick is a
character a name may hold, so a fence line is a name standing where a form goes, which the grammar refuses and its
recovery deletes. The generation `first-experiment.off.zebra` of the probe's record: 213 lines, the two findings
at 1:1 and 213:1, and the block, read alone, loads and answers *one*.

## Evidence

The probe's record, `plans/m2_nl_to_ir/p2.2_formalizer/model_runtime/probe.jsonl.gz`, the line
`first-experiment.off.zebra`, its reading `whole`; `einaf/tests/prompts.rs`'s
`a_fenced_answer_read_whole_is_refused_at_its_fence_lines_only` holds it on a small program. The probe's report
counts, condition by condition, the runs whose whole reading failed at its fence lines only.

## What it serves

A model's most common wrapping costs the application nothing it cannot read off the kernel's values: the findings'
places are the fence's lines and their repairs delete them, so EinAf's reading of the fenced block and the kernel's
own recovery agree. A loop that hands the model the refusal hands it a sentence that says what to do.
