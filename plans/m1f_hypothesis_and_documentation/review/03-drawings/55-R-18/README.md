# R-18 — KB views laid out by `dot` are wide strips; the binary's `fdp` hint is a comment

## 1. The question

`ein render kb` (and the program view's digraph `kb`) writes, as its second line, the comment
`// suggested layout: fdp`. Graphviz does not read comments: `ein render kb FILE | dot -Tsvg`, as the rendering page
writes it, lays the view out with `dot`, whose layered layout puts every atom on one rank and every fact's octagon on
the next, a strip two ranks high, the arrows' numbers crowded where arrows meet. The pages' KB drawings are drawn so:
their sources are the binary's output with the comment dropped (or kept, unread), and `utils/render_diagrams.py` draws
every source with `dot -Tsvg`. `fdp`, the layout the comment names, spreads the same graph in two dimensions. Should
the hint be something Graphviz reads, should the pages' tool read it, or should the pictures stay as they are?

Recorded: P1f.23's [findings](../../../p1f.23_review_processing/findings.md), row **R-18** (S1f.23.1 task 2's
finding 4): *"KB views laid out by `dot` are wide two-rank strips with crowded arrow numbers (`05-program-kb`,
`12-kb-view`, `05-jack-canonical`): the binary's `fdp` hint is a comment the sources drop"* — **the user's, at the
review of the drawings**.

## 2. What is so today

**The pages' KB drawings**, 36 sources whose digraph is `kb` ([`runs/kb-sources.tsv`](runs/kb-sources.tsv), by
[`tools/kb-sources.sh`](tools/kb-sources.sh)): **31 drop the comment** (copies of the binary's output, `//` lines
dropped), **5 keep it** (the rendering page's `12-*`, which keep the binary's output whole), and **none names a
`layout` Graphviz reads**. `render_diagrams.py` draws a `.dot` with `dot -Tsvg`, and its docstring says *"a graph
that names its `layout` is laid out by it"* (read, not run): so all 36 are `dot`'s.

**The three pictures R-18 names**, each as the page draws it (`dot`) and by `fdp` ([`runs/svg-sizes.tsv`](runs/svg-sizes.tsv),
[`runs/draw-times.tsv`](runs/draw-times.tsv); every one under a tenth of a second):

| page source | `dot`, as on the page | `fdp` |
|---|---|---|
| `03-syntax-and-semantics/diagrams/05-program-kb` | [976 × 125 pt](renders/syntax.05-program-kb.dot-layout.svg) | [730 × 622 pt](renders/syntax.05-program-kb.fdp-layout.svg) |
| `04-reference/diagrams/12-kb-view` | [898 × 125 pt](renders/ref.12-kb-view.dot-layout.svg) | [577 × 288 pt](renders/ref.12-kb-view.fdp-layout.svg) |
| `02-guide/diagrams/05-jack-canonical` | [539 × 125 pt](renders/guide.05-jack-canonical.dot-layout.svg) | [385 × 307 pt](renders/guide.05-jack-canonical.fdp-layout.svg) |

![05-program-kb as the syntax page draws it, by dot: a strip of 21 octagons under a row of twelve boxes, the arrows' numbers and the rules' notes overlapping along the bottom](renders/syntax.05-program-kb.dot-layout.svg)

