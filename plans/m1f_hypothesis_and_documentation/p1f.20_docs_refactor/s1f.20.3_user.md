# S1f.20.3 — `docs/ein/user/` — the black box

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 5 days.
**Status:** not started. It was `T1f.5.20.3` until 2026-11-08.
**Depends on:** [S1f.20.2](s1f.20.2_the_disposition_table.md)'s table, reviewed.

## What it writes

*"problem → [solve as a black box] → result"*: every page the table sends to `docs/ein/user/`, rewritten
and not copied.

The cheapest and most valuable third. `docs/guide/` moves nearly verbatim (**1** history line in 693, on
2026-08-28), and the reference pages are assembly rather than writing.

Two things are genuinely new, and they are the reason the perspective split is worth doing:

- **`syntax-and-semantics/` states each concept in four representations**: ein-lang, graph, NL and math, as
  the user's outline asks. That is a *format* decision applied uniformly, and it is the thing no current page
  does. The grammar states the syntax, `01-ein-graph/` states the graph, nothing states the maths, and the NL
  is [S1f.5.5](../p1f.5_documentation_and_other/s1f.5.5_nl_required.md)'s subject. Pick the format once, in
  `objects.md`, and hold it.
- **`theory.md`**: a rule set as a *theory*, and how rules interact. Nothing in the tree says this today,
  and it is the concept [M2](../../m2_nl_to_ir/README.md)'s theory selection and theory synthesis are about.
  It may be the single most load-bearing new page in the phase.

## How it moves a page

Each page moves with its inbound links, in one commit ([P1f.20 § Two rules](README.md#two-rules-across-the-stages)).
A page's release-voice count falls to 0 in the same commit, and the baseline is lowered with it.

## The method

### Task T1f.20.3.1 — the format, in `objects.md`

The four representations, settled on one page, and shown to the user before the other pages take it.

### Task T1f.20.3.2 — `syntax-and-semantics/`, `guide/` and `reference/`

Each page of the table's `user/` rows, with its links.

### Task T1f.20.3.3 — `theory.md`

The new page.

## Acceptance

- Every row the table sends to `user/` is written there, and its source page is gone or carries the table's
  disposition.
- Every concept page states its concept in the four representations.
- The link check is green, and the release-voice baseline has no line for a page under `docs/ein/user/`.
- `./run_tests.sh` is green.
