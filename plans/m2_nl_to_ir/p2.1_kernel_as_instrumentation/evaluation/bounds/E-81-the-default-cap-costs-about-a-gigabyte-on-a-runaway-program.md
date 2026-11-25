# E-81 — The default cap costs about a gigabyte on a runaway program

**Kind:** limit
**Status:** open
**Filed:** by [S2.1.2](../../s2.1.2_diagnostics_surface.md), 2026-11-25, on `e74139cf`
**Owner:** none

## The limit

**The default cap costs about a gigabyte on a runaway program**

## Evidence

`ein solve ein.rs/crates/ein-cli/tests/bounded/grow.ein`: stopped at `--max-steps 1048576` after 1.16 s, peak 967 MB (gate build, this host, other tests running)

## What it bounds

a caller that runs many programs in one process wants a lower cap of its own
