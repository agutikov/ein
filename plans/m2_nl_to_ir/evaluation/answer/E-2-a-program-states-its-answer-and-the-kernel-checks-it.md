# E-2 — A program states its answer and the kernel checks it

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**A program states its answer and the kernel checks it**: `:expect` on a query and on a `(solve …)`, `ein test DIR --json-report`, `--diff full`

## Evidence

[audit of P2.3](../../analysis/audit_p2.3.md)

Confirmed by S2.2.2, 2026-11-26, on programs a model wrote: shown the corpus's word problems with their `:expect`, the model wrote one beside its query in seven of the eleven programs of the theory-given condition that loaded with thinking off, stating the answer it expected; the kernel refuted six of the seven and held one (`theory-given.off.gap_fact`), under `--single` ([the probe's record](../../p2.2_formalizer/model_runtime/README.md)). The model's own answer, stated where the kernel checks it, is a check the application gets for nothing.

## What it gives the application

the kernel's half of the benchmark's check exists; a gold answer is a claim in the program
