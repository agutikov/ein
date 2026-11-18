# L-96 — a literal `\n` in the KB view's octagon labels (and L-63 (f), the constraints view)

## 1. The question

`ein render kb` draws a fact of three arguments or more as an octagon, its label the relation's name in brackets
and, on a second line, where the fact comes from: the `:source` of a given fact, `by <rule>` for a derived one. The
second line was meant to be a line break, but the DOT the view writes has `\\n` (an escaped backslash, then `n`),
so Graphviz draws the two characters `\n` and keeps it on one line: `(gives)\ns1`, `(gives)\nby pass`. The same is
true of the constraint view's boxes, `next-to\n«symmetric»` (L-63 (f)). The fix is one line in the KB view (the
label is built with `"\\n"` and then quoted by `quote`, which escapes the backslash again), but it moves the drawing
of every program that has such an octagon, and the shape golden (which digests every rendering) has to be re-banked.
Do it, or leave it?

Recorded: [`../../findings.md`](../../../findings.md#the-engines-renderers), row **L-96** (P1f.22's N-24), and row
**L-63** (f) in the same section.

## 2. What is so today

All runs use today's binary, `build/review-bin/ein-head-b5fb8d70`.

**The witness** (the row's program, written out with real `:why` texts): [`programs/gives.ein`](programs/gives.ein).

```lisp
(relation gives Person Person Pet :why "{?1} gives {?2} the {?3}")
(gives Ann Bob Rex :source "s1")
(rule pass () :match (gives ?a ?b ?p) :assert (gives ?b ?a ?p) :why "{?b} gives {?p} back to {?a}")
(solve :no-search)
```

`ein render kb programs/gives.ein` → [`renders/gives.kb.dot`](renders/gives.kb.dot), drawn with `fdp` (the view's
suggested layout) → [`renders/gives.kb.svg`](renders/gives.kb.svg). The two lines
([`runs/gives.render-kb.grep-backslash-n.txt`](runs/gives.render-kb.grep-backslash-n.txt)):

<!-- output -->
```text
13:  f_3c979f278e [shape=octagon, label="(gives)\\ns1", color="#1f77b4", fontcolor="#1f77b4", style=solid];
17:  f_eeb47e8530 [shape=octagon, label="(gives)\\nby pass", color="#1f77b4", fontcolor="#1f77b4", style=dashed];
```

![ein render kb: the octagons read (gives)\ns1 and (gives)\nby pass, on one line each](renders/gives.kb.svg)

- Every KB variant has it: `--colour-by origin`, `--colour-by none`, `--no-types`, `--no-instances` each print the
  same two labels ([`runs/gives.variants.count-backslash-n.tsv`](runs/gives.variants.count-backslash-n.tsv); the
  DOTs and SVGs are `renders/gives.kb-*.{dot,svg}`).
- The region trace's KB snapshots (`ein solve --trace OUT.md -F`) draw the KB the same way and carry the same two
  labels: [`runs/gives.trace-full-kb.md`](runs/gives.trace-full-kb.md) lines 82 and 86
  (the read-out of that run: [`runs/gives.solve.read-out.txt`](runs/gives.solve.read-out.txt)).
- A corpus program, copied: [`programs/12_expect_false.ein`](programs/12_expect_false.ein) (from
  `examples/features/12_expect_false.ein`) draws `(false)\nby no-room`:
  [`renders/12_expect_false.kb.dot`](renders/12_expect_false.kb.dot) line 28,
  [`renders/12_expect_false.kb.svg`](renders/12_expect_false.kb.svg).

**How many corpus programs.** `ein render kb` on every program of `corpus/corpus.toml` (all 894, cheap: a few
seconds on 12 workers):

| | programs |
|---|---|
| in the corpus | 894 |
| refused at load (all under `examples/ein-bugs/` and `examples/broken/`, by design) | 224 ([list](runs/corpus-sweep.render-kb.refused.tsv)) |
| loaded and drawn | 670 |
| with at least one octagon | 563 |
| **with at least one octagon label carrying a literal `\n`** | **329** |
| such labels, in all | 95 605 (95 496 `by <rule>`, 109 a `:source`) |

Per program: [`runs/corpus-sweep.render-kb.tsv`](runs/corpus-sweep.render-kb.tsv) (labels hit, octagons, path). By
directory: `tests/stdlib/` 221, `examples/` 101 (`word_problems` 20, `arith` 14, `binary_logic` 12, `beyond_arith`
12, `binary_arithmetic` 5, `features` 3, `beliefs` 3, the six zebra programs, …), `stdlib/bool` 4, other `tests/`
3. Every one of the 329 has `dot[kb]` lines in the shape golden
(`ein.rs/crates/ein-render/tests/golden/corpus_shapes.md5`), which digests six KB renderings per program (`kb`,
`kb-origin`, `kb-none`, `kb-no-types`, `kb-no-instances`, `kb-since`); so a fix moves at most 329 × 6 golden lines.
Fewer in fact: the golden's five plain KB views draw the KB **as loaded, before saturation** (`ein-render/src/shape.rs`,
only `kb-since` saturates), so a derived octagon's `by <rule>` label is only in its `kb-since` line, while a
`:source` octagon's is in all six (read off the golden and `shape.rs`, not run; the exact number needs the
library).

