# E-60 — The answer turns on one line that states the lattice

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**The answer turns on one line that states the lattice.** One small program in three forms: with `(solve :no-search)` it is `Solution`; with the line deleted it is flagged and `Ambiguity`; with the relation its rule reads closed instead it is monotone and `Ambiguity`, the blind enumerator supposing facts of the relation the rule concludes

## Evidence

[`contract_example.ein`](../../analysis/probes/p2.2/contract_example.ein), [`contract_example_no_mechanism.ein`](../../analysis/probes/p2.2/contract_example_no_mechanism.ein), [`contract_example_closed.ein`](../../analysis/probes/p2.2/contract_example_closed.ein), each under `--single`

## What it costs the application

the hypothesis mechanism is a part of the formalization a model has to get right, and the contract makes it one
