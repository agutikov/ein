# E-79 — `ein saturate --max-steps N` crashes on a term nested past some 30 000

**Kind:** bug
**Status:** open
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** [S2.1.4](../../s2.1.4_bugs_the_phase_found.md), planned

## What

**`ein saturate --max-steps N` crashes on a term nested past some 30 000**: the step-limit line renders the last firing's bindings whole through a recursive printer, and the main thread's stack overflows. A crash is no exit code

## Evidence

`ein saturate --max-steps 40000 ein.rs/crates/ein-cli/tests/bounded/grow.ein`: *thread 'main' has overflowed its stack*, exit 134

## What it costs the application

any surface that prints a deeply nested fact whole can take the caller with it
