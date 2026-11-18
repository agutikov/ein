# S1f.22.7 — a frame around what is in a context only, and no filling

**Phase:** [P1f.22](README.md)
**Status:** planned 2026-11-18 on the user's re-ruling of Q-M1f.398, and run on the user's word
(*"continue till the end of phase"*). It runs on master once the `is-a` row's change
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

Not started.
