# Q-M1f.471 — `01-object-order`: six unlabelled octagons, their arrows left out

## 1. The question

The knowledge-base page (`reasoning/01-static-ir/01-knowledge-base.md`) defines a total order $\preceq$ on objects,
atoms before facts, and shows a chain of nine objects in it: three atoms and six facts, joined by arrows that read
*comes before*. Before, each of the six was a box labelled with its text, `co-located(a, p(a))` (form 4, now ruled
out). It was drawn again as six **unlabelled octagons named $f_1$ to $f_6$ beside them, their own arrows left out**,
and the page gives each fact in TeX above the picture. Is that the drawing, or should each fact be drawn with its
arrows?

Recorded: [`open_questions.md` § Q-M1f.471](../../../open_questions.md#q-m1f471--01-object-orders-drawing), S1f.23.1's
task 2, its reading 5.

## 2. What is so today

The drawing, copied as the page holds it ([`renders/01-object-order.dot`](renders/01-object-order.dot)):

![Today: the atoms 10, 2 and a as boxes, then six unlabelled octagons f1 to f6, joined left to right by arrows labelled ≺](renders/01-object-order.svg)

and the page around it ([`runs/page-excerpt.md`](runs/page-excerpt.md)), quoted:

> A chain of nine objects in the order, three atoms and six facts. The drawing is of the order and not of a
> knowledge base: a box is an atom, labelled with its name, and an octagon a fact, with the letter the text calls it
> beside it; the arrows under each fact are left out, and are these: $f_1 = \operatorname{adjacent}(\mathsf{b},
> \mathsf{b})$, … $f_5 = \operatorname{co-located}(\mathsf{a}, \operatorname{p}(\mathsf{a}))$, $f_6 =
> \operatorname{co-located}(\mathsf{b})$ *(a TeX block on the page)*. … Each arrow reads *comes before*. The third is
> an atom before a fact; the fourth is decided at the first arrow, by the relation's name; the fifth by the count, a
> prefix first; the seventh by an atom before a fact under the third arrow.

Before task 2 (`1418fba5`; [`renders/01-object-order.at-1418fba5.svg`](renders/01-object-order.at-1418fba5.svg),
[diff](runs/01-object-order.at-1418fba5-vs-today.diff), [the page then](runs/page-excerpt.at-1418fba5.md)): the same
chain top to bottom, each fact a box with its text, and the page saying *"each box is one object, written as a
term"*.

![Before: the same chain, each fact a box labelled with its text, adjacent(b, b) to co-located(b)](renders/01-object-order.at-1418fba5.svg)

## 3. The options

- **(a) six unlabelled octagons in the chain, their arrows left out, the page giving each fact in TeX** (the
  default): the picture shows the order and nothing else; to see why $f_4$ precedes $f_5$ the reader reads the TeX.
- **(b) each fact with its arrows.** **A mock**, today's source edited by hand
  ([`renders/01-object-order.option-b-arrows-drawn.MOCK.dot`](renders/01-object-order.option-b-arrows-drawn.MOCK.dot)):
  the chain kept, grey and heavy; each fact's arrows drawn and numbered, `1` at its relation and `2`, `3` at its
  arguments; every object once, so the chain's atom `a` is the `a` the facts point at, and `p(a)` is a seventh fact,
  no member of the chain.

![Mock: the chain along the top, and below it the arrows of each fact, numbered, to adjacent, co-located, b, p and the chain's own a; p(a) an octagon of its own under f5's arrow 3](renders/01-object-order.option-b-arrows-drawn.MOCK.svg)

  The comparisons the page names become visible (f3 and f4 differ at the arrow `3`, at `a` and `b`; f5's arrow `3`
  is at a fact), at the cost of 18 arrows crossing the chain, and with the chain's `a` doubling as an argument.

Both are a hand drawing; no option changes code.

## 4. The default

**(a)**, taken by task 2: *"`01-object-order` draws its six facts as unlabelled octagons named $f_1$ to $f_6$ in the
chain $\prec$, their arrows left out, and the page gives each fact in TeX."*

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | the copies, the page excerpts and the drawings, from the repo root |
| [`renders/01-object-order.dot`](renders/01-object-order.dot) / [`.svg`](renders/01-object-order.svg) | the page's drawing today |
| [`renders/01-object-order.at-1418fba5.dot`](renders/01-object-order.at-1418fba5.dot) / [`.svg`](renders/01-object-order.at-1418fba5.svg) | the same before task 2 |
| [`renders/01-object-order.option-b-arrows-drawn.MOCK.dot`](renders/01-object-order.option-b-arrows-drawn.MOCK.dot) / [`.svg`](renders/01-object-order.option-b-arrows-drawn.MOCK.svg) | mock: option (b) |
| [`runs/page-excerpt.md`](runs/page-excerpt.md), [`runs/page-excerpt.at-1418fba5.md`](runs/page-excerpt.at-1418fba5.md) | the page's paragraph and image, today and before |
| [`runs/01-object-order.at-1418fba5-vs-today.diff`](runs/01-object-order.at-1418fba5-vs-today.diff) | the source's change |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/51-Q-M1f.471/rerun.sh`. It keeps
the `.MOCK.dot` and draws it.

## 7. Found while building

- The page's TeX writes the facts in function notation, $\operatorname{co-located}(\mathsf{a}, \mathsf{b})$, the
  reasoning pages' notation; the drawing's names `f1` … `f6` are `xlabel`s, as the page's letters $f_1$ … $f_6$.
- In the mock the chain's `a` and the argument `a` are one node, as every object once wants; the drawing then mixes
  two relations of objects, the order and the arrows, which the page's sentence *"The drawing is of the order and
  not of a knowledge base"* keeps apart today.

## 8. Your call

- **(a)** the chain of unlabelled octagons, the facts in TeX (default).
- **(b)** each fact with its arrows, in the chain.
