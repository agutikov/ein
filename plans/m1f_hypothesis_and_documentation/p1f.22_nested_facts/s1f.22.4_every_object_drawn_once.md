# S1f.22.4 — `ein render` draws every object once

**Phase:** [P1f.22](README.md)
**Status:** **done 2026-11-17**: `811846ce`; the gate exit 0 here and `utils/ci_container.sh` exit 0 on it; findings N-23 to N-28.
**Rulings:** [Q-M1f.395](../open_questions.md#q-m1f395--what-ein-render-draws-of-a-fact-under-an-arrow) (every
object is drawn once; a fact of three arrows is a labelled arrow only when nothing points at it) and
[Q-M1f.398](../open_questions.md#q-m1f398--how-the-graph-is-drawn-to-show-the-global-context-and-a-facts-context)
(a fact is filled where held, and inside a dashed frame labelled with the context's relation where it is in a
context).

## What it builds

1. **`ein render kb`** (and every view that draws a knowledge base with it): a fact that a fact points at is
   drawn as **its node**, an octagon with the relation's name in it and its argument arrows numbered, held or
   not, once; filled where the knowledge base holds it; inside a dashed frame `R ctx` for the relation `R` of the
   facts that point at it, one frame naming every such relation, frames side by side. No node is labelled with a
   fact's text, and no fact is drawn both as an arrow and as a node (N-3, N-12).
2. **`ein render rule`**: a nested pattern the same way, its variables and atoms drawn, so that what it binds is
   seen (N-4).
3. **The pages, in the same commits**: the row *a fact under an arrow of another* of the knowledge-base page's
   table of the labelled-arrow form; `12-rendering.md`'s legend; the decision Q-M1f.395.
4. **The goldens**: the shape golden's lines of every program whose drawing moves, re-banked for every corpus
   program (`EIN_BLESS=1 EIN_CORPUS_SLOW=1`), with the count of programs that moved in the record.

## Acceptance

`./run_tests.sh` exit 0, and **`utils/ci_container.sh` exit 0 on the stage's last commit**, as for every change
to a DOT writer: `dot_wellformed.rs` is the one authority on a view being well-formed, and the runner's Graphviz is
part of what it checks. A drawing of a program with a fact in two contexts is in the record, as a file.

## Record

### Record (2026-11-17)

**Commit**, on master: `811846ce`, the four writers, their test, the shape golden, the pages, the decision and
the redrawn diagrams together, since each page is true only with the writers' change.

**The rule, in one place.** `ein-render/src/dot_util.rs` has what the four writers share: `ctx_label`, the frame's
label (the relations in order, comma-separated, and ` ctx`); `ctx_frames`, the frames of one drawing as DOT, one
`subgraph cluster_ctx_…` a label, the labels in order, side by side and none in another; `HELD_FILL`, the filling;
`FRAMED_RANKING`, below. A fact is in a frame by its node's declaration standing in the frame's subgraph; no
arrow is written in a frame, since an arrow in a subgraph takes both its ends into it.

**The writers that changed**, each computing, for the facts one drawing draws, every fact that a drawn fact
points at, at any depth, with the relations of the facts that point at it, in the order first met and not by the
ids facts were interned under:

| writer | views | what it does |
|---|---|---|
| `kb_dot.rs` | `ein render kb`, with its options; a `--dump-answer` state or root drawn by it; the trace's state and solution drawings (`slice::render_state`, `render_solution`) | a held fact that a drawn fact points at is skipped where the held facts are drawn, and written under `// facts in a context` as its octagon in its frame, `style="filled,…"`, with its numbered arrows; a fact under an arrow that the knowledge base does not hold the same, not filled; an arrow's end that is a fact is that fact's node |
| `ir_dot.rs`, `builder.rs` | `ein render program`, its `--levi` form; each of its digraphs (background, given, derived) | the same in each digraph, a fact told by its text; the builder writes a framed node inside its frame. A negation, which this view draws as the fact it denies, dashed, is an octagon `not` that points at the fact's node where another fact points at that fact |
| `rules.rs` | `ein render rule`, `ein render rules`, the rules of `ein render program`, both modes | a pattern under an arrow of a pattern is its octagon in its frame, with numbered arrows to its variables, its names and the patterns under it; filled where it is also a premise, filled and dashed where it is a literal of the conclusion. A panel is one drawing in the side-by-side mode and the whole rule is one in the overlay, where a conclusion the match points at is the match's node. A pattern that an absence alone holds is framed inside the absence's frame. A `(not P)` whose `P` is a node is a red octagon `not` that points at it |
| `constraints.rs` | `ein render constraints` | a fact that a drawn fact has as an argument is its octagon in its frame, its unary properties a badge under its relation's name, filled where the program states it |

**The views that do not change, and why.** `ein render lattice`, `ein render region` and `ein render h-space`
(`region_dot.rs`, `structure.rs`) draw commitments, states and candidates, each a node, and no fact with its
arrows. The slice of a trace and the derivation graph (`slice::render_slice`, `derivation.rs`), and the graph of a
written `(trace …)` (`ir_dot::render_trace`), draw a derivation: a node is a fact of it, drawn once, with its
sentence, and has no arrow to its arguments (N-28).

**The test of the rule**: `ein-render/tests/every_object_once.rs`, six tests, each writer on one small knowledge
base or rule: a fact in the contexts of two relations is one octagon in one frame that names both (and, in the KB
view, `written`); held, it is filled, and no arrow of it is drawn; the fact under it is one empty octagon in its
own frame; a held fact that nothing points at is an arrow; a knowledge base with no fact under an arrow has no
frame and no filling; a negation of a fact that is a node points at the node, and alone is drawn as it was; the
variables of a nested pattern are drawn under numbered arrows; the overlay's conclusion is the match's node,
filled and dashed; a premise that is also under an arrow is one filled node and no arrow.

**Graphviz.** With frames in them, Graphviz 16.0.0 refused eight views of four corpus programs, all the
calculator's: six with *trouble in init_rank*, two with *Unable to reclaim box space in spline routing*. A digraph
that holds a frame now sets `newrank=true` (`dot_util::FRAMED_RANKING`), in the KB view, the program view and
the constraint view; under it none of the eight is refused. A rule's drawing does not set it: its frames stand
inside its panels, and no corpus rule is refused.

**Verified by invariance, on every view of every corpus program** (10 252 texts of 782 programs, each written
to a file before the change and after by a throw-away example over `ein_render::shape`, the same calls the
sweeps make):

- 3 644 views of 355 programs moved, and **each of them had a node named by a fact's text before**; 6 608 did not
  move. No view without such a node moved.
- One view had such a node and did not move: `examples/syntax/trace-steps.ein`'s graph of its written trace (N-28).
- After the change no node of any other view is labelled with a fact's text.

**The golden.** `EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test id_order_invariance`
re-banked `corpus_shapes.md5` for every corpus program: **3 644 of its 28 285 lines moved, all of them `dot[…]`
lines, of 355 programs**, the same views the comparison found. The test then passes unblessed over every program
(`EIN_CORPUS_SLOW=1`), the ids permuted. No other golden moved: `tests/golden/dot/` holds no fact under an arrow.

**The 1 000-edge bound of `dot_wellformed.rs`.** Views past it: **1 330 before, 1 342 after.** The twelve new ones
are views of three programs in directories `SKIPPING` already names, so it does not grow:
`examples/arith/many_calculations.ein` (`ir-forms` 103 → 1 509 edges, `ir-levi` 203 → 1 029),
`stdlib/math/decimal-arith.ein` (`ir`, `ir-levi`, `ir-forms`, `ir-trace-dag`, 619 → 1 057) and
`stdlib/math/int.ein` (`ir`, `ir-levi`, `ir-forms`, `ir-trace-dag` 545 → 1 099, `ir-overlay` 545 → 1 006, `rules`
499 → 1 053). No view left the set. The sweep over every corpus program (`EIN_CORPUS_SLOW=1`) passes.

**The pages, in the same commit.**

- `reasoning/01-static-ir/01-knowledge-base.md`, § *Drawn with labelled arrows*: the table's rows say *that no
  fact points at* of the three forms a held fact is drawn in, and its last row is the fact that a fact points at,
  its node, once; the sentence before the table no longer says the form has no place for it.
- `reasoning/02-kb-evolution/02-saturation.md`: the words under the fixpoint's drawing, which is `ein render kb`'s.
- `user/04-reference/12-rendering.md`: three rows of the legend; a new § *A fact under an arrow — every object
  once*, with a program and its drawing by `ein render program` and by `ein render kb`; the pattern under an
  arrow in § *Rules*, with a drawing; a sentence in § *Constraints*; two rows and the *left out* paragraph of
  § *The KB*.
- `user/03-syntax-and-semantics/03-facts.md` (two sentences), `01-objects.md` (one), `02-guide/08-word-problems.md`
  (two): what the drawing they show is.
- `ein.rs/08-reasoning-in-code/01-static-ir.md`: the sentence on what `ein render` draws of a fact in a context,
  and the claim's row with the test that holds it.
- The decision `docs/decisions/cli/Q-M1f.395-ein-render-draws-every-object-once.md`, and its index line.
- `07-diagnostics.md` § 2.30's sentence, *`ein render kb` draws it, an octagon that points at the fact*, is true
  as it stands and was not changed.

**The diagrams.** Drawn again from the new writers, each the way its page says it is made: five hand-kept
sources that held a text node (`02-note-fixpoint`, `03-program-kb`, `01-rule-first-argument-fits`,
`08-problem-facts`, `08-rule-gave-means-less`); three new ones for the rendering page (`12-context-program`,
`12-context-kb`, `12-context-rule`); and the standard library's rule drawings, by
`utils/gen_stdlib_rule_pages.py --bin ein.rs/target/gate/ein`: *9 region(s) and 329 source(s) changed*, of which
320 are drawings that moved and nine are drawings that went (N-25). `utils/render_diagrams.py` drew every picture.

**For the user to look at**, the new drawings as files under [`diagrams/`](diagrams/), each `ein`'s output as it
is and drawn with `dot -Tsvg`:

- [`render-two-contexts.svg`](diagrams/render-two-contexts.svg): `ein render kb` of the four facts of the frame's
  trial. `(= 5 (+ 2 2))`, held and under the arrows of `think` and `says`, is one filled octagon; `(+ 2 2)` one
  empty octagon; `(= 4 (+ 2 2))` an arrow. The frames name `written` (N-23).
- [`render-decimal.svg`](diagrams/render-decimal.svg): `ein render program` of a program that imports
  `std.math.decimal` and writes `(price Apple (n (n z 4) 2))` and `(price Pear (n z 4))`: the numeral `(n z 4)`
  is one octagon in the frame `n, price ctx`. The KB view of the same program holds the digit tables, 7 937
  arrows, and is no drawing to look at.
- [`render-rule-nested.svg`](diagrams/render-rule-nested.svg): `ein render rule` of the contradiction in a context
  with the context and the relation parameters, `(?C ?p (?R ?a ?b))` beside `(?C ?p (not (?R ?a ?b)))`.
- [`render-rule-unpack-overlay.svg`](diagrams/render-rule-unpack-overlay.svg): the overlay of a rule that unpacks,
  its conclusion the match's node, filled and dashed.

**The checks**, on `811846ce`:

| check | result |
|---|---|
| `./run_tests.sh`, here (Graphviz 16.0.0, on AC power) | **exit 0**: the ten static checks, 162 test binaries ok and none failed, the process's checks (`check_generated.py`: *9 generators, every file they write as written*; the review bundle's smoke, *0 DOTs refused*), the bench smoke |
| `utils/ci_container.sh` on `811846ce` (Ubuntu 26.04.1, Graphviz 14.1.2, `libpathplan4 14.1.2-1ubuntu1 amd64v3`) | **exit 0**: the same gate in the runner's image, 162 test binaries ok and none failed, *0 DOTs refused* |
| `EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test dot_wellformed`, every corpus program, here | exit 0, 4 passed |
| `EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test id_order_invariance`, unblessed, every corpus program | exit 0, 4 passed |
| every view of every corpus program that holds a frame, put to the runner's Graphviz build by itself, before the commit | 2 424 views under the bound, none refused |
| `doc_audit.py --links --check --tracked`, `check_doc_blocks.py`, `render_diagrams.py --check`, `check_release_voice.py`, on the commit | each exit 0 |

