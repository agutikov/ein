# Drawings

[← the bundle](../README.md)

The views of `ein render` and the pages' drawings, after every object is drawn once (Q-M1f.395) and a frame goes
around what is in a context only, no octagon filled (Q-M1f.398).

| # | item | the question | default |
|---|---|---|---|
| 06 | [L-107](06-L-107/README.md) | the extent's drawing: $g$, which no frame can mark | as built: grey, frame kept |
| 10 | [L-96](10-L-96/README.md) | a literal `\n` in the KB view's octagon labels | as is |
| 25 | [L-97](25-L-97/README.md) | nine `div9-step` rules have no drawing | the bound stays |
| 26 | [L-98](26-L-98/README.md) | the program view is several digraphs | as is |
| 27 | [L-99](27-L-99/README.md) | the derivation views keep a fact's text in a node | none |
| 28 | [L-100](28-L-100/README.md) | the beliefs' two KB drawings have no test | as is |

## Found while building

- **06**: every option is drawn (as built; the first arrows with no frame, $g$ grey or plain; $g$ left out, frame kept
  or dropped), beside the drawing before the filling went and the first trial's. Two of them need the page's
  sentence, which names the frame, rewritten with them.
- **10**: **329** of the corpus's 670 loading programs have such a label in `ein render kb` (95 605 labels), **485**
  in `ein render constraints` (1 862). A user page's picture shows it:
  `03-syntax-and-semantics/diagrams/07-constraints-view.svg`, `knows\n«symmetric, transitive»`. The constraints view
  builds its label in two places, so its fix is two lines.
- **25**: `div9-step-1` is 183 nodes against the bound of 120; 16 of the library's 511 rules are over it. A bound of
  183 or more also lets in three closing rules (`props` 129, `arith9` 157, `binary-arith` 160), so a third option is
  written: raise it and leave closing rules out by kind. No page says why the nine are shown as Ein only.
- **26**: no corpus program has the row's exact case; 34 draw atoms in two digraphs (373 atoms, zebra2 35).
  `ein render program examples/syntax/config.ein` exits 1, *no renderable forms*.
- **27**: two of the three views have no `ein` command (the derivation graph is called only by its crate's test; the
  written trace's `dag` and `aggregate` views only by the library), against Q-M1e.20; read, not run. Q-M1f.395 says
  these nodes carry the fact's *sentence*; they carry its text.
- **28**: the shape golden draws a KB as loaded, not saturated, so it does not cover these drawings either; the
  regeneration line draws with `dot` where the view suggests `fdp`.
