# Q-M1f.435 — `xlabel`, not `label`: where an arrow's number and a folded arrow's relation are written

## 1. The question

The user asked that both the KB and the program view *"follow the same label="owns", and number the arrows 1/2"*.
In DOT an arrow can carry its text two ways: as its **`label`**, which the `dot` layout places as one more node on
the arrow before laying the graph out, or as its **`xlabel`**, which Graphviz places beside the arrow after the
layout. The build writes **`xlabel`** for every arrow's number, in every drawing of facts, and for a folded arrow's
relation in a drawing of a knowledge base (the KB view, the program view's `kb`, the constraint view): `xlabel="1"`,
`xlabel="owns"`. A rule's panel and a trace's slice, which are small, keep a folded arrow's relation as its
`label`. The reason is the layout's time: one `label` in a digraph makes `dot` lay every arrow over two ranks, and a
knowledge base of some hundreds of facts then takes minutes. So the user's literal `label="owns"` is read as *the
word written on the arrow is `owns`, not `(owns)`*. The other reading, (b), writes `label`, as the reasoning pages'
hand drawings do, with the gate's DOT sweep slower by the large views.

Recorded: [`open_questions.md` § Q-M1f.435](../../../open_questions.md#q-m1f435--where-an-arrows-number-is-written);
the numbers themselves are [§ Q-M1f.423](../../../open_questions.md#q-m1f423--how-the-arrows-of-a-fact-are-numbered).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; Graphviz 16.0.0. Two programs:
[`programs/small.ein`](programs/small.ein), the drawings' shared small program, and
[`programs/long_numbers.ein`](programs/long_numbers.ein), a copy of `examples/arith/long_numbers.ein` (five requests
on numbers of 10 to 100 digits named by `let`). Each drawing is the binary's DOT and **mocks**, the same DOT with
`xlabel` turned into `label` by one `sed` line and nothing else edited:

| file | edit | what it writes as a `label` |
|---|---|---|
| `….numbers-as-label.MOCK.dot` | `sed '/->/ s/xlabel=/label=/'` on the canonical view | every arrow's number |
| `….fold.relation-as-label.MOCK.dot` | `sed '/->/ s/xlabel="\([^0-9"][^"]*\)"/label="\1"/'` on the folded view | a folded arrow's relation (its note with it), the numbers left as `xlabel` |
| `….fold.all-as-label.MOCK.dot` | `sed '/->/ s/xlabel=/label=/'` on the folded view | both |

The KB view and the program view's `kb` suggest `fdp` (`// suggested layout: fdp`), and `fdp` lays out a `label` and
an `xlabel` alike: the layout's time is the same either way. **`dot` is what the gate's DOT sweep runs**
(`dot_wellformed.rs`, on every view of every corpus program outside `tests/`, per commit), and the time is `dot`'s.
So each drawing here is drawn twice: `X.svg` by `fdp`, `X.dot-layout.svg` by `dot`.

### 2.1 The small program

Graphviz's seconds, [`runs/small.times.tsv`](runs/small.times.tsv): every one under 0.1 s, either layout, either
attribute. What differs is the picture under `dot`. The binary's, canonical, every number an `xlabel`, laid out in
two ranks of octagons under a row of boxes:

![ein render kb of small.ein laid out by dot, every number an xlabel: a row of boxes above, the octagons in two rows under them, each arrow's number beside its arrow near the arrow's end](renders/small.kb.dot-layout.svg)

The same with every number a `label` (mock), laid out by `dot`: each number a point on its arrow, the arrows longer,
the picture wider (1 301 × 230 pt against 1 092 × 182):

![Mock: the same knowledge base with every number a label, laid out by dot: each number sits in the middle of its arrow, the arrows bend around the numbers, the drawing wider and taller](renders/small.kb.numbers-as-label.MOCK.dot-layout.svg)

Folded, the binary's (the relation on a folded arrow an `xlabel`, `owns`, `friends (2)`, `claims`) and the mock with
the relation a `label`:

![ein render kb --fold of small.ein laid out by dot, each folded arrow's relation an xlabel beside it](renders/small.kb.fold.dot-layout.svg)

![Mock: the same with each folded arrow's relation a label, laid out by dot: the relation's name sits in the middle of each folded arrow, and the layout moves round it, 1 064 × 310 pt against 892 × 277](renders/small.kb.fold.relation-as-label.MOCK.dot-layout.svg)

Laid out by `fdp`, the layout the views suggest, label and xlabel give the same picture:
[`renders/small.kb.fold.svg`](renders/small.kb.fold.svg) and
[`renders/small.kb.fold.relation-as-label.MOCK.svg`](renders/small.kb.fold.relation-as-label.MOCK.svg) are 920 ×
756 pt each; the canonical pair, [`renders/small.kb.svg`](renders/small.kb.svg) and
[`renders/small.kb.numbers-as-label.MOCK.svg`](renders/small.kb.numbers-as-label.MOCK.svg), 794 × 980 pt each.

### 2.2 `long_numbers.ein`'s program view, digraph `kb`

`ein render program --fold` ([`renders/long_numbers.program.fold.dot`](renders/long_numbers.program.fold.dot)), its
digraph `kb` cut out ([`renders/long_numbers.program.fold.kb.dot`](renders/long_numbers.program.fold.kb.dot)): 240
octagons and 730 arrows, ten of them folded (five `let`, `add`, `sub`, `cmp`, `mul`, `div`); canonical,
[`renders/long_numbers.program.kb.dot`](renders/long_numbers.program.kb.dot), 250 octagons and 750 arrows
([`runs/long_numbers.counts.tsv`](runs/long_numbers.counts.tsv)). Graphviz's seconds,
[`runs/long_numbers.times.tsv`](runs/long_numbers.times.tsv), written by [`tools/time-labels.sh`](tools/time-labels.sh):

| drawing of the digraph `kb` | arrows written as `label` | `dot` | `fdp` |
|---|---:|---:|---:|
| folded, as the binary writes it (`xlabel`) | 0 of 730 | 45.4 s | 0.53 s |
| folded, the relation a `label` (mock) | 10 of 730 | **stopped at 2 400 s** (exit 124) | 0.51 s |
| folded, numbers and relation `label`s (mock) | 730 of 730 | 339.4 s | 0.54 s |
| canonical, as the binary writes it (`xlabel`) | 0 of 750 | 26.2 s | 0.61 s |
| canonical, the numbers `label`s (mock) | 750 of 750 | 243.3 s | 0.52 s |

**Ten labels among 720 `xlabel`s are the worst case**: the layout that writes only the folded arrows' relations as
labels did not end in 40 minutes, where the one that writes all 730 as labels ended in under six. Under `fdp` the
attribute changes neither the time nor the picture: each labelled mock's `fdp` SVG is byte for byte the binary's
(`cmp`: `long_numbers.program.fold.kb.svg` and its relation-as-label mock, `long_numbers.program.kb.svg` and its
numbers-as-label mock, and `small.kb.fold.svg` and its mock, the same bytes).

![ein render program --fold of long_numbers, digraph kb, laid out by fdp in half a second: the numerals' octagons in the frame n ctx, a knot of arrows to the digit boxes 0 to 9, n and z; the five let facts and the five requests at the lower left, the folded arrows a to b labelled add, sub and cmp on top of each other](renders/long_numbers.program.fold.kb.svg)

Laid out by `dot`, the binary's drawings are 17 430 × 6 474 pt (folded) and 17 795 × 6 368 pt (canonical):
[`renders/long_numbers.program.fold.kb.dot-layout.svg`](renders/long_numbers.program.fold.kb.dot-layout.svg),
[`renders/long_numbers.program.kb.dot-layout.svg`](renders/long_numbers.program.kb.dot-layout.svg); the labelled
mocks', [`renders/long_numbers.program.fold.kb.all-as-label.MOCK.dot-layout.svg`](renders/long_numbers.program.fold.kb.all-as-label.MOCK.dot-layout.svg)
and [`renders/long_numbers.program.kb.numbers-as-label.MOCK.dot-layout.svg`](renders/long_numbers.program.kb.numbers-as-label.MOCK.dot-layout.svg),
taller still (8 273 and 8 144 pt). At this size no layout is readable as a whole.

## 3. The options

### (a) `xlabel` (the default)

§ 2: numbers and, in a knowledge base's drawing, a folded arrow's relation are `xlabel`s; a rule's panel and a
trace's slice keep a folded arrow's `label`.

### (b) `label`, as the pages draw it

The mocks of § 2. Under `fdp` nothing changes. Under `dot`, the gate's layout, the arrows bend round their labels and
the picture grows, and a large view's layout time grows by a factor the table shows; the gate's sweep lays out each
view of every corpus program outside `tests/`, so its time grows by its largest views.

## 4. The default

**(a)** (the ledger): *the arrow's `xlabel` … in every drawing of facts … and in a drawing of a knowledge base a
folded arrow's relation too, `xlabel="owns"`: one `label` in a digraph makes `dot` lay every arrow over two ranks …
The user's *label="owns"* is read as the word written on the arrow, `owns` and not `(owns)`.* Built in
`fact_graph.rs`, `rules.rs`, `slice.rs`, `derivation.rs`.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing but the long timings, from the repo root |
| [`tools/time-labels.sh`](tools/time-labels.sh) | the long timings of long_numbers' drawings, both layouts, binary and mocks |
| [`programs/small.ein`](programs/small.ein) | the drawings' shared small program |
| [`programs/long_numbers.ein`](programs/long_numbers.ein) | a copy of `examples/arith/long_numbers.ein` |
| [`renders/small.kb.dot`](renders/small.kb.dot), [`renders/small.kb.fold.dot`](renders/small.kb.fold.dot) | `ein render kb`, both forms |
| [`renders/small.kb.numbers-as-label.MOCK.dot`](renders/small.kb.numbers-as-label.MOCK.dot), [`renders/small.kb.fold.relation-as-label.MOCK.dot`](renders/small.kb.fold.relation-as-label.MOCK.dot), [`renders/small.kb.fold.all-as-label.MOCK.dot`](renders/small.kb.fold.all-as-label.MOCK.dot) | the mocks |
| `renders/small.*.svg` ([kb](renders/small.kb.svg), [fold](renders/small.kb.fold.svg), [numbers mock](renders/small.kb.numbers-as-label.MOCK.svg), [relation mock](renders/small.kb.fold.relation-as-label.MOCK.svg), [all mock](renders/small.kb.fold.all-as-label.MOCK.svg)) | each laid out by `fdp` |
| `renders/small.*.dot-layout.svg` ([kb](renders/small.kb.dot-layout.svg), [fold](renders/small.kb.fold.dot-layout.svg), [numbers mock](renders/small.kb.numbers-as-label.MOCK.dot-layout.svg), [relation mock](renders/small.kb.fold.relation-as-label.MOCK.dot-layout.svg), [all mock](renders/small.kb.fold.all-as-label.MOCK.dot-layout.svg)) | each laid out by `dot` |
| [`renders/long_numbers.program.dot`](renders/long_numbers.program.dot), [`renders/long_numbers.program.fold.dot`](renders/long_numbers.program.fold.dot) | `ein render program`, both forms |
| [`renders/long_numbers.program.kb.dot`](renders/long_numbers.program.kb.dot), [`renders/long_numbers.program.fold.kb.dot`](renders/long_numbers.program.fold.kb.dot) | their digraph `kb`, cut out |
| [`renders/long_numbers.program.kb.numbers-as-label.MOCK.dot`](renders/long_numbers.program.kb.numbers-as-label.MOCK.dot), [`renders/long_numbers.program.fold.kb.relation-as-label.MOCK.dot`](renders/long_numbers.program.fold.kb.relation-as-label.MOCK.dot), [`renders/long_numbers.program.fold.kb.all-as-label.MOCK.dot`](renders/long_numbers.program.fold.kb.all-as-label.MOCK.dot) | the mocks |
| `renders/long_numbers.*.svg`, `renders/long_numbers.*.dot-layout.svg` (linked in § 2.2) | each laid out by `fdp` and by `dot`, where the layout ended |
| [`runs/small.times.tsv`](runs/small.times.tsv) | the small program's drawings: layout, seconds, exit |
| [`runs/long_numbers.times.tsv`](runs/long_numbers.times.tsv) and [`runs/times/`](runs/times/) | long_numbers' drawings: layout, seconds, exit (124: stopped at the timeout) |
| [`runs/long_numbers.counts.tsv`](runs/long_numbers.counts.tsv) | arrows, octagons, `label`s and `xlabel`s of each long_numbers drawing |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | each command's exit code |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/46-Q-M1f.435/rerun.sh               # the views, the small program's drawings
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/46-Q-M1f.435/tools/time-labels.sh   # long_numbers' timed layouts, each stopped at 2 400 s
```

`rerun.sh` does not call `tools/time-labels.sh`. The timings were taken on a machine shared with other runs (the gate
and the nightly of another session, and the other review items' drawings); the four labelled layouts ran side by
side, each Graphviz process on one thread.

## 7. Found while building

- **Under `fdp`, the layout the KB and program views suggest, `label` and `xlabel` draw the same bytes**, and lay
  out a knowledge base of 750 arrows in about half a second; the choice matters only to `dot`. `dot` is what the
  gate's DOT sweep runs on every view, and what `utils/render_diagrams.py` draws a page's diagram with (it does not
  read the `fdp` hint: item 55, R-18).
- **A few labels among many `xlabel`s are slower than all labels**: ten folded relations as labels among 720
  numbers as `xlabel`s did not finish in 2 400 s of `dot`; all 730 as labels took 339 s. S1f.23.1's Record saw the
  same view *past a quarter of an hour*; it is past forty minutes here. Q-M1f.435's 360 s for the canonical view
  with labels is 243 s here, and its 38 s with `xlabel`s is 26 s (another machine load; the order holds).
- Under `dot`, the binary's long_numbers drawings are 17 430 × 6 474 pt (folded) and 17 795 × 6 368 pt (canonical);
  under `fdp` 1 675 × 1 630 and 2 137 × 1 891 pt, a knot of the numerals' arrows to the ten digit boxes, the folded
  arrows `add`, `sub` and `cmp` from `a` to `b` written on top of each other. Neither reads as a whole.

## 8. Your call

- **(a) `xlabel`** (default): every number, and a folded arrow's relation in a knowledge base's drawing, beside its
  arrow; the word is `owns`.
- **(b) `label`**, as the pages draw it: the same words on the arrow, the `dot` layouts of large views slower by the
  table's factor.
