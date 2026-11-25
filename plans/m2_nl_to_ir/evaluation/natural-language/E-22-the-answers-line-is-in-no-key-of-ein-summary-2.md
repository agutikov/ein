# E-22 — The answer's line is in no key of `ein-summary/2`

**Kind:** gap
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none; [M1p](../../../m1p_ein_to_natural_language/README.md)'s ground

## What is missing

**The answer's line is in no key of `ein-summary/2`**: the summary holds a query's rows, not its `:goal-text` filled. The library has it, each row rendered, beside the rows ([`04-api.md`](../../../../docs/ein/ein.rs/04-api.md))

## Evidence

the ledger, [Q-M2.12](../../open_questions/answer/Q-M2.12-what-prints-the-answer.md); `ein_infer::read_out`'s class

## What it costs the application

nothing to a caller of the library; a caller of the binary reads the answer off stdout
