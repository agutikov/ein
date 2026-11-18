# 62 · A derivation's three levels, built

[← the topic](../README.md) · [← the bundle](../../README.md)

## The question

On item 59 you answered: *"Yes, but don't use black color for highlighting. use both color and thickness, derived
fact also colored. same color as derivation but more bright. The added fact is the brightest, then derivation
explanation, and then all the rest with default black and default thickness."* (Q-M1f.492, .493,
[open questions](../../../open_questions.md#q-m1f492--the-derivations-palette)). S1f.23.11 built it. This item shows
it against the drawing you answered on, and the readings the stage took where the answer is silent.

## What is so today

*Before* is `build/review-bin/ein-head-d124e63b`, the engine you answered on; *after* is `build/review-bin/ein-s1f2311`.
Every `dot` block of `ein solve FILE --trace OUT.md` is a file under `renders/`, drawn by this host's Graphviz
(`.host.svg`) and by the CI image's (`.ci.svg`); none of the 140 drawings was refused by either
([`runs/draw.tsv`](runs/draw.tsv)).

- **Three levels**, each step in a hue picked by its rule's name:

  | level | what | drawn |
  |---|---|---|
  | 1, the brightest | the fact a step added: its octagon and its arrows | the hue's bright shade, `penwidth=3` |
  | 2 | the derivation's explanation: the step's border and label, its arrows from each premise and to what it added | the same hue, deeper, `penwidth=2` |
  | 3 | everything else: the premises, the relations, the names | Graphviz's default black, default width |

- **The palette**: six hues, each in two shades (explanation · added fact): blue `#0e61cc` · `#418cfa`, teal
  `#11706e` · `#1c9e9a`, green `#117622` · `#1ca433`, orange `#98530f` · `#d37619`, purple `#951be7` · `#b46afa`,
  magenta `#af2b94` · `#f240cd`. Against black every shade is at a contrast of 3.5 or more (before: indigo
  `#1a237e` 1.6 and brown `#5c3317` 1.9, gone; purple `#6a0dad` 2.3, lightened); against white the deeper shade,
  in which a step's label is written, at 5.8 to 5.9, the bright one, a line only, at 3.3. A test holds those bounds
  (`no_shade_of_the_derivation_palette_is_near_black`) and another the three levels
  (`a_derivation_is_drawn_at_three_levels`).
- **Every drawing of a derivation**: a trace's slices, the derivation graph (the library's, in the first hue, since it
  records no rule) and a written trace's `dag` view (whose derived facts took their relation's colour before; a fact
  no step derives there is black now).

| program | before | after |
|---|---|---|
| a step of four premises, [`12_typed_blind_solve.ein`](programs/12_typed_blind_solve.ein) (from `examples/branching/`), state 2's step 6 | [svg](renders/12_typed_blind_solve.before.block13.host.svg) | [svg](renders/12_typed_blind_solve.after.block13.host.svg) |
| the same rule, firing redundantly, root's step 7: dashed, it adds nothing | [svg](renders/12_typed_blind_solve.before.block7.host.svg): its conclusion bold black | [svg](renders/12_typed_blind_solve.after.block7.host.svg): its conclusion plain |
| packing's first step, a nested fact, [`02_packing.ein`](programs/02_packing.ein) (from `examples/beliefs/`) | [svg](renders/02_packing.before.block1.host.svg) | [svg](renders/02_packing.after.block1.host.svg) |
| the reasoning pages' two steps, [`fox.ein`](programs/fox.ein), two premises each, the second concluding `(false)` | [1](renders/fox.before.block1.host.svg), [2](renders/fox.before.block2.host.svg) | [1](renders/fox.after.block1.host.svg), [2](renders/fox.after.block2.host.svg) |
| the derivation graph (the golden `kb_provenance_dag.dot`) | [svg](renders/golden-kb_provenance_dag.before.host.svg) | [svg](renders/golden-kb_provenance_dag.after.host.svg) |
| a written trace's `dag` view (the golden `ir_render_trace_dag.dot`) | [svg](renders/golden-ir_render_trace_dag.before.host.svg) | [svg](renders/golden-ir_render_trace_dag.after.host.svg) |
| a slice of three steps, a commitment's red and a negation's grey beside them (the golden `slice_render_slice.dot`) | [svg](renders/golden-slice_render_slice.before.host.svg) | [svg](renders/golden-slice_render_slice.after.host.svg) |

The traces' text is unchanged; only the blocks' colours and widths moved
([`runs/fox.before-vs-after.diff`](runs/fox.before-vs-after.diff) and the other two). The pages that show a
derivation were drawn again: the four slices of the syntax-and-semantics pages re-taken from the binary, the reasoning
pages' four restyled by the same rule; the rendering reference's trace section has the three levels as a table, and
Q-M1f.428's decision says them.

## The readings the stage took

Where your answer is silent, the stage took each of these and recorded it (S1f.23.11's Record):

1. **A premise's own node is the rest**, black: the explanation is the step and its arrows. A premise another step of
   the same drawing added keeps that step's bright shade.
2. **A commitment's red and a negation's grey are kept** beside the three levels: they say what the state assumed and
   what is denied, not how a fact was derived. A derived negation takes its step's bright shade, as any added fact.
3. **The widths**: the added fact `penwidth=3`, the explanation `penwidth=2`, so the order is told twice, by
   brightness and by width.
4. **A redundant firing adds nothing**: what it derives was there before it, so it is drawn as the rest, and its
   step, dashed, and its arrows as the explanation. Before, it was bold.
5. **The refutation's arrows are the core's red**: the core's to `⊥` were, and the learned no-good's, black at width 2
   before, are now red too, so nothing emphasised is black.
6. **In the derivation graph a fact is added** where a derivation concludes it and its first record is a rule's: a
   given fact that a derivation also concludes was there before it.

## The options

- **(a) As built** (default): the three levels and the six readings above.
- **(b) One width**: the added fact at `penwidth=2`, as the explanation, the two told apart by brightness alone.
- **(c) The premises as explanation**: each premise's octagon and arrows in the step's deeper shade, so that a step's
  slice is all coloured but its relations and names.
- **(d) A redundant firing's conclusion bright** as any derived fact (before's marking, in colour).

## The default

(a): it is your answer, with each silent point read so that the brightest thing in a drawing is the fact a step
added and nothing else.

## Files

- `programs/fox.ein` (the reasoning pages' derivation), `programs/02_packing.ein` (from `examples/beliefs/`),
  `programs/12_typed_blind_solve.ein` (from `examples/branching/`).
- `runs/NAME.trace.before.md`, `runs/NAME.trace.after.md`, the traces; `runs/NAME.solve.*.txt`, the read-outs;
  `runs/NAME.before-vs-after.diff`; `runs/draw.tsv`, every drawing and Graphviz's exit; `runs/refused.tsv`, none.
- `renders/NAME.SIDE.blockN.dot` and `.host.svg`, `.ci.svg`; `renders/golden-NAME.SIDE.dot` and their drawings.

## Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/62-derivation-colours/rerun.sh`
(`BEFORE`, `AFTER` and `IMAGE` override the binaries and the CI image; the binaries are found in the main checkout's
`build/review-bin/`).

## Your call

- (a) as built.
- (b) one width.
- (c) the premises as explanation.
- (d) a redundant firing's conclusion bright.
