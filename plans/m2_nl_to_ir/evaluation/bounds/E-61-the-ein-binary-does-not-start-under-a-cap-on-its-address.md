# E-61 — The `ein` binary does not start under a cap on its address space

**Kind:** limit
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## The limit

**The `ein` binary does not start under a cap on its address space**: with `ulimit -v 4000000` it says *Failed to initialise snmalloc*

## Evidence

`(ulimit -v 4000000; ein --version)`


Confirmed by S2.2.5, 2026-11-26: `(ulimit -v 4000000; ein --version)` prints *Failed to initialise snmalloc.* and
exits 134; under the same cap `einaf outcome examples/zebra2.ein --single`, which links the library with the system's
allocator, answers and exits 0. A bound from outside on a call's memory is EinAf's to set on its own process.

## What it bounds

a caller that bounds the kernel's memory bounds what it resides in, not what it maps; a library caller links no such allocator
