# S1f.23.6 — the user's answers on the drawings, built

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word (*"plans/m1f_hypothesis_and_documentation/review/03-drawings/ANSWERS.md
- I answered, act on them"*), in a worktree beside the phase's final checks on master; stopped, and continued on the
user's word (*"continue"*); **built** in the worktree `build/wt-s1f236` on the branch `s1f.23.6-answers`: the first
build `ad20a5e0`, then the folded arrows within a layer `29792a38` and this Record; waiting for the merge, after which one gate,
`utils/ci_container.sh` (the stage changes DOT writers) and one nightly hold it. Its readings are the Record's, for
the main session to file; the user's look is the re-created `review/03-drawings/`, items 56 to 60.
**The answers:** the user's file as written, commit `e4c0eaf6`
(`git show e4c0eaf6:plans/m1f_hypothesis_and_documentation/review/03-drawings/ANSWERS.md`); the topic left the bundle
with every item answered (`ff130d1b`).

## The answers, item by item

| item | the question | the user's answer |
|---|---|---|
| L-97 | nine `div9-step` rules past the drawing bound | *"Keep, and in future ignore all cases when render hits any limit, unless I explicitely ask about it."* |
| L-100 | the beliefs' KB drawings have no test | *"No tests required here!"* — and: tests verify the DOT graph's structure, not its layout, which is the user's preference; **the preferred default layout is layered**, objects → facts → objects → …; in the simple case five layers: the `relation` atom on top → the declarations → the relations and the labels → the relations' facts → the plain objects; a relation's properties (`symmetric`) at the `relation` atom's level (preferred, not required) |
| Q-M1f.423 | the arrows' numbering | (a) |
| Q-M1f.424 | a fact's notes as an `xlabel` | (a) |
| Q-M1f.425 | the fold | **(a) + (b)**: a fact of three arrows folds into a labelled arrow, **and a fact of two arrows into a loop on its argument** |
| Q-M1f.426 | the constraint view | (a); and *"would be nice to formalize this KB slice with something, e.g. ein query or rule that selects all nodes where relation is argument (is it correct definition?), and together with `constraints` allow to render arbitrary query"*: [S1f.23.7](s1f.23.7_a_slice_by_a_query.md) |
| Q-M1f.427, R-15 | a rule's pattern; a `forall`'s guard | (a); R-15 (i), as (a) |
| Q-M1f.428 | a derivation's drawing | (a), *"just use more contrast colors and 2x thicker lines then default"* |
| Q-M1f.429 | a folded fact in a slice | *"do not fold, when we show derivation with additional arrow (of different color); general rule for rendering - no arrow to/from arrows, arrows go only to/from shapes; So other 3-facts and 2-facts can be folded, but not those that hold derivation arrows."* |
| Q-M1f.430 to .435 | the region, lattice, h-space views; the program view's parts; the KB view whole; every atom a box; the stdlib's fallback; `xlabel` | (a) each; Q-M1f.433: *"object - labelled box, fact - non-labelled octagon, variable - labelled diamond"* |
| Q-M1f.467 to .469, .471 to .473 | the pages' drawings | (a) each; .471: *"their own arrows have nothing related to this ordering example"* |
| Q-M1f.470 | what the canonical form leaves | (a): *"canonical drawing is ONLY for Ein KB (objects and facts), everything else can be drawn in any other best way"*; the mock of a small lattice with its states' facts drawn, and `08-constructions`, *"look good"*: an option for a small lattice and a small KB, an exception |
| Q-M1f.474 | an arrow that is no arrow of the graph | *"Allowed as hand-written, for explanations, not render output."* |
| R-18 | KB views laid out as wide strips | *"same as L-100"*: the layered layout |

**The user's note on the layered layout, 2026-11-18, during the stage** (relayed by the main session), verbatim:
*"maybe we still can combine nested facts and layered layout if allow have multiple layers of nested facts, so still
have objects-facts-objects sandwich, but facts can have multiple layers (ranks); And global-only facts can be on one
layer/rank, and nested go under on lower rank"*. Read as: the layers stay a sandwich, objects / facts / objects; a band
of facts between two layers of objects may take several ranks; in a band the facts held in the global context only,
nothing pointing at them, share one rank, a fact under another's arrow goes a rank below it, deeper nesting lower still;
so nesting deepens a band instead of breaking the sandwich. Built (§ 1 of the Record).

## What it builds

1. **The layered layout, by default** (L-100, R-18): every drawing of the knowledge base in the canonical form (the KB
   view, the program view's `kb`, the constraint view; a rule's panel and a trace's slice where it fits) lays out
   objects and facts in layers: **a fact's first arrow drawn up to its relation** (as the reasoning pages' hand
   drawings write it, `"left-of" -> f3 [label="1", dir=back]`), its other arrows down to its arguments, `dot` the
   layout and the `fdp` hint gone, so `relation` stands on top and a relation's properties at its level. No test pins
   a layout (the user's rule): the structure tests stay what they are.
2. **The fold, (a) and (b)**: under `--fold` a fact of two arrows that nothing drawn points at is a loop on its
   argument, labelled with the relation, beside the three-arrow fold.
3. **No arrow to or from an arrow** (Q-M1f.429, a rule for every drawing): in a trace's slice a fact a derivation
   arrow touches is never folded; every other fact of two or three arrows folds under `--fold`. The point node that
   carried a folded fact's derivation goes.
4. **A derivation's drawing** (Q-M1f.428): colours of more contrast and lines twice Graphviz's default width.
5. **R-33**, the slices Graphviz refuses: taken again over every trace of the corpus in both forms after 1 and 3;
   what still fails is fixed within the layered layout, and the probe's numbers recorded.
6. **The ledger**: each answer filed against its question as ruled, the user's words verbatim (`open_questions.md`,
   the milestone's and the phase's findings); a decision file for each ruled question that states Ein's surface, or
   the decision it amends, per `docs/decisions/README.md`; `12-rendering.md` and the pages brought to the rulings.
7. **The review bundle**: the stage's own readings, each a new item of a re-created `review/03-drawings/`, with the
   layered drawings on real programs and an `ANSWERS.md`, for the user's look.

## Acceptance

- No answer moves: of the shape golden only `dot[…]` and `region-trace` lines move.
- Every drawing of every corpus program well-formed (`dot_wellformed`), and the trace probe of 5's numbers.
- The gate, `utils/ci_container.sh` and the nightly, once on the merged tree.

## Record

In the worktree `build/wt-s1f236`, branch `s1f.23.6-answers`, on `6de9101b`, beside the phase's final checks on
master. Binaries: *before* is master's code at the stage's start (`build/review-bin/ein-head-6de9101b`, the gate
build of `23539de5`, whose code is `6de9101b`'s), *after* this branch's (`build/review-bin/ein-s1f236`).

### 1. The layered layout

**As built.** A fact's first arrow is written from its relation's atom to the fact and drawn back,
`"rel" -> f [xlabel="1", dir=back, …]`, one function for every writer (`ein_render::fact_graph::first_arrow`): the
KB view, the program view's `kb`, the constraint view, a rule's panels, a trace's slices, the derivation graph and a
written trace's `dag` view. `rankdir=TB` everywhere (the KB view was `BT`, the constraint view and the slices `LR`,
the derivation graph `BT`); the `// suggested layout: fdp` comment is gone. No name is read (Q-M1f.421); the views
of a knowledge base also write their layers, below, on the user's note. The structure is unchanged: the same nodes, the same
arrows read as drawn, so `every_object_once.rs` reads an arrow with `dir=back` from its head to its tail and fails on
an arrow drawn back that is no first arrow.

**The simple case gives the user's five layers**, by the arrows alone, before the user's note (the groups below
add nothing here), `ein render kb` of the page program `12-layers`
(`docs/ein/user/04-reference/diagrams/12-layers.dot`; the layers read off `dot -Tplain`):

```dot
  "relation" -> f_0307f66032 [xlabel="1", dir=back, color="#bcbd22", style=solid];
  f_0307f66032 -> "owns" [xlabel="2", color="#bcbd22", style=solid];
  f_0307f66032 -> "Person" [xlabel="3", color="#bcbd22", style=solid];
  f_0307f66032 -> "Pet" [xlabel="4", color="#bcbd22", style=solid];
  "symmetric" -> f_dc459fa9ff [xlabel="1", dir=back, color="#7f7f7f", style=solid];
  f_dc459fa9ff -> "owns" [xlabel="2", color="#7f7f7f", style=solid];
  "owns" -> f_15e15b12a5 [xlabel="1", dir=back, color="#ff7f0e", style=solid];
  f_15e15b12a5 -> "Alice" [xlabel="2", color="#ff7f0e", style=solid];
  f_15e15b12a5 -> "Dog" [xlabel="3", color="#ff7f0e", style=solid];
```

```text
layer 1: symmetric relation
layer 2: the five facts of relation and symmetric
layer 3: Pet Person owns "lives-in" House
layer 4: the four facts of owns and lives-in
layer 5: "House-2" "House-1" Dog Cat Bob Alice
```

`symmetric`, whose fact is about `owns`, stands on `relation`'s layer and its fact beside the declarations, by
structure alone. Before, `dot` laid the same view out in two layers, every atom above every fact.

**Nested facts, first by the arrows alone** (`examples/beliefs/01_common_belief.ein`, `ein render kb`): seven ranks
with the sandwich broken: `likes` and `not` beside the facts of `believes`, a rank below the declared relations, since
`dot` keeps their facts' first arrows short; nested facts on the rank of the names under `believes`.

**Then the bands, on the user's note** (beside the answers above): the view writes the layers for `dot`,
`fact_graph::layers`, one `{ rank=same; … }` group a rank under `// layers`, computed off the arrows alone:

- each atom has an object layer and each octagon a band and a depth in it, by a fixpoint: a fact's band is the
  largest of its relation's layer and its pointers' bands, its depth one more than the deepest pointer in the same
  band (0 for a fact nothing points at), and a name under a fact's argument arrow is in the layer below the fact's band;
- a relation under no argument arrow stands in the layer just above its highest fact, and a flat fact of such a
  relation (holding no fact, nothing pointing at it) sinks to the band just above its names;
- the groups: each object layer, and each (band, depth), top first, **ordered by a chain of invisible anchors**,
  `"layer 0" -> "layer 1" -> … [style=invis]`, one in each group (an id with a space, which no name has): a
  `rank=same` group says only that its members share a rank, and without the chain two groups no arrow joins shared
  one (`06_double_negation.ein`: the inner negations of `(not (not g))` on the rank of the declared relations);
- where the fixpoint does not settle (the arrows go round: a fact of a relation whose own name is under its arrows,
  `01-instance-of-instance`'s `(is-a is-a …)`), no groups are written and `dot` layers by the arrows;
- a folded fact is no node and in no group, but places its names as its octagon would (*Folded arrows*, below);
  slices, rules and the derivation graph write no groups (a step stands between facts there).

The beliefs program in bands: seven ranks, the sandwich kept:

```text
rank 1: relation __closed__                          objects
rank 2: the seven declarations, (__closed__ person)  facts, band 1
rank 3: person Person likes Food believes Claim not  objects
rank 4: the twelve facts nothing points at           facts, band 2, depth 0
rank 5: the four facts under believes' arrows        facts, band 2, depth 1
rank 6: (likes Dan Fish), under not's arrow          facts, band 2, depth 2
rank 7: Ann Bob Cid Dan Soup Tea Eve Fish            objects
```

The simple case is the five layers as before, now written. zebra2's program `kb`: five layers, `co-located` on the
top one since its facts' arguments include relations of the third (`(co-located nation-loc Englishman color-loc
Red)`). A calculator program's `kb` (`examples/arith/add.ein`): `add`, `let` and `n` on top, then one band seven ranks
deep (the numerals nest that deep), then every digit, `z`, `apples` and `pears` on the bottom layer. Review item 56
shows each against the drawing by the arrows alone.

**Folded arrows, within a layer.** First built ranked as any arrow: the brief's `constraint=false` tried and set
aside, since alone it leaves the names only folded arrows touch ranked by nothing, on the top layer beside
`relation`, and a `{ rank=sink; … }` group of them is ignored under `newrank`, which every drawing with a frame sets
(a probe: a group of unconnected names lands on rank 0 under `newrank=true`, on the maximal rank without it). The
stage was then continued on a brief that restates the target, *"folded arrows within a layer"*, and **builds it**,
the names placed by the layers rather than by a sink: a folded arrow ranks nothing (`constraint=false`), and its
fact, no node, places its names as its octagon would. `fact_graph::layers` reads the folded facts with the
octagons: a folded fact's band is the largest of its relation's layer and its pointers', its names stand in the
layer under that band, and a fact it points at stands in that band too, on the band's top rank, since the folded
fact's octagon is not there; no group holds a folded fact. So a folded drawing keeps the canonical drawing's layers
of objects and loses the folded facts' ranks: the five-layer program under `--fold` is four layers, `relation`, the
two declarations, the relations and labels, and the six plain objects with the `owns` and `lives-in` arrows running
between them. A drawing every fact of which folds is one layer of objects with its arrows as arcs over it
(`05-row-state`, nine names), and zebra2's folded program `kb` has 38 names on its bottom layer; ranked, the same
drawings were eight layers and six, the houses a chain by `right-of` and the `is-a` hierarchy down to `T`. Item 57
puts the two side by side, the ranked drawings by the first build's binary (`build/review-bin/ein-s1f236-ranked`,
`ad20a5e0`'s code); reading 2.

### 2. The fold of a fact of two arrows

`fact_graph::folds` (`args == 1 || args == 2`): under `--fold`, a fact of two arrows that nothing drawn points at is
`a -> a [xlabel="rel note", …]` in a drawing of a knowledge base, `a -> a [label="rel", …]` in a rule's panel and in a
slice. In every view that draws facts and in the rule views (`Scene::folds`). A stored negation `(not g)` folds into
a loop on `g`'s octagon (reading 4); a closing rule's `(__closed__ R)` patterns into loops, so five closing rules are
now drawn folded on the reference pages (reading 5).

### 3. No arrow to or from an arrow

`slice.rs`: a fact a derivation's arrow touches (a premise, a fact derived, a fact of the core or of the no-good) is
never folded; any other fact of two or three arrows folds; the point node is gone. **A step's slice has one firing and
no commitment** (`trace/linearize.rs`), so every fact of it is a premise or a conclusion: under `--fold` a step's slice
is the canonical one byte for byte (reading 8), held by `render_doors.rs` on `02_packing.ein`, and across the corpus
by the probe of § 5 (every trace's folded blocks against its canonical ones). `--fold` folds a trace's `-F`
snapshots.

### 4. A derivation's drawing

`slice.rs`' `DERIVATION_PALETTE`: eight dark colours, each at a contrast of 6.0 to 13.2 against white (the
relations' palette runs 2.0 to 5.9), none red and none grey, picked by the rule's name by the same SHA-1 index as a
relation's colour (`palette::stable_index`); `DERIVATION_WIDTH`, `penwidth=2`, on every arrow of the derivation
(premise to step, step to conclusion, core to `⊥`, the no-good's) and on a step's border (which was `style=bold`).
The derivation graph, which records no rule, draws in the first colour; a written trace's `dag` view picks by the
step's rule. A derived fact stays bold (reading 7). Hand drawings on the reasoning pages restyled the same way
(`09-cyclic`, `09-proof-graph`; `02-derivation` and `09-core-in-derivation` by the transformer below).

### 5. R-33

**The probe**, `review/03-drawings/tools/probe_r33.py` (ignored; item 60 has its files): every corpus program (888,
`corpus/corpus.toml`), `ein solve FILE --trace OUT.md` and with `--fold`, 600 s each; then every distinct `dot` block
put once to `dot -Tsvg` (Graphviz 16.0.0), refused where `dot` exits non-zero or writes to stderr, and counted apart
where it lays nothing out within 120 s. On *before* (`ein-head-6de9101b`) and on this stage's first build
(`ad20a5e0`'s code); the final binary writes every trace's blocks byte for byte as the first build does (every trace
of the corpus compared in both forms, 1 776 of 1 776 the same), since the folded arrows it changed are in the views
of a knowledge base and not in a slice.

| | before | after |
|---|---|---|
| programs; traces; traces with a drawing | 888; 644; 510 | the same |
| blocks, in each form | 163 672 | 163 672 |
| canonical blocks refused | 192, in 18 traces | none |
| folded blocks refused | 558, in 34 traces | none |
| programs with a refusal | 37 | none |
| blocks not laid out within 120 s | 289, all `long_numbers.ein`'s (50 canonical, 239 folded) | none |
| folded blocks that differ from the canonical trace's | 68 814 | none |
| distinct blocks laid out | 217 225 | 154 062 |

- **No program left out**: every trace finished within 600 s with both binaries, `examples/arith/long_numbers.ein`'s
  too (2 861 blocks), which R-33's probe had left out; the 244 programs with no trace are refused at load by both
  (`examples/ein-bugs/` and `examples/broken/`, two each of `examples/lattice/` and `examples/branching/`).
- **Before**: every program with a refusal the calculator's (`examples/arith/`, `beyond_arith/`,
  `binary_arithmetic/`, `word_problems/pencils.ein`, `tests/stdlib/math/`); *triangulation failed* (then
  *Pshortestpath failed*) on all but nine folded blocks, *lost edge* on those nine. With `long_numbers.ein` left out
  the counts are R-33's own, 169 canonical in 17 traces and 487 folded in 33, of 36 programs; its 94 refused blocks
  (23, 71) and 289 not laid out within 120 s are the rest.
- **After: none refused, none past 120 s, and every folded block the canonical one.** What fixed it is § 1's layout,
  top to bottom with the first arrow drawn back where a slice was laid out left to right under `newrank`, and § 3,
  no point for a folded fact; R-33's sample had found `rankdir=TB` routing every refused block it tried. **Nothing is
  left to fix** within the layered layout.

**The check** (reading 11): `dot_wellformed`'s `graphviz_accepts_every_slice_of_the_witnesses_traces`, at every tier,
puts every slice of two traces to `dot` on the pool: `examples/arith/div.ein`, the smallest program a canonical slice
of which was refused, and `examples/binary_arithmetic/first.ein`, the smallest a folded one of which was, 611 slices,
1.8 s on this host with the default sixteen workers (another run beside it). **It belongs in `dot_wellformed`**: Graphviz's refusal of a DOT the binary writes is a fault of the DOT, the
test is the authority the DOT views have on that (`AGENTS.md`), and the user's rule leaves out the layout only; R-33
reached no one because no test put a trace's drawing to Graphviz. **Bounded**, since the probe after laid out 154 062
distinct blocks in 27 483 s of CPU with the traces (17 min 46 s on 32 threads, with other runs beside it), past any
per-commit budget and the nightly's five minutes a step; the two witnesses stand for the shape that failed, one in
each form, and a step's folded slice is its canonical one, so their canonical traces stand for both forms.

### 6. The ledger and the decisions

`open_questions.md`: Q-M1f.423 to .435 and .467 to .474, each entry with **Ruled 2026-11-18**, the item it was
answered on and the user's words verbatim, and its summary row. The milestone's findings L-97 and L-100, the phase's
R-15 and R-18, and R-33 (§ 5).

Decision files, new: `cli/` Q-M1f.425 (the fold, both forms), .426 (the constraint view), .427 (a rule's pattern; no
macro by name), .428 (a derivation, its contrast and width), .429 (no arrow to or from an arrow), .430 (the canonical
form for a knowledge base; the views of states left), .433 (box, octagon, diamond), .435 (`xlabel`); `docs/`
Q-M1f.434 (the standard library's drawings), .467 (a command's drawing on a page), .468 (a cut), .469 (terminals and
core), .470 (only a knowledge base canonical), .473 (a program no file holds), .474 (an arrow a schematic may draw).
Amended: Q21 (the layered layout, and that no test holds a layout), Q-M1f.419 (the loop, no point), Q-M1f.382 (the
loop), Q-M1f.420 (Q-M1f.431's and .432's details). Q-M1f.423 and .424 are carried by Q-M1f.419; .471 and .472 have no
file (one drawing; file names). The folders' READMEs list the new files.

### 7. The pages and the drawings

- `12-rendering.md`: the opening (layers, a preference), the shapes (Q-M1f.433), the legend's first arrow and the loop
  rows, § A fact as the graph has it (the fold of two arrows), **a new § Layers** with its program and the drawing
  `12-layers`, the program view's and the constraint view's fold sentences, the KB table, § The trace (layers, the
  derivation standing out, the `--fold` row: no arrow to or from an arrow). `05-cli.md`, `13-glossary.md`, the
  reasoning's `README.md` and `01-knowledge-base.md` § Drawn with labelled arrows (the table's loop row), and every
  page whose words described a folded fact of two arrows as an octagon or a slice as left to right: `01-objects.md`,
  `02-relations.md`, `03-facts.md`, `05-rules.md`, `07-examples.md`, `02-saturation.md`, `03-absent-and-naf.md`,
  `04-hypothesis-generation.md`, `10-satisfiability.md`, `01-rules-as-data.md`, `04-bijections.md`,
  `04-stdlib/README.md`, `ein.rs/08-reasoning-in-code/01-static-ir.md` and `02-saturation-and-negation.md`.
- **Every copy of a command's output under `docs/` drawn again** by a transformer
  (`review/03-drawings/tools/relayer.py`) that rewrites a drawing the binary wrote before into what it writes now,
  first arrows drawn back, `rankdir`, the `fdp` comment, the slices' steps, and under `--fold` the facts of two
  arrows into loops, **validated against the binary** (`review/03-drawings/tools/validate.sh`): over every corpus program, the
before binary's output transformed against this branch's, `ein render kb` 664 of 664 the same in each form,
`constraints` 664 of 664 in each, `program` 663 of 663 canonical, `rules` 347 of 347 in each mode canonical; the folded
rule views (and `program --fold`, which holds them) differ exactly where the transformer refuses, a rule with a pattern
of two arrows, and nowhere else. 97 sources drawn again and `12-layers` added; the layers written into the 29 copies
of a knowledge base's view. The four folded rule drawings with a pattern of
  two arrows, which the transformer refuses (a loop renumbers a panel's octagons), drawn by the binary from their
  pages' programs (`02-rule-note`, `03-rule-soup-unless-allergic`, `04-rule-total-owed`, `10-rule-guest-owes-seat`);
  the rendering page's drawings checked equal to the binary's output on the page's programs, all eight. The
  hand-drawn graphs of `reasoning/01-static-ir/`, already layered by `rank` groups or `dir=back`, untouched.
- **The thirteen folded drawings of a knowledge base on the pages refolded within a layer** (§ 1, *Folded arrows*),
  by a second transformer (`review/03-drawings/tools/refold.py`): every folded arrow of a digraph `kb` or
  `constraints` takes `constraint=false` and the layers are computed again with the folded facts; **validated against
  the binary** (`tools/validate_refold.py`): over every corpus program, the first build's output (a folded arrow
  ranked) put through it against this build's, `render kb --fold` 663 of 664 the same, `render program --fold` 662 of
  663, `render constraints --fold` 664 of 664, and every canonical view of the two builds the same (664, 663, 664);
  the one that differs, `examples/features/15_unicode_escapes.ein`, has a name with a line break that the
  transformer, reading a line at a time, does not parse, and no page draws it. `05-row-state`, every fact of which
  folds, is now one layer; `01-instance-of-instance` writes no layers, as its canonical drawing writes none, since
  its folded `(is-a is-a Relation)` places `is-a` under its own relation (the arrows go round): ranked, it had the
  declarations' three.
- `utils/gen_stdlib_rule_pages.py`: the folded image's text says *each of two a loop*, its docstring the closing rules;
  regenerated: 511 rules, **493 canonical, 6 folded, 12 Ein only** (before 493, 1, 17: `decimal-arith`, `rat-arith`,
  `binary-arith`, `arith9` and `props` move from Ein only to folded; `int-arith`, `mul9-step`, `div9-start` and the nine
  `div9-step` rules stay Ein only); 499 sources and 5 regions changed. `utils/render_diagrams.py` drew every picture.
- `examples/beliefs/0{1,2}_*.kb.dot` and `.svg` drawn again by their line in `examples/README.md` (L-100: no test).

### 8. Checks

**The first build** (`ad20a5e0`), in the worktree, after master's final checks had finished (none of mine ran beside
their sweeps; the R-33 probe was paused through their nightly):

- `cargo test --profile gate -p ein-render -p ein-cli`: 467 tests in 77 binaries, all passing (six failed on the first
  run, assertions that read a first arrow or a fold literally: `dot_util`'s unit test, two of `every_object_once.rs`,
  three of `presentation_semantics.rs`; brought to the drawn-back arrow and the loop and run again green);
  `help_shape.txt` re-blessed for `--fold`'s help; the goldens of `golden_dot`, `derivation_dot` re-blessed. Per commit,
  `dot_wellformed` 404.8 s on this machine with the new trace test (611 slices of 2 traces, none refused).
- The shape golden, `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test id_order_invariance` (88.1 s):
  29 027 lines before and after, **10 476 moved, none added or removed**: `dot[constraints]` 781,
  `dot[constraints-fold]` 781, `dot[ir]` 770, `dot[ir-forms]` 770, `dot[ir-overlay]` 770, `dot[ir-trace-dag]` 771,
  `dot[ir-fold]` 764, `dot[kb]`, `dot[kb-fold]`, `dot[kb-none]`, `dot[kb-origin]`, `dot[kb-since]` 665 each,
  `dot[rules]` 424, `dot[rules-overlay]` 424, `dot[rules-fold]` 386, and `region-trace` 510. No line of any other kind
  moved.
- `EIN_CORPUS_SLOW=1 cargo test … -p ein-render --test dot_wellformed -- --nocapture`: **8 819 views of 888 files
  accepted, none refused**, 1 205 past the 1 000-edge bound in 276 files, the directories `SKIPPING` names unchanged;
  796 of the structure's graphs; 611 slices; 357.0 s.
- `cargo clippy -p ein-render -p ein-cli --all-targets -- -D warnings` and `cargo fmt --all --check` clean;
  `RUSTDOCFLAGS="-D warnings" cargo doc -p ein-render -p ein-cli --no-deps` clean.
- `utils/gen_stdlib_rule_pages.py` (twice: the image text changed after the first run), then
  `utils/render_diagrams.py`: every picture drawn; `--check` 754 diagrams as they stand.
- `utils/doc_audit.py --links --check --tracked`: every finding under the review bundle's other topics, whose ignored
  outputs a worktree lacks; none elsewhere; 0 decision-id findings. `utils/check_doc_blocks.py` rules 1 to 5 hold;
  `utils/check_generated.py` 9 generators as written (with `EIN_BIN` this branch's binary);
  `utils/check_release_voice.py` ok; `utils/check_gate_steps.py` ok.

**The stage continued** (folded arrows within a layer, R-33's probe, the review items), in the worktree, other
stages' worktrees running their tests beside it:

- `cargo test --profile gate -p ein-render -p ein-cli --no-fail-fast` (`EIN_TEST_JOBS=8`, beside the R-33 probe): 467
  tests in 77 binaries, 466 passing and the shape golden's failing on the lines the change moves (re-banked below, and
  passing); no other golden holds a folded view of a knowledge base.
- The shape golden re-banked, `EIN_BLESS=1 EIN_CORPUS_SLOW=1 … -p ein-render --test id_order_invariance` (82.7 s):
  29 027 lines, **2 063 moved against the first build, none added or removed: `dot[ir-fold]` 729,
  `dot[constraints-fold]` 682, `dot[kb-fold]` 652**; against the stage's start, the same 10 476 lines of the kinds
  above as before, no other kind.
- `EIN_CORPUS_SLOW=1 … --test dot_wellformed -- --nocapture`: **8 819 views of 888 files accepted, none refused**,
  1 205 past the 1 000-edge bound in 276 files; 796 of the structure's graphs; 611 slices; 360.6 s. The trace check
  alone, 1.8 s.
- Every trace of the corpus in both forms, the final binary against the first build: 1 776 of 1 776 the same blocks.
  The views of a knowledge base: the first build's put through `refold.py` against the final binary's (§ 7).
- `cargo clippy -p ein-render -p ein-cli --all-targets -- -D warnings`, `cargo fmt --all --check` and
  `RUSTDOCFLAGS="-D warnings" cargo doc -p ein-render --no-deps` clean.
- `utils/gen_stdlib_rule_pages.py` (511 rules, 493 canonical, 6 folded, 12 Ein only, as written: a rule's panel
  draws no layers), then `utils/render_diagrams.py` (the thirteen refolded drawings; `--check` 754 as they stand).
- `utils/check_doc_blocks.py` rules 1 to 5 hold; `utils/check_generated.py` 9 generators as written (`EIN_BIN` the
  final binary); `utils/check_release_voice.py` ok; `utils/check_gate_steps.py` ok; `utils/doc_audit.py --links
  --check --tracked`: 2 546 findings, every one a *file missing* under the review bundle's topics 01, 02 and 04 and
  P1f.21's `review/`, whose ignored outputs a worktree lacks; none under `03-drawings/`, whose outputs this worktree
  holds, and none elsewhere; 0 decision-id findings.

### Readings (defaults taken where the answers are silent; for the main session to file)

Each is built as its default and, where the user should see it, shown by an item of the re-created
`review/03-drawings/` (56 to 60).

1. **The layers are written, from the arrows alone**: a first arrow drawn back, `rankdir=TB`, and in a view of a
   knowledge base `rank=same` groups, objects and facts in turn, a band as deep as its facts nest, on the user's note
   (the words followed: *"global-only facts"* read as the held facts nothing points at, and *"nested"* as a fact
   under another's arrow, held as well or not); no name read; groups ordered by invisible anchors; no groups where the
   arrows go round; none in a rule's panel, a slice or the derivation graph. A relation under no argument arrow stands
   just above its highest fact, and a flat fact of it just above its names. *Default: as built*; the option, no
   groups and `dot` ranking by the arrows alone, which breaks the sandwich where facts nest. Item 56.
2. **A folded arrow runs within a layer**, as the continuing brief states: it ranks nothing (`constraint=false`) and
   its fact places its names as its octagon would, so the folded drawing keeps the canonical one's layers of objects;
   a drawing every fact of which folds is one row. This reverses the first build's reading (a folded arrow ranked as
   any arrow, which keeps chains as chains). *Default: within a layer*; the option, ranked. § 1, item 57.
3. **A rule's panel and a slice keep a folded arrow's `label`** and its ranking (Q-M1f.435 (a)); `constraint=false`
   with a `label` is what Graphviz could not route (R-31). *Default: as built.*
4. **A stored negation `(not g)` is a fact of two arrows** and folds into a loop on `g`'s octagon, in the KB views and,
   `(not P)`, in a rule's panel; no name is read. *Default: it folds*; the option, a negation never folded, which
   reads the name `not`. Item 58.
5. **The closing rules drawn folded**: with the loop they fall within `MAX_NODES` (unchanged, L-97), so five are drawn
   where they were Ein only. *Default: drawn folded*; the option, a closing rule left Ein only. Item 58.
6. **The derivation's palette**: eight dark colours by the rule's name; the derivation graph in the first; a step's
   border `penwidth=2` in place of `style=bold`; a redundant firing dashed at the same width. *Default: as built*;
   the option, one colour for every derivation. Item 59.
7. **A derived fact stays bold** (`penwidth=2`, its octagon and its arrows), the width of the derivation's arrows,
   told from them by colour. *Default: bold*; the option, the width the derivation's alone. Item 59.
8. **A step's slice under `--fold` is the canonical one**: every fact in it is touched by the step's arrows; `--fold`
   folds the `-F` snapshots. No page says otherwise; across the corpus every trace's folded blocks are its canonical
   ones (§ 5). *Default: as built.* Item 59.
9. **The copies of commands' outputs on the pages were rewritten by a validated transformer**, not by re-running
   each page's command, so that a cut stays the cut it was (Q-M1f.468); the hand derivations `09-cyclic` and
   `09-proof-graph` restyled by hand to the derivation's palette and width; `reasoning/01-static-ir/`'s hand graphs
   left. *Default: as done.*
10. **The `fdp` provisioning is left as it is** (the CI's `libgvplugin-neato-layout8`, `run_tests.sh`'s exit 127 where
    `fdp` lays out nothing, and the comments in `AGENTS.md`, the two workflows, `run_tests.sh`,
    `utils/ci_container.sh`, `utils/review_bundle.py` and `utils/review/util.py` that say every KB view suggests
    `fdp`): no view suggests it now, so the plugin guards no drawing the gate makes (`render_examples.sh` alone still
    picks `fdp`, for files named `*ontology*` or `*constraints*`). *Default: left*; what CI installs, and the
    comments, which are wrong from this branch on, are the main session's call.
11. **R-33's check belongs in `dot_wellformed`, per commit and bounded** (§ 5). *Default: as built*, two witnesses;
    the options, no check (the probe a script to run by hand) or every trace's blocks. Item 60.
12. **No page's `rank` pinning**: the rendering page says the layers are a preference no test holds, and the
    decision Q21 carries the user's rule that a test holds structure and never layout. *Default: as written.*
