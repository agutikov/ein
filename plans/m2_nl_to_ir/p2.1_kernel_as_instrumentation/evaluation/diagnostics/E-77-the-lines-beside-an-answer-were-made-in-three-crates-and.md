# E-77 — The lines beside an answer were made in three crates and printed in a fourth

**Kind:** weakness
**Status:** closed by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`e74139cf`)
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** **Closed** by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`e74139cf`): each is a value until it is printed, and the summary lists them

## What

**The lines beside an answer were made in three crates and printed in a fourth**, and the binary kept the load's notes as text until the run ended: no surface but stderr carried them

## Evidence

`ein-cli`'s `keep_notes` before `e74139cf`

## What it costs the application

none now