**Pictures in the tree that show it** (grep of the tracked SVGs' text):
- `examples/beliefs/01_common_belief.kb.svg`: `(likes)\nby common-belief-likes`, `(not)\nby common-belief-not-likes`;
  `examples/beliefs/02_packing.kb.svg`: `(confirmed)\nby confirm` (these are L-100's two drawings).
- `docs/ein/user/03-syntax-and-semantics/diagrams/07-constraints-view.svg`, on the user's page
  `07-properties-and-operators.md`: `knows\n«symmetric, transitive»`, `above\n«asymmetric»`, `std.algebra\n«import»`.
  Copied as [`renders/page-07-constraints-view.tracked.dot`](renders/page-07-constraints-view.tracked.dot) /
  [`.svg`](renders/page-07-constraints-view.tracked.svg).

### The constraints view, L-63 (f)

`ein render constraints examples/zebra2.ein` → [`renders/zebra2.constraints.dot`](renders/zebra2.constraints.dot),
[`renders/zebra2.constraints.svg`](renders/zebra2.constraints.svg): 10 labels with `\\n`
([`runs/zebra2.render-constraints.grep-backslash-n.txt`](runs/zebra2.render-constraints.grep-backslash-n.txt)), as
the row says, e.g. `"next-to" [shape=rectangle, label="next-to\\n«symmetric»"]`.

![ein render constraints zebra2: every box reads name\n«property» on one line](renders/zebra2.constraints.svg)

Over the corpus, 485 of the 670 programs that load have such a label in their constraint view, 1 862 labels in all
([`runs/corpus-sweep.render-constraints.tsv`](runs/corpus-sweep.render-constraints.tsv)). In the code (read, not
run) the constraint view builds the label at two sites, `constraints.rs` lines 275 and 296
(`format!("{rel}\\n«{}»", …)`), each then quoted by `dot_util::quote`; the KB view at one,
`kb_dot.rs` line 219 in `hyperedge_node`.

## 3. The options

**Fix**: build the label with a real DOT line break, `\n` in the DOT text. Shown here as **mocks**: each is the
binary's DOT with every `\\n` replaced by `\n` (`sed 's/\\\\n/\\n/g'`) and nothing else, then drawn:

| today | mock of the fix | the edit |
|---|---|---|
| [`renders/gives.kb.svg`](renders/gives.kb.svg) | [`renders/gives.kb.fixed.MOCK.svg`](renders/gives.kb.fixed.MOCK.svg) ([dot](renders/gives.kb.fixed.MOCK.dot)) | [diff](runs/gives.kb.vs-MOCK-fixed.diff) |
| [`renders/12_expect_false.kb.svg`](renders/12_expect_false.kb.svg) | [`renders/12_expect_false.kb.fixed.MOCK.svg`](renders/12_expect_false.kb.fixed.MOCK.svg) | [diff](runs/12_expect_false.kb.vs-MOCK-fixed.diff) |
| [`renders/zebra2.constraints.svg`](renders/zebra2.constraints.svg) | [`renders/zebra2.constraints.fixed.MOCK.svg`](renders/zebra2.constraints.fixed.MOCK.svg) | [diff](runs/zebra2.constraints.vs-MOCK-fixed.diff) |
| [`renders/page-07-constraints-view.tracked.svg`](renders/page-07-constraints-view.tracked.svg) | [`renders/page-07-constraints-view.tracked.fixed.MOCK.svg`](renders/page-07-constraints-view.tracked.fixed.MOCK.svg) | [diff](runs/page-07-constraints-view.tracked.vs-MOCK-fixed.diff) |

Side by side, today left and the mock right: [`renders/gives.kb.before-after.png`](renders/gives.kb.before-after.png);
today above and the mock below: [`renders/zebra2.constraints.before-after.png`](renders/zebra2.constraints.before-after.png).

![mock of the fix: the octagons read (gives) over s1, and (gives) over by pass](renders/gives.kb.fixed.MOCK.svg)

What it changes: the label of 95 605 octagons in 329 programs' KB views and 1 862 boxes in 485 programs' constraint
views; the shape golden's lines for those renderings re-banked; the three tracked pictures above drawn again (the
beliefs' two by their regeneration line, the user page's constraint view by `utils/render_diagrams.py`, its `.dot`
edited too, since it is hand-copied output). No answer moves: it is a label.

**As is**: the labels stay one line with a visible `\n`; the user's page `07-properties-and-operators.md` keeps
showing it.

## 4. The default

The row names no default: it is **the user's call**, and until ruled the view stays as it is. (The row says the cost:
*"a one-line fix, which moves the drawing of every program with such an octagon, and a re-bank of the shape
golden"*.)

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`programs/gives.ein`](programs/gives.ein) | the row's witness, written for this review |
| [`programs/12_expect_false.ein`](programs/12_expect_false.ein) | copy of `examples/features/12_expect_false.ein` |
| [`renders/gives.kb.dot`](renders/gives.kb.dot) / [`.svg`](renders/gives.kb.svg) | `ein render kb` of the witness, drawn with `fdp` |
| [`renders/gives.kb.fixed.MOCK.dot`](renders/gives.kb.fixed.MOCK.dot) / [`.svg`](renders/gives.kb.fixed.MOCK.svg) | the same with `\\n` → `\n` (mock) |
| [`renders/gives.kb-origin.dot`](renders/gives.kb-origin.dot) / [`.svg`](renders/gives.kb-origin.svg) | `--colour-by origin` |
| [`renders/gives.kb-none.dot`](renders/gives.kb-none.dot) / [`.svg`](renders/gives.kb-none.svg) | `--colour-by none` |
| [`renders/gives.kb-no-types.dot`](renders/gives.kb-no-types.dot) / [`.svg`](renders/gives.kb-no-types.svg) | `--no-types` |
| [`renders/gives.kb-no-instances.dot`](renders/gives.kb-no-instances.dot) / [`.svg`](renders/gives.kb-no-instances.svg) | `--no-instances` |
| [`renders/gives.kb.before-after.png`](renders/gives.kb.before-after.png) | today and the mock, side by side |
| [`renders/12_expect_false.kb.dot`](renders/12_expect_false.kb.dot) / [`.svg`](renders/12_expect_false.kb.svg) | `ein render kb` of the corpus program |
| [`renders/12_expect_false.kb.fixed.MOCK.dot`](renders/12_expect_false.kb.fixed.MOCK.dot) / [`.svg`](renders/12_expect_false.kb.fixed.MOCK.svg) | its mock |
| [`renders/zebra2.constraints.dot`](renders/zebra2.constraints.dot) / [`.svg`](renders/zebra2.constraints.svg) | `ein render constraints examples/zebra2.ein` |
| [`renders/zebra2.constraints.fixed.MOCK.dot`](renders/zebra2.constraints.fixed.MOCK.dot) / [`.svg`](renders/zebra2.constraints.fixed.MOCK.svg) | its mock |
| [`renders/zebra2.constraints.before-after.png`](renders/zebra2.constraints.before-after.png) | today above, the mock below |
| [`renders/page-07-constraints-view.tracked.dot`](renders/page-07-constraints-view.tracked.dot) / [`.svg`](renders/page-07-constraints-view.tracked.svg) | copies of the user page's tracked drawing |
| [`renders/page-07-constraints-view.tracked.fixed.MOCK.dot`](renders/page-07-constraints-view.tracked.fixed.MOCK.dot) / [`.svg`](renders/page-07-constraints-view.tracked.fixed.MOCK.svg) | its mock |
| [`runs/gives.render-kb.grep-backslash-n.txt`](runs/gives.render-kb.grep-backslash-n.txt) | the two label lines |
| [`runs/gives.variants.count-backslash-n.tsv`](runs/gives.variants.count-backslash-n.tsv) | the count per KB variant |
| [`runs/gives.trace-full-kb.md`](runs/gives.trace-full-kb.md) | `ein solve --trace … -F`: the region trace with KB snapshots |
| [`runs/gives.solve.read-out.txt`](runs/gives.solve.read-out.txt) | that run's read-out |
| [`runs/zebra2.render-constraints.grep-backslash-n.txt`](runs/zebra2.render-constraints.grep-backslash-n.txt) | the constraint view's ten lines |
| [`runs/corpus-sweep.render-kb.tsv`](runs/corpus-sweep.render-kb.tsv) | per corpus program that loads: labels hit, octagons |
| [`runs/corpus-sweep.render-kb.refused.tsv`](runs/corpus-sweep.render-kb.refused.tsv) | the 224 refused at load |
| [`runs/corpus-sweep.render-constraints.tsv`](runs/corpus-sweep.render-constraints.tsv) | per corpus program: exit, labels hit |
| [`runs/gives.kb.vs-MOCK-fixed.diff`](runs/gives.kb.vs-MOCK-fixed.diff) | `diff -u` of the witness's KB DOT against its mock |
| [`runs/12_expect_false.kb.vs-MOCK-fixed.diff`](runs/12_expect_false.kb.vs-MOCK-fixed.diff) | the same, the corpus program |
| [`runs/zebra2.constraints.vs-MOCK-fixed.diff`](runs/zebra2.constraints.vs-MOCK-fixed.diff) | the same, zebra2's constraint view |
| [`runs/page-07-constraints-view.tracked.vs-MOCK-fixed.diff`](runs/page-07-constraints-view.tracked.vs-MOCK-fixed.diff) | the same, the user page's drawing |

