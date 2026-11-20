# 84 · The classic zebra's structure under the union

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the finding](../../../findings.md#v-48--the-classic-zebras-structure-under-the-union)

A finding of the phase that is **yours to call**. It was on the findings page alone; it is here so that everything
that waits for you is in one place. Nothing waits on it. It is why `zebra.ein` costs 2.1 times what it did in [the final table](../../../costs/README.md#the-final-table-before-after-improved-after).

## What it is

`examples/zebra.ein`'s $H$ holds each placement beside its mirror since the union: 112 candidates where it held
56, its rule proposing one orientation of the symmetric `co-located` and its obligation the other. The answer is
the same one state. What moved is the cost and the structure: describing $R$ takes 353 saturations where it took
120, and `ein render h-space` reads 352 groups and no grid where it read 36 groups and five grids, and 33 305
coupling edges where 386. The structure view reads two candidates that imply each other as two.

## The question

- **(a)** As built: $H$ is what the union gives, and the structure view shows it. **Default.**
- **(b)** The structure view folds two candidates that imply each other into one, so the zebra reads as it did.
- **(c)** `zebra.ein` is written so that one writer proposes its placements, and its $H$ is 56 again.

## On the built engine

Taken by [S1f.21.19](../../../s1f.21.19_the_review_items_on_the_built_engine.md) on two engines: the final binary,
the release build of `7b72bd84` (`runs/version.txt`), and the engine before the phase's second part, `ddf09492`, run
on its own corpus and library (`runs/old/version.txt`; its runs are under `runs/old/`). The tool is `tools/run.sh`,
with `tools/gen.py` and `tools/folds.py` beside it; `runs/` and `tools/` are in the main checkout beside this page,
not in git.

Both programs are the corpus's, [`examples/zebra.ein`](../../../../../../examples/zebra.ein) and, for contrast,
[`examples/zebra2.ein`](../../../../../../examples/zebra2.ein). Each has, on each engine: `ein show hyps` and a count
of its mirrored pairs (`X.mirrors.txt`), the read-out and `-p`, `ein saturate --dump`, `ein render h-space` (its
read-out, and `--dump`: `h-space.json` and the four graphs as `.dot` and `.svg`), the summary and the events, and
the knowledge base and the lattice drawn. Not taken: the oracle and the region render, H being 112 and 56 candidates,
past the oracle's 16. The final binary's event stream of `zebra.ein` is 103 321 662 bytes and is not kept, its
summary is (`runs/zebra.events.dropped.txt`). `runs/drawn.txt` names every graph drawn and the seconds it took: every
graph is drawn but two of the final binary's `zebra.ein`, `zebra.h-space/coupling.dot` (33 628 lines) and
`coupling_one_step.dot` (39 561 lines), which Graphviz did not lay out within the tool's 600 seconds each; they are
kept as `.dot`, with no `.svg`. The two rewrites below are kept as `.dot` alone.

**The finding's figures beside the engines'.**

| `examples/zebra.ein` | the finding's figure, before → under the union | `ddf09492` | the final binary | where |
|---|---|---|---|---|
| H | 56 → 112 | 56 | 112 | `runs/old/zebra.show-hyps.txt`, `runs/zebra.show-hyps.txt` |
| a placement beside its mirror, `(co-located a b)` beside `(co-located b a)` | none → each | 0 pairs | 56 pairs, all 112 candidates in one | `X.mirrors.txt` |
| saturations to describe R | 120 → 353 | 120 | 353 | the `work` row of `X.solve.txt` |
| groups | 36 → 352 | 36 | 352 | the `cover` row of `X.h-space.txt` |
| grids | five → none | 5 | 0 | its `grids` row |
| coupling edges | 386 → 33 305 | 386 | 33 305 | its `coupling` row |
| root's layer, facts beyond root | 181 088 → 705 808 | 181 088 | 705 808 | its `work` row |

Every figure of the finding is what the binary gives.

**The rest of what moved.** The same files' rows; `zebra2.ein` beside it, which moved in nothing the structure reads.

| | `zebra.ein` on `ddf09492` | `zebra.ein`, final | `zebra2.ein` on `ddf09492` | `zebra2.ein`, final |
|---|---|---|---|---|
| H's writer (`ein show hyps`' note) | ``built by the rung `hrules` `` | ``from `union` `` | ``built by the rung `hrules` `` | ``from `union` `` |
| candidates dead alone | 18 | 36 | 21 | 21 |
| R | 1 state, 18 candidates, 261 982 commitments | 1 state, 36 candidates, 68 719 459 314 commitments | 1 state, 261 982 commitments | the same |
| facts beyond root, describing R | 10 638 | 34 060 | 5 411 | 5 411 |
| pairs · implications · equivalence classes | 1 540 · 237 · 9 | 6 216 · 1 024 · 17 | 1 540 · 234 · 9 | the same |
| owed groups · groups nothing owes | 18 · 18 | 36 · 316 | 36 · 0 | the same |
| instances L5 does not license, of those root owes | 18 of 36 | 0 of 36 | 0 of 36 | the same |
| bijections · components that are no grid | 0 · 0 | 0 · 5 | 5 · 0 | the same |
| coupling: questions · ties · edges one application back | 36 · 629 · 420 | 352 · 2 554 · 34 309 | 36 · 535 · 495 | the same |
| debts that are edges (unlicensed) | 18 (18) | 0 (0) | 0 (0) | the same |
| the small KB's layer, saturations · facts | 1 597 · 67 244 | 6 273 · 266 960 | 1 597 · 32 086 | the same |

The one state of `zebra.ein` holds the same 66 `co-located` facts on both engines
(`runs/zebra.state-final-vs-old.txt`): the answer did not move.
`zebra2.ein`'s rows differ only in its writer's name and in *the small KB drops 18 of 66 stated facts* against
*of 71*.

**Who writes zebra.ein's H** (`runs/zebra.folds.txt`, read off `ein saturate --dump`'s `open` facts, which name
their writer): the hypothesis rule `guess` writes 125 facts `(co-located T House-i)`, of which 56 are in H (root
holds or denies the rest), and `std.slots.slot-owed-fill` writes the 56 mirrors, `(co-located House-i T)`. Under
`ddf09492` H was the rule's 56 alone.

