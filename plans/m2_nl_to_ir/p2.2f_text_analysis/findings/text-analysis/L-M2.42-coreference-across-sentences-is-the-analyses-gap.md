# L-M2.42 — Coreference across sentences is the analyses' gap

**Status:** open
**Found:** by [S2.2f.1](../../s2.2f.1_the_check.md), 2026-11-27
**Owner:** [S2.2f.3](../../s2.2f.3_step1_with_the_analysis.md), which reads whether step 1 still resolves a pronoun
given an analysis that does not. [S2.2f.2](../../s2.2f.2_the_analysis_written.md) decided what the analysis says of
one (2026-11-27): **what the sentence's parser wrote, and nothing more**. `he` and `they` stay the parser's nodes
(`(z2 / he)`, `(amr-node S2 s2-z2 he)`), no resolver's chain is written beside them (the resolver tied the model,
S2.2f.1's verdict), and the analysis is given a sentence a graph, never the text whole.

## What

- **A sentence parse resolves no pronoun**: every AMR parser keeps `(h / he)` and `(t / they)`; the UD parses mark
  them as pronouns. The model's step 1 resolved *He* to Bob on both texts.
- **The whole text as one input** (amrlib's XFM and SPRING): on the short texts every sentence is kept and *2* is
  written as 2 apples, XFM makes the question's *Bob* the first sentence's node, and *he* and *they* stay; on the
  Zebra puzzle XFM's graph holds 8 of the 17 sentences, SPRING's 13.
- **Stanza's coreference resolver** links *He* to Bob (2 of 2) and *Kools* across sentences 8 and 12, gives *they* a
  chain of its own, and leaves *the Norwegian* (10, 15) and *the green house* (4, 6) unlinked; on its five cells it
  ties the model, 4 and 4, and so does not pass the stage's rule.
- **IBM's document parser** (coreference as `:same-as`) links all five repeated mentions of the seating, one of
  three right on the Zebra puzzle, and is malformed on the four word problems, with no `:same-as`.

## Evidence

[`check/README.md`](../../check/README.md) § 3 (cells a4, t4, t7, z19, z24), § 5; `check/runs/stanza/*.coref.json`,
`check/runs/amr-*/*.whole.amr`, `check/runs/amr-ibm-doc/` (gitignored).

## What it costs

An analysis given sentence by sentence says nothing of who *he* is, which the model reads right alone; the classes'
other cells are where it helps. A text longer than the Zebra puzzle (M17) cannot be given whole.
