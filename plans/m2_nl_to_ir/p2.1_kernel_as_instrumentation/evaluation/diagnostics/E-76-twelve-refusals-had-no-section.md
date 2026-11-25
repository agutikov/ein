# E-76 — Twelve refusals had no section

**Kind:** bug
**Status:** closed by [S2.1.2](../../s2.1.2_diagnostics_surface.md) (`62a8ee45`)
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** **Closed** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), `62a8ee45`: § 2.32 to § 2.35 state them

## What

**Twelve refusals had no section** on a page that says it holds every line a program can meet: a duplicate rule or macro, a missing `:match` or `:assert`, a kept name, conflicting module definitions, a macro's call at another count, a runaway expansion, an unimported `std.macro`, an import cycle, a bare `(import std)`, `:as` beside `:symbols`, a `std.*` module's file-relative import, a written `:using` loop

## Evidence

`07-diagnostics.md` before `62a8ee45`; § 1.5 named six only for how they end

## What it costs the application

a refusal with no section had no kind to be read by
