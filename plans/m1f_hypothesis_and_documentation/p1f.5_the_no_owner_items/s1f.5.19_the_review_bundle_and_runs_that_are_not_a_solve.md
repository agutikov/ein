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
**Done 2026-11-09**, in three commits: T1f.5.19.1, this file and its placement (`919380c2`); T1f.5.19.2, the fix
(`00c4ed34`, [its record](#t2-record-2026-11-09)); and T1f.5.19.3, the merged tree's first nightly, green in every job
and step, and the records ([its record](#t3-record-2026-11-09)). The acceptance is re-taken
[below](#acceptance-re-taken-2026-11-09).
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

### T2 record, 2026-11-09

**The index guarded.** `case.py`'s index row is `index_row`, its two register columns `answer_cell`, which writes
whatever a block states: a verdict and k, an error, a refusal (`refused`, with its exit where it is not 1), a
command's `answered`, and who gave it (*region run*, *command*) and on what (*container*); `differs_cell` names a
refusal or an `answered` the right states and the wrong does not; and `now_cell` says what a command's run answered.
Every recorded answer reads the same way on the case pages: `answer_words` and the stderr cell show a refusal's line
and exit (W41, W43, W55, W56), `recorded_by` and the § 3 head name *a command's run* (`by = command`), and the fixed
callout and § 1's `now` cell say *on the container*.

**The command case page**, [`utils/review/command.py`](../../../utils/review/command.py). `case_page` hands it a row
whose `[case.now]` is a command's run (`is_command`, `ein_corpus::register::Now::is_command`'s test), with the title,
the fixed callout, the § 1 table (`case_rows`) and the checklist the solve page writes, so both read the same way.
It runs the row through the corpus's run planner (`plan_argv`, `ein_corpus::plan::argv` ported), `{out}` the run's
`raw/out/`; with `container`, `ein kb save` first into `raw/container/`, and the same run on the witness into
`raw/witness-out/`, compared through `comparable` (`wrong_answers.rs`' function, ported). Whether the run answers
`[case.now]` is `differences`, `wrong_answers.rs`' reading of a command's row: a refusal or an error is its exit and
stderr's first line, `{FILE}` the path handed and `{out}` the scratch directory; `answered` is exit 0, nothing on
stderr, and with a container the witness's stdout. The page has the solve page's eight sections: § 2 the witness
drawn (`render program`, `render rules`, `render kb`) and the container step; § 3 the run against `wrong`, `now` and
`right`; § 4 every invocation's exit, stdout and stderr in full, inline and linked; § 5 what the run wrote, a view's
DOT drawn beside the witness's, a `saturate`'s KB drawn (`render kb` on the file it read), and each file under
`{out}` linked with its size, a `.einb` listed (`ein saturate F --dump`) and its stored program drawn (`render
program F`); § 6 `look_at`, `root` resolved to § 2's drawing; § 7 every raw file; § 8 the checklist. `runs/now/run.md`
is the run's page, as a solve's is.

**A container solve runs on the container** ([Q-M1f.55](../open_questions.md#q-m1f55--a-container-on-a-solves-now)):
`W43/einb-dump-answer`'s case page saves the witness and hands `build_run` the `.einb`, every invocation of which
exits 0 on it (main, the two forms, `--diff-models`, the oracle, five `render` views, the region's three and the three
state drawings), 10 of 10 cross-checks passing; `RunSpec.source` gives the program page the witness's text, which it
says is the program the container stores.

**Found on the way: every rule view the bundle drew was refused.** `util.split_digraphs` ended a digraph at the first
line that read `}`, which in a rule view is a `subgraph cluster_…`'s close, so every `render rules` and `render
program` rule graph was cut there and Graphviz refused it: 654 refused graphs over the 145 cases of the bundle S1f.5.13
built at `87d18cf6` (each case's `pages/consistency.md`, § Graphs Graphviz refused), four of them the gate smoke's own
`W2/forced-2`, since S1e2.1.5 (`4778af1f`). It now ends a digraph where its braces balance outside quoted strings and
comments; over the 31 graphs `render program`, `render rules` and `render constraints` print for `zebra.ein`, `features/01_not_and_absent`, `branching/01_saturate_only` and W40's witness, every one splits whole and lays out, and the whole bundle
below refuses none. A review artefact's defect, not an answer: no register row.

**The gate catches the class** ([Q-M1f.53](../open_questions.md#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read)).
`--smoke` first reads every register row with no run, `case.register_pass`: the index's row, the title, the fixed
callout, the § 1 table, the checklist and the run's plan, each failure named by row. It then builds `W40/kb-save`
(a container, and the container its run writes) and `W41/saturate-import` (a refused `wrong`) beside `W2/forced-2` and
the refused program, and holds every DOT it kept to its SVG beside the SVG and link checks it had. Mutation-checked
(2026-11-09, each a scratch patch in the Python process, reverted): with the index's old `wrong` cell the pass fails
on `W41/saturate-import`, `W43/einb-dump-answer`, `W55/goal-neq-free` and `W56/goal-absent`; with every run taken for
a solve, as before, on W40's six and W41; with the old `split_digraphs`, the smoke exits 1. `per-commit.yml` and
`run_tests.sh` run the same command as before, so neither changed and `gate_steps.rs` has nothing new to hold.

**The smoke's time** (`python3 utils/review_bundle.py --smoke`, the gate's debug `ein`, three runs each, 2026-11-09,
`build/s1f.5.19/smoke_{before,after}_{1,2,3}.log`): **0.87, 0.81 and 0.82 s before; 1.36, 1.33 and 1.33 s after**.
The register pass reads 175 rows in it.

**The whole bundle** (`python3 utils/review_bundle.py`, release, 2026-11-09, `build/s1f.5.19/bundle_full.log`): **175
cases, 0 failed, exit 0, 395.8 s**: 168 solve pages and 7 command pages (W40's six, W41), no case's now differing from
the register, and 0 refused graphs over the 168 solve cases' `consistency.md`; 6 739 pages, 3 948 SVGs, 3.1 GB
(`find build/review -name '*.md' | wc -l`, `-name '*.svg'`, `du -sh build/review`). The nightly's link check after it
(`python3 utils/doc_audit.py --links --check build/review`): **6 739 pages, 0 findings**, exit 0, 10 s.

*Read later (S1f.5.29, 2026-11-10): the counts above and the smoke's were taken here, on Graphviz 16.0.0, and are local.
On the runner's Ubuntu 26.04 Graphviz, which has no `fdp` layout, the smoke refused every KB view it drew, so the
per-commit job was red from `00c4ed34` on, and the whole bundle, re-taken in a runner-like image at `28da7eb5`, refused
all 612 of its KB views and trace blocks and exited 0 ([S1f.5.29 § T1
record](s1f.5.29_the_review_bundle_smoke_on_the_runner.md#t1-record-2026-11-10)).*

**`build/review/` rebuilt** ([Q-M1f.54](../open_questions.md#q-m1f54--the-2026-11-06-inventorys-evidence-in-buildreview)):
it is gitignored (`git check-ignore -v build/review`: `.gitignore:6:build/`). The 2026-11-06 inventory's evidence,
`build_open_items.py`, `gen_no_owner.py`, `items/`, `commands/`, `programs/` and `.state/`, moved intact to
`build/inventory-2026-11-06/`; the rest, the bundle's own pages, was cleared and the bundle rebuilt from nothing. The
four dead links S1f.5.13 found are F1's, in the moved pages, which the nightly's link check no longer reads.

**The gate**, on this code: green, 15 min 49 s (`build/gate-s1f.5.19-T2.log`), its smoke step reading 175 rows and building three cases and the program, 37 SVGs, 58 pages linked. Committed as `00c4ed34`.

### Task T1f.5.19.3 — the merged tree's first nightly, and the records

`./run_tests.sh --nightly` once on T2's commit, the first nightly on the merged tree: each job and step, pass or fail,
with its time. A failure that is not this stage's is recorded as a finding with its log lines and not fixed here; a
wrong answer would be a register row first. Then this file's status and records with the acceptance re-taken, the
phase README's row, a sentence in AGENTS.md's P1f.5 account, `utils/README.md`'s line for `review_bundle.py`, and
Q-M1f.52 marked built.

### T3 record, 2026-11-09

**The merged tree's first nightly is green, every job and every step.** `./run_tests.sh --nightly` on `00c4ed34`, the
tree clean, from 2026-11-09 22:40 to 2026-11-10 00:02 (+04), exit 0 in **4 914 s, 81 min 54 s**
(`build/nightly-s1f.5.19.log`; each step's start stamped in `build/s1f.5.19/nightly_steps.txt`). A cargo step's time is
its `test result` line's, a script's the gap between its banner and the next:

| job | step | result | time |
|---|---|---|---:|
| deep-corpus | `corpus_cli`, the slow entries | 6 passed | 97.9 s |
| deep-corpus | `id_order_invariance`, eight seeds | 2 passed | 481.3 s |
| deep-corpus | `answer_dump` | 8 passed | 39.0 s |
| deep-corpus | `answer_round_trip`, the slow and `heavy_dump` entries | 9 passed | 148.6 s |
| deep-corpus | `lever_invariance`, the slow cells | 4 passed | 16.0 s |
| deep-corpus | `corpus_cores`, the slow entries | 3 passed | 203.7 s |
| deep-corpus | `utils/check_test_scratch.py` | nothing left | < 1 s |
| deep-corpus | `utils/stdlib_census.py --check` | 77 of 77 rules activated, 0 never | 429 s |
| deep-corpus | `utils/review_bundle.py` | **175 cases, 0 failed** | 356.2 s |
| deep-corpus | `utils/doc_audit.py --links --check build/review` | **6 739 pages, 0 findings** | 10 s |
| deep-corpus | `utils/stdlib_mutants.py --check` | 167 of 217 killed, the banked set | 42 s |
| frontend-fuzz | seed 1 | 6 passed | 1 532.7 s |
| frontend-fuzz | seed 987654321 | 6 passed | 1 538.5 s |
| benches | `cargo bench --bench engine -- --test` | every bench `Success` | 5 s |

`deep-corpus` took 831 s, the fuzz job 3 071 s and the bench smoke 5 s. No step failed, so there is no finding to
record and no register row to file. The bundle step is the one S1f.5.13 found red; the fuzz job's cost is Q-M1f.8's and
the phase close's, as S1f.5.13 left it.

**The pages.** [`utils/README.md`](../../../utils/README.md)'s line for `review_bundle.py` says what `--smoke` reads and
builds, what a command's case page shows, and the whole bundle's size today; P1f.5's README has the stage's row, §
The order's sentence and, where it names the inventory's evidence, its new place; AGENTS.md's P1f.5 account has a
sentence after the merge's; and
[Q-M1f.52](../open_questions.md#q-m1f52--the-nightlys-review-bundle-and-the-registers-runs-that-are-not-a-solve) is
marked built.

**Found outside the brief:**
- **Every rule view the bundle drew was refused by Graphviz**, since S1e2.1.5 (`4778af1f`): 654 graphs over the 145
  cases of S1f.5.13's bundle, four of them the gate smoke's own case. Fixed here, since a command case's view would have
  been one ([§ T2 record](#t2-record-2026-11-09)); the smoke now holds every DOT it keeps to its SVG.
- **`ein kb save` of a container writes 40 bytes more than `ein kb save` of the `.ein`** (`W40/kb-save`'s page: 1 288
  bytes against 1 248). The two containers list the same KB (`ein saturate F --dump`, the lines that say what the run
  read aside) and store a program `ein render program` draws the same, so no answer differs and the register's row
  holds what it claims, exit 0 and the witness's stdout. Recorded, not filed: a container is a private cache format,
  never an interchange one.

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

### Acceptance, re-taken 2026-11-09

On the stage's code commit, `00c4ed34`, item by item:

- **`python3 utils/review_bundle.py` builds every register row's case page and exits 0, and the nightly's link check
  on `build/review` finds nothing.** 175 cases, 0 failed, exit 0, by hand (395.8 s) and as the nightly's step
  (356.2 s); `doc_audit.py --links --check build/review`: 6 739 pages, 0 findings, both times
  ([§ T2 record](#t2-record-2026-11-09), [§ T3 record](#t3-record-2026-11-09)). **Met.**
- **A command's run has a case page that shows the command as run, its exit, its stdout and stderr in full (inline and
  as files), the register's `wrong`, `now` and `right` beside it with whether the run answers `now` as
  `wrong_answers.rs` reads it, and every artefact the run writes, linked and, where the engine draws one, drawn.** The
  seven command pages (W40's six, W41): § 3 the run against the three blocks and `differences`, every one
  *matches*; § 4 every invocation's stdout and stderr inline, each linked under `raw/`, and with a container the
  witness's run and the comparison; § 5 each `render` view drawn beside the witness's, `saturate`'s KB drawn, and
  `kb save`'s two containers linked, listed and their programs drawn. **Met.**
- **The index reads every answer each of its two register columns can state.** `answer_cell` writes a verdict, an
  error, a refusal and `answered`, and `register_pass` reads all 175 rows through it, the 13 shapes of the census among
  them; with the old cell it fails W41, W43, W55 and W56. **Met.**
- **The per-commit gate builds a command case and reads every register row through the index and the case pages'
  register-only parts, and its time is taken before and after.** `--smoke` reads 175 rows, builds `W40/kb-save` and
  `W41/saturate-import` beside `W2/forced-2` and the refused program, and holds every DOT to its SVG; 0.81 to 0.87 s
  before, 1.33 to 1.36 s after (three runs each, debug). **Met.**
- **`./run_tests.sh` is green, and `./run_tests.sh --nightly` is run once on the fix, each job and step reported.**
  The gate, 15 min 49 s (`build/gate-s1f.5.19-T2.log`); the nightly, every step green, 81 min 54 s, each step's result
  and time in [§ T3 record](#t3-record-2026-11-09). **Met.**
- **No answer, golden or register row moves.** The stage changed `utils/` alone: the register reads 161 fixed, 13 open
  and 1 not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-10), as it did before. **Met.**
