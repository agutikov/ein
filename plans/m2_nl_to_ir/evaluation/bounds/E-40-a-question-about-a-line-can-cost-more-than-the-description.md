# E-40 — A question about a line can cost more than the description of R

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**A question about a line can cost more than the description of R**: `zebra2` takes 267 saturations and 11 807 facts under `--minimal`, 120 and 5 411 described, 7 and 278 under `--single`

## Evidence

`ein solve examples/zebra2.ein [--minimal | --single] --json-summary F`, the `work` key

## What it costs the application

with E-17: *the least work its question needs* is not ordered by how small the question looks
