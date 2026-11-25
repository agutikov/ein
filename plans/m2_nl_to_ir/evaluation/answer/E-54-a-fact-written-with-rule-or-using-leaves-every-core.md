# E-54 — A fact written with `:rule` or `:using` leaves every core

**Kind:** weakness
**Status:** open
**Filed:** by [the re-plan](../../README.md#the-plan-as-re-planned), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** F24

## What

**A fact written with `:rule` or `:using` leaves every core**

## Evidence

[F24](../../../followups/f24_written_provenance.md)'s table

## What it costs the application

a model that writes provenance keywords empties its own core; the contract forbids them
