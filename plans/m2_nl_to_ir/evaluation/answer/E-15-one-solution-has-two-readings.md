# E-15 — *One solution* has two readings

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** [Q-M2.10](../../open_questions/answer/Q-M2.10-what-does-ein-certifies-a-unique-model-mean-on-r.md)

## What

***One solution* has two readings.** R holds partial commitments where a program states no obligation, so one program is `Ambiguity` over all of R and `Solution` over its maximal line; and there are two counts, of states and of commitments. A reading that is pruned with nothing owed leaves ∅ beside the survivor, with no word that a reading was left unchosen

## Evidence

[analysis § 2](../../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk); `ein solve examples/lattice/03_state_hash_collision.ein` under `--single` and under `--maximal --single`; [`two_readings_no_owed.ein`](../../analysis/probes/p2.4/two_readings_no_owed.ein)

## What it costs the application

*certified unique* is not one test
