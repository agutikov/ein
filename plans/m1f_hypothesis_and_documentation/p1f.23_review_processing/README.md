# P1f.23 — the review's rulings, built

**Milestone:** [M1f](../README.md)
**Status:** opened 2026-11-18 on the user's instruction (below), with its first stage,
[S1f.23.1](s1f.23.1_drawing.md), the drawing, **started on the user's word** (*"add a stage for drawing, implement
this"*). A later stage is planned when the user rules more of the review's items, and **is run only on the user's
word**.
**Depends on:** P1f.22, closed; the review bundle [`../review/`](../review/README.md), one folder an item that waited for
the user at P1f.22's close, its pages and programs committed, what its runs produce ignored and re-taken by each
item's re-run.
**Blocks:** the close of M1f, with P1f.21.

## What it is

The user reviews the bundle topic by topic and rules its items. This phase builds the rulings, a stage per batch,
re-takes what the bundle shows of them, and stops for the user's next look. Each ruling is recorded where its item is
(`open_questions.md`, `findings.md`, P1f.21's findings, the followups), with the user's words.

## The instruction

The user, 2026-11-18, verbatim, after reading `review/03-drawings/` (answered by the user 2026-11-18 and removed) and the
main session's account of the four ways a fact is drawn:

> Ok, L-107 - I want all 3 - grey + octagon + color arrows; Then important: I want `ein render` now draw first form
> by default and optionally fold 3-facts into labelled arrows, and not produce labelled octagons anymore. Forms 2
> and 4 must not be produces or used in docs. open new phase for review processing, add a stage for drawing,
> implement this, re-render everything including plans/m1f_hypothesis_and_documentation/review then I will review
> plans/m1f_hypothesis_and_documentation/review/03-drawings again; Both KB and program view must follow the same
> label="owns", and number the
>     arrows 1/2; `ein render kb` renders whole KB, `ein render program` renders 3 parts: the same as KB + rules +
> query; DO NOT split KB into ontology and facts. This split is relevant during program sythesis, not during it's
> review/analysis.

The four forms, as the main session put them that day:

1. **canonical**: a fact is an octagon with no name in it, its first arrow at the relation's atom and the next ones
   at its arguments, numbered; the graph as the reasoning pages define it (Q-M1f.379 to .381);
2. **labelled octagon**: the relation's name written in the octagon, numbered arrows to the arguments, no node for the
   relation (the Levi-bipartite drawing, Q1);
3. **labelled arrow**: a fact of a relation and two arguments drawn as one arrow from the first argument to the second,
   labelled with the relation's name (a convention of drawing, Q-M1f.382);
4. **text node**: a box labelled with a fact's text, `claims(Carol, owns(Bob, Dog))`, in the drawings of a derivation.

Ruled as [Q-M1f.419](../open_questions.md#q-m1f419--how-ein-render-draws-a-fact) (the forms),
[Q-M1f.420](../open_questions.md#q-m1f420--what-ein-render-program-draws) (the program view) and L-107 (the
extent's drawing, in [the milestone's findings](../findings.md)); they answer L-98 and L-99 as well.

## Stages

| stage | what | state |
|---|---|---|
| [S1f.23.1](s1f.23.1_drawing.md) | **The drawing.** Every view of `ein render` draws a fact in the canonical form, and folds a fact of three arrows into a labelled arrow on request; no labelled octagon and no text node is produced, or drawn on any page; the KB and the program views label and number alike; `ein render program` is the program's knowledge base as one graph, its rules and its query; L-107's drawing as ruled; every drawing of the tree and of the review bundle drawn again; the user then reviews `review/03-drawings/` again | tasks 1 to 4 **done and merged** (`8edf546b`, `4d31d9c0`; `1d858bb1` merged by `231486f0`; task 3, the review bundle drawn again, `43bba0be`; task 4, what the phase's checks and that re-render found, `18bc66ac`, `82e8097b`, `d5fceff9` and its record `db86d660`, on master), their readings Q-M1f.421 to .435, .467 to .474 and task 4's .483 to .486; its findings R-15 to R-21 and R-31 to R-33, R-31, R-32, R-22, R-24 and R-28 fixed by task 4, R-33 open; next the user's second look at `review/03-drawings/` |
| [S1f.23.2](s1f.23.2_no_key_and_no_grid.md) | **`--models key` and `--models grid` removed**, with the code, tests, script runs, pages and decisions that exist only for them (the user: *"remove key and grid, add a stage for it"*) | **done and merged 2026-11-18**: the build `377e3761` and its record `05bc83b8`, on the branch `s1f.23.2-models-forms`, carried by S1f.23.3's branch and merged into master with it by `34c2d79b` (the help golden as merged, the shape golden unmoved by the merge); its readings filed as Q-M1f.448 to .453, its findings R-7 and R-8 (its (a) is R-5), R-1 and R-3 ended with the code |
| [S1f.23.3](s1f.23.3_test_diffs_its_claims.md) | **`--diff-models` leaves `ein solve`; `ein test` diffs a claim's models against what it found**, under an option, as stats and/or diffs, the near-miss (*one fact off*) kept (the user: *"remove --diff-models as it is now … can we add it as an option for test ?"*) | **done and merged 2026-11-18**: the build `2fd6827f` and its record `b1e66537`, on the branch `s1f.23.3-test-diff` (S1f.23.2's work and master's plans merged in by `dae8b5ea`), merged into master by `34c2d79b`, with `4b152a70` (R-2's two refusal texts, no theory's name; the shape golden re-banked, 30 lines of the five programs the line refuses); its readings filed as Q-M1f.454 to .466, its findings R-9 to R-13 (its (a) is R-5) |
| [S1f.23.4](s1f.23.4_the_kernel_reads_no_theorys_name.md) | **The kernel reads no theory's name, and the built-in symmetry goes**: `program_relations` without `is-a`; `__symmetric__`'s native mirror removed, its programs on `std.algebra`'s rule after an invariance shows they answer alike; a renaming test over the corpus (the user: *"ein.rs must expect the minimal shape of the program … must not pollute the kernel"*, *"let's drop also built-in symmetry"*) | **done and merged 2026-11-18**: the build `c8a5ea5a` and its record `5eae6474`, merged into master by `96244ed4` (the shape golden re-banked, 240 lines of the retired programs gone; the renaming test's `READS` emptied, every view compared), with `e5b99bee` (the decision Q-M1f.436; Q-M1f.7's file renamed); the rulings and readings filed as Q-M1f.436 to .446, its findings R-4 to R-6 |
| [S1f.23.5](s1f.23.5_a_why_reads_its_own_form.md) | **A `:why` reads the form it is written on, `{?n}` its n-th arrow from the relation as 1**: `(relation owns Person Pet :why "a {?3} {?2} a {?4}")` reads *a Person owns a Pet*; a fact of `owns` reads by the fallback; every relation's `:why` renumbered (the user: *"universal renderings of NL"*, "Declaration only", "{?n}, renumbered") | task 1, the change on real programs, done (`11a113cf`); every open point ruled as its default by the user; **task 2, the build, done and merged 2026-11-18**: `3047bbae` (L-105), `fbe2ba66` (L-102), `bd21f0bb` (the build) and its record `fb849a06`, on the branch `s1f.23.5-why`, merged into master by `79dd213b` (the shape golden re-banked on the merged tree, every line one parent's: the branch's `ir`, `load`, `region-trace` and `states` lines, master's `dot[…]` ones); its readings filed as Q-M1f.475 to .482, its findings as R-34 to R-38; L-102 and L-105 built, Q-M1f.58 rewritten |
| [S1f.23.6](s1f.23.6_the_drawings_answers.md) | **The user's answers on the drawings, built**: the layered layout (the relation above its facts), the fold of a fact of two arrows into a loop, no arrow to or from an arrow (a fact a derivation touches never folded), a derivation's contrast and width, R-33's refused slices; every answer filed as ruled | **done and merged 2026-11-18**: the build `ad20a5e0` and `29792a38` (a folded arrow within a layer) and its record `2244c95e`, on the branch `s1f.23.6-answers`, merged into master by `547afea5`; the review's `03-drawings/` holds what it built (items 56 to 60) for the user's look; its twelve readings filed as Q-M1f.487 to .498, each defaulted |
| [S1f.23.7](s1f.23.7_a_slice_by_a_query.md) | **A slice of the knowledge base defined by a query, and drawn**: the constraint view's slice stated in Ein, any goal's slice drawn (the user's addition to Q-M1f.426) | planned 2026-11-18; task 1, the examples, done and **accepted 2026-11-19** (*"S1f.23.7 - I accept"*), its ten open points ruled as their defaults, Q-M1f.523 to .532; its findings filed as R-39 to R-41, R-39 and R-40 fixed by task 2; **task 2, the build, runs after S1f.23.10 is merged** |
| [S1f.23.8](s1f.23.8_save_and_load_answers.md) | **The user's answers on save and load, built**: a node's container opens as the node (Q-M1f.413 (c)); `ein test` runs nothing for a saved node (Q-M1f.416 (b)); the carried-module note dropped (D-24); every answer filed | **done and merged 2026-11-18**: the build `86d46410` and its ledger `d07eb5bb`, on the branch `s1f.23.8-save-load`, merged into master by `aad59286`; its fourteen readings filed as Q-M1f.499 to .512, each defaulted (a fifteenth, a note, not filed) |
| [S1f.23.9](s1f.23.9_numerals_answers.md) | **The user's answers on the numerals' check, built**: every numeral not produced by an operation from valid ones checked, wherever it comes from (Q-M1f.405); the walk in `std.math.positional` (Q-M1f.408 (b)); validity composed decimal → integer → rational, a rational's natural part checked (L-101); every answer filed | **done and merged 2026-11-19**: the build `90e88122`, `f4378b23` and its record `1f6ec77e`, on the branch `s1f.23.9-numerals`, merged into master by `04433406` (the rule pages and drawings written again by the merged binary) and `04b0e2f6` (the shape golden re-banked on the merged tree); Q-M1f.405 to .411 and L-101 ruled and built; its nine readings filed as Q-M1f.513 to .521, each defaulted (a tenth, a note, done in `557ad664`) |
| [S1f.23.10](s1f.23.10_language_answers.md) | **The user's answers on the language and three decisions' sentences, built**: a relation is any atom first in a tuple (V-7, Q-M1f.522), one notion for the engine's four lists; Q-M1f.362's and Q-M1f.119's sentences and two pages' stale sentences brought to the code (L-92, L-94, L-93); N-15 filed in F21, Q-M1f.412's question answered | **started 2026-11-19** on the user's word, in a worktree |
| [S1f.23.11](s1f.23.11_drawings_second_answers.md) | **The user's answers on the drawings S1f.23.6 built**: the layers, a folded arrow within a layer, the two-arrow fold and R-33's check stand (Q-M1f.487, .488, .490, .491, .497); a derivation's colours, the derived fact brightest in its derivation's hue, no colour near black (Q-M1f.492, .493); a concentric layout, examples for the user's look (items 56, 57) | **started 2026-11-19** on the user's word, in a worktree beside S1f.23.10 |
| S1f.23.n | the next batch of the review's rulings | planned when ruled; run on the user's word |
| S1f.23.z | **The close.** | last |

**The checks on the merged tree, 2026-11-18**, after `e9e640f6` (R-16 and R-17), on AC: `./run_tests.sh` on
`2e3684ec` **exit 0, 22 m 19 s** (a first try on `e9e640f6` stopped at the link check after 6 s, on F20's move into
P1h.6, then uncommitted in the shared tree, committed as `2e3684ec`); `utils/ci_container.sh` on `fd8da416` **exit 1
after 7 s** at the link check, 2 574 findings, every one in the 26 pages of [`../review/`](../review/README.md) that
link their runs' ignored outputs, which a clean clone does not have (known, `c8b26770`: the folder is removed before a
push), then `utils/ci_container.sh --tests-only` there **exit 0, 21 m 13 s**, and the other static steps and the bench
smoke in the same image on the same clone **exit 0, 51 s**; `./run_tests.sh --nightly` on `fd8da416` **exit 101 after
9 m 51 s**, at `deep-corpus: dot_wellformed, every program` (264.9 s): 6 of 8 821 views refused, the six that draw the
rule `column-wrong` of `tests/stdlib/math/digits/02_a_core_names_its_rows.ein`, whose `≠` link Graphviz 16.0.0 and the
runner's 14.1.2 amd64v3 alike cannot route (*Unable to reclaim box space in spline routing*; the drawing before the
phase routes it, and the link written as an `xlabel` routes under both); the nightly's later steps not run. Each
earlier step under 45 s. That step's time is `examples/arith/long_numbers.ein`'s seven views of 730 to 772 edges, 18 to
64 s each under `dot`, one after another in one worker: 265.5 s on four P-core threads as on sixteen
(`bench_env.sh --cores PT:4`), and per commit too (259.6 s).

