# S1f.20.9 — The history sweep: finished plans and `docs/history/` removed, nothing outside `plans/` refers to a plan or a date

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Status:** planned 2026-11-15 on the user's rulings of that day
([P1f.20 § The user's rulings](README.md#the-users-rulings-2026-11-15)). Not started.
**Depends on:** [S1f.20.8](s1f.20.8_decisions.md), since a ledger is deleted only once its decisions are
files, and [S1f.20.6](s1f.20.6_dehistoricise_the_code.md), which rewrites the code's references. **It runs
last**, and takes over S1f.20.6's dissolve of `docs/history/`.
**Second half of the history sweep**; its first half is [S1f.20.8](s1f.20.8_decisions.md).

## What it does

The user's rule: *"code+comment+docs(with decisions) must be closed and not connected to plans, because plans
are gone after finished"*. So:

1. **`docs/history/` is deleted**, every page of it. What the disposition table marks *rewritten into* has
   been rewritten by S1f.20.3 to S1f.20.5; its decisions are files (S1f.20.8);
   `m1d_satisfiability/ideas.md` moves to `plans/ideas/`. The rest lives in git.
2. **Every finished plan is deleted**: the folder of every closed milestone (`plans/m1e2_correctness/` and any
   other the census finds closed), of every closed phase of M1f, and every followup marked done. **What stays
   under `plans/`** is work not finished: open milestones, phases and followups, the open questions, the open
   findings with their owners, and `plans/ideas/`. M1f's own folder goes in M1f's closing commit.
3. **Nothing outside `plans/` refers to a plan or a date.** Over `docs/`, every crate's code, comments and
   tests, `stdlib/`, `examples/`, `tests/`, `corpus/`, `utils/`, `c/`, `nl2smt/`, `AGENTS.md` and `README.md`:
   - a path into `plans/`, or a milestone, phase, stage or task id (`M1x`, `P1x.y`, `S1x.y.z`, `T…`, `E-n`,
     `D-n` of a plan) → a link to the `docs/ein/` page or the decision that states the behaviour, or a plain
     statement with no id, or deleted;
   - a date that dates a development event (*"since 2026-11-03"*, *"ruled 2026-11-08"*) → deleted, the
     sentence restated as what is true now;
   - a decision id (`Q-M1f.304`) **stays**, linked to its file.
4. **Names that carry a stage id are renamed**: tests, goldens, fixtures, functions
   (`s1a_6_9_resumed_saturation`, and whatever S1f.20.1's census finds).
5. **The check, with no baseline.** S1f.20.1's release-voice check, widened by this stage to dates and paths
   into `plans/`, passes with its baseline empty and deleted. Decision ids pass it; `AGENTS.md`'s § Where the
   work is, which points at the current milestone, is its one exemption.

## The defaults, each the user's to revise at S1f.20.2's review

1. **What *finished* is**: a milestone or phase whose README says *closed*, a followup that says *done*;
   anything else stays.
2. **Measurements stay only as current state**: a re-takable command and what it gives on today's engine, on a
   named machine. A measurement of an engine that no longer exists is deleted. A number in a page carries its
   command or its test, and no longer a date (Q-M1e.4's third leg dropped).
3. **Pages superseded with a banner are deleted** or rewritten to the current state; no banner remains, and
   `AGENTS.md`'s rule *do not rewrite a superseded page* goes with the record it protected.
4. **The wrong-answer register stays** as test data (`corpus/wrong_answers.toml`, its witnesses and its test),
   its rows stripped of stage ids and dates; a `fixed` row is a regression test, not a record.
5. **Open findings (E-n) and followup rows (D-n)** are work items and stay with their owners under `plans/`;
   a closed one goes with its folder. A code comment that needs one states the behaviour instead.
6. **Commit messages are not touched**: they are git, where the history is.

## The method

### Task T1f.20.9.1 — the census of references

S1f.20.1's census, widened: every line outside `plans/` with a plan path, a plan id or a dated development
event, per file. It is the check's last baseline.

### Task T1f.20.9.2 — the references rewritten

By subject, each commit lowering the baseline. The slow ones are the sentences whose subject is the
development event; S1f.20.6's substitution 3 is the method.

### Task T1f.20.9.3 — the names renamed

Tests, goldens, fixtures and functions, with every reader (gate lists, `nightly_steps.rs`, `gate_steps.rs`,
the corpus manifest).

### Task T1f.20.9.4 — the deletions

`docs/history/`, then each finished plan folder, each in a commit of its own after the link check is green
without it. `AGENTS.md` and `README.md` rewritten to the current state, last.

## Acceptance

- `docs/history/` does not exist; no finished milestone, phase or followup has a folder under `plans/`.
- **0 references outside `plans/`** to a plan path, a plan id or a dated development event, by the census;
  the check is in the gate with no baseline.
- Every decision id cited outside `plans/` resolves (S1f.20.8's check).
- `RUSTDOCFLAGS="-D warnings" cargo doc` and `./run_tests.sh` are green.
