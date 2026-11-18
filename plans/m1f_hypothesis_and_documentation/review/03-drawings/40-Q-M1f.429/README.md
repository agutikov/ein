# Q-M1f.429 — a folded fact in a drawing of a derivation

## 1. The question

You ruled that the trace's drawings fold under `--fold` too (Q-M1f.422). In a trace's slice every fact is the end of
an arrow: a premise feeds the firing's box, the firing's box points at what it derives. An arrow cannot end at an
arrow, so a fact folded into a labelled arrow has nowhere for the firing's arrows to meet it. The build folds it
**through a point**: a fact of three arrows that no fact of the slice points at is an arrow from its first argument
to a small point, labelled with its relation, and on from the point to its second argument; the firing's arrows
enter and leave the point. The other reading: under `--fold` a slice folds nothing, and only the `-F` snapshots (each
traced state's whole knowledge base, a KB view) fold.

Recorded: [`open_questions.md` § Q-M1f.429](../../../open_questions.md#q-m1f429--a-folded-fact-in-a-drawing-of-a-derivation)
(put, not ruled; default (a)).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; slices laid out by `dot`, left to right; a snapshot by `fdp`, as
it suggests.

**A chain of facts of three arrows**, [`programs/chain.ein`](programs/chain.ein): `right-of` is transitive and makes
neighbours. Step 1, `transitive`, from `(right-of H3 H2)` and `(right-of H2 H1)` to `(right-of H3 H1)`, canonical
([`runs/chain.trace.md`](runs/chain.trace.md)): three octagons, each with its arrows `1` at `right-of` and `2`, `3` at
the houses:

![step 1 without --fold: three octagons pointing at right-of and the houses, the rule's box between the two premises and the bold conclusion](renders/chain.trace.root.step1.svg)

and under `--fold` ([`runs/chain.trace.fold.md`](runs/chain.trace.fold.md)): each fact an arrow `H3 — right-of → • →
H2` through a point, the firing's arrows leaving the premises' points and entering the conclusion's, which is bold:

![step 1 under --fold: three labelled arrows right-of, each through a black point; the rule's box takes its arrows from the two premises' points and gives one to the conclusion's point](renders/chain.trace.fold.root.step1.svg)

Every slice of chain.ein, small.ein and four of zebra2's, counted (arrows and octagons, without/with `--fold`, and
the points under `--fold`):

| slice | arrows | octagons | points |
|---|---|---|---|
| chain, step 1 (`transitive`) | 12 / 9 | 3 / 0 | 3 |
| chain, steps 2 to 4 (`neighbours`) | 8 / 6 | 2 / 0 | 2 |
| small, steps 1 and 4 (`symmetric`) | 8 / 6 | 2 / 0 | 2 |
| small, steps 2 and 3 (`pet-owner`: the conclusion has two arrows) | 7 / 6 | 2 / 1 | 1 |
| zebra2, step 46 (`symmetric`, `next-to`) | 8 / 6 | 2 / 0 | 2 |
| zebra2, step 110 (`transitive`, `is-a*`) | 12 / 9 | 3 / 0 | 3 |
| zebra2, step 231 (`std.bijection.domain-elimination`) | 23 / 20 | 6 / 3 | 3 |
| zebra2, step 277 (`co-located`) | 8 / 6 | 2 / 0 | 2 |

**On a real program**: zebra2's root, 337 steps, traced with `--single`
([`runs/zebra2.trace.md`](runs/zebra2.trace.md), [`runs/zebra2.trace.fold.md`](runs/zebra2.trace.fold.md); every
slice cut to `runs/zebra2.trace.slices/`). Step 231, `domain-elimination`, derives `(nation-loc Norwegian House-1)`:
its three premises of two and four arrows stay octagons; `(is-a Norwegian Nationality)`, `(is-a House-1 House)` and
the conclusion go through points:

![zebra2 step 231 under --fold: the premises functional, total and the declaration as octagons, the two is-a facts as labelled arrows through points, the conclusion a long bold arrow nation-loc from Norwegian through a point to House-1](renders/zebra2.trace.fold.root.step231.svg)

Canonical: [`renders/zebra2.trace.root.step231.svg`](renders/zebra2.trace.root.step231.svg).

**The `-F` snapshot, folded** ([`runs/chain.trace.fold-F.md`](runs/chain.trace.fold-F.md)): state 1's whole knowledge
base, drawn as `ein render kb --fold` draws it, `fdp`: the facts of three arrows as labelled arrows with no point, the
claimed `(right-of H1 H3)` an octagon in `claims ctx`:

![the -F snapshot of chain.ein's state, folded: right-of and next-to as labelled arrows between the houses, the declarations as octagons, the claimed fact framed](renders/chain.trace.fold-F.state1.state.svg)

## 3. The options

### (a) A labelled arrow through a point (the default, built)

As above: the slice and the snapshot fold alike, each fact of three arrows a word on an arrow, and the slice gives the
firing's arrows a point to meet.

### (b) A slice folds nothing; only the `-F` snapshots fold

Under `--fold` each slice stays as without it, every fact its octagon, and only the snapshot, a KB view, is folded.
Needs a code change. Its look needs no new drawing: the slices would be today's unfolded ones
(`renders/chain.trace.root.step*.svg`) and the snapshot today's folded one. **A mock** of the trace it would write:
[`runs/chain.trace.fold-F.option-b.MOCK.md`](runs/chain.trace.fold-F.option-b.MOCK.md), today's `--fold -F` trace of
chain.ein with each step's `dot` block replaced by the same step's block without `--fold`; nothing else edited.

## 4. The default

**(a)**, taken by S1f.23.1: a fact of three arrows that no fact of the cone points at is an arrow from its first
argument to a point, labelled with its relation, and on from the point to its second argument; the firings' arrows
enter and leave the point. Built in `ein-render/src/slice.rs`; held by `render_doors.rs`'
`every_door_draws_a_fact_in_both_forms`.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | re-takes every run and drawing |
| [`programs/chain.ein`](programs/chain.ein) | a chain of facts of three arrows, written for this item |
| [`programs/small.ein`](programs/small.ein) | the small program, written for the drawings' review |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | every command and its exit code |
| [`runs/chain.trace.fold-F.md`](runs/chain.trace.fold-F.md) | `ein solve programs/chain.ein --trace … --fold -F` |
| [`runs/chain.trace.fold-F.option-b.MOCK.md`](runs/chain.trace.fold-F.option-b.MOCK.md) | **mock**, option (b): the fold-F trace with each step's slice replaced by the unfolded one |
| [`runs/chain.trace.fold-F.stdout.txt`](runs/chain.trace.fold-F.stdout.txt) | its read-out |
| [`runs/chain.trace.fold.md`](runs/chain.trace.fold.md) | `ein solve programs/chain.ein --trace … --fold` |
| [`runs/chain.trace.fold.stdout.txt`](runs/chain.trace.fold.stdout.txt) | its read-out |
| [`runs/chain.trace.md`](runs/chain.trace.md) | `ein solve programs/chain.ein --trace …` |
| [`runs/chain.trace.stdout.txt`](runs/chain.trace.stdout.txt) | its read-out |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds, a drawing |
| [`runs/small.trace.fold-F.md`](runs/small.trace.fold-F.md) | `ein solve programs/small.ein --trace … --fold -F` |
| [`runs/small.trace.fold-F.stdout.txt`](runs/small.trace.fold-F.stdout.txt) | its read-out |
| [`runs/small.trace.fold.md`](runs/small.trace.fold.md) | `ein solve programs/small.ein --trace … --fold` |
| [`runs/small.trace.fold.stdout.txt`](runs/small.trace.fold.stdout.txt) | its read-out |
| [`runs/small.trace.md`](runs/small.trace.md) | `ein solve programs/small.ein --trace …` |
| [`runs/small.trace.stdout.txt`](runs/small.trace.stdout.txt) | its read-out |
| [`runs/zebra2.trace.fold.md`](runs/zebra2.trace.fold.md) | `ein solve programs/zebra2.ein --trace … --fold` |
| [`runs/zebra2.trace.fold.stdout.txt`](runs/zebra2.trace.fold.stdout.txt) | its read-out |
| [`runs/zebra2.trace.md`](runs/zebra2.trace.md) | `ein solve programs/zebra2.ein --trace …` |
| [`runs/zebra2.trace.stdout.txt`](runs/zebra2.trace.stdout.txt) | its read-out |
| [`renders/chain.trace.fold-F.state1.state.dot`](renders/chain.trace.fold-F.state1.state.dot) / [`.svg`](renders/chain.trace.fold-F.state1.state.svg) | the `-F` snapshot of chain's state 1, folded |
| [`renders/chain.trace.fold.root.step1.dot`](renders/chain.trace.fold.root.step1.dot) / [`.svg`](renders/chain.trace.fold.root.step1.svg) | step1 of chain's root, `--fold` |
| [`renders/chain.trace.fold.root.step2.dot`](renders/chain.trace.fold.root.step2.dot) / [`.svg`](renders/chain.trace.fold.root.step2.svg) | step2 of chain's root, `--fold` |
| [`renders/chain.trace.fold.root.step3.dot`](renders/chain.trace.fold.root.step3.dot) / [`.svg`](renders/chain.trace.fold.root.step3.svg) | step3 of chain's root, `--fold` |
| [`renders/chain.trace.fold.root.step4.dot`](renders/chain.trace.fold.root.step4.dot) / [`.svg`](renders/chain.trace.fold.root.step4.svg) | step4 of chain's root, `--fold` |
| [`renders/chain.trace.root.step1.dot`](renders/chain.trace.root.step1.dot) / [`.svg`](renders/chain.trace.root.step1.svg) | step1 of chain's root |
| [`renders/chain.trace.root.step2.dot`](renders/chain.trace.root.step2.dot) / [`.svg`](renders/chain.trace.root.step2.svg) | step2 of chain's root |
| [`renders/chain.trace.root.step3.dot`](renders/chain.trace.root.step3.dot) / [`.svg`](renders/chain.trace.root.step3.svg) | step3 of chain's root |
| [`renders/chain.trace.root.step4.dot`](renders/chain.trace.root.step4.dot) / [`.svg`](renders/chain.trace.root.step4.svg) | step4 of chain's root |
| [`renders/small.trace.fold-F.state1.state.dot`](renders/small.trace.fold-F.state1.state.dot) / [`.svg`](renders/small.trace.fold-F.state1.state.svg) | the `-F` snapshot of small's state 1, folded |
| [`renders/small.trace.fold.root.step1.dot`](renders/small.trace.fold.root.step1.dot) / [`.svg`](renders/small.trace.fold.root.step1.svg) | step1 of small's root, `--fold` |
| [`renders/small.trace.fold.root.step2.dot`](renders/small.trace.fold.root.step2.dot) / [`.svg`](renders/small.trace.fold.root.step2.svg) | step2 of small's root, `--fold` |
| [`renders/small.trace.fold.root.step3.dot`](renders/small.trace.fold.root.step3.dot) / [`.svg`](renders/small.trace.fold.root.step3.svg) | step3 of small's root, `--fold` |
| [`renders/small.trace.fold.root.step4.dot`](renders/small.trace.fold.root.step4.dot) / [`.svg`](renders/small.trace.fold.root.step4.svg) | step4 of small's root, `--fold` |
| [`renders/small.trace.root.step1.dot`](renders/small.trace.root.step1.dot) / [`.svg`](renders/small.trace.root.step1.svg) | step1 of small's root |
| [`renders/small.trace.root.step2.dot`](renders/small.trace.root.step2.dot) / [`.svg`](renders/small.trace.root.step2.svg) | step2 of small's root |
| [`renders/small.trace.root.step3.dot`](renders/small.trace.root.step3.dot) / [`.svg`](renders/small.trace.root.step3.svg) | step3 of small's root |
| [`renders/small.trace.root.step4.dot`](renders/small.trace.root.step4.dot) / [`.svg`](renders/small.trace.root.step4.svg) | step4 of small's root |
| [`renders/zebra2.trace.fold.root.step110.dot`](renders/zebra2.trace.fold.root.step110.dot) / [`.svg`](renders/zebra2.trace.fold.root.step110.svg) | step110 of zebra2's root, `--fold` |
| [`renders/zebra2.trace.fold.root.step231.dot`](renders/zebra2.trace.fold.root.step231.dot) / [`.svg`](renders/zebra2.trace.fold.root.step231.svg) | step231 of zebra2's root, `--fold` |
| [`renders/zebra2.trace.fold.root.step277.dot`](renders/zebra2.trace.fold.root.step277.dot) / [`.svg`](renders/zebra2.trace.fold.root.step277.svg) | step277 of zebra2's root, `--fold` |
| [`renders/zebra2.trace.fold.root.step46.dot`](renders/zebra2.trace.fold.root.step46.dot) / [`.svg`](renders/zebra2.trace.fold.root.step46.svg) | step46 of zebra2's root, `--fold` |
| [`renders/zebra2.trace.root.step110.dot`](renders/zebra2.trace.root.step110.dot) / [`.svg`](renders/zebra2.trace.root.step110.svg) | step110 of zebra2's root |
| [`renders/zebra2.trace.root.step231.dot`](renders/zebra2.trace.root.step231.dot) / [`.svg`](renders/zebra2.trace.root.step231.svg) | step231 of zebra2's root |
| [`renders/zebra2.trace.root.step277.dot`](renders/zebra2.trace.root.step277.dot) / [`.svg`](renders/zebra2.trace.root.step277.svg) | step277 of zebra2's root |
| [`renders/zebra2.trace.root.step46.dot`](renders/zebra2.trace.root.step46.dot) / [`.svg`](renders/zebra2.trace.root.step46.svg) | step46 of zebra2's root |
| `runs/zebra2.trace.slices/zebra2.trace[.fold].{root,state1}.stepN.dot` | every slice of the two zebra2 traces, cut from them (1 162, not drawn; four of root's each way are copied to `renders/`) |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/40-Q-M1f.429/rerun.sh
```

It traces chain.ein and small.ein three ways and zebra2 two ways with today's binary, cuts the slices, copies four of
zebra2's to `renders/`, and draws them. The mock is a hand-edited trace and is not drawn.

## 7. Found while building

- **The point puts the relation's word on the first half of the arrow only**: `H3 — right-of → •` has the word, `• →
  H2` none and an arrowhead, so the fact reads as two arrows of which the second is unlabelled; in a long slice
  (zebra2, step 231) the conclusion's labelled half is the longest arrow of the drawing.
- **A slice is black, a snapshot coloured**: the slice's facts and their arrows are black (the rule's box and its
  arrows take the rule's colour), where the `-F` snapshot colours each fact by its relation, as the KB view does.
- **A fact that applies a rule is no premise**: zebra2's step 277, `co-located`, lists `(color-loc Yellow House-1)`
  as its one premise; `(co-located smoke-loc Kools color-loc Yellow)`, which the rule's parameters read, is not drawn.
- Tracing zebra2 writes 1 162 slices (`--single`: root 337 steps, the state 244, each twice); the rule boxes carry the
  rule's sentence, which for `domain-elimination` is a line wider than the rest of the drawing.

## 8. Your call

- **(a)** a fact of three arrows in a slice: a labelled arrow through a point (default, built).
- **(b)** a slice folds nothing under `--fold`; only the `-F` snapshots fold.
