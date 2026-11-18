# Q-M1f.430 — the region, lattice and h-space views, left as they were

## 1. The question

Every view that draws **facts** now draws each one in the canonical form: an octagon with no name in it, its arrow
`1` at its relation's box and `2`, `3`, … at its arguments, and no node labelled with a fact's text. Three views
draw something else: **states, nodes of the lattice and candidates**. `ein render region` draws each commitment of
the hypothesis cube as a node labelled with the facts it commits to, `(a-prop X)\n(b-prop X)`, and a state the run
reported as an **octagon** with those facts written in it; `ein render lattice` draws each node a run visited as a
rounded box with its facts' text; `ein render h-space` draws each candidate as a box or an ellipse labelled
`(seats Ann C1)`. The stage read these labels as *a list of what a state holds*, not as a drawing of a fact, and left
the three views as they were. Is that right, or are they the text nodes (form 4) the ruling forbids, so that each
state's or candidate's facts should be drawn as nodes?

Recorded: [`open_questions.md` § Q-M1f.430](../../../open_questions.md#q-m1f430--the-region-lattice-and-h-space-views).

## 2. What is so today

Today's binary, `build/review-bin/ein-head-17085b27`; Graphviz 16.0.0. Three programs: two small corpus programs with
candidates, copied, and zebra2:

- [`programs/02_genuine_3set_death.ein`](programs/02_genuine_3set_death.ein), from `examples/lattice/`: three
  candidates `(a-prop X)`, `(b-prop X)`, `(c-prop X)`, all three together dead; R is the other seven nodes;
- [`programs/05_owed_seats_bounded.ein`](programs/05_owed_seats_bounded.ein), from `examples/lattice/`: two guests,
  two chairs, four candidates `(seats … …)`, root owing four instances; R is the two seatings;
- [`programs/zebra2.ein`](programs/zebra2.ein), from `examples/`: 56 candidates, too many for the cube (`ein oracle`
  saturates $2^n$ nodes), so its lattice and its h-space are drawn, not its region.

**Nothing in these views moved with the canonical form.** The same views by `ein-head-b5fb8d70`, the binary before
the stage, on the same dumps and programs, are byte for byte today's:
[`runs/before-b5fb8d70-vs-today.diff`](runs/before-b5fb8d70-vs-today.diff) is empty (the region view with its run,
the full lattice, the four h-space graphs, both small programs; the outputs in
[`runs/before-b5fb8d70/`](runs/before-b5fb8d70/)).

### 2.1 The region view: a reported state is an octagon with its facts' text in it

`ein oracle --dump` classifies every node of the cube; `ein solve --dump-lattice --dump-answer` is the run laid over
it; `ein render region` draws both. Each node's label is its facts, one a line, then its status and what the run did;
the **shape** says what the run did with it: an **octagon** for a state the run reported, a rounded box for a node it
visited, an ellipse for one it did not. Each octagon is filled with the node's status colour.

![ein render region of 02_genuine_3set_death: a Hasse diagram of 8 nodes in four layers; ∅ and the three nodes of one candidate and the three of two are green filled octagons labelled with their facts' text, the state number and R · min or R · max; the node of all three candidates is a salmon rounded box, dead](renders/02_genuine_3set_death.region.svg)

([dot](renders/02_genuine_3set_death.region.dot)) — 7 octagons, each a reported state, labelled
`(a-prop X)\n(b-prop X)\nR · max\nstate 5`; the dead node a rounded box. The cube alone, no run on it:
[`renders/02_genuine_3set_death.region.cube.svg`](renders/02_genuine_3set_death.region.cube.svg) (every node an
ellipse: no run, so no octagon). The same as a table: [`runs/02_genuine_3set_death.region.table.txt`](runs/02_genuine_3set_death.region.table.txt).

![ein render region of 05_owed_seats_bounded: 16 nodes in five layers; root and the four nodes of one candidate are amber rounded boxes, owing; the two seatings are double-bordered filled octagons, green and blue, R's minimal and maximal line; the rest salmon, dead, boxes where visited and ellipses where not](renders/05_owed_seats_bounded.region.svg)

([dot](renders/05_owed_seats_bounded.region.dot)) — the two states of R are octagons labelled
`(seats Ann C2)\n(seats Bob C1)\nR · min · max\nstate 2`; owing nodes amber, dead salmon. The read-outs:
[`runs/02_genuine_3set_death.solve.txt`](runs/02_genuine_3set_death.solve.txt),
[`runs/05_owed_seats_bounded.solve.txt`](runs/05_owed_seats_bounded.solve.txt); the cube's own:
[`runs/02_genuine_3set_death.oracle.txt`](runs/02_genuine_3set_death.oracle.txt),
[`runs/05_owed_seats_bounded.oracle.txt`](runs/05_owed_seats_bounded.oracle.txt).

### 2.2 The lattice view: a node is a rounded box with its facts' text

`ein render lattice` runs a describing `ein solve` and draws the nodes it visits, one rounded box a commitment,
labelled with its facts: R green, owing amber, dead red.

![ein render lattice of zebra2: 65 of 120 visited nodes, rounded boxes labelled with up to six facts of color-loc, drink-loc, nation-loc, pet-loc and smoke-loc each, green for R, amber for root, red for dead](renders/zebra2.lattice.svg)

([dot](renders/zebra2.lattice.dot); `--view full`, 120 nodes: [`renders/zebra2.lattice.full.svg`](renders/zebra2.lattice.full.svg).)
The small programs': [`renders/05_owed_seats_bounded.lattice.svg`](renders/05_owed_seats_bounded.lattice.svg),
[`renders/05_owed_seats_bounded.lattice.full.svg`](renders/05_owed_seats_bounded.lattice.full.svg),
[`renders/02_genuine_3set_death.lattice.svg`](renders/02_genuine_3set_death.lattice.svg),
[`renders/02_genuine_3set_death.lattice.full.svg`](renders/02_genuine_3set_death.lattice.full.svg).

### 2.3 The h-space view: a candidate is a box or an ellipse with its fact's text

`ein render h-space` prints a read-out ([`runs/zebra2.h-space.txt`](runs/zebra2.h-space.txt): 56 candidates,
36 groups, 5 grids) or one of four undirected graphs. In `exclusion` a candidate is a **box** labelled with its
fact; in `cover` a group is a box (`g0 (color-loc Green *)`, a pattern of the candidates it holds) and a candidate an
**ellipse** labelled with its fact; in `coupling` and `one-step` a question is a box.

![ein render h-space --view cover of zebra2: five clusters, one a grid; in each, owed groups as double-bordered boxes labelled with a pattern such as (color-loc Green *) and the candidates as ellipses labelled with their fact, such as (color-loc Green House-4), each joined to the two groups it is in; candidates dead alone grey and dashed](renders/zebra2.h-space.cover.svg)

([dot](renders/zebra2.h-space.cover.dot).) The other graphs, each `.dot` beside its `.svg`:
[`exclusion`](renders/zebra2.h-space.exclusion.svg) (56 boxes, 410 lines),
[`coupling`](renders/zebra2.h-space.coupling.svg) (37 nodes, 419 lines),
[`one-step`](renders/zebra2.h-space.one-step.svg) (125 nodes, 729 lines, the slowest here: 6.8 s of `dot`,
[`runs/draw-times.tsv`](runs/draw-times.tsv)); and the small programs' eight,
[`renders/05_owed_seats_bounded.h-space.exclusion.svg`](renders/05_owed_seats_bounded.h-space.exclusion.svg) (four
boxes `(seats Ann C1)` …, four lines) among them, read-outs
[`runs/05_owed_seats_bounded.h-space.txt`](runs/05_owed_seats_bounded.h-space.txt),
[`runs/02_genuine_3set_death.h-space.txt`](runs/02_genuine_3set_death.h-space.txt).

## 3. The options

### (a) Left as they are (the default)

A node of these views is a state, a commitment or a candidate, and its label lists what it holds; it is not a
drawing of a fact, so the canonical form does not apply. Nothing changes.

### (b) They are form 4 too: each state's or candidate's facts drawn as nodes

**A mock**, [`renders/02_genuine_3set_death.region.canonical-states.MOCK.dot`](renders/02_genuine_3set_death.region.canonical-states.MOCK.dot)
/ [`.svg`](renders/02_genuine_3set_death.region.canonical-states.MOCK.svg), made from today's region DOT by
[`tools/mock_region_canonical.py`](tools/mock_region_canonical.py) (its docstring lists every edit): each node becomes
a frame (a cluster), filled and bordered as the node was and labelled with the label's lines that are not facts
(`R · max · state 5`), holding each of its facts as an unlabelled octagon with its arrow `1` at its relation's box
and `2` at `X`; the cube's arrows run frame to frame; the legend is left out.

![Mock of option (b): the same Hasse diagram with each state a filled frame holding its facts as unlabelled octagons, each with arrow 1 to a box a-prop, b-prop or c-prop and arrow 2 to a box X; the drawing is three times as wide, a layer is two rows, and the octagon of a reported state, the double border of the minimal line and the shape channel are gone](renders/02_genuine_3set_death.region.canonical-states.MOCK.svg)

What it costs, as the mock shows: every state draws its own atoms (`X` appears twelve times): a box shared by two frames
cannot be drawn, since a node lies in one cluster, so *every object once* cannot hold across states; the view's
**shape** channel (octagon reported, rounded box visited, ellipse neither) has no node left to carry it, and a
frame cannot be an octagon or double-bordered, so the minimal line's double border goes too; the eight-node cube is
1 692 pt wide against 536 pt for today's drawing without its legend. For zebra2's lattice, whose nodes hold up to six facts each, every node would be a frame of
six octagons and some fifteen boxes. The `cover` and `exclusion` graphs draw a candidate as **one** node joined to
others; drawn as an octagon with arrows, each line of the graph would end at an octagon among its own atoms.

## 4. The default

**(a), left as they are** (the ledger): *a node is a state or a candidate, and its label a list of what it holds,
not a drawing of a fact.* Nothing was built: `region_dot.rs`, `dump/region_lattice.rs` and `structure.rs` are
unchanged, as the empty diff in § 2 shows. The pages took the same reading for their hand drawings of states
(Q-M1f.470, item 50).

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | every run and drawing below, from the repo root |
| [`tools/mock_region_canonical.py`](tools/mock_region_canonical.py) | the script that wrote option (b)'s mock source once, from today's region DOT |
| [`programs/02_genuine_3set_death.ein`](programs/02_genuine_3set_death.ein), [`programs/05_owed_seats_bounded.ein`](programs/05_owed_seats_bounded.ein) | copies of `examples/lattice/` |
| [`programs/zebra2.ein`](programs/zebra2.ein) | a copy of `examples/zebra2.ein` |
| `runs/NAME.oracle/` ([02](runs/02_genuine_3set_death.oracle/), [05](runs/05_owed_seats_bounded.oracle/)) | `ein oracle --dump`: `region.json`, `region.md`, every node |
| `runs/NAME.lattice/`, `runs/NAME.answer/` ([02](runs/02_genuine_3set_death.lattice/), [02](runs/02_genuine_3set_death.answer/), [05](runs/05_owed_seats_bounded.lattice/), [05](runs/05_owed_seats_bounded.answer/)) | `ein solve --dump-lattice --dump-answer`: the run laid over the cube |
| [`runs/02_genuine_3set_death.oracle.txt`](runs/02_genuine_3set_death.oracle.txt), [`runs/05_owed_seats_bounded.oracle.txt`](runs/05_owed_seats_bounded.oracle.txt) | `ein oracle`'s read-out |
| [`runs/02_genuine_3set_death.solve.txt`](runs/02_genuine_3set_death.solve.txt), [`runs/05_owed_seats_bounded.solve.txt`](runs/05_owed_seats_bounded.solve.txt) | `ein solve`'s read-out |
| [`runs/02_genuine_3set_death.region.table.txt`](runs/02_genuine_3set_death.region.table.txt), [`runs/05_owed_seats_bounded.region.table.txt`](runs/05_owed_seats_bounded.region.table.txt) | `ein render region --view table`, markdown |
| [`runs/02_genuine_3set_death.h-space.txt`](runs/02_genuine_3set_death.h-space.txt), [`runs/05_owed_seats_bounded.h-space.txt`](runs/05_owed_seats_bounded.h-space.txt), [`runs/zebra2.h-space.txt`](runs/zebra2.h-space.txt) | `ein render h-space`'s read-out |
| `renders/NAME.region.cube.dot` / `.svg` ([02](renders/02_genuine_3set_death.region.cube.svg), [05](renders/05_owed_seats_bounded.region.cube.svg)) | the cube with no run on it |
| `renders/NAME.region.dot` / `.svg` ([02](renders/02_genuine_3set_death.region.svg), [05](renders/05_owed_seats_bounded.region.svg)) | the cube with the run on it |
| [`renders/02_genuine_3set_death.region.canonical-states.MOCK.dot`](renders/02_genuine_3set_death.region.canonical-states.MOCK.dot) / [`.svg`](renders/02_genuine_3set_death.region.canonical-states.MOCK.svg) | option (b), a mock |
| `renders/NAME.lattice.dot` / `.svg`, `renders/NAME.lattice.full.dot` / `.svg` ([02](renders/02_genuine_3set_death.lattice.svg), [02 full](renders/02_genuine_3set_death.lattice.full.svg), [05](renders/05_owed_seats_bounded.lattice.svg), [05 full](renders/05_owed_seats_bounded.lattice.full.svg), [zebra2](renders/zebra2.lattice.svg), [zebra2 full](renders/zebra2.lattice.full.svg)) | `ein render lattice`, `--view solution` (default) and `full` |
| `renders/NAME.h-space.VIEW.dot` / `.svg`, VIEW `exclusion`, `cover`, `coupling`, `one-step` ([02 cover](renders/02_genuine_3set_death.h-space.cover.svg), [05 exclusion](renders/05_owed_seats_bounded.h-space.exclusion.svg), [zebra2 cover](renders/zebra2.h-space.cover.svg), [zebra2 exclusion](renders/zebra2.h-space.exclusion.svg)) | `ein render h-space --view VIEW`, twelve graphs |
| [`runs/before-b5fb8d70/`](runs/before-b5fb8d70/) | the region, full lattice and four h-space views by `ein-head-b5fb8d70` |
| [`runs/before-b5fb8d70-vs-today.diff`](runs/before-b5fb8d70-vs-today.diff) | their `diff -u` against today's: empty |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | each drawing's layout and Graphviz's seconds |
| [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) | each command's exit code |

## 6. Re-run

From the repo root:

```sh
bash plans/m1f_hypothesis_and_documentation/review/03-drawings/41-Q-M1f.430/rerun.sh
```

It runs `ein oracle --dump`, `ein solve --dump-lattice --dump-answer` and `ein render region|lattice|h-space` with
`build/review-bin/ein-head-17085b27`, the same views with `build/review-bin/ein-head-b5fb8d70` for the diff, and draws
every `.dot` with `dot` (none of these views suggests a layout), the mock's from its kept source.

## 7. Found while building

- **The region view's reported state is a filled octagon with its facts' text in it**: in the drawings of facts an
  octagon is a fact and is never filled (Q-M1f.398), so the two vocabularies now share a shape with two meanings; a
  reader who has learnt the canonical form may take a region octagon for a fact. Each view has its own legend
  (the region's in the picture), so it is read correctly from the legend.
- **A box is an atom in the drawings of facts (Q-M1f.433) and a candidate in `ein render h-space --view
  exclusion`**, labelled with the candidate's fact; in `cover` a box is a group. Here too the views do not share a
  vocabulary.
- No view of the three draws an `is-a`, `relation` or any relation otherwise than another: they read no name.
- `ein render region --view table` is markdown; this item keeps it as `.txt`, as the re-run's helper names every
  stdout.

## 8. Your call

- **(a) Left as they are** (default): a state's, node's or candidate's label lists its facts; these views are not
  drawings of facts.
- **(b) Drawn as facts**: each state, node or candidate a frame of canonical octagons, with the costs the mock shows
  (atoms drawn once per state, the shape channel and the minimal line's double border gone, three times as wide).
