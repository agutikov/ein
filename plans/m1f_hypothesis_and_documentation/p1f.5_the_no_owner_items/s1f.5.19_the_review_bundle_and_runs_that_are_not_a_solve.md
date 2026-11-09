# S1f.5.19 — The review bundle reads every register row, and a command's run gets a case page of its own

**Phase:** [P1f.5](README.md) (The no-owner items)

**Status:** specified 2026-11-09 by the coordinator, who placed
[Q-M1f.52](../open_questions.md#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve) as a
stage of its own and took its (b), on the user's instruction of that day (*"just continue with default decisions till
the end of the phase"*). It runs after [S1f.5.13](s1f.5.13_large_dumps_to_the_slow_tier.md) and the merge of the second
lane, T1f.5.M (`a90c452b`), and before [S1f.5.14](s1f.5.14_nl_required.md)
([§ The order](README.md#the-order), item 10). Three choices are taken by default and recorded:
[Q-M1f.53](../open_questions.md#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read), the
gate's check; [Q-M1f.54](../open_questions.md#q-m1f54--the-2026-11-06-inventorys-evidence-in-buildreview), the
inventory's evidence in `build/review/`; and
[Q-M1f.55](../open_questions.md#q-m1f55--a-container-on-a-solves-now), a container on a solve's `now`.
**Depends on:** nothing. The rows it reads are W40's and W41's, filed by S1f.5.5's T13 under the register's schema
`/16` (`fc53818f`), W43's under `/19` (S1f.5.12), and W55's and W56's, which the merge brought in.

## The question

[Q-M1f.52](../open_questions.md#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve),
raised by S1f.5.13's T1f.5.13.3 running `./run_tests.sh --nightly` on `87d18cf6` (`build/nightly-s1f.5.13.log`). **The
nightly has been red since `fc53818f`, and the gate has not.** `python3 utils/review_bundle.py`, a step of the
nightly's `deep-corpus` job ([`nightly.yml`](../../../.github/workflows/nightly.yml); `./run_tests.sh --nightly`),
exits 1, and on the runner every step after it in that job is skipped. Two causes:

- **The index reads `[case.wrong] verdict` unguarded** (`utils/review/case.py`'s `index_page`). A refusal has no
  verdict. Four rows hold one: `W41/saturate-import` (schema `/16`), `W43/einb-dump-answer` (`/19`, a solve's usage
  refusal, exit 2), and the merge's `W55/goal-neq-free` and `W56/goal-absent`, whose `[case.wrong]` is `ein saturate`'s
  refusal at load beside a `now` that is a solve. The bundle stops with a `KeyError`.
- **A register run is a solve**, `case.py`'s `flags_of` asserts. With the index guarded (tried by S1f.5.13 and
  reverted), seven cases fail to build: W40's six, a command run on the witness saved as a `.einb`
  (`saturate --dump`, `render rules`, `render rule --name step`, `render constraints`, `render program`, `kb save
  {out}/again.einb`), and `W41/saturate-import` (`saturate`) (`build/nightly-s1f.5.13-rest.log`, 145 cases, 375 s).

**Why the gate stayed green.** Its step, `review_bundle.py --smoke`, builds one register case, `W2/forced-2`, and a
program, so its index has one row, whose `wrong` has a verdict.

**What else reads a row shape wrong, and builds.** A census of the register's 175 rows by the shapes the bundle reads
(the `wrong` block's answer and `by`, the `right` block's, and whether `now` is a solve, refused, answered or on a
container; a scratch read of `corpus/wrong_answers.toml`, 2026-11-09) finds thirteen shapes, and three more the bundle
builds with the wrong words or the wrong input:

- **A command's `wrong` beside a solve's `now`** (W55, W56): the case page names the record *the walk's*, its answer
  *—, k = —, exhausted = —*, and its stderr cell *—*, where the register records a refusal at load.
- **A region run's refusal** (W43's `wrong`, exit 2): the same, its refusal line not shown.
- **A `container` on a solve's `now`** (W43): the page runs `ein solve` on the `.ein`, and says *this build re-ran it*,
  where `wrong_answers.rs` runs the container.

## The ruling

Q-M1f.52's (b), taken by the coordinator by default on the user's instruction of 2026-11-09: **the index is guarded,
and a register case whose run is not a solve gets its own case page.** The user's standing rule for review artefacts
applies: *anything built for the user to review carries every engine artefact, raw and rendered.*

## What it builds

**The index guarded**, both the `wrong` and the `right` column, for every answer the register's schema lets each
block state: a verdict (the walk's or a region run's), an error (schema `/7`), a refusal (`/4`, `/16`, `/19`'s exit
2), and a command's `answered` (`/16`). The `now` column says what a command's run answered.

**A case page for a command's run**, `utils/review/command.py`, in the solve page's look: the same title, fixed
callout, numbered sections and checklist, and a run directory under `runs/now/` with its `run.md`, `raw/` and `img/`.
It shows:

- **the command as run**, through the corpus's run planner (`ein_corpus::plan::argv`, ported), `{out}` the run's
  scratch directory under `raw/`; with `container`, the witness saved with `ein kb save` first, that step's exit,
  stdout and stderr, and the container;
- **its exit code**, and **stdout and stderr in full**, inline and as the files under `raw/`;
- **against the register's `wrong`, `now` and `right`**, with whether the run answers `now` as
  `ein-cli/tests/wrong_answers.rs` reads it: a refusal or an error is its exit and its stderr's first line, `{FILE}`
  the path handed and `{out}` the scratch directory; `answered` is exit 0 with nothing on stderr, and with `container`
  the same run on the witness, compared as `wrong_answers.rs`' `comparable` compares them (ported), which the page
  runs and shows beside it;
- **every artefact the run writes, linked**: a `render` view's DOT drawn as SVG, the witness's beside it; a
  `saturate`'s dump, with the KB it saturated drawn (`ein render kb` on the same file); every file under `{out}`,
  linked with its size, and a `.einb` among them listed (`ein saturate F --dump`) with the program it stores drawn
  (`ein render program F`);
- **the problem**, as the solve page has it: the witness, its header and source, and the program, its rules and root
  drawn (`render program`, `render rules`, `render kb`);
- **the row's `look_at`**, resolved where the page has the artefact, and named absent with why where it does not.

**The solve page's words for every recorded answer**: a command's `wrong` is *a command's run*, a refusal is shown as
one with its line and exit, and the case page runs a `container` solve on the container
([Q-M1f.55](../open_questions.md#q-m1f55--a-container-on-a-solves-now)).

**The gate catches the class.** `--smoke` builds a command case beside its solve case and its program, and reads every
register row with no run: the index's cells, the case table, the fixed callout and the checklist each page writes from
the register alone, so a row shape the bundle cannot read fails per commit
([Q-M1f.53](../open_questions.md#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read)).
The smoke's time is taken before and after. If `per-commit.yml` changes, `run_tests.sh` changes the same way, which
`ein-cli/tests/gate_steps.rs` holds.

**The bundle regenerated.** `build/review/` is gitignored. The four dead links the nightly's link check finds there are
in pages the 2026-11-06 inventory wrote for F1's files, which `ccd1007f` deleted. That evidence moves aside intact, and
the bundle is rebuilt from nothing
([Q-M1f.54](../open_questions.md#q-m1f54--the-2026-11-06-inventorys-evidence-in-buildreview)).

**What does not move.** No engine code, answer, golden or register row: the register reads 161 fixed, 13 open and 1
not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-09).

## Defaults

Each taken by default on the user's instruction of 2026-11-09 and recorded as a defaulted row of
[`open_questions.md`](../open_questions.md):

- [Q-M1f.53](../open_questions.md#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read),
  **(b)**: the smoke builds a command case, and reads every row of the register with no run.
- [Q-M1f.54](../open_questions.md#q-m1f54--the-2026-11-06-inventorys-evidence-in-buildreview), **(b)**: the
  inventory's evidence moves to `build/inventory-2026-11-06/`, intact and at the same depth, and `build/review/` is
  cleared and rebuilt.
- [Q-M1f.55](../open_questions.md#q-m1f55--a-container-on-a-solves-now), **(a)**: a `container` solve runs on the
  container, as `wrong_answers.rs` runs it.

## Tasks

### Task T1f.5.19.1 — the stage file and its placement

This file; its row in [the phase README](README.md#stages) and its place in [§ The order](README.md#the-order);
Q-M1f.52 restated, and Q-M1f.53 to Q-M1f.55 recorded. Docs only.

### Task T1f.5.19.2 — the fix

The index guarded, the command case page, the solve page's words, the container solve, and the smoke's two
additions, with its time before and after. Then the whole bundle, `python3 utils/review_bundle.py`, every case built
and exit 0, and the nightly's link check on `build/review` (`python3 utils/doc_audit.py --links --check
build/review`), after the inventory's evidence moves aside and the directory is rebuilt. The gate green before the
commit.

### Task T1f.5.19.3 — the merged tree's first nightly, and the records

`./run_tests.sh --nightly` once on T2's commit, the first nightly on the merged tree: each job and step, pass or fail,
with its time. A failure that is not this stage's is recorded as a finding with its log lines and not fixed here; a
wrong answer would be a register row first. Then this file's status and records with the acceptance re-taken, the
phase README's row, a sentence in AGENTS.md's P1f.5 account, `utils/README.md`'s line for `review_bundle.py`, and
Q-M1f.52 marked built.

## Acceptance

- `python3 utils/review_bundle.py` builds every register row's case page and exits 0, and the nightly's link check on
  `build/review` finds nothing.
- A command's run has a case page that shows the command as run, its exit, its stdout and stderr in full (inline and
  as files), the register's `wrong`, `now` and `right` beside it with whether the run answers `now` as
  `wrong_answers.rs` reads it, and every artefact the run writes, linked and, where the engine draws one, drawn.
- The index reads every answer each of its two register columns can state.
- The per-commit gate builds a command case and reads every register row through the index and the case pages'
  register-only parts, and its time is taken before and after.
- `./run_tests.sh` is green, and `./run_tests.sh --nightly` is run once on the fix, each job and step reported.
- No answer, golden or register row moves.
