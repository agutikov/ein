# E-35 — `--maximal` counts the full answers exactly where a description is stopped

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**`--maximal` counts the full answers exactly where a description is stopped**: the ambiguous Zebra variant lists its 32 states, exhausted, in 108 saturations

## Evidence

`ein solve examples/zebra2-minus-15.ein --maximal --json-summary F`

## What it gives the application

*how many answers* has a cheap exact question when the answers are maximal states
