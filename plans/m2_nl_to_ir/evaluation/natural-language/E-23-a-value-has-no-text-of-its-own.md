# E-23 — A value has no text of its own

**Kind:** gap
**Status:** open
**Filed:** by [the analysis](../../analysis/README.md), 2026-11-25, on `458bf90b`, before any stage ran
**Owner:** EinAf's formatters ([S2.2.5](../../p2.2_formalizer/s2.2.5_from_text.md)) meanwhile; M1p's P1p.1

## What is missing

**A value has no text of its own**: the number 3 prints as `(n z 3)`, a fact that stands as a value as its term

## Evidence

the ledger, Q-M2.12


EinAf's formatters exist, S2.2.5, 2026-11-26 (`8c2037b0`, [from a text to a short answer § 3](../../../../docs/einaf/08-from-text.md#3-the-formatters)):
the decimal numeral as its digits, and the word theory's quantities by cases that are data
(`einaf/src/format/cases/words.txt`, the theory's declarations' own `:why`s); a value with no case prints as its
term, and the answer's record counts the values that still do (`answer.json`'s `terms`).

## What it costs the application

the answer is not short or readable without a formatter outside the kernel
