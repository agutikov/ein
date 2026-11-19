# Part 2 — `open`: what a state owes, and the hypotheses

[← the bundle](../README.md) · [← the phase](../../README.md) · [your answers](ANSWERS.md)

**Three items are open, each a folder here with its page, its programs and their runs, and each with a default.**
They came out of your answers of 2026-11-19 to this part's first three items. They wait with their part: nothing of
part 1 needs them.

## What is left for you

| # | question | what it asks | the default |
|---|---|---|---|
| [**44**](44-Q-M1f.566/README.md) | Q-M1f.566 | how the *must* is emitted, beside the hypotheses' `(open H)` | (b): a held question with the hole's domain in its place, `(owe seats Ann Chair)`; *discharged* read at the fixpoint |
| [**45**](45-Q-M1f.567/README.md) | Q-M1f.567 | the `hrule` form dropped for a rule that concludes `(open F)` | (b): the form goes, its activators stay in `(solve :hrules …)`. It revises your *"leave all existing options … as is"* |
| [**46**](46-Q-M1f.568/README.md) | Q-M1f.568 | four cells of the table of `absent`, `open` and `not` | the direct `(absent (absent P))` refused; `(open (open P))` refused; `(not (open P))`, `(open (not P))` and a written `(open P)` legal |

Write each answer in [`ANSWERS.md`](ANSWERS.md); an item left blank keeps its default.

## What to read beside them

| page | what it is |
|---|---|
| [`absent_open_not.md`](../../absent_open_not.md) | the table you asked for: `absent`, `open` and `not`, each inside each, as today's binary reads it, where each cell is defined, your four cells against it, and the table two existing rules give once `(open P)` is a fact. Item 46 is its last section on programs |
| [`open_and_owe.md`](../../open_and_owe.md) | your four questions of item 34, answered: what an obligation emits today; how a requirement is counted and could be emitted; the connectors; and whether the `hrule` form can go. Items 44 and 45 are its second and fourth on programs |

## What was ruled, and is out of the bundle

| item | question | ruled | where it is now |
|---|---|---|---|
| 32 | Q-M1f.390 | an obligation emits `(open H)`, no justification in it; the engine records where it was emitted for itself; `absent` reads it as any fact | its page and programs, kept as a survey the part's stages cite: [`q-m1f.390_what_an_obligation_emits/`](../../q-m1f.390_what_an_obligation_emits/README.md) |
| 34 | Q-M1f.549 | `(open …)` is the context for hypotheses, *may*; an obligation is *must*, emitted another way | the same: [`q-m1f.549_hypotheses_as_facts/`](../../q-m1f.549_hypotheses_as_facts/README.md) |
| 39 | Q-M1f.560 | (b): `std.debts` is merged now. **Done**: `edbe6c3b`; none of its programs is flagged, and no other program's answer moves | in `git log` |

Each answer is in your words in [the phase's README](../../README.md#the-answers-to-part-2s-items) and under its
question in the ledger.

**The binary.** The three items run on `build/review-bin/ein-head-edbe6c3b`, the gate build of master at
`edbe6c3b`, with the checkout's `stdlib/`; each item's `tools/run.sh` re-takes its `runs/`.
