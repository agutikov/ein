# 61 · A concentric layout: the layers drawn as rings

[← the topic](../README.md) · [← the bundle](../../README.md)

## The question

On item 56 you wrote: *"Perfect! (a) But large diagrams still look messy. Does Graphviz have circular/concentric
layout? Objects will be outside, then facts, then relations and types opbejcts, then relations definitions and
properties facts and in the center `relation` `symmetric` and other, is it possible?"*, and on 57: *"for large
diagrams concentric layout maybe can improve visibility."*

Your rings, inside out, are the layered layout's layers read from the top: `relation` and `symmetric`; the
declarations and the facts of `symmetric`; the relations and their labels; the facts; the plain objects. A band of
facts as deep as its facts nest is as many rings.

**Short answer: yes, two ways, neither built.** Graphviz has a radial layout, `twopi`, which puts each node on the
ring of its graph distance from a root; with a hidden centre joined to the top layer its rings are your rings in the
simple case and not in general. And any layout can be drawn with **positions given**: `dot` lays the layers out as
today, each layer becomes a ring and each node's place in its layer an angle, and Graphviz draws the arrows between
pinned nodes. That gives your rings exactly. Both were mocked from this stage's binary's DOT, which for every view
here is byte for byte the binary you answered on; **nothing in the engine changed**.

## The ways, each drawn

| way | how | your rings |
|---|---|---|
| **layered** | `dot`, as today | the layers, top to bottom |
| **twopi** | `twopi`, a hidden centre joined to the top layer, `root` it; Graphviz's default ring spacing | rings by graph distance |
| **twopi-fit** | the same, each ring's radius computed to hold its nodes and given as `ranksep`'s list | rings by graph distance |
| **bent** | `dot`'s layers as rings, the top layer the centre; a node's angle its place in its layer, each ring just large enough for its nodes; positions pinned, `neato -n2` | exactly |
| **bent-bary** | the same, each ring's order then refined for a circle: a node moved toward the circular mean of its neighbours' angles on the next ring, six sweeps inside out and back | exactly |
| **annulus** | the same with a node's angle its x in `dot`'s drawing: the layered drawing itself rolled into an annulus, every crossing `dot` left kept | exactly |
| **circo** | `circo`, on the four small programs only | none: it puts each biconnected component on a circle and knows no layer |

The radial ways' arrows are straight. Routed round the nodes (`--splines true`), the bent drawing of zebra2's
knowledge base took 240 s on this host where straight it takes a tenth of a second; the router is also what the
runner's Graphviz build refuses some drawings in (AGENTS.md, *Which Graphviz build*).

## The pictures

Each sheet puts one case's six ways side by side, drawn by this host's Graphviz 16.0.0 (`host`) or the CI image's
14.1.2 (`ci`); every way's own drawing is linked beside it.

