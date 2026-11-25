# E-84 — Four readers of root take no bound

**Kind:** limit
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## The limit

**Four readers of root take no bound**: `ein solve --monotone`, `ein show hyps`, `ein render h-space` and `ein render kb` run without end on a root with no fixpoint, and `--monotone` reads no `-T`

## Evidence

`timeout 5 ein solve ein.rs/crates/ein-cli/tests/bounded/grow.ein --monotone` (and `-T 1`), `show hyps`, `render h-space`, `render kb`: each killed at 5 s

## What it bounds

the static check first, which E-67 asks for, is unbounded on a runaway program
