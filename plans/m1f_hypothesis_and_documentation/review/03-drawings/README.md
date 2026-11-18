# Drawings

[← the bundle](../README.md)

**The user's second look.** At the first look this topic held six items on the views of `ein render` and the pages'
drawings; the user ruled how a fact is drawn (Q-M1f.419), what `ein render program` draws (Q-M1f.420), that `is-a`
and `relation` are ordinary relations (Q-M1f.421), that the trace folds on request (Q-M1f.422) and L-107's extent,
and P1f.23's S1f.23.1 drew every fact anew, in the code and on the pages. Four items went with those rulings (06,
10, 26, 27); 25 and 28 are re-taken; **34 to 55 are new**: the readings S1f.23.1 took where the rulings are silent,
each built with its default and put to the user, and its finding R-18.

## What the drawings are now

Today's binary, `build/review-bin/ein-head-17085b27`; the first look's, `ein-head-b5fb8d70`, is *before*.

- **A fact is drawn as the graph has it, in every view, by default**: an octagon with no name in it, its arrow `1` at
  its relation's atom and `2`, `3`, … at its arguments, each number the arrow's `xlabel` (34, 46). A relation is one
  box for every fact of it in a drawing.
- **`--fold`** (`ein render program|kb|rules|rule|constraints`, `ein solve --trace`) folds a fact of three arrows that
  no drawn fact points at into one arrow from its first argument to its second, the relation's name on it, as the
  arrow's `xlabel` in a drawing of a knowledge base and its `label` in a rule's panel and a trace's slice (36, 40, 46).
  Nothing else folds: a fact of two arrows is no loop.
- **No labelled octagon and no node labelled with a fact's text** in any view of facts, or on any page; `is-a` and
  `relation` are drawn as any relation; every atom is a box (44); what a fact rests on stands beside its octagon (35).
- **`ein render kb` draws every fact the knowledge base holds** (43), a stored negation a fact of `not`, a fact that
  applies a rule an octagon whose arrow `1` is at the rule's name; **`ein render program` draws three parts**, the
  program's `kb` by the KB view's code, a digraph per rule, hrule and macro, and its `query` (42); the constraint view
  draws its facts as facts (37); a rule's pattern is drawn as a fact, no macro read by its name (38).
- **A derivation** is each fact its octagon and each step a node of its own (39); the region, lattice and h-space views,
  which draw states and candidates, are as they were (41).
- **The pages**: 97 diagram sources drawn again or added and 4 removed, each a copy of the binary's output on the
  page's program (canonical, or folded where the page's point is the compact view, 47; whole, or a cut the page
  states, 48) or a hand drawing in the reasoning pages' vocabulary (49 to 54); the standard library's rule drawings
  regenerated, canonical, else folded, else Ein only (45, and 25).