## 6. Re-run

From the repo root, `E=build/review-bin/ein-head-b5fb8d70`, `D=plans/m1f_hypothesis_and_documentation/review/03-drawings/10-L-96`:

```sh
$E render kb $D/programs/gives.ein > $D/renders/gives.kb.dot
for o in "--colour-by origin:origin" "--colour-by none:none" "--no-types:no-types" "--no-instances:no-instances"; do
  $E render kb $D/programs/gives.ein ${o%%:*} > $D/renders/gives.kb-${o##*:}.dot; done
$E render kb $D/programs/12_expect_false.ein > $D/renders/12_expect_false.kb.dot
$E render constraints examples/zebra2.ein > $D/renders/zebra2.constraints.dot
$E solve $D/programs/gives.ein --trace $D/runs/gives.trace-full-kb.md -F > $D/runs/gives.solve.read-out.txt
for f in gives.kb 12_expect_false.kb zebra2.constraints page-07-constraints-view.tracked; do
  sed 's/\\\\n/\\n/g' $D/renders/$f.dot > $D/renders/$f.fixed.MOCK.dot; done
for f in $D/renders/*kb*.dot; do fdp -Tsvg "$f" > "${f%.dot}.svg"; done
for f in $D/renders/zebra2*.dot $D/renders/*fixed.MOCK.dot; do case $f in *kb*) ;; *) dot -Tsvg "$f" > "${f%.dot}.svg";; esac; done
# the corpus sweep: every corpus program, its KB view, the octagon labels with a literal \n
grep -E '^path\s*=' corpus/corpus.toml | sed -E 's/^path\s*=\s*"(.*)"/\1/' | while read p; do
  $E render kb "$p" 2>/dev/null | grep -c 'shape=octagon, label="[^"]*\\\\n' | sed "s|\$|\t$p|"; done
```

