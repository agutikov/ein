# E-14 — The kernel's word is not the problem's class

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** [F26](../../../followups/f26_consistency_and_completeness.md)

## What

**The kernel's word is not the problem's class.** `gap.ein`, whose right answer is *not enough information*, says `Solution` (the gap is a fact the program derives); `boxes12.ein`, with one answer, says `Ambiguity`

## Evidence

[audit of P2.3](../../analysis/audit_p2.3.md)

## What it costs the application

a benchmark cannot read an instance's class off the verdict; the class is the program's to state