| case | program | sheet | layered | twopi | twopi-fit | bent | bent-bary | annulus | circo |
|---|---|---|---|---|---|---|---|---|---|
| your five layers | [`five-layers.ein`](programs/five-layers.ein), `ein render kb` | [host](renders/five-layers.kb.side-by-side.host.png) · [ci](renders/five-layers.kb.side-by-side.ci.png) | [h](renders/five-layers.kb.layered.host.svg) [c](renders/five-layers.kb.layered.ci.svg) | [h](renders/five-layers.kb.twopi.host.svg) [c](renders/five-layers.kb.twopi.ci.svg) | [h](renders/five-layers.kb.twopi-fit.host.svg) [c](renders/five-layers.kb.twopi-fit.ci.svg) | [h](renders/five-layers.kb.bent.host.svg) [c](renders/five-layers.kb.bent.ci.svg) | [h](renders/five-layers.kb.bent-bary.host.svg) [c](renders/five-layers.kb.bent-bary.ci.svg) | [h](renders/five-layers.kb.annulus.host.svg) [c](renders/five-layers.kb.annulus.ci.svg) | [h](renders/five-layers.kb.circo.host.svg) [c](renders/five-layers.kb.circo.ci.svg) |
| a rule's panels | [`five-layers-rule.ein`](programs/five-layers-rule.ein), `ein render rule --name pet-of-owner` | [host](renders/five-layers-rule.rule.side-by-side.host.png) · [ci](renders/five-layers-rule.rule.side-by-side.ci.png) | [h](renders/five-layers-rule.rule.layered.host.svg) [c](renders/five-layers-rule.rule.layered.ci.svg) | [h](renders/five-layers-rule.rule.twopi.host.svg) [c](renders/five-layers-rule.rule.twopi.ci.svg) | [h](renders/five-layers-rule.rule.twopi-fit.host.svg) [c](renders/five-layers-rule.rule.twopi-fit.ci.svg) | [h](renders/five-layers-rule.rule.bent.host.svg) [c](renders/five-layers-rule.rule.bent.ci.svg) | [h](renders/five-layers-rule.rule.bent-bary.host.svg) [c](renders/five-layers-rule.rule.bent-bary.ci.svg) | [h](renders/five-layers-rule.rule.annulus.host.svg) [c](renders/five-layers-rule.rule.annulus.ci.svg) | [h](renders/five-layers-rule.rule.circo.host.svg) [c](renders/five-layers-rule.rule.circo.ci.svg) |
| nested facts | [`01_common_belief.ein`](programs/01_common_belief.ein), `ein render kb` | [host](renders/common-belief.kb.side-by-side.host.png) · [ci](renders/common-belief.kb.side-by-side.ci.png) | [h](renders/common-belief.kb.layered.host.svg) [c](renders/common-belief.kb.layered.ci.svg) | [h](renders/common-belief.kb.twopi.host.svg) [c](renders/common-belief.kb.twopi.ci.svg) | [h](renders/common-belief.kb.twopi-fit.host.svg) [c](renders/common-belief.kb.twopi-fit.ci.svg) | [h](renders/common-belief.kb.bent.host.svg) [c](renders/common-belief.kb.bent.ci.svg) | [h](renders/common-belief.kb.bent-bary.host.svg) [c](renders/common-belief.kb.bent-bary.ci.svg) | [h](renders/common-belief.kb.annulus.host.svg) [c](renders/common-belief.kb.annulus.ci.svg) | [h](renders/common-belief.kb.circo.host.svg) [c](renders/common-belief.kb.circo.ci.svg) |
| negations of negations | [`06_double_negation.ein`](programs/06_double_negation.ein), `ein render kb` | [host](renders/double-negation.kb.side-by-side.host.png) · [ci](renders/double-negation.kb.side-by-side.ci.png) | [h](renders/double-negation.kb.layered.host.svg) [c](renders/double-negation.kb.layered.ci.svg) | [h](renders/double-negation.kb.twopi.host.svg) [c](renders/double-negation.kb.twopi.ci.svg) | [h](renders/double-negation.kb.twopi-fit.host.svg) [c](renders/double-negation.kb.twopi-fit.ci.svg) | [h](renders/double-negation.kb.bent.host.svg) [c](renders/double-negation.kb.bent.ci.svg) | [h](renders/double-negation.kb.bent-bary.host.svg) [c](renders/double-negation.kb.bent-bary.ci.svg) | [h](renders/double-negation.kb.annulus.host.svg) [c](renders/double-negation.kb.annulus.ci.svg) | [h](renders/double-negation.kb.circo.host.svg) [c](renders/double-negation.kb.circo.ci.svg) |
| zebra2 as written | [`zebra2.ein`](programs/zebra2.ein), `ein render program`, its `kb` | [host](renders/zebra2.program-kb.side-by-side.host.png) · [ci](renders/zebra2.program-kb.side-by-side.ci.png) | [h](renders/zebra2.program-kb.layered.host.svg) [c](renders/zebra2.program-kb.layered.ci.svg) | [h](renders/zebra2.program-kb.twopi.host.svg) [c](renders/zebra2.program-kb.twopi.ci.svg) | [h](renders/zebra2.program-kb.twopi-fit.host.svg) [c](renders/zebra2.program-kb.twopi-fit.ci.svg) | [h](renders/zebra2.program-kb.bent.host.svg) [c](renders/zebra2.program-kb.bent.ci.svg) | [h](renders/zebra2.program-kb.bent-bary.host.svg) [c](renders/zebra2.program-kb.bent-bary.ci.svg) | [h](renders/zebra2.program-kb.annulus.host.svg) [c](renders/zebra2.program-kb.annulus.ci.svg) | |
| a calculator program as written | [`add.ein`](programs/add.ein), `ein render program`, its `kb` | [host](renders/add.program-kb.side-by-side.host.png) · [ci](renders/add.program-kb.side-by-side.ci.png) | [h](renders/add.program-kb.layered.host.svg) [c](renders/add.program-kb.layered.ci.svg) | [h](renders/add.program-kb.twopi.host.svg) [c](renders/add.program-kb.twopi.ci.svg) | [h](renders/add.program-kb.twopi-fit.host.svg) [c](renders/add.program-kb.twopi-fit.ci.svg) | [h](renders/add.program-kb.bent.host.svg) [c](renders/add.program-kb.bent.ci.svg) | [h](renders/add.program-kb.bent-bary.host.svg) [c](renders/add.program-kb.bent-bary.ci.svg) | [h](renders/add.program-kb.annulus.host.svg) [c](renders/add.program-kb.annulus.ci.svg) | |
| **large**: zebra2 saturated, 452 facts | [`zebra2.ein`](programs/zebra2.ein), `ein render kb` | [host](renders/zebra2.kb.side-by-side.host.png) · [ci](renders/zebra2.kb.side-by-side.ci.png) | [h](renders/zebra2.kb.layered.host.svg) [c](renders/zebra2.kb.layered.ci.svg) | [h](renders/zebra2.kb.twopi.host.svg) [c](renders/zebra2.kb.twopi.ci.svg) | [h](renders/zebra2.kb.twopi-fit.host.svg) [c](renders/zebra2.kb.twopi-fit.ci.svg) | [h](renders/zebra2.kb.bent.host.svg) [c](renders/zebra2.kb.bent.ci.svg) | [h](renders/zebra2.kb.bent-bary.host.svg) [c](renders/zebra2.kb.bent-bary.ci.svg) | [h](renders/zebra2.kb.annulus.host.svg) [c](renders/zebra2.kb.annulus.ci.svg) | |
| **large**: the biggest beliefs program, nested facts, 36 facts | [`10_everybody_contradicts.ein`](programs/10_everybody_contradicts.ein) (from `examples/beliefs/`), `ein render kb` | [host](renders/everybody-contradicts.kb.side-by-side.host.png) · [ci](renders/everybody-contradicts.kb.side-by-side.ci.png) | [h](renders/everybody-contradicts.kb.layered.host.svg) [c](renders/everybody-contradicts.kb.layered.ci.svg) | [h](renders/everybody-contradicts.kb.twopi.host.svg) [c](renders/everybody-contradicts.kb.twopi.ci.svg) | [h](renders/everybody-contradicts.kb.twopi-fit.host.svg) [c](renders/everybody-contradicts.kb.twopi-fit.ci.svg) | [h](renders/everybody-contradicts.kb.bent.host.svg) [c](renders/everybody-contradicts.kb.bent.ci.svg) | [h](renders/everybody-contradicts.kb.bent-bary.host.svg) [c](renders/everybody-contradicts.kb.bent-bary.ci.svg) | [h](renders/everybody-contradicts.kb.annulus.host.svg) [c](renders/everybody-contradicts.kb.annulus.ci.svg) | |
| **large**: a calculator saturated, 456 facts | [`first.ein`](programs/first.ein) (from `examples/binary_arithmetic/`), `ein render kb` | [host](renders/binary-first.kb.side-by-side.host.png) · [ci](renders/binary-first.kb.side-by-side.ci.png) | [h](renders/binary-first.kb.layered.host.svg) [c](renders/binary-first.kb.layered.ci.svg) | [h](renders/binary-first.kb.twopi.host.svg) [c](renders/binary-first.kb.twopi.ci.svg) | [h](renders/binary-first.kb.twopi-fit.host.svg) [c](renders/binary-first.kb.twopi-fit.ci.svg) | [h](renders/binary-first.kb.bent.host.svg) [c](renders/binary-first.kb.bent.ci.svg) | [h](renders/binary-first.kb.bent-bary.host.svg) [c](renders/binary-first.kb.bent-bary.ci.svg) | [h](renders/binary-first.kb.annulus.host.svg) [c](renders/binary-first.kb.annulus.ci.svg) | |

