# E-102 — A deep fact is rendered and drawn in time that grows with the square of its depth, after the run and outside the clock

**Kind:** limit
**Status:** open
**Filed:** by [S2.1.4](../../s2.1.4_bugs_the_phase_found.md), 2026-11-25, on `df75643c`
**Owner:** none

## The limit

**A deep fact is rendered and drawn in time that grows with the square of its depth, after the run and outside the clock**: what [E-99](E-99-a-deep-term-is-rendered-in-time-that-grows-faster-than-the.md) found is left once the printers keep no call stack. The read-out prints a deep term whole, the text the square of the chain's length; a view draws each nested fact as its own node, its id a digest of its whole text; a trace draws a view at each step. The clock is not read there, so a run the clock would have stopped is answered and drawn

## Evidence

`python3 analysis/probes/p2.1/deep_term.py 20000 > F.ein` ([`deep_term.py`](../../../analysis/probes/p2.1/deep_term.py)): `ein solve F.ein --timing`, read-out 3 879 ms, where the binary of `837acf6e` took 356 095 ms for the same bytes; at 5 000, 249 ms against 4 543 ms. `einaf outcome F.ein --single -T 1`, *one* after 6.3 s at 20 000 (E-99's 256 s) and 0.5 s at 5 000 (3.8 s). The program `ein-cli/tests/deep_terms.rs` writes, forty steps of 990 levels (a fact 39 600 deep): `ein render kb p.ein`, 21 s and 18 MB; fifty steps of a hundred (5 000 deep): `ein solve p.ein -r t.md`, 20 s, a step's drawing the square of the depth

## What it bounds

a program that counts by nesting answers past its clock, by seconds where the depth is tens of thousands; a view or a trace of it costs the square of its depth at each fact or step drawn
