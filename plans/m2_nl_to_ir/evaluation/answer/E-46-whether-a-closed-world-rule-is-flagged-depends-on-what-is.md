# E-46 — Whether a closed-world rule is flagged depends on what is hypothesised beside it

**Kind:** limit
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## The limit

**Whether a closed-world rule is flagged depends on what is hypothesised beside it**: the same minimum by `absent` is monotone where the program says `(solve :no-search)` and flagged where candidates of the relation it reads exist

## Evidence

[`youngest_absent.ein`](../../analysis/probes/p2.3/youngest_absent.ein), [`youngest_absent_hyp.ein`](../../analysis/probes/p2.3/youngest_absent_hyp.ein)

## What it bounds

with E-31: a family's canonical program is admissible or not by its hypotheses, not by its `absent` alone
