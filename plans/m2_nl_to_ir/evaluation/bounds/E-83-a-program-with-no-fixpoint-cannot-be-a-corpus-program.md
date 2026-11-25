# E-83 — A program with no fixpoint cannot be a corpus program

**Kind:** limit
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## The limit

**A program with no fixpoint cannot be a corpus program**: most corpus sweeps saturate every file through readers no question bounds, so the witnesses live beside their test

## Evidence

[`bounded_saturation.rs`](../../../../ein.rs/crates/ein-cli/tests/bounded_saturation.rs)'s module doc; `timeout 5 ein saturate ein.rs/crates/ein-cli/tests/bounded/grow.ein`: killed

## What it bounds

the runaway class is held by two fixtures, not by the corpus's sweeps