![The same source by fdp: the declarations' octagons around the boxes relation and Person, the facts of parent and has-dog around Bea, the derived facts of ancestor and pet-owner dashed with their rules beside them, the numbers readable](renders/syntax.05-program-kb.fdp-layout.svg)

**A real program**, [`programs/zebra2.ein`](programs/zebra2.ein) (a copy of `examples/zebra2.ein`), its whole KB view:

| view | edges | `dot` | `fdp` |
|---|---|---|---|
| `ein render kb` | 1 446 | 0.5 s, **16 104 × 201 pt** ([svg](renders/zebra2.kb.dot-layout.svg)) | 13.3 s, 1 947 × 1 613 pt ([svg](renders/zebra2.kb.fdp-layout.svg)) |
| `ein render kb --fold` | 1 042 | 0.3 s, 10 909 × 809 pt ([svg](renders/zebra2.kb.fold.dot-layout.svg)) | 3.9 s, 1 890 × 1 520 pt ([svg](renders/zebra2.kb.fold.fdp-layout.svg)) |

(seconds on this machine, shared with another agent's test runs.) Neither reads: `dot`'s is a strip eighty times as
wide as it is high; `fdp`'s a knot of 452 facts, the 62 stored negations a fan of red rule notes around the box `not`.

## 3. The options

- **(a) as it is**: the comment stays a comment; the pages' KB pictures are `dot`'s strips; a reader who wants the
  other layout runs `fdp`. No default is recorded: R-18 is the user's.
- **(b) the pages' tool reads the hint**: the copies keep `// suggested layout: fdp` and `render_diagrams.py` lays a
  source out by the layout its comment names; the 36 pictures drawn again. A change to the process's tool, not to
  the binary; `ein render kb | dot -Tsvg` stays a strip for everyone else.
- **(c) the binary writes the layout as an attribute**, `layout=fdp;` in place of the comment, which Graphviz reads
  whatever command lays it out: `dot -Tsvg` then draws fdp's picture. **A mock** of each of the three sources, edited
  by hand: the comment taken off and `layout=fdp;` written after `digraph kb {`, drawn by `dot -Tsvg` as
  `render_diagrams.py` would
  ([`renders/syntax.05-program-kb.layout-fdp.MOCK.dot`](renders/syntax.05-program-kb.layout-fdp.MOCK.dot) /
  [svg](renders/syntax.05-program-kb.layout-fdp.MOCK.svg),
  [`renders/ref.12-kb-view.layout-fdp.MOCK.dot`](renders/ref.12-kb-view.layout-fdp.MOCK.dot) /
  [svg](renders/ref.12-kb-view.layout-fdp.MOCK.svg),
  [`renders/guide.05-jack-canonical.layout-fdp.MOCK.dot`](renders/guide.05-jack-canonical.layout-fdp.MOCK.dot) /
  [svg](renders/guide.05-jack-canonical.layout-fdp.MOCK.svg)): each picture is the size of `fdp`'s, so `dot -Tsvg`
  honours it. A code change (`kb_dot.rs`, and the program view's `kb`); every user's `| dot -Tsvg` lays a KB out by
  `fdp`; and `dot_wellformed.rs`, which lays out each view of every corpus program per commit with the `dot` command
  (read, not run), would lay the KB views out by `fdp`, zebra2's in 13 s where `dot` takes half a second.
- **(d) per drawing, on the pages**: a page's copy names `layout=fdp` where that reads better; a copy is then the
  binary's output plus one line, which Q-M1f.468's reading of a copy would have to allow.

![Mock (c): 05-program-kb's source naming layout=fdp, drawn by dot -Tsvg, the same picture fdp draws](renders/syntax.05-program-kb.layout-fdp.MOCK.svg)

## 4. The default

None: R-18 is *"the user's, at the review of the drawings"*. Until ruled, (a).

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing, from the repo root |
| [`tools/kb-sources.sh`](tools/kb-sources.sh) | the pages' `digraph kb` sources, the comment kept or not, a `layout` named or not |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| [`runs/kb-sources.tsv`](runs/kb-sources.tsv) | the 36 sources |
| [`runs/draw-times.tsv`](runs/draw-times.tsv), [`runs/svg-sizes.tsv`](runs/svg-sizes.tsv) | Graphviz's layout and seconds each drawing; each picture's size |
| [`renders/syntax.05-program-kb.dot`](renders/syntax.05-program-kb.dot), [`renders/ref.12-kb-view.dot`](renders/ref.12-kb-view.dot), [`renders/guide.05-jack-canonical.dot`](renders/guide.05-jack-canonical.dot) | the three page sources, copied |
| `renders/*.dot-layout.svg`: [`syntax.05-program-kb`](renders/syntax.05-program-kb.dot-layout.svg), [`ref.12-kb-view`](renders/ref.12-kb-view.dot-layout.svg), [`guide.05-jack-canonical`](renders/guide.05-jack-canonical.dot-layout.svg) | each by `dot`, as the page |
| `renders/*.fdp-layout.svg`: [`syntax.05-program-kb`](renders/syntax.05-program-kb.fdp-layout.svg), [`ref.12-kb-view`](renders/ref.12-kb-view.fdp-layout.svg), [`guide.05-jack-canonical`](renders/guide.05-jack-canonical.fdp-layout.svg) | each by `fdp` |
| `renders/*.layout-fdp.MOCK.dot` / `.svg`: [`syntax.05-program-kb`](renders/syntax.05-program-kb.layout-fdp.MOCK.dot), [`ref.12-kb-view`](renders/ref.12-kb-view.layout-fdp.MOCK.dot), [`guide.05-jack-canonical`](renders/guide.05-jack-canonical.layout-fdp.MOCK.dot) | mock (c): the source naming `layout=fdp`, drawn by `dot -Tsvg` |
| [`renders/zebra2.kb.dot`](renders/zebra2.kb.dot), [`renders/zebra2.kb.fold.dot`](renders/zebra2.kb.fold.dot) | `ein render kb [--fold]` of zebra2 |
| [`renders/zebra2.kb.dot-layout.svg`](renders/zebra2.kb.dot-layout.svg), [`renders/zebra2.kb.fdp-layout.svg`](renders/zebra2.kb.fdp-layout.svg), [`renders/zebra2.kb.fold.dot-layout.svg`](renders/zebra2.kb.fold.dot-layout.svg), [`renders/zebra2.kb.fold.fdp-layout.svg`](renders/zebra2.kb.fold.fdp-layout.svg) | each by `dot` and by `fdp` |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/55-R-18/rerun.sh`. It keeps the
three `.MOCK.dot` files and draws them; zebra2's `fdp` drawing takes some seconds.

## 7. Found while building

- **The five sources that keep the comment are drawn by `dot` too**: the rendering page's `12-*` hold the binary's
  output whole, `// suggested layout: fdp` and all, and `render_diagrams.py` reads no comment.
- **`dot -Tsvg` honours a `layout=fdp` attribute**: each mock's picture is exactly `fdp`'s size.
- **A whole KB view of a real program reads in neither layout**: zebra2's is a strip of 16 104 pt by `dot`, a knot by
  `fdp`. The pages show such a program cut (Q-M1f.468, item [48](../48-Q-M1f.468/README.md)); what a reader of
  `ein render kb examples/zebra2.ein`, the rendering page's first command, gets is one of these two.

## 8. Your call

- **(a)** as it is: `dot`'s strips on the pages, the hint a comment.
- **(b)** `render_diagrams.py` reads the comment; the 36 pictures drawn again by `fdp`.
- **(c)** the binary writes `layout=fdp` as an attribute (every `dot -Tsvg` follows it; the per-commit DOT sweep lays
  the KB views out by `fdp`).
- **(d)** per drawing, a page's copy names its layout.