Every drawing here is drawn with the layout its DOT suggests: a KB view's `// suggested layout: fdp` by `fdp`, every
other by `dot` ([`tools/draw.sh`](tools/draw.sh), which each item's `rerun.sh` sources); a page's copy keeps no such
line and is drawn by `dot`, as the page draws it (55). A `.MOCK.` file is a drawing edited by hand to show an option.

**A way through**: 34, 36 and 46 are the form itself; 35, 43 and 44 what a KB drawing holds; 42 and 37 the program
and constraint views; 38 to 40 rules and derivations; 41 and 50 what is left as it was; 47 to 54 the pages; 25, 45 and
55 what a large drawing does.

| # | item | the question | default |
|---|---|---|---|
| 25 | [L-97](25-L-97/README.md) | nine `div9-step` rules (and now `mul9-step`) have no drawing | the bound stays |
| 28 | [L-100](28-L-100/README.md) | the beliefs' two KB drawings have no test | as is |
| 34 | [Q-M1f.423](34-Q-M1f.423/README.md) | how a fact's arrows are numbered | (a) `1` at the relation, `2`, `3`, … at the arguments |
| 35 | [Q-M1f.424](35-Q-M1f.424/README.md) | where what a fact rests on is drawn | (a) beside its octagon, an `xlabel` |
| 36 | [Q-M1f.425](36-Q-M1f.425/README.md) | the fold's option, and what it folds | (a) `--fold`; a fact of three arrows nothing points at |
| 37 | [Q-M1f.426](37-Q-M1f.426/README.md) | what the constraint view draws | (a) each fact as a fact, declarations among them |
| 38 | [Q-M1f.427](38-Q-M1f.427/README.md) | how a rule's pattern is drawn, and R-15 (`forall`'s guard drawn as facts of `and` and `neq`) | (a) a pattern as a fact; no macro read by name |
| 39 | [Q-M1f.428](39-Q-M1f.428/README.md) | how a derivation is drawn | (a) each fact its node, each step a node |
| 40 | [Q-M1f.429](40-Q-M1f.429/README.md) | a folded fact in a derivation's drawing | (a) a labelled arrow through a point |
| 41 | [Q-M1f.430](41-Q-M1f.430/README.md) | the region, lattice and h-space views | (a) left as they are |
| 42 | [Q-M1f.431](42-Q-M1f.431/README.md) | the program view's three parts | (a) as built |
| 43 | [Q-M1f.432](43-Q-M1f.432/README.md) | what the KB view leaves out | (a) nothing |
| 44 | [Q-M1f.433](44-Q-M1f.433/README.md) | the shape of an atom | (a) a box |
| 45 | [Q-M1f.434](45-Q-M1f.434/README.md) | the standard library's rule drawings past the bound | (a) canonical, else folded, else Ein only |
| 46 | [Q-M1f.435](46-Q-M1f.435/README.md) | where an arrow's number and a folded arrow's relation are written: `xlabel`, not the user's literal `label="owns"` | (a) its `xlabel` |
| 47 | [Q-M1f.467](47-Q-M1f.467/README.md) | a command's output on a page: canonical or folded | (a) canonical; folded where the page's point is the compact view |
| 48 | [Q-M1f.468](48-Q-M1f.468/README.md) | a page's KB drawing: the whole view or a cut | (a) a cut, the page saying what it leaves out |
| 49 | [Q-M1f.469](49-Q-M1f.469/README.md) | how a derivation's drawing marks a terminal and the core | (a) double-bordered; the core red |
| 50 | [Q-M1f.470](50-Q-M1f.470/README.md) | which hand drawings the canonical form leaves as they are | (a) states, candidates, dependencies, schematics, design diagrams |
| 51 | [Q-M1f.471](51-Q-M1f.471/README.md) | `01-object-order`'s drawing | (a) unlabelled octagons, their arrows left out |
| 52 | [Q-M1f.472](52-Q-M1f.472/README.md) | the names of the drawings that changed form | (a) `canonical`, `folded` |
| 53 | [Q-M1f.473](53-Q-M1f.473/README.md) | programs a page draws that no file holds | (a) reconstructed from the page |
| 54 | [Q-M1f.474](54-Q-M1f.474/README.md) | an arrow in a hand schematic that is no arrow of the graph | (a) allowed where the page says so |
| 55 | [R-18](55-R-18/README.md) | KB views laid out by `dot` are wide strips; the binary's `fdp` hint is a comment | none recorded: the user's |

## Found while building

What the ledger does not say, each on its item's page under *Found while building*.

- **25**: the fold does not bring the nine back (194 nodes, 192 folded, against 120), and `mul9-step` lost its drawing
  too (129, 127 folded); `div9-step-0` lands on the bound exactly, 120 folded. No page says why the nine are Ein only.
- **28**: the drawings match the binary, after S1f.23.1 drew them again by hand a third time; the shape golden still
  digests the KB as loaded, `dot[kb-fold]` too, so nothing holds them.
- **34**: zebra2's whole KB, canonical, cannot be read at any size (1 446 numbered arrows, stars of a hundred arrows
  at `is-a` and `not`); the arrows' numbers and the frames' labels are in Graphviz's default serif font, the views
  setting `fontname` on nodes only.
