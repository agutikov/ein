# E-67 — A flagged program is answered in full before it can be filed as flagged

**Kind:** weakness
**Status:** closed by [S2.1.3](../../s2.1.3_feedback_object.md) (`0ab1aef0`), for the application
**Filed:** by [S2.1.1](../../s2.1.1_tree_and_outcomes.md), 2026-11-25, on `0effa7b0`
**Owner:** [S2.1.3](../../s2.1.3_feedback_object.md): ask the static check first (`ein_infer::explore::check_monotone`, root's saturation) where a question needs the cube. **Closed for the application** by [S2.1.3](../../s2.1.3_feedback_object.md): EinAf asks a question that needs the cube under a limit of 0, which only a flagged program reads, so the run reads the static check at root and stops at once on a flagged one and is the run asked on every other; `terminus.ein` described in 8.4 s before and 0.008 s after (`--maximal` 8.1 s and 0.008 s, `--count` 0.58 s and 0.007 s; release, one job, AC), and the corpus sweep needs no lowered limit. `ein solve` still computes the cube

## What

**A flagged program is answered in full before it can be filed as flagged.** The flag is known at root, by the static check, and the outcome is *flagged* whatever the answer; but a question that needs the cube computes it, up to `--region-limit`

## Evidence

`einaf outcome examples/saturation/square-unique/terminus.ein`: 16.0 s for *flagged*; `--region-limit 4` or `--single`: 0.016 s, 0.011 s; `ein solve … --monotone`: 0.011 s (release, one job, AC). Four corpus programs cost 6 to 9 s a question this way

## What it costs the application

the loop pays the cube of a program it will not read; EinAf's corpus sweep runs at `--region-limit 4` to stay at 3.6 s
