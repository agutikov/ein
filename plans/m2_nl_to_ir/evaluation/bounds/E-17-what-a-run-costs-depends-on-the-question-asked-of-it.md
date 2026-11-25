# E-17 — What a run costs depends on the question asked of it

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**What a run costs depends on the question asked of it**, by two orders of magnitude: the ambiguous Zebra variant answers in 0.07 s under `--maximal` and 10.6 s described, with a description of 1.3 MB

## Evidence

[analysis § 3, P2.5](../../analysis/README.md#p25-and-p26--records-baselines-metrics-ablations-audit)

## What it costs the application

the loop has to choose a question per step, and a wrong choice is the whole budget
