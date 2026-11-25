# E-16 — A core is an explanation

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** [F22](../../../followups/f22_exact_core_past_the_cap.md)

## What

**A core is an explanation**: not minimal, may vary with rule order, `[]` is a legal one, and under `--single` it is `null`

## Evidence

[analysis § 2](../../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk)

## What it costs the application

a core as feedback is not stable from run to run, and the cheapest question does not give one