## 7. Found while building

- The row's witness and counts reproduce: `(gives)\\ns1`, `(gives)\\nby pass`; ten in zebra2's constraint view.
- It is on a user page: `docs/ein/user/03-syntax-and-semantics/diagrams/07-constraints-view.svg` (a hand-copied
  constraint-view output) draws `knows\n«symmetric, transitive»` on `07-properties-and-operators.md`. The row
  names the beliefs' drawings only through L-100, and the user page not at all.
- The constraint view builds its label at two sites, not one, so its fix is two lines.
- The trace's `-F` KB snapshots carry the same labels (the row does not name the trace).
- The shape golden's `dot[kb]`, `kb-origin`, `kb-none`, `kb-no-types` and `kb-no-instances` lines digest the KB
  **as loaded**, not saturated, where `ein render kb` saturates (`shape.rs`); so the `by <rule>` labels, 95 496 of
  the 95 605, reach the golden only through `kb-since`. (Also L-100's: see `../28-L-100/`.)

## 8. Your call

- **Fix**: a real line break in the KB view's octagons (one line) and the constraint view's boxes (two lines); the
  drawings of 329 + 485 programs move, the golden re-banked, three tracked pictures drawn again.
- **As is**: the visible `\n` stays. (No default is recorded; until ruled, it stays as is.)
