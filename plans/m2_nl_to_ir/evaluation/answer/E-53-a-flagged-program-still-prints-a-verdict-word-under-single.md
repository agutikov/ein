# E-53 — A flagged program still prints a verdict word under `--single`

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** the design: [Q-M1f.564](../../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)

## What

**A flagged program still prints a verdict word under `--single`**, below the warning

## Evidence

`ein solve examples/lattice/04_obligation_trigger_hole.ein --single`: `WARNING`, then `Ambiguity`

## What it costs the application

a caller that reads the word before the warning files a flagged program as an answered one
