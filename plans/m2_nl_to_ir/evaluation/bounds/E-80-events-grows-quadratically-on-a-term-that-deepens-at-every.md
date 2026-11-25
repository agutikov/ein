# E-80 — `--events` grows quadratically on a term that deepens at every firing

**Kind:** limit
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## The limit

**`--events` grows quadratically on a term that deepens at every firing**: every `enqueue` and `fire` line renders its bindings whole. The cap bounds firings, not a firing's narration; only the clock bounds that

## Evidence

`timeout 20 ein solve ein.rs/crates/ein-cli/tests/bounded/grow.ein --max-steps 20000 --events F`: killed at 20 s, `F` 141 MB

## What it bounds

provenance as feedback on a runaway program costs more than the run
