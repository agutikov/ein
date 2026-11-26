# L-M2.20 — The check's narration of a runaway root grows with the square of its firings

**Status:** closed by S2.2.5 (`8c2037b0`), as far as memory; the time stays the clock's
**Found:** by S2.2.5, 2026-11-26, testing `einaf from-text` on a program with no fixpoint
**Owner:** [S2.2.5](../../s2.2.5_from_text.md)

## What

**`einaf::kernel::root` keeps root's narration whole**, every `fire` line at the verbose level, which the contract's
check reads a selection's activity off. A root whose rule builds a fact out of the fact it matched
([`grow.ein`](../../../../../einaf/tests/fixtures/grow.ein)) narrates each firing with a term one deeper: twenty
thousand firings are 3.2 GB of narration (`ein saturate grow.ein --max-steps 20000 --events F --events-level
verbose`), so the check of such a program held gigabytes in memory until the clock stopped root, 60 s, and `einaf
steps check grow.ein` took 77 s; the bounded call itself returns it *not decided* at the cap in about a second.

## Evidence

`einaf steps check einaf/tests/fixtures/grow.ein` before and after `8c2037b0`: 77 s and memory growing, then 61 s
with what is kept bounded.

## What was done

What is kept of root's narration is bounded, 64 MiB (`einaf::kernel::NARRATION_LIMIT`); a narration cut so is said
(`Root::narration_cut`), and `selection-inactive` is not read off it. The kernel renders the narration whatever is
kept, so a runaway root still costs the check its clock: `einaf from-text`'s tests run that fixture under a clock of
5 s. Bounding the time too would take root's saturation once with no narration first, which doubles every root's
cost, E-126's 72 s among them; not done, recorded as the stage's default.
