# E-140 — A program of facts alone is guessed about at root

**Kind:** weakness, as filed; **expected behaviour**, the user's ruling
**Status:** closed by the user's ruling, 2026-11-27: Ein works as designed, and a program that is data says
`(solve :no-search)`, as EinAf's facts form does
**Filed:** by [S2.2f.2](../../s2.2f.2_the_analysis_written.md), 2026-11-27, on `dd3910aa` with the stage's
`einaf analysis`
**Owner:** none; EinAf's facts form states `(solve :no-search)`, which takes the cost away

**Ruled by the user, 2026-11-27:** *"E-140, a program made only of facts gets 84,309 guessing candidates at root unless it ends with (solve :no-search) - this is expected behaviour; E-141, ein render kb draws every one of those candidates - also expected behaviour, maybe have to print warning and add limit and flag to change it; E-143, and and or can never be names in Ein - also expected behaviour"*.

## What

**A program of facts alone is guessed about at root.** The AMR analysis of a
text written as Ein facts is eight declarations and the facts, no rule and no query: data. A program that states no
hypothesis mechanism has H written by the blind enumerator ([E-13](../../../evaluation/hypotheses/E-13-a-program-that-names-no-hypothesis-mechanism-is-answered-by.md)),
over every relation it declares and every name it writes, and under `--no-search` root's `open` facts are written all
the same. On the six texts' analyses with no `(solve :no-search)`: 3 195 to 12 892 candidates for the four word
problems and the seating, **84 309 for the Zebra puzzle's 194 SPRING facts and 126 773 for IBM's 296**: root holds
84 519 facts where the program wrote 210, and `ein solve --no-search` takes 3.6 s and 5.9 s where the same program
with the line takes 14 and 16 ms (one run each, on this host, on AC). Nothing in the read-out says that the
candidates are the cost: `universe 84 309 candidates, none visited`.

## Evidence

`python3 plans/m2_nl_to_ir/p2.2f_text_analysis/tools/write_forms.py` writes each text's facts with and without the
line under `forms/runs/` (gitignored) and prints the table, its last column; one program by hand:

```sh
einaf analysis show einaf/tests/fixtures/analysis/zebra.json --form facts | grep -v '^(solve' > z.ein
ein solve --no-search z.ein --json-summary s.json     # universe 84 309 candidates; root.facts_by_relation.open
```

## What it costs the application

A program the formalizer is handed as data, or that a reading arm loads to apply rules to, pays seconds a load and
root's facts four hundred times its own, unless it says `(solve :no-search)`; EinAf's facts form says it. The
lattice of such a program is not asked for, and no surface says that root's work went to H.
