# L-100 — the beliefs' two KB drawings are no generator's and no test's

## 1. The question

`examples/beliefs/` keeps, beside two of its programs, the knowledge base `ein render kb` draws for each, as a DOT
and its picture: `01_common_belief.kb.dot` / `.kb.svg` and `02_packing.kb.dot` / `.kb.svg`. They are drawn by a
command line written in `examples/README.md`, by hand, each time the renderer changes. Nothing compares them with
the binary: not a test, not a generator's `--check`. During the nested-facts work the renderer moved twice and the
drawings were found behind it once; since the first look the renderer moved a third time (the canonical form,
Q-M1f.419) and the drawings were drawn again by hand, because the stage's own list named them. Should they be held to
`ein render kb` by a test, be put on `AGENTS.md`'s list of generated files with a generator, or stay as they are?

Recorded: [`../../findings.md`](../../../findings.md#examples), row **L-100** (P1f.22's N-33).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; Graphviz 16.0.0.

**They match the binary today.** `ein render kb examples/beliefs/NN.ein` against the checked-in `NN.kb.dot`:

| program | diff | file |
|---|---|---|
| `01_common_belief` | no difference (exit 0) | [`runs/01_common_belief.kb.tracked-vs-today.diff`](runs/01_common_belief.kb.tracked-vs-today.diff) |
| `02_packing` | no difference (exit 0) | [`runs/02_packing.kb.tracked-vs-today.diff`](runs/02_packing.kb.tracked-vs-today.diff) |

and the checked-in `.kb.svg` of each is byte for byte `dot -Tsvg` of today's output on this machine's Graphviz
([`runs/svg-is-dot-of-today.txt`](runs/svg-is-dot-of-today.txt)). The two programs here are the corpus files
([`runs/programs-are-the-corpus.txt`](runs/programs-are-the-corpus.txt)).

**They are drawn in the canonical form now**: every fact an octagon with no name in it, its arrow `1` at its
relation's box and `2`, `3`, … at its arguments; `01_common_belief` 24 octagons and 66 arrows, `02_packing` 19 and
53. The held facts a rule derived, `(likes Dan Soup)` and `(not (likes Dan Fish))` in the first, are dashed, with
`by common-belief-likes` and `by common-belief-not-likes` beside them; the facts believed and not held stand in the
frames `believes ctx` and `not ctx`. The checked-in pictures are `dot`'s, a wide strip, where the view's own DOT
says `// suggested layout: fdp` (R-18, item [55](../55-R-18/README.md)):

![01_common_belief as checked in: dot's layout of ein render kb, a strip two ranks high, the facts' octagons in a row under the boxes of the atoms, numbers crowded where arrows meet](renders/01_common_belief.kb.tracked.svg)

![01_common_belief, today's output laid out by fdp, the layout the DOT suggests: the facts of believes around the box believes, the declarations around relation, the frames believes ctx and not ctx](renders/01_common_belief.kb.today.fdp-layout.svg)

- [`renders/01_common_belief.kb.tracked.svg`](renders/01_common_belief.kb.tracked.svg) ([dot](renders/01_common_belief.kb.tracked.dot)),
  [`renders/02_packing.kb.tracked.svg`](renders/02_packing.kb.tracked.svg) ([dot](renders/02_packing.kb.tracked.dot)):
  the checked-in drawings, copied (1 135 × 243 pt, 1 075 × 201 pt).
- [`renders/01_common_belief.kb.today.fdp-layout.svg`](renders/01_common_belief.kb.today.fdp-layout.svg),
  [`renders/02_packing.kb.today.fdp-layout.svg`](renders/02_packing.kb.today.fdp-layout.svg): today's output by
  `fdp` (855 × 757 pt, 685 × 789 pt); [`…dot-layout.svg`](renders/01_common_belief.kb.today.dot-layout.svg) by `dot`,
  the checked-in picture again.
- [`renders/01_common_belief.kb.today.fold.svg`](renders/01_common_belief.kb.today.fold.svg),
  [`renders/02_packing.kb.today.fold.svg`](renders/02_packing.kb.today.fold.svg): `ein render kb --fold`, which the
  regeneration line does not ask (14 and 13 octagons left: the facts something points at, the declarations, the
  facts of two arrows).

**They were drawn again by hand since the first look.** Their history
([`runs/drawings-history.txt`](runs/drawings-history.txt)) is a commit of each program's, then three commits of the
renderer that each named them: `bf9693dc` (after being found behind), `58b91dc3` (the frame) and `8edf546b`
(S1f.23.1, the canonical form). What the last one changed:
[`runs/01_common_belief.kb.tracked-b5fb8d70-vs-today.diff`](runs/01_common_belief.kb.tracked-b5fb8d70-vs-today.diff),
[`runs/02_packing.kb.tracked-b5fb8d70-vs-today.diff`](runs/02_packing.kb.tracked-b5fb8d70-vs-today.diff) — at the
first look a fact of four arrows or more was a labelled octagon, `(relation)` for `(relation likes Person Food)`, a
fact of three arrows a labelled arrow, `"person" -> "Person" [label="relation"]`, and a fact of two a loop,
`"likes" -> "likes" [label="relation"]`; the first look's pictures:
[`renders/01_common_belief.kb.tracked-at-b5fb8d70.svg`](renders/01_common_belief.kb.tracked-at-b5fb8d70.svg),
[`renders/02_packing.kb.tracked-at-b5fb8d70.svg`](renders/02_packing.kb.tracked-at-b5fb8d70.svg).

**The regeneration line**, `examples/README.md`, the `beliefs/` row (quoted):

> Beside each of the two, its saturated KB drawn by `ein render kb` (`NN_….kb.dot`, `.kb.svg`; regenerate with
> `ein render kb examples/beliefs/NN_….ein > NN_….kb.dot && dot -Tsvg NN_….kb.dot -o NN_….kb.svg`).

and `corpus/corpus.toml`'s note on `02_packing` ends *"Its KB drawn beside it, `02_packing.kb.dot` and `.svg`
(`ein render kb`)."*

**They were behind once.** At `0a4a97a3`, that commit's binary against that commit's checked-in DOTs
(`build/review-bin/ein-0a4a97a3`, `EIN_STDLIB` its tree's): both differ —
[`runs/01_common_belief.kb.at-0a4a97a3.tracked-vs-that-binary.diff`](runs/01_common_belief.kb.at-0a4a97a3.tracked-vs-that-binary.diff),
[`runs/02_packing.kb.at-0a4a97a3.tracked-vs-that-binary.diff`](runs/02_packing.kb.at-0a4a97a3.tracked-vs-that-binary.diff).
The checked-in drawing still had nodes labelled with a fact's text, `"(likes Dan Soup)"`, where the binary already
drew an octagon. The pictures then:
[`renders/01_common_belief.kb.tracked-at-0a4a97a3.svg`](renders/01_common_belief.kb.tracked-at-0a4a97a3.svg),
[`renders/02_packing.kb.tracked-at-0a4a97a3.svg`](renders/02_packing.kb.tracked-at-0a4a97a3.svg).

**The shape golden still does not hold them** ([`runs/shape-golden-vs-render-kb.txt`](runs/shape-golden-vs-render-kb.txt)):

| line of `corpus_shapes.md5` | golden | `ein render kb` today |
|---|---|---|
| `01_common_belief.ein::dot[kb]` | `a910f6551b0eadb3`, 107 lines | `d131ae491815f1c4…`, 107 lines |
| `01_common_belief.ein::dot[kb-fold]` | `a929c0d8b76af710`, 77 lines | `f080da002c649046…`, 77 lines |
| `02_packing.ein::dot[kb]` | `202ee107868fa126`, 75 lines | `f3607fe41bc6f157…`, 86 lines |
| `02_packing.ein::dot[kb-fold]` | `5a3d383423d5b0a3`, 60 lines | `bcfe1b564583201a…`, 68 lines |

The golden's plain KB views draw the KB as loaded, before saturation (read from `ein-render/src/shape.rs`: only
`kb-since` saturates), while `ein render kb` and these drawings are of the saturated KB; for `01_common_belief` the
line counts happen to agree and the digests do not.

## 3. The options

All three are read, not run: nothing here changes code.

### A test that holds them to `ein render kb`

A crate test (in `ein-cli`, beside the other CLI tests) runs `ein render kb` on each of the two programs and diffs
the output against the checked-in `.kb.dot`, with `EIN_BLESS=1` rewriting it, as the goldens are. It reads only
project files (`examples/beliefs/*.ein`, `*.kb.dot`), so it is a project test, and `cargo test --workspace` fails the
commit that moves the renderer and forgets them. The `.svg` cannot be held this way: a picture is never compared
with a second drawing, its bytes being Graphviz's; it stays drawn by hand from the held `.dot`, or gets a stamp of its
source's hash as the `diagrams/` pictures do (`utils/render_diagrams.py`), which the pictures under `examples/` do
not have today.

### The generator list of `AGENTS.md`

A script under `utils/` writes both files (`ein render kb`, then Graphviz) and has a `--check` that
`utils/check_generated.py` runs, a gate step; `AGENTS.md` § Edits that go together lists the four files as
*generated, never edited by hand*, and `utils/README.md` names the script. That is the process holding a project
file, the way the zebra2 variants and the `04-stdlib/` rule drawings are held. One cost: *"a generated project file
says it is generated"*, so the `.kb.dot` would carry a line saying so, and would no longer be byte for byte
`ein render kb`'s output. Either of the first two could draw the picture with the layout the DOT suggests, `fdp`.

### As they are

The line in `examples/README.md` regenerates them; nothing catches drift, as `0a4a97a3` shows, and a stage that
moves the renderer has to know to name them, as S1f.23.1 did.

## 4. The default

The row names no default: **the user's call**, *"a test that holds them to `ein render kb`, the generator list of
`AGENTS.md`, or as they are"*. Until ruled, they stay as they are.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing below, from the repo root |
| [`programs/01_common_belief.ein`](programs/01_common_belief.ein), [`programs/02_packing.ein`](programs/02_packing.ein) | copies of `examples/beliefs/` |
| [`renders/01_common_belief.kb.tracked.dot`](renders/01_common_belief.kb.tracked.dot) / [`.svg`](renders/01_common_belief.kb.tracked.svg) | the checked-in drawing, copied |
| [`renders/02_packing.kb.tracked.dot`](renders/02_packing.kb.tracked.dot) / [`.svg`](renders/02_packing.kb.tracked.svg) | the same for `02_packing` |
| [`renders/01_common_belief.kb.today.dot`](renders/01_common_belief.kb.today.dot), [`renders/02_packing.kb.today.dot`](renders/02_packing.kb.today.dot) | `ein render kb` today |
| [`renders/01_common_belief.kb.today.fdp-layout.svg`](renders/01_common_belief.kb.today.fdp-layout.svg), [`renders/02_packing.kb.today.fdp-layout.svg`](renders/02_packing.kb.today.fdp-layout.svg) | today's output drawn with `fdp`, the view's suggested layout |
| [`renders/01_common_belief.kb.today.dot-layout.svg`](renders/01_common_belief.kb.today.dot-layout.svg), [`renders/02_packing.kb.today.dot-layout.svg`](renders/02_packing.kb.today.dot-layout.svg) | the same with `dot`, as the regeneration line draws it |
| [`renders/01_common_belief.kb.today.fold.dot`](renders/01_common_belief.kb.today.fold.dot) / [`.svg`](renders/01_common_belief.kb.today.fold.svg), [`renders/02_packing.kb.today.fold.dot`](renders/02_packing.kb.today.fold.dot) / [`.svg`](renders/02_packing.kb.today.fold.svg) | `ein render kb --fold`, by `fdp` |
| [`renders/01_common_belief.kb.tracked-at-b5fb8d70.dot`](renders/01_common_belief.kb.tracked-at-b5fb8d70.dot) / [`.svg`](renders/01_common_belief.kb.tracked-at-b5fb8d70.svg), [`renders/02_packing.kb.tracked-at-b5fb8d70.dot`](renders/02_packing.kb.tracked-at-b5fb8d70.dot) / [`.svg`](renders/02_packing.kb.tracked-at-b5fb8d70.svg) | the checked-in drawings at the first look |
| [`renders/01_common_belief.kb.tracked-at-0a4a97a3.dot`](renders/01_common_belief.kb.tracked-at-0a4a97a3.dot) / [`.svg`](renders/01_common_belief.kb.tracked-at-0a4a97a3.svg), [`renders/02_packing.kb.tracked-at-0a4a97a3.dot`](renders/02_packing.kb.tracked-at-0a4a97a3.dot) / [`.svg`](renders/02_packing.kb.tracked-at-0a4a97a3.svg) | the checked-in drawings at `0a4a97a3`, behind their binary |
| [`runs/01_common_belief.kb.tracked-vs-today.diff`](runs/01_common_belief.kb.tracked-vs-today.diff), [`runs/02_packing.kb.tracked-vs-today.diff`](runs/02_packing.kb.tracked-vs-today.diff) | `diff -u` checked-in against today: empty, exit 0 |
| [`runs/01_common_belief.kb.tracked-b5fb8d70-vs-today.diff`](runs/01_common_belief.kb.tracked-b5fb8d70-vs-today.diff), [`runs/02_packing.kb.tracked-b5fb8d70-vs-today.diff`](runs/02_packing.kb.tracked-b5fb8d70-vs-today.diff) | what S1f.23.1 changed in the checked-in drawings |
| [`runs/01_common_belief.kb.at-0a4a97a3.tracked-vs-that-binary.diff`](runs/01_common_belief.kb.at-0a4a97a3.tracked-vs-that-binary.diff), [`runs/02_packing.kb.at-0a4a97a3.tracked-vs-that-binary.diff`](runs/02_packing.kb.at-0a4a97a3.tracked-vs-that-binary.diff) | the same at `0a4a97a3`: behind |
| [`runs/drawings-history.txt`](runs/drawings-history.txt) | the commits that changed the two drawings |
| [`runs/shape-golden-vs-render-kb.txt`](runs/shape-golden-vs-render-kb.txt) | the golden's `dot[kb]` and `dot[kb-fold]` digests against `ein render kb`'s |
| [`runs/svg-is-dot-of-today.txt`](runs/svg-is-dot-of-today.txt), [`runs/programs-are-the-corpus.txt`](runs/programs-are-the-corpus.txt) | the two `cmp` checks |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds, each drawing |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/28-L-100/rerun.sh`. It needs
the worktree `build/wt-rev-0a4/` for the `0a4a97a3` comparison.

## 7. Found while building

- The row reproduces on today's binary: no difference, DOT and picture.
- **The drawings had to be drawn again by hand a third time** (S1f.23.1 named them in its list); nothing would have
  failed had it not.
- **The shape golden still does not cover them**: its `dot[kb]` and its new `dot[kb-fold]` digest the KB as loaded,
  not saturated.
- The regeneration line draws with `dot`, while the view suggests `fdp`: the checked-in pictures are a strip two ranks
  high whose arrow numbers crowd where arrows meet; `fdp` lays them out in a square with room for the numbers (R-18,
  item [55](../55-R-18/README.md)).
- Since the first look (b5fb8d70): the checked-in drawings went from labelled octagons and labelled arrows to the
  canonical form, with the binary; still no difference between them and the binary.

## 8. Your call

- **A test**: a crate test diffs each `.kb.dot` against `ein render kb`; the `.svg` stays drawn by hand (or gets a
  stamp).
- **The generator list**: a `utils/` generator with `--check` in the gate, the files listed in `AGENTS.md`; the
  `.dot` gains a *generated* line.
- **As they are**: the README's line, nothing holding them. (No default is recorded.)
