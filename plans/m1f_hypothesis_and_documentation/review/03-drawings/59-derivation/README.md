# 59 · A derivation's drawing: contrast, width, layers, and no arrow to or from an arrow

[← the topic](../README.md) · [← the bundle](../../README.md)

## The question

You ruled how a derivation is drawn (Q-M1f.428): *"(a) I like what we already have, just use more contrast colors
and 2x thicker lines then default"*; and a folded fact in it (Q-M1f.429): *"do not fold, when we show derivation with
additional arrow (of different color); general rule for rendering - no arrow to/from arrows, arrows go only to/from
shapes; So other 3-facts and 2-facts can be folded, but not those that hold derivation arrows."* S1f.23.6 built both,
and laid a slice out in layers as a knowledge base is. Where the rulings are silent it took readings (S1f.23.6's
Record, readings 6 to 8), shown here: **which colours** and **which lines**, and that **a step's slice under `--fold`
is the canonical one**, since every fact in it is a premise or a conclusion of the step.

## What is so today

*Before* is the binary at the stage's start, *after* this stage's; every `dot` block of `ein solve FILE --trace OUT.md`,
canonical and `--fold`, is a file under `renders/`.

- **The colours**: a step's node and its arrows take one colour of eight, picked by the rule's name as a relation's
  colour is, from a palette of dark colours each at a contrast of 6 or more against white (the palette before:
  2.0 to 5.9), none red (a commitment's facts, the core, `⊥`) and none grey (a stored negation):
  `#0047ab` `#006400` `#6a0dad` `#a14a00` `#00686b` `#8b008b` `#5c3317` `#1a237e`.
- **The lines**: every arrow of the derivation, premise to step, step to conclusion, core to `⊥`, the no-good's, and a
  step's border, at `penwidth=2`; a fact's own arrows keep the default width, and a fact a step derived stays bold,
  its octagon and its arrows at `penwidth=2`, told from the derivation's arrows by their colour.
- **The layers**: a slice is laid out top to bottom, each premise above the step, the step above what it derives, each
  relation above its facts; before, left to right.

  Packing's first step ([`02_packing.ein`](programs/02_packing.ein)): [after](renders/packing.trace.after.block1.svg),
  [before](renders/packing.trace.before.block1.svg); the two refutation steps of the reasoning pages' example
  ([`fox.ein`](programs/fox.ein)): [after](renders/fox.trace.after.block2.svg),
  [before](renders/fox.trace.before.block2.svg). The DOT's changes: [`runs/packing.before-vs-after.diff`](runs/packing.before-vs-after.diff).
- **`--fold` in a slice**: before, a fact a step read or derived was folded into an arrow through a point that the
  step's arrows met: four points in packing's three steps and five in fox's two
  ([before, folded](renders/fox.trace.before-fold.block2.svg)). After, no fact a derivation's arrow touches is folded,
  and in a step's slice that is every fact, so the folded trace's blocks are the canonical ones byte for byte
  ([`runs/packing.after.canonical-vs-fold.diff`](runs/packing.after.canonical-vs-fold.diff) and fox's are empty).
  `--fold` folds a trace's `-F` snapshots, which are KB views. A fact only a slice's commitment holds and no step
  reads would fold, but the trace draws no such slice today.
- The derivation graph and a written trace's `dag` view draw their steps the same way (the goldens
  `ein-render/tests/golden/dot/kb_provenance_dag.dot`, `ir_render_trace_dag.dot`), the derivation graph in the first
  colour since it records no rule. The reasoning pages' derivations (`02-derivation`, `09-core-in-derivation`,
  `09-cyclic`, `09-proof-graph`) were drawn again so.

## The options

- **(a) As built** (default).
- **(b) One colour for every derivation**, the rule's name in the step's node only.
- **(c) Width on the derivation's arrows only**, a derived fact's octagon and arrows back to the default width, so
  that width means *the derivation* alone.

## The default

(a): the colour by rule tells two steps of one slice apart, as before, only darker; a derived fact bold is what the
slice drew before, and the user asked for more contrast, not for less marking.

## Files

- `programs/02_packing.ein` (from `examples/beliefs/`), `programs/fox.ein` (the reasoning pages' derivation, from
  `docs/ein/ein.rs/08-reasoning-in-code/02-saturation-and-negation.md`).
- `runs/NAME.trace.KIND.md`, the trace, `KIND` `before`, `before-fold`, `after`, `after-fold`; `runs/NAME.solve.KIND.txt`,
  the read-out; `renders/NAME.trace.KIND.blockN.dot|svg`, each block; `runs/*.diff`.

## Re-run

`bash plans/m1f_hypothesis_and_documentation/review/03-drawings/tools/build.sh` from the repo root.

## Your call

- (a) as built.
- (b) one colour for every derivation.
- (c) width on the derivation's arrows only.
