# L-M2.40 — Trained parsers read the six texts right on most of the cells where step 1 misread them

**Status:** open
**Found:** by [S2.2f.1](../../s2.2f.1_the_check.md), 2026-11-27, in [its table](../../check/README.md#2-the-counts)
**Owner:** [S2.2f.3](../../s2.2f.3_step1_with_the_analysis.md), which measures whether step 1 given it does better;
[S2.2f.2](../../s2.2f.2_the_analysis_written.md) wrote what passed (2026-11-27): the three AMR parsers' graphs and
spaCy's entities, recorded for the six texts under `einaf/tests/fixtures/analysis/`, in three forms

## What

On 68 cells (a sentence of the six texts and a failure class with a case in it), three AMR parsers (amrlib's XFM
and SPRING, IBM's transition parser) and two UD parses with their entities (Stanza, spaCy) are right on 50 to 56
cells each, wrong on 3 to 12; the model's step 1 was right on 41 (43 on the reading alone) in S2.2.2's probe and on
27 (28) of 61 in S2.2.4's steps, wrong on 20 to 23. Every parser keeps each number in its fact with its noun (8 of 8
cells, the probe 0 or 2), states each denial (where S2.2.4's step 1 wrote two statements, L-M2.18), reads *next to*
and *older than* with both their arguments in order (L-M2.24's fourth defect), gives the colours as properties of
the houses (its first), and invents nothing (where the probe wrote three placements). The passive's roles: SPRING 4
of 4, XFM 3, the model 1 and 0.

What they read worse: a pronoun (*He* is Bob), which no sentence parse resolves and the model did; an elided noun
(*He gave 2*), left a bare quantity; and *Chesterfields*, the one brand the model read right.

## Evidence

[`check/README.md`](../../check/README.md) § 2 to § 4; the cells [`check/cells.tsv`](../../check/cells.tsv), the
counts `python3 check/tally.py`; the outputs under `check/runs/` (gitignored).

## What it costs

A tool's right cell is the reading present in its analysis, not a formalization made from it: whether the model
uses it, or follows a parse that is wrong (L-M2.41), is S2.2f.3's to measure. The phase goes on by its rule.
