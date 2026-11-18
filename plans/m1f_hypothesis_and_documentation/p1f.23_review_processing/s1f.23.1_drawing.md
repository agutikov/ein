# S1f.23.1 — the drawing: the canonical form, the fold, and the program in three parts

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word (*"add a stage for drawing, implement this"*).
**Rulings:** [Q-M1f.419](../open_questions.md#q-m1f419--how-ein-render-draws-a-fact),
[Q-M1f.420](../open_questions.md#q-m1f420--what-ein-render-program-draws), L-107 in
[the milestone's findings](../findings.md); the user's words are in [the phase's README](README.md#the-instruction).
**Ends with** the user's second look at `review/03-drawings/` (answered by the user 2026-11-18 and removed).

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

## Readings this stage takes (defaults, each put in `open_questions.md` from Q-M1f.423 and shown in the bundle; the user's rulings during the stage are Q-M1f.421 and Q-M1f.422)

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
- **`is-a` and `relation`: ruled by the user, 2026-11-18, during the stage** (Q-M1f.421), replacing this reading:
  *"`is-a` - is just ordinary relation, no special dashed arrows, nothing special, completely ordinary relations. even
  `relation` relation is ordinary relation"*. No drawing gives either a look of its own, in either form: no dashed
  arrow with an empty head, no box for what an `is-a` fact points at and no oval for what it starts from, every atom
  one shape; a declaration is the fact it is, an octagon whose first arrow is at `relation`, or folded. `--no-types`
  and `--no-instances` go. The reads of `is-a` outside the drawings are the phase's findings R-1 to R-3.
- **The trace's drawings: ruled by the user, 2026-11-18, during the stage** (Q-M1f.422): *"trace can also draw folded
  relations if --fold flag provided;"*. `ein solve` takes `--fold` for the trace's drawings, its slices and its `-F`
  snapshots, canonical by default.
- **No theory's name in the drawing code: the user's general ruling, 2026-11-18, during the stage** (filed by the
  main session): *"ein.rs must expect the minimal shape of the program: rules, relations, ... Not special names like
  `is-a` `type` `instance` `symmetric` `bijection` `slot`"*. The writers this stage touches read no such name.
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

### Task 1 — the writers, the tests, the generated drawings, the decisions (2026-11-18)

**What changed, by writer** (`ein.rs/crates/ein-render/src/`):

- **`fact_graph.rs`, new**: a set of facts drawn as the graph has it, the one drawing of facts the KB view, the
  program view's `kb` and the constraint view share. A fact is an octagon, `label=""`, 0.3 in, its arrow numbered
  `1` at its relation's atom and `2`, `3`, … at its arguments, each number the arrow's `xlabel`; folded, a fact of
  three arrows that no drawn fact points at is `a -> b [xlabel="rel note"]`, an `xlabel` too (Q-M1f.435); a fact a
  drawn fact points at is its node, framed where not held, as before (`ctx_frames`). What a fact rests on is its
  octagon's `xlabel` (Q-M1f.424).
- **`kb_dot.rs`**: onto `fact_graph`; every fact the KB holds is drawn, a stored negation and a fact that applies a
  rule among them (Q-M1f.432); `KbDotOpts` loses `include_types` and `include_instances` and gains `fold`; no
  `is-a` read, no `// types` and `// instances`; every atom a box, the node default (Q-M1f.433); `ranksep=0.55`.
- **`ir_dot.rs`**: the program view in three parts (Q-M1f.420, Q-M1f.431): `kb` by `fact_graph` from the parsed
  forms (`written_fact_set`, the loader's order), a digraph per rule, hrule and macro, the digraph `query` (each
  goal a cluster drawn by the rule view's code, its clauses a note; `(solve …)` a note); the wrapper forms
  (`ontology`, `facts`, `reasoning`) are ordinary facts; the `=` class node, the `is-a` arrow and `--levi` go; a
  written trace's `dag` view draws each fact its node and each step a node (Q-M1f.428).
- **`rules.rs`**: a pattern drawn as a fact, its relation a box per panel or the variable's diamond; `(not P)` a
  red fact of `not` pointing at `P`'s node; `()` draws nothing; `fold`. **`std.macro`'s `forall` is no longer read
  by its name** (Q-M1f.427; S1f.23.4's renaming-invariance test caught the read, `dot[rules]` and
  `dot[rules-overlay]` moving under a renaming): `head_name` no longer maps `std.macro.forall` to `forall`, and
  `is_guard` is `absent` alone, so a call of `forall` is drawn as the form it is, a pattern whose first arrow is at
  the box `forall`; nothing about it needed the name. The `∀` cluster goes; `absent` keeps its cluster, a word of
  the kernel's.
- **`constraints.rs`**: each structural fact drawn as a fact by `fact_graph`; `KERNEL_ONTOLOGY_HEADS` and its
  `type` and `instance` dropped on the user's ruling on theory names, and `relation` with them, read as what the
  loader stores (Q-M1f.426); the `«property»` badges, which wrote a literal `\n`, go.
- **`slice.rs`**: each fact its octagon, seeds red and negations grey with no fill, nested facts framed where the
  state lacks them, the no-good a note pointing at its facts; under `fold` a fact of three arrows is a labelled
  arrow through a point (Q-M1f.429). **`derivation.rs`**: each fact its octagon with its provenance as an `xlabel`,
  a diamond per derivation. **`trace/`**: `Shape.fold`, `Diagrams { drawn, fold }` to the slices and the `-F`
  snapshots (Q-M1f.422). **`builder.rs`**: what the per-step trace view needs, no octagon. **`dot_util.rs`**:
  `TYPE_SHAPE`, `INSTANCE_SHAPE`, `EQUALITY_SHAPE` go, `GROUND_SHAPE` is `box`. **`shape.rs`**: the views `ir-fold`,
  `rules-fold`, `constraints-fold` and `kb-fold`; `ir-levi`, `kb-no-types` and `kb-no-instances` go.
- **`ein-cli`**: `--fold` on `render program|kb|rules|rule|constraints` and on `solve` (for `--trace`); `--levi`,
  `--no-types` and `--no-instances` go; `--fold` is among the flags the header leaves out.

**The readings taken** (each in `open_questions.md`, default (a)): Q-M1f.423 the numbering, Q-M1f.424 the notes
as `xlabel`s (L-96 gone with them), Q-M1f.425 the option, Q-M1f.426 the constraint view, Q-M1f.427 the rule view,
Q-M1f.428 the derivations, Q-M1f.429 a folded fact in a derivation, Q-M1f.430 the region, lattice and h-space views
left, Q-M1f.431 the program view's parts, Q-M1f.432 the KB view leaves nothing out, Q-M1f.433 an atom a box,
Q-M1f.434 the standard library's fallback, Q-M1f.435 the numbers as `xlabel`s. The user's two rulings during the
task are Q-M1f.421 (`is-a` and `relation`) and Q-M1f.422 (the trace folded).

**The shape golden**, re-banked whole (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`, `id_order_invariance`): 28 480 lines before,
29 267 after. **Moved, 7 979**: `dot[ir]` 784, `dot[ir-overlay]` 784, `dot[ir-forms]` 784, `dot[ir-trace-dag]` 786,
`dot[constraints]` 787, `dot[kb]` 671, `dot[kb-origin]` 671, `dot[kb-none]` 671, `dot[kb-since]` 671, `dot[rules]`
428, `dot[rules-overlay]` 428, and `region-trace` 514, the trace digested with its slices' `dot` blocks: 513 of them
are, by `ein solve FILE --trace` of the binary before the stage and of this one with every `dot` block cut out, the
same bytes, and the 514th, `examples/ein-bugs/closed-and-owing.ein`, is refused at load by both binaries (the op
reads it past the refusal). **Added, 3 148**: `dot[ir-fold]`, `dot[kb-fold]`, `dot[rules-fold]`,
`dot[constraints-fold]`, 787 each. **Removed, 2 361**: `dot[ir-levi]`, `dot[kb-no-types]`, `dot[kb-no-instances]`,
787 each. No line of any other kind moved: `load`, `saturate`, `plan`, `match`, `naf`, `explain`, `read-out`,
`cores`, `states`, `oracle`, `ir[…]`, `region-lattice` are as they were.

**The standard library's rule drawings** (`utils/gen_stdlib_rule_pages.py`, Q-M1f.434), of 511 rules: before, 495
drawn and 16 as Ein only; after, **493 canonical, 1 folded** (`std.math.num9`'s `div9-step-0`) **and 17 as Ein
only** (`mul9-step` joins the sixteen: 120 nodes is the bound, and its folded drawing is past it too).

**The other generated drawings**: the rendering page's eight `12-*.dot`, from the binary on the page's three
programs (`12-program-ontology` and `12-program-facts` go, `12-program-kb` and `12-program-kb-fold` come), and
`examples/beliefs/0{1,2}_*.kb.dot` by their line in `examples/README.md`; `render_diagrams.py` drew every picture.

**`dot_wellformed.rs` against its bound** (1 000 edges): every fact an octagon with an arrow to its relation and
to each argument, three directories newly hold a view past the bound, each a saturated knowledge base, `kb-since`,
and are named in `SKIPPING` with why: `examples/` (`zebra.ein` 2 321 edges, `zebra2.ein` and
`zebra2-obligations.ein` 1 446, `zebra2-hints.ein` 1 482, `zebra2-minus-15.ein` and its `-obligations` twin 1 202),
`examples/ein-bugs/` (`zebra2-bad.ein` 1 741, `zebra2-hints.ein` 1 007) and, under `EIN_CORPUS_SLOW`,
`tests/stdlib/math/binary/` (its three programs, 1 256 to 1 289); no other directory changed (a sweep of every
view's edges over every corpus program, `tests/` included). The sweep outside `tests/`: 6 295 views accepted, 384
skipped past the bound in 83 files, no view refused, green in 354 s on this machine beside two other test runs. Its tail
is `examples/arith/long_numbers.ein`, whose seven parse views take `dot` some 25 to 50 s each, as its program view
took 142 s before the stage. **The view that sat on one layout** (the run S1f.23.4 saw past half an hour) was
`examples/arith/long_numbers.ein [ir-fold]`, and `[constraints-fold]` with it, while a folded arrow's relation was
its `label`: ten labelled arrows among 752 made `dot` lay every arrow over two ranks, 204 ranks and 82 524 nodes in
its position phase (`dot -v`), past a quarter of an hour; with the relation as the arrow's `xlabel`, as every number
is, 51 s (Q-M1f.435). It is the labels and not the count of arrows, so the 1 000-edge bound is kept as it counts.

**Pages left for task 2** (their words say the relation's name is in a fact's node, describe a text node, the
ontology/facts digraphs, `--levi`, `--no-types`, the `=` class node or the `is-a` convention):

- the reasoning: `README.md` (the pages draw with labelled arrows, a fact of one argument a loop),
  `01-static-ir/01-knowledge-base.md` (§ Drawn with labelled arrows: the relation's name in the octagon, the loop;
  the extent and L-107's sentence), `01-static-ir/02-rewriting-rules.md` (drawings *with labelled arrows* by
  `ein render rules`), `02-kb-evolution/01-firing.md`, `02-kb-evolution/03-absent-and-naf.md` (a loop),
  `02-kb-evolution/04-hypothesis-generation.md` (the legend: a box a label, an oval an atom, a dashed arrow with an
  open head a membership), `03-rule-evolution/01-rules-as-data.md`;
- the guide: `01-objects-and-relations.md` (`--levi`, Levi-bipartite), `05-jack-drinks-coffee.md`;
- syntax and semantics: `01-objects.md` (§ As a graph: compact and `--levi`, the table, `--no-types`, the box and
  oval, the `=` `doublecircle`), `02-relations.md` (the relation a word in the octagon or on the arrow, the loops),
  `03-facts.md` (an octagon with the relation's name in it), `04-ontology.md`, `05-rules.md` (a pattern of two
  arguments a labelled arrow, of one an octagon over its argument), `07-properties-and-operators.md` (the property
  facts as self-loops, its picture);
- the reference: `02-syntax.md` (a fact of `=` an equality node, a `doublecircle`), `13-glossary.md` (§ Hyperedge,
  Levi-bipartite and its `--levi`; an octagon with its relation's name; `--no-types`), `04-stdlib/README.md` (§ How a
  rule is shown: a fact of two arguments an arrow, and the folded fallback to say), `04-stdlib/01-macros.md`
  (`forall` drawn as a dashed cluster `forall (∀)`, now a pattern as called; an octagon `marked`),
  `04-stdlib/10-positional-rules.md` (an octagon labelled with the parameter);
- `ein.rs/`: `08-reasoning-in-code/02-saturation-and-negation.md` (the view draws declarations as `(relation)`
  octagons).

**Hand-written diagrams left for task 2** (a census of the sources under `docs/` by what they draw; the region,
lattice, cube and h-space drawings, which label states and candidates, are Q-M1f.430's and not counted): a labelled
octagon in `reasoning/01-static-ir/` 2 (`02-absence-nobody-home`, `02-domain-elimination`, both from `ein render`),
`reasoning/02-kb-evolution/` 5 (`02-rule-note`, `03-rule-soup-unless-allergic`, `04-rule-total-owed`,
`10-rule-guest-owes-seat`, `10-rule-one-seat`), `reasoning/03-rule-evolution/01-instance-of-instance`, the guide 8
(`01-declaration`, `01-fact-levi`, `01-tiny-program`, `03-rule-disjunctive-prune-fwd`, `05-jack-levi`,
`07-zebra-fragment`, `08-problem-facts`, `08-rule-gave-means-less`), syntax and semantics 20 (`01-fact-levi`,
`01-program-kb`, `01-rule-every-declared-pair`, `01-rule-first-argument-fits`, `02-program-as-written`,
`02-program-kb`, `03-program-kb`, `05-rule-no-child`,
`05-rule-owns-a-pet`, `06-rule-cat-not-in-b2`, `06-rule-one-box-per-pet`, `07-rule-asymmetric`,
`08-rule-everyone-sits`, `08-rule-total-owed`, `09-rule-one-chair-each`, `09-rule-one-person-a-chair`,
`10-rule-arith`, `10-rule-division-by-zero-is-false`, `11-rule-bag-is-ready-with-forall`,
`11-rule-bag-is-ready-written-out`), the standard library's reference 3 (`01-rule-complete-via-expansion`,
`01-rule-complete-via-macro`, `01-rule-third-state`); the `is-a` arrow or an oval in
`reasoning/02-kb-evolution/` (`01-before-firing`, `01-after-firing`, `02-negative-completion`, `03-soup-saturated`,
`04-blind-enumerator`, `04-root-and-payments`), the guide (`02-transitive-fired`, `05-jack-compact`), syntax and
semantics (`01-is-a-convention`, `04-facts-about-anything`, `04-program-kb`), `04-stdlib/` (`02-operators-state`,
`04-elimination-state`); a badge, `07-constraints-view`; a node labelled with a fact's text in
`reasoning/01-static-ir/` (`01-object-order`, `02-gluing-square`, schematics of objects and of the square),
`reasoning/02-kb-evolution/` (`01-two-layers`, `02-derivation`, `02-resume`, `04-ladder`, `07-leaves`,
`08-constructions`, `09-core-in-derivation`, `09-cyclic`), the guide (`03-bijective-family`, `04-zebra2-pieces`),
syntax and semantics (`03-first-firing`, `05-firing`); and the drawings `ein render rules` made of three-houses
(`reasoning/01-static-ir/diagrams/02-pet-follows-owner*.dot`), now canonical by default. The census is a `grep` of
the sources (a labelled octagon, `arrowhead=empty`, `shape=oval`, `«`, `label="#…"`, a label that is a fact's text),
and task 2's own reading decides each.

### Task 2 — the hand-written drawings and the pages under `docs/` (2026-11-18)

In the worktree `build/wt-s1f231t2`, branch `s1f.23.1-task2`, on `1418fba5`. Every machine drawing is of the binary
`build/review-bin/ein-head-8edf546b` (task 1's commit). **97 diagram sources drawn again or added, 4 removed**, every
picture drawn by `utils/render_diagrams.py`; 44 pages, a decision and a design page reworded. Nothing under
`ein.rs/`, `examples/`, `tests/` or `stdlib/` changed, so no answer and no shape-golden line moves.

**How each drawing was drawn again** (the readings below say why):

- **a copy of a command's output** (`rule_*_lhs_rhs`, `kb`, the old `ontology`/`facts` digraphs, `constraints`,
  a trace's `stepN`): drawn again by the binary from the page's own program, the command as the page writes it,
  `//` comment lines and blank lines dropped and nothing else changed. Each program was checked first: the old
  drawing equals `--fold`'s output today but for what the rulings changed (a labelled octagon, `rectangle`/`oval`
  atoms, the empty-headed `is-a` arrow, `#n`, `¬rel`, the `#empty` octagon), so the program taken is the one drawn.
- **a hand drawing of facts**: in the reasoning pages' vocabulary (`01-canonical-graph.dot`): every atom a box, a
  fact an octagon with `label=""`, its arrow `1` at its relation and `2`, `3`, … at its arguments; folded only where
  a fact has three arrows, nothing points at it and the page says the drawing folds; no fact of two arrows a loop.
- **a derivation** (a firing, a proof graph): as the trace's slice draws one, each fact its octagon, each firing a
  node of its own from its premises to what it concludes, `(false)` an octagon at the box `false`.

**By folder** (sources; the pictures with them):

| folder | drawn again | how |
|---|---|---|
| `reasoning/01-static-ir/` | 6 | `01-relation-extent` (L-107, below) and `01-object-order` by hand; `02-absence-nobody-home`, `02-pet-follows-owner`, `02-pet-follows-owner-overlay`, `02-domain-elimination` byte for byte `ein render rule[s]` of the three-house example (`ein.rs/08-reasoning-in-code/01-static-ir.md`'s source) and of `stdlib/bijection.ein`, canonical |
| `reasoning/02-kb-evolution/` | 17 | the rule drawings (`02-rule-note`, `03-rule-soup-unless-allergic`, `04-rule-total-owed`, `04-rule-guess-drink`, `10-rule-guest-owes-seat`, `10-rule-one-seat`) byte for byte `ein render rule --fold`; the KB drawings (`01-before-firing`, `01-after-firing`, `02-note-fixpoint`, `03-soup-saturated`) `ein render kb --fold`, cut; `02-negative-completion`, `04-root-and-payments`, `04-blind-enumerator` by hand, folded; `02-derivation`, `09-proof-graph`, `09-cyclic`, `09-core-in-derivation` as slices, each fact its octagon. `01-rule-neighbour` was already `--fold`'s output byte for byte |
| `reasoning/03-rule-evolution/` | 1 | `01-instance-of-instance`, the whole of `ein render kb --fold` |
| `user/02-guide/` | 25 | the rule drawings (`02-rule-*`, `03-rule-*`, `04-rule-guess`, `07-rule-*`, `08-rule-gave-means-less`) `ein render rule`, canonical; the KB drawings `ein render kb`, whole or cut, `--fold` for `07-zebra-fragment` and `05-jack-folded`; chapter 1's fact both ways, `ein render program` and `--fold`; `01-is-a-facts` and `03-bijective-family` by hand. Renamed with `git mv`: `01-fact-levi` → `01-fact-canonical`, `01-fact-compact` → `01-fact-folded`, `05-jack-levi` → `05-jack-canonical`, `05-jack-compact` → `05-jack-folded` |
| `user/03-syntax-and-semantics/` | 40, 2 added, 4 removed | every drawing a page presents as a command's output, byte for byte the binary's from that page's program, canonical: ~30 rule drawings, the `kb` drawings, the program view's `kb` (`02-program-as-written`, `04-facts-about-anything`, `07-property-facts`), the constraint view (`07-constraints-view`, no `«badge»`), the slices `03-first-firing`, `05-firing`, `10-div-by-zero-flagged` and `10-div-by-zero-refuted`; added `01-program-kb-fold` (`ein render kb --fold`, the page's point) and `12-query-view` (the program view's digraph `query`); removed `01-fact-levi`, `01-is-a-convention`, `12-solve-form`, `12-query-form` |
| `user/04-reference/04-stdlib/` | 6 | `01-rule-complete-via-macro`, `01-rule-complete-via-expansion`, `01-rule-third-state` byte for byte `ein render rule` of `tests/stdlib/macro/01_forall.ein` and `02_unknown.ein`, canonical (`forall` and `unknown` patterns as called, their arguments framed); `02-operators-state`, `04-elimination-state`, `05-row-state` `ein render kb --fold`, cut to the relations the page is about |

**L-107** (`reasoning/01-static-ir/diagrams/01-relation-extent.dot`), as ruled: no frame; the first arrows of
$f_3$, $f_4$ and $g$ coloured (`#1f77b4`, `penwidth=2`); $g$ an octagon, grey, its argument arrows grey. The
page's sentence now reads *the first arrows of $f_3$, $f_4$ and $g$, drawn coloured, meet at the atom left-of: they
are the part of the extent drawn*.

**Pages reworded**, to what is drawn (no relation's name in an octagon, no loop for a fact of two arrows, no text
node, no `is-a` convention, no `--levi`, `--no-types`, `ontology`/`facts` digraphs, `«badge»`, `#1`, `=`
`doublecircle`; a page whose drawing folds says so):

- reasoning: `README.md` (how the pages draw); `01-knowledge-base.md` (L-107's sentence; § Drawn with labelled
  arrows: the loop row and the labelled-octagon rows gone, the closing paragraph says the later drawings are of the
  graph and one that folds says so, no oval or box for `is-a`; § the order's facts in TeX); `02-rewriting-rules.md`
  (the four engine drawings described canonical, `forall` a pattern); `01-firing.md`, `02-saturation.md`,
  `03-absent-and-naf.md`, `04-hypothesis-generation.md`, `10-satisfiability.md` (each says its drawings fold, and
  what a cut leaves out); `09-cores.md` (the proof graph's facts as octagons, the core red rather than filled);
  `03-rule-evolution/01-rules-as-data.md`;
- `ein.rs/08-reasoning-in-code/`: `02-saturation-and-negation.md` (§ 1.4, § 2.4, § 3.4, § 4.5: `--fold` in the
  commands, the `(relation)` octagons gone, what each drawing leaves out), `03-hypotheses-and-the-answer.md` § 1.2
  and `04-rule-evolution.md` (`--fold` in the commands);
- the guide: `README.md` (a command's drawing is its output, maybe cut, canonical unless `--fold`), `01` (the fact
  both ways, canonical first; the declaration; every atom a box), `02`, `03`, `04`, `05` (§ As a graph), `07`, `08`;
  no `lisp` or transcript block changed;
- syntax and semantics: `README.md`, `01-objects.md` (§ As a graph rewritten: the KB view, then `--fold`; the
  `--levi` table, the box and oval and the `doublecircle` gone), `02-relations.md`, `03-facts.md`, `04-ontology.md`,
  `05-rules.md`, `06-constraints.md`, `07-properties-and-operators.md`, `08-obligations.md`, `09-hypotheses.md`,
  `10-theory.md` (its trace excerpt renumbered, steps 15 and 16, finding 5), `11-macros.md`, `12-query.md`; no `lisp`
  block changed, `14-natural-language.md` untouched;
- the reference: `02-syntax.md` (a fact of `=` drawn as any fact), `13-glossary.md` (§ Fact; § Type, no
  `--no-types`; § *Hyperedge, Levi graph*, renamed from *Levi-bipartite*, the drawing as the graph);
  `04-stdlib/README.md` (§ How a rule is shown: a pattern an octagon, the folded fallback, Ein alone),
  `01-macros.md` (`forall`, `unknown` and the expansion as drawn), `02-relation-algebra.md`, `04-bijections.md`,
  `05-slots.md` (the state drawings, folded, and what each leaves out), `10-positional-rules.md` (a premise headed by
  a parameter: its arrow `1` at the diamond), `17-nine-digit-numbers-rules.md` (`mul9-step` as Ein alone,
  `div9-step-0` folded);
- `ein.rs/03-design/01-determinism-and-order.md` (the KB view no longer sorts names as types and instances);
  [Q-M1f.372](../../../docs/decisions/language/Q-M1f.372-ein-has-no-types-a-declarations-slot-names-are.md) (no
  drawing marks a label; `--no-types` gone).

**The readings task 2 took** (defaults, for the main session to file):

1. **A copy of a command's output is the binary's**, as the page writes the command, canonical by default. Folded
   where the drawing's point is the compact view, and the page writes `--fold` (outside `reasoning/`) or says the
   drawing folds (in it): the guide's and syntax pages' fact shown both ways, `07-zebra-fragment`, every rule and KB
   drawing of `reasoning/02-kb-evolution/` and `03-rule-evolution/` (folded before, now `--fold`'s output byte for
   byte), the standard library's three states. `reasoning/01-static-ir/`'s four engine drawings canonical.
2. **A KB drawing may be a cut of the view**: the facts of the relations the section is about, every fact they point
   at with them, a fact in a frame kept only with a fact that points at it; the page says what is left out (the
   declarations' facts, the facts that apply a rule, the stored negations). Q-M1f.368's *a drawing of the engine's
   is kept as the file the command wrote* is read as: a whole view is kept as written; a cut is stated on its page.
   Whole where it reads (`01-instance-of-instance`, `07-zebra-fragment`, the syntax pages' `kb` drawings).
3. **A derivation's terminal is double-bordered and its core red**, where it was filled (Q-M1f.398: no octagon
   filled); a fact's text is never its label nor its `xlabel`.
4. **Left as they are** (Q-M1f.430's reading, extended to every drawing that is not of facts): the region,
   lattice, cube and h-space drawings (`06-*`, `07-*`, `08-cube-*`, the seven `08-region-*`, `09-upset`,
   `10-branch`, `03-dead-not-upward-closed`, `05-cover`, `05-exclusion-graph`, the syntax pages' four `*-region-*`
   and `09-exclusion-chairs`, `ein.rs/08`'s `03-single-run-on-owed-seats`); the states and constructions
   (`01-two-layers`, `02-two-phases`, `02-resume`, `03-boundary`, `08-constructions`, `08-fifo-gap`,
   `08-state-and-status`, `08-path-tree-and-cube`, `10-rule-and-claim`); the dependency graphs (`03-strata`,
   `08-reads`, `02-dependency`, `02-parity`, `10-theory-dependencies`); the schematics and flowcharts
   (`02-four-levels`, `02-gluing-square`, `01-main-steps`, `04-ladder`, `03-reading`, `01-the-line`, the guide's
   `04-zebra2-pieces`); `ein.rs/`'s design diagrams. Task 1's list named `01-two-layers`, `02-resume`, `04-ladder`,
   `07-leaves`, `08-constructions`, `02-gluing-square` and `04-zebra2-pieces` as text nodes: each labels a state, a
   graph or a form, not a fact, and is left by this reading.
5. **`01-object-order`** draws its six facts as unlabelled octagons named $f_1$ to $f_6$ in the chain $\prec$, their
   arrows left out, and the page gives each fact in TeX.
6. **Renamed or merged**: a drawing named for a form that is gone takes the form's new name (`levi` → `canonical`,
   `compact` → `folded`); `01-is-a-convention` is dropped with its paragraph rewritten (drawn again it would repeat
   `01-program-kb`); the two `12-*-form` drawings are one, `12-query-view`, since the program view writes both forms
   in its digraph `query`.
7. **Two programs reconstructed**, since no file holds them: `guess-drink`'s (the `is-a` and `drinks` declarations
   and the hrule; its `--fold` output equals the old drawing but for two atoms' look) and the written-out
   `bag-is-ready` (the page's program with `forall`'s expansion, as the page describes; it loads and answers
   `(model (ready Red))`).
8. **A hand schematic may carry one arrow that is no arrow of the graph**, where its page says so
   (`03-bijective-family`: the fact `(bijective color-loc)` an octagon, and a dashed arrow to the logic it switches
   on).

**Findings** (not fixed):

1. **`forall`'s arguments in a rule's drawing.** With `forall` drawn as a pattern read by no name (Q-M1f.427), its
   guard is drawn as nested facts: `std.bijection`'s `domain-elimination` shows `(and (?isa …) (neq ?b_other ?b))`
   as octagons of `and` and `neq` in frames, where a `neq` of a match is a dotted `≠` link and the language says
   `and` and `neq` are never a nested head. The user's call, with Q-M1f.427.
2. **Corpus comments describe the old drawing**: `examples/syntax/equality.ein` (*an arity-2 `=` draws a
   `doublecircle`*), `examples/syntax/unary-and-nary.ein` (*the renderers' unary special case*), and
   `examples/syntax/is-a-shapes.ein` calls `is-a` *the kernel `is-a` head* (Q-M1f.436). Outside `docs/`, left.
3. **`ein-render/src/rules.rs`' module comment** still says *negation in red with a `¬` prefix*; the `¬` went with
   the labelled octagon.
4. **A KB view laid out by `dot` is a wide strip of two ranks with its arrows' numbers crowded** where facts share
   atoms (`03-syntax-and-semantics/diagrams/05-program-kb`, `12-kb-view`, `02-guide/diagrams/05-jack-canonical`):
   the binary suggests `fdp` in a comment, which a source drops and `render_diagrams.py` does not read.
5. **`10-theory.md`'s trace excerpt had drifted before this stage**: it numbered the steps 12 and 13, where the
   binary of `b5fb8d70` already wrote 15 and 16; renumbered with the slices.
6. **No file holds `guess-drink`'s program**; `ein.rs/08-reasoning-in-code/03-hypotheses-and-the-answer.md` § 1.2
   says only that it is drawn the same way by its name.
7. **Two hand drawings of the syntax pages, `03-facts-in-contexts` and `05-rule-in-a-context`, are not said to be
   by hand**, which the folder's README asks of a drawing no command makes.
8. `reasoning/02-kb-evolution/diagrams/10-key.dot` (`--models key`) is S1f.23.2's and untouched.

**The census**, by `gvpr` over every `.dot` under `docs/` but `docs/lib/` and the generated rule drawings
(`04-stdlib/diagrams/NN-std.*`, which `utils/check_generated.py` holds): a labelled octagon, a node labelled with a
fact's text, an `oval`/`ellipse`/`rectangle`/`doublecircle` node, an empty arrowhead, an arrow numbered `#n`, a `«`:

```sh
rev=HEAD   # or WORKTREE for the files on disk
list() { if [ "$rev" = WORKTREE ]; then git ls-files -co --exclude-standard 'docs/*.dot'; else git ls-tree -r --name-only "$rev" docs | grep '\.dot$'; fi; }
src() { if [ "$rev" = WORKTREE ]; then cat "$1"; else git show "$rev:$1"; fi; }
list | grep -v '^docs/lib/' | grep -v -E '/04-stdlib/diagrams/[0-9]{2}-std\.' | while read -r f; do
  [ -e "$f" ] || [ "$rev" != WORKTREE ] || continue
  src "$f" | gvpr 'N{printf("N\t%s\t%s\n",$.shape,$.label)} E{printf("E\t%s\t%s\t%s\n",$.arrowhead,$.label,$.xlabel)}' 2>/dev/null | sed "s|^|$f\t|"
  src "$f" | grep -q '«' && printf '%s\tB\n' "$f"
done | awk -F'\t' '
  $2=="N" && $3=="octagon" && $4!="" { print $1 "\tlabelled-octagon" }
  $2=="N" && ($4 ~ /^[?A-Za-z][^ ()]*\([^()]*(\([^()]*\))?[^()]*\)$/ || $4 ~ /^\([^ ()]+( [^ ()]+)+\)$/) { print $1 "\tfact-text" }
  $2=="N" && ($3=="oval" || $3=="ellipse" || $3=="doublecircle" || $3=="rectangle") { print $1 "\t" $3 }
  $2=="E" && $3=="empty" { print $1 "\tempty-head" }
  $2=="E" && ($4 ~ /^#[0-9]/ || $5 ~ /^#[0-9]/) { print $1 "\thash-number" }
  $2=="B" { print $1 "\tbadge" }' | sort -u
```

| | files | labelled octagon | fact's text | `oval` | `rectangle` | `ellipse` | empty head | `#n` | `«` |
|---|---|---|---|---|---|---|---|---|---|
| before (`1418fba5`) | 108 | 48 | 19 | 17 | 21 | 27 | 14 | 8 | 1 |
| after (this branch) | 43 | 9 | 10 | 0 | 2 | 27 | 0 | 0 | 0 |

(a column counts files.) **Every one of the 43 left is a drawing of reading 4**: the 9 labelled octagons are the
region views' reported states, the 10 *fact's text* labels are states, candidates or the design diagrams'
module and step names, the 27 `ellipse` files are the lattices', cubes', flowcharts' and design diagrams' default
node shape, and the 2 `rectangle` files are `02-dependency` and `02-parity`. **No drawing of facts is flagged.**
Graphviz gives an unset shape as `ellipse`, so that column counts drawings whose nodes have no shape set; no atom of
a drawing of facts is one.

**Checks**, in the worktree: `python3 utils/render_diagrams.py --check` (749 diagrams, every picture drawn from its
source as it stands); `python3 utils/check_doc_blocks.py` (rules 1 to 5 hold); `python3 utils/check_release_voice.py`
(ok); `EIN_BIN=build/review-bin/ein-head-8edf546b python3 utils/check_generated.py` (9 generators, every file as
written); `python3 utils/doc_audit.py --links --check --tracked`: 2 574 findings, every one under
`plans/…/review/`, whose ignored outputs a worktree lacks (the same count on `1418fba5` here), none elsewhere, and 0
decision-id findings.

**The tests that read a changed page**, alone in the worktree: `cargo test --manifest-path ein.rs/Cargo.toml
--profile gate -p ein-cli --test guide_programs --test guide_transcripts --test show_hyps --test h_space_view`, 23
passed (the guide's programs and its marked transcripts; `03-hypotheses-and-the-answer.md`'s generated blocks). No
other test reads a page this task changed: the others that name one name it in a comment or an anchor.

**Commits**: `1d858bb1` (the drawings and the pages), and this Record. **Likely to meet master's merge of S1f.23.2
and S1f.23.3**: `docs/ein/ein.rs/08-reasoning-in-code/02-saturation-and-negation.md` (§ 1.4, § 2.4, § 3.4, § 4.5's
commands and sentences) and `docs/ein/user/04-reference/13-glossary.md` (§ Fact, § Type, § Hyperedge); `06-config.md`
and `AGENTS.md` are untouched.

### Task 4 — what the phase's checks and the review re-render found, fixed (2026-11-18)

On master, after the checks recorded under [the phase's stages table](README.md#stages) and task 3's re-render.
Five fixes in three commits, two with code and one of pages, and this Record; R-31 and R-32 are the nightly's, R-22,
R-24 and R-28 the review's.

**1. The nightly's refusal (R-31), `18bc66ac`.** `rules.rs`' `constraint_line` wrote a guard's sign as its dotted
link's `label`, on a link with `constraint=false`; Graphviz 16.0.0 and the runner's 14.1.2 amd64v3 could not route
the `≠` of `column-wrong` (`tests/stdlib/math/digits/02_a_core_names_its_rows.ein`), six views. The sign is now the
link's `xlabel`, as an arrow's number is (Q-M1f.435's reading), and the rule routes. **The other `label`s on an
arrow, each checked:**

- **the region view's `sat` and `✗ alive`** (`region_dot.rs`), words on arrows with `constraint=false`: **the same
  pattern, changed** to `xlabel`s, though none was refused (a probe: `ein oracle FILE --region-limit 10 --dump`, then
  `ein render region` in `full`, `border` and `lines`, bare, with `--visited`, `--answer` and both, every corpus
  program: 7 728 views of 644 programs, none refused, 244 with no cube); the two drawings under `docs/` that copy
  the view, `06-class-of-two` and `08-region-merged-states`, drawn again with it;
- **a folded arrow's relation in a rule's panel** (`relation_lines`), Q-M1f.435's exception: **left**. No
  `constraint=false`, and `rules-fold`, `ir-fold` and the other folded views of every corpus program pass
  `dot_wellformed` under `EIN_CORPUS_SLOW=1`;
- **a trace's slice**: a folded fact's arrow through its point (`label=` the relation) and the no-good's `lifts to`.
  **Left**: they have no `constraint=false`, and the slices' refusals are not theirs. A probe put every `dot` block
  of `ein solve FILE --trace` and `--fold` to Graphviz, and found **169 canonical and 487 folded blocks refused, of
  the calculator's 36 programs (R-33)**, where a canonical slice has no word on an arrow at all and writing the
  folded ones' `label`s as `xlabel`s routes 25 of 60 sampled. What routes them is the slice's layout: without
  `newrank` fewer are refused but some newly (tried and reverted), `rankdir=TB` every one sampled. That changes how a
  slice is laid out, so it is filed, not done;
- **the rest, left**: the region view's cube arrows carry `+N`, or a repair's `+fact`, as a `label`, with no
  `constraint=false` and none refused in the probe above; the h-space graphs' strokes carry `label`s, and
  `dot_wellformed` puts every one of them to Graphviz; the lattice dump's and the derivation graph's words are on
  nodes; `fact_graph.rs`, `kb_dot.rs` and `constraints.rs` write no `label` on an arrow.

**2. `dot_wellformed`'s time (R-32), `82e8097b`.** The sweep gave each of `available_parallelism` workers a whole
file, so `examples/arith/long_numbers.ein`'s seven parse views (730 to 772 edges, 18 to 64 s each alone under `dot`)
ran one after another in one worker. It is now on `ein_corpus::pool::map`, in two passes: each file's views drawn, a
unit a file, the file loaded once as before; then each view put to Graphviz, a unit a (file, view). `EIN_TEST_JOBS`
holds and no verdict depends on it: the results are folded in the files' and views' order, into the same report. No
program or view is left out. **Measured**, the whole test binary, `utils/bench_env.sh --cores PT:4 env
EIN_TEST_JOBS=4 EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test dot_wellformed` (on AC, four
threads of two P-cores): **343.1 s before** (task 1's sweep with fix 1, its four tests green; the phase's checks
timed the one test alone on the same threads, 265.5 s, and the nightly's 264.9 s was the whole binary on every
thread), **261.2 s after**. The after run's units (`EIN_TEST_UNITS`): drawing, 888
units, 43.4 s in all, the longest 0.66 s (`stdlib/math/digits.ein`); Graphviz, 8 821 units, 827.0 s in all,
`long_numbers.ein`'s seven 41.8 to 132.8 s each (four at once on two physical cores, each slower than alone); the
structure's graphs, 169.1 s. The work is now what bounds it, not one file: **under five minutes, so not sharded**
(Q-M1f.293), with some 40 s to spare on the runner's four threads. `long_numbers.ein` is not added to
`ein_corpus::pool::LONG`: at four workers the order units start in cannot shorten a sweep the work bounds.

**3. A nested `absent` (R-22), `18bc66ac`.** The rule view drew an `absent` under an `absent` as red patterns inside
the outer cluster, so `(absent (and G (absent (not B))))` and `(absent (and G (not B)))` drew alike. Every `absent`
is now its cluster wherever it stands (`absent_block`): one under an `absent` a cluster inside that one's, holding
the nodes it alone names (a node's home is the innermost absence that holds every place it is drawn at,
`absence_paths` and `node_homes`), and one in an arm of an `(or …)`, which was drawn red too, a cluster. A pattern
under an absence is in its relation's colour; only a pattern of `not` is red. Ids: a top-level absence keeps
`cluster_absent_<panel>_<i>`, a nested one adds its index, `cluster_absent_L_0_0`, so a rule with no nested absence
draws the bytes it drew. **Test**: `presentation_semantics.rs`' `an_absent_under_an_absent_is_a_cluster_inside_it`,
the two rules drawn differently, the inner cluster inside the outer holding `not` and its pattern, `?o` the outer's,
`forall` written out with `marked` in the inner cluster and not red, an `(or …)` arm's absence a cluster; it fails
on the drawing before. Pages: `12-rendering.md` (an `absent` a cluster at any depth; a guard's sign beside its
link), `11-macros.md` and `04-stdlib/01-macros.md` (the written-out `forall`, as drawn now).

**4. Q-M1f.119 and `02-syntax.md` § Query (R-24), `d5fceff9`.** Both said `is-a` is every program's relation. They
now say `eq`, `neq` and `relation` are every program's and no theory's relation is, `is-a` one where the program
writes it, citing Q-M1f.436; checked on the binary: a goal over `(is-a ?R ?x)` in a program that writes none is
refused, over `(relation ?R ?x)` it loads.

**5. `09-core-in-derivation` (R-28), `d5fceff9`.** The two facts its derivations added (`near`, `false`) are
`penwidth=2` again; the core's three terminals stay red and double-bordered and are no longer `penwidth=2` too, so
*heavy* means what `09-proof-graph` and the page say, *a derivation added it*. The picture's text and the page's
sentence say so. Drawn again by `utils/render_diagrams.py`.

**The shape golden**, re-banked (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`, `id_order_invariance`, 29 027 lines before and
after): **720 lines of 92 programs moved**, `dot[ir]`, `dot[ir-fold]`, `dot[ir-forms]`, `dot[ir-overlay]`,
`dot[ir-trace-dag]` 92 each and `dot[rules]`, `dot[rules-fold]`, `dot[rules-overlay]` 86 each (the rule views draw
no query or macro): 88 programs by a guard's sign, 3 by a nested absent (`stdlib/macro.ein`,
`tests/stdlib/macro/01_forall.ein`, `examples/branching/14_lookahead_unjudgeable.ein`), 1 by an absent in an arm of
an `(or …)` (`examples/features/08_disjunct_guard_sets.ein`). No line of another kind moved, `region-trace` among
them (a slice draws no guard).

**The drawings drawn again.** The standard library's (`utils/gen_stdlib_rule_pages.py`, this task's binary): 33
sources changed, every one a guard's sign (none of its rules nests an absent); 511 rules, 493 canonical, 1 folded,
17 as Ein only, as before. Under `docs/`: the 11 rule drawings with a `≠` link (10 the binary's output, the sign's
attribute moved as the binary now writes it; `reasoning/01-static-ir/diagrams/02-transitive` by hand); the two of
`forall` written out, `04-stdlib/diagrams/01-rule-complete-via-expansion` (`ein render rule --name
complete-via-expansion tests/stdlib/macro/01_forall.ein`) and `03-syntax-and-semantics/diagrams/11-rule-bag-is-ready-written-out`
(the page's program with the call replaced by its expansion, reconstructed as task 2 did; both differ from the old
drawing in the nested cluster alone); the two region drawings; `09-core-in-derivation`. `render_diagrams.py` drew
every picture. The review bundle's drawings are not drawn again here.

**Checks**, on master after the commits (`b3ed198e`, the main session's plans commit on top), not run:
`./run_tests.sh`, the nightly, `utils/ci_container.sh`. `cargo test --profile gate -p ein-render -p ein-cli`
**exit 0**, 77 test binaries, 466 tests, 549 s (the tests this task touched among them: `presentation_semantics`,
`every_object_once`, `id_order_invariance`, `dot_wellformed`, `region_render`); `EIN_CORPUS_SLOW=1` `dot_wellformed`
**exit 0**, 8 821 views of 888 files accepted and **none refused**, 1 203 past the bound in the directories
`SKIPPING` names, 796 of the structure's graphs, 123.5 s on every thread at the default sixteen workers, with
S1f.23.5's workspace tests running in its worktree; `EIN_CORPUS_SLOW=1` `renaming_invariance` **exit 0**, 21.6 s;
`cargo clippy -p ein-render --all-targets -- -D warnings` and `cargo fmt --check` clean;
`utils/doc_audit.py --links --check --tracked` 0 findings and 0 decision-id findings (in the root tree, which holds
the review's outputs); `utils/check_doc_blocks.py` rules 1 to 5 hold, 748 diagrams; `utils/check_generated.py` 9
generators, every file as written; `utils/check_release_voice.py` ok; `utils/check_gate_steps.py` ok.

**Readings this task took** (for the main session to file): **(i)** the region view's `sat` and `✗ alive` count as
R-31's pattern and are changed, though none was refused; **(ii)** a trace's slice is left as it is, its refusals
filed (R-33), since what routes them changes the slice's layout; **(iii)** in `09-core-in-derivation` the core is
not drawn heavy, *heavy* being *derived* on the page; **(iv)** `long_numbers.ein` stays off `pool::LONG`.

**Findings**: R-31, R-32 (fixed here) and R-33 (open) in [the phase's findings](findings.md); R-22, R-24 and R-28
marked fixed.
