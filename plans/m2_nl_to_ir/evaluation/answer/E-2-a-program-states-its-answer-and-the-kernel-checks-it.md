# E-2 — A program states its answer and the kernel checks it

**Kind:** strength
**Status:** standing (a strength)
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**A program states its answer and the kernel checks it**: `:expect` on a query and on a `(solve …)`, `ein test DIR --json-report`, `--diff full`

## Evidence

[audit of P2.3](../../analysis/audit_p2.3.md)

## What it gives the application

the kernel's half of the benchmark's check exists; a gold answer is a claim in the program
