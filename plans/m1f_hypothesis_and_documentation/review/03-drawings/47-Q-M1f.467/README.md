# Q-M1f.467 — a command's output on a page: canonical, or folded where the page's point is the compact view

## 1. The question

`ein render` draws every fact canonical by default, an octagon with no name in it, its arrow `1` at its relation's
box and `2`, `3`, … at its arguments, and folds a fact of three arrows into one labelled arrow under `--fold`
(Q-M1f.419). Many pages under `docs/ein/` show what a command draws. When the pages were drawn again, each such
drawing was made the binary's output for the command as the page writes it, canonical by default; but where the
page's point was the compact view (the reasoning pages' firings and states, the guide's fact shown both ways, the
Zebra fragment, the standard library's three states), the drawing is `--fold`'s output and the page writes `--fold`
(outside `reasoning/`, whose pages name no command) or says the drawing folds (in `reasoning/`). Is that right, or
should every page show the canonical form?

Recorded: [`open_questions.md` § Q-M1f.467](../../../open_questions.md#q-m1f467--a-commands-output-on-a-page-canonical-or-folded),
S1f.23.1's task 2, its reading 1.

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; the pages as merged (`231486f0`).

**24 page drawings of facts fold**: 23 of the 123 hand-kept drawings of facts under `docs/ein/` (a source with an
octagon whose arrows are numbered, or with an arrow labelled with a relation's name), and 1 of the 494 generated rule
drawings of the standard library ([`runs/folded-drawings.md`](runs/folded-drawings.md), by
[`tools/folded.py`](tools/folded.py), from the census of every arrow's label that is not a number,
[`runs/edge-labels.tsv`](runs/edge-labels.tsv)): 15 in `reasoning/`, the
guide's 3, `01-program-kb-fold` of the syntax pages, `12-program-kb-fold` of the rendering page, the standard
library's three states and its one folded rule drawing, `div9-step-0` (Q-M1f.434, item
[45](../45-Q-M1f.434/README.md)). **Every one's section says so**, but `div9-step-0`'s, a generated region whose image
text says *folded* and whose § Division says why. The reasoning pages say it in words (*"Both drawings fold each fact
of three arrows into a labelled arrow"*); the user's pages write the command (*"As a graph, from
`ein render kb --fold`"*). The guide's chapter 1, the syntax page `01-objects.md` and the rendering page show a fact
both ways, canonical first.

**Three of them, against the binary.** A page's copy of a command's output drops the `//` comment lines and the blank
lines, and nothing else:

| page drawing | program | page against the binary's `--fold` output |
|---|---|---|
| `reasoning/…/03-rule-soup-unless-allergic.dot` | [`programs/soup.ein`](programs/soup.ein), from `ein.rs/08-reasoning-in-code/02-saturation-and-negation.md` § 3.4 | the same ([`runs/soup.rule.page-vs-binary-fold.diff`](runs/soup.rule.page-vs-binary-fold.diff), exit 0) |
| `reasoning/…/03-soup-saturated.dot` | the same | a cut: every line of the page is the binary's, and the declarations' facts are left out, as the page says ([`runs/soup.kb.page-vs-binary-fold.diff`](runs/soup.kb.page-vs-binary-fold.diff); the cut is item [48](../48-Q-M1f.468/README.md)) |
| `user/02-guide/diagrams/07-zebra-fragment.dot` | [`programs/fragment.ein`](programs/fragment.ein), from the guide's `07-examples.md` | the same ([`runs/fragment.kb.page-vs-binary-fold.diff`](runs/fragment.kb.page-vs-binary-fold.diff), exit 0) |

**The rule, folded as the page shows it, and canonical**, `ein render rule [--fold] --name soup-unless-allergic`:

![soup-unless-allergic folded, as the reasoning page shows it: seats an arrow from ?g to ?c, meal a dashed arrow from ?g to Soup, the absence a cluster holding a fact of allergic, an octagon, since it has two arrows](renders/soup.rule.fold.svg)

![soup-unless-allergic canonical: each pattern an octagon, seats' with arrows 1, 2, 3 at seats, ?g and ?c, meal's dashed with 1, 2, 3 at meal, ?g and Soup](renders/soup.rule.svg)

**The guide's Zebra fragment**, folded as the guide shows it (`dot`'s layout: the page's copy keeps no
`// suggested layout: fdp`), and canonical, `ein render kb`, laid out by `fdp` as its DOT suggests:

![The guide's Zebra fragment as the page shows it, folded: the is-a facts a tree of labelled arrows, the clues co-located (10), co-located (2) and right-of (6) as labelled arrows, the declarations and the fact of transitive as octagons along the top](renders/fragment.kb.on-the-page.svg)

![The same knowledge base canonical, by fdp: 21 octagons, every is-a fact an octagon with its arrow 1 at the box is-a, the three clues octagons with (10), (2) and (6) beside them](renders/fragment.kb.svg)

The canonical drawing holds every fact as a node, so `is-a` is one box under 11 first arrows; the folded one reads
the hierarchy as a tree of arrows. Graphviz takes under a tenth of a second for each
([`runs/draw-times.tsv`](runs/draw-times.tsv)).

## 3. The options

- **(a) the binary's output as the page writes the command, canonical by default; folded where the page's point is
  the compact view, the page writing `--fold`, or in `reasoning/` saying it folds** (the default). What is on the
  pages today: the 24 above.
- **(b) the canonical form on every page.** The 24 drawings drawn again without `--fold`, and their sentences
  rewritten; for the two above, the page would show [`renders/soup.rule.svg`](renders/soup.rule.svg),
  [`renders/soup.kb.svg`](renders/soup.kb.svg) (whole, or cut as item 48 decides) and
  [`renders/fragment.kb.svg`](renders/fragment.kb.svg), each the binary's own output, so no mock is needed. The
  pages that show a fact both ways (the guide's chapter 1, `01-objects.md`, the rendering page) would lose their
  folded half, or keep it as the one place the fold is shown. No code changes either way.

## 4. The default

**(a)**, taken by task 2: *"Folded where the drawing's point is the compact view, and the page writes `--fold`
(outside `reasoning/`) or says the drawing folds (in it)"*. The ruling (Q-M1f.419) makes the canonical form the
default of the command and the fold an option; it says nothing of which a page shows, and forbids only forms 2 and 4.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing, from the repo root |
| [`tools/edge-labels.sh`](tools/edge-labels.sh) | every docs drawing's arrow labels that are not numbers |
| [`tools/folded.py`](tools/folded.py) | the 24 folded drawings of facts, each with its page and what its section says |
| [`programs/soup.ein`](programs/soup.ein) | the soup program, from `ein.rs/08-reasoning-in-code/02-saturation-and-negation.md` § 3.4 (it states no `(solve …)`, so it is drawn, not solved: its blind universe is past the region limit) |
| [`programs/fragment.ein`](programs/fragment.ein) | the guide's larger Zebra fragment, from `user/02-guide/07-examples.md` |
| [`runs/edge-labels.tsv`](runs/edge-labels.tsv) | the census |
| [`runs/folded-drawings.md`](runs/folded-drawings.md) | the 24, a table |
| [`runs/soup.rule.page-vs-binary-fold.diff`](runs/soup.rule.page-vs-binary-fold.diff), [`runs/soup.kb.page-vs-binary-fold.diff`](runs/soup.kb.page-vs-binary-fold.diff), [`runs/fragment.kb.page-vs-binary-fold.diff`](runs/fragment.kb.page-vs-binary-fold.diff) | each page drawing against the binary's `--fold` output, its comments and blank lines dropped |
| [`runs/fragment.solve.txt`](runs/fragment.solve.txt) | `ein solve` of the fragment |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt), [`runs/draw-times.tsv`](runs/draw-times.tsv) | the exit codes; Graphviz's layout and seconds |
| [`renders/soup.rule.dot`](renders/soup.rule.dot) / [`.svg`](renders/soup.rule.svg), [`renders/soup.rule.fold.dot`](renders/soup.rule.fold.dot) / [`.svg`](renders/soup.rule.fold.svg), [`renders/soup.rule.on-the-page.dot`](renders/soup.rule.on-the-page.dot) / [`.svg`](renders/soup.rule.on-the-page.svg) | the rule canonical, folded, and the page's copy |
| [`renders/soup.kb.dot`](renders/soup.kb.dot) / [`.svg`](renders/soup.kb.svg) ([dot layout](renders/soup.kb.dot-layout.svg)), [`renders/soup.kb.fold.dot`](renders/soup.kb.fold.dot) / [`.svg`](renders/soup.kb.fold.svg), [`renders/soup.kb.on-the-page.dot`](renders/soup.kb.on-the-page.dot) / [`.svg`](renders/soup.kb.on-the-page.svg) | the knowledge base canonical, folded, and the page's cut |
| [`renders/fragment.kb.dot`](renders/fragment.kb.dot) / [`.svg`](renders/fragment.kb.svg) ([dot layout](renders/fragment.kb.dot-layout.svg)), [`renders/fragment.kb.fold.dot`](renders/fragment.kb.fold.dot) / [`.svg`](renders/fragment.kb.fold.svg), [`renders/fragment.kb.on-the-page.dot`](renders/fragment.kb.on-the-page.dot) / [`.svg`](renders/fragment.kb.on-the-page.svg) | the fragment canonical, folded, and the page's copy |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/47-Q-M1f.467/rerun.sh`.

## 7. Found while building

- **A page's copy drops the binary's `// suggested layout: fdp`**, so every KB drawing on a page is laid out by `dot`
  (`render_diagrams.py` reads no hint): the fragment above is a strip on the page, a square by `fdp` (R-18, item
  [55](../55-R-18/README.md)). Folded, `dot`'s strip reads well here, since the folded `is-a` facts make a tree.
- The page's soup program states no `(solve …)`, so `ein solve` on it asks 2^61 saturations, past the region limit;
  it is a program to draw, and the page says so.
- The generated `div9-step-0`'s heading section does not say *fold*; its image text and the § Division paragraph do.

## 8. Your call

- **(a)** canonical by default, folded where the page's point is the compact view and the page says so (default).
- **(b)** canonical on every page.
