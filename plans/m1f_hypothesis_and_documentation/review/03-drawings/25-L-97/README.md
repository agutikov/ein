# L-97 — nine `div9-step` rules have no drawing on the reference page

## 1. The question

The reference pages under `docs/ein/user/04-reference/04-stdlib/` show each standard-library rule as its Ein text
and as the engine's drawing of it. Both are generated (`utils/gen_stdlib_rule_pages.py`), and the generator leaves
out the drawing of any rule whose drawing has more than `MAX_NODES = 120` nodes: such a rule is shown as Ein alone.
Since every object is drawn once (Q-M1f.395), a pattern under an arrow — `(row9 ?pc ?p8 …)` inside
`(multiples9 …)` inside `(div9-at …)` — is drawn as its own octagon with an arrow to each of its variables, where it
used to be one node labelled with its text. That pushed the drawings of `div9-step-1` to `div9-step-9` of
`std.math.num9` from 102 nodes to 183, over the bound, so the nine lost their drawings; `div9-step-0` (115) kept
its. Keep the bound, or change it so they come back?

Recorded: [`../../findings.md`](../../../findings.md#the-users-pages-and-the-standard-librarys), row **L-97** (P1f.22's
N-25), *with L-77*.

## 2. What is so today

Today's binary, `build/review-bin/ein-head-b5fb8d70`. The generator renders a module with
`ein render rules stdlib/math/num9.ein`, splits the output at each `digraph`, and counts a node as a line matching
`\s+"[^"]*" \[` (its function `nodes`). The same here:

- [`runs/num9.render-rules.dot`](runs/num9.render-rules.dot): the raw output, 48 digraphs, one per rule.
- [`runs/num9.nodes-per-rule.tsv`](runs/num9.nodes-per-rule.tsv): the count per rule of `std.math.num9`.
- `ein render rule --name div9-step-0 stdlib/math/num9.ein` and `… div9-step-1 …` print byte for byte the digraph
  cut from `render rules` (checked); the page's tracked source `17-std.math.num9.div9-step-0.dot` is that output
  too ([`runs/div9-step-0.page-source-vs-binary.diff`](runs/div9-step-0.page-source-vs-binary.diff): empty, exit 0).

The bound and what the folder holds ([`runs/generator-bound-and-check.txt`](runs/generator-bound-and-check.txt)):

<!-- output -->
```text
$ grep -n "^MAX_NODES" utils/gen_stdlib_rule_pages.py
70:MAX_NODES = 120

$ ls docs/ein/user/04-reference/04-stdlib/diagrams/ | grep div9-step
17-std.math.num9.div9-step-0.dot
17-std.math.num9.div9-step-0.svg

$ python3 utils/gen_stdlib_rule_pages.py --check --bin build/review-bin/ein-head-b5fb8d70
gen_stdlib_rule_pages: 511 rules of 24 modules, their regions and the 495 sources of their drawings are as written
```

The two drawings, counted ([`runs/node-counts.txt`](runs/node-counts.txt)):

| drawing | nodes | diamonds (variables) | octagons | rectangles | match / assert panel | on the page |
|---|---|---|---|---|---|---|
| `div9-step-0` today | **115** | 93 | 19 | 3 | 72 / 43 | drawn |
| `div9-step-1` today | **183** | 146 | 33 | 4 | 120 / 63 | Ein only |
| `div9-step-0` before every object drawn once | 63 | 42 | 11 | 10 | 55 / 8 | drawn |
| `div9-step-1` before every object drawn once | 102 | 69 | 21 | 12 | 94 / 8 | drawn |

