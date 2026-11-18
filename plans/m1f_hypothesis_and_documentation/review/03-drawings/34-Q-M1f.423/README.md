# Q-M1f.423 — how a fact's arrows are numbered

## 1. The question

Every view of `ein render` now draws a fact as the knowledge base has it: an octagon with no name in it, one arrow
to its relation's atom and one to each argument, in order. You asked for the arrows to be numbered (*"number the
arrows 1/2"*). The build numbered them **as the reasoning pages count a node's arrows**: the first arrow, the one at
the relation, is `1`, and the arguments' arrows are `2`, `3`, … in order, so in `(owns Bob Dog)` the arrow to `owns`
is 1, to `Bob` 2, to `Dog` 3. The other reading is that the numbers count the **arguments**: `Bob` 1, `Dog` 2, and
the arrow to the relation unnumbered (or `0`). A folded arrow (`--fold`) carries the relation's name and no number in
either reading.

Recorded: [`open_questions.md` § Q-M1f.423](../../../open_questions.md#q-m1f423--how-the-arrows-of-a-fact-are-numbered)
(put, not ruled; default (a)).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; Graphviz 16.0.0. Every number is the arrow's `xlabel`, digits
alone, no `#` ([Q-M1f.435](../../../open_questions.md#q-m1f435--where-an-arrows-number-is-written) is where it is
written).

**The KB view of the small program** ([`programs/small.ein`](programs/small.ein): two declarations, a fact of each,
a fact of one argument, a fact claimed only and one held and claimed, a stored negation, a fact that applies a rule,
three derived facts), drawn with `fdp`, the layout the view suggests:
[`renders/small.kb.dot`](renders/small.kb.dot) / [`.svg`](renders/small.kb.svg). Each fact of `owns` has `1` at
`owns`, `2` at the owner, `3` at the pet; a declaration, `(relation owns Person Pet)`, has `1` at `relation`, `2` at
`owns`, `3` and `4` at its labels; the loader's `(relation owns)` and the fact `(symmetric friends)` have `1` and
`2`; a stored negation has `1` at `not` and `2` at the fact it denies.

![ein render kb of the small program, fdp: every fact an octagon, its arrow 1 at its relation's box and 2, 3, 4 at its arguments; the declarations' arrow 1 at relation](renders/small.kb.svg)

What the numbers are, counted over that drawing ([`runs/small.kb.xlabels.txt`](runs/small.kb.xlabels.txt)): 16
arrows `1`, 16 `2`, 10 `3`, 2 `4`, beside the notes `(1)`, `(2)`, `by pet-owner` (2), `by symmetric`.

**A rule's panels** number a pattern the same way, the arrow `1` at the relation's box or at the variable's diamond
where the relation is a variable: `symmetric`'s `(?rel ?a ?b)` has `1` at `?rel`
([`renders/small.rules.01-rule_symmetric_lhs_rhs.svg`](renders/small.rules.01-rule_symmetric_lhs_rhs.svg); the
whole file [`renders/small.rules.dot`](renders/small.rules.dot), split by digraph).

![ein render rules, the rule symmetric: in match and in assert one octagon, its arrow 1 at the diamond ?rel and 2, 3 at ?a and ?b (swapped in assert)](renders/small.rules.01-rule_symmetric_lhs_rhs.svg)

**A trace's slice** numbers alike: step 1 of the small program's root derivation, `symmetric` from
`(friends Bob Carol)` to `(friends Carol Bob)`, the derived fact bold
([`runs/small.trace.md`](runs/small.trace.md), its four slices cut out as `renders/small.trace.root.step1.dot` to
`step4.dot`):

![the trace's slice of step 1: two octagons, each with 1 at friends and 2, 3 at Bob and Carol, the rule's box between them](renders/small.trace.root.step1.svg)

**The reasoning pages' own drawing** of the graph, `docs/ein/reasoning/01-static-ir/diagrams/01-canonical-graph.dot`,
copied ([`renders/reference.01-canonical-graph.dot`](renders/reference.01-canonical-graph.dot) /
[`.svg`](renders/reference.01-canonical-graph.svg)): `1` at the relation, `2` and `3` at the arguments, as its page
says a node's arrows are *counted from the first*. The binary's numbering is that one. (The page writes its numbers
as `label`s, the binary as `xlabel`s: Q-M1f.435.)

![the reasoning page's canonical graph: four facts f1 to f4, each with 1 at owns, lives-in or left-of and 2, 3 at its arguments](renders/reference.01-canonical-graph.svg)

**On a real program**, zebra2's KB view ([`programs/zebra2.ein`](programs/zebra2.ein), a copy of
`examples/zebra2.ein`): 452 facts, 1 446 arrows, every one numbered (452 `1`, 452 `2`, 351 `3`, 87 `4`, 68 `5`, 36
`6`: a `co-located` fact has five arrows, an `adjacent-via` fact six), `fdp` 13.9 to 29.8 s
([`runs/draw-times.tsv`](runs/draw-times.tsv)): [`renders/zebra2.kb.dot`](renders/zebra2.kb.dot) /
[`.svg`](renders/zebra2.kb.svg). At that size no number can be read: the picture is a mass of arrows around the
boxes `is-a` and `not` (see *Found while building*).

## 3. The options

### (a) The first arrow `1`, the arguments `2`, `3`, … (the default, built)

As above: the numbers count a fact's arrows from the first, as the reasoning pages count them, so a drawing and the
page's $o_1, o_2, \dots$ agree, and the relation is one of the numbered things a fact points at.

### (b) The arguments `1`, `2`, …; the first arrow unnumbered, or `0`

The numbers count the arguments, as an argument's place is spoken of (*the first argument of `owns`*). Against it:
your ruling on a `:why`'s placeholders (S1f.23.5, ruled and not built: *"{1} is the same as 1-arrow of this fact,
{2} 2nd-arrow"*) counts a form's arrows from the relation as 1, which is (a)'s count; under (b) the drawing's `2`
would be a `:why`'s `{?3}`. Needs a code change: **mocks**, today's DOT with each number moved down one, nothing else
edited:

- the first arrow unnumbered (its `xlabel` deleted):
  [`renders/small.kb.option-b-unnumbered.MOCK.dot`](renders/small.kb.option-b-unnumbered.MOCK.dot) /
  [`.svg`](renders/small.kb.option-b-unnumbered.MOCK.svg), and a rule's panel,
  [`renders/small.rule-symmetric.option-b-unnumbered.MOCK.svg`](renders/small.rule-symmetric.option-b-unnumbered.MOCK.svg)
  ([dot](renders/small.rule-symmetric.option-b-unnumbered.MOCK.dot));
- the first arrow `0`: [`renders/small.kb.option-b-zero.MOCK.dot`](renders/small.kb.option-b-zero.MOCK.dot) /
  [`.svg`](renders/small.kb.option-b-zero.MOCK.svg).

![MOCK, option (b): the same drawing, each fact's arrows to its arguments numbered 1, 2, 3 and its arrow to the relation unnumbered](renders/small.kb.option-b-unnumbered.MOCK.svg)

Under (b) a declaration's arrow to the relation it declares is `1` and its labels `2`, `3`; a stored negation's
arrow to the fact it denies is `1`.

## 4. The default

**(a)**, taken by S1f.23.1: the first arrow is `1`, at the relation, and the arguments' are `2`, `3`, … in order, as
the reasoning pages number them (`01-canonical-graph.dot`, *counted from the first*); digits alone in every view; a
folded arrow carries the relation's name and no number. Built in `ein-render/src/fact_graph.rs`, `rules.rs`,
`slice.rs`, `derivation.rs`, `ir_dot.rs`.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | re-takes every run and drawing below |
| [`programs/small.ein`](programs/small.ein) | the small program, written for the drawings' review |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| [`renders/small.kb.dot`](renders/small.kb.dot) / [`.svg`](renders/small.kb.svg) | `ein render kb`, canonical, `fdp` |
| [`renders/small.kb.fold.dot`](renders/small.kb.fold.dot) / [`.svg`](renders/small.kb.fold.svg) | `ein render kb --fold`: the folded arrows carry `owns`, `friends (2)`, no number |
| [`renders/small.rules.dot`](renders/small.rules.dot) | `ein render rules`, two digraphs |
| [`renders/small.rules.01-rule_symmetric_lhs_rhs.dot`](renders/small.rules.01-rule_symmetric_lhs_rhs.dot) / [`.svg`](renders/small.rules.01-rule_symmetric_lhs_rhs.svg) | its first part, `symmetric` |
| [`renders/small.rules.02-rule_pet_owner_lhs_rhs.dot`](renders/small.rules.02-rule_pet_owner_lhs_rhs.dot) / [`.svg`](renders/small.rules.02-rule_pet_owner_lhs_rhs.svg) | its second, `pet-owner` |
| [`runs/small.trace.md`](runs/small.trace.md) | `ein solve --trace`, the region trace; [`runs/small.trace.stdout.txt`](runs/small.trace.stdout.txt) its read-out |
| [`renders/small.trace.root.step1.dot`](renders/small.trace.root.step1.dot) / [`.svg`](renders/small.trace.root.step1.svg) … [`step4`](renders/small.trace.root.step4.svg) | the trace's four slices, cut from it |
| [`renders/reference.01-canonical-graph.dot`](renders/reference.01-canonical-graph.dot) / [`.svg`](renders/reference.01-canonical-graph.svg) | the reasoning page's drawing, copied |
| [`renders/zebra2.kb.dot`](renders/zebra2.kb.dot) / [`.svg`](renders/zebra2.kb.svg) | `ein render kb` of zebra2, `fdp` |
| [`renders/small.kb.option-b-unnumbered.MOCK.dot`](renders/small.kb.option-b-unnumbered.MOCK.dot) / [`.svg`](renders/small.kb.option-b-unnumbered.MOCK.svg) | **mock**, option (b), the first arrow unnumbered |
| [`renders/small.kb.option-b-zero.MOCK.dot`](renders/small.kb.option-b-zero.MOCK.dot) / [`.svg`](renders/small.kb.option-b-zero.MOCK.svg) | **mock**, option (b), the first arrow `0` |
| [`renders/small.rule-symmetric.option-b-unnumbered.MOCK.dot`](renders/small.rule-symmetric.option-b-unnumbered.MOCK.dot) / [`.svg`](renders/small.rule-symmetric.option-b-unnumbered.MOCK.svg) | **mock**, option (b) in a rule's panel |
| [`runs/small.kb.xlabels.txt`](runs/small.kb.xlabels.txt) | every `xlabel` of the small KB view, counted |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | every command and its exit code (all 0) |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds, a drawing |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/34-Q-M1f.423/rerun.sh
```

It runs `build/review-bin/ein-head-17085b27` on the two programs, cuts the trace's slices, copies the reasoning
page's drawing, draws every `.dot` with the layout it suggests (`../tools/draw.sh`), and draws the three mocks from
their hand-edited DOT, which it does not touch.

## 7. Found while building

- **zebra2's whole KB, canonical, cannot be read** at any size: 1 446 numbered arrows under `fdp`, the boxes `is-a`
  (every type fact) and `not` (every stored negation) each at the centre of a star of a hundred arrows, the notes
  `by transitive`, `by includes`, `by std.bijection.functional-negative` written over one another. The numbers are
  legible on the small program, in a rule's panels and in a slice. Not this question's: what a page shows of a large
  knowledge base is [Q-M1f.468](../../../open_questions.md#q-m1f468--a-pages-kb-drawing-the-whole-view-or-a-cut)'s
  (a cut), and how `dot` and `fdp` lay one out is R-18's ([P1f.23's findings](../../../p1f.23_review_processing/findings.md)).
- `fdp` on that view took 13.9 s once and 29.8 s in the re-run, on a machine another agent was loading with the gate.
- The arrows' numbers and the frames' labels are drawn in Graphviz's default serif font, the boxes in `Inter`: the
  views set `fontname` on nodes only.

## 8. Your call

- **(a)** the first arrow `1`, at the relation, the arguments `2`, `3`, … (default, built).
- **(b)** the arguments `1`, `2`, …, the first arrow unnumbered (or `0`).
