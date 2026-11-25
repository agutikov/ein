# E-55 — A claim that names an atom the program does not have fails like a wrong answer

**Kind:** gap
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What is missing

**A claim that names an atom the program does not have fails like a wrong answer**

## Evidence

[`bob2.ein`](../../analysis/probes/p2.5/bob2.ein): `ein test --diff full` says `FAILED`, one fact missing and one extra

## What it costs the application

a gold claim written over the canonical program's names is not comparable with a model's program that names things otherwise, and the kernel does not say so
