# S1f.10.16 — What the walk left in the generator

**Phase:** [P1f.10](README.md)
**Status:** planned 2026-11-15, at the re-plan. Not started.
**Depends on:** nothing. It runs with whichever batch of code commits it fits.
**Takes over:** the old [S1f.10.5](s1f.10.5_ordering.md)'s *`most-constrained`, implemented or deleted*, and
the old [S1f.10.7](s1f.10.7_tree_calibration_and_flag.md)'s call on `generate_one_branch`.

## Context

The walk was deleted on 2026-11-03 and the generator stayed, because H is built by its ladder. Some of what
stayed has no reader on any command's path. This phase is about to write code beside it and pages about it, so
it settles each piece first: deleted, or kept with the reader that needs it named at the item. An argument for
leaving something alone holds only while its premise is enforced by something that fails
([`standard_of_proof.md`](../../../docs/kernel/standard_of_proof.md)).

## The items, as found on 2026-11-15 at `dbedc535`

| | item | what was found |
|---|---|---|
| 1 | `hypgen::generate_one_branch`, and the `one_branch` argument it threads through `generate_rungs`, `generate_with` and `oblgen::generate` | `pub`, and nothing calls it. It was the walk's tree's generator. The obligation tree reads `oblgen::alternatives` instead |
| 2 | `hypgen::score_hypothesis`, re-exported from `lib.rs`; the mode `most-constrained`, which returns `0.0`; `hypgen-scoring`, `hypgen-rel-weight`, `hypgen-obj-weight` | called by its own tests. The fields are in `config.rs`'s list of retired names with the reason *an order lever, and every route's order is part of its definition*, and still in its `FIELDS` |
| 3 | `Lookahead`, the one-step simulator, and `enable-pre-branch-lookahead` | reached through `hypgen::generate`, which no command runs: the universe is taken by `generate_config_free`. Its readers are the shape digest's `hyp` op, the `hypgen_calls` example and tests |
| 4 | `lattice-order`, `lattice-order-seed`, `candidate-order-seed` and the other names retired at the switch that `FIELDS` still holds | what a program that sets one gets today is for the stage to read off `config_reference.rs` and state |
| 5 | module headers that describe the walk as the engine's search: `oblgen.rs`, § What it proposes (*a breadth-first lattice over root's `alive` set*); `hypgen.rs`, § What must not change (*it becomes `layer_1`'s singleton order*); `oblgen.rs`'s note on `apriori::order_candidates` | stale statements of what the engine does |

## Tasks

1. **T1f.10.16.1 — the readers.** For each item, every reader in the workspace named: a command's path, a
   test, a golden, an example, a page. For 3, in particular: which goldens hold the `hyp` shape
   (`corpus_shapes.md5`), and whether that op digests anything a route uses.
2. **T1f.10.16.2 — the dispositions**, each by one rule: **a piece no command's path, no golden and no page a
   user reads depends on is deleted with its tests; a piece one of them depends on is kept, and the dependence
   is written at the item.** A golden that would move is named here before it moves. If the `hyp` op digests
   only a generator no command runs, that is a finding about the digest, shown, and the op is not removed here
   without the user's word: it is part of the per-commit shape check.
3. **T1f.10.16.3 — the headers.** Item 5's three, and any other T1 finds, restated to say what the engine
   does, with no stage id and no date.
4. **T1f.10.16.4 — the pages.** [`configuration.md`](../../../docs/kernel/configuration.md)'s rows for every
   name touched, pinned by `config_reference.rs`; `docs/api/rust.md` if a `pub` item it shows goes.

## Acceptance

- Each item has a disposition recorded here, with its readers.
- `generate_one_branch` and `one_branch` are gone, or the reader that keeps them is named.
- **`most-constrained` is in one of two states**: it computes something a reader uses, or it does not exist. A
  documented mode that returns a constant does not survive the stage.
- No answer and no read-out changes; a golden that moved is on T2's list.
- `./run_tests.sh` green on the batch's last commit.

## What it must not do

- **Not delete the generator's ladder or `generate_config_free`**: H is built by them.
- **Not remove an op from the shape digest** on its own judgment.
- **Not bring a retired lever back** to give a piece a reader.
