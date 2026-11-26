# E-80 — `--events` grows quadratically on a term that deepens at every firing

**Kind:** limit
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## The limit

**`--events` grows quadratically on a term that deepens at every firing**: every `enqueue` and `fire` line renders its bindings whole. The cap bounds firings, not a firing's narration; only the clock bounds that

## Evidence

`timeout 20 ein solve ein.rs/crates/ein-cli/tests/bounded/grow.ein --max-steps 20000 --events F`: killed at 20 s, `F` 141 MB

Met again by S2.2.5, 2026-11-26, through `einaf::kernel::root`, which keeps root's narration at the verbose level for
the contract's check: `ein saturate grow.ein --max-steps 20000 --events F --events-level verbose` writes 3.2 GB in
20 s, and the check of that program held gigabytes until its clock; EinAf now keeps 64 MiB of it
([L-M2.20](../../p2.2_formalizer/findings/einaf/L-M2.20-the-check-s-narration-of-a-runaway-root-grows-with-the-square-of-its-firings.md)),
and the time stays the clock's.

## What it bounds

provenance as feedback on a runaway program costs more than the run