The gate here ran on the tree that became `811846ce` less one row of a table of
`08-reasoning-in-code/01-static-ir.md`, added while it ran; the four page checks were run again with the row, and
the container's gate is of the commit itself. Before it, the sweep over every corpus program failed twice on the
way: once on the eight views Graphviz refused without `newrank`, and the golden's test once, before the re-bank.

**What it found**, in [`findings.md`](findings.md): N-23 (a frame of the KB view names `written`), N-24 (a
literal `\n` in an octagon's label, not new), N-25 (nine rule drawings gone from the reference), N-26 (only an
octagon in a frame is filled), N-27 (the program view is several drawings), N-28 (the drawings of a derivation
are as they were). N-3, N-4 are built; N-12's drawing is said on the rendering page.

**Not done.** The nightly was not run: no engine source outside `ein-render` changed, and its DOT steps are the
two sweeps above run over every program. `MAX_NODES`, the generator's bound, is as it was (N-25). The literal
`\n` of the KB view's labels is not mended (N-24), no held octagon outside a frame is filled (N-26), and the
program view is still several digraphs (N-27): each would move the drawing of a program with no fact under an
arrow, which the stage holds to its bytes, and each is the user's call. `fdp`, the layout a KB view suggests and
no test runs over the corpus, was put by hand to the 31 corpus KB views that hold a frame and have at most 300
arrows, Graphviz 16.0.0: none refused.
