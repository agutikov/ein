# E-141 — `ein render kb` draws every candidate root holds

**Kind:** weakness, as filed; **expected behaviour**, the user's ruling, with a feature the user proposes, *maybe*
**Status:** open, as the user's proposal only: the drawing of every candidate is as designed; a warning, a limit and
a flag that changes it are proposed, not decided
**Filed:** by [S2.2f.2](../../s2.2f.2_the_analysis_written.md), 2026-11-27, on `dd3910aa` with the stage's
`einaf analysis`
**Owner:** none; the proposal is the iteration of Ein's to weigh with the user, and no stage of M2 builds it

**Ruled by the user, 2026-11-27:** *"E-140, a program made only of facts gets 84,309 guessing candidates at root unless it ends with (solve :no-search) - this is expected behaviour; E-141, ein render kb draws every one of those candidates - also expected behaviour, maybe have to print warning and add limit and flag to change it; E-143, and and or can never be names in Ein - also expected behaviour"*.

**The proposal, as the coordinator reads it** (the user may correct it): `ein render kb` prints a warning where the
drawing passes a limit, draws to the limit, and a flag lifts the limit. Not designed here: what the limit counts
(arrows, candidates, bytes), what is left out at it (H first, which is what grew here), and whether it is the same
limit for every view.

## What

**`ein render kb` draws every candidate root holds.** Root's `open` facts are facts of its knowledge base, written at
its fixpoint ([Q-M1f.655](../../../../../docs/decisions/engine/Q-M1f.655-open-facts-are-written-at-root-and-where-a-state.md)),
and the drawing of the knowledge base draws each, with an arrow to each argument and to the fact it supposes. The
Zebra puzzle's analysis as facts, SPRING's graphs with the named entities beside, 205 facts and eight declarations,
written with no `(solve :no-search)` ([E-140](../hypotheses/E-140-a-program-of-facts-alone-is-guessed-about-at-root.md)):
`ein render kb` takes 27 s and 50 s in two runs and writes 67 894 063 bytes of DOT, 490 137 arrows, for the 97 894
candidates; the same program with the line, 11 and 16 ms and 108 191 bytes, 848 arrows, which `dot -Tsvg` lays out
in 127 and 161 ms. No option of `ein render kb` leaves H
out of the drawing, and nothing in it says what share of it is H.

## Evidence

```sh
einaf analysis show einaf/tests/fixtures/analysis/zebra.json --form facts --entities | grep -v '^(solve' > z.ein
ein render kb z.ein | grep -c -- '->'                   # 490 137
```

Taken by hand on the stage's branch; `tools/write_forms.py` draws each text's facts with the line, under
`forms/runs/<text>/<parser>.kb.dot` (gitignored).

## What it costs the application

A knowledge base drawn to be read, a program's own facts, is lost in its candidates wherever the program states no
mechanism: the drawing is the size of H, not of what the program wrote.
