# L-97 — nine `div9-step` rules have no drawing on the reference page

## 1. The question

The reference pages under `docs/ein/user/04-reference/04-stdlib/` show each standard-library rule as its Ein text
and as the engine's drawing of it. Both are generated (`utils/gen_stdlib_rule_pages.py`), and the generator bounds
a drawing at `MAX_NODES = 120` nodes. Since every object is drawn once (Q-M1f.395), a pattern under an arrow, the
`(row9 ?pc ?p8 …)` inside `(multiples9 …)` inside `(div9-at …)`, is its own octagon with an arrow to each of its
variables, and the drawings of `div9-step-1` to `div9-step-9` of `std.math.num9` passed the bound: the nine are shown
as Ein alone. Keep the bound, or change it so they come back?

Since the first look the canonical form is the default (Q-M1f.419), and the generator falls back to the folded
drawing where the canonical one is past the bound and the folded one is not (Q-M1f.434, item
[45](../45-Q-M1f.434/README.md)). **That fallback does not bring the nine back**, so the question stands.

Recorded: [`../../findings.md`](../../../findings.md#the-users-pages-and-the-standard-librarys), row **L-97** (P1f.22's
N-25), *with L-77*.

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; `ein-head-b5fb8d70` is the first look's (labelled octagons).
The generator renders a module with `ein render rules` and `ein render rules --fold`, splits each output at each
`digraph`, and counts a node as a line matching `\s+"[^"]*" \[` (its function `nodes`); a rule is drawn canonical
where that count is at most 120, else folded where the folded count is, else shown as Ein alone (its function
`chosen`). [`tools/count.py`](tools/count.py) calls the generator's own functions on every module.

- [`runs/num9.render-rules.dot`](runs/num9.render-rules.dot), [`runs/num9.render-rules.fold.dot`](runs/num9.render-rules.fold.dot):
  the raw outputs, 48 digraphs each, one a rule.
- [`runs/stdlib-all-rules.nodes.tsv`](runs/stdlib-all-rules.nodes.tsv): every rule of the library, its canonical and
  folded counts, what the page shows; [`runs/stdlib-all-rules.summary.txt`](runs/stdlib-all-rules.summary.txt):
  `MAX_NODES 120; 511 rules; canonical 493, folded 1, Ein only 17`.
- The page's source `17-std.math.num9.div9-step-0.dot` is the folded digraph the binary prints for the rule
  ([`runs/div9-step-0.page-source-vs-binary-fold.diff`](runs/div9-step-0.page-source-vs-binary-fold.diff): empty, exit 0).

The bound, the folder and the generator's check ([`runs/generator-bound-and-check.txt`](runs/generator-bound-and-check.txt)):

<!-- output -->
```text
$ grep -n "^MAX_NODES" utils/gen_stdlib_rule_pages.py
73:MAX_NODES = 120

$ ls docs/ein/user/04-reference/04-stdlib/diagrams/ | grep -E 'div9-step|mul9-step'
17-std.math.num9.div9-step-0.dot
17-std.math.num9.div9-step-0.svg
17-std.math.num9.mul9-step-zero.dot
17-std.math.num9.mul9-step-zero.svg

$ python3 utils/gen_stdlib_rule_pages.py --check --bin build/review-bin/ein-head-17085b27
gen_stdlib_rule_pages: 511 rules of 24 modules (493 drawn canonical, 1 folded, 17 as Ein only), their regions and the 494 sources of their drawings are as written
exit 0
```

The drawings, counted ([`runs/node-counts.txt`](runs/node-counts.txt), [`tools/shapes.py`](tools/shapes.py)):

| drawing | nodes | diamonds (variables) | octagons | boxes | match / assert panel | on the page |
|---|---|---|---|---|---|---|
| `div9-step-0` canonical | **122** | 93 | 20 | 9 | 77 / 45 | — |
| `div9-step-0` folded | **120** | 93 | 19 | 8 | 75 / 45 | **drawn, folded** |
| `div9-step-1` canonical | **194** | 146 | 34 | 14 | 127 / 67 | Ein only |
| `div9-step-1` folded | **192** | 146 | 33 | 13 | 125 / 67 | |
| `div9-step-1` overlay (`--rule-mode overlay`) | 131 | 92 | 31 | 8 | one panel | |
| `mul9-step` canonical | **129** | 91 | 28 | 10 | 94 / 35 | Ein only |
| `mul9-step` folded | **127** | 91 | 27 | 9 | 92 / 35 | |
| `mul9-step` overlay | 97 | 63 | 27 | 7 | one panel | |
| `div9-step-0` at the first look (b5fb8d70) | 115 | 93 | 19 | 3 `rectangle` | 72 / 43 | drawn then |
| `div9-step-1` at the first look | 183 | 146 | 33 | 4 `rectangle` | 120 / 63 | Ein only then |
| `mul9-step` at the first look | 120 | 91 | 27 | 2 `rectangle` | 87 / 33 | drawn then, at the bound |
| `div9-step-1` before every object once (`811846ce^`) | 102 | 69 | 21 | 12 `rectangle` | 94 / 8 | drawn then |