Zebra2's knowledge base, the six ways (host):

![zebra2's saturated knowledge base drawn six ways: the layered drawing a thin strip, twopi's nodes piled on each other, the fitted twopi, the bent rings, the refined bent rings and the annulus each a disc](renders/zebra2.kb.side-by-side.host.png)

The beliefs program with nested facts, the six ways (host):

![the beliefs program drawn six ways: the layered drawing, twopi and fitted twopi, and the three bent drawings with their frames stretched into large boxes](renders/everybody-contradicts.kb.side-by-side.host.png)

**Neither build refused anything**: 116 drawings, every exit 0 and no warning ([`runs/draw.host.tsv`](runs/draw.host.tsv),
[`runs/draw.ci.tsv`](runs/draw.ci.tsv), [`runs/refused-or-warned.tsv`](runs/refused-or-warned.tsv), empty). A pinned
position is drawn where it is put by both: the same bent DOT laid out by the two builds puts every node at the same
place, to the point.

## What each way gives and costs

**The size of the drawing** (host, inches): the rings turn a strip into a disc, and are larger.

| case | layered | twopi | twopi-fit | bent | bent-bary | annulus |
|---|---|---|---|---|---|---|
| your five layers | 7×4 | 9×10 | 13×14 | 14×13 | 14×14 | 15×12 |
| nested facts (common belief) | 12×6 | 12×10 | 16×13 | 18×18 | 19×18 | 20×19 |
| zebra2 as written | 53×4 | 10×8 | 35×32 | 20×19 | 20×19 | 32×32 |
| a calculator as written | 13×8 | 7×6 | 9×9 | 22×21 | 22×22 | 22×21 |
| zebra2 saturated | 210×4 | 12×9 | 90×89 | 52×51 | 51×51 | 81×81 |
| everybody contradicts | 15×7 | 11×12 | 17×20 | 22×21 | 22×22 | 22×21 |
| a calculator saturated | 382×8 | 14×9 | 236×231 | 145×143 | 146×143 | 332×318 |

