# L-M2.18 — A sentence's polarity and a thing's kind go wrong in step 1 with nothing to see them

**Status:** open
**Found:** by S2.2.4, 2026-11-26, in its runs of the steps with the model (`7aaefe6f`)
**Owner:** [S2.4.4](../../../p2.4_loop/s2.4.4_faithfulness.md), the faithfulness judge; the contract's check has no
reading of either

## What

**Step 1 writes a denial as a statement**, and **a person as a chair**, and every check passes. The seating's step 1,
under its grammar, writes *"Cy does not sit in the first chair."* as `(sits Cy First :source "Cy does not sit in the
first chair.")` and *"Bob does not sit in the middle chair."* likewise, the stored negation the prompt asks for (and
its examples show) dropped; and *"Ann, Bob and Cy sit in a row of three chairs."* as six memberships of one kind,
`(instance Ann Chair …)` beside `(instance First Chair …)`. Each fact's `:source` is a sentence of the text, so the
contract's first finding is silent; the seventh finds step 2's `(instance Ann Person …)` written again otherwise, which
is the kind corrected, not the polarity. The kernel answers the program as written: root is dead at step 2, and the
core names the slot rule that found more people than chairs.

## Evidence

[S2.2.4's record](../../steps/README.md): `runs/steps/seating/step1.out.ein`, and its findings by step in `run.json`
(step 1: `undeclared` six, `instance` not yet declared; steps 2 and 3: `rewritten` three).

## What it costs

A wrong fact with the right sentence is the faithfulness judge's to read (a negation word in the sentence and no
`not` in the fact; two kinds named in one sentence and one written), and the loop's to repair, the facts being the
part a repair changes last.
