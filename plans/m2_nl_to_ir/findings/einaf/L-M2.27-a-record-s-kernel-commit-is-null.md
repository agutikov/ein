# L-M2.27 — A record's `kernel.commit` is `null`

**Status:** open
**Found:** by [the analysis of P2.2 against its goals](../../p2.2_formalizer/report/ANALYSIS.md#5-where-this-reading-differs-from-the-reports),
2026-11-26, reading a replayed run's `config.json`; filed at the phase's close, 2026-11-27
**Owner:** [S2.5.1](../../p2.5_harness/s2.5.1_experiment_record.md), whose record names the repository's commit
(git's, since neither the binary nor the library states one)

## What

`einaf from-text --record` writes `config.json` with the model's file by its SHA, the templates, the excerpt, the
catalogue and the library hashed, and **`kernel.commit` `null`** in every one of the phase's records. The record's
layout is S2.2.5's stand-in for S2.5.1's, which has `repository.commit` and `dirty`.

## Evidence

`plans/m2_nl_to_ir/p2.2_formalizer/from_text/runs/*/*/config.json` and the replays under `report/out/from-text/`:
the key reads `null` in each, read again at the close.

## What it costs

Level B's fourth criterion asks every run to name Ein's commit. Nothing of P2.2's numbers changes: no code changed
between the runs and the report (`git diff --stat d124b7b2 HEAD -- einaf ein.rs stdlib` was empty).