**Why the fold does not help here.** In the canonical form a pattern's relation is a box in its panel, one box a
relation a panel, and folding a pattern of three arrows takes away only a box no other pattern of the panel still
points at: `div9-step-1` has one pattern of three arrows, `(adj ?j ?k)`, so its folded drawing is two nodes smaller
(the octagon and the box `adj`). Its other patterns are facts of `div9-at`, `num9`, `multiples9`, `row9` and the like,
of many arrows, which no fold touches; its 146 variables are the bulk. `div9-step-0` is folded onto the bound exactly (120).

- [`renders/div9-step-1.render-rule.svg`](renders/div9-step-1.render-rule.svg) ([dot](renders/div9-step-1.render-rule.dot))
  — canonical, 194 nodes, the drawing the page does not show (14 045 × 328 pt); and
  [folded](renders/div9-step-1.render-rule.fold.svg) ([dot](renders/div9-step-1.render-rule.fold.dot)), 192 nodes.
- [`renders/div9-step-0.render-rule.fold.svg`](renders/div9-step-0.render-rule.fold.svg) — 120 nodes, the drawing the
  page shows (8 996 × 305 pt); [canonical](renders/div9-step-0.render-rule.svg), 122.
- [`renders/mul9-step.render-rule.svg`](renders/mul9-step.render-rule.svg) and
  [folded](renders/mul9-step.render-rule.fold.svg): 129 and 127, **newly Ein only** since the first look.
- The overlay ([`renders/div9-step-1.render-rule-overlay.svg`](renders/div9-step-1.render-rule-overlay.svg), 131;
  [`mul9-step`](renders/mul9-step.render-rule-overlay.svg), 97): it would bring `mul9-step` under the bound and not
  the nine.
- The first look's drawings, labelled octagons: [`renders/div9-step-1.render-rule.at-b5fb8d70.svg`](renders/div9-step-1.render-rule.at-b5fb8d70.svg),
  [`div9-step-0`](renders/div9-step-0.render-rule.at-b5fb8d70.svg), [`mul9-step`](renders/mul9-step.render-rule.at-b5fb8d70.svg);
  before every object was drawn once: [`renders/div9-step-1.before-every-object-once.svg`](renders/div9-step-1.before-every-object-once.svg),
  [`div9-step-0`](renders/div9-step-0.before-every-object-once.svg).

