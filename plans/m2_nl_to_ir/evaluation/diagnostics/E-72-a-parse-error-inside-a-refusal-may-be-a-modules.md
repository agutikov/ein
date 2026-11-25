# E-72 — A parse error inside a refusal may be a module's

**Kind:** weakness
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What

**A parse error inside a refusal may be a module's**: `KbLoadError::parse()` is the imported module's report where the module does not parse, its place in the module

## Evidence

`ein solve examples/broken/load/reserved_open_symbols.ein --json-summary F`: one `parse` finding placed in `reserved_open_direct.ein:16:8`

## What it costs the application

a caller that shows the model its own text has to read the place's file before it quotes a line
