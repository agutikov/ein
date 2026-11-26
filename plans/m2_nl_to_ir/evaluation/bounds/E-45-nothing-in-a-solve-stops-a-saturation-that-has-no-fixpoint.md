# E-45 — Nothing in a solve stops a saturation that has no fixpoint

**Kind:** limit
**Status:** closed by S2.1.2, T2.1.2.5b (`83b82833`)
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## The limit

**Nothing in a solve stops a saturation that has no fixpoint**: not `-T`, not `--region-budget`. A rule that nests its conclusion one deeper than its premise runs until it is killed; `ein saturate --max-steps` stops it

## Evidence

[`grow.ein`](../../analysis/probes/p2.7/grow.ein): `timeout 20 ein solve grow.ein --single -T 2`, and `--no-search -T 1`, and `--single --region-budget 1000`, each ended by the timeout

Its closing held on the first programs a model wrote (S2.2.2, 2026-11-26): the probe's `library.off.bob1` concludes `(has ?giver (sub ?have ?given) ?kind ?time)` from `(has ?giver ?have ?kind ?time)`, a fact one deeper than the one it matched, and the bounded call returned it *not decided* at its clock of 30 seconds, under `--no-search` and `--single` alike; the cap on firings, 2^20, did not stop it first ([the probe's record](../../p2.2_formalizer/model_runtime/README.md)).

And again by S2.2.5, 2026-11-26: `einaf from-text`, its program the one shot's, on
`einaf/tests/fixtures/grow.ein`, prints *No answer: not decided.* with *stopped by the bound steps in a saturation of
root* beneath it, within the call's bound; `ein solve --max-steps` exists, so *only `ein saturate --max-steps` stops
it*, the re-plan's line, no longer holds. What the clock does not stop is
[E-126](E-126-a-root-that-dies-on-a-link-several-rules-derive-takes-its-time-past-every-clock.md).

## What it bounds

a model's program can take the caller with it. **Closed** by S2.1.2, T2.1.2.5b (`83b82833`), by [Q-M2.16](../../open_questions/kernel/Q-M2.16-what-of-the-kernel-may-change-inside-m2.md): the clock and a cap on firings, `--max-steps` (default 2^20), reach every saturation of a question about R, root's first; past the cap the run is not decided and establishes nothing of R, at the clock it aborts as before; the decision is [`Q-M2.16`](../../../../docs/decisions/answer/Q-M2.16-every-saturation-a-question-about-r-computes.md). The word of its own for *did not stop* and a bound on the saturation by default stay [M1g](../../../m1g_logic/README.md)'s; [F26](../../../followups/f26_consistency_and_completeness.md)
