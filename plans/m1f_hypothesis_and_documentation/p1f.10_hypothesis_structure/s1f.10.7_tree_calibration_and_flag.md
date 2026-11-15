# S1f.10.7 — ~~Why the tree wins on `zebra2-minus-15`, and the flag that replaces `EIN_TRAVERSAL`~~

**Phase:** [P1f.10](README.md)
**Status:** **dropped at the re-plan of 2026-11-15.** It never ran. The id is used and vacated, and is not
reassigned.

## What it was

Planned 2026-08-28 on the user's instruction of that day: *investigate why tree search works so good on
minus-15 multi-model solution; add a CLI flag to use tree search instead of the env var.* It had two halves:
attribute the **200 053×** in enterings between the walk's two traversals on
`zebra2-minus-15-obligations.ein` (86 against 17 204 592), and promote `EIN_TRAVERSAL` to
`--traversal {lattice,tree}`.

## Why it is dropped

All three of its subjects went with the walk, which M1e2 deleted on 2026-11-03:

- **Both traversals.** No command walks a lattice of commitment sets, and the walk's tree is gone. What
  answers `-n`, `--single` and the lines today is the obligation tree over states, which is every program's
  route and not an alternative to one.
- **`EIN_TRAVERSAL`.** Deleted with them; there is no variable for a flag to replace. And a flag that chose a
  route would be a lever: each route's order is part of its definition, and no option reaches it.
- **The number.** 200 053× compares two pieces of deleted code. The question behind it, *what does branching
  on structure buy?*, is live, and its measurement on today's routes is
  [S1f.10.13](s1f.10.13_the_price_list.md): `zebra2-minus-15`'s describe spends 5 530 of 8 391 saturations on
  alive owing states before the budget stops it, and `--maximal`, which takes the tree, finds the 32 states in
  2 346 (2026-11-15, [README § What was measured at the re-plan](README.md#what-was-measured-at-the-re-plan)).

## Where its remainders went

| what | where |
|---|---|
| `hypgen::generate_one_branch` and the `one_branch` argument, the walk's tree's generator, `pub` and uncalled | [S1f.10.16](s1f.10.16_what_the_walk_left.md) |
| learning from a death, the half of [Q-M1e.5](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e5--is-experimental-a-licence-to-ship-a-lying-surface) M1e2 handed here | a source [S1f.10.13](s1f.10.13_the_price_list.md) prices, *the pair of a death*, and [S1f.10.3](s1f.10.3_the_restricted_join.md) builds if it pays |
| whether describe should take what the tree reaches | a question with numbers, [S1f.10.14](s1f.10.14_exactly_one_spent.md)'s T4 |

The stage file as planned is in `git log -- plans/m1f_hypothesis_and_documentation/p1f.10_hypothesis_structure/s1f.10.7_tree_calibration_and_flag.md`.
