# E-32 — Names are ASCII, and there are no numbers

**Kind:** limit
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## The limit

**Names are ASCII, and there are no numbers**: a Cyrillic atom is refused, `7` and `07` are two names, arithmetic is a theory

## Evidence

[analysis § 3, P2.2](../../analysis/README.md#p22--the-formalizer-audit)

## What it bounds

a text in another language needs its names transliterated; a number is a term the application formats