**The final checks after S1f.23.5's merge, 2026-11-18**, on AC: `./run_tests.sh` on `14fa2636` (the merge `79dd213b`
and its filing) **exit 0, 19 m 22 s**, 162 test binaries, 1 350 tests, none failed; `utils/ci_container.sh
--tests-only` on `23539de5` (the main session's plans-only commits on top, with `.gitignore`) **exit 0, 20 m 37 s**,
the same 1 350 tests; the other static steps and the bench smoke in the same image on the same clone **exit 0, 50 s**
but the link check, whose 2 551 findings are every one a *file missing* in the review bundle's pages, 2 184 in
[`../review/`](../review/README.md) and 367 in [P1f.21's](../p1f.21_verdicts_as_rules/review/README.md), each target
one of 1 942 ignored outputs the root tree holds and none tracked, and none elsewhere; every decision id resolves.
`./run_tests.sh --nightly` on `23539de5` **exit 0, 37 m 56 s**, every step under four minutes on this host's every
thread: the longest `answer_round_trip`'s shard 2/4 (225.6 s), then its shards 3/4 (159.6 s) and 4/4 (149.5 s);
`dot_wellformed, every program` 120.6 s (264.9 s before R-32's fix); each `stdlib_mutants.py` shard about 50 s.
S1f.23.6's stage ran `dot` in its worktree alongside the nightly.

## Rules of the phase

The standing ones ([`AGENTS.md`](../../../AGENTS.md)): a stage per subagent; a claim about the engine is a run; what a
stage finds outside its scope goes to this phase's [`findings.md`](findings.md) (R-n) and is shown; no estimate is
written. A reading a stage takes where the ruling is silent is put in `open_questions.md` with its default, and the
bundle shows it, so that the user's next look decides it.
