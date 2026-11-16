# S1f.20.2 — The disposition table

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Estimate:** 2 days.
**Status:** **done 2026-11-17**: T1 and T2 written 2026-11-16 ([§ Record](#record-2026-11-16)), the inbound column
filled from S1f.20.1's checker, and **reviewed by the user on 2026-11-17**, whose rulings are
[P1f.20 § The user's rulings at S1f.20.2's review](README.md#the-users-rulings-at-s1f202s-review-2026-11-17); the
table takes them where they move a row. It was `T1f.5.20.2` until 2026-11-08.
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
- **Every question ledger** (the user's rulings of 2026-11-15): its decisions go to `docs/decisions/`
  ([S1f.20.8](s1f.20.8_decisions.md)), the ledger itself is *deleted* with its folder.
- **Every finished plan folder** (closed milestones and phases, done followups): *deleted* by
  [S1f.20.9](s1f.20.9_the_history_sweep.md), once its decisions are files and nothing outside `plans/` links it.
- **A page superseded with a banner** has no *kept as record* disposition any more: it is rewritten to the
  current state or deleted (only the current state is documented).

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
table and the tree, with `docs/decisions/`' layout ([S1f.20.8](s1f.20.8_decisions.md)'s T2) and the defaults
[S1f.20.8](s1f.20.8_decisions.md#the-defaults-each-the-users-to-revise-at-s1f202s-review) and
[S1f.20.9](s1f.20.9_the_history_sweep.md#the-defaults-each-the-users-to-revise-at-s1f202s-review) take under
the user's rulings of 2026-11-15. Nothing moves until the user has read them.

## Acceptance

- Every source page has exactly one row, and every row one disposition.
- No page that is the only written intent for shipped code is *deleted*.
- The user has reviewed the table and the tree.

## Record (2026-11-16)

**What was written.** [`docs/ein/README.md`](disposition.md): the three perspectives (provisional),
§ The tree redrawn against the tree of 2026-11-16, § Where each page went, and § For the review. Nothing moved;
no page was rewritten or deleted.

**The rows: 175, every source exactly once.** `docs/kernel/` 48 pages and its 28 diagram files as 5 rows,
`docs/api/` 2, `docs/guide/` 5, `docs/install.md`, `docs/history/` 77 files (73 pages, `lattice_exhaustive_run.log`
and three probe files: more than the 73 of 2026-11-08's count, which counted `*.md` only), 27 pages of finished
plan folders, and 10 ledgers under `plans/`. By disposition: **90 rewritten into, 7 evidence, 77 deleted, 1 moved**
(`m1d_satisfiability/ideas.md`), by the `awk` command in the page's § Counts. Nine plan folders are finished
(the page's § Plan folders gives the line that says so for each), and the rest stay.

**What the table adds to this file's list of rows.** The file names `solve_interface.md` as the one user-facing
reference under `plans/`. It is not the only page of a finished plan that shipped code cites as its specification:
`claim.rs` cites M1e2's `region_examples.md` § 1, `expect.rs` its `region_language.md` § 5, `detector.rs` and
`explore.rs` its `detector_proposal.md`, `monotone.rs` its `site_census.md` § 5, `states.rs` its
`alive_set_reach.md` § 3, and three files M1e2's `read_out.md`, a record. Deleting those folders as they stand
would delete the only written intent for that code, which the acceptance forbids, so every page of a finished
folder cited from outside `plans/` (stage files, ledgers and READMEs aside) has a row: 14 of M1e2's, 9 of P1f.6's,
P1f.10's close, F9, F18 and M1j's README.

**The specification-bearing history pages**, none of them *deleted*: the M1a design contracts (85 Rust
references), the sections of `m1a_rust/README.md` (96), `m1c_external_validation/README.md` (17) and
`m1d_satisfiability/README.md` (15) that the code cites, `shared_state.md` § 4, and M1d's `completeness.md`,
`domain_contract.md`, `obligation_forms.md`, `the_boundary.md` and `hypotheses_from_obligations.md`. Each count is
the inbound checker's `rs` column.

**How the rows were judged.** Each page's opening, banner and section list; who cites it, first by a scratch scan
that resolves every path in a tracked text file, then by S1f.20.1's checker, which the column takes;
for a measurement, whether its instrument runs on today's engine (`utils/README.md` lists the walk's seven, which
re-take only against a binary at or before `7ec8a480`); for a page a test or the binary reads, the test, generator
or help text that moves with it.

**For the user** (the page's § For the review): the decisions' layout, the defaults of S1f.20.8 and S1f.20.9,
nine rows I was least sure of, five questions to decide personally, six sources with no obvious destination, and
six findings outside this stage's scope, each with its witness.

**The inbound counts.** This branch merged `master` at `def265b3` (S1f.20.1 done) and filled the column from
`python3 utils/doc_audit.py --inbound --tracked --target docs/kernel/ --target docs/api/ --target docs/guide/
--target docs/install.md --target docs/history/ --target plans/`, each row the target's `all` (a diagram group
the sum of its files'); the page's § The inbound counts holds the command and the rule. The plan-folder table's
figures are the same command's, less the references from `plans/`. The most-referenced sources are ledgers and
records (M1f's ledger 926, `m1a_rust/README.md` 422, M1e2's ledger 302); of the pages that move,
`defined_behaviour.md` (219), `solution_semantics.md` (158) and `read_out_examples.md` (131).

**The release-voice baseline.** `docs/ein/README.md` is in the check's scope and, as scaffolding, names plans,
stages and dates on purpose: its count is banked in `utils/release_voice_baseline.txt` (`--bless`; the one line
the bless adds), to be brought to 0 when the phase closes and the page is rewritten as the tree's index.