![div9-step-1 as ein render rule draws it today: a strip of 194 nodes, the match panel beside the assert panel, each pattern an unlabelled octagon whose arrow 1 is at its relation's box](renders/div9-step-1.render-rule.svg)

![div9-step-0 folded, as the page shows it: 120 nodes](renders/div9-step-0.render-rule.fold.svg)

**The page.** `17-nine-digit-numbers-rules.md` § Division (copied:
[`runs/page-17-division-section.md`](runs/page-17-division-section.md)) shows `div9-step-0` folded, and says why:
*"`div9-step-0` is drawn folded, each pattern of three arrows one labelled arrow: its canonical drawing has more
nodes than a page can show, and its folded one does not."* It says why `div9-start` is Ein alone, and **still says
nothing of `div9-step-1` … `-9`**, each a heading, a marker, a ` ```lisp ` block and the closing marker
([`runs/page-region-div9-step-1.today.md`](runs/page-region-div9-step-1.today.md)). § Multiplication now says
*"`mul9-step` is shown as Ein alone: its drawing, folded or not, has more nodes than a page can show"*
([`runs/page-region-mul9-step.today.md`](runs/page-region-mul9-step.today.md));
[`runs/page-region-div9-step-0.today.md`](runs/page-region-div9-step-0.today.md) is the folded one's region.

**Every rule of the library past the bound** ([`runs/stdlib-all-rules.nodes.tsv`](runs/stdlib-all-rules.nodes.tsv)),
17, against 16 at the first look:

| canonical | folded | rule | kind |
|---|---|---|---|
| 367 | 365 | `std.math.num9` `div9-start` | a rule |
| 273 | 273 | `std.math.int` `int-arith` | closing rule |
| 241 | 241 | `std.math.rat` `rat-arith` | closing rule |
| 235 | 235 | `std.math.decimal-arith` `decimal-arith` | closing rule |
| 194 × 9 | 192 × 9 | `std.math.num9` `div9-step-1` … `div9-step-9` | rules — **this row** |
| 161 | 161 | `std.math.binary-arith` `binary-arith` | closing rule |
| 157 | 157 | `std.math.num9` `arith9` | closing rule |
| 129 | 129 | `std.bool.props` `props` | closing rule |
| 129 | 127 | `std.math.num9` `mul9-step` | a rule, **new** |

and `div9-step-0`, 122 canonical, is drawn folded at 120.

## 3. The options

### Keep the bound (the default)

`MAX_NODES = 120`; the nine stay Ein only, and `mul9-step` with them. Nothing changes.

### Raise the bound

`MAX_NODES` to 194 or more brings the nine drawings back, canonical, and `mul9-step` (129) with them. Any bound that
admits 194 also admits three closing rules, `props` (129), `arith9` (157) and `binary-arith` (161), which the
generator's docstring and the library README (*"for a closing rule, which names every relation and rule of its module
and whose drawing is a few hundred nodes, the rule is shown as Ein alone"*) mean to keep out; at 235 or more,
`decimal-arith` too. So raising it alone draws 14 more pictures, three of them closing rules. A page region would gain
one image line each — **a mock**:
[`runs/page-region-div9-step-1.bound-raised.MOCK.md`](runs/page-region-div9-step-1.bound-raised.MOCK.md)
([diff](runs/page-region-div9-step-1.today-vs-MOCK.diff)): today's region with the image line the generator writes
for a canonical drawing added by hand; the picture would be
[`renders/div9-step-1.render-rule.svg`](renders/div9-step-1.render-rule.svg).

### Raise it and leave out closing rules by what they are (read, not run)

The generator could leave out a closing rule by being one (the rule a module's entry names) rather than by its size,
and then a bound of 194 or more draws the nine and `mul9-step` and none of the closing rules. That is a change to the
generator alone; the engine and the drawings are as they are.

## 4. The default

**The bound stays** (the row: *"the bound is the generator's (`MAX_NODES`), and stays by default"*). The number
120 is the generator's, not a decision's. What the decision says, Q-M1f.375 (read, not run):

> **A rule that is listed is shown**: as Ein, the rule as its module writes it, and as the drawing `ein render
> rule` writes for it. A rule whose drawing is too large to read, a closing rule that names every relation and rule
> of its module, is shown as Ein alone.

It names the closing rule as *the* case of a drawing too large to read; the nine and `mul9-step` are not closing
rules. With L-77: the nine were already the library's widest drawings before the canonical form (11 051 pt), and
today's `div9-step-1` is 14 045 pt.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing below, from the repo root |
| [`tools/count.py`](tools/count.py) | every rule of the library: canonical and folded counts, what the page shows, by the generator's functions |
| [`tools/shapes.py`](tools/shapes.py) | a drawing's nodes by shape and by panel |
| [`renders/div9-step-1.render-rule.dot`](renders/div9-step-1.render-rule.dot) / [`.svg`](renders/div9-step-1.render-rule.svg) | `ein render rule --name div9-step-1 stdlib/math/num9.ein`, drawn |
| [`renders/div9-step-1.render-rule.fold.dot`](renders/div9-step-1.render-rule.fold.dot) / [`.svg`](renders/div9-step-1.render-rule.fold.svg) | the same, `--fold` |
| [`renders/div9-step-1.render-rule-overlay.dot`](renders/div9-step-1.render-rule-overlay.dot) / [`.svg`](renders/div9-step-1.render-rule-overlay.svg) | `--rule-mode overlay`, 131 nodes |
| [`renders/div9-step-0.render-rule.dot`](renders/div9-step-0.render-rule.dot) / [`.svg`](renders/div9-step-0.render-rule.svg) | `div9-step-0`, canonical, 122 |
| [`renders/div9-step-0.render-rule.fold.dot`](renders/div9-step-0.render-rule.fold.dot) / [`.svg`](renders/div9-step-0.render-rule.fold.svg) | `div9-step-0`, folded, 120: the page's |
| [`renders/div9-step-0.render-rule-overlay.dot`](renders/div9-step-0.render-rule-overlay.dot) / [`.svg`](renders/div9-step-0.render-rule-overlay.svg) | `div9-step-0`, overlay, 81 |
| [`renders/mul9-step.render-rule.dot`](renders/mul9-step.render-rule.dot) / [`.svg`](renders/mul9-step.render-rule.svg) | `mul9-step`, canonical, 129 |
| [`renders/mul9-step.render-rule.fold.dot`](renders/mul9-step.render-rule.fold.dot) / [`.svg`](renders/mul9-step.render-rule.fold.svg) | `mul9-step`, folded, 127 |
| [`renders/mul9-step.render-rule-overlay.dot`](renders/mul9-step.render-rule-overlay.dot) / [`.svg`](renders/mul9-step.render-rule-overlay.svg) | `mul9-step`, overlay, 97 |
| [`renders/div9-step-1.render-rule.at-b5fb8d70.dot`](renders/div9-step-1.render-rule.at-b5fb8d70.dot) / [`.svg`](renders/div9-step-1.render-rule.at-b5fb8d70.svg), [`div9-step-0`](renders/div9-step-0.render-rule.at-b5fb8d70.svg), [`mul9-step`](renders/mul9-step.render-rule.at-b5fb8d70.svg) | the first look's binary, labelled octagons |
| [`renders/div9-step-1.before-every-object-once.dot`](renders/div9-step-1.before-every-object-once.dot) / [`.svg`](renders/div9-step-1.before-every-object-once.svg), [`div9-step-0`](renders/div9-step-0.before-every-object-once.svg) | the tracked sources at `811846ce^`, drawn |
| [`runs/num9.render-rules.dot`](runs/num9.render-rules.dot), [`runs/num9.render-rules.fold.dot`](runs/num9.render-rules.fold.dot) | `ein render rules [--fold] stdlib/math/num9.ein`, raw, as the generator reads them |
| [`runs/stdlib-all-rules.nodes.tsv`](runs/stdlib-all-rules.nodes.tsv), [`runs/stdlib-all-rules.summary.txt`](runs/stdlib-all-rules.summary.txt) | every rule of every module, largest first, and the totals |
| [`runs/node-counts.txt`](runs/node-counts.txt) | each drawing's counts by shape and by panel |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds, each drawing (all `dot`, under a tenth of a second) |
| [`runs/generator-bound-and-check.txt`](runs/generator-bound-and-check.txt) | `MAX_NODES`, the folder's files, the generator's `--check` |
| [`runs/div9-step-0.page-source-vs-binary-fold.diff`](runs/div9-step-0.page-source-vs-binary-fold.diff) | the page's source against the binary's folded digraph: no difference |
| [`runs/page-17-division-section.md`](runs/page-17-division-section.md) | the page's § Division, copied |
| [`runs/page-region-div9-step-1.today.md`](runs/page-region-div9-step-1.today.md), [`runs/page-region-div9-step-0.today.md`](runs/page-region-div9-step-0.today.md), [`runs/page-region-mul9-step.today.md`](runs/page-region-mul9-step.today.md) | three regions as they are |
| [`runs/page-region-div9-step-1.bound-raised.MOCK.md`](runs/page-region-div9-step-1.bound-raised.MOCK.md) | the region with its image line (mock) |
| [`runs/page-region-div9-step-1.today-vs-MOCK.diff`](runs/page-region-div9-step-1.today-vs-MOCK.diff) | their diff |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/25-L-97/rerun.sh`. It writes
every file above but the mock, which it diffs; it runs `python3 utils/gen_stdlib_rule_pages.py --check`, which writes
nothing.

## 7. Found while building

- **The fold does not save the nine**: two nodes a rule (194 to 192), since what makes them large is their
  variables and their facts of many arrows. Q-M1f.434's fallback draws one rule of the library,
  `div9-step-0`, which lands on the bound exactly (120); a rule one node larger would be Ein only.
- **One more rule lost its drawing**: `mul9-step`, 120 at the first look (at the bound), is 129 canonical and 127
  folded today: the canonical form's relation boxes, one a relation a panel, push it past. The page says so; the
  nine still have no sentence (as at the first look).
- The overlay would draw `mul9-step` (97) and not the nine (131).
- Since the first look (b5fb8d70): the counts above moved (`div9-step-1` 183 → 194, `div9-step-0` 115 → 122
  canonical, drawn folded at 120, `mul9-step` 120 → 129); every drawing is the canonical form; the generator's line
  now counts the three ways; the `MAX_NODES` line moved from 70 to 73.
- Q-M1f.375's text reads *too large to read* as *a closing rule*; the generator reads it as *more than 120 nodes*,
  canonical and folded. Eleven rules that are not closing rules are Ein only today (`div9-start`, the nine,
  `mul9-step`).

## 8. Your call

- **Keep `MAX_NODES = 120`** (default): the nine stay Ein only, and `mul9-step` with them.
- **Raise it to ≥ 194**: the nine and `mul9-step` are drawn, and `props`, `arith9`, `binary-arith` with them.
- **Raise it and leave closing rules out by kind**: the nine and `mul9-step` are drawn, no closing rule is.
