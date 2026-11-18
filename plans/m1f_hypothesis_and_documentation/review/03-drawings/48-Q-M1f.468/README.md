# Q-M1f.468 — a page's KB drawing: the whole view, or a cut of it

## 1. The question

`ein render kb` draws every fact the knowledge base holds (Q-M1f.432): beside the facts a page is about, every
declaration's two facts, every fact that applies a rule, every activator a module's setup derives, every stored
negation. When the pages were drawn again, a page's KB drawing was allowed to be **a cut** of the view: the facts of
the relations the section is about and every fact they point at, a framed fact kept only where a kept fact points
at it, the page saying what is left out. The rule that a drawing of the engine's is kept as the file the command
wrote (Q-M1f.368) was read as being about a whole view. Is a cut allowed, or must a page show the whole view, however
large?

Recorded: [`open_questions.md` § Q-M1f.468](../../../open_questions.md#q-m1f468--a-pages-kb-drawing-the-whole-view-or-a-cut),
S1f.23.1's task 2, its reading 2.

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; the pages as merged (`231486f0`).

**13 page drawings are cuts**, each with a sentence of its page that says so
([`runs/cut-drawings.md`](runs/cut-drawings.md), by [`tools/cuts.py`](tools/cuts.py)): the reasoning pages' firing,
saturation and soup states (*"drawn without its declarations' facts"*), six of the guide's (*"from `ein render kb`
on that file, cut to the facts of `is-a`"*), and the standard library's three states (*"the declarations' facts and
the facts that apply a rule … are left out"*). The other KB drawings of the pages are whole: the syntax pages'
`kb` drawings, `01-instance-of-instance`, the guide's `07-zebra-fragment` and `05-jack-*`.

**Three of them against the binary**, each a page's copy against the whole view the command writes, by
[`tools/cut.py`](tools/cut.py): every line of the page is a line of the whole, in its order, and what is left out is
whole facts.

| page drawing | program | the whole view's facts | kept | left out |
|---|---|---|---|---|
| `reasoning/…/03-soup-saturated` (folded) | [`programs/soup.ein`](programs/soup.ein) | 17 | 9: `is-a` 5, `seats` 2, `allergic`, `meal` | 8, the four declarations' two facts each ([`runs/soup.cut.txt`](runs/soup.cut.txt)) |
| `04-stdlib/…/04-elimination-state` (folded) | [`programs/elimination.ein`](programs/elimination.ein) | 39 | 21: `is-a` 6, `wears` 9, `not` 6 | 18: the declarations' 4, `(bijective wears)`, `(bijection-hierarchy is-a)` and the twelve activators and markers the setup derives ([`runs/elimination.cut.txt`](runs/elimination.cut.txt)) |
| the guide's `02-transitive-fired` (canonical) | [`programs/taxonomy.ein`](programs/taxonomy.ein), a copy of `examples/saturation/transitive/taxonomy.ein` | 6 | 3, the facts of `is-a` | 3: the declaration's 2, `(transitive is-a)` ([`runs/taxonomy.cut.txt`](runs/taxonomy.cut.txt)) |

**The bijection example, whole and cut.** The whole view under `--fold`, laid out by `fdp` as its DOT suggests: of
the twelve facts the setup derives about `wears`, the eight activators of three arrows, `(std.algebra.total wears
is-a)` and the like, fold into eight arrows from `wears` to `is-a` whose labels, the relation's full name and `by
std.bijection.bijective-setup`, lie on one another; the four markers of two arrows, `(std.algebra.total wears)` and
the like, stay octagons:

![The bijection example's whole knowledge base, folded, by fdp: the facts of wears and not at the left, and between the boxes wears and is-a a fan of dashed arrows whose labels overlap into an unreadable band](renders/elimination.kb.fold.svg)

The page's cut, laid out by `dot` as the page draws it, the facts of `is-a`, `wears` and `not`:

![The page's cut: the is-a facts as arrows up to Agent and Colour, the wears facts, two dashed by domain-elimination, and six stored negations, each an octagon pointing at a wears fact in the frame not ctx](renders/elimination.kb.on-the-page.svg)

Canonical, the whole view holds 39 octagons ([`renders/elimination.kb.svg`](renders/elimination.kb.svg)). Graphviz
takes a tenth of a second or less for each drawing here ([`runs/draw-times.tsv`](runs/draw-times.tsv)).

## 3. The options

- **(a) a KB drawing may be a cut of the view**, the page saying what it leaves out; a whole view kept as written
  (the default). What the pages hold today.
- **(b) only whole views, however large.** The 13 drawn again whole: for the three above, the page would show
  [`renders/soup.kb.fold.svg`](renders/soup.kb.fold.svg), [`renders/elimination.kb.fold.svg`](renders/elimination.kb.fold.svg)
  and [`renders/taxonomy.kb.svg`](renders/taxonomy.kb.svg), the binary's own output (no mock is needed); the
  bijection state would carry the twelve setup facts and their labels above, and every declaration's two facts would
  stand beside every state. No code changes either way.

A third way, read, not run: a view that takes the relations to draw (`ein render kb --only is-a,wears,not`), so that
a page's cut is a command's whole output. That is a code change, and no question has put it.

## 4. The default

**(a)**, taken by task 2: *"A KB drawing may be a cut of the view … the page says what is left out … Q-M1f.368's
*a drawing of the engine's is kept as the file the command wrote* is read as: a whole view is kept as written; a cut
is stated on its page."*

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing, from the repo root |
| [`tools/cuts.py`](tools/cuts.py) | the 13 cut drawings, each with its page's sentence |
| [`tools/cut.py`](tools/cut.py) | a page's drawing against the whole view: in order, and the facts kept and left out by relation |
| [`programs/soup.ein`](programs/soup.ein) | from `ein.rs/08-reasoning-in-code/02-saturation-and-negation.md` § 3.4 |
| [`programs/elimination.ein`](programs/elimination.ein) | from `user/04-reference/04-stdlib/04-bijections.md` § Root decides |
| [`programs/taxonomy.ein`](programs/taxonomy.ein) | a copy of `examples/saturation/transitive/taxonomy.ein`, the guide's chapter 2 program |
| [`runs/cut-drawings.md`](runs/cut-drawings.md) | the 13, a table |
| [`runs/soup.cut.txt`](runs/soup.cut.txt), [`runs/elimination.cut.txt`](runs/elimination.cut.txt), [`runs/taxonomy.cut.txt`](runs/taxonomy.cut.txt) | each cut against its whole view |
| [`runs/elimination.solve.txt`](runs/elimination.solve.txt), [`runs/taxonomy.solve.txt`](runs/taxonomy.solve.txt) | `ein solve` of the two |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt), [`runs/draw-times.tsv`](runs/draw-times.tsv) | the exit codes; Graphviz's layout and seconds |
| [`renders/soup.kb.dot`](renders/soup.kb.dot) / [`.svg`](renders/soup.kb.svg), [`renders/soup.kb.fold.dot`](renders/soup.kb.fold.dot) / [`.svg`](renders/soup.kb.fold.svg), [`renders/soup.kb.on-the-page.dot`](renders/soup.kb.on-the-page.dot) / [`.svg`](renders/soup.kb.on-the-page.svg) | the soup state: whole canonical, whole folded, the page's cut |
| [`renders/elimination.kb.dot`](renders/elimination.kb.dot) / [`.svg`](renders/elimination.kb.svg), [`renders/elimination.kb.fold.dot`](renders/elimination.kb.fold.dot) / [`.svg`](renders/elimination.kb.fold.svg), [`renders/elimination.kb.on-the-page.dot`](renders/elimination.kb.on-the-page.dot) / [`.svg`](renders/elimination.kb.on-the-page.svg) | the bijection state, the same three |
| [`renders/taxonomy.kb.dot`](renders/taxonomy.kb.dot) / [`.svg`](renders/taxonomy.kb.svg), [`renders/taxonomy.kb.fold.dot`](renders/taxonomy.kb.fold.dot) / [`.svg`](renders/taxonomy.kb.fold.svg), [`renders/taxonomy.kb.on-the-page.dot`](renders/taxonomy.kb.on-the-page.dot) / [`.svg`](renders/taxonomy.kb.on-the-page.svg) | the guide's taxonomy, the same three |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/48-Q-M1f.468/rerun.sh`.

## 7. Found while building

- **Folded facts of one pair of atoms lie on one another.** In the bijection example's whole view, eight facts of
  three arrows join `wears` to `is-a`; folded, they are eight arrows between the same two boxes, and their labels,
  the relations' full names with `by std.bijection.bijective-setup`, overlap into one band no one can read. The cut
  leaves them out, which is one reason a page cuts. (Canonical, each is an octagon of its own, and readable.)
- The pages' cuts are drawn by `dot` (a page's copy keeps no `// suggested layout: fdp`): the bijection cut is a strip
  whose rule notes overlap at its foot (R-18, item [55](../55-R-18/README.md)).
- Every cut checked here is exact: the page's lines are the binary's, in order, with whole facts left out, and the
  page names what is left out.

## 8. Your call

- **(a)** a cut, its page saying what it leaves out (default).
- **(b)** whole views only.
