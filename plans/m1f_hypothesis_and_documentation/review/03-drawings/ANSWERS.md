# Your answers — Drawings

[← the topic](README.md) · [← the bundle](../README.md)

One entry an item, in the topic's order: its options as the item's page puts them, the default that stands
until you answer, and a blank. Write your answer after **Your answer:** — a letter, *default*, or your own
words — and anything else under **Notes**. An item left blank keeps its default. This file is yours: a
re-render of the bundle rewrites the READMEs and never this file.

## 61 · A concentric layout: the layers drawn as rings

[The item](61-concentric/README.md)

**Options:**

- **(a) No ring layout** (default): the layers stay the only layout of a knowledge base's views; nothing is built.
- **(b) `--layout radial`**: the binary writes a hidden centre, `layout=twopi`, `root` and fitted ring radii;
  Graphviz does the rest. Rings by graph distance, not exactly yours; no frames.
- **(c) `--layout rings`**: the binary writes every position, its layers as rings, an order of its own on each ring,
  drawn by `dot -Tsvg` as today. Your rings exactly; the binary computes a layout.
- **(d) A documented recipe**: `ein render kb FILE | dot -Tjson0 | <bend> | neato -n2 -Tsvg`, the binary unchanged.

**Your answer:** 

**Notes:** 

---

## 62 · A derivation's three levels, built

[The item](62-derivation-colours/README.md)

**Options:**

- **(a) As built** (default): the fact a step added the brightest, in its rule's hue at `penwidth=3`; the step and
  its arrows the same hue, deeper, at `penwidth=2`; everything else default black; six hues of two shades, none near
  black; and the six readings on the item's page (a premise black, red and grey kept, a redundant firing adds
  nothing, the no-good's arrows red, a fact added in the derivation graph where its first record is a rule's).
- **(b) One width**: the added fact at `penwidth=2`, as the explanation, told apart by brightness alone.
- **(c) The premises as explanation**: each premise's octagon and arrows in the step's deeper shade.
- **(d) A redundant firing's conclusion bright**, as any derived fact.

**Your answer:** 

**Notes:** 
