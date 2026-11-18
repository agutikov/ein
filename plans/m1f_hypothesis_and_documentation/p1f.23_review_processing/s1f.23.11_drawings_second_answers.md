# S1f.23.11 — the user's answers on the drawings S1f.23.6 built: the derivation's colours, and a concentric layout

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-19 on the user's word (*"process plans/m1f_hypothesis_and_documentation/review/03-drawings/ANSWERS.md"*),
in a worktree beside S1f.23.10; merged by the main session, and one `utils/ci_container.sh` and one nightly hold
both. Task 2 stops for the user's look.
**The answers:** the user's file as written, `d124e63b`
(`git show d124e63b:plans/m1f_hypothesis_and_documentation/review/03-drawings/ANSWERS.md`), and the removed items'
pages beside it in that commit (`…/03-drawings/NN-…/README.md`), with their programs.

## The answers

| item | the reading it answers | the user's answer |
|---|---|---|
| 56, the layers | Q-M1f.487 | *"Perfect! (a) But large diagrams still look messy. Does Graphviz have circular/concentric layout? Objects will be outside, then facts, then relations and types opbejcts, then relations definitions and properties facts and in the center `relation` `symmetric` and other, is it possible?"* |
| 57, a folded arrow's rank | Q-M1f.488 | *"(a) All good, but still, for large diagrams concentric layout maybe can improve visibility."* |
| 58, the two-arrow fold | Q-M1f.490, .491 | *"(a)"* |
| 59, a derivation's drawing | Q-M1f.492, .493, .494 | *"Yes, but don't use black color for highlighting. use both color and thickness, derived fact also colored. same color as derivation but more bright. The added fact is the brightest, then derivation explanation, and then all the rest with default black and default thickness."* |
| 60, R-33 | Q-M1f.497 | *"(a)"* |

Items 56, 57, 58 and 60 stand as built. What is left to do is item 59's change and the question of items 56 and 57.

## Task 1 — a derivation's colours (item 59), built

**The ruling**: three levels of emphasis, and no colour near black among them.

1. **The fact a derivation adds**: the brightest, in its derivation's hue, and thick.
2. **The derivation's explanation**: its arrows (a premise to its step, a step to what it derives, the core to `⊥`)
   and its steps' borders, in the same hue, less bright, and thick (`penwidth=2`, as today).
3. **Everything else**: Graphviz's default black at the default width.

So `DERIVATION_PALETTE` (`ein-render/src/slice.rs`) becomes a hue a rule, each in two shades, the brighter the derived
fact's. No shade may be near black: today's indigo `#1a237e` and brown `#5c3317` go, and every shade keeps a
readable contrast against white. The hues stay clear of red (a commitment's and the core's) and grey (a negation's).
Every writer that reads the palette follows it: the trace's slices, the derivation graph, and a written trace's
`dag`. The stage finds them by the palette's name.

Readings with a default, the stage's to take and record:
- whether a premise's own node is *explanation* (coloured) or *the rest* (black), its arrow coloured either way;
- whether a commitment's red and a negation's grey are kept beside the three levels;
- the widths of levels 1 and 2.

**Pages**:
- Q-M1f.428's decision file amended;
- `12-rendering.md` and every page showing a derivation's drawing, each copy re-taken from the binary;
- the shape golden re-banked, its moved kinds named (`region-trace`, `dot[ir-trace-dag]`, the derivation graph's).

**For the user's look**: a new item **62** under `review/03-drawings/`, with a derivation drawn before (`d124e63b`'s
binary) and after on two or three programs, one of them a step with several premises.

## Task 2 — a concentric layout (items 56, 57), examples; stops for the user

**The question**: does Graphviz have a circular or concentric layout? The user's rings, from outside to centre:
1. objects;
2. facts;
3. relations and type objects;
4. relation declarations and property facts;
5. at the centre, `relation`, `symmetric` and the like.

These are the layered layout's layers, read inside out.

**Examples, mocked from today's binary's drawings**, no engine change. At least these two ways:
- **`twopi`**: Graphviz's radial layout. It puts each node on the ring of its graph distance from a `root`. Use a
  hidden centre joined to the innermost layer, and show where distance gives the user's rings and where it does not:
  a nested fact, a fact whose argument is a relation, a type that is also an object.
- **The layered drawing bent into circles**: `dot` lays the layers out as today, each layer becomes a ring (its depth
  the radius) and each node's place in its layer an angle. The positions are pinned, and `neato -n2` routes the
  edges. This gives the user's rings exactly, and keeps the crossings `dot` already reduced.

Others where they help, for example `twopi` with per-ring `ranksep`, or `circo`. Say for each why or why not.

**On which programs**:
- the six programs of item 56;
- three large knowledge bases, the drawings the user finds messy: `examples/zebra2.ein`'s, a beliefs program with
  nested facts, and a calculator program;
- each drawn three ways: layered as today, `twopi`, and bent.

**The item**: **61** under `review/03-drawings/`, with:
- the pictures side by side;
- what each way gives and costs;
- how it would be asked for, for example `ein render kb --layout concentric` or a `layout=` hint. The drawings stay
  structure-checked: tests check structure, not layout.

**Re-create** `review/03-drawings/` with its `README.md`, an `ANSWERS.md` with an entry each for items 61 and 62, and
rows in the bundle's `README.md`. Follow the bundle's `.gitignore`: pages and programs tracked, outputs ignored.

## Acceptance

- **Task 1**:
  - `dot_wellformed` (here; the main session's container run is the authority for the runner's Graphviz);
  - the renderers' tests;
  - `renaming_invariance.rs`;
  - `utils/render_diagrams.py --check` and `utils/check_doc_blocks.py`;
  - the shape golden re-banked;
  - `cargo fmt --check`, `cargo clippy` and `cargo doc -D warnings` on `ein-render`.
- **Task 2**: the item's pictures, drawn by this host's Graphviz and by the CI image's
  (`ein-ci:ubuntu-26.04-amd64v3-…`, `docker run … dot`/`twopi`/`neato`), each refusal named.
- The full gate is the main session's, once on the merged tree.

## Record

Not started.
