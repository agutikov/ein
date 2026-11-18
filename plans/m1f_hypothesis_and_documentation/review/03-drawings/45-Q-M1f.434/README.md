# Q-M1f.434 — the standard library's rule drawings past the bound

## 1. The question

The reference pages under `docs/ein/user/04-reference/04-stdlib/` show each rule of the standard library as its Ein
text and as `ein render rule`'s drawing of it, both written by a generator, `utils/gen_stdlib_rule_pages.py`. A
drawing of more than `MAX_NODES = 120` nodes is not shown. With every fact drawn canonical, each pattern has a box for
its relation, and some drawings passed the bound. The build took a fallback: **the canonical drawing where it is
within the bound, else the folded one (`--fold`) where that is, else Ein only**, the image's text saying a drawing is
folded. The other reading, (b), is **the canonical drawing or Ein only**, never a folded one.

Recorded: [`open_questions.md` § Q-M1f.434](../../../open_questions.md#q-m1f434--the-standard-librarys-rule-drawings-past-the-bound).
The nine `div9-step` rules that are Ein only either way are L-97's question, item
[25](../25-L-97/README.md); this item is the fallback alone.

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; *before* is `build/review-bin/ein-head-b5fb8d70`, the binary
before the stage (the standard library is the same in both). The runs read the modules' own files under `stdlib/`, as the generator does, so
this item has no `programs/`.

**What the generator chose**, rule by rule, by its own functions (`modules`, `rules_of`, `drawings_of`, `nodes`,
`chosen`, `drawn`, imported by [`tools/census.py`](tools/census.py), which writes nothing else):
[`runs/rules.tsv`](runs/rules.tsv), and [`runs/census.txt`](runs/census.txt):

<!-- output -->
```text
511 rules: 493 canonical, 1 folded, 17 as Ein only (MAX_NODES = 120)
past the bound canonical: 18 · of them within it folded: 1
```

The generator's own check agrees and writes nothing ([`runs/generator-check.txt`](runs/generator-check.txt)):
*511 rules of 24 modules (493 drawn canonical, 1 folded, 17 as Ein only), their regions and the 494 sources of their
drawings are as written*.

**The rules past the bound** (from [`runs/rules.tsv`](runs/rules.tsv)):

| nodes canonical | nodes folded | rule | the page shows |
|---:|---:|---|---|
| 367 | 365 | `std.math.num9` `div9-start` | Ein only |
| 273 | 273 | `std.math.int` `int-arith` (closing rule) | Ein only |
| 241 | 241 | `std.math.rat` `rat-arith` (closing rule) | Ein only |
| 235 | 235 | `std.math.decimal-arith` `decimal-arith` (closing rule) | Ein only |
| 194 | 192 | `std.math.num9` `div9-step-1` … `div9-step-9` (nine) | Ein only (item 25) |
| 161 | 161 | `std.math.binary-arith` `binary-arith` (closing rule) | Ein only |
| 157 | 157 | `std.math.num9` `arith9` (closing rule) | Ein only |
| 129 | 129 | `std.bool.props` `props` (closing rule) | Ein only |
| **129** | **127** | `std.math.num9` **`mul9-step`** | Ein only |
| **122** | **120** | `std.math.num9` **`div9-step-0`** | **folded** |

**The fold saves little in these drawings.** It folds a pattern of three arrows that nothing points at; the rules
past the bound are closing rules, which fold nothing, and rules whose patterns have more than three arrows
(`(num9 …)` ten, `(fs …)` six) nested in one another, so it saves 0 to 2 nodes each. Across the
library it saves 2.2 nodes on the mean, 16 at most. `div9-step-0` is folded by one pattern, `(adj ?j ?k)`, which
becomes an arrow `?j → ?k` labelled `adj` (2 nodes: its octagon and the box `adj`), and lands on the bound exactly,
120. `mul9-step` saves 2 and stays past it, 127.

**`div9-step-0`, both ways** (each about 9 000 pt wide and 300 high, a strip; the page shows the folded one):

![ein render rule --fold --name div9-step-0 of std.math.num9: a strip of 120 nodes, the match cluster above the assert cluster, octagons of div9-at, num9 and fs with numbered arrows to diamonds; the one folded pattern, adj, an arrow from ?j to ?k labelled adj](renders/div9-step-0.fold.svg)

- [`renders/div9-step-0.fold.dot`](renders/div9-step-0.fold.dot) / [`.svg`](renders/div9-step-0.fold.svg): the
  page's drawing; the page's source `17-std.math.num9.div9-step-0.dot` is this output byte for byte
  ([`runs/div9-step-0.page-source-vs-fold.diff`](runs/div9-step-0.page-source-vs-fold.diff), empty);
- [`renders/div9-step-0.canonical.dot`](renders/div9-step-0.canonical.dot) / [`.svg`](renders/div9-step-0.canonical.svg):
  122 nodes, past the bound by 2;
- [`renders/div9-step-0.before-b5fb8d70.dot`](renders/div9-step-0.before-b5fb8d70.dot) / [`.svg`](renders/div9-step-0.before-b5fb8d70.svg):
  before the stage, 115 nodes, drawn.

**`mul9-step`**: [`renders/mul9-step.canonical.svg`](renders/mul9-step.canonical.svg) (129),
[`renders/mul9-step.fold.svg`](renders/mul9-step.fold.svg) (127), before the stage
[`renders/mul9-step.before-b5fb8d70.svg`](renders/mul9-step.before-b5fb8d70.svg) (120, exactly the bound, drawn). It
lost its drawing with the canonical form, folded or not.

**The pages' words**: the num9 page's § Multiplication says *`mul9-step` is shown as Ein alone: its drawing, folded
or not, has more nodes than a page can show* ([`runs/page-17.multiplication-intro.md`](runs/page-17.multiplication-intro.md));
§ Division says *`div9-step-0` is drawn folded … its canonical drawing has more nodes than a page can show, and its
folded one does not* ([`runs/page-17.division-intro.md`](runs/page-17.division-intro.md)); the library README's
§ How a rule is shown states the fallback ([`runs/page-04-stdlib-README.how-a-rule-is-shown.md`](runs/page-04-stdlib-README.how-a-rule-is-shown.md)).
`div9-step-0`'s region today: [`runs/page-17.region-div9-step-0.today.md`](runs/page-17.region-div9-step-0.today.md),
its image's text *as the engine draws it folded, each pattern of three arrows one labelled arrow*.

## 3. The options

### (a) Canonical, else folded, else Ein only (the default)

§ 2: one rule, `div9-step-0`, is drawn folded; the page and the README say so.

### (b) Canonical or Ein only

`div9-step-0` joins the seventeen as Ein only: 493 canonical, 0 folded, 18 as Ein only, and every drawing on the
pages is in one form. **Mocks** of the page as (b) would write it:
[`runs/page-17.region-div9-step-0.as-ein-only.MOCK.md`](runs/page-17.region-div9-step-0.as-ein-only.MOCK.md), the
region with its image line taken out ([diff](runs/page-17.region-div9-step-0.today-vs-MOCK.diff)), and
[`runs/page-17.division-intro.as-ein-only.MOCK.md`](runs/page-17.division-intro.as-ein-only.MOCK.md), § Division's
sentence on `div9-step-0` replaced by *each is shown as Ein alone*; the README's sentence on the folded fallback
would go too, and the generator would lose its `--fold` run. Nothing else is edited.

## 4. The default

**(a)** (the ledger), built in `utils/gen_stdlib_rule_pages.py`' `chosen`; the counts are S1f.23.1's Record and
reproduce here (493, 1, 17).

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing below, from the repo root |
| [`tools/census.py`](tools/census.py) | the generator's choice, rule by rule, by its own functions |
| [`runs/rules.tsv`](runs/rules.tsv) | every rule: module, rule, nodes canonical, nodes folded, what the page shows |
| [`runs/census.txt`](runs/census.txt) | the counts |
| [`runs/generator-check.txt`](runs/generator-check.txt) | `gen_stdlib_rule_pages.py --check`: as written (`--missing` printed nothing: every rule is on a page) |
| [`runs/max-nodes.txt`](runs/max-nodes.txt) | the bound, `MAX_NODES = 120` |
| `renders/div9-step-0.{canonical,fold,before-b5fb8d70}.dot` / `.svg` ([canonical](renders/div9-step-0.canonical.svg), [fold](renders/div9-step-0.fold.svg), [before](renders/div9-step-0.before-b5fb8d70.svg)) | `ein render rule --name div9-step-0 stdlib/math/num9.ein`, both forms, and before |
| `renders/mul9-step.{canonical,fold,before-b5fb8d70}.dot` / `.svg` ([canonical](renders/mul9-step.canonical.svg), [fold](renders/mul9-step.fold.svg), [before](renders/mul9-step.before-b5fb8d70.svg)) | the same for `mul9-step` |
| [`runs/div9-step-0.page-source-vs-fold.diff`](runs/div9-step-0.page-source-vs-fold.diff) | the page's source against `--fold`'s output: empty |
| [`runs/page-17.multiplication-intro.md`](runs/page-17.multiplication-intro.md), [`runs/page-17.division-intro.md`](runs/page-17.division-intro.md) | the num9 page's two sections' openings, copied |
| [`runs/page-17.region-div9-step-0.today.md`](runs/page-17.region-div9-step-0.today.md) | `div9-step-0`'s region, copied |
| [`runs/page-04-stdlib-README.how-a-rule-is-shown.md`](runs/page-04-stdlib-README.how-a-rule-is-shown.md) | the library README's § How a rule is shown, copied |
| [`runs/page-17.region-div9-step-0.as-ein-only.MOCK.md`](runs/page-17.region-div9-step-0.as-ein-only.MOCK.md), [`runs/page-17.division-intro.as-ein-only.MOCK.md`](runs/page-17.division-intro.as-ein-only.MOCK.md) | option (b), mocks |
| [`runs/page-17.region-div9-step-0.today-vs-MOCK.diff`](runs/page-17.region-div9-step-0.today-vs-MOCK.diff) | the region against its mock |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | each drawing's layout and Graphviz's seconds |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | each command's exit code |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/45-Q-M1f.434/rerun.sh
```

It runs the census and the generator's `--check` and `--missing` with `build/review-bin/ein-head-17085b27`,
`ein render rule` on the two rules (both forms, and with `build/review-bin/ein-head-b5fb8d70`), diffs the page's
source, copies the page's sections and draws every `.dot` with `dot`.

## 7. Found while building

- **The fallback rescues one drawing by its last two nodes**: `div9-step-0` is 122 canonical and 120 folded, the
  bound being 120; one pattern folds. A rule a node larger, or a bound a node smaller, and the fallback draws nothing.
- **`mul9-step` lost its drawing with the canonical form** (120 before, exactly the bound; 129 now, 127 folded); the
  page says so.
- **The counts L-97 (item 25) quoted moved**: `div9-step-1` … `-9` are 194 nodes today (183 at the first look's
  binary, `b5fb8d70`), `div9-step-0` 122 (115), `div9-start` 367 (359); `decimal-arith` and `binary-arith` one node more, the other closing rules as
  they were.
- Every drawing past the bound is a strip some 9 000 pt wide (`div9-step-0` 8 996 × 305 pt folded): within the bound
  is not the same as readable on a page.

## 8. Your call

- **(a) Canonical, else folded, else Ein only** (default): `div9-step-0` drawn folded.
- **(b) Canonical or Ein only**: `div9-step-0` as Ein only, one form on every page.
