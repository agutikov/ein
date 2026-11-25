# E-78 — The `.einb` container's own refusals had no kind

**Kind:** gap
**Status:** closed by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`e74139cf`)
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** **Closed** by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`e74139cf`): each is § 2's `unreadable`, a file this build cannot read

## What is missing

**The `.einb` container's own refusals had no kind**: its magic, its version, its digest, a truncated or malformed section, a build with no `einb` feature

## Evidence

`07-diagnostics.md` § 2 before `e74139cf`

## What it costs the application

a refused container's summary could say why in no kind
