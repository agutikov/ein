# S1f.23.11 — the user's answers on the drawings S1f.23.6 built: the derivation's colours, and a concentric layout

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-19 on the user's word (*"process plans/m1f_hypothesis_and_documentation/review/03-drawings/ANSWERS.md"*),
in a worktree beside S1f.23.10; merged by the main session, and one `utils/ci_container.sh` and one nightly hold
both. Task 2 stops for the user's look. **Ran to its stop 2026-11-19** in `build/wt-s1f2311` (branch
`s1f.23.11-drawings`): task 1 built, task 2's examples made, items 61 and 62 put; waits for the user's look.
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

### Task 1 — a derivation's three levels, built (`21728654`)

- **The palette**, `ein_render::slice::DERIVATION_PALETTE`: before, eight single colours, `#0047ab` `#006400`
  `#6a0dad` `#a14a00` `#00686b` `#8b008b` `#5c3317` `#1a237e`, at 1.6 (indigo) to 3.5 (burnt orange) against black;
  after, six hues of two shades (`Hue { explanation, derived }`): blue `#0e61cc`/`#418cfa`, teal `#11706e`/`#1c9e9a`,
  green `#117622`/`#1ca433`, orange `#98530f`/`#d37619`, purple `#951be7`/`#b46afa`, magenta `#af2b94`/`#f240cd`.
  The deeper shade at 5.8 to 5.9 against white and 3.6 against black, the bright one at 3.3 and 6.4; computed in
  OKLCH at a fixed WCAG luminance a level (0.13, 0.27), each hue's chroma near its gamut's edge, clear of red (27°)
  and grey. `derivation_colour` became `derivation_hue`.
- **The levels**: a fact a step added, its octagon and its arrows, the bright shade at `DERIVED_WIDTH` (`penwidth=3`);
  a step's border and label and the derivation's arrows the deeper shade at `penwidth=2`; the rest default black,
  default width.
- **The writers**: `render_slice` (the trace's slices); `derivation_dag_to_dot` (the derivation graph, the first hue;
  its provenance note stays black); `ir_dot::render_trace_dag` (a written trace's `dag`: a derived fact took its
  relation's colour at width 2, now its step's bright shade at 3; a fact no step derives, `DEFAULT_INK`, black, where it
  took its relation's colour).
- **Tests**: `presentation_semantics.rs`' `a_derivation_is_drawn_at_three_levels`,
  `a_redundant_firings_conclusion_is_not_drawn_as_added`, `no_shade_of_the_derivation_palette_is_near_black`.
- **Goldens**: `slice_render_slice.dot`, `kb_provenance_dag.dot`, `ir_render_trace_dag.dot`; the shape golden
  re-banked with `EIN_BLESS=1 EIN_CORPUS_SLOW=1` (`EIN_TEST_JOBS=8`): 522 lines moved, **`region-trace` 521 and
  `dot[ir-trace-dag]` 1**, no line count changed. The derivation graph has no corpus kind (library only).
- **Pages**: `12-rendering.md` § The trace (a table of the three levels); Q-M1f.428's decision amended, its title too
  (and `docs/decisions/cli/README.md`'s line); the four slices of `user/03-syntax-and-semantics/` re-taken from the
  binary from their pages' programs (`03-first-firing`, block 1 of `03-facts.md`'s program; `05-firing`, block 4 of
  `05-rules.md`'s; `10-div-by-zero-flagged` and `-refuted`, blocks 17 and 18 of `10-theory.md`'s second program,
  which the stale digraph names `step15`, `step16` had hidden: byte for byte the *before* binary's but for those
  names), each matched against the *before* binary first; the reasoning pages' four (`02-derivation`,
  `09-core-in-derivation`, `09-cyclic`, `09-proof-graph`) restyled by the same rule, the steps' hues the binary's for
  `neighbour` (blue) and `fox-keeps-away` (magenta), the hand ones' primary blue and alternative magenta; every picture
  drawn by `utils/render_diagrams.py`; the pages' words and pictures' texts (`03-facts.md`, `05-rules.md`,
  `10-theory.md`, `02-saturation.md`, `09-cores.md`).
