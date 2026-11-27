# E-98 — The clock is not read where nothing fires, and the cap counts only firings

**Kind:** limit
**Status:** open
**Filed:** by S2.1.3, 2026-11-25, on `0ab1aef0`
**Owner:** none

## The limit

**The clock is not read where nothing fires, and the cap counts only firings**: a join that never fires runs between two reads of the clock, and the blind enumerator builds H before the first. Each ends: the call returns, late

## Evidence

[`wide_join.ein`](../../analysis/probes/p2.1/wide_join.ein) (a cycle of five edges asked of the complete bipartite graph of 40 and 40 nodes): `einaf outcome wide_join.ein --single -T 1` stopped after 4.7 s, unbounded 20.1 s (*one*), `--max-steps 1000` 4.5 s; [`wide_h.ein`](../../analysis/probes/p2.1/wide_h.ein) (a relation of two places over 400 names, H 159 600 candidates): `-T 1` 3.5 s, unbounded 10.6 s (*several*), cap and budget read nothing; `ein solve` the same within 0.3 s

## What it bounds

a model's program can hold the call several times its clock; a wider graph or more names, longer

## Since S2.2e.3

The clock is read in one more place, a dead node's core search (`3a5c162b`, E-126's close); the two works this entry
names, a join that fires nothing and the blind enumerator building H, are as they were.
