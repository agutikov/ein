# Q-M1f.426 — what the constraint view draws

## 1. The question

`ein render constraints` draws a program's structural facts: what it states about its relations rather than with
them, *next-to is symmetric*, *right-of implies next-to*, *color-loc is a bijection*. Before S1f.23.1 it wrote a
property into the relation's box, `next-to\n«symmetric»` (a fact's text in a node, and a literal `\n`), drew a fact of
two relations as a labelled arrow, drew an import as a box `std.algebra\n«import»`, and chose what to draw by leaving
out three names, `relation`, `type` and `instance`. With every fact drawn as a fact and no theory's name read
(Q-M1f.436), the build draws **each such fact as a fact**, by the KB view's code, and chooses them by no name:
every fact the program states with no `:source`, `:rule` or `:using`, of a relation it does not declare. A
declaration is a fact of `relation`, which no program declares, so **the declarations are drawn among them**. The
other reading: the same, with the declarations left out by the name `relation`.

Recorded: [`open_questions.md` § Q-M1f.426](../../../open_questions.md#q-m1f426--what-the-constraint-view-draws)
(put, not ruled; default (a)).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; the view suggests no layout, so `dot` draws it, left to right.

**A small program of properties**, [`programs/props.ein`](programs/props.ein): two declarations, `(symmetric
next-to)`, `(implies right-of next-to)` (each applying a rule of the program), and a given fact
`(right-of H2 H1 :source "clue (1)")`. The view ([`renders/props.constraints.dot`](renders/props.constraints.dot) /
[`.svg`](renders/props.constraints.svg)) draws four facts: the two declarations (arrow `1` at `relation`, `2` at the
relation declared, `3` and `4` at `House`), `(symmetric next-to)` (two arrows) and `(implies right-of next-to)` (three
arrows). The given fact is not drawn (it has a source and its relation is declared).

![ein render constraints of props.ein: two declarations at relation, House, right-of and next-to; the fact of symmetric at next-to; the fact of implies at right-of and next-to](renders/props.constraints.svg)

Folded ([`renders/props.constraints.fold.svg`](renders/props.constraints.fold.svg)): `(implies right-of next-to)` is
an arrow `right-of → next-to` labelled `implies`; the rest stay octagons. **Before S1f.23.1**
(`build/review-bin/ein-head-b5fb8d70`, [`renders/props.constraints.before-b5fb8d70.svg`](renders/props.constraints.before-b5fb8d70.svg),
[dot](renders/props.constraints.before-b5fb8d70.dot)): two boxes, `next-to\n«symmetric»` and `right-of`, and the
arrow `implies`; no declaration.

**zebra2** ([`programs/zebra2.ein`](programs/zebra2.ein), a copy of `examples/zebra2.ein`;
[`renders/zebra2.constraints.dot`](renders/zebra2.constraints.dot) / [`.svg`](renders/zebra2.constraints.svg)): 20
facts by their relation ([`runs/facts-by-relation.txt`](runs/facts-by-relation.txt)): 9 declarations, 5 of
`bijective`, 2 of `includes`, one each of `typecheck-hierarchy`, `transitive`, `symmetric`, `bijection-hierarchy`. The
nine declarations, of four arrows each, are 36 of its 60 arrows and fill the lower half of the drawing.

![ein render constraints of zebra2: a column of octagons on the left, a column of boxes on the right; the declarations' arrows to relation, House, Color, Nationality and the other labels fill the lower half](renders/zebra2.constraints.svg)

Before ([`renders/zebra2.constraints.before-b5fb8d70.svg`](renders/zebra2.constraints.before-b5fb8d70.svg)): eleven
boxes with their badges, `color-loc\n«bijective»` (five), `is-a*\n«typecheck-hierarchy, transitive»`,
`std.algebra\n«import»`, …, and two `includes` arrows ([`runs/before-badges.txt`](runs/before-badges.txt), every
badge).

![before: boxes labelled with their relation and a badge, color-loc\n«bijective» and the others, the imports as boxes std.algebra\n«import», two includes arrows](renders/zebra2.constraints.before-b5fb8d70.svg)

On the small program of the review ([`programs/small.ein`](programs/small.ein),
[`renders/small.constraints.svg`](renders/small.constraints.svg)) the view draws the declarations, `(symmetric
friends)`, the two facts of `claims`, the stored negation, and the three facts of `owns` they point at, the two that
are not stated in their frames, `claims ctx` and `not ctx`.

## 3. The options

### (a) Each fact drawn as a fact, chosen by no name: the declarations among them (the default, built)

As above. What the view draws is decided by the program's form alone: a fact with no source of a relation the program
does not declare.

### (b) As (a), the declarations left out by the name `relation`

The view reads `relation` by its name to leave the declarations out, as it read `relation`, `type` and `instance`
before. Needs a code change: **mocks**, today's DOT with every fact whose arrow `1` is at `relation` deleted, its
octagon and its arrows, nothing else edited:
[`renders/props.constraints.option-b.MOCK.dot`](renders/props.constraints.option-b.MOCK.dot) /
[`.svg`](renders/props.constraints.option-b.MOCK.svg) (2 declarations out) and
[`renders/zebra2.constraints.option-b.MOCK.dot`](renders/zebra2.constraints.option-b.MOCK.dot) /
[`.svg`](renders/zebra2.constraints.option-b.MOCK.svg) (9 out, 24 arrows left).

![MOCK, option (b): zebra2's constraint view with the declarations deleted: five facts of bijective, the facts of includes, transitive, symmetric, typecheck-hierarchy and bijection-hierarchy](renders/zebra2.constraints.option-b.MOCK.svg)

`relation` is the kernel's word for a declaration today, so (b) reads a kernel name and no theory's; it becomes a
theory's name if P1f.21 makes `relation` a convention of the standard library (Q-M1f.447, open).

## 4. The default

**(a)**, taken by S1f.23.1: each fact drawn as a fact by the KB view's code, `(symmetric next-to)` an octagon whose
second arrow is at `next-to`, folded where it has three arrows; the facts drawn are every fact the program states with
no `:source`, `:rule` or `:using`, of a relation it does not declare; no name is left out by what it means, so a
declaration is drawn among them. The `«property»` badges, which wrote a literal `\n`, go. Built in
`ein-render/src/constraints.rs`; held by `every_object_once.rs`' `the_constraint_view_draws_a_property_as_a_fact`.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | re-takes every run and drawing below |
| [`programs/props.ein`](programs/props.ein) | properties of two relations, written for this item |
| [`programs/small.ein`](programs/small.ein) | the small program, written for the drawings' review |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| [`renders/props.constraints.dot`](renders/props.constraints.dot) / [`.svg`](renders/props.constraints.svg), [`renders/props.constraints.fold.dot`](renders/props.constraints.fold.dot) / [`.svg`](renders/props.constraints.fold.svg) | `ein render constraints`, without and with `--fold` |
| [`renders/small.constraints.dot`](renders/small.constraints.dot) / [`.svg`](renders/small.constraints.svg), [`renders/small.constraints.fold.dot`](renders/small.constraints.fold.dot) / [`.svg`](renders/small.constraints.fold.svg) | the same, small.ein |
| [`renders/zebra2.constraints.dot`](renders/zebra2.constraints.dot) / [`.svg`](renders/zebra2.constraints.svg), [`renders/zebra2.constraints.fold.dot`](renders/zebra2.constraints.fold.dot) / [`.svg`](renders/zebra2.constraints.fold.svg) | the same, zebra2 |
| [`renders/props.constraints.before-b5fb8d70.dot`](renders/props.constraints.before-b5fb8d70.dot) / [`.svg`](renders/props.constraints.before-b5fb8d70.svg), [`renders/zebra2.constraints.before-b5fb8d70.dot`](renders/zebra2.constraints.before-b5fb8d70.dot) / [`.svg`](renders/zebra2.constraints.before-b5fb8d70.svg) | the view by the binary before S1f.23.1 |
| [`renders/props.constraints.option-b.MOCK.dot`](renders/props.constraints.option-b.MOCK.dot) / [`.svg`](renders/props.constraints.option-b.MOCK.svg), [`renders/zebra2.constraints.option-b.MOCK.dot`](renders/zebra2.constraints.option-b.MOCK.dot) / [`.svg`](renders/zebra2.constraints.option-b.MOCK.svg) | **mocks**, option (b) |
| [`runs/facts-by-relation.txt`](runs/facts-by-relation.txt) | the facts each view draws, by relation |
| [`runs/before-badges.txt`](runs/before-badges.txt) | every `«badge»` the before view wrote |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt), [`runs/draw-times.tsv`](runs/draw-times.tsv) | every command's exit code; Graphviz's seconds a drawing |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/37-Q-M1f.426/rerun.sh
```

## 7. Found while building

- **The view no longer draws an import.** Before, `(import std.algebra …)` was a box `std.algebra\n«import»`; today an
  import is no fact, and nothing of it is drawn. The program view says the same of an import (*an `(import …)` and a
  `(config …)` draw nothing*); this view's section of `12-rendering.md` does not mention imports.
- **The view is a two-rank strip under `dot`**: every octagon in one column and every atom in the next, so zebra2's is
  a column 20 octagons tall with crossing arrows. The view suggests no layout; `fdp`, which the KB view suggests, is
  not tried here.
- A program's own facts of a relation it declares are never drawn, however structural: zebra2's `right-of` facts
  (`:source "condition (1)"`) are not, nor would they be without a source.

## 8. Your call

- **(a)** each fact drawn as a fact, the declarations among them, no name read (default, built).
- **(b)** the declarations left out by the name `relation`.