- **Checks**, all green: `cargo fmt --check -p ein-render`; `cargo clippy -p ein-render --all-targets -D warnings`;
  `cargo doc -p ein-render --no-deps` with `-D warnings`; `ein-render`'s tests but the sweeps (lib, `derivation_dot`,
  `diff_declines`, `every_object_once`, `golden_dot`, `golden_trace`, `idea08_acceptance`,
  `line_modes_past_the_limit`, `presentation_semantics`); `dot_wellformed` (5, this host's Graphviz 16.0.0) and
  `renaming_invariance` (3) at `EIN_TEST_JOBS=8`; the shape golden's bless run; `utils/render_diagrams.py --check`;
  `utils/check_doc_blocks.py`; `utils/check_release_voice.py`; `utils/check_generated.py` (9 generators);
  `utils/doc_audit.py --links --check --tracked`: 368 findings, every one an ignored review output absent in a
  worktree (P1f.21's items 32 and 33, the bundle's `tools/draw.sh`), none elsewhere. Item 62's 140 drawings in both
  Graphviz builds: none refused.

### Task 2 — a concentric layout, examples (item 61); stopped for the user

- **Ways**: layered (`dot`); `twopi` from a hidden centre joined to the top layer, at Graphviz's spacing and with each
  ring's radius fitted (`ranksep` as a list); the layers bent into rings, by order (`bent`), by order refined with
  circular barycentre sweeps (`bent-bary`) and by `dot`'s x (`annulus`), positions pinned, `neato -n2`; `circo` on the
  four small programs. Rebuilt from `dot -Tjson0` by `review/03-drawings/61-concentric/tools/concentric.py`
  (ignored, with `rerun.sh` and `sheets.py`).
- **Programs**: item 56's six (`five-layers` kb, `five-layers-rule` rule, `01_common_belief` kb,
  `06_double_negation` kb, `zebra2` program `kb`, `add` program `kb`) and three large: `zebra2.ein` kb (452 facts),
  `10_everybody_contradicts.ein` kb (36 facts, the largest beliefs program; every beliefs program is small) and
  `binary_arithmetic/first.ein` kb (456 facts; a decimal calculator's saturated KB is 2 172 facts, 11 174 arrows).
  Views byte for byte the same from `ein-head-d124e63b` and `ein-s1f2311`.
- **What they show**: the rings turn a strip into a disc (zebra2's KB 210 × 4 in layered, 52 × 51 bent; the
  calculator's 382 × 8 and 145 × 143) and are larger; below a few dozen facts the layers read best; twopi at default
  spacing piles nodes, fitted it is the largest, and it drops the frames; twopi's rings miss the layers on nested
  facts, facts with relation arguments, objects split by nearest fact, an undeclared relation, a type that is also an
  object (0 of 22 nodes moved on the five layers, 99 of 532 on zebra2's KB); the bent ways give the rings exactly but
  fan a relation's arrows across the centre and stretch a frame's box.
- **Graphviz**: this host's 16.0.0 and the CI image's 14.1.2 (`ein-ci:ubuntu-26.04-amd64v3-rust1.97.1-eca9ef96`,
  through `docker run --rm -i`): 116 drawings, **none refused, no warning, in either build**. With the router on
  (`splines=true`), zebra2's bent KB took 240 s on this host (straight: 0.13 s), so the radial ways draw straight
  arrows; one DOT with `layout=neato`, `inputscale=72` and every `pos` pinned is drawn by a plain `dot -Tsvg` exactly
  as by `neato -n2`, in both builds (0.0 pt apart).

### The review

`review/03-drawings/` re-created: `README.md`, `ANSWERS.md` (61, 62), items `61-concentric/` and
`62-derivation-colours/` (pages and programs tracked; `rerun.sh`, `tools/`, `runs/`, `renders/` ignored, made in this
worktree); the bundle's README: the topic row, the two binaries, the item rows. The binaries are
`build/review-bin/ein-head-d124e63b` (master's engine at `d124e63b` = `7d5b6262`) and `build/review-bin/ein-s1f2311`.

### Readings, each with its default (to be filed)

1. **A premise's own node is the rest**, black at the default width; its arrow into the step is the explanation. A
   premise another step of the drawing added keeps that step's bright shade.
2. **A commitment's red and a negation's grey are kept** beside the three levels; a derived negation takes its step's
   bright shade (it was grey and bold), the added fact being the brightest whatever its relation.
3. **The widths**: level 1 `penwidth=3`, level 2 `penwidth=2`, the order told by brightness and width both.
4. **A redundant firing adds nothing** (`Firing::derived` holds facts already there): its conclusion is the rest; it
   was bold. Its step stays dashed in the deeper shade.
5. **The refutation's arrows are red**: the core's to `⊥` were; the learned no-good's, black at width 2, are red now
   (library only: no CLI path draws a `⊥` slice).
6. **The derivation graph's added fact** is one a drawn derivation concludes whose first record is a rule's; its
   provenance note stays black, since a bright shade is for lines and reads poorly as text.
7. **"Near black"** read as a contrast under 3.5 against black, and readable as a contrast of 5.5 against white for
   the shade a step's label is written in, 3 for a line; held by a test. Six hues, not eight: fewer hues that stay
   distinct at two shades each, clear of red and grey.
8. **The reasoning pages' hand drawings** follow the rule: `09-cyclic`'s given `r(a, b)`, which the alternative
   derivation also concludes, stays plain (given, not added); `09-proof-graph`'s `F` in the primary's hue.
9. **The re-taken user slices keep the binary's bytes**, digraph names `step17`, `step18` for the division by zero.
10. **Not touched, outside item 59**: `ein render kb --since` still draws what one state holds beyond another at
    `penwidth=3` in black; `render_slice`'s library-only `since` keeps width 3. Black emphasis in a view that is no
    derivation's: the user's rule may reach it; the main session's to put.
11. **Item 61**: straight arrows in the radial ways; frames in the bent ways from computed boxes (`neato -n2` draws a
    frame only from a given `bb`); the default (a), nothing built, since the user asked whether it is possible.
