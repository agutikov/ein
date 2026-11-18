# 56 · The layered layout, as built, with nested facts in bands

[← the topic](../README.md) · [← the bundle](../../README.md)

## The question

You asked for a layered layout by default (L-100, and R-18 *"same as L-100"*): objects, facts, objects, …; in the
simple case five layers, `relation` on top, the declarations, the relations and labels, the relations' facts, the
plain objects, and a relation's properties at `relation`'s level. During the stage you added: *"maybe we still can
combine nested facts and layered layout if allow have multiple layers of nested facts, so still have
objects-facts-objects sandwich, but facts can have multiple layers (ranks); And global-only facts can be on one
layer/rank, and nested go under on lower rank"*.

S1f.23.6 built both: **a fact's first arrow is written from its relation to the fact and drawn back**,
`"owns" -> f [xlabel="1", dir=back]`, and **a view of a knowledge base writes its layers for `dot`**, one
`{ rank=same; … }` group a rank under `// layers`, ordered by a chain of invisible anchors: a layer of objects, a
band of facts, a layer of objects, …; in a band, the facts nothing points at on its top rank, a fact under another's
arrow a rank below it, deeper nesting lower still; the band's names below the whole band. The layers are read off the
arrows alone, no name. Recorded in [S1f.23.6's Record](../../../p1f.23_review_processing/s1f.23.6_the_drawings_answers.md#record)
(§ 1, reading 1) and the decision [Q21](../../../../../docs/decisions/cli/Q21-every-form-has-a-fixed-dot-shape-and-its-layout.md).
This item shows it on five programs against the drawing by the arrows alone, which is what `dot` makes of the same
DOT with the groups taken out.

## What is so today

*Before* is the binary at the stage's start (`ein-head-6de9101b`, the KB views drawn by `fdp` as their DOT then
suggested); *after* this stage's (`ein-s1f236`); the `.MOCK.` drawing of each is the *after* DOT with its `// layers`
lines taken out (the only edit). `runs/NAME.after.layers.txt` and `runs/NAME.option-b.layers.txt` list the ranks `dot`
makes, top first (anchors left out).

| program | as built: bands | by the arrows alone (mock) |
|---|---|---|
| the simple case, [`five-layers.ein`](programs/five-layers.ein), `ein render kb` | [svg](renders/five-layers.kb.after.svg), [ranks](runs/five-layers.kb.after.layers.txt): the five layers, `symmetric` beside `relation` | [svg](renders/five-layers.kb.option-b-arrows-alone.MOCK.svg): the same five; before the stage, two ([svg](renders/five-layers.kb.before.svg)) |
| one and two levels of nesting, [`01_common_belief.ein`](programs/01_common_belief.ein), `ein render kb` | [svg](renders/common-belief.kb.after.svg), [ranks](runs/common-belief.kb.after.layers.txt): `relation` · the declarations · every relation and label · the twelve facts nothing points at · the four under `believes`' arrows · `(likes Dan Fish)` under `not`'s · every name | [svg](renders/common-belief.kb.option-b-arrows-alone.MOCK.svg), [ranks](runs/common-belief.kb.option-b.layers.txt): `likes` among the facts, `Ann`, `Bob`, `Cid` among the nested facts, the names over three ranks |
| negations of negations, [`06_double_negation.ein`](programs/06_double_negation.ein), `ein render kb` | [svg](renders/double-negation.kb.after.svg), [ranks](runs/double-negation.kb.after.layers.txt): two bands, the first two ranks deep (`(not (not g))`), the second five (`(think Bob (not (not (= 7 (+ 3 4)))))` and what the rules derived), every name on the bottom layer | [svg](renders/double-negation.kb.option-b-arrows-alone.MOCK.svg), [ranks](runs/double-negation.kb.option-b.layers.txt): objects and facts mixed on five of nine ranks |
| zebra2, [`zebra2.ein`](programs/zebra2.ein), `ein render program`, its `kb` | [svg](renders/zebra2.program-kb.after.svg), [ranks](runs/zebra2.program-kb.after.layers.txt): five layers; `co-located` on the top one, since its facts' arguments include relations, `(co-located nation-loc Englishman color-loc Red)` | [svg](renders/zebra2.program-kb.option-b-arrows-alone.MOCK.svg): seven ranks |
| a calculator program, [`add.ein`](programs/add.ein), `ein render program`, its `kb` | [svg](renders/add.program-kb.after.svg), [ranks](runs/add.program-kb.after.layers.txt): `add`, `let`, `n`; one band seven ranks deep, as deep as the numerals nest; every digit, `z`, `apples` and `pears` | [svg](renders/add.program-kb.option-b-arrows-alone.MOCK.svg): the digits spread over the band's ranks |

A rule's panels are laid out by their arrows alone, each pattern's relation above it and its variables below
([`five-layers-rule.ein`](programs/five-layers-rule.ein): [after](renders/five-layers.rule.after.svg),
[before](renders/five-layers.rule.before.svg)); so are a trace's slices and the derivation graph, where a step stands
between facts. Every drawing under `docs/` that is a command's view of a knowledge base was drawn again with its
layers, and the rendering page has a section and a drawing for them (`12-layers`).

## The options

- **(a) As built** (default): layers and bands written, from the arrows alone.
- **(b) The arrows alone**: no groups written; `dot` ranks by the arrows' directions, and nested facts break the
  sandwich (the mocks).

## The default

(a): it is your note, built; no test holds a layout (your rule: tests hold structure), so either is a preference.
Where the arrows go round (a fact of a relation whose own name is under its arrows) no layers are written and `dot`
finds its own.

## Files

- `programs/five-layers.ein`, `five-layers-rule.ein`: written for this item; `01_common_belief.ein` and
  `06_double_negation.ein` from `examples/beliefs/`, `zebra2.ein` from `examples/`, `add.ein` from `examples/arith/`.
- `renders/NAME.before.dot|svg`, `renders/NAME.after.dot|svg`, `renders/NAME.option-b-arrows-alone.MOCK.dot|svg`.
- `runs/NAME.diff`: `diff -u` of the two DOTs; `runs/NAME.*.layers.txt`: the ranks; `runs/*.err`: stderr (empty).

## Re-run

From the repo root, with `build/review-bin/ein-s1f236` (this stage's binary) and `build/review-bin/ein-head-6de9101b`
(the stage's start): `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/tools/build.sh`.

## Your call

- (a) as built.
- (b) the arrows alone.
