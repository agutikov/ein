# E-41 — A lower bound is written as a plain number

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**A lower bound is written as a plain number**: a stopped description's `region.k_c` is an integer like any other, and only `exhausted false` and `stopped` beside it say it is a bound

## Evidence

`ein solve examples/zebra2-minus-15.ein --count --json-summary F`

## What it costs the application

a reader that takes the count without the flag reads a bound as a count
