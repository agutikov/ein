# E-52 — A parse error's repairs are in the parser's words

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**A parse error's repairs are in the parser's words**: *Insert `(`, Shift `a`, Delete `:assert`*

## Evidence

`ein solve examples/broken/unclosed_paren.ein`

## What it costs the application

whether a model can act on them is the ladder's first measurement ([S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md))
