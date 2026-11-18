# Q-M1f.469 — how a derivation's drawing marks a terminal and the core

## 1. The question

The reasoning page on cores draws a derivation as a **proof graph**: facts, the derivations between them, and its
**terminals**, the facts every walk grounds out on (the givens and the commitments); and, for a dead node, its
**core**, the fewest terminals the contradiction follows from. Before the pages were drawn again, each fact was a box
labelled with its text (form 4, now ruled out), a terminal double-bordered and the core's facts **filled** salmon. Now
each fact is an unlabelled octagon with its arrows, and no octagon is filled (Q-M1f.398). The pages' four derivation
drawings were drawn again with **a terminal double-bordered and the core red** (the octagon's outline, heavy). Is that
the mark, or another, or none?

Recorded: [`open_questions.md` § Q-M1f.469](../../../open_questions.md#q-m1f469--how-a-derivations-drawing-marks-a-terminal-and-the-core),
S1f.23.1's task 2, its reading 3.

## 2. What is so today

The four derivation drawings of `reasoning/02-kb-evolution/` are by hand. Copied as the pages hold them
([`renders/`](renders/)), and as they were before task 2 (`1418fba5`):

| drawing | page | today | before |
|---|---|---|---|
| `02-derivation` | `02-saturation.md` | each fact an octagon; what a firing derives heavy; no terminal or core mark ([svg](renders/02-derivation.svg)) | text boxes ([svg](renders/02-derivation.at-1418fba5.svg)) |
| `09-proof-graph` | `09-cores.md` | the givens A, B, C double-bordered octagons, their letters beside them; F heavy ([svg](renders/09-proof-graph.svg)) | text boxes `A\ngiven`, double-bordered ([svg](renders/09-proof-graph.at-1418fba5.svg)) |
| `09-cyclic` | `09-cores.md` | the given r(a, b) double-bordered ([svg](renders/09-cyclic.svg)) | text boxes ([svg](renders/09-cyclic.at-1418fba5.svg)) |
| `09-core-in-derivation` | `09-cores.md` | five terminals double-bordered, the core's three **red and heavy** ([svg](renders/09-core-in-derivation.svg)) | text boxes, the core's three filled salmon ([svg](renders/09-core-in-derivation.at-1418fba5.svg)) |

![The derivation with its core, today: five double-bordered octagons are the terminals, three of them red, the facts of lives-in, next-to and owns; the two of is-a black; the firings neighbour and fox-keeps-away as rounded boxes, the derived near fact and the fact of false plain octagons](renders/09-core-in-derivation.svg)

![The same before task 2: each fact a box with its text, the core's three filled salmon](renders/09-core-in-derivation.at-1418fba5.svg)

The page says: *"The derivation of saturation, with its terminals double-bordered. The core is the three terminals
drawn red; the two givens of $\operatorname{is-a}$, which no derivation of $\bot$ reads, are outside it."*

**What the binary draws of the same derivation** has neither mark. [`programs/fox.ein`](programs/fox.ein) is the
program the page draws (from `ein.rs/08-reasoning-in-code/02-saturation-and-negation.md` § 2.4); `ein solve --trace`
([`runs/fox.trace.md`](runs/fox.trace.md)) refutes root, its read-out naming the three stated facts as the core
([`runs/fox.solve.txt`](runs/fox.solve.txt)), and its two slices draw each fact an octagon, what a step derives
heavy, and no terminal or core mark ([`renders/fox.trace-step1.svg`](renders/fox.trace-step1.svg),
[`renders/fox.trace-step2.svg`](renders/fox.trace-step2.svg)). A slice marks a commitment's facts red (Q-M1f.428),
so in the binary's drawings red is *a commitment's*, on the page *the core's*; a core is made of terminals, and a
commitment's facts are terminals, so the two readings meet but are not the same.

## 3. The options

- **(a) a terminal double-bordered, the core red** (the default): today's pages.
- **(b) another mark, or none.** Two mocks, each `09-core-in-derivation.dot` edited by hand:
  - **no mark** ([`renders/09-core-in-derivation.option-b-no-mark.MOCK.dot`](renders/09-core-in-derivation.option-b-no-mark.MOCK.dot),
    [svg](renders/09-core-in-derivation.option-b-no-mark.MOCK.svg)): `peripheries=2`, the red and the heavy outline
    taken off; the page would say which facts are terminals and which the core, in words;
  - **the core in a frame** ([`renders/09-core-in-derivation.option-b-core-frame.MOCK.dot`](renders/09-core-in-derivation.option-b-core-frame.MOCK.dot),
    [svg](renders/09-core-in-derivation.option-b-core-frame.MOCK.svg)): the three in a dashed red frame labelled
    `core`, the terminals kept double-bordered. A dashed frame is what every view draws around a fact in a context
    only (Q-M1f.398), so this mark reads as one.

![Mock: the core's three facts in a dashed frame labelled core, black, the terminals double-bordered](renders/09-core-in-derivation.option-b-core-frame.MOCK.svg)

These are drawings by hand on a page; no option changes code.

## 4. The default

**(a)**, taken by task 2: *"A derivation's terminal is double-bordered and its core red, where it was filled
(Q-M1f.398: no octagon filled); a fact's text is never its label nor its `xlabel`."*

## 5. Files

| file | what it is |
|---|---|
| [`README.md`](README.md) | this page |
| [`rerun.sh`](rerun.sh) | the copies, the trace and every drawing, from the repo root |
| [`programs/fox.ein`](programs/fox.ein) | the derivation's program, from `ein.rs/08-reasoning-in-code/02-saturation-and-negation.md` § 2.4 |
| [`renders/02-derivation.dot`](renders/02-derivation.dot) / [`.svg`](renders/02-derivation.svg), [`renders/09-proof-graph.dot`](renders/09-proof-graph.dot) / [`.svg`](renders/09-proof-graph.svg), [`renders/09-cyclic.dot`](renders/09-cyclic.dot) / [`.svg`](renders/09-cyclic.svg), [`renders/09-core-in-derivation.dot`](renders/09-core-in-derivation.dot) / [`.svg`](renders/09-core-in-derivation.svg) | the four drawings, copied from `docs/ein/reasoning/02-kb-evolution/diagrams/` |
| [`renders/02-derivation.at-1418fba5.dot`](renders/02-derivation.at-1418fba5.dot) / [`.svg`](renders/02-derivation.at-1418fba5.svg), [`renders/09-proof-graph.at-1418fba5.dot`](renders/09-proof-graph.at-1418fba5.dot) / [`.svg`](renders/09-proof-graph.at-1418fba5.svg), [`renders/09-cyclic.at-1418fba5.dot`](renders/09-cyclic.at-1418fba5.dot) / [`.svg`](renders/09-cyclic.at-1418fba5.svg), [`renders/09-core-in-derivation.at-1418fba5.dot`](renders/09-core-in-derivation.at-1418fba5.dot) / [`.svg`](renders/09-core-in-derivation.at-1418fba5.svg) | the same before task 2 |
| [`renders/09-core-in-derivation.option-b-no-mark.MOCK.dot`](renders/09-core-in-derivation.option-b-no-mark.MOCK.dot) / [`.svg`](renders/09-core-in-derivation.option-b-no-mark.MOCK.svg) | mock: no mark |
| [`renders/09-core-in-derivation.option-b-core-frame.MOCK.dot`](renders/09-core-in-derivation.option-b-core-frame.MOCK.dot) / [`.svg`](renders/09-core-in-derivation.option-b-core-frame.MOCK.svg) | mock: the core in a frame |
| [`renders/fox.trace-step1.dot`](renders/fox.trace-step1.dot) / [`.svg`](renders/fox.trace-step1.svg), [`renders/fox.trace-step2.dot`](renders/fox.trace-step2.dot) / [`.svg`](renders/fox.trace-step2.svg) | the binary's two slices of the derivation |
| [`runs/fox.trace.md`](runs/fox.trace.md), [`runs/fox.solve.txt`](runs/fox.solve.txt) | `ein solve --trace` and its read-out |
| [`runs/draw-times.tsv`](runs/draw-times.tsv) | Graphviz's layout and seconds |

## 6. Re-run

From the repo root: `bash plans/m1f_hypothesis_and_documentation/review/03-drawings/49-Q-M1f.469/rerun.sh`. It keeps
the two `.MOCK.dot` files and draws them.

## 7. Found while building

- **Red means two things across the drawings**: a commitment's facts in the binary's slices (Q-M1f.428), the core's
  terminals on the cores page. A commitment's facts are terminals, so a core can be all red in both, but a core of
  givens is red only on the page.
- **Heavy means *derived* in both**: the binary's slices draw what a step derives at `penwidth=2`, and so do the
  hand drawings; the core's red octagons are heavy too, the one place heavy does not mean derived.
- `02-derivation` marks no terminal; it is the saturation page's, where terminals are not yet defined.
- **`09-core-in-derivation` lost the heavy outline of its two derived facts** in the redrawing: before, `near` and
  `false` were at `penwidth=2`, as they are in `02-derivation` and in the binary's slices; today they are plain
  octagons, so on the cores page nothing marks what was derived but the arrows from the firings. A slip of the
  redrawing, not a reading; the user may want it put back with whichever option stands.

## 8. Your call

- **(a)** terminals double-bordered, the core red (default).
- **(b)** another mark (e.g. a frame, which reads as a context), or none (the page says it in words).
