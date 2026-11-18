# Q-M1f.470 — which drawings on the pages the canonical form leaves as they are

## 1. The question

The ruling (Q-M1f.419) says a **fact** is drawn canonical, and that no page holds a labelled octagon or a node
labelled with a fact's text. Many drawings on the pages are not drawings of facts: a lattice, a cube or a region, whose
nodes are **states** labelled with the facts they hold; the hypothesis space, whose nodes are **candidates**; a
dependency graph of relations or rules; a schematic of steps; a design diagram of the crates. The views of `ein
render` that draw states and candidates were left as they are (Q-M1f.430, item [41](../41-Q-M1f.430/README.md)), and
the pages were read the same way: every drawing that is not a drawing of facts was left as it is, though its nodes
carry a fact's text or a region's state is an octagon with its facts in it. Is that right, or should each of them draw
its facts in the canonical form?

Recorded: [`open_questions.md` § Q-M1f.470](../../../open_questions.md#q-m1f470--which-hand-drawings-the-canonical-form-leaves-as-they-are),
S1f.23.1's task 2, its reading 4.

## 2. What is so today

**The census** S1f.23.1's task 2 took ([`tools/census.sh`](tools/census.sh), copied from its stage record) flags every
`.dot` under `docs/` (not `docs/lib/` and not the generated rule drawings) that draws a labelled octagon, a node
labelled with a fact's text, an `oval`, `ellipse`, `rectangle` or `doublecircle` node, an empty arrowhead, an arrow
numbered `#n`, or a `«`. Re-taken on today's tree and on task 2's base
([`runs/census.HEAD.counts.txt`](runs/census.HEAD.counts.txt), [`runs/census.at-1418fba5.counts.txt`](runs/census.at-1418fba5.counts.txt)):

| | files | labelled octagon | fact's text | `ellipse` | `rectangle` | `oval` | empty head | `#n` | `«` |
|---|---|---|---|---|---|---|---|---|---|
| before task 2 (`1418fba5`) | 108 | 48 | 19 | 27 | 21 | 17 | 14 | 8 | 1 |
| today (`17085b27`) | 43 | 9 | 10 | 27 | 2 | 0 | 0 | 0 | 0 |

(a column counts files; the record's table reproduces.) **The 43 left, by what they draw**
([`runs/left-as-they-are.md`](runs/left-as-they-are.md), by [`tools/categories.py`](tools/categories.py)):

| what it draws | files |
|---|---|
| a region view (`ein render region`), its states labelled with their facts, a reported state an octagon | 13 |
| a lattice, a cube or a lattice dump, its nodes states | 13 |
| the hypothesis space, its nodes candidates | 3 |
| a state or a construction, its nodes states | 2 |
| a dependency graph, its nodes relations or rules | 4 |
| a schematic or a flowchart | 2 |
| a design diagram of `ein.rs/` | 6 |

The 27 `ellipse` files are Graphviz's default shape, a drawing that sets none; the 9 labelled octagons are all the
region views' reported states; the 10 *fact's text* labels are states, candidates, modules or steps. **No drawing of
facts is flagged.** Reading 4 also names drawings the census does not flag (their nodes carry no fact's text), such as
`01-two-layers`, `02-two-phases`, `03-boundary`, `01-main-steps`; they are left too.

**One of each kind**, copied as the pages hold them ([`renders/`](renders/)):

![A region view of three_components: each reported state an octagon labelled with its facts, (h C) R · min state 1, and the like, the lattice's nodes rounded boxes and ellipses, filled by status](renders/08-region-three-components.svg)

![A lattice of two candidates: four states, each an ellipse labelled with its facts, ∅, (picky Ann), (vegan Ann), both](renders/06-class-of-two.svg)

- [`renders/05-exclusion-graph.svg`](renders/05-exclusion-graph.svg): candidates as boxes with their text, joined
  where they exclude each other;
- [`renders/08-constructions.svg`](renders/08-constructions.svg): states and how one is made from another;
- [`renders/03-strata.svg`](renders/03-strata.svg): relations, joined by what a rule reads;
- [`renders/04-zebra2-pieces.svg`](renders/04-zebra2-pieces.svg): the guide's schematic of zebra2's parts.

## 3. The options

- **(a) left as they are** (the default): a node there is a state, a candidate, a relation, a step or a module, and
  its label names what it holds or is; it is not a drawing of a fact.
- **(b) each drawn with its facts in the canonical form**: a state, a candidate set, a region's reported state, each a
  frame holding its facts, each fact an octagon whose arrow `1` is at its relation. **A mock** on the smallest,
  `06-class-of-two`, edited by hand
  ([`renders/06-class-of-two.option-b-facts-drawn.MOCK.dot`](renders/06-class-of-two.option-b-facts-drawn.MOCK.dot)):
  each state a filled frame with its facts drawn, the fact saturation adds dashed, the lattice's arrows between the
  frames.

![Mock: the same lattice, each state a filled frame holding its facts as octagons with arrows 1 and 2 at picky, vegan and Ann, the fact saturation adds dashed](renders/06-class-of-two.option-b-facts-drawn.MOCK.svg)

  The four nodes take some forty nodes and edges; the region views above, of 7 candidates and up to four facts a
  state, would each hold a few hundred, and the 13 region views are generated by `ein render region`, so (b) for them
  is a change to the region view's code (Q-M1f.430) as well as to the pages. A frame is also what the views draw
  around a fact in a context only (Q-M1f.398), so a state's frame would need a look of its own (filled, here).

## 4. The default

**(a)**, taken by task 2: *"Left as they are (Q-M1f.430's reading, extended to every drawing that is not of facts):
the region, lattice, cube and h-space drawings …; the states and constructions …; the dependency graphs …; the
schematics and flowcharts …; `ein.rs/`'s design diagrams."* It stands or falls with Q-M1f.430 for the 13 region views.

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | the census, the table and the drawings, from the repo root |
| [`tools/census.sh`](tools/census.sh) | S1f.23.1 task 2's census, copied; its one argument the revision (`HEAD`, or `WORKTREE`) |
| [`tools/categories.py`](tools/categories.py) | the flagged files by what they draw, read off their names |
| [`runs/census.HEAD.tsv`](runs/census.HEAD.tsv), [`runs/census.HEAD.counts.txt`](runs/census.HEAD.counts.txt) | the census today, raw and counted |
| [`runs/census.at-1418fba5.tsv`](runs/census.at-1418fba5.tsv), [`runs/census.at-1418fba5.counts.txt`](runs/census.at-1418fba5.counts.txt) | the same before task 2 |
| [`runs/left-as-they-are.md`](runs/left-as-they-are.md) | the 43, by kind, each with its flags |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds |
| [`renders/08-region-three-components.dot`](renders/08-region-three-components.dot) / [`.svg`](renders/08-region-three-components.svg), [`renders/06-class-of-two.dot`](renders/06-class-of-two.dot) / [`.svg`](renders/06-class-of-two.svg), [`renders/05-exclusion-graph.dot`](renders/05-exclusion-graph.dot) / [`.svg`](renders/05-exclusion-graph.svg), [`renders/08-constructions.dot`](renders/08-constructions.dot) / [`.svg`](renders/08-constructions.svg), [`renders/03-strata.dot`](renders/03-strata.dot) / [`.svg`](renders/03-strata.svg), [`renders/04-zebra2-pieces.dot`](renders/04-zebra2-pieces.dot) / [`.svg`](renders/04-zebra2-pieces.svg) | one drawing of each kind, copied from the pages |
| [`renders/06-class-of-two.option-b-facts-drawn.MOCK.dot`](renders/06-class-of-two.option-b-facts-drawn.MOCK.dot) / [`.svg`](renders/06-class-of-two.option-b-facts-drawn.MOCK.svg) | mock: option (b) |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/50-Q-M1f.470/rerun.sh`. It keeps
the `.MOCK.dot` and draws it.

## 7. Found while building

- The record's census reproduces on today's tree, file for file: 108 flagged before, 43 after, the same counts a flag.
- The 9 labelled octagons that stay are all one thing, the region view's *a state the run reported*, whose octagon
  carries the state's facts as text: the one place an octagon is labelled, and the binary writes it.

## 8. Your call

- **(a)** left as they are: states, candidates, relations, steps and modules are no drawing of facts (default).
- **(b)** each drawn with its facts canonical (and the region view with them, Q-M1f.430).
