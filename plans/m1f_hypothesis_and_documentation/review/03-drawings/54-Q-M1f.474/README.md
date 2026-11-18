# Q-M1f.474 — an arrow in a hand schematic that is no arrow of the graph

## 1. The question

In a drawing of facts, every arrow is an arrow of the knowledge-base graph: a fact's arrow `1` to its relation, `2`,
`3`, … to its arguments. The guide's chapter 3 has one hand schematic that breaks that, on purpose:
`03-bijective-family` draws the fact `(bijective color-loc)` as the octagon it is, and a **dashed arrow** from it to a
box listing *the logic it switches on* (`functional`, `injective`, `total`, `surjective`, the eliminations, negative
completion). That arrow is no arrow of the graph; the page says so. Is such an arrow allowed where its page says so,
or never?

Recorded: [`open_questions.md` § Q-M1f.474](../../../open_questions.md#q-m1f474--an-arrow-in-a-hand-schematic-that-is-no-arrow-of-the-graph),
S1f.23.1's task 2, its reading 8.

## 2. What is so today

The schematic, copied as the guide holds it ([`renders/03-bijective-family.dot`](renders/03-bijective-family.dot)):

![Today: the fact (bijective color-loc) an octagon, its arrow 1 at bijective and 2 at color-loc, and a grey dashed arrow from it to a rounded frame, the logic it switches on, listing negative-completion, domain- and range-elimination, surjective, total, injective, functional](renders/03-bijective-family.svg)

and the page ([`runs/page-excerpt.md`](runs/page-excerpt.md)):

> One tag pulls in a whole family of imported logic. The tag is a fact like any other, an octagon whose arrow `1` is
> at `bijective` and whose arrow `2` is at `color-loc`; the dashed arrow is no arrow of the graph, and points at what
> the fact switches on:

It is the only page that says so ([`runs/pages-that-say-no-arrow-of-the-graph.txt`](runs/pages-that-say-no-arrow-of-the-graph.txt)).
Before task 2 (`1418fba5`) the schematic was a note labelled with the fact's text and six plain arrows to the family
([`renders/03-bijective-family.at-1418fba5.svg`](renders/03-bijective-family.at-1418fba5.svg)), form 4.

**What the fact switches on, as the binary draws it.** [`programs/bijective-tag.ein`](programs/bijective-tag.ein),
written for this review, states the tag and `(bijection-hierarchy is-a)` over two colours and two houses;
`ein render kb` draws its knowledge base ([`renders/bijective-tag.kb.svg`](renders/bijective-tag.kb.svg), by `fdp`),
and cut to the facts with an arrow at `color-loc` but the declarations'
([`renders/bijective-tag.kb.cut-to-color-loc.svg`](renders/bijective-tag.kb.cut-to-color-loc.svg), by
[`tools/cut_to.py`](tools/cut_to.py)): the tag, the four markers `std.algebra.bijective-properties` derives,
`(std.algebra.functional color-loc)` and the like, and the eight activators `std.bijection.bijective-setup` derives,
`(std.bijection.domain-elimination color-loc is-a)` and the like, each a dashed octagon with its rule beside it.

![What the tag switches on, as ein render kb draws it, cut to the facts about color-loc: the tag a solid octagon, and twelve dashed octagons around color-loc, each with its arrow 1 at a relation of std.algebra or std.bijection and its rule beside it](renders/bijective-tag.kb.cut-to-color-loc.svg)

## 3. The options

- **(a) allowed where its page says so** (the default): the schematic as it is, its page saying that the dashed arrow
  is no arrow of the graph.
- **(b) never.** Two ways to draw the same point with graph arrows only:
  - the dashed arrow taken off, the family beside the fact, the page saying in words that the fact switches it on —
    **a mock**, today's source edited by hand
    ([`renders/03-bijective-family.option-b-no-arrow.MOCK.dot`](renders/03-bijective-family.option-b-no-arrow.MOCK.dot),
    [svg](renders/03-bijective-family.option-b-no-arrow.MOCK.svg));
  - the binary's own drawing of what the fact derives, the cut above, in place of the schematic (a page's cut,
    Q-M1f.468); it shows the facts and not the logic, the rules, they activate.

Both are pages' drawings; no option changes code.

## 4. The default

**(a)**, taken by task 2: *"A hand schematic may carry one arrow that is no arrow of the graph, where its page says
so (`03-bijective-family`: the fact `(bijective color-loc)` an octagon, and a dashed arrow to the logic it switches
on)."*

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | the copies, the program's runs and the drawings, from the repo root |
| [`tools/cut_to.py`](tools/cut_to.py) | cut a canonical KB view to the facts with an arrow at one atom |
| [`programs/bijective-tag.ein`](programs/bijective-tag.ein) | one tag and its hierarchy, written for this review |
| [`renders/03-bijective-family.dot`](renders/03-bijective-family.dot) / [`.svg`](renders/03-bijective-family.svg) | the guide's schematic today |
| [`renders/03-bijective-family.at-1418fba5.dot`](renders/03-bijective-family.at-1418fba5.dot) / [`.svg`](renders/03-bijective-family.at-1418fba5.svg) | the same before task 2 |
| [`renders/03-bijective-family.option-b-no-arrow.MOCK.dot`](renders/03-bijective-family.option-b-no-arrow.MOCK.dot) / [`.svg`](renders/03-bijective-family.option-b-no-arrow.MOCK.svg) | mock: the dashed arrow taken off |
| [`renders/bijective-tag.kb.dot`](renders/bijective-tag.kb.dot) / [`.svg`](renders/bijective-tag.kb.svg), [`renders/bijective-tag.kb.fold.dot`](renders/bijective-tag.kb.fold.dot) / [`.svg`](renders/bijective-tag.kb.fold.svg) | `ein render kb`, canonical and `--fold` |
| [`renders/bijective-tag.kb.cut-to-color-loc.dot`](renders/bijective-tag.kb.cut-to-color-loc.dot) / [`.svg`](renders/bijective-tag.kb.cut-to-color-loc.svg) | the canonical view cut to the facts about `color-loc` |
| [`runs/bijective-tag.solve.txt`](runs/bijective-tag.solve.txt) | `ein solve` of the program |
| [`runs/page-excerpt.md`](runs/page-excerpt.md), [`runs/pages-that-say-no-arrow-of-the-graph.txt`](runs/pages-that-say-no-arrow-of-the-graph.txt) | the guide's paragraph; the pages that say it |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/54-Q-M1f.474/rerun.sh`. It keeps
the `.MOCK.dot` and draws it.

## 7. Found while building

- The program here answers **R empty, every alive node owing** ([`runs/bijective-tag.solve.txt`](runs/bijective-tag.solve.txt)):
  `(bijective color-loc)` owes a house to each colour and nothing proposes one under `(solve :no-search)`. The
  drawing is of root's knowledge base, which is what the tag switches on; the answer is not the point.
- The schematic's family names logic (`negative-completion`, `domain- and range-elimination`) that the knowledge base
  holds as activators of rules with other names (`std.bijection.functional-negative`, `…domain-elimination`), so the
  two drawings do not name the same things one for one.

## 8. Your call

- **(a)** allowed where its page says so (default).
- **(b)** never: the arrow taken off, or the binary's cut in its place.
