# E-59 — A module comes whole

**Kind:** limit
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## The limit

**A module comes whole**: `std.algebra` imports `std.contradiction`, so a program cannot have the one without the other

## Evidence

`stdlib/algebra.ein`'s imports

## What it bounds

*the library without the contradiction* is not a condition an ablation can put a program under
