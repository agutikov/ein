# E-86 — `ein test` asks a cut file twice

**Kind:** weakness
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What

**`ein test` asks a cut file twice**: the least question stops at the cap, and the describing question, asked of a second load, stops there again

## Evidence

`ein test ein.rs/crates/ein-cli/tests/bounded/grow_node.ein --max-steps 1000`: `NOT CHECKED`, *asked first: --single, which stopped at --max-steps 1000*

## What it costs the application

a runaway program costs two caps under `ein test`
