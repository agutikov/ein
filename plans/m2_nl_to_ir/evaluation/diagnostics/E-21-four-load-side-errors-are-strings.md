# E-21 — Four load-side errors are strings

**Kind:** gap
**Status:** closed by S2.1.2 (`62a8ee45`)
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** **Closed** by S2.1.2, `62a8ee45`: every refusal of the parse and the load is a list of `ein_core::Diagnostic`s, kind, severity, place and line; `CompileError` keeps none, no program reaching it

## What is missing

**Four load-side errors are strings** (`KbLoadError`, `LoadError`, `MacroError`, `CompileError`): no kind, no place

## Evidence

[audit of P2.1](../../analysis/audit_p2.1.md)

## What it costs the application

a caller of the library cannot tell one refusal from another but by its text
