# L-100 — the beliefs' two KB drawings are no generator's and no test's

## 1. The question

`examples/beliefs/` keeps, beside two of its programs, the knowledge base `ein render kb` draws for each, as a DOT
and its picture: `01_common_belief.kb.dot` / `.kb.svg` and `02_packing.kb.dot` / `.kb.svg`. They are drawn by a
command line written in `examples/README.md`, by hand, each time the renderer changes. Nothing compares them with
the binary: not a test, not a generator's `--check`. During the nested-facts work the renderer moved twice and the
drawings were found behind it once. Should they be held to `ein render kb` by a test, be put on `AGENTS.md`'s list
of generated files with a generator, or stay as they are?

Recorded: [`../../findings.md`](../../../findings.md#examples), row **L-100** (P1f.22's N-33).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-b5fb8d70`; Graphviz 16.0.0.

**They match the binary today.** `ein render kb examples/beliefs/NN.ein` against the checked-in `NN.kb.dot`:

| program | diff | file |
|---|---|---|
| `01_common_belief` | no difference (exit 0) | [`runs/01_common_belief.kb.tracked-vs-today.diff`](runs/01_common_belief.kb.tracked-vs-today.diff) (empty) |
| `02_packing` | no difference (exit 0) | [`runs/02_packing.kb.tracked-vs-today.diff`](runs/02_packing.kb.tracked-vs-today.diff) (empty) |

and the checked-in `.kb.svg` of each is byte for byte `dot -Tsvg` of today's output on this machine's Graphviz
(`cmp`, no difference). So the row's *"They match it at the close"* holds on master's binary too.

The drawings (the checked-in pictures, copied):

- [`renders/01_common_belief.kb.tracked.svg`](renders/01_common_belief.kb.tracked.svg)
  ([dot](renders/01_common_belief.kb.tracked.dot)) — `(likes Dan Soup)` and `(not (likes Dan Fish))` held (derived by
  the two common-belief rules, dashed, in no frame), `(likes Dan Tea)`, `(likes Eve Fish)` in the frame
  `believes ctx`, `(likes Dan Fish)` in `not ctx`.

  ![01_common_belief: the saturated KB, two held beliefs outside every frame, two in believes ctx, one in not ctx](renders/01_common_belief.kb.tracked.svg)

- [`renders/02_packing.kb.tracked.svg`](renders/02_packing.kb.tracked.svg)
  ([dot](renders/02_packing.kb.tracked.dot)) — `(likes Dan Soup)` packed by `witnessed` and confirmed, held;
  `(likes Dan Tea)` in `claims ctx`.

  ![02_packing: the saturated KB, the packed fact held and confirmed, Bob's other claim in claims ctx](renders/02_packing.kb.tracked.svg)

The view's own DOT says `// suggested layout: fdp`, but the regeneration line draws with `dot`; the `fdp` pictures,
for comparison: [`renders/01_common_belief.kb.today.fdp-layout.svg`](renders/01_common_belief.kb.today.fdp-layout.svg),
[`renders/02_packing.kb.today.fdp-layout.svg`](renders/02_packing.kb.today.fdp-layout.svg). (Both drawings also carry
L-96's literal `\n`: `(confirmed)\nby confirm`, `(likes)\nby common-belief-likes`.)

**The regeneration line**, `examples/README.md`, the `beliefs/` row (quoted):

> Beside each of the two, its saturated KB drawn by `ein render kb` (`NN_….kb.dot`, `.kb.svg`; regenerate with
> `ein render kb examples/beliefs/NN_….ein > NN_….kb.dot && dot -Tsvg NN_….kb.dot -o NN_….kb.svg`).

and `corpus/corpus.toml`'s note on `02_packing` ends *"Its KB drawn beside it, `02_packing.kb.dot` and `.svg`
(`ein render kb`)."* No other file names them (`grep` of the `.md`, `.rs`, `.py`, `.toml`, `.ein` files).

**They were behind once.** At `0a4a97a3` (after every object was drawn once, `811846ce`, and before they were drawn
again in `bf9693dc`), that commit's binary against that commit's checked-in DOTs
(`build/review-bin/ein-0a4a97a3`, `EIN_STDLIB` its tree's): both differ —
[`runs/01_common_belief.kb.at-0a4a97a3.tracked-vs-that-binary.diff`](runs/01_common_belief.kb.at-0a4a97a3.tracked-vs-that-binary.diff),
[`runs/02_packing.kb.at-0a4a97a3.tracked-vs-that-binary.diff`](runs/02_packing.kb.at-0a4a97a3.tracked-vs-that-binary.diff).
The checked-in drawing still had nodes labelled with a fact's text, `"(likes Dan Soup)"`, where the binary already
drew the octagon `f_b10ac10970`. The pictures then:
[`renders/01_common_belief.kb.tracked-at-0a4a97a3.svg`](renders/01_common_belief.kb.tracked-at-0a4a97a3.svg),
[`renders/02_packing.kb.tracked-at-0a4a97a3.svg`](renders/02_packing.kb.tracked-at-0a4a97a3.svg).

**The shape golden does not hold them either.** `corpus_shapes.md5` has a `dot[kb]` line for each program, but its
digest is not `ein render kb`'s output: `01_common_belief` `bc835a09ec1c521e 55L` against the binary's
`f316b39f53c1a54e…` 55 lines; `02_packing` `489082959760603c 42L` against `532fb84514a4b9ab…` 48 lines
([`runs/shape-golden-vs-render-kb.txt`](runs/shape-golden-vs-render-kb.txt)). Read from `ein-render/src/shape.rs`:
the golden's plain KB views draw the KB as loaded, before saturation (only `kb-since` saturates), while
`ein render kb` and these drawings are of the saturated KB.

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
`ein render kb`'s output.

### As they are

The line in `examples/README.md` regenerates them; nothing catches drift, as `0a4a97a3` shows. Their use is to be
looked at beside the program, and nothing reads them.

## 4. The default

The row names no default: **the user's call**, *"a test that holds them to `ein render kb`, the generator list of
`AGENTS.md`, or as they are"*. Until ruled, they stay as they are.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`programs/01_common_belief.ein`](programs/01_common_belief.ein), [`programs/02_packing.ein`](programs/02_packing.ein) | copies of `examples/beliefs/` |
| [`renders/01_common_belief.kb.tracked.dot`](renders/01_common_belief.kb.tracked.dot) / [`.svg`](renders/01_common_belief.kb.tracked.svg) | the checked-in drawing, copied |
| [`renders/02_packing.kb.tracked.dot`](renders/02_packing.kb.tracked.dot) / [`.svg`](renders/02_packing.kb.tracked.svg) | the same for `02_packing` |
| [`renders/01_common_belief.kb.today.dot`](renders/01_common_belief.kb.today.dot), [`renders/02_packing.kb.today.dot`](renders/02_packing.kb.today.dot) | `ein render kb` today |
| [`renders/01_common_belief.kb.today.fdp-layout.svg`](renders/01_common_belief.kb.today.fdp-layout.svg), [`renders/02_packing.kb.today.fdp-layout.svg`](renders/02_packing.kb.today.fdp-layout.svg) | today's output drawn with `fdp`, the view's suggested layout |
| [`renders/01_common_belief.kb.tracked-at-0a4a97a3.dot`](renders/01_common_belief.kb.tracked-at-0a4a97a3.dot) / [`.svg`](renders/01_common_belief.kb.tracked-at-0a4a97a3.svg) | the checked-in drawing at `0a4a97a3`, behind its binary |
| [`renders/02_packing.kb.tracked-at-0a4a97a3.dot`](renders/02_packing.kb.tracked-at-0a4a97a3.dot) / [`.svg`](renders/02_packing.kb.tracked-at-0a4a97a3.svg) | the same for `02_packing` |
| [`runs/01_common_belief.kb.tracked-vs-today.diff`](runs/01_common_belief.kb.tracked-vs-today.diff), [`runs/02_packing.kb.tracked-vs-today.diff`](runs/02_packing.kb.tracked-vs-today.diff) | `diff -u` checked-in against today: empty |
| [`runs/01_common_belief.kb.at-0a4a97a3.tracked-vs-that-binary.diff`](runs/01_common_belief.kb.at-0a4a97a3.tracked-vs-that-binary.diff), [`runs/02_packing.kb.at-0a4a97a3.tracked-vs-that-binary.diff`](runs/02_packing.kb.at-0a4a97a3.tracked-vs-that-binary.diff) | the same at `0a4a97a3`: behind |
| [`runs/shape-golden-vs-render-kb.txt`](runs/shape-golden-vs-render-kb.txt) | the golden's `dot[kb]` digests against `ein render kb`'s |

## 6. Re-run

From the repo root, `E=build/review-bin/ein-head-b5fb8d70`, `D=plans/m1f_hypothesis_and_documentation/review/03-drawings/28-L-100`:

```sh
for n in 01_common_belief 02_packing; do
  $E render kb examples/beliefs/$n.ein > $D/renders/$n.kb.today.dot
  diff -u examples/beliefs/$n.kb.dot $D/renders/$n.kb.today.dot > $D/runs/$n.kb.tracked-vs-today.diff; echo "$n $?"
  dot -Tsvg $D/renders/$n.kb.today.dot | cmp - examples/beliefs/$n.kb.svg && echo "$n svg same"
  fdp -Tsvg $D/renders/$n.kb.today.dot -o $D/renders/$n.kb.today.fdp-layout.svg
  # behind at 0a4a97a3: that commit's binary on that commit's tree
  EIN_STDLIB=$PWD/build/wt-rev-0a4/stdlib build/review-bin/ein-0a4a97a3 render kb build/wt-rev-0a4/examples/beliefs/$n.ein \
    | diff -u build/wt-rev-0a4/examples/beliefs/$n.kb.dot - > $D/runs/$n.kb.at-0a4a97a3.tracked-vs-that-binary.diff
done
grep -E 'beliefs/0[12]_[a-z_]+\.ein::dot\[kb\]' ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5
```

## 7. Found while building

- The row reproduces: no difference on master's binary.
- **The shape golden does not cover them**: its `dot[kb]` digests the KB as loaded, not saturated, so even a
  per-commit golden line for each program is not a check of `ein render kb`'s output or of these files. (This also
  bears on L-96's count of moved golden lines.)
- The regeneration line draws with `dot`, while the view suggests `fdp`; the checked-in pictures are `dot`'s
  (byte-identical here).

## 8. Your call

- **A test**: a crate test diffs each `.kb.dot` against `ein render kb`; the `.svg` stays drawn by hand (or gets a
  stamp).
- **The generator list**: a `utils/` generator with `--check` in the gate, the files listed in `AGENTS.md`; the
  `.dot` gains a *generated* line.
- **As they are**: the README's line, nothing holding them. (No default is recorded.)
