# E-13 — A program that names no hypothesis mechanism is answered by the blind enumerator

**Kind:** weakness
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** none

## What

**A program that names no hypothesis mechanism is answered by the blind enumerator.** A ten-line probe: 83 candidates and a summary of 69.6 MB, exit 0

## Evidence

[analysis § 2](../../analysis/README.md#2-what-the-kernel-says-today-where-the-plan-expects-the-walk), [audit of P2.2](../../analysis/audit_p2.2.md); [`no_mechanism.ein`](../../analysis/probes/p2.2/no_mechanism.ein), five forms: `Ambiguity`, and no word that no mechanism was stated

Confirmed by S2.2.1, 2026-11-26: the five forms are `einaf/contract/youngest/no-theory.ein`, held by
`einaf/tests/contract.rs`'s `the_worked_example_turns_on_its_mechanism_line`: 7 blind candidates, 128 states, *several*,
and no diagnostic that names a mechanism. What the steps' partial programs show beside it is
[E-114](E-114-a-partial-program-s-h-is-set-by-its-relations-count-of-places.md).

## What it costs the application

an under-specified program does not sit idle: it costs, and answers a question nobody asked
