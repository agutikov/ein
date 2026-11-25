# E-71 — A line's place is not always in front

**Kind:** weakness
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What

**A line's place is not always in front**: `kb load error: (rule x): macro m/2 invoked with 1 args at FILE:6:20` places the call at the end, where the loader's other lines put it first

## Evidence

`ein solve examples/broken/load/macro_arity_mismatch.ein`

## What it costs the application

none to a reader of the value, whose place is the call's; a reader of the line meets two spellings
