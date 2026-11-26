# L-M2.10 — A program with no fixpoint costs the bounded call its clock once for the load and once a question

**Status:** open
**Found:** by S2.2.2, 2026-11-26, judging the probe's `library.off.bob1`, the first program a model wrote with no
fixpoint
**Owner:** the stage that runs the loop, [P2.4](../../../p2.4_loop/README.md), and the feedback object's page,
[`docs/einaf/03-feedback.md`](../../../../../docs/einaf/03-feedback.md#1-the-bounded-call)

## What

The bounded call sets its clock **from the call's start for the load's judgement and from each question's start for
the question** ([the feedback object § 1](../../../../../docs/einaf/03-feedback.md#1-the-bounded-call)). A program
whose root has no fixpoint is saturated by the judgement of its mechanisms and again by every question, and each
runs to the clock: the probe asked `--no-search` and `--single` under 30 seconds, and the call took 94 (the load's
judgement 31.4 s, each question 31.4 s), both objects *not decided*, `--max-time 30 s reached before the question was
decided`. The cap on firings, 2^20, did not end any of the three first: the rule builds a term one deeper at every
firing, so a firing costs more the longer the run.

## Evidence

The probe's record, [`model_runtime/probe.jsonl.gz`](../../model_runtime/probe.jsonl.gz), the line
`library.off.bob1`: its objects' `work.load_us` 31 404 123 and `work.ask_us` 31 362 480 and 31 383 861.

## What it costs

A loop that asks k questions of a runaway program waits k + 1 clocks for one verdict that the first would have
given: the first stop is not carried to the next question. Either the call stops asking once the load's judgement
or a question has stopped at the clock on root (every later question saturates the same root), or the clock is the
call's and not each question's; which is the loop's to choose.
