# L-M2.3 — EinAf's load judged the hypothesis mechanisms with no bound

**Status:** closed by [S2.1.3](../../s2.1.3_feedback_object.md#what-was-built) (`0ab1aef0`)
**Found:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md#einaf-t2127), T2.1.2.7, 2026-11-25
**Owner:** S2.1.3

## What

S2.1.2's record, *For S2.1.3*, verbatim: *"**The judgement is unbounded in EinAf**: `Program::load_text` runs
`judge_loaded` outside any `saturator::bounded`, so a program whose root has no fixpoint hangs the load. S2.1.3 should
run the load's judgement under the question's bound, `ein_infer::saturator::bounded( Bound { deadline, max_steps },
|| judge_loaded(…))`, as `ein solve` does by setting the switch before the load; a judgement the bound stops returns
`Ok` with no notes (`check` gives up on a saturation error), so the question's own run then reports the stop."*

A finding of EinAf's, not of Ein's: `ein solve` and `ein test` set the bound before the load, and the judgement Ein
runs for them is bounded.

## Evidence

S2.1.3's record, verbatim: *"the load's judgement runs under `saturator::bounded(Bound { deadline, max_steps })`, its
clock from the call's start, and each question under its own clock from its start. A judgement the bound stops judges
nothing and the question reports the stop (`tests/fixtures/grow_judged.ein`: the load hung before, killed at 10 s;
now *not decided* at the cap, 0.01 s)."*

## What it cost

Until `0ab1aef0`, a model's program whose root has no fixpoint, with a hypothesis mechanism to judge, would have
held EinAf's call in its load, past every bound the call was given.
