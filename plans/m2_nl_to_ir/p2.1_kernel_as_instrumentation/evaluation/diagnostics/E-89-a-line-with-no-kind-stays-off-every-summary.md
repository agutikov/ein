# E-89 — A line with no kind stays off every summary

**Kind:** gap
**Status:** open
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** none

## What is missing

**A line with no kind stays off every summary**: `ein oracle`'s refusal of a cube past `--region-limit` (exit 2, no summary), its projection warnings, `-y`'s engine error and a usage error

## Evidence

`ein oracle examples/zebra2.ein --json-summary F`: `error: … 56 root candidates, over --region-limit 16 …`, exit 2, no `F`

## What it costs the application

a caller of the oracle tells *refused* from *too large* by the exit code and stderr
