# S1f.22.7 — a frame around what is in a context only, and no filling

**Phase:** [P1f.22](README.md)
**Status:** **done 2026-11-18**, `58b91dc3`; planned 2026-11-18 on the user's re-ruling of Q-M1f.398, and run
on the user's word (*"continue till the end of phase"*). It runs on master once the `is-a` row's change
([P1f.21's V-6](../p1f.21_verdicts_as_rules/findings.md#v-6--the-read-out-reads-is-a-by-its-name)) is in, since
both work in the main tree and a gate builds the whole tree. S1f.22.5 runs meanwhile in its worktree, and touches
no renderer.
**Ruling:** [Q-M1f.398](../open_questions.md#q-m1f398--how-the-graph-is-drawn-to-show-the-global-context-and-a-facts-context), its paragraphs
*Re-ruled 2026-11-18* and *The filling, ruled 2026-11-18*.

## The ruling, in the user's words

> draw nested facts that were wrapped and still present also in global context
>
> structurally we see clearly nexted facts, and see what context they belong to, even without frames
>
> but the only thing we don't see on rendering is whether this fact is also in global context
>
> let's draw a frame around objects that are in context only

and then *"drop the filling"*.

So: **a frame is around the objects that are in a context only**, pointed at and not held. **A held object is in no
frame**, whatever points at it. **No octagon is filled.** Which contexts an object is in is read off the arrows that
enter it. A frame is labelled as today, with the relations of the facts that point at what it holds and `ctx`. Frames
are side by side, one for each label, and none holds another.

The trial is [`diagrams/frame-ctx-only-*`](diagrams/): S1f.22.4's three multi-context figures, with every held
octagon moved out of its frame, a frame left empty dropped, and no filling.

## What changes

1. **The DOT writers**: the KB view and the program view, and `ein render rule`'s panels. In the rule view a
   pattern that is also a premise, or a literal of the conclusion's panel, is today filled in its frame; it leaves
   its frame and is not filled. Read every writer that emits a context frame or the filling
   (`git grep -n -l 'cluster_ctx\|fillcolor' -- ein.rs/crates`: `kb_dot.rs`, `ir_dot.rs`, `rules.rs`,
   `dot_util.rs`, `constraints.rs`, `slice.rs`, `structure.rs`, `region_dot.rs`, `dump/region_lattice.rs`), and
   change those that draw this rule. A `fillcolor` that marks something else, a region's node say, is not this
   rule's.
2. **The tests**: `ein-render/tests/every_object_once.rs` and any golden that holds a frame or the filling; the
   shape golden where a rendering it digests changes, re-banked per `AGENTS.md`.
3. **The pages**:
   - `docs/ein/user/04-reference/12-rendering.md`: the table's two rows (filled; the frame) and the frame rule's
     section, with its drawings `12-context-kb`, `12-context-program`, `12-context-rule` drawn again from the
     binary as they were drawn;
   - every hand-written diagram source under `docs/` that draws a context frame or the filling for *held*:
     `git grep -l 'ctx"' -- 'docs/**/*.dot'` outside `04-stdlib/` gives 18, in `reasoning/01-static-ir/`, the
     guide, `03-syntax-and-semantics/` and `04-reference/`, of which some fill. Each is drawn again by
     `utils/render_diagrams.py`, with the text of its page where the text says *filled* or *in a frame* of a held
     fact;
   - the standard library's rule drawings, through `utils/gen_stdlib_rule_pages.py`.
4. **The decisions**: Q-M1f.398's file (`docs/decisions/docs/Q-M1f.398-…`) rewritten to the answer that stands;
   its slug changes if the slug says otherwise, and every citation moves with it. Q-M1f.395's (`docs/decisions/cli/`),
   where it says *filled*.

## Acceptance

- Every object is still drawn once (Q-M1f.395).
- A figure with no fact under an arrow is the drawing it was.
- `./run_tests.sh`: exit 0 on the last commit.
- `utils/ci_container.sh`: exit 0, since the DOT writers change ([Q-M1f.77](../open_questions.md#q-m1f77--when-is-utilsci_containersh-part-of-done)).
- No answer changes: no stage of this one reads or writes a state.

## Record

**Done 2026-11-18**: `58b91dc3`, the build with its pages, drawings, golden and decisions; this record beside it.
The checks are at the end.

**What changed, by writer** (`ein.rs/crates/ein-render/src/`):

- `kb_dot.rs`, the KB view (`ein render kb`): a fact that a drawn fact points at is declared under
  `// facts in a context` as before; where the knowledge base holds it, outside every frame, and where it does not,
  in the frame of the relations that point at it. `hyperedge_node` lost its `filled` argument. `newrank=true` is
  written only where a frame is left.
- `builder.rs` and `ir_dot.rs`, the program view (`ein render program`, compact and Levi): `Builder::framed` is
  `Builder::fact_node(id, attrs, Option<frame>)`; a fact the program states is declared in no frame. The digraph
  keeps `ranksep=0.55` wherever it draws such an octagon, framed or not, as it did; `newrank` only with a frame.
- `rules.rs`, the rule view (`ein render rule`/`rules`, both modes): `nested_lines` returns a `Nested` whose frame
  is `None` where the pattern is a premise, or a literal of the conclusion; such a node goes with the panel's
  declarations (or the absence's block), and a literal of the conclusion that is no premise keeps its dashed
  outline, without the fill.
- `constraints.rs`, the constraint view: a fact the program states is declared in no frame.
- `dot_util.rs`: `HELD_FILL` is gone with its last reader; the section comment states the rule.
- Not this rule's, read and left: `slice.rs` (the seed's fill), `structure.rs` (a component's), `region_dot.rs`
  and `dump/region_lattice.rs` (a state's). The `is-a` boxes and dashed edges of `kb_dot.rs` and `ir_dot.rs`, and
  `grid.rs`, untouched.

On the three trial programs, `ein render kb` writes the trial figures [`diagrams/frame-ctx-only-*`](diagrams/) line
for line, but for the trial's hand-written comments and a blank line, and the trial's `newrank=true` in
*held-in-two-contexts*, which has no frame (default 1 below).

**Readings taken, each a default:**

1. `newrank=true` only where a frame is left, as the rendering page says (*a view that holds a frame asks Graphviz
   to rank the whole graph at once*). A view whose every fact under an arrow is held has no frame and ranks as a
   view with none did.
2. In the rule view, a literal of the conclusion that is no premise loses its fill and its frame and keeps its
   dashed outline: in the overlay it is the one mark of what the conclusion adds, as the conclusion's arrows are
   dashed. The ruling says *leaves its frame and is not filled*; nothing of the outline.
3. **The hand-written drawings filled every held octagon, framed or not**: the reasoning's convention
   (`reasoning/README.md`, *an octagon is a fact, filled where the knowledge base holds it*; the knowledge-base page,
   *a filled octagon is held, an empty one is only pointed at*). The stage file names *a context frame or the
   filling for held*; the grep for a frame finds 18 sources, and 9 more fill with no frame. The filling went from all
   of them, so that no page fills a held octagon beside one that does not: 20 hand-written sources in all.
4. **The extent's drawing** (`01-relation-extent`) draws a fact $g$ that no knowledge base holds and nothing points
   at: no frame can say it is not held, and the filling was its one mark. It is drawn grey, octagon and arrows.
   Shown as N-50, with the extent's unlabelled dashed frame, which now reads against the convention.
5. The beliefs' two KB drawings under `examples/` (N-33) are `ein render kb`'s output with no test behind them;
   drawn again by `examples/README.md`'s line, so that they are not behind the renderer once more.

**The tests and goldens.** `ein-render/tests/every_object_once.rs`: every writer is held to *no node is filled*; a
fact held and in two contexts is one octagon in no frame (KB view, program view in both forms), a pattern that is a
premise or a literal of the conclusion is in no frame (the rule view, both modes, the overlay's dashed), a fact the
program states is in no frame (the constraint view), and a fact in a context only is in the frame that names every
relation that points at it, as before. A new test, `a_knowledge_base_whose_facts_under_an_arrow_are_all_held_has_no_frame`,
draws the trial's *held-in-two-contexts* (no frame, no `newrank`) and *two-contexts-of-one-relation* (the held fact
outside, the thought one alone in `think ctx`). `the_kb_view_draws_a_fact_in_two_contexts_once_in_one_frame` is
`…_once`.

**The shape golden**, `ein-render/tests/golden/corpus_shapes.md5`, re-banked with
`EIN_BLESS=1 EIN_CORPUS_SLOW=1 cargo test --profile gate -p ein-render --test id_order_invariance`: **225 lines, every
one a rendering**, `dot[…]`, of 144 programs; no line of any other kind moved (`load`, `saturate`, `plan`, `match`,
the read-out under each question, the states, the lattice, the cores, the oracle's cube):

| kind | lines |
|---|---|
| `dot[kb-since]` | 133 |
| `dot[ir-overlay]` | 23 |
| `dot[rules-overlay]` | 17 |
| `dot[ir]`, `dot[ir-levi]`, `dot[ir-trace-dag]` | 7 each |
| `dot[kb]`, `dot[kb-origin]`, `dot[kb-none]`, `dot[kb-no-types]`, `dot[kb-no-instances]` | 6 each |
| `dot[constraints]` | 1 |

The diffs were read whole, by a before/after dump of all 14 views of those 144 programs (2 016 views; a scratch
example over `ein_render::shape::dot_shape`, run on this build and on HEAD's six writers, and not committed): **225
views differ, exactly the golden's**, and in each one the 500 octagons that were filled left their frames, the 14 387
other framed nodes kept their frame and its label, the lines are the same as multisets once the fill and `newrank`
are set aside, and `newrank` is there exactly where a frame is. No other golden holds the frame or the filling:
`golden/dot/`'s one fill is the slice's seed.

**No answer changes**: the stage reads and writes no state, and the gate's sweeps hold every `ein solve` output and
exit code of the corpus (`corpus_cli`, the read-out goldens, the shape golden's every non-`dot` line, which moved
none).

**The pages and their drawings.**

- `user/04-reference/12-rendering.md`: the legend's two rows (the frame, for a fact in a context only; *in no
  frame* for a held one, no octagon filled), § *A fact under an arrow* (one mark, said by the arrows' limit), its two
  drawings' words, the paragraph on `newrank`, § *Rules*, § *Constraints*, the KB table's row and *Left out*.
  `12-context-program` and `12-context-kb` drawn again from the binary as they were drawn (the KB one with the
  `fdp` hint and the font line left out, as before); `12-context-rule` drawn again: the same bytes, since the pattern
  under `claims` is no premise.
- `reasoning/README.md`'s graph convention; `reasoning/01-static-ir/01-knowledge-base.md`: *how the graph is
  drawn*, the extent's words (default 4), the fact about a fact, the three cases' table (*drawn*: in no frame, in a
  frame, in no frame with an argument arrow entering), the words under the three drawings, *How the two are drawn*,
  the labelled-arrow table, the four statuses, the negation's drawing; `02-rewriting-rules.md`: the pattern's
  convention, packing's and unpacking's words under the drawings.
- `user/03-syntax-and-semantics/03-facts.md` (two passages), `user/02-guide/08-word-problems.md` (one sentence),
  `ein.rs/08-reasoning-in-code/01-static-ir.md` (the paragraph and the claim's row).
- Unchanged and still true: `01-objects.md`'s `fits ctx`, `05-rules.md`'s drawing, the guide's rule drawing.

**Drawings redrawn**: `utils/render_diagrams.py` drew **22** pictures: the rendering page's two, and **20**
hand-written sources that lost the held filling, of which six also had a held octagon or a premise moved out of its
frame (`01-context-both`, `02-pack-match`, `02-pack-after`, `02-unpack-match`, `02-unpack-after`,
`03-facts-in-contexts`) and one a fact drawn grey (`01-relation-extent`). Of the 18 framed sources, 4 are as they
were (`08-problem-facts`, `08-rule-gave-means-less`, `01-rule-first-argument-fits`, `03-program-kb`: every fact in
their frames is in a context only), and `12-context-rule` the same bytes. **The standard library's 329 framed rule
drawings: 0 redrawn**; `utils/gen_stdlib_rule_pages.py --bin ein.rs/target/gate/ein` says *511 rules of 24 modules,
their regions and the 495 sources of their drawings are as written*, since they are the side-by-side mode and no
pattern under an arrow there is a premise of its panel. Under `examples/beliefs/`, 2 KB drawings and their `.svg`.

**The decisions.** Q-M1f.398's file rewritten to the answer that stands, *A fact in a context only is drawn in a
frame named for the context's relation*, renamed with its title to
`docs/decisions/docs/Q-M1f.398-a-fact-in-a-context-only-is-drawn-in-a-frame.md`; its one page citation
(`12-rendering.md`) and the folder's index line moved with it, and Q-M1f.395's link. Q-M1f.395's file: its
paragraph on the mark, the rule view's clause and *Held by*.

**The checks**, on `58b91dc3`:

| check | result |
|---|---|
| `./run_tests.sh`, here (Graphviz 16.0.0, on AC power) | **exit 0** in 1 101 s: the ten static checks, the workspace's tests at the gate profile with 0 failed, the process's checks (`check_generated.py`: *9 generators, every file they write as written*; the review bundle's smoke, *0 DOTs refused*), the bench smoke |
| `utils/ci_container.sh` (ubuntu 26.04, the amd64v3 packages, Graphviz 14.1.2) | **exit 0**, on `58b91dc3` as committed |
| before the commit: `check_doc_blocks.py`, `doc_audit.py --links --check --tracked`, `render_diagrams.py --check`, `check_release_voice.py`, `EIN_BIN=… check_generated.py`, `cargo clippy -p ein-render --all-targets -D warnings`, `cargo doc -p ein-render -D warnings` | exit 0 each: *rules 1, 2, 3, 4, 5 hold*; *0 finding(s)*; *every picture drawn from its source as it stands*; *none speaks of the development*; *9 generators, every file they write as written* |
| the nightly | not run: no answer changes, and the shape golden was re-banked over every corpus program (`EIN_CORPUS_SLOW=1`) |
