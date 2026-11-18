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

The user, 2026-11-18, verbatim, after reading [`../review/03-drawings/`](../review/03-drawings/README.md) and the
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
| [S1f.23.1](s1f.23.1_drawing.md) | **The drawing.** Every view of `ein render` draws a fact in the canonical form, and folds a fact of three arrows into a labelled arrow on request; no labelled octagon and no text node is produced, or drawn on any page; the KB and the program views label and number alike; `ein render program` is the program's knowledge base as one graph, its rules and its query; L-107's drawing as ruled; every drawing of the tree and of the review bundle drawn again; the user then reviews `review/03-drawings/` again | **started 2026-11-18** on the user's word |
| [S1f.23.2](s1f.23.2_no_key_and_no_grid.md) | **`--models key` and `--models grid` removed**, with the code, tests, script runs, pages and decisions that exist only for them (the user: *"remove key and grid, add a stage for it"*) | **started 2026-11-18** on the user's word, in a worktree beside S1f.23.1, merged after it |
| [S1f.23.3](s1f.23.3_test_diffs_its_claims.md) | **`--diff-models` leaves `ein solve`; `ein test` diffs a claim's models against what it found**, under an option, as stats and/or diffs, the near-miss (*one fact off*) kept (the user: *"remove --diff-models as it is now … can we add it as an option for test ?"*) | planned 2026-11-18 on the user's word; runs after S1f.23.2 is merged |
| [S1f.23.4](s1f.23.4_the_kernel_reads_no_theorys_name.md) | **The kernel reads no theory's name, and the built-in symmetry goes**: `program_relations` without `is-a`; `__symmetric__`'s native mirror removed, its programs on `std.algebra`'s rule after an invariance shows they answer alike; a renaming test over the corpus (the user: *"ein.rs must expect the minimal shape of the program … must not pollute the kernel"*, *"let's drop also built-in symmetry"*) | **started 2026-11-18** on the user's word, in a worktree |
| [S1f.23.5](s1f.23.5_a_why_reads_its_own_form.md) | **A `:why` reads the form it is written on, `{?n}` its n-th arrow from the relation as 1**: `(relation owns Person Pet :why "a {?3} {?2} a {?4}")` reads *a Person owns a Pet*; a fact of `owns` reads by the fallback; every relation's `:why` renumbered (the user: *"universal renderings of NL"*, "Declaration only", "{?n}, renumbered") | task 1, the change on real programs, done (`11a113cf`); every open point ruled as its default by the user; **the build runs after the other stages are merged**, on the user's word |
| S1f.23.n | the next batch of the review's rulings | planned when ruled; run on the user's word |
| S1f.23.z | **The close.** | last |

## Rules of the phase

The standing ones ([`AGENTS.md`](../../../AGENTS.md)): a stage per subagent; a claim about the engine is a run; what a
stage finds outside its scope goes to this phase's [`findings.md`](findings.md) (R-n) and is shown; no estimate is
written. A reading a stage takes where the ruling is silent is put in `open_questions.md` with its default, and the
bundle shows it, so that the user's next look decides it.
