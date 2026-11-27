# L-M2.41 — The Zebra puzzle's brand names and its passive of *drink* are where trained parsers misread

**Status:** open
**Found:** by [S2.2f.1](../../s2.2f.1_the_check.md), 2026-11-27 (the phase's risk *the genre*)
**Owner:** [S2.2f.3](../../s2.2f.3_step1_with_the_analysis.md), whose arms show whether the model follows a wrong
parse; [S2.2f.2](../../s2.2f.2_the_analysis_written.md), which writes the analysis with its misreadings in it

## What

Of the parsers' wrong cells, all of IBM's twelve and most of the others' are on the Zebra puzzle, in two places.

- **The brands.** *Chesterfields*: a road (XFM, SPRING, the smoking located at it), a local region (IBM), a person
  (Stanza), the statesman (the word senses). *Parliaments*: the legislature (every AMR parser, the word senses), an
  organisation (Stanza, spaCy). *Old Gold*: gold, modified by old (every AMR parser), an organisation (Stanza).
  *Kools*: the word `kool` with no kind (XFM), Kool-Aid, a drink's brand, renamed (IBM). *Lucky Strike*: a company
  (XFM, IBM; XFM's smoker smokes nothing). Right: spaCy's entities, *Old Gold*, *Chesterfields* and *Lucky Strike*
  as products (*Parliaments* an organisation, *Kools* none); SPRING, *Kools* and *Lucky Strike* as products.
- **The passive of *drink*.** *Coffee is drunk in the green house* and *Milk is drunk in the middle house*: IBM's AMR
  reads intoxication (`drunk :domain (person :name "Coffee")`, `drunk :domain milk-02`, PropBank's *exploit*), and
  so do both UD parses (*drunk* an adjective, the drink its subject); XFM reads coffee right and milk the drinker;
  SPRING both right. *Kools are smoked* and *the horse is kept*: every parser right (IBM with `smoke-01`, *emit
  smoke*). The model had made both *drunk* sentences active with the drink the drinker: where both are wrong, the
  parsers are wrong otherwise.

## Evidence

[`check/README.md`](../../check/README.md) § 3 (cells z5, z10 to z14, z21, z25, z27, z28) and § 5; the senses read
from PropBank's frame files.

## What it costs

A wrong parse given to the model is a wrong premise it may follow (the phase's risk *the analysis may mislead*); the
brands are proper names a puzzle invents or borrows, and the benchmark's texts will have their own. The three AMR
parsers disagree on 11 cells, all but two (*by how many*, *Ann sits next to Bob*) on the Zebra puzzle, which marks a
misreading there; on 4 they are wrong alike (*Old Gold*'s two cells, *Chesterfields*, *Parliaments*), and nothing
in their agreement shows it.
