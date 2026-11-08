# S1f.20.2 — The disposition table

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 2 days.
**Status:** not started. It was `T1f.5.20.2` until 2026-11-08.
**Depends on:** [S1f.20.1](s1f.20.1_the_two_checks.md), whose link check counts each page's inbound links.

## What it is

The phase's contract. One row per source page, with:
- its destination path;
- its disposition: *rewritten into*, *evidence* or *deleted*
  ([P1f.20 § The collision](README.md#the-collision-stated-once));
- its inbound link count;
- a one-line reason.

**It is reviewed by the user before a single file moves.**

## The rows

- Every page under `docs/kernel/`, `docs/api/` and `docs/guide/`, and `docs/install.md`.
- Every page under `docs/history/`: 73 on 2026-11-08, and more if M1e2's record has moved in by the time
  this runs.
- The read-out's two kernel pages, `docs/kernel/inference/read_out.md` and `read_out_examples.md`, which
  [S1f.5.16](../p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md) made the live vocabulary and
  golden page. Each moves with its test, its generator's path and its inbound links in one commit, as `rust.md`'s
  region does (default destination `user/reference/`), and neither is rewritten from a plan.
  [`solve_interface.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md) is the one
  user-facing reference still under `plans/`. M1e2's `read_out.md` and `read_out_examples.md` are records from
  S1f.5.16 on, and get no row.
- `docs/history/m1d_satisfiability/ideas.md`, whose row is decided: it moves to `plans/ideas/`.

The inbound counts are produced mechanically, by S1f.20.1's checker. The destinations are chosen by hand.

## The tree redrawn

The phase's [target tree](README.md#the-target-tree) is the map of 2026-08-28. This stage redraws it against
the tree as it is, and [§ What moved since it was written](README.md#what-moved-since-it-was-written) lists
what is known to have moved:
- three pages of M1e2's, `answer_dump.md`, `region_dump.md` and `monotonicity.md`, with no row;
- `search.md`'s source, a walk that has no code since 2026-11-03, where the answer is now the region R and its
  routes;
- `algorithms.md`'s source, `features.md`, whose levers are mostly retired.

## Where it lives

At `docs/ein/README.md` § Where each page went, until the phase closes, and then in the closing commit's
message.

## The method

### Task T1f.20.2.1 — the rows

Every source page, its inbound count, and its destination and disposition.

### Task T1f.20.2.2 — the tree redrawn, and the review

The target tree redrawn and written into `docs/ein/README.md` with the table. **Stop and show the user** the
table and the tree. Nothing moves until the user has read both.

## Acceptance

- Every source page has exactly one row, and every row one disposition.
- No page that is the only written intent for shipped code is *deleted*.
- The user has reviewed the table and the tree.
