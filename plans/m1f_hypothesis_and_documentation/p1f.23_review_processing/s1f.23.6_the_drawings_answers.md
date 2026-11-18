# S1f.23.6 — the user's answers on the drawings, built

**Phase:** [P1f.23](README.md)
**Status:** started 2026-11-18 on the user's word (*"plans/m1f_hypothesis_and_documentation/review/03-drawings/ANSWERS.md
- I answered, act on them"*), in a worktree beside the phase's final checks on master; merged after them, and one gate,
`utils/ci_container.sh` and one nightly hold it.
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

Not started.
