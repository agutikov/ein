# E-85 — The cap counts firings, an engine count

**Kind:** weakness
**Status:** open
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** none

## What

**The cap counts firings, an engine count**: a redundant firing counts, and a node resumed from root does not fire root's applications again, so the count a state takes is the engine's, not a property of the program, which the facts it derives would be

## Evidence

[Q-M2.16](../../../../../docs/decisions/answer/Q-M2.16-every-saturation-a-question-about-r-computes.md); `ein_infer::saturator`'s `steps`

## What it costs the application

a cap chosen for one program does not carry to a rewrite of it with the same facts
