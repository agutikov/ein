# E-99 — A deep term is rendered in time that grows faster than the square of its depth, after the run and outside the clock

**Kind:** limit
**Status:** closed by [S2.1.4](../../s2.1.4_bugs_the_phase_found.md), `df75643c`, as far as *faster than the square*: the printers walked a term on the call stack and built each level's text anew, and they keep a stack of their own now ([E-79](E-79-ein-saturate-max-steps-n-crashes-on-a-term-nested-past-some.md)); the read-out of the same program at 5 000 takes 249 ms and at 20 000 3 879 ms, the same bytes, and `einaf outcome … --single -T 1` 0.5 s and 6.3 s. What is left, a deep fact's text and drawing in the square of its depth after the run and outside the clock, is [E-102](E-102-a-deep-fact-is-rendered-and-drawn-in-time-that-grows-with-the.md)
**Filed:** by [S2.1.3](../../s2.1.3_feedback_object.md), 2026-11-25, on `0ab1aef0`
**Owner:** none

## The limit

**A deep term is rendered in time that grows faster than the square of its depth, after the run and outside the clock**: the read-out renders each state's facts and each query's rows, the clock is not read there, and a run the clock would have stopped is answered

## Evidence

`python3 analysis/probes/p2.1/deep_term.py 5000 > F.ein` ([`deep_term.py`](../../../analysis/probes/p2.1/deep_term.py)): `ein solve F.ein --timing`, route 5.6 ms, read-out 4 551 ms; `einaf outcome F.ein --single -T 1`, *one* after 3.8 s. At 20 000 steps: `einaf outcome … --single -T 1`, *one* after 256 s, peak 77 MB, no stack overflow on the main thread; `ein solve … -T 20` killed at 100 s

## What it bounds

a program that counts by nesting returns minutes past its clock; four times the depth cost 67 times the time
