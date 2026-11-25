# E-94 — The library and the binary say the same of every run EinAf compared

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by [S2.1.3](../../s2.1.3_feedback_object.md), 2026-11-25, on `0ab1aef0`
**Owner:** none

## What

**The library and the binary say the same of every run EinAf compared.** The feedback object, built from the library's values alone, agrees with `ein solve --json-summary` of the same run on 949 runs, every question the 917 corpus programs that are not `slow` declare: the outcome, the readings, the states, every query's classes and rows and claim, the program's claim, root's debts, the owing frontier, the core with its sources, the work's counts, and the diagnostics one for one; of the 61 flagged objects, the flag and the static check's sites. No disagreement but the spelling of a path (E-101)

## Evidence

`cargo test --manifest-path einaf/Cargo.toml --profile gate --test corpus -- --nocapture` (`every_object_says_what_the_binary_says_of_the_same_run`), 5.2 s for the sweep with the binary's runs, 16 workers

## What it gives the application

the object can be trusted as the kernel's word, and the binary is the guard against the two drifting
