# S1f.23.1 — the drawing: the canonical form, the fold, and the program in three parts

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word (*"add a stage for drawing, implement this"*).
**Rulings:** [Q-M1f.419](../open_questions.md#q-m1f419--how-ein-render-draws-a-fact),
[Q-M1f.420](../open_questions.md#q-m1f420--what-ein-render-program-draws), L-107 in
[the milestone's findings](../findings.md); the user's words are in [the phase's README](README.md#the-instruction).
**Ends with** the user's second look at [`../review/03-drawings/`](../review/03-drawings/README.md).

## What is ruled

1. **A fact is drawn in the canonical form, by default, in every view**: an octagon with no name in it, its first
   arrow at its relation's atom, the next ones at its arguments, every arrow numbered. The relation is a node, one
   for every fact of it in the drawing, labelled with its name, `owns`.
2. **On request, a fact of three arrows**, a relation and two arguments, **is folded into one labelled arrow**, from
   its first argument to its second, `label="owns"`. Nothing else is folded.
3. **No labelled octagon** (the relation's name written in a fact's node, form 2) **and no text node** (a node
   labelled with a fact's text, form 4) is produced by any command, or drawn on any page under `docs/`.
4. **The KB view and the program view label and number alike**: a relation's name as it is written, `owns`, never
   `(owns)`; the arrows numbered with digits alone, never `#1`.
5. **`ein render kb` draws the whole knowledge base**, as today; **`ein render program` draws three parts**: the
   program's knowledge base, drawn as the KB view draws one, *"the same as KB"*, as one graph; its rules; its query.
   **The knowledge base is not split into ontology and facts.**
6. **L-107**: the extent's drawing (`reasoning/01-static-ir/diagrams/01-relation-extent.dot`) is the bundle's option 2,
   *"grey + octagon + color arrows"*: no frame; the extent drawn as the first arrows of its facts, coloured; $g$ kept,
   an octagon, grey. The page's sentence that names the frame is rewritten with it.

## Readings this stage takes (defaults, each put in `open_questions.md` from Q-M1f.421 and shown in the bundle)

- **The numbering**: the first arrow is `1`, at the relation, and the arguments `2`, `3`, … in order, as the
  reasoning pages number them (`01-canonical-graph.dot`, *counted from the first*). The user's *"number the arrows
  1/2"* is read as: the same digits in both views, no `#`. A folded arrow has no number.
- **What a fact's label carried** in the KB view (a source's short id, `by <rule>`) goes beside the fact's octagon
  (an `xlabel`), as real lines where it has two parts: no literal `\n` (so L-96 is fixed in passing, for every view
  this stage rewrites). Derived stays dashed, a state's new facts under `--since` stay bold, on the octagon and its
  arrows; colour stays by relation.
- **The fold's option**: one name on every subcommand that draws facts, `--fold` the suggestion; `--levi` goes, since
  what it asked for is now the default. A fact of two arrows (one argument) is not folded: no self-loop. A fact that a
  drawn fact points at is never folded: it is a node.
- **`is-a`**: in the canonical form an `is-a` fact is a fact like any other. Folded, it keeps its dashed arrow with an
  empty head. The atoms' shapes (box, oval) and `--no-types`/`--no-instances` are left as they are; the user's earlier
  remarks on `is-a` are still open.
- **The constraints view**: a fact about a relation is drawn as a fact, not written into the relation's node as
  `«symmetric»` (which is a fact's text in a node).
- **The rule view**: a pattern is drawn as a fact is, its relation's atom a node or, where the relation is a
  variable, the variable's diamond; guards, `absent` and `(not P)` keep their own marks, and a negation is drawn as
  the reasoning pages draw it (`01-context-negation.dot`).
- **The derivation views** (the trace's slice, the derivation graph, a written trace's views, `ein solve --trace`'s
  drawings): each fact as its node in the canonical form, each step a node of its own from its premises to what it
  concludes; no node labelled with a fact's text (L-99's mock, `../review/03-drawings/27-L-99/`).
- **The region, lattice and h-space views** draw states and candidates, labelled with their facts' text: read as a
  label of a state, not a drawing of a fact, and left as they are; put as a question, since the user may count them as
  form 4.
- **The program view**: the digraph `kb` holds every fact and declaration the program states, as written (macros as
  called, imports not resolved), drawn by the KB view's code; a digraph per rule (and hrule and macro), as `ein render
  rule` draws it; the digraph `query`, its queries and its `(solve …)` clause. A program with no rule, no import and
  no macro has a `kb` digraph equal to what `ein render kb` draws of it, but for what the loader adds: a test holds
  that.
- **The standard library's rule drawings**: the generator draws a rule in the canonical form; where that passes its
  bound (`MAX_NODES`) and the folded drawing does not, folded; else as Ein only, as today. Report how many rules each
  way, before and after (L-97 is still open).

## What changes

1. **The writers** (`git grep -l digraph -- ein.rs/crates`): `kb_dot.rs`, `ir_dot.rs` and `builder.rs`, `rules.rs`,
   `constraints.rs`, `dot_util.rs`, `derivation.rs`, `slice.rs`, `trace/*.rs`, and `ein-cli`'s `cmdline.rs` and
   `render.rs` (the option, the program view's parts). `structure.rs`, `region_dot.rs` and
   `dump/region_lattice.rs` are read and left unless they draw a fact.
2. **The tests**: `every_object_once.rs` (no octagon with a name, no node with a fact's text, in every view and in
   both forms), `render_doors.rs` (one program in both forms), `dot_wellformed.rs` (its 1 000-edge skip and
   `SKIPPING`: the canonical form has more edges), the program view's invariance against the KB view above, every
   golden a view writes, and the shape golden re-banked whole (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`), every moved line a
   `dot[…]` line.
3. **The drawings of the tree**: every generated one drawn again by its generator (the rendering page's `12-*` from
   the binary, the standard library's rule drawings by `utils/gen_stdlib_rule_pages.py`, the beliefs' KB drawings by
   their line in `examples/README.md`); every hand-written `.dot` under `docs/` that draws form 2 or form 4 (a census
   by the main session found 3 under `reasoning/01-static-ir/`, 13 under `02-kb-evolution/`, 1 under
   `03-rule-evolution/`, 8 in the guide, 20 under `03-syntax-and-semantics/`, 1 under `ein.rs/08-reasoning-in-code/`)
   drawn again in form 1, or in form 3 where it folds a fact of three arrows that nothing points at and the page says
   it does (Q-M1f.382); `utils/render_diagrams.py` draws the pictures.
4. **The pages**: `12-rendering.md` (the legend, *Hyperedge encoding*, every view's section, the KB table), the
   reasoning's `01-knowledge-base.md` (§ *Drawn with labelled arrows*, the extent and L-107's sentence), `03-facts.md`,
   `01-objects.md` (*As a graph*), and every page whose words say *the relation's name in the octagon* or describe a
   text node; `05-cli.md` and `06-config.md` for the option; `ein.rs/`'s pages on the renderers.
5. **The decisions**: new files for Q-M1f.419 and Q-M1f.420; Q1 (the Levi-bipartite encoding), Q-M1f.382 (the
   labelled arrow, now the fold), Q-M1f.395 (every object once, with no exception for a derivation's drawing),
   Q-M1f.398 and Q21 rewritten where they say otherwise, their slugs with their titles and every citation moved.
6. **The review bundle**, last: a binary built at the stage's last code commit, copied to `build/review-bin/`; every
   item's re-run pointed at it for *today* (the earlier binaries stay *before*) and run again; every `.MOCK.` drawing
   made again in the forms now ruled; every README that describes a drawing brought to what is drawn, and
   `03-drawings/`' six pages re-taken whole: 06 (L-107, ruled, built), 26 (L-98) and 27 (L-99), answered by these
   rulings, 10 (L-96), 25 (L-97) and 28 (L-100) re-taken; the bundle's READMEs committed, its outputs still ignored.

## Acceptance

- No answer moves: of the shape golden only `dot[…]` lines move.
- Every view of every corpus program: no octagon with a name in it, no node with a fact's text, every relation of a
  drawn fact a node under first arrows, in both forms (a test).
- `./run_tests.sh` exits 0 on the last code commit; `utils/ci_container.sh` exits 0 on it (Q-M1f.77: the DOT writers
  change); `./run_tests.sh --nightly` exits 0 on it (the every-program sweeps draw bigger views; a nightly step past
  five minutes on the runner is reported, Q-M1f.293).
- `python3 utils/doc_audit.py --links --check --tracked`, `utils/check_doc_blocks.py`, `utils/check_generated.py`
  green.

## Record

Not started.
