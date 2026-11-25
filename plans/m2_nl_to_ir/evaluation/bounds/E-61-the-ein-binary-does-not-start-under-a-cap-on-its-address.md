# E-61 — The `ein` binary does not start under a cap on its address space

**Kind:** limit
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## The limit

**The `ein` binary does not start under a cap on its address space**: with `ulimit -v 4000000` it says *Failed to initialise snmalloc*

## Evidence

`(ulimit -v 4000000; ein --version)`

## What it bounds

a caller that bounds the kernel's memory bounds what it resides in, not what it maps; a library caller links no such allocator