- **35**: a commitment's fact in the KB view has no note and is drawn as derived (dashed; blue under `--colour-by
  origin`); no note has two parts, so *real lines* has no case; `claims` and `relation` share a colour.
- **36**: under `--fold` a pattern whose relation is a variable folds, and its diamond becomes a word on an arrow, so
  one variable is a diamond in one pattern and a label in another; `ein render lattice --fold` is refused, exit 2.
- **37**: the constraint view no longer draws an import, which its section of `12-rendering.md` does not say; under
  `dot` it is a two-rank strip.
- **38**: **a nested `absent` is drawn as a red pattern, as `(not P)` is, not as a cluster**: `(absent (and G (absent
  (not B))))` and `(absent (and G (not B)))`, two rules of different meaning, draw byte for byte alike; a `forall`'s
  guard stands in a frame named `and ctx` (R-15).
- **39**: **three drawings this question rules are reached by no command**: the derivation graph, a written trace's
  `dag` view, and a slice's `⊥` with its learned no-good (Q-M1e.20); no trace step draws a commitment's fact red,
  though the rendering page says so; `ein render program` of a written `(trace …)` draws none of its facts.
- **40**: a folded fact in a slice has its word on the first half of its arrow only; a slice is black where a `-F`
  snapshot is coloured by relation; tracing zebra2 writes 1 162 slices.
- **41**: these views draw byte for byte what the first look's binary drew; their shapes now mean other things than in
  the drawings of facts: the region view's reported state is a **filled octagon** with its facts' text, the h-space
  `exclusion` graph's candidate (a fact) a **box**. The (b) mock is three times as wide and loses the shape channel.
- **42**: `(neq …)` cannot be written at a program's top level (refused at the parse), though the ledger's reading and
  `12-rendering.md` name it among the checks the `kb` leaves out; the query clusters are all labelled `query`, not
  numbered; `add.ein`'s program view has no rule digraph, its rules being imported.
- **43**: **before the stage, zebra2's KB view left out eight of its fifteen given conditions** (each written as a fact
  that applies the rule `co-located`) and derived facts such as `(pet-owner Bob)`, each left out because its relation
  is named like a rule; (b) would leave them out again, and leaving its 264 facts out still does not make zebra2's
  view readable.
- **44**: a rule's view sets `shape=box` on each atom, the other views once as the node default.
- **45**: the fallback rescues one drawing, `div9-step-0`, by its last two nodes; across the library the fold saves
  some two nodes a rule; within the bound is not readable on a page either (a strip some 9 000 pt wide).
- **46**: **under `fdp` `label` and `xlabel` draw the same bytes, in half a second**; only `dot` cares, which the
  gate's DOT sweep and `render_diagrams.py` use: on `long_numbers`' folded `kb`, `dot` took 45 s with `xlabel`s, 339 s
  with every number and relation a `label`, and **did not finish in 2 400 s** with only the ten relations as `label`s
  (the ledger says *past a quarter of an hour*); the canonical view 26 s against 243 s (the ledger's 38 s and 360 s).
- **47**: every one of the 24 folded page drawings says it folds, but the generated `div9-step-0` in its own section.
- **48**: folded facts of one pair of atoms lie on one another: the bijection example's eight activators fold into
  eight arrows from `wears` to `is-a` whose labels make one unreadable band; the cut leaves them out.
- **49**: `09-core-in-derivation` lost the heavy outline of its two derived facts in the redrawing, where
  `02-derivation` and the binary's slices keep it; red means the core on the page and a commitment in the slices.
- **50**: the record's census reproduces file for file (108 flagged before, 43 after); the 9 labelled octagons left are
  all the region view's reported states.
- **51**: the mock with each fact's arrows makes the chain's atom `a` the facts' argument too, mixing the order and the
  graph, which the page keeps apart.
- **52**: the renames show in git as deletions and additions; `git log --follow` does not join the old and new names.
- **53**: both reconstructions reproduce their drawings byte for byte, and the written-out `bag-is-ready` saturates
  to the page's facts.
- **54**: the schematic's family names logic the knowledge base holds as activators of other names, so it and the
  binary's drawing of what the tag derives do not match one for one.
- **55**: the five page sources that keep `// suggested layout: fdp` are drawn by `dot` too; `dot -Tsvg` honours a
  `layout=fdp` attribute; zebra2's whole KB view reads in neither layout (16 104 pt wide by `dot`, 452 facts in a
  knot by `fdp`, 13 s).
