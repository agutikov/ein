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
