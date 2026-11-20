# S1f.21.19 — the review items on the built engine

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 on the user's word (*"after implementation complete add renders with the final ein
version. I will review questions after."*); **not started**.
**Waits on:** every build of the phase: [S1f.21.12](s1f.21.12_hypotheses_as_facts.md),
[S1f.21.18](s1f.21.18_the_hrule_form_removed.md), [S1f.21.13](s1f.21.13_the_search_and_rules.md), and the pages.
**Writes:** the review items of [the bundle](review/README.md): 58 to 65, 77 and 78, and the seven findings'
items, 79 to 85: their pages, their programs, their runs and renders. No engine file, no page of `docs/`.

## What it is for

The eight items were written before anything was built, so each shows a stand-in: `question` for `open`, a
hypothesis rule for what an obligation or the union would propose. The user took their defaults to let the build
run, and reviews the questions after it. This stage gives that review the real thing beside the stand-in.

## What it does

For each item, 58 to 65:

1. **Its programs in the spelling ruled**, which the built engine reads: each `*.target.ein` run as it is; each
   stand-in program written again with `open` where it wrote `question`, and with a rule where it wrote a
   hypothesis rule.
2. **Every run again, on the final binary**: the read-out, `ein show hyps`, `ein saturate --dump`, the oracle's
   read-out and its dump where the cube is small, the events and the summary, and **the renders**: the knowledge
   base, the lattice and the region drawn, as `.dot` and `.svg`.
3. **A section on its page, *On the built engine***: each number the stand-in gave beside the number the built
   engine gives, and where the two differ, why. A stand-in's number is never replaced: it stays, marked as what
   it was.
4. **Item 58's list taken again**: every corpus program that moved, as it answers before the phase's second part
   and after it, by the corpus as built.
5. **Item 64's program**, `examples/ein-bugs/wrong-answers/oblig-derived-domain.ein`, with what it answers: the user
   expects it still solvable, *"my expectation, not requirement"*, and reviews it here.

**Added 2026-11-20**, once every build was in master and the user had asked that everything that waits for an
answer be in the review folder:

6. **The seven findings that are the user's call**, items 79 to 82 of
   [part 1](review/part-1/README.md) and 83 to 85 of [part 2](review/part-2/README.md): each gets its
   `programs/`, its runs on the final binary, and its section *On the built engine* with what the binary says
   today. Where a finding's figure is no longer what the binary gives, both are shown.
7. **[Item 77](review/part-2/77-what-a-drawing-of-root-shows/README.md)**: the other seven of the nine pictures,
   each as the page has it beside what `ein render kb` draws today, as its first two are; and what option (b)
   and option (c) would draw, composed from the drawing by leaving the `open` facts or the blind enumerator's
   out, marked as composed.
8. **[Item 78](review/part-2/78-the-key-of-a-state/README.md)**: the profile its figures are from, taken again on
   the final binary and kept in its `runs/`.

## What must hold

Nothing of the answers' file is filled in: the sections are the user's. No question is marked ruled. A number on a
page is in the item's `runs/`. `python3 utils/doc_audit.py --links --check --tracked` is clean.

## Record

Nothing yet.