(*before* = the tracked sources at `811846ce^`, the commit before Q-M1f.395's build removed the nine.) The assert
panel is where it grew most: its one `div9-at` fact was 8 nodes with its nested facts as text nodes, and is 63 with
each nested fact its octagon and each variable its diamond. `div9-step-2` … `-9` are `div9-step-1`'s shape with
another digit, 183 nodes each.

- [`renders/div9-step-0.render-rule.dot`](renders/div9-step-0.render-rule.dot) /
  [`.svg`](renders/div9-step-0.render-rule.svg) — 115 nodes, the drawing the page shows (8775 × 328 pt).
- [`renders/div9-step-1.render-rule.dot`](renders/div9-step-1.render-rule.dot) /
  [`.svg`](renders/div9-step-1.render-rule.svg) — 183 nodes, the one the page does not show (13531 × 517 pt).
- [`renders/div9-step-1.before-every-object-once.dot`](renders/div9-step-1.before-every-object-once.dot) /
  [`.svg`](renders/div9-step-1.before-every-object-once.svg), and `div9-step-0`'s, the drawings as they were.
- In `--rule-mode overlay` the two are 76 and **124** nodes
  ([`renders/div9-step-1.render-rule-overlay.svg`](renders/div9-step-1.render-rule-overlay.svg)): the overlay alone
  would not bring `div9-step-1` under 120.

![div9-step-1 as ein render rule draws it: a strip of 183 nodes, the match panel above the assert panel](renders/div9-step-1.render-rule.svg)

![div9-step-0 as the page shows it: 115 nodes](renders/div9-step-0.render-rule.svg)

**The page.** `17-nine-digit-numbers-rules.md` § Division (copied:
[`runs/page-17-division-section.md`](runs/page-17-division-section.md)) shows `div9-step-0` with its image and each
of `div9-step-1` … `-9` as Ein only. Its `div9-step-1` region today
([`runs/page-region-div9-step-1.today.md`](runs/page-region-div9-step-1.today.md)) is the heading, the marker, the
` ```lisp ` block and the closing marker, with no image line. The section's opening sentence says why for
`div9-start` alone: *"`div9-start` computes the divisor's multiples by one to nine once, and is shown as Ein alone:
its drawing has more nodes than a page can show."*

**Every rule of the library, counted** ([`runs/stdlib-all-rules.nodes.tsv`](runs/stdlib-all-rules.nodes.tsv),
sorted by size): 511 rules, 16 over the bound — the generator's own 511 − 495. Over 120:

| nodes | rule | kind |
|---|---|---|
| 359 | `std.math.num9` `div9-start` | a rule (Ein only before Q-M1f.395 too) |
| 273 | `std.math.int` `int-arith` | closing rule |
| 241 | `std.math.rat` `rat-arith` | closing rule |
| 234 | `std.math.decimal-arith` `decimal-arith` | closing rule |
| 183 × 9 | `std.math.num9` `div9-step-1` … `div9-step-9` | rules — **this row** |
| 160 | `std.math.binary-arith` `binary-arith` | closing rule |
| 157 | `std.math.num9` `arith9` | closing rule |
| 129 | `std.bool.props` `props` | closing rule |

and `mul9-step` is exactly at the bound, 120, drawn.

## 3. The options

### Keep the bound (the default)

`MAX_NODES = 120`; the nine stay Ein only. Nothing changes.

### Raise the bound

`MAX_NODES` to 183 or more brings the nine drawings back. Any bound that admits 183 also admits three closing rules,
`binary-arith` (160), `arith9` (157) and `props` (129), which the generator's docstring and the library README
(*"A closing rule … is shown as Ein alone: its drawing is a few hundred nodes"*) mean to keep out; at 234 or more,
`decimal-arith` too. So raising it alone draws 12 more pictures, three of them closing rules. The page region would
gain one image line each — **a mock**:
[`runs/page-region-div9-step-1.bound-raised.MOCK.md`](runs/page-region-div9-step-1.bound-raised.MOCK.md)
([diff](runs/page-region-div9-step-1.today-vs-MOCK.diff)), the picture being
[`renders/div9-step-1.render-rule.svg`](renders/div9-step-1.render-rule.svg).

### Raise it and leave out closing rules by what they are (read, not run)

The generator could leave out a closing rule by being one (the rule a module's entry names) rather than by its size,
and then a bound of 183 or more draws the nine and none of the closing rules. That is a change to the generator
alone; the engine and the drawings are as they are.

## 4. The default

**The bound stays** (the row: *"the bound is the generator's (`MAX_NODES`), and stays by default"*). The number
120 is the generator's, not a decision's. What the decision says, Q-M1f.375 (read, not run):

> **A rule that is listed is shown**: as Ein, the rule as its module writes it, and as the drawing `ein render
> rule` writes for it. A rule whose drawing is too large to read, a closing rule that names every relation and rule
> of its module, is shown as Ein alone.

It names the closing rule as *the* case of a drawing too large to read; the nine are not closing rules. With L-77:
before Q-M1f.395's build the nine were already the library's widest drawings (11 051 pt), and today's
`div9-step-1` would be 13 531 pt.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`renders/div9-step-0.render-rule.dot`](renders/div9-step-0.render-rule.dot) / [`.svg`](renders/div9-step-0.render-rule.svg) | `ein render rule --name div9-step-0 stdlib/math/num9.ein`, drawn |
| [`renders/div9-step-1.render-rule.dot`](renders/div9-step-1.render-rule.dot) / [`.svg`](renders/div9-step-1.render-rule.svg) | the same for `div9-step-1` |
| [`renders/div9-step-0.render-rule-overlay.dot`](renders/div9-step-0.render-rule-overlay.dot) / [`.svg`](renders/div9-step-0.render-rule-overlay.svg) | `--rule-mode overlay`, 76 nodes |
| [`renders/div9-step-1.render-rule-overlay.dot`](renders/div9-step-1.render-rule-overlay.dot) / [`.svg`](renders/div9-step-1.render-rule-overlay.svg) | `--rule-mode overlay`, 124 nodes |
| [`renders/div9-step-0.before-every-object-once.dot`](renders/div9-step-0.before-every-object-once.dot) / [`.svg`](renders/div9-step-0.before-every-object-once.svg) | the tracked source at `811846ce^`, drawn |
| [`renders/div9-step-1.before-every-object-once.dot`](renders/div9-step-1.before-every-object-once.dot) / [`.svg`](renders/div9-step-1.before-every-object-once.svg) | the same for `div9-step-1` |
| [`runs/num9.render-rules.dot`](runs/num9.render-rules.dot) | `ein render rules stdlib/math/num9.ein`, raw, as the generator reads it |
| [`runs/num9.nodes-per-rule.tsv`](runs/num9.nodes-per-rule.tsv) | nodes per `std.math.num9` rule, by the generator's count |
| [`runs/stdlib-all-rules.nodes.tsv`](runs/stdlib-all-rules.nodes.tsv) | nodes per rule of every module, largest first |
| [`runs/node-counts.txt`](runs/node-counts.txt) | the two rules' counts by shape and by panel, today and before |
| [`runs/generator-bound-and-check.txt`](runs/generator-bound-and-check.txt) | `MAX_NODES`, the folder's `div9-step` files, the generator's `--check` |
| [`runs/div9-step-0.page-source-vs-binary.diff`](runs/div9-step-0.page-source-vs-binary.diff) | the page's source against the binary: no difference |
| [`runs/page-17-division-section.md`](runs/page-17-division-section.md) | the page's § Division, copied |
| [`runs/page-region-div9-step-1.today.md`](runs/page-region-div9-step-1.today.md) | the `div9-step-1` region as it is |
| [`runs/page-region-div9-step-1.bound-raised.MOCK.md`](runs/page-region-div9-step-1.bound-raised.MOCK.md) | the region with its image line (mock) |
| [`runs/page-region-div9-step-1.today-vs-MOCK.diff`](runs/page-region-div9-step-1.today-vs-MOCK.diff) | their diff |

## 6. Re-run

From the repo root, `E=build/review-bin/ein-head-b5fb8d70`, `D=plans/m1f_hypothesis_and_documentation/review/03-drawings/25-L-97`:

```sh
$E render rules stdlib/math/num9.ein > $D/runs/num9.render-rules.dot
for r in div9-step-0 div9-step-1; do
  $E render rule --name $r stdlib/math/num9.ein > $D/renders/$r.render-rule.dot
  $E render rule --name $r --rule-mode overlay stdlib/math/num9.ein > $D/renders/$r.render-rule-overlay.dot
  git show 811846ce^:docs/ein/user/04-reference/04-stdlib/diagrams/17-std.math.num9.$r.dot > $D/renders/$r.before-every-object-once.dot
done
for f in $D/renders/*.dot; do dot -Tsvg "$f" > "${f%.dot}.svg"; done
grep -c -E '^\s+"[^"]*" \[' $D/renders/div9-step-1.render-rule.dot     # 183: the generator's count
grep -n '^MAX_NODES' utils/gen_stdlib_rule_pages.py
python3 utils/gen_stdlib_rule_pages.py --check --bin $E                # writes nothing
```

## 7. Found while building

- The row reproduces: 183 nodes for each of `div9-step-1` … `-9`, 115 for `div9-step-0`, `MAX_NODES = 120`, one
  `div9-step` drawing in the folder.
- **The pages say why only for closing rules and `div9-start`.** The library README's § How a rule is shown says
  *"A closing rule … is shown as Ein alone"*, and the num9 page's § Division says so of `div9-start`; nothing on
  either says why `div9-step-1` … `-9` (not closing rules) have no drawing while `div9-step-0` has one.
- **A raised bound brings in three closing rules first**: `props` (129), `arith9` (157), `binary-arith` (160) all
  sit between 120 and 183, so the nine cannot come back by the bound alone without them.
- The overlay mode does not save it: `div9-step-1` is 124 nodes there.
- Q-M1f.375's text reads *too large to read* as *a closing rule*; the generator reads it as *more than 120 nodes*.
  `div9-start` (359) was already Ein only before the nine passed 120 (it has no source at `811846ce^`); now ten
  rules that are not closing rules (`div9-start` and the nine) are Ein only.

## 8. Your call

- **Keep `MAX_NODES = 120`** (default): the nine stay Ein only.
- **Raise it to ≥ 183**: the nine are drawn, and `props`, `arith9`, `binary-arith` with them.
- **Raise it and leave closing rules out by kind**: the nine are drawn, no closing rule is.
