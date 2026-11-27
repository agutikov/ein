# L-M2.43 — Word senses give the common nouns their kinds and read every name as a word

**Status:** open
**Found:** by [S2.2f.1](../../s2.2f.1_the_check.md), 2026-11-27
**Owner:** [S2.2f.4](../../s2.2f.4_verdict.md), the phase's verdict, which says whether a kind's grouping is sought
elsewhere; nothing of S2.2f.2 takes it

## What

WordNet's senses (Open English WordNet 2024, through the `wn` package) chosen among their glosses by a T5-large
trained on SemCor give the common nouns the kinds the hand's step 1 writes and no other analysis groups: coffee, tea
a *beverage*, milk and juice *food*, dog *canine*, snails *gastropod*, horse and zebra *equine*, house a *dwelling*,
chair a *seat*, the colours *colour* adjectives. They read every name WordNet holds as a common word: *Bob* a former
British monetary unit, a racing sled or a haircut (by sentence), *Strike* a labour strike, *Gold* coins,
*Chesterfield* the statesman, *the fox* the statesman Charles James Fox, *Parliaments* a legislature; *Ann*, *Cy*,
*Alice*, *Kools* and *ivory* are not in it. On the 26 kind cells: 8 right, 9 wrong; the model 18 and 7. It does not
pass the stage's rule.

## Evidence

[`check/README.md`](../../check/README.md) § 3, the `wsd` column, and § 5; `check/runs/wsd/*.json` (gitignored).

## What it costs

The kind the puzzle needs is a group (a `Pet`, a `Drink`), which an AMR concept (`dog`) does not give and a
hypernym does; a sense inventory without the names' entity types reads the names wrong. Joining them (a name's
entity type from a parse, a common noun's hypernym) is a design, not a tool, and not this stage's.
