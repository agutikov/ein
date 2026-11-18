# Drawings

[← the bundle](../README.md)

**The user's third look.** The second look's 24 items were answered by the user (`e4c0eaf6`) and left the bundle;
P1f.23's [S1f.23.6](../../p1f.23_review_processing/s1f.23.6_the_drawings_answers.md) built the answers: the layered
layout, the fold of a fact of two arrows, no arrow to or from an arrow, a derivation's contrast and width, and R-33's
refused slices. Every answer is filed as ruled. **56 to 60 are new**: the readings the stage took where the answers
are silent, each built with its default, and R-33's result.

## What the drawings are now

This stage's binary, `build/review-bin/ein-s1f236`; the binary at the stage's start, `ein-head-6de9101b`, is
*before*.

- **Layers** (56): a fact's first arrow is written from its relation to the fact and drawn back,
  `"rel" -> f [xlabel="1", dir=back]`, and a view of a knowledge base writes its layers, objects and facts in turn,
  a band of facts as deep as its facts nest (your note during the stage): the five layers you described in the simple
  case, and the sandwich kept with nested facts. No KB view suggests
  `fdp` any more, and every drawing here is drawn by `dot` ([`tools/draw.sh`](../tools/draw.sh)).
- **`--fold`** (57, 58): a fact of three arrows that nothing drawn points at is a labelled arrow, a fact of two a
  labelled loop on its argument; a folded arrow runs within a layer, its names where the canonical drawing puts them.
- **A derivation** (59): its arrows and steps dark and twice the default width; a fact its arrows touch is never
  folded, so a step's slice is the same with `--fold`, and no point is drawn.
- **R-33** (60): every slice of every corpus trace put to Graphviz, before and after.
- **The pages**: every copy of a command's output under `docs/` drawn again in the layers (a cut kept a cut), the
  rendering page's new section *Layers* and its drawing `12-layers`, the standard library's rule drawings
  regenerated, and the decisions for the ruled questions written (`docs/decisions/cli/`, `docs/decisions/docs/`).

A `.MOCK.` file is a drawing edited by hand to show an option, and the item says what was edited.

| # | item | the question | default |
|---|---|---|---|
| 56 | [the layers](56-layers/README.md) | the layered layout as built, nested facts in bands (your note), on five programs | (a) layers and bands written |
| 57 | [a folded arrow's rank](57-folded-ranks/README.md) | under `--fold`, a folded arrow within a layer, or ranked as any arrow | (a) within a layer |
| 58 | [the two-arrow fold](58-two-arrow-fold/README.md) | a stored negation folds into a loop on the fact it denies; five closing rules now drawn folded | (a) as built |
| 59 | [a derivation's drawing](59-derivation/README.md) | the palette, the widths, a slice's layers, and `--fold` changing no step's slice | (a) as built |
| 60 | [R-33](60-R-33/README.md) | the slices Graphviz refused: refused no more; a per-commit check of two traces | (a) as built |
