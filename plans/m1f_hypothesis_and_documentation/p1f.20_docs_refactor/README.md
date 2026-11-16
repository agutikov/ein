# P1f.20 — `docs/ein/`: the tree a released system would have

**Milestone:** [M1f](../README.md)
**Estimate:** ~5 weeks — 7 stages, **25 days**: the sum of the split the stage carried, which had put the
whole at ~23.
**Status:** created 2026-11-08 on the user's instruction: *"convert
plans/m1f_hypothesis_and_documentation/p1f.5_documentation_and_other/s1f.5.20_docs_refactor.md into new phase
p1f.20"*. **In progress** since 2026-11-16: [S1f.20.1](s1f.20.1_the_two_checks.md), [S1f.20.2](s1f.20.2_the_disposition_table.md)
and [S1f.20.8](s1f.20.8_decisions.md) are done; S1f.20.3 to S1f.20.5 are next.
**Was S1f.5.20**, a stage of [P1f.5](../p1f.5_the_no_owner_items/README.md), until 2026-11-08. Since it
was written on 2026-08-28, its § How to cut it had proposed splitting it into seven stages and left taking
the split to the user. This phase takes it. No stage had run, so nothing shipped is renumbered:
`T1f.5.20.1`–`.7` are `S1f.20.1`–`.7`, in the same order.
**Depends on:** M1e's [S1e.2.2](../../../docs/history/m1e_review_processing/README.md#s1e22--code--doc-the-canonical-tree),
[S1e.3.7](../../../docs/history/m1e_review_processing/README.md#s1e37--code--doc-consistency-medium) and
[S1e.3.8](../../../docs/history/m1e_review_processing/README.md#s1e38--documentation-medium). The dependency
was hard: those three stages triaged and fixed 38 pages, and a page fixed and then moved is cheap, where a
move followed by a re-triage of 38 renamed files is not. **Satisfied 2026-08-31**: all three ran. It runs
after [P1f.5](../p1f.5_the_no_owner_items/README.md), which edits kernel pages this phase moves. M1f follows
[M1e2](../../m1e2_correctness/README.md), so this phase runs on the tree M1e2 leaves
([§ What moved since it was written](#what-moved-since-it-was-written)).
**Blocks:** nothing. Everything downstream of it wants it and nothing waits on it.
**Source:** the user's note of 2026-08-28, restated in [§ The instruction](#the-instruction), and the user's
rulings of 2026-11-15 ([§ The user's rulings](#the-users-rulings-2026-11-15)), which add the history sweep,
[S1f.20.8](s1f.20.8_decisions.md) and [S1f.20.9](s1f.20.9_the_history_sweep.md), and override this page and
every stage file where they differ. **The user's rulings at S1f.20.2's review (2026-11-17)**
([§ below](#the-users-rulings-at-s1f202s-review-2026-11-17)) override them in turn where they differ.

---

## The user's rulings at S1f.20.2's review (2026-11-17)

Shown the disposition table ([`docs/ein/README.md`](../../../docs/ein/README.md)), the decisions' layout
([`s1f.20.8_layout_proposal.md`](s1f.20.8_layout_proposal.md)), S1f.20.8's and S1f.20.9's defaults and
[Q-M1f.314](../open_questions.md#q-m1f314--what-the-release-voice-check-reads-and-what-fails-a-line), the user
answered, verbatim:

> The tree
> 1. standard_of_proof.md: page under docs/ein/
> 2. Measurements: keeps the censuses
> 3. P1f.6's results pages: rewrites them into stdlib.md and a guide page.
> 4. The deleted search's seven measurement scripts in utils/ - delete
>
> Decisions
>
> 5. Layout: one file per decision in 20 subject folders, <id>-<slug>.md.
> 6. Process-only decisions belong to plans. These are decisions about development process. Development process
>    is outside of the project: process{plans/, ..., project{code, docs/}}; So wrong answers are also tracked in
>    plans, because they are temporary wrong, when fixed they became tests, decisions, whatever else.
> 7. Local numbers: give them new ids: Q-M1e.23–.26, Q-M1e2.29, Q-M1f.315–.340.
> 8. Rulings with no id: take the next free id of the milestone that ruled them.
> 9. Q-M1e.4's "date" warrant goes: a number in prose carries a command or a test, here and in AGENTS.md.
> 10. Open questions: the 10 in docs/history/ ledgers move to plans/open_questions.md if they are open, otherwise
>     into decisions; All open questions live in plans. No open question is referred from code/comments or docs.
>     If something refers the open question then it is not open - it may be postponed with justification,
>     waiting for something, or just postponed because of minority.
> 11. The rest, defaults as in § 6 of the proposal page: decided but unbuilt questions stay with their plans;
>     std.math.crypt (P1f.6's #42) goes to a followup; superseded ids that are still cited get rewritten to the
>     id that carries the decision, with no redirect files; colliding open ids take their ledger's prefix when
>     next touched; the 16 answers flagged "verify" are checked against today's engine before their files are
>     written.
>
> Other defaults
>
> 12. S1f.20.9's six defaults as written. "Finished" means the README says closed or done. That deletes M1e2,
>     M1j, P1f.5/.6/.7/.8/.10, F9 and F18. Superseded-with-banner pages are deleted or rewritten. The register
>     stays as test data, stripped of ids and dates. Commit messages are untouched.
> 13. Q-M1f.314: the release-voice check reads every doc comment and every .ein comment, and every ISO date
>     fails a line. A falling count is reported, not failed. F/E/W/K ids are not read yet.

And, the same day, on item 6:

> plans is a folder inside the project root, but conceptually it is external to the project because the project
> is in some state and plans see different states, evolution

What they change:

- **The dependency runs one way.** `plans/` may read the project, which it sees in many states; nothing in the
  project (code, tests, comments, docs, decisions) reads, links or names `plans/`. The release-voice check is
  this rule's gate for prose; a test or script of the project that reads `plans/` breaks it too.
- **Process decisions are not carried** into `docs/decisions/` (item 6): the 64 the census classes `process`, and
  Q-M1f.36's register reading with them, which the proposal had carried. They stay with their plans and leave
  with them; a process rule still in force (the register's filing rules) is restated in `plans/`, where the
  process lives.
- **The wrong-answer register splits** (items 6 and 12, read together with the second ruling; the coordinator's
  reading, the user's to correct): an `open` row is a wrong answer, a work item, so it and its witness move to
  `plans/`, with whatever re-runs them; a `fixed` row is a regression test and stays in the project as test data,
  stripped of ids and dates. No project test holds an open row any more, since that would read `plans/`.
  S1f.20.9 does it.
- **An open question cited by the project is not open** (item 10): each of the 9 open ids cited outside `plans/`
  is written as a decision of what holds today (postponed, with why, or what it waits for, in the release voice),
  and an id that is truly open is cited by nothing outside `plans/`. The 10 open rows of `docs/history/`'s
  ledgers are read the same way: a decision, or a row of `plans/open_questions.md`, ids kept.
- **`standard_of_proof.md` is a page**, `docs/ein/standard-of-proof.md` (item 1); Q-M1e.1 and Q-M1e.2 stay
  decisions, which the page states.
- **The walk's seven instruments** in `utils/` are deleted at S1f.20.9 (item 4), with what only they read.
- **Q-M1e.4's date warrant goes** (item 9), in `AGENTS.md` now.
- **Ids** (items 7, 8): the map is fixed at Q-M1e.23 to .26, Q-M1e2.29 and Q-M1f.315 to .340; a ruling with no id
  takes the next free id of its milestone after those, Q-M1f.341 on for M1f's.
- Everything else as the defaults were written; Q-M1f.314 is ruled as defaulted.

**The boundary, the same day.** The user: *"yes, utils/ containing tooling used for development also should
belong to process"*; then, asked where the boundary runs: **process** is `plans/`, `utils/`, the gate and CI
(`run_tests.sh`, `.github/workflows/`) and `AGENTS.md` (so `CLAUDE.md`); *"c/, nl2smt/, docs/lib - are outside of
the process also, like external resources"*, neither project nor process; **the project** is the rest: `README.md`,
`ein.rs/` with its tests, `stdlib/`, `examples/`, `tests/`, `corpus/`, `docs/` but `docs/lib/`. And:

- **The censuses kept (item 2) live with their tools**, on the process side: no project page states a measured
  number or names a `utils/` command. `docs/ein/ein.rs/benchmarks/` is not built; the table's *evidence* rows go
  beside their scripts.
- **A generated project file says it is generated** and not edited by hand, and names no tool; which script
  writes it is recorded on the process side (`utils/README.md`).
- **A crate test that reads a process file or runs a process tool** (`gate_steps.rs` and `nightly_steps.rs`,
  which diff `run_tests.sh` against the workflows; the tests that spawn a `utils/` generator's `--check` *"so that
  it is in the gate"*): the user asked what they are for; **default, the user's to correct**: the diffs become a
  `utils/` check and the generators' checks plain gate steps, so `cargo test` reads only the project. S1f.20.9.
- **Decisions about process are not carried**: the 32 the census files under `gate/`, and every one about a
  `utils/` script, beside the 64 `process` ones. A decision file names only project artefacts in *Held by*.
- **`AGENTS.md` leaves the release-voice check's scope**: it is process, and may point at plans.

**The phase's end, the same day.** The user: *"when everything complete in the phase don't close it, I will
review new docs"*. Every stage runs to its end; then the coordinator stops for the user's review of the new
trees, with no close page, no *closed* status and no deletion of this phase's folder until the user has read them.

## The user's rulings (2026-11-15)

The user, 2026-11-15, verbatim:

> add history sweep into the phase and take those rulings into account while reworking docs
>
> remove complete milestones and docs/history;
> leave only information about current state of the project;
> We don't need development history in docs, it present in git, and we don't need to refer to ususally;
> decisions (questions with answers) - yes and move to doc/decisions;
> only the final one, no decision history;
> decisions grouped logically into files and folders, keep reasonable files size or just put every one into separate file;
> decision and questions may preserve numbering, despite plans removed: Q-M1f.304 remain while M1f completely gone;
> docs and comments may refer decisions, but not dates and not plans;
> plans are only work scheduling, while work may include investigations, analysis and any other work together with development;
> so resulting code+comment+docs(with decisions) must be closed and not connected to plans, because plans are gone after finished

What they change in this plan:

- **Two stages are added, the history sweep.** [S1f.20.8](s1f.20.8_decisions.md) writes every decision, the
  final one only, into **`docs/decisions/`**, ids kept, before the pages are rewritten, so that a page cites a
  decision and not a plan. [S1f.20.9](s1f.20.9_the_history_sweep.md), last, deletes `docs/history/` and every
  finished plan (closed milestones, closed phases, done followups), and brings to 0 every reference outside
  `plans/` to a plan or to a dated development event. It takes over [S1f.20.6](s1f.20.6_dehistoricise_the_code.md)'s
  dissolve of `docs/history/`.
- **A decision id may be cited**; a plan, a stage id or a date may not. [S1f.20.1](s1f.20.1_the_two_checks.md)'s
  release-voice check is built to that rule: decision ids pass, plan paths, plan ids and dated events fail.
- **Only the current state is documented.** [§ The collision](#the-collision-stated-once)'s disposition (b),
  *evidence … with their date*, now keeps a measurement only as a re-takable command and what it gives on
  today's engine; a page superseded with a banner is deleted or rewritten, not kept.
- **Plans are work scheduling only**, investigations and analysis included, and are deleted when finished: no
  page, comment or test may need one to be read.

Each stage's defaults under these rulings are listed in [S1f.20.8](s1f.20.8_decisions.md#the-defaults-each-the-users-to-revise-at-s1f202s-review)
and [S1f.20.9](s1f.20.9_the_history_sweep.md#the-defaults-each-the-users-to-revise-at-s1f202s-review), and are
shown to the user at [S1f.20.2](s1f.20.2_the_disposition_table.md)'s stop.

## The instruction

Five things, and they are not one job:

1. **Re-tree.** `docs/ein/{user,reasoning,ein.rs,overview}`, `docs/lib` kept,
   `docs/history` **removed**. Reshape `docs/api`, `docs/guide`, `docs/kernel`,
   `docs/install.md` into it — *"connect each document to the new docs tree,
   but not copy. Rewrite instead"* — dropping
   references to plans / open questions / history, referencing only `ein.rs/`
   and `docs/ein/`, and separating three perspectives: **user surface**
   (problem → solve-as-a-black-box → result), **implementation-agnostic
   reasoning design**, **the concrete `ein.rs` implementation**.
2. **De-historicise the code.** In code, tests and docs, replace
   *"totality OBLIGATIONS: total-owed / surjective-owed (M1d P1d.2 S1d.2.4)"*
   with a reference to a design page — so that *"code, comments and docs [are]
   considered as a complete released state of the software produced from design
   docs, rather than an intermediate development state produced by a number of
   decisions made during the development process."*
3. `docs/ein/overview` — a feature comparison with z3, CVC4, LEAN, K, OpenCog,
   Prolog, Certora.
4. `docs/demo` — the article and presentation *"From Zebra puzzle to
   Autoformalization"*.
5. **EinAF** — LLM autoformalization, semantic analysis, theory synthesis, BBH.

**(3), (4) and (5) already have owners**, and this phase does not take them:
[M5](../../m5_presentation/README.md) Track A *is* the comparison, written
against [`docs/lib/`](../../../docs/lib/README.md)'s twelve catalogue files;
M5's outputs are the paper and the talk, and the paper is Stage P of
[`EinAf.md`](../../m2_nl_to_ir/EinAf.md); (5) is [M2](../../m2_nl_to_ir/README.md)
whole, with BBH at [P2.3](../../m2_nl_to_ir/p2.3_benchmark/README.md) and
[F13](../../followups/f13_puzzles_beyond_zebra/ideas.md). What they lack is not
a plan — it is a **home in the doc tree**, and this phase builds the two
directories and the index rows, empty, so the milestone that fills them has
somewhere to put the file. That is [S1f.20.7](s1f.20.7_the_two_empty_homes.md), and it is one day.

**This phase owns (1) and (2).**

## Context — the size of it

Measured at `9ba2349`, 2026-08-28. [S1f.20.1](s1f.20.1_the_two_checks.md) re-takes it, with the commands
written down, before anything moves.

| tree | files | lines | carries a milestone id | links into `plans/` or `docs/history/` |
|---|---:|---:|---:|---:|
| `docs/kernel` | 38 | 12 724 | 498 lines, 36 files | 105 lines, 24 files |
| `docs/api` | 7 | 1 412 | 41 lines | 19 lines |
| `docs/guide` | 5 | 693 | **1 line** | 0 |
| `docs/install.md` | 1 | 176 | 3 lines | 3 lines |
| `docs/history` | 43 | 20 734 | — it is the history | — |
| `docs/lib` | 13 | 2 507 | 0 | 0 — untouched by this phase |
| `ein.rs/crates/**/*.rs` | 193 | — | **963 lines, 175 files** | **469 lines, 142 files** |
| `stdlib` + `examples` + `tests` `.ein` | 197 | — | 303 lines, 104 files | 24 lines |
| `corpus/corpus.toml` | 1 | 1 150 | 47 lines | 1 line |
| `examples` + `stdlib` + `utils` `README.md` | 3 | 441 | 55 lines | 25 lines |
| `AGENTS.md` | 1 | 632 | 78 lines | 59 lines |
| `README.md` | 1 | 645 | 45 lines | 92 lines |

That is **2 034 lines across 330 files carrying a development-history
reference**. Counted the other way — by link rather than by line — the crates
alone hold **464 doc-comment links into `docs/history/`**: 368 into
`m1a_rust`, 69 into `m1d_satisfiability`, 27 into `m1c_external_validation`.

### Re-taken 2026-11-16, by a script

[S1f.20.1](s1f.20.1_the_two_checks.md) re-took the table with `python3 utils/history_census.py`, on the tree
of the commit that banks the script; the table above stays as the 2026-08-28 starting point, whose counting
rule was not written down. The rules are the script's docstring: **files** are the row's tracked files of its
kind (`.md` for a docs tree, so `docs/kernel`'s 14 `.dot` and 14 `.svg` are not counted), **lines** their
newlines; **carries an id** is a line with a milestone, phase, stage or task id by the release-voice check's
patterns, whole lines (a `.rs` file's code and strings too), a decision id `Q-M…` not counted;
**refers into `plans/` or `docs/history/`** is a line with a markdown link that resolves there or a path
written into either, bare or relative; **dates** is a line with an ISO date, a column the 2026-11-15 rulings
add. Each is given as lines and as the files with one.

| tree | files | lines | carries an id | refers into `plans/` or `docs/history/` | dates |
|---|---:|---:|---:|---:|---:|
| `docs/kernel` | 48 | 26 661 | 2 046 lines, 45 files | 585 lines, 40 files | 581 lines, 34 files |
| `docs/api` | 2 | 438 | 22 lines, 2 files | 12 lines, 2 files | 2 lines, 1 file |
| `docs/guide` | 5 | 796 | 9 lines, 3 files | 0 lines, 0 files | 3 lines, 2 files |
| `docs/install.md` | 1 | 226 | 8 lines, 1 file | 9 lines, 1 file | 2 lines, 1 file |
| `docs/history` | 73 | 24 247 | 1 558 lines, 66 files | 1 290 lines, 51 files | 583 lines, 48 files |
| `docs/lib` | 14 | 2 518 | 2 lines, 1 file | 5 lines, 2 files | 0 lines, 0 files |
| `ein.rs/crates/**/*.rs` | 332 | 208 385 | 4 096 lines, 280 files | 449 lines, 186 files | 558 lines, 164 files |
| `stdlib` + `examples` + `tests` `.ein` | 890 | 46 997 | 2 089 lines, 763 files | 34 lines, 34 files | 748 lines, 421 files |
| `corpus/corpus.toml` | 1 | 5 400 | 763 lines, 1 file | 2 lines, 1 file | 302 lines, 1 file |
| `examples` + `stdlib` + `utils` `README.md` | 3 | 2 248 | 212 lines, 3 files | 76 lines, 3 files | 96 lines, 3 files |
| `AGENTS.md` | 1 | 675 | 53 lines, 1 file | 30 lines, 1 file | 7 lines, 1 file |
| `README.md` | 1 | 741 | 97 lines, 1 file | 90 lines, 1 file | 38 lines, 1 file |

**9 397 lines across 1101 files carry a milestone, phase, stage or task id**, every tree but `docs/history`.

The crates' comment links into `docs/history/`: **273** — 197 into `m1a_rust`, 37 into `m1e_review_processing`, 22 into `m1d_satisfiability`, 17 into `m1c_external_validation`; 84 of them into `m1a_rust/design/`. Into `plans/`: 142.

Tracked names outside `plans/` and `docs/history/` that carry an id: 0

The release-voice check over its scope (its baseline): **6 743 lines in 1041 files** — shipped crates' doc comments 3 271, `.ein` comments 2 501, `corpus/corpus.toml` 765, `README.md` 152, `AGENTS.md` 54.

So the tree grew since 2026-08-28 (`docs/kernel` from 12 724 lines to 26 661, the `.ein` row from 197 files to
890 with `tests/`), and so did its history: the id column outside `docs/history` is 9 397 lines in 1 101 files
against 2 034 in 330, under a rule that is now written down and may count more than the old one did. The
crates' links into `docs/history/` fell from 464 to 273 (the links of every comment line, `//` too, resolved
from the file), and they carry 142 links into `plans/`, which the old table did not count by link. No tracked name outside `plans/`
and `docs/history/` carries an id: the test the plan named, `s1a_6_9_resumed_saturation`, is gone.

**The release-voice check's first baseline is its own count over its scope** (6 743 lines in 1 041 files),
not the census rows: it reads only a shipped crate's doc comments, a `.ein` file's comments, `AGENTS.md`
without § Where the work is, and fails a path into `plans/` and a date beside the ids. Its per-file counts are
`utils/release_voice_baseline.txt`; `python3 utils/check_release_voice.py --list PATH` lists the lines.

**Inbound references per target**, which [S1f.20.2](s1f.20.2_the_disposition_table.md)'s table takes for each
source page: `python3 utils/doc_audit.py --inbound --tracked --target docs/kernel/` (any prefix, repeatable),
every reference that resolves by referrer kind (`.md`, with those from `plans/` and `docs/history/` counted
again apart, Rust comment links, `.ein` comment paths, `corpus.toml` paths).

### The collision, stated once

`docs/history` is not only a record. **106 of those 464 links point at
`m1a_rust/design/`** — the eleven contracts
[`AGENTS.md`](../../../AGENTS.md) describes as *"the contracts the crates cite
as their specification"*, and for several modules they are the **only**
statement of intent that is not the implementation.

So: deleting the directory as a directory is right, and deleting its contents
is not. The rule this phase applies — and it is the repo's own, from
[Q-M1e.3](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)
— is **dissolve, don't delete**: every page is triaged into

- **(a) becomes a `docs/ein/` page**, rewritten as an as-built design document
  with the milestone scaffolding removed — the design contracts, and the M1d
  pages the engine is built on;
- **(b) becomes evidence** under `docs/ein/ein.rs/benchmarks/` — the
  re-takable censuses and the measurements, **as a command and what it gives on today's engine**, on a named
  machine (the user's rulings of 2026-11-15: a measurement of an engine that no longer exists is deleted);
- **(c) is deleted**, and lives in git — the phase/stage records; the open-question ledgers, **once their
  decisions are files under `docs/decisions/`** ([S1f.20.8](s1f.20.8_decisions.md)).

After it, `docs/history/` does not exist, no `docs/ein/` page says *M1d P1d.2*,
and no crate links to a file that is gone. That is the user's instruction
carried out; the sentence above is the one place this plan disagrees with a
literal reading of it, and it disagrees by preserving specifications rather
than by keeping a directory.

## What moved since it was written

The plan was written on 2026-08-28. Taken on 2026-11-08:

- **The link check's markdown half exists.** `utils/doc_audit.py --links --check` has been a gate step since
  M1e S1e.3.8 (`DO-M2`), over `docs`, `plans` and the tree's READMEs (`per-commit.yml`). So
  [S1f.20.1](s1f.20.1_the_two_checks.md) extends that script rather than writing a second checker. It does not read
  a Rust doc comment, a `.ein` header or `corpus.toml`, so those three are still unbuilt. So is the
  release-voice check.
- **`docs/history/` grew from 43 pages to 73** (`find docs/history -name '*.md' | wc -l`). M1e's record moved
  in on 2026-10-14, as a fourth entry. M1e2's may be a fifth by the time this phase runs, since M1f follows
  it.
- **`docs/kernel/` grew from 38 pages to 43.** `inference/answer_dump.md`, `inference/region_dump.md` and
  `inference/monotonicity.md` came with M1e2, and the map below has no row for them.
- **Part of the map is stale.** `inference/algorithm_layer_n.md`, `lattice_diagrams.md` and
  `parity_baselines.md` are superseded, with banners. The walk they describe has had no code since M1e2
  S1e2.1c.5 (2026-11-03), so `search.md ← inference/algorithm_layer_n.md` maps a page onto a search that is
  gone: the answer is now the region R and its routes
  ([`solution_semantics.md`](../../../docs/kernel/inference/solution_semantics.md)). `inference/features.md`'s
  levers are mostly retired ([`config.md`](../../../docs/ein/user/reference/config.md) lists them).
- **User-facing references live under `plans/`.** The read-out's vocabulary,
  [`read_out.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/read_out.md), its golden page
  [`read_out_examples.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/read_out_examples.md), and
  [`solve_interface.md`](../../m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md) describe what
  `ein solve` prints, and the map has no row for any of them. Since
  [S1f.5.16](../p1f.5_the_no_owner_items/s1f.5.16_the_read_out_reference.md) the first two are records, and the live
  pages are [`docs/kernel/inference/read-out.md`](../../../docs/ein/user/reference/read-out.md) and its
  [examples](../../../docs/ein/user/reference/read-out-examples.md); `solve_interface.md` is the one left.
- **`CLAUDE.md` is a symlink to `AGENTS.md`.** Both names are one file.

So [S1f.20.2](s1f.20.2_the_disposition_table.md) redraws the target tree against the tree as it is when the
phase runs. The map below is the 2026-08-28 one, kept as the starting point.

## The target tree

The user's outline, with the pages of 2026-08-28 mapped onto it. `→` is *rewritten into*, never *copied*.

```text
docs/
  decisions/                      ← S1f.20.8: every decision, the final one, ids kept; by subject
  ein/
    README.md                     ← new: the three perspectives, and how to read them
    user/                         "problem → [solve as a black box] → result"
      syntax-and-semantics/       every concept in four representations:
                                    ein-lang · graph · NL · math
        objects.md                ← ir/01-ein-graph/01_kb.md (objects half)
        relations.md · facts.md   ← ir/01-ein-graph/01_kb.md · ir/02-data-model/01_entities.md
        ontology.md               ← ir/01-ein-graph/03_ein_model.md
        rules.md                  ← ir/01-ein-graph/02_rules.md
          properties-and-operators.md   ← ir/03-ein-lang/07_stdlib_api.md (the algebra half)
          constraints.md          ← the (false) / (not …) upper bound
          obligations.md          ← history/m1d/obligation_forms.md (the (open ?R) lower bound)
        theory.md                 ← new: a rule set, and how rules interact
        macros.md · query.md      ← ir/03-ein-lang/01_grammar.md, split
        solutions-and-models.md   ← inference/solution_semantics.md  (S1e.5.2, already written)
        config.md                 ← S1e.5.1's configuration reference
      guide/                      ← docs/guide/ 01–04, near-verbatim
        …plus 04_jack_drinks_coffee.md and inference/zebra_walkthrough.md
      reference/
        syntax.md                 ← ir/03-ein-lang/{00_ebnf,01_grammar,02_patterns,06_reserved_names}
        config.md · stdlib.md     ← S1e.5.1 · stdlib/README.md + 07_stdlib_api.md
        cli.md                    ← docs/install.md + inference/events.md + the 50 options
        diagnostics.md            ← kernel/defined_behaviour.md
        glossary.md               ← kernel/glossary.md
    reasoning/                    implementation-agnostic
      static-ir/                  the KB up to the AST state
        names-and-atoms.md · relations-and-facts.md   ← ir/02-data-model/{01_entities,02_store}
        rules-as-statements.md    ← ir/01-ein-graph/{02_rules,05_four_level_kb}
      kb-evolution/               after compilation, and onwards
        firing.md · saturation.md ← inference/{README,architecture_and_algorithms}
        absent-and-naf.md         ← inference/absent_semantics.md
        search.md                 ← inference/algorithm_layer_n.md + history/m1a/design/07
        atms-and-nogoods.md       ← inference/architecture_and_algorithms.md (the ATMS half)
        hypothesis-generation.md  ← history/m1d/hypotheses_from_obligations.md
                                    + P1f.10's exclusion / groups results
        satisfiability.md         ← history/m1d/{the_boundary,completeness,the_verdict}
        solution-semantics.md     ← inference/solution_semantics.md (linked, not duplicated)
      rule-evolution/
        rules-as-data.md          ← ir/03-ein-lang/08_self_describing.md
        analysis-of-rules.md      ← new — and S1f.5.15's stratification is its first content
        transformations.md        ← TBD, a stub with the question in it
    ein.rs/
      api.md                      ← docs/api/rust.md (marker-guarded region intact)
      architecture.md             ← kernel/architecture.md + inference/implementation.md
                                    + ir/02-data-model/03_implementation.md
      design/                     ← history/m1a_rust/design/ 02–12, de-scaffolded
      algorithms.md               ← inference/features.md (the levers and what they cost)
      read-outs.md                ← inference/{lattice_dump,lattice_diagrams,events}.md
      benchmarks/                 ← history/m1a/measurements/ + the four M1d censuses
                                    + history/m1c/stdlib_census.md
    overview/                     ← M5 Track A fills it; S1f.20.7 builds it empty
      README.md                   the axes; one page per comparand
      inspirations.md             ← ir/03-ein-lang/05_inspirations.md
  demo/                           ← M5 fills it; S1f.20.7 builds it empty
  lib/                            unchanged
```

One page has no row above, and it is a decision, not an oversight:

- **`docs/history/m1d_satisfiability/ideas.md`** — the user's own, and
  `AGENTS.md` says it is authoritative on intent *"as `plans/ideas/*` is"*.
  **Move to `plans/ideas/`**, which is where the repo already keeps that class.

## Two rules across the stages

The milestone asks for `./run_tests.sh` green at every stage boundary. A refactor spread over seven stages
meets that only by two rules, which the single stage did not need:

1. **A page moves with its inbound links, in one commit.** The link check is a gate step. So each of
   S1f.20.3 to S1f.20.5 rewrites every link to a page it moves, in the commit that moves it, from a `.md`, a
   Rust doc comment, a `.ein` header and `corpus.toml` alike. This is substitution 2 of
   [S1f.20.6](s1f.20.6_dehistoricise_the_code.md), done by the stage that moves the page. S1f.20.6 does the
   other three.
2. **The release-voice check is a ratchet.** [S1f.20.1](s1f.20.1_the_two_checks.md) builds it, and it cannot
   pass before S1f.20.6 has run. So it reads a banked baseline of each file's count, and fails where a count
   rises or a new file appears. Each stage lowers the baseline in the commit that lowers a count. S1f.20.6
   brings every count to 0 and deletes the baseline. `tests/mutation_survivors.txt`, a banked set the gate
   holds, is the precedent.

## Stages

| ID | title | est. | ends with |
|---|---|---:|---|
| [S1f.20.1](s1f.20.1_the_two_checks.md) | The two checks: every link resolves, and the release voice holds | 3 d | the link check reads Rust doc comments, `.ein` headers and `corpus.toml` too; the release-voice check is in `run_tests.sh` as a ratchet; the baseline numbers written down |
| [S1f.20.2](s1f.20.2_the_disposition_table.md) | The disposition table | 2 d | one row per source page, the target tree redrawn, reviewed by the user before anything moves |
| [S1f.20.3](s1f.20.3_user.md) | `docs/ein/user/` — the black box | 5 d | the black-box perspective, guide included |
| [S1f.20.4](s1f.20.4_reasoning.md) | `docs/ein/reasoning/` — implementation-agnostic | 6 d | the implementation-agnostic perspective |
| [S1f.20.5](s1f.20.5_ein_rs.md) | `docs/ein/ein.rs/` — the concrete implementation | 3 d | the concrete one, `rust.md`'s marked region intact |
| [S1f.20.6](s1f.20.6_dehistoricise_the_code.md) | De-historicise the code, and dissolve `docs/history/` | 5 d | 0 ids outside `plans/`; `docs/history/` gone; `cargo doc` green |
| [S1f.20.7](s1f.20.7_the_two_empty_homes.md) | The two empty homes | 1 d | `overview/` and `demo/`, indexed and owned |
| [S1f.20.8](s1f.20.8_decisions.md) | The decisions, in `docs/decisions/` (the history sweep, first half; 2026-11-15) | — | every decision a file, the final one, ids kept; every cited id resolves. **Done 2026-11-17**: 348 files in 19 folders, the census held to them both ways, every decision id the project cites resolving in the link check's gate step |
| [S1f.20.9](s1f.20.9_the_history_sweep.md) | The history sweep: finished plans and `docs/history/` removed, nothing outside `plans/` refers to a plan or a date (2026-11-15) | — | 0 such references, the check with no baseline; only unfinished work under `plans/` |

**The order.** S1f.20.1, then S1f.20.2, before anything moves; S1f.20.2's review shows the decisions' layout
and the sweep's defaults too. Then **S1f.20.8**, the decisions, so the rewritten pages cite them. Then
S1f.20.3, S1f.20.4 and S1f.20.5, in any order, each moving its pages with their links. S1f.20.4's `analysis-of-rules.md` is sequenced with
[S1f.5.15](../p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md), so the page is written once. S1f.20.6
runs after all three. S1f.20.7 can run any time after S1f.20.2, since it moves `05_inspirations.md`.
**S1f.20.9 runs last**: the deletions, and the references brought to 0.

The stage's own § Ordering said to run after M1e's S1e.2.2, S1e.3.7 and S1e.3.8. Those three triaged 38 kernel
pages into current, bannered or history and fixed what drifted; this phase moves and rewrites the survivors.
Reversing the order would mean every finding lands on a path that no longer exists, and the tree is triaged
twice. That is satisfied. The same rule holds for M1e2's edits to kernel pages: fix first, then move.

## Acceptance for the phase

- **`docs/history/` does not exist**, and no reference to it does — not in a
  `.md`, not in a Rust doc comment, not in a `.ein` comment, not in
  `corpus.toml`, not in `AGENTS.md`.
- **`docs/decisions/` holds every decision, the final one only**, ids kept, and every decision id cited
  outside `plans/` resolves to it ([S1f.20.8](s1f.20.8_decisions.md)).
- **Nothing outside `plans/` refers to a plan, a plan id or a dated development event**, and `plans/` holds
  only unfinished work ([S1f.20.9](s1f.20.9_the_history_sweep.md)).
- **`docs/ein/` exists with the three perspectives**, every page reachable from
  `docs/ein/README.md`, and every source page accounted for in the disposition
  table by exactly one of *rewritten into / evidence / deleted*.
- **Two checks are in `./run_tests.sh`**: the link check, over every tracked `.md`, every repo-relative path
  in a Rust doc comment, and the bare paths in `.ein` headers and `corpus.toml`; and the release-voice check,
  with its baseline empty and gone.
- **The history-referencing lines are resolved**, each into one of:
  a link to a `docs/ein/` page, a plain factual statement with no id, or a
  deletion. The per-file count is the census S1f.20.1 takes, and the closing count
  is **0 outside `plans/`**.
- `./run_tests.sh` green at every stage boundary, and `RUSTDOCFLAGS="-D warnings" cargo doc` green. The
  second matters more than usual: 464 doc comments change, and rustdoc's own history here is that it had
  never run until M1c S1c.1.5 and found nineteen defects on the first try.
- **`docs/lib/` is untouched**, and `docs/demo/` + `docs/ein/overview/` exist
  as indexes with their owning milestone named in the first line.

## Risks

- **The tree is rebuilt while another milestone edits it.** This was M1e's three stages when the plan was
  written. It is now [P1f.5](../p1f.5_the_no_owner_items/README.md), M1e2's P1e2.5 until 2026-11-08, which
  edits kernel pages: S1f.5.5 generates `00_ebnf.md`'s blocks, and S1f.5.11 removes `:where` from six pages.
  P1f.5 runs first, so the order holds: fix first, then move. If the user wants this phase first, those
  stage files must be re-aimed at the new paths *before* it starts, not afterwards.
- **A rewrite that loses a specification.** Half of `docs/history` is the only
  written intent for code that ships. The disposition table exists to make each
  such loss a decision with a name on it; the failure mode is a page rewritten
  into a summary of itself. The repo already states the rule —
  *a page rewritten to describe the current engine is neither history nor a
  specification* — and this phase is the largest opportunity to break it.
- **The release-voice check is a blunt instrument.** A grep for `S1a.` will
  hit a test named `s1a_6_9_resumed_saturation`, a golden file name, and a
  changelog. Scope it to doc comments and `docs/ein/`, allow an explicit
  `release-voice-ok:` escape the way `check_hashmap_iteration.py` allows
  `determinism-ok:` — and note that that check's *first finding was the check
  itself*, which is the failure mode to expect here too.
- **Scope, honestly.** This is 25 days of work that fixes no finding and moves no answer. M1f's acceptance
  is unchanged if it is cut whole.

## Connections

- [`AGENTS.md`](../../../AGENTS.md) — the largest single consumer of the
  current tree, and the file that will read most differently afterwards. `CLAUDE.md` is the same file.
- [`README.md`](../../../README.md) — the root page, which already keeps
  **Ein** and **EinAf** apart and is the natural top of `docs/ein/`.
- [Q-M1e.3](../../../docs/history/m1e_review_processing/open_questions.md#q-m1e3--who-owns-a-page-that-should-be-neither-fixed-nor-deleted)
  — *who owns a page that should be neither fixed nor deleted* — the ruling
  this phase's triage is an application of, at scale.
- [S1e.5.1](../../../docs/history/m1e_review_processing/README.md#s1e51--the-configuration-reference) and
  [S1e.5.2](../../../docs/kernel/inference/solution_semantics.md) — two pages
  written into the *old* tree; both are `docs/ein/user/` pages by this
  phase's map, and both are cited by anchor text rather than section number so
  the move does not break them.
- [S1f.5.15](../p1f.5_the_no_owner_items/s1f.5.15_rule_priority.md) — writes the first content of
  `reasoning/rule-evolution/analysis-of-rules.md`.
- [M5](../../m5_presentation/README.md) Track A and
  [`EinAf.md`](../../m2_nl_to_ir/EinAf.md) — the owners of items 3, 4 and 5 of
  the instruction, and the reason this phase builds two empty directories
  instead of five populated ones.
