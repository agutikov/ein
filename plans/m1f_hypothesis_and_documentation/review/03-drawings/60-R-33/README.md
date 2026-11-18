# 60 · R-33 — the slices Graphviz refused

[← the topic](../README.md) · [← the bundle](../../README.md)

## The question

A trace's drawings, the slice of each step that `ein solve FILE --trace OUT.md` writes as an inline `dot` block, were
refused by Graphviz for the calculator's programs (*triangulation failed*, *Pshortestpath failed*), canonical and
folded, and no test put a trace's drawing to Graphviz. The phase filed it as R-33 and left the slices as they were,
since what routes them changes their layout. S1f.23.6 laid every slice out in layers, top to bottom (item 56), and
folds no fact a step's arrows touch (item 59); then it put every slice of every corpus trace to Graphviz again, with
the binary at the stage's start and with its own, and decided whether a check of a trace's drawings belongs to the
per-commit `dot_wellformed`. Recorded in [P1f.23's findings](../../../p1f.23_review_processing/findings.md), R-33, and
[S1f.23.6's Record](../../../p1f.23_review_processing/s1f.23.6_the_drawings_answers.md#record), § 5 and reading 11.

## What is so today

**The probe** ([`tools/probe_r33.py`](../tools/probe_r33.py)): every program of the corpus (`corpus/corpus.toml`,
888), `ein solve FILE --trace OUT.md` and with `--fold`, 600 s each; then every distinct `dot` block put to
`dot -Tsvg` (Graphviz 16.0.0) once, refused where `dot` exits non-zero or writes to stderr, and counted apart where it
lays nothing out within 120 s. The counts are [`runs/probe-summary.md`](runs/probe-summary.md), the lines
[`runs/probe-before.tsv`](runs/probe-before.tsv) and [`runs/probe-after.tsv`](runs/probe-after.tsv) (a program, its
form, its blocks, those refused, for `--fold` the blocks that differ from the canonical trace's, those not laid out in
time), and every refused block before is a line of [`runs/probe-refused.tsv`](runs/probe-refused.tsv).

| | before (`ein-head-6de9101b`) | after (`ein-s1f236`) |
|---|---|---|
| programs; traces; traces with a drawing | 888; 644; 510 | the same |
| blocks, in each form | 163 672 | 163 672 |
| canonical blocks refused | **192**, in 18 traces | **none** |
| folded blocks refused | **558**, in 34 traces | **none** |
| programs with a refusal | 37, every one the calculator's | none |
| blocks `dot` did not lay out within 120 s | 289, every one `long_numbers.ein`'s (50 canonical, 239 folded) | none |
| folded blocks that differ from the canonical trace's | 68 814 | none |
| distinct blocks laid out | 217 225 | 154 062 |

- **After, nothing refused**, in either form, and no block past 120 s; every trace's folded blocks are its
  canonical ones (a step's slice folds nothing a step touches, item 59). No program was left out: every trace
  finished within 600 s, `examples/arith/long_numbers.ein`'s among them (2 861 blocks). The 244 programs with no
  trace are refused at load, the same for both binaries (`examples/ein-bugs/` and `examples/broken/`, and two each of `examples/lattice/` and `examples/branching/`).
  The binary that drew these is the stage's first build; this stage's final binary writes every one of the same
  blocks byte for byte (checked over the corpus), since the folded arrows it changed are in the views of a knowledge
  base and not in a slice.
- **Before**, 192 canonical blocks refused in 18 traces and 558 folded in 34, of 37 programs, every one the
  calculator's (`examples/arith/`, `beyond_arith/`, `binary_arithmetic/`, `word_problems/pencils.ein`,
  `tests/stdlib/math/`): Graphviz says *triangulation failed* (and *Pshortestpath failed* after it) of all but nine
  folded blocks, and *lost edge* of those nine ([`runs/probe-summary.md`](runs/probe-summary.md)). The phase's probe
  counted 169 and 487 in 36 programs, `long_numbers.ein` left out since its trace did not finish within that probe's
  120 s; with it left out here the counts are the same, 169 in 17 traces and 487 in 33. Here its trace finished:
  94 of its blocks were refused (23 canonical, 71 folded) and 289 not laid out within 120 s.
- **What fixed it** is the layout: top to bottom with the first arrow drawn back, where the slices were laid out left
  to right under `newrank`; and no point for a folded fact under `--fold`. The phase's sample had found the same:
  `rankdir=TB` routed every refused block it tried. Nothing was left to fix.
- **The refused blocks**, before and after, in `renders/`: the smallest refused block of each Graphviz message, and
  the first refused block of each witness below, each with Graphviz's stderr (`NAME.before.dot`,
  `NAME.before.dot.err`) and the same step's block from this stage's binary, drawn (`NAME.after.dot|svg`).

**The per-commit check**: `ein-render/tests/dot_wellformed.rs`' `graphviz_accepts_every_slice_of_the_witnesses_traces`
puts every slice of two traces to `dot`, a unit of the pool each: `examples/arith/div.ein` (the smallest program a
canonical slice of which was refused) and `examples/binary_arithmetic/first.ein` (the smallest a folded one of which
was), 611 slices, 1.8 s on this host with the default sixteen workers. Every slice of the second's folded trace is its canonical one, so the canonical traces stand
for both forms.

## The options

- **(a) As built** (default): the slices laid out in layers, and the bounded per-commit check of the two witnesses in
  `dot_wellformed`.
- **(b) No per-commit check**: the corpus probe kept as a script to run by hand.
- **(c) Every trace's slices per commit or nightly**: some three hundred thousand `dot` runs, the probe's.

## The default

(a): a refusal by Graphviz is a fault of the DOT, as `dot_wellformed` holds for every view (well-formedness, not
layout, which your rule leaves to a reader), and R-33 reached no one because no test put a trace's drawing to
Graphviz; the whole corpus is past any per-commit or nightly budget, and the two programs whose slices were refused
first keep the check within one.

## Files

- `runs/probe-before.tsv`, `runs/probe-after.tsv`, `runs/probe-summary.md`, `runs/probe-refused.tsv`: above.
- `renders/KIND.PROGRAM.FORM.blockN.before.dot` and `.before.dot.err`, a refused block and Graphviz's stderr;
  `renders/KIND.PROGRAM.FORM.blockN.after.dot|svg`, the same step today; `KIND` the message, or `witness`.

## Re-run

From the repo root: `R33=1 bash plans/m1f_hypothesis_and_documentation/review/03-drawings/tools/build.sh` runs the
probe for both binaries into `build/r33/` (a long run: it lays out some three hundred and seventy thousand distinct
blocks) and writes this item's files; without `R33=1` it writes them from the probe's work already there.

## Your call

- (a) as built.
- (b) no per-commit check.
- (c) every trace's slices checked.