- **layered**, today: every node's place is the layer you asked for, and `dot` reduces the crossings. A knowledge base
  with many objects is a strip, as wide as its widest layer: zebra2's is fifty times wider than tall, so a picture of
  it fitted to a screen is a line. Below a few dozen facts it reads best of all the ways here.
- **twopi**: one Graphviz pass and no positions to compute, so the binary would write only a hidden centre, `root` and
  `layout=twopi`. At Graphviz's default spacing it piles a ring's nodes on each other (zebra2's twopi is 12 × 9 in for
  532 nodes); it does not widen a ring for its nodes. With each ring's radius computed (twopi-fit) nothing overlaps,
  but the drawing is the largest of all, since twopi's distance rings put many nodes on one ring. It draws **no frame**:
  twopi ignores clusters, so a fact in a context only is no longer marked. On the calculator as written it is the
  clearest drawing here (`n` in the middle, the digits round it). **Its rings are not yours** ([`runs/*.rings.host.txt`](runs/common-belief.kb.rings.host.txt)
  lists every node it moves): your five layers, 0 of 22 nodes moved; common belief, 13 of 41; double negation, 27 of
  41; zebra2 as written, 29 of 131; zebra2 saturated, 99 of 532; the calculator saturated, 54 of 669. Where it misses:
  - **a nested fact**: `(likes Dan Fish)`, two arrows deep in common belief, is as near its relation `likes` as any
    fact of `likes`, so it joins the top-level facts' ring and the band's depth is lost;
  - **a fact whose arguments are relations**: zebra2's `(co-located nation-loc Englishman color-loc Red)` stands with
    the declarations, one step from the centre, and pulls its objects `Englishman` and `Red` onto the relations' ring;
  - **objects split over rings**: an object is as far out as its nearest fact or declaration, so in zebra2 as written
    29 objects, those the declarations name as labels and those a fact like `co-located`'s pulls in, stand on the
    relations' ring, and the other 9, `Zebra` and `Water` among them, on the outermost; in common belief every object
    comes two rings in, the band's depth lost;
  - **a relation nothing declares**: `not` in common belief is reached only through the negations, three rings out;
  - **a type that is also an object**: zebra2's `House`, a label of declarations and an argument of `(is-a House-1
    House)`. twopi puts it with the labels, your ring 3; the layers put it below the `is-a` facts, with the objects.
- **bent**: your rings exactly, by construction: ring k is layer k. Two passes, `dot` for the layers and their order,
  then the drawing with every position given, which one DOT can carry for a plain `dot -Tsvg` (`layout=neato`,
  `inputscale=72`, every `pos` pinned; checked to the point against `neato -n2` in both builds). What it costs:
  - **arrows across the centre**: a relation is one point on an inner ring and its facts go round the whole outer
    ring, so its first arrows fan across the disc; the layered drawing fans them locally;
  - **frames**: Graphviz draws a frame from given positions only from a given box, and a frame's members, neighbours
    in `dot`'s order, still spread along their ring, so its box covers much of the disc (the beliefs programs, the
    calculator as written);
  - **the outer ring is as large as the facts' ring makes it**, and with few objects sparse;
  - small drawings get larger and read worse than layered.
- **bent-bary**: the same rings, the order refined on the circle; in zebra2 as written fewer arrows cross the
  centre, and the frames' boxes are no smaller.
- **annulus**: `dot`'s drawing rolled up, every crossing it left kept, but every ring at least as long as the strip was
  wide: the centre is empty and `relation` is not in it. Of the bent ways the largest.
- **circo**: circles of biconnected components, no layer in them; shown on the four small programs, and not further.

## How it would be asked for

A ring layout is a choice of positions, and the DOT's structure, its nodes and arrows, stays what it is: the tests
hold structure, not layout (Q21), so none would change. The drawings it applies to are the views of a knowledge base
(`ein render kb`, the program view's `kb`, `ein render constraints`, a trace's `-F` snapshots); a rule's panels and a
slice are layered by their arrows alone and would stay so.

- **(b) `ein render kb FILE --layout radial`**: the binary writes the hidden centre, `layout=twopi`, `root`, and the
  rings' radii as `ranksep`'s list, which it can compute from the graph; Graphviz does the rest in one pass. Rings by
  distance, not yours; no frames. The radii need each node's size, which only Graphviz knows exactly (the binary
  would estimate a box from its label's length).
- **(c) `ein render kb FILE --layout rings`**: the binary writes every position: its layers (it writes them today), an
  order on each ring of its own (a barycentre on the circle, as bent-bary refines `dot`'s; the binary has no `dot`
  order to start from), the radii from estimated sizes, the frames' boxes; `layout=neato`, `inputscale=72` and pinned
  positions, drawn by `dot -Tsvg` as today. Your rings exactly; the binary computes a layout, which today it leaves to
  Graphviz entirely (Q21).
- **(d) A recipe beside the binary**: `ein render kb FILE | dot -Tjson0 | <bend> | neato -n2 -Tsvg`, the bending a
  script the reference documents, the binary unchanged. Exactly your rings, `dot`'s order, two Graphviz passes.

Each would be a row of [`06-config.md`](../../../../../docs/ein/user/04-reference/06-config.md) (it changes no answer) and a
paragraph of the rendering reference's *Layers*.

## The options

- **(a) No ring layout** (default): the layers stay the only layout of a knowledge base's views.
- **(b) `--layout radial`**: twopi from a hidden centre, the radii fitted.
- **(c) `--layout rings`**: your rings exactly, the binary computing the positions.
- **(d) A documented recipe**, the binary unchanged.

## The default

(a): your question was whether it is possible, and it is, at the costs above; on these programs no way is clearly
easier to read than the layers but by its shape, so the choice is yours.

## Files

- `programs/`: item 56's six (`five-layers.ein`, `five-layers-rule.ein` written for it; `01_common_belief.ein`,
  `06_double_negation.ein` from `examples/beliefs/`; `zebra2.ein` from `examples/`; `add.ein` from `examples/arith/`),
  and `10_everybody_contradicts.ein` (from `examples/beliefs/`) and `first.ein` (from `examples/binary_arithmetic/`).
- `tools/concentric.py`, which draws every way: the binary's DOT, `dot -Tjson0` for its positions, each way's DOT
  rebuilt from them; `tools/sheets.py`, the side-by-side pictures.
- `renders/NAME.dot`, the binary's DOT; `renders/NAME.WAY.TAG.dot` and `.svg`, each way as drawn, `TAG` `host` or
  `ci`; `renders/NAME.side-by-side.TAG.png`.
- `runs/NAME.rings.TAG.txt`, the layers as rings and every node twopi puts elsewhere; `runs/draw.TAG.tsv`, every
  Graphviz call with its time and exit; `runs/graphviz.TAG.txt`, the versions.

## Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/61-concentric/rerun.sh`
(`BIN`, `IMAGE` and `SPLINES` override the binary, the CI image and the radial ways' arrows; the binary is found in
the main checkout's `build/review-bin/`).

## Your call

- (a) no ring layout.
- (b) `--layout radial`.
- (c) `--layout rings`.
- (d) a documented recipe.
