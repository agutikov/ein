# L-M2.25 — The steps' two partial parts are written twice, never revised, and a step before their uses

**Status:** open
**Found:** by [the analysis of P2.2 against its goals](../../p2.2_formalizer/report/ANALYSIS.md#112-what-the-two-partial-parts-cost),
2026-11-26, from S2.2.4's and S2.2.5's records; filed at the phase's close, 2026-11-27
**Owner:** [S2.2d.2](../../p2.2d_formalizer_corrected/s2.2d.2_the_cut.md), which measures the cut;
[Q-M2.36](../../open_questions/formalizer/Q-M2.36-how-are-the-three-steps-cut.md), the user's

## What

The three steps make a partial ontology in step 1 and a partial theory in step 2, each *expected to change in the
next step*. On the sixteen texts of S2.2.4:

- **The partial ontology is written twice.** Step 2 writes the ontology whole, step 1's again: 75 of the 208 forms
  the sixteen step 2s wrote are step 1's form for form, 38 of the Zebra puzzle's 69.
- **It is not revised.** Step 2 dropped or changed no declaration on any text: additions only. What step 1 decides
  of the kinds, with no theory in view, stands.
- **Where a step did revise, the program holds both versions.** The seating's step 1 wrote the three people as
  chairs and step 2 as persons; step 1's facts are never rewritten, so the assembled program says both, and the
  kernel's core is made of the stale ones.
- **A fault of what is no fault.** Step 1 is asked to declare what it uses, so a fact of a relation it did not
  declare is `undeclared` at step 1 (9 findings).
- **The import's list is written a step before its uses.** The Zebra puzzle's step 2 lists `(symmetric includes
  converse top meet)` and no `compose`; step 3's grammar, rendered on that program, offers `compose` only under its
  full name, and step 3 wrote none.
- **The mechanism is settled in one step and stated in the next**: step 2's `slot-partition` makes the obligations
  the writer of H, and step 3 states `(solve :no-search)`.
- **The query is written before the relation it reads exists, and the ontology then claims it**: `gap`'s step 1
  asks `sum-of`, step 2 declares `sum-of` as the program's own, and the import cannot list it. Four texts
  (`name-not-listed`).

With no grammar the steps load 11 programs where the one call loads 14: four are lost to slips between steps.

## Evidence

[The analysis §§ 11.1 and 11.2](../../p2.2_formalizer/report/ANALYSIS.md#11-the-three-steps-against-the-one-call-and-how-the-steps-are-cut),
with its counts (throwaway scripts, kept nowhere) and its re-runs; [S2.2.4's report](../../p2.2_formalizer/steps/README.md)
and [tables](../../p2.2_formalizer/steps/tables.md).

## What it costs

Two dead roots (the Zebra puzzle, the seating) and four of the thirteen word texts. **A strict cut was not
investigated**: S2.2.4's conditions vary the grammar and steps against one call, never what a step may write.
