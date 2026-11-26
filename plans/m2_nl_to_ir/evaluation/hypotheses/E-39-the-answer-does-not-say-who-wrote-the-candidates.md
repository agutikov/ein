# E-39 — The answer does not say who wrote the candidates

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**The answer does not say who wrote the candidates.** A program with no `(solve …)` form gets 11 blind candidates, `(is-a Ann Bob)` among them, and is flagged; that they came from the blind enumerator is on `ein show hyps` and `ein render h-space`, not on the read-out of a solve or in the summary

## Evidence

[`youngest_absent_nosolve.ein`](../../analysis/probes/p2.3/youngest_absent_nosolve.ein)

Confirmed by S2.2.1, 2026-11-26: EinAf's feedback object holds no writer either, and the contract's tests read it off
`ein show hyps` ([L-M2.6](../../p2.2_formalizer/findings/einaf/L-M2.6-the-bounded-call-returns-no-writer-of-h.md)).

Read around by S2.2.4, 2026-11-26: `einaf::kernel::root` calls `ein_infer::structure::hypotheses_with`, a public
function, which returns the writer (`Hypotheses::rung`) at the price of root's saturation a second time beside the
bounded call; the read-out still says nothing of it, and a run of the steps records it beside each step's feedback.

## What it costs the application

with E-13: the feedback cannot say *you named no hypothesis mechanism* from the answer alone
