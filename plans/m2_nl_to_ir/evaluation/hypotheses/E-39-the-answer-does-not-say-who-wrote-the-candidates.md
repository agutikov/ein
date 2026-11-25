# E-39 — The answer does not say who wrote the candidates

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**The answer does not say who wrote the candidates.** A program with no `(solve …)` form gets 11 blind candidates, `(is-a Ann Bob)` among them, and is flagged; that they came from the blind enumerator is on `ein show hyps` and `ein render h-space`, not on the read-out of a solve or in the summary

## Evidence

[`youngest_absent_nosolve.ein`](../../analysis/probes/p2.3/youngest_absent_nosolve.ein)

## What it costs the application

with E-13: the feedback cannot say *you named no hypothesis mechanism* from the answer alone