**For option (b)**, what folding would make of H (`runs/zebra.folds.txt`, from `runs/zebra.h-space/h-space.json`):

| | `zebra.ein`, final | `zebra.ein` on `ddf09492` |
|---|---|---|
| mirror pairs in H | 56: 38 of two candidates alive alone, which imply each other both ways, and 18 of two dead alone, among which no implication is read | 0 |
| H with each mirror pair folded to one | 56 | 56 |
| the structure's equivalence classes (candidates that imply each other) | 17, of 76 candidates, sizes 2 (eight), 4 (four), 6 (two), 8, 10 and 14; 8 of them exactly a mirror pair; as placements, a fact with its mirror one, 1 (eight), 2 (four), 3 (two), 4, 5 and 7 | 9, of 30 candidates, sizes 2, 2, 2, 2, 3, 3, 4, 5 and 7 |
| H with each equivalence class folded to one | 53 | 35 |

Every class of the final binary holds each member's mirror: folded by mirror, its 17 classes are the 9 the engine
before read and 8 of one placement, a fact and its mirror (`runs/zebra.classes-final-vs-old.txt`). So two
readings of (b) differ. Folding a fact with its mirror gives 56 and the classes the zebra had. Folding every two
candidates that imply each other gives 53, not 56: it folds the 9 old classes too, which the zebra did not fold
before, and it leaves apart the 18 mirror pairs of candidates dead alone, among which no implication is read.

**For option (c)**, two rewrites of `zebra.ein`, composed for this item and run on the final binary (the two files
in [`programs/`](programs/), written by `tools/gen.py` from the corpus's file):

| | H (`X.show-hyps.txt`) | writers of root's `open` facts (`X.folds.txt`) | describing R | structure (`X.h-space.txt`) |
|---|---|---|---|---|
| [`zebra.rule-deleted.ein`](programs/zebra.rule-deleted.ein): the hypothesis rule and its five activators deleted | 112, ``from `obligations` ``, the same 112 as `zebra.ein`'s (`X.H-vs-zebra.diff.txt`) | `slot-owed-room` 56 `(co-located T House-i)`, `slot-owed-fill` 56 `(co-located House-i T)` | 1 state, 353 saturations | 352 groups, 0 grids, 33 305 edges |
| [`zebra.activators-reversed.ein`](programs/zebra.activators-reversed.ein): the activators written `(guess co-located House T)` | 112, ``from `union` ``, the same 112 | `guess` 125 `(co-located House-i T)`, 56 in H; `slot-owed-room` 56 `(co-located T House-i)` | 1 state, 353 saturations | 352 groups, 0 grids, 33 305 edges |

So the mirrors are not the rule's doing alone: `std.slots` activates two obligations on `zebra.ein`, and at root
`slot-owed-room` writes the rule's orientation and `slot-owed-fill` the other (the dump names one writer of a
fact two write). Neither rewrite brings H back to 56. `zebra2.ein` is the shape that has 56 under the union, a
relation of its own for each attribute type and no mirror: at its root the dump names the rule `guess` as the writer
of all 125 `open` facts, 25 of each of its five relations, all `(R T House-i)`, of which 56 are in H
(`runs/zebra2.folds.txt`).
