# E-26 — No surface carries a time or a memory figure

**Kind:** gap
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none. [S2.1.3](../../p2.1_kernel_as_instrumentation/s2.1.3_feedback_object.md): the library hands over the run's wall clock, `explore::Answer::wall` (`--stats` prints it as `wall`), which the feedback object keeps as `work.run_us`; the load's and the judgement's are no value, and EinAf measures them (`work.load_us`, `ask_us`). No memory figure (E-100)

## What is missing

**No surface carries a time or a memory figure**, and `ein --version` names no commit

## Evidence

[analysis § 2 and § 3, P2.5](../../analysis/README.md#p25-and-p26--records-baselines-metrics-ablations-audit)

## What it costs the application

the application measures the clock itself and reads the commit from git
