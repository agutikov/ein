# E-57 — The clock's abort hands back nothing of what was established

**Kind:** gap
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What is missing

**The clock's abort hands back nothing of what was established**: the library's deadline is an error, where a budget's stop is a partial answer

## Evidence

`ein_infer`'s `SolveError::Deadline`

## What it costs the application

a run the clock stops has to be asked again to learn what a budget would have said
