# E-82 — A run the cap stops hands back nothing it established

**Kind:** gap
**Status:** open
**Filed:** by S2.1.2, 2026-11-25, on `e74139cf`
**Owner:** none

## What is missing

**A run the cap stops hands back nothing it established**: at a node's stop root's standing, H and the static check are known and are dropped; the summary's `region.root`, `root` and `monotonicity` are `null`

## Evidence

`ein solve ein.rs/crates/ein-cli/tests/bounded/grow_node.ein --max-steps 1000 --json-summary F`: `region.stopped` `{why: steps, steps: 1000, saturation: node}`, `region.root`, `root`, `monotonicity` `null`

## What it costs the application

with E-57: a stopped run has to be asked again, under another question, to learn what it had
