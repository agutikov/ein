# E-93 — A fixture banks one finding of two

**Kind:** weakness
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What

**A fixture banks one finding of two**: `examples/broken/load/expect_unknown_relation.expected` holds one line, and the refusal has two `claim` findings, the second the goal's

## Evidence

`ein solve examples/broken/load/expect_unknown_relation.ein --json-summary F`

## What it costs the application

a refusal's later findings are held by no `.expected`
