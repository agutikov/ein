# E-141 — `ein render kb` draws every candidate root holds

**Kind:** weakness
**Status:** open
**Filed:** by [S2.2f.2](../../s2.2f.2_the_analysis_written.md), 2026-11-27, on `dd3910aa` with the stage's
`einaf analysis`
**Owner:** none

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
