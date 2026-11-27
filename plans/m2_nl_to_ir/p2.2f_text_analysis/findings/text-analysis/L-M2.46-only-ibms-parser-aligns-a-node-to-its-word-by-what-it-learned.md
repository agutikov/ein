# L-M2.46 — Only IBM's parser aligns a node to its word by what it learned, and the default analysis shows no word

**Status:** open
**Found:** by [S2.2f.2](../../s2.2f.2_the_analysis_written.md), 2026-11-27
**Owner:** [Q-M2.50](../../open_questions/formalizer/Q-M2.50-does-q11-reach-the-rule-based-parts-of-the-trained-tools.md),
whose default this follows; [S2.2f.3](../../s2.2f.3_step1_with_the_analysis.md)

## What

Each node of every graph has its sentence: a graph is one sentence's. Its words are another matter. **IBM's parser
aligns all 182 of its nodes on the six texts**, by what it learned; amrlib's RBW aligner, rule-based, aligns 95 of
SPRING's 181 and 97 of XFM's 183, and leaves the rest with no word: per text, SPRING 8 of 14 (`apples`), 8 of 16
(`more`), 15 of 28 (`seating`), 6 of 14 (`together`), 15 of 18 (`youngest`), 34 of 91 (the Zebra puzzle). The
unaligned are mostly AMR's own concepts no word spells (`person`, `name`, `country`, `amr-unknown`, `thing`) and the
passive's `drink-01`, which RBW, by lemma, does not find in *drunk*. Following Q-M2.50's default and the brief
(*prefer a learned alignment where a node's words are needed*), **a form shows a node's words only where the aligner
is learned**: IBM's graphs carry them (`[drunk]` in a line, `(amr-word s4-d drunk)` as a fact), SPRING's and XFM's
carry none, their RBW markers kept in the recorded file and counted. The default analysis, SPRING's, so shows no
word, and gives each node its sentence alone.

## Evidence

`einaf analysis counts einaf/tests/fixtures/analysis/*.json`; `einaf/tests/analysis.rs`'
`every_recorded_analysis_is_of_its_text_and_every_graph_reads` holds the three totals and
`the_text_forms_hold_every_sentence_and_no_marker` the words shown; [the stage's record](../../s2.2f.2_the_analysis_written.md#record).

## What it costs

An arm that wants SPRING's reading with each node's word cannot have both under the default: the words are IBM's,
on IBM's graphs, which read the Zebra puzzle worse (L-M2.41). Ruled otherwise, Q-M2.50's second alternative changes
nothing here; a ruling that a rule-based aligner may be shown is one line of `einaf::analysis` and a version of the
templates.
