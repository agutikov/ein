# 57 · Under `--fold`, how a folded arrow is laid out

[← the topic](../README.md) · [← the bundle](../../README.md)

## The question

In the canonical form a fact's arguments stand in one layer, under the fact. Folded, the fact is an arrow from its
first argument to its second (or a loop on its one argument), and Graphviz either lets that arrow rank, its second
argument a layer below its first, or not (`constraint=false`). S1f.23.6 first built the arrow ranked, then, on the
brief that continued the stage (*"folded arrows within a layer"*), built it **within a layer**: the folded arrow ranks
nothing, and its fact, though no node, places its names where its octagon would, so the folded drawing keeps the
canonical drawing's layers of objects and loses only the folded facts' ranks. Recorded in
[S1f.23.6's Record](../../../p1f.23_review_processing/s1f.23.6_the_drawings_answers.md#record) (§ 1, *Folded arrows,
within a layer*; reading 2) and the decision
[Q21](../../../../../docs/decisions/cli/Q21-every-form-has-a-fixed-dot-shape-and-its-layout.md). The two read
differently where most facts fold, and you may prefer the other.

## What is so today, and the other

*After* is this stage's binary (`ein-s1f236`), a folded arrow within a layer; *option (b)* is the stage's first build
(`ein-s1f236-ranked`, `ad20a5e0`'s code), a folded arrow ranked as any arrow: both are the binary's output, no mock.
*Before* is the stage's start. `runs/NAME.option-*.layers.txt` are the ranks `dot` makes, top first (anchors left
out); `runs/NAME.option-b-vs-a.diff` the DOT's difference.

| program | (a) as built: within a layer | (b) ranked as any arrow |
|---|---|---|
| the five-layer program ([`five-layers.ein`](programs/five-layers.ein)), `ein render kb --fold` | [svg](renders/five-layers.kb-fold.after.svg), [ranks](runs/five-layers.kb-fold.option-a.layers.txt): four layers, `relation` · the two declarations · the relations and labels · the six plain objects, the `owns` and `lives-in` arrows between them | [svg](renders/five-layers.kb-fold.option-b-ranked.svg), [ranks](runs/five-layers.kb-fold.option-b.layers.txt): three layers, `Alice` and `Bob` on the top one beside `relation`, their objects one below |
| zebra2 ([`zebra2.ein`](programs/zebra2.ein)), `ein render program --fold`, its `kb` | [svg](renders/zebra2.program-kb-fold.after.svg), [ranks](runs/zebra2.program-kb-fold.option-a.layers.txt): four layers, the 38 names of the `is-a`, `right-of` and colour facts on the bottom one, arcs between them | [svg](renders/zebra2.program-kb-fold.option-b-ranked.svg), [ranks](runs/zebra2.program-kb-fold.option-b.layers.txt): six layers, the houses a chain by `right-of` |
| beliefs ([`01_common_belief.ein`](programs/01_common_belief.ein)), `ein render kb --fold` | [svg](renders/beliefs.kb-fold.after.svg), [ranks](runs/beliefs.kb-fold.option-a.layers.txt): six ranks, the band of the believed facts kept, each `believes` an arrow from a person on the bottom layer up to the fact believed | [svg](renders/beliefs.kb-fold.option-b-ranked.svg), [ranks](runs/beliefs.kb-fold.option-b.layers.txt) |
| the page drawing `05-row-state` (`04-stdlib/`), every fact folded | [svg](renders/row-state.page.after.svg), [ranks](runs/row-state.page.option-a.layers.txt): one layer of nine names, every arrow an arc over it | [svg](renders/row-state.page.option-b-ranked.svg), [ranks](runs/row-state.page.option-b.layers.txt): eight layers, a chain |

Every drawing on the pages that is a folded view of a knowledge base (thirteen) was redrawn within a layer, by a
transformer validated against the binary over the corpus (S1f.23.6's Record, § 7).

## The options

- **(a) Within a layer** (default, built): a folded arrow ranks nothing; its names stand where the canonical drawing
  puts them.
- **(b) Ranked as any arrow**: its second argument a layer below its first, so chains read down the page, and the
  layers the view writes hold only what the octagons place.

## The default

(a): it is the brief's target, and it keeps the sandwich of the canonical drawing, folding only removes the facts'
ranks; (b) reads better where most facts fold and form chains (the row state, zebra2's houses), which is why it is
put to you.

## Files

- `programs/five-layers.ein` (written for item 56), `programs/zebra2.ein` (from `examples/`),
  `programs/01_common_belief.ein` (from `examples/beliefs/`); the row state is the page's drawing,
  `docs/ein/user/04-reference/04-stdlib/diagrams/05-row-state.dot`, as it is now and as it was at `ad20a5e0`.
- `renders/NAME.after.dot|svg` (option a), `renders/NAME.option-b-ranked.dot|svg` (option b),
  `renders/NAME.before.dot|svg` (the stage's start).
- `runs/NAME.option-a.layers.txt`, `runs/NAME.option-b.layers.txt`, `runs/NAME.option-b-vs-a.diff`, `runs/*.diff`
  (before against after).

## Re-run

`bash plans/m1f_hypothesis_and_documentation/review/03-drawings/tools/build.sh` from the repo root, with
`build/review-bin/ein-s1f236`, `ein-s1f236-ranked` and `ein-head-6de9101b`.

## Your call

- (a) within a layer.
- (b) ranked as any arrow.
