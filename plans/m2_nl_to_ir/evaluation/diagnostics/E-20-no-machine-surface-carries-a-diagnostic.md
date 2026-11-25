# E-20 — No machine surface carries a diagnostic

**Kind:** gap
**Status:** closed by [S2.1.2](../../p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md) (`e74139cf`)
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** **Closed** by [S2.1.2](../../p2.1_kernel_as_instrumentation/s2.1.2_diagnostics_surface.md) (`e74139cf`): the run summary's `diagnostics` carry every line stderr prints, a refusal's findings and the clock's abort among them, and the summary is written for a run with no answer; `region.core_sources` carries the core's `:source` texts; `ein test --json-report`'s `error` rows carry theirs (E-68). The core under `--single` stays `null`, by [Q-M1f.29](../../../../docs/decisions/read-out/Q-M1f.29-a-question-that-prints-no-core-writes-a-dead.md); [F23](../../../followups/f23_diagnostics_in_eins_words.md)'s D-18 and L-125 closed with it

## What is missing

**No machine surface carries a diagnostic**: a parse or load refusal, a `-T` abort, a load's `note:` lines, a core's `:source`, the core under `--single`

## Evidence

[analysis § 3, P2.1](../../analysis/README.md#p21--the-kernel-as-instrumentation-audit), [audit of P2.1](../../analysis/audit_p2.1.md) § 0

## What it costs the application

the loop would read stderr
