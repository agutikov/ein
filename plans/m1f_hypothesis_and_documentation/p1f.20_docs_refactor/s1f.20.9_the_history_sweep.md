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

**Ruled 2026-11-17** ([P1f.20 § The user's rulings at S1f.20.2's review](README.md#the-users-rulings-at-s1f202s-review-2026-11-17)):
all six as written, and three more for this stage. **The dependency runs one way**: nothing in the project
reads, links or names `plans/`. **The register splits**: an `open` row and its witness move to `plans/`, with
whatever re-runs them, and the register's filing rules with them; a `fixed` row stays as a regression test,
stripped of ids and dates, and no project test holds an open row (default 4 as the rulings read it; the
coordinator's reading, the user's to correct). **The walk's seven instruments** in `utils/` are deleted, with
what only they read. An open question the project cites is written as a decision by S1f.20.8, so none is cited
here.

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

**Before rewriting a `.ein` comment** ([S1f.20.8's gate](s1f.20.8_decisions.md#the-gate-on-the-stages-batch-2026-11-17)): keep the file's line count, since register rows and goldens record positions after it, and expect `ein-ir/tests/golden/fuzz_seeds.txt`, which quotes every fuzz seed's text, to need a bless checked to be comment-only.


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

## Record: part C1 (2026-11-17)

Run in a worktree beside part C2 (the register's case ids in project files, the release-voice check widened), on
parts A and B merged (`f3e540b2`). No file of C2's lane was edited: no project comment for an id, not
`utils/check_release_voice.py`. It does T1f.20.9.4, the deletions, and brings the process files to the tree they
leave. Each deletion is a commit of its own, made after `doc_audit.py --links --check --tracked` read 0 and 0
without it.

**The order.** The walk's instruments and the decision census went first, so their links into `docs/history/` went
with them; then every reference into `docs/history/` and the tree; then what the finished plans still owned or were
read by, every reference into them, and the folders one by one; then the process's indexes and `AGENTS.md`.

**What was deleted**: 483 files.

| what | files | why it is finished, or the ruling |
|---|---:|---|
| `docs/history/` | 63 | the user's rulings of 2026-11-15; 64 were tracked when the part began (the stages before it had rewritten the rest of the table's 77), less `ideas.md`, which moved |
| `plans/m1e2_correctness/` | 219 | README: *Closed 2026-11-08* |
| `plans/m1j_nlp_smt_poc/` | 1 | README: *done 2026-11-13* |
| `p1f.5_the_no_owner_items/` | 105 | README: *Closed on 2026-11-11* (its parser probe's `.rs` and `.ein` files with it) |
| `p1f.6_calculator/` and `p1f.8_school_algebra_and_geometry/` | 45 + 1 | README: *Closed 2026-11-15*; P1f.8 a redirect stub kept for P1f.6's replies |
| `p1f.7_findings_outside_scope/` | 13 | README: *closed 2026-11-11* |
| `p1f.10_hypothesis_structure/` | 24 | README: *Closed 2026-11-16* |
| `plans/followups/f9_e_catalog.md`, `f18_world_aware_negatives.md` | 1 + 1 | *closed 2026-08-17*; *closed without being done, 2026-11-04* |
| the walk's seven instruments in `utils/` | 7 | the user's ruling, item 4: `fork_split.py`, `e2e_baseline.py`, `feature_matrix.py`, `layer_census.py`, `openness_census.py`, `model_set_census.py`, `closure_census.py`; with `feature_matrix_results.json`'s `.gitignore` line, their `utils/README.md` rows and paragraph |
| `utils/decision_census.py`, `.txt` | 2 | the default: its ledgers are deleted |
| `utils/structure_bundle.py` | 1 | not in the task's list: P1f.10's review bundle, which renders that phase's pages and cannot run without them |

**`ideas.md`** is [`plans/ideas/11-saturation-and-satisfiability.md`](../../ideas/11-saturation-and-satisfiability.md)
(`git mv`, so the history follows it), verbatim under a title and a two-line note saying whose it is, indexed as idea
11 in `plans/ideas/README.md` with what shipped of it; its 15 links followed it.

**The census**, its last run, on `f3e540b2` before anything was deleted:

```text
$ python3 utils/decision_census.py --check
620 ids; 0 gaps
351 carried decisions, 351 files under docs/decisions/; 0 mismatches
```

Then retired with its hand half (L-17 closed). No gate step, workflow or `AGENTS.md` line ran it; F27 records the one
pending id it carried (Q-M1f.337); `doc_audit.py`'s decision-id check, the gate's, stays.

**References, re-aimed or text.** By a script over every tracked `.md` (each markdown link that resolves into a deleted
path: from a page that stays, re-aimed where a successor states it, else its label text; from a finished folder, its
label text), and by hand in the non-Markdown files.

| into | from what stays | re-aimed | label text | from a finished folder | by hand |
|---|---:|---:|---:|---:|---|
| `docs/history/` | 185 | 52: a design contract to its `docs/ein/ein.rs/design/` page, `the_boundary.md` and `completeness.md` to `satisfiability.md`, a ledger anchor to its decision file | 133 | 131 | three workflows' headers (*Tier N of three* where they cited design/12 § 4, which the new page does not have), `release.yml`'s header and musl paragraph, `deny.toml` (re-aimed to § 2), `rust-toolchain.toml`, ten `utils/` scripts' docstrings and comments (`nl_census.py` and `check_hashmap_iteration.py` re-aimed, `doc_audit.py`'s a dangling reference definition), `AGENTS.md`'s bullet |
| the finished plan folders | 1 142 | 17, each an M1e2 ledger anchor to its decision file | 1 125 | their links into one another | `nightly.yml`'s header, `nl_census.py`, `region_census.py` (re-aimed to Q-M1e2.7's file), the review driver (below) |
| the seven instruments | 3 | 0 | 3 (M10's README and S10.3, M2's S2.5.3 say the script went with the walk) | 8 | `bench_env.sh`, `corpus_cost.py`, `region_census.py`, `render_examples.sh`, `zebra2_trace.sh`, and `ein-infer/tests/obligation_reports.rs`' doc comment (a project file that named `layer_census`; its line count kept) |
| the census, `structure_bundle.py` | 4 | 0 | 4 (the S1f.20.8 records say the census is retired) | 1 (`structure_bundle.py`, from P1f.10) | F27's sentence |

A link whose label would now mislead was relabelled (F20, F26, `docs/lib/03`: `satisfiability.md`). The finished
folders' targets that have a successor page in `docs/ein/` (M1e2's `read_out.md`, `site_census.md`, `solve_interface.md`)
are text, not re-aimed: the citing sentences name their sections, which the new pages do not have.

**What still read or owned something in a finished folder moved first:**

- **S1e2.4.1's fourteen probes** are [`plans/followups/f21_probes/`](../../followups/f21_probes/): the hole, node and taint
  censuses run them in their re-take commands, and F21 cites `via_hole.ein`. Each probe's re-take comment and the
  censuses' commands and recorded output follow the move; F21 and `plans/followups/README.md` say what the folder is.
- **The review bundle** names no M1e2 page: its index says it is built from the open register, a trace page's
  definitions cite `rendering.md` § The trace, the cross-checks page names no stage, and an open row's owner links to
  its followup's page (the lookup searched M1e2's stage files, where no owner of an open row is).
- **P1f.10's findings the user handed on** (its close, answer 3, *"Accept as proposed"*): H6-1, H6-3, H6-4 and K-41 are
  a section of F21, verbatim, which they had not been; K-21 (*with Q-M1f.311*) and K-13's stdlib half (*a stdlib change
  of its own, later*, no owner) are rows beside Q-M1f.311 in M1f's ledger. The K rows the close did not hand to an owner
  went with the folder. M1e2's ledger had no open row; P1f.6's findings and P1f.7's open part were already with their
  owners (M1g, M1h, F21 to F24, F27).

**The process files, current.** `AGENTS.md`, 51 999 characters to 51 062: § Where the work is points at P1f.20 alone,
every stage done, stopping for the user's review of the new docs and not closed before it, and states that a finished
plan is deleted; § Where things live names idea 11, the `utils/` catalogue's groups and its censuses; the standing
rules cite Q-M1f.313's decision file and drop E-32, E-33 and the P1f.20 sentence; § Running the gate keeps every
command without its stage annotations, and its prose without its stage and date asides. `utils/README.md`: 42 scripts,
sixteen not driving the engine, the measurement set six scripts and four Rust examples (K-3), one sentence where the
walk's instruments' paragraph was. `plans/README.md`, `plans/followups/README.md` and M1f's README say which plans and
records are deleted and in git; `plans/open_questions.md` and M1f's ledger keep their text with the links made text or
re-aimed. The root `README.md` names nothing deleted.

**Checks**, on the last commit: `doc_audit.py --links --check --tracked` (0 and 0), `check_release_voice.py` (0 lines),
`check_gate_steps.py` (16, 75, 79 steps), `check_tool_mirrors.py`, `review_bundle.py --smoke` with the gate binary of
the main checkout as `EIN_BIN` (15 rows read, 2 cases and 1 program built, 25 SVGs, the link check 0),
`history_census.py` (the crates' links into `docs/history/` and into `plans/` both 0), and
`git grep -n 'docs/history' -- ':!plans' ':!docs/history' ':!utils' ':!AGENTS.md' ':!run_tests.sh' ':!.github'`, and the
same grep for every deleted path, print nothing. No `./run_tests.sh`; no crate test, since nothing the project runs
changed (one Rust doc comment, two TOML comments).

**Commits**: `8d7485a5` (`ideas.md` moved), `1626c688` (the instruments), `096b2cfd` (the census), `322f7a71` (references
into `docs/history/`), `cf78590e` (`docs/history/` deleted), `8d9c0bbe` (`structure_bundle.py`), `e86c56f4` (the probes),
`0cc689d9` (the review bundle), `75e91ff2` (references into the finished folders, the findings handed on), `1eff75e2`,
`5d5101a7`, `653a123e`, `abe38379`, `3cd904eb`, `2a03a13b`, `be5e106d`, `7e88d2a7` (the eight deletions), `cc9cea82`
(the process's indexes), `0e96cb09` (`AGENTS.md`), and this record.

**Decisions the user may want to revisit**: `utils/structure_bundle.py` retired with P1f.10, which the task did not
list; the probes' home, a folder beside F21's page rather than the page made a directory; the census retired rather than
re-pointed; `release.yml`'s release notes no longer quote a figure for the musl build (it was the walk's, on
`solve -e`), and the workflows' headers cite no page; `docs/lib/03` touched for one link, though the phase's acceptance
says `docs/lib/` is untouched, because it would dangle otherwise; `rust-toolchain.toml` keeps *MSRV = stable-2* with no
page behind it, since `toolchain-and-layout.md` states none.

**Found outside the part**:

- `plans/wrong_answers/register.toml:382`, a row's note, names `plans/m1e2_correctness/probes/alt-cap-shared.ein` as
  text; the register keeps its notes whole, so it stays.
- `utils/history_census.py` keeps a `docs/history` row, now 0, and `doc_audit.py --inbound` a `hist` column; both count
  a category, not a page.
- `history_census.py` finds 7 lines carrying an id in 4 crate files (part B's prototype spellings among them) and 34 in
  `utils/README.md`, 14 in `AGENTS.md`; the latter two are process and may.
- `1eff75e2`'s message says no file outside `plans/` names M1e2's folder; `AGENTS.md` still did, as text, until
  `0e96cb09`.

## Record: part B (2026-11-17)

Run in a worktree beside part A (the register's split and the project tests that read the process), on the
findings [L-12, L-16, L-27, L-32, L-37, L-38, L-40, L-46](findings.md) and L-42's one trivial part. No file of part
A's was edited: `corpus/wrong_answers.toml`, `corpus/corpus.toml`, `run_tests.sh`, the workflows, `gate_steps.rs`,
`nightly_steps.rs`, `natural_language_page.rs`, `help_surface.rs`, `region_dump.rs`, `wrong_answers.rs`, the
generators' `--check` tests and `ein-corpus/src/register.rs`.

**What the binary prints carries no development history** (L-12, L-27, L-37, L-16). Every string literal under
`ein.rs/crates/*/src` was read by a script that blanks comments and keeps strings; what it found, and what each
says now:

- the help: `--no-require-why` (`(M1f T1f.5.14.7)`), `render` (`(S1.6.1)`), `render lattice` and `oracle`
  (`(M1e2)`), `test` (`(M1c)`), `kb` (`(P1a.8)`): the tag dropped, the sentence kept;
- `-L`/`-K`'s refusals: *is retired (M1e2 S1e2.1c.5):* → *is retired:*;
- `ein_core::config::RETIRED` loses its stage column, `(flag, why)`; the loader's note is *config flag :F is
  retired: WHY*, and the two reasons with a decision cite it, `enable-symmetric-mirror` (Q-M1e2.30) and
  `warn-derived-naf` (Q-M1e2.5);
- the lattice keyword's note on a `(query …)` drops *since M1e2 S1e2.1a.4*; *forbid*'s refusal says
  *(dead is not upward-closed under absent; Q-M1e2.5)* for *…; forbidden since M1e2 S1e2.2.5)*;
- `naf_deps`' two warnings: *S1.21.8). See S1.7.4.* gone, *Q-M1e.9* → Q-M1e2.5, its carrier, and the one row of
  `utils/doc_audit.py`'s `DECISION_IDS_ALLOWED` removed (the check reads 0 let stand, 0 findings);
- the flagged program's `assumes` row, the oracle dump's `assumes.open` and region page, `ein test`'s warning line and
  the region drawing's legend: *open: Q-M1e2.2, F21* / *open (F21)* → *not decided: Q-M1e2.2* / *not decided
  (Q-M1e2.2)*, Q-M1e2.2 being the decision that leaves it undecided;
- `EinbError::Int`: *which Ein has not had since M1f S1f.5.4* → *which Ein does not have (a run of digits is a name,
  Q-M1e2.24)*; `$EIN_STDLIB`'s refusal names no script: *…or name a directory that holds the stdlib and its
  MANIFEST.sha256*;
- a state file's header: *— read_out.md § 8.2.* → *(answer-dump.md, every state file loads).*;
- `ein-corpus`' manifest tests: *(TE-L1). Use `-E`* → *Use `--region-budget FACTS`* (the `-E` it advised is
  refused), the slow-set and cost messages name no `utils/` command and no deleted page;
- an assertion's *(M1f S1f.5.23)* in `from_ir`, *M1's two predicates* in `predicates`, and the examples'
  printed lines (`detector_draft`, `hole_census`, `site_census`).

What quotes them moved in the same commit, each checked before a bless: `golden/help_shape.txt` (18 lines, the five
tags), `examples/broken/config_retired_flag.expected` and `query_lattice_keyword.expected` (one note each, the test's
own diff), `ein-ir`'s `config_coercion.txt`, `grammar_table.txt` and `fuzz_seeds.txt` (35 lines, every one a retired
note or the lattice note, read pair by pair after normalising the old wording), `config_reference.rs`'
retired-flag assertion, `grammar_pages.rs`' two deleted-line sentences, `region_modes.rs`' `assumes`, `invalidation.rs`'
`Int`, `read-out-examples.md` (re-written by its script from the gate binary: the 11 `assumes` lines and nothing
else), `region-dump.md`'s `assumes.open` row, Q-M1e.10's quoted note, and the four `monotonicity_*` drawings of
flagged programs (three re-drawn by `monotonicity.md` § 7's command, `three_components`, which loads no more, by its
`.dot` and `dot -Tsvg`; one `<text>` line each, Graphviz 16.0.0 as before). **`corpus_shapes.md5`**: an unblessed
`EIN_CORPUS_SLOW=1` run moved 542 of 28 324 lines (95 programs: 245 read-outs, 100 cores, 57 `naf`, 49 oracle, 49
region-trace, 20 region-lattice, 20 states, 2 ir); a scratch hook wrote every rendering out, and for each moved line
the rendering with this stage's string changes undone has exactly the golden's old digest, 542 of 542, 0 unexplained.
Then the hook went and the golden was blessed.

**Not moved, part A's** (`corpus/wrong_answers.toml`): 16 `refused` lines quote *forbid*'s text, and `wrong_answers.rs`
fails on exactly those 16 runs until they read `Q-M1e2.5` for `forbidden since M1e2 S1e2.2.5` (each run's recorded
line, so substituted, is its line today). Five `qualifier` lines still say *open (F21)*, compared by nothing.

**Test messages and names** (L-37, L-38). Messages: `cli_semantics.rs`' event cover and *B1 ontology*,
`region_modes.rs`' unread-programs note (now Q-M1f.121), `roundtrip.rs`' size message, `guide_programs.rs`,
`kb_semantics.rs`, `world_anchors.rs` (*TE-L2*), `corpus_cli.rs`' cost message, `config_reference.rs`'
`--require-why` (now Q-M1f.62). Renamed, with every reader:

| was | is | readers |
|---|---|---|
| `the_lexer_gives_the_tokens_t4s_did`, `as_t4_wrote_it` | `the_lexer_gives_the_tokens_its_plain_form_does`, `plain_form` | `ein.l` (two comments), Q-M1f.8, `ir-frontend.md`, the file's own table |
| `the_stdlib_duals_resolve_before_the_stage_that_ships_them` | `the_stdlib_obligations_shapes_resolve_their_projection` | none |
| `w36_is_exempt_from_refuted_by_until_f22` | `w36_is_exempt_from_refuted_by_while_its_row_is_open` | the module doc; the `("open", "F22")` owner check is the status alone, its message the exemption's reason |
| `a_saturated_zebra2_is_small_and_opens_cold_in_under_a_millisecond` | `a_saturated_zebra2_is_under_80_kb_and_opens_cold_under_6_ms_or_20_with_assertions` | `ein-einb/src/lib.rs`, `binary-format.md` |
| `t9s_probe_is_refused_at_its_three_writeback_names` (found by the scan) | `a_probe_is_refused_at_its_three_writeback_names` | none |

No gate or nightly list names any of them. The lexer test no longer walks `plans/`' `.ein` files (L-36's
`generated_parser.rs:592`): 901 files under `examples/`, `stdlib/`, `tests/` and `corpus/` remain, against its floor of
500.

**L-32.** `config.md` § 4 lists what the binary reads and nothing else; its *Read by the test harness* class moved
to `AGENTS.md` § Running the gate as the harness's census, with a command that re-takes the set, and it gained four
names the page had said nothing reads: `EIN_ROUND_TRIP_LIST`, `EIN_REGION_PRINT`, `EIN_REGION_CENSUS`,
`EIN_ROUTE1_TIMES` (tests read them). `config_reference.rs` holds the shipped class only:
`the_dev_crates_environment_is_in_the_harness_class` is deleted, its scanner folded into the shipped one. Nothing
in the gate now holds the harness census to the crates; the command beside it re-takes it.

**L-40, stale comments with no id**, every `.ein` line count kept, the stdlib manifest re-written:

- `:priority` restated as the agenda's order (Q-M1f.313) and the boundary: `std.algebra` (`difference`,
  `functional`/`injective`, `bijective-properties`, the obligations' report), `std.bijection` (`functional-negative`,
  the typechecks), `std.slots` (the *priority bands* section, now *the order rules fire in*, and the obligations),
  and the probes of `tests/stdlib/algebra/07`, `18` and `slots/13`; `std.algebra`'s nonexistent
  `absents_still_pass` with them, and `std.macro`'s *loads under `--require-why`*, a retired flag;
- `branching/12`: only `(__closed__ R)` closes a relation (`hypgen::closed_relations`), and nothing auto-closes;
  its header's numbers checked (51 candidates, 47 dead alone, R three states);
- `branching/02`'s title: R is 20 states (`ein solve`: 20 states, maximal line 4), and the anchor kills the
  wrong-place guess, which root plus `(co-located Blue H1)` shows dead;
- `features/09`: H is 82 candidates with the typing or without it (`ein show hyps`), not 99, since the blind
  generator is untyped (Q-M1e.12): the typing kills the type-wrong ones, it does not keep them out; `(colour-loc Fox
  T)` is no candidate, `(colour-loc Fox House-1)` is;
- `features/03`: the compiler does not refuse a `forall` whose bound variable is outside its guard: the probe loads
  and saturates, the variable reading *some* under the inner `absent`.

No fuzz seed's text moved (`fuzz_seeds.txt` moved by the notes alone).

**L-42**, its trivial part: `corpus_cli.rs`' `budgeted` no longer matches the retired `-E`/`--max-enterings`, so a
refused run is not taken for a budget; no corpus run names either. The `checked >= 7` floor and `primitive_arity`'s
`false` stay the user's.

**L-46**, three decisions: [Q-M1f.351](../../../docs/decisions/language/Q-M1f.351-a-bare-variable-or-name-is-no-whole-conclusion.md)
(a bare variable or name is no whole conclusion, the grammar's `Assert`; `cli_semantics.rs` gains the `:assert ?x`
case so the parse half is held),
[Q-M1f.352](../../../docs/decisions/stdlib/math/int/Q-M1f.352-an-integer-is-two-parts-a-positive.md) (an integer is
two parts) and [Q-M1f.353](../../../docs/decisions/stdlib/math/rat/Q-M1f.353-a-rational-is-an-integer-over-a-natural-number.md)
(a rational is an integer over a natural number). Each indexed in its folder, a `given` row in
`utils/decision_census.txt` (`--check`: 351 carried, 351 files, 0 mismatches), cited by the comments that state it
(`int.ein`, `rat.ein`, `ir_semantics.rs`, `presentation_semantics.rs`, `firing.rs`) and by `stdlib.md`.

**Found outside the part**, for the coordinator:

- **Test strings left, in part A's files or L-36's**: `digit_tables.rs:221` (*S1f.6.2's table*, a message, in a
  generator's test); the generators' tests and `stdlib_coverage.rs:452`, `:544` name `utils/` scripts in messages;
  `register.rs`' fixtures (owners, notes, dates); `cli_semantics.rs`' editor-grammar test reads
  `utils/vscode-ein/`, and `detector.rs` runs `utils/region_synth.py`.
- **Lists of the Python prototype's spellings** in `cli_semantics.rs`, `corpus_cli.rs` and `compile_semantics.rs`
  hold `"S1.8."` and `"M1 compiler"` as strings no output may contain: data, left.
- **`config_reference.rs`' module doc** says *the 17 `(config …)` flags*: 2 flags and 15 retired. Left.

**Commits**, on the worktree's branch: `faff9aa8` (what the binary prints, with every golden and expected output
that quotes it), `7044d6ab` (test messages and names, and L-42's part), `dece9b50` (L-32), `b2156d6b` (L-40),
`0fd189db` (L-46), and this record. A file whose changes belong to two of them was staged through the index in
its intermediate state, so each commit holds its own change alone.

**Checks**, on the last code commit: the tests of every file, golden and expected output touched, green but
`wrong_answers.rs` as above — `ein-ir`: lib, `grammar_pages`, `load_semantics`, `fuzz_properties`,
`grammar_decisions`, `generated_parser`, `engine_names`, `kb_semantics`, `ir_semantics`; `ein-cli`: `help_surface`,
`config_reference`, `cli_semantics`, `region_modes`, `lever_invariance`, `guide_programs`, `world_anchors`,
`corpus_cli`, `macro_bodies`, `expect_cli`, `primitive_arity`, `read_out_reference`, `region_dump`,
`answer_round_trip`; `ein-core`, `ein-einb`, `ein-corpus` whole; `ein-infer`: lib, `compile_semantics`,
`naf_semantics`, `detector`, `stdlib_coverage`, `int_theory`, `rat_theory`, `region`; `ein-render`: lib,
`id_order_invariance` (and under `EIN_CORPUS_SLOW=1` for the bless), `jobs_invariance`, `presentation_semantics`.
`cargo fmt --check`, clippy `--all-targets -D warnings`, rustdoc `-D warnings`, `doc_audit.py --links --check
--tracked` (0 and 0), `check_release_voice.py` and `decision_census.py --check` are green, and the string scan of
`ein.rs/crates/*/src` finds a milestone, stage, task or followup id, a plan or `utils/` path or a date in no printed
string outside `register.rs`. No `./run_tests.sh`.

## Record: part A (2026-11-17)

Run in a worktree beside part B, on the findings [L-23, L-31, L-36, L-39, L-43](findings.md): the register's split
and the project tests that read or run the process. Section 2 (the tests) ran in a fork of this part in the same
worktree; part B's branch was merged in at the end and its forbid text applied to the rows that quote it.

**The register splits** (L-39, the coordinator's reading of rulings 6 and 12, the user's to correct).

- **A fixed row is a regression**: [`corpus/regressions.toml`](../../../corpus/regressions.toml), schema
  `ein-regressions/1`, the format of `ein_corpus::regressions`. 333 rows: the 332 fixed ones and the one
  `not-wrong` row, `branching/06`'s, read as a regression, since its right answer is decided (Q-M1e.12 (a)) and the
  row holds a program's answer to it; what made it `not-wrong`, its review, is process and went. A row keeps what
  a regression needs: a name, the title (the wrong answer it guards against; W16's and W28's say no *int*), the
  settings (one stage id dropped), `decisions` (the decision ids among the old `question`, each resolving to a
  file; the 81 entries that were stage ids, findings or reviews dropped), the witness, an `id_order` where a
  permutation once moved the answer (three rows, lifted out of the dropped `[case.wrong]`), `[case.right]` with no
  `reading`, `derived` or alternatives, `[case.region]` and `[case.now]`. Dropped: `case`, `class`, `owner`,
  `status`, `look_at`, `note`, `open_because`, `[case.wrong]`, `[case.control]`, `[[case.twin]]`, `[case.review]`,
  `[case.fixed]`, so no row carries a register id, an owner, a followup, a date, a commit or a stage id. The names
  are the old id's slug; where two rows shared one, the witness's stem in front (`einb-every-command/saturate`);
  where they shared both, by what they check (`d3-unsat-core-id-order`, `binding-key-layout-names`); W75's are
  `closed-stdout/<run>`, and four short slugs take their witness's name. *Why `regressions`*: the corpus already
  calls the witnesses' group `regression`, and the word says what a fixed row is in a project that tracks no
  wrong answer.
- **What holds them**: `ein-cli/tests/regressions.rs` runs every row's `[case.now]` through the binary and
  recomputes every `[case.region]` with the library (`EIN_BLESS=1` re-banks the blocks); `ein-corpus`'s
  `regressions` module holds the format, every row's `now` answering its `right` without a run (`Held::Right`), no
  key the format does not read, every witness a corpus program and every decision a file;
  `id_order_invariance.rs` holds the three `id_order` rows, now that none is open, to not moving.
  `register_region.rs` and `wrong_answers.rs` are gone into those.
- **An open row is a wrong answer, a work item**: [`plans/wrong_answers/register.toml`](../../wrong_answers/register.toml),
  15 rows in the same format with the register's own keys beside it (kept whole, W ids, owners F21, F22, F24, the
  recorded walk answers, reviews), and [its README](../../wrong_answers/README.md), the filing rules that were
  AGENTS.md's (AGENTS.md now points there). [`fixed_rows.tsv`](../../wrong_answers/fixed_rows.tsv) maps the 333 old
  ids to the regressions' names.
- **The runner, generic, in the project**: `ein-cli`'s example `held_answers --ein BIN [--open] [--bless] FILE`
  holds any file of the format: its format checks, every row right (`Held::Right`) or, with `--open`, none right by
  the method (`Held::Open`: the old `an_open_row_is_open_by_the_method`, *contested* read as a `right` with
  alternatives instead of the process word in `class`), every `[case.now]` through the binary given, every region
  through the library. It shares `ein-cli/tests/held/` (the runner and the region check, `mod.rs` and
  `regions.rs`) with `regressions.rs`, so the project's test and the process's step run one code. Not in
  `ein-corpus`: `serde_json` in its graph made `assert_eq!(v, [])` ambiguous in three `ein-infer` tests
  (`cover`, `theory_share`, `exactly_one`); not in `ein-infer`: it needs the binary and the library both, and
  `ein-cli` has both.
- **Where the gate runs it**: [`utils/wrong_answers.py`](../../../utils/wrong_answers.py), a step of `run_tests.sh`
  and `per-commit.yml` after `check_region_routes.py`, outside the static-check guard (`check_gate_steps.py` holds
  the two lists the same: 16 steps). It checks the register's own keys (the validation that was `register.rs`'s
  and named plan ids: an owner is a followup with a file under `plans/followups/` or a stage, `status` open,
  `open_because` stated, a review's vocabulary, date and bundle, `deferred` waiting on its owner, `look_at`'s
  words, each recorded answer's `run`, `by` and `why`, one title and owner per case, no unknown key), builds `ein`
  and the example under the gate profile, and runs `held_answers --open` on the register: 15 rows, 15 runs, 15
  regions. A `[case.now]` changed by hand (`k = 1` → `9` on `W8/features-07`) fails it, exit 1, on the format and on
  the run, so a silent fix still fails the gate.
- **The witnesses**: the ten files used by nothing but their open rows moved to
  [`plans/wrong_answers/witnesses/`](../../wrong_answers/witnesses/), with their corpus entries and their 20 lines
  of `corpus_exits.txt` and 390 of `corpus_shapes.md5`: `closure-shared-given` and its control, `guard-only-second`
  and `guard-only-first`, `alt-flag-refuted-by`, `alt-flag-two-derivations`, `written-rule-no-such-rule`,
  `written-rule-leaves-core` and its twin, `written-using-never-held`. **Five stay, a decision the user may want to
  revisit** (the default said the witness moves): a project test or page reads each for a claim of its own —
  `alt-cap-shared` (Q-M1e2.11's documented limit, `explain_semantics.rs`), `f07-rules-swapped` and `f07-fpoff`
  (`detector.rs`, `monotonicity.md`'s c8), `or-one-key` and its twin (`h_space_view.rs`, `show_hyps.rs`,
  `alive_invariant.rs`) — and the five project programs that are open rows' witnesses (`features/07`,
  `lattice/10_fifo_gap`, `corner-house`, `fork-delta-order`, `zebra2`) stay as they are. The project names no open
  row: each states its gap in words.
- **What followed** (L-39, L-23): `explain.rs`, `kb.rs`, `explain_semantics.rs` say *a known gap* for
  `W10/alt-cap-shared`; `lever_invariance.rs`' exemption is `REFUTED_BY_MOVES`, `features/12` alone, held by
  `the_refuted_by_gap_still_has_its_program` (the two W36 witnesses left with their rows); `answer_round_trip.rs`'
  `WRITTEN_PROVENANCE` is empty (its one entry moved); `KNOWN_R1`, `DO_NOT_READ_BACK`, `DO_NOT_LOAD`,
  `ORACLE_STOPS` and the lists beside them say *known wrong answer*, not *open register row*; `monotonicity.md`'s
  c8 row says no test states it; `detector.rs` counts 47 flagged corpus programs, two flagged witnesses having
  left; `answer_dump.rs`, `route_invariance.rs` and the thirty sweeps that added the register's witnesses (and
  their controls and twins) add the regressions' (every one a corpus program); the review bundle reads the open
  register, `guard_split.py` and `region_census.py` both files.
- **Every project file names a fixed row by its regression's name** and `corpus/regressions.toml` for
  `corpus/wrong_answers.toml`: 334 files by `fixed_rows.tsv` (a `W<n>/<slug>` id, with *register row(s)* →
  *regression(s)*), then the register's other phrasings (*the register's witnesses*, *case W15 of the register*,
  …) by hand or by phrase, the corpus's witness notes and witness headers among them, every `.ein` its line count
  kept. `fuzz_seeds.txt` re-blessed, four seeds, each checked comment-only (its text with `;` comments stripped is
  the same). The 116 witness notes of `corpus.toml` that narrated their defect in the past tense or cited a case id
  are restated as what the regression checks.

**The project tests that read or run the process** (L-31, L-36), each check moved, and where it runs now:

| check | was | now |
|---|---|---|
| `run_tests.sh` against `per-commit.yml`, `--nightly` against `nightly.yml` (matrix expanded), `--dev-host` against the nightly and `DEV_HOST_ONLY`, `--tests-only`'s guard and the header's count, the fuzz budgets, the checked `TMPDIR`, every shard, every slow half on a tier | `gate_steps.rs`, `nightly_steps.rs`, `steps/mod.rs` | `utils/check_gate_steps.py`, a static gate step |
| the editor grammar's four lists against the engine's, the NL census's kernel against `RESERVED`, the review driver's artefact flags against `ein solve --help` | `cli_semantics.rs`, `natural_language_page.rs`, `help_surface.rs` | `utils/check_tool_mirrors.py`, a static gate step |
| every generator's `--check`: the digit and bit tables, `num9`, the gates, the arith examples, the read-out examples page, the zebra2 and SEND variants | `digit_tables.rs`, `num9_theory.rs`, `bool_theory.rs`, `arith_search_free.rs`, `read_out_examples.rs`, `cli_semantics.rs` | `utils/check_generated.py --bin ein.rs/target/gate/ein`, after the tests |
| the oracle against `region.py` on ten programs, `region_synth.py --random 300` read back | `region_dump.rs` | `utils/check_region_routes.py`, after the tests |
| the detector's 200 random programs, the width-2 num9 theory | `detector.rs` and `num9_theory.rs` spawning a script | banked in the project (`ein-infer/tests/golden/random_programs.txt`, `num2_theory.ein`), written by `gen_random_programs.py` and `gen_num9.py`, checked by `check_generated.py` |
| the lexer over `plans/`' `.ein` files | `generated_parser.rs` | dropped (by part B and this part alike): the project's 901 files, 500 mutants, the edge cases and 20 000 random strings stay |
| the page's kernel row; its decision ids | `natural_language_page.rs` against `utils/nl_census.py` and a plan's ledger | the same test, against `RESERVED` and `docs/decisions/` |

What each test keeps of the project stays in the crate; `stdlib_coverage.rs`' two messages name no script. The
two example generators moved from `examples/` to `utils/` (process tooling), and the five files they write say they
are generated and name no tool, each line count kept. `--tests-only` now skips the two new static checks.

**The process writes into the project** (L-43): `utils/fuzz_engine.py`'s `write_finding` writes no found-date and
no command line (an engine find names its corpus run, an id-order find *under a permuted id space*);
`tests/mutation_survivors.txt` stays in `tests/` (the suite's own record, explained line by line in
`tests/README.md`), its header naming no script.

**Part B, merged** (`6002b8b7`): three conflicts — `lever_invariance.rs` keeps this part's exemption over part B's
rename of the W36 test it replaced; `region_modes.rs` takes part B's message; the shape golden drops the moved
witnesses' lines and keeps part B's re-banked ones. Part B's forbid text, `…; Q-M1e2.5)` for `…; forbidden since
M1e2 S1e2.2.5)`, is applied to the 16 `refused` lines of `corpus/regressions.toml` that quote it. The five
qualifiers that said *open (F21)* were alternatives' and the note that cited `read_out.md § 8.2` a row's note: both
went with the process fields, so no row records them, and the open register records neither.

**Commits**: `05893b61` (L-43), `c965d070` (the tests that read the process), `7accb3be` (the split), `6002b8b7`
(part B merged), `7982f4a9` (the witness notes), and this record with the followups' links.

**Checks**, on the merged tree: `cargo test --profile gate --workspace --no-fail-fast`, 160 test binaries green (one floor moved with the corpus: `explore.rs`' flagged ∅-rule cells, 75 → 70, the moved flagged witnesses' five each gone, as `detector.rs`' count 49 → 47); `python3 utils/wrong_answers.py` (15 rows, 15 runs, 15 regions, 0 defects) and a hand-mutated `[case.now]` failing it; `python3 utils/review_bundle.py --smoke` (15 rows read, 2 cases and 1 program built, 25 SVGs, the link check 0); `check_gate_steps.py` (16, 75, 79 steps), `check_tool_mirrors.py`, `doc_audit.py --links --check --tracked` (0 and 0), `check_release_voice.py`, `decision_census.py --check`, `cargo fmt --check`, clippy `--all-targets -D warnings` and rustdoc `-D warnings`. No `./run_tests.sh`.

**Decisions the user may want to revisit**: the name `regressions`; `branching/06`'s `not-wrong` row as a
regression; the five witnesses an open row shares with a project test staying in the project; a regression row
carries no explanation of its right answer beyond its title and decision ids (`derived` and `reading` are history
and stayed in git); the runner as an `ein-cli` example sharing `tests/held/`; the review bundle reading the open
register alone, so its smoke builds `W39/or-one-key` and `W87/written-rule-no-such-rule` and no longer exercises the
container and closed-stdout pages, whose rows are regressions now; `fixed_rows.tsv` kept on the process side for
the W ids the project still cites; the generators in `utils/`; `region.py` and `region_synth.py` read as the
oracle's second route (a process check) rather than as fixture writers.

**Found outside the part**:

- **1 250 bare case ids (`W62`, `C1`) remain in 457 project files** (`git grep -cE '\b[WC][0-9]{1,3}\b' -- ein.rs
  stdlib examples tests corpus docs README.md ':!docs/lib' ':!docs/history'`), none with a slug: a case is several
  rows, so no map renames them; the release-voice check does not read them (Q-M1f.314). A pass that restates each
  is the W ids' share of the sweep.
- **The ten moved programs leave the project's sweeps** (oracle agreement, the routes, claims, the agenda, about
  thirty): the process holds each row's recorded run and region, not those properties.
- **Part B's forbid text and every printed string a regression records move together**: a later change to what
  the binary prints edits `corpus/regressions.toml` in the same commit, as any golden.

## Record: part C2 (2026-11-17)

Run in a worktree beside lane C1 (the deletions, the decision census, `AGENTS.md`, `utils/README.md` and the plans'
indexes), on part A's finding *1 250 bare case ids remain in 457 project files* and item 5 of § What it does. No
file of C1's was edited: `AGENTS.md`, `utils/README.md`, `docs/history/`, `plans/` but this record.

**The register's case ids leave the project.** `git grep -cE '\b[WC][0-9]{1,3}\b'` over the project read 1 219
lines in 457 files. Of them, 781 lines in 417 files were case ids in prose (the widened check's count, below), 12
were case ids in code, a string or a golden (`models_grid.rs`' `const W1`, `open_reserved.rs`' message, the two
`ein.l` comments, `fuzz_seeds.txt`'s quote of `empty_absent.ein`, `site_census`'s `C1`, `types_as_objects.rs`'
and `oblig-stdlib-total-owed.ein`'s `C1`), and one was spelled `W-22` (`grammar_pages.rs`). The rest are not case
ids, and stay: the chairs `C1`–`C16` of the seating programs, their pages and drawings (184 lines of `.svg` and
`.dot`), the CLI contract's C1–C4 (`cli.md`, `explore.rs`), the domain contract's C1–C4 (`oblgen.rs`), the split
`sat(P ∪ C1) ∪ C2`, the scratch names `w1`, `w2`, `w17`, `w20`. After: no `W<n>` in any project file; 429 lines of
`C<n>` in 58 files, none a case.

How each went, by kind:

- **A tag beside a regression's name or a decision id, or after a statement it only tagged: dropped** — 506 lines
  by a script over the check's findings, read after (about 260 witness headers' `— **W80**, regression `x`` →
  `— regression `x``, with the line-wrapped `register` part A's split left; about 246 parentheticals, `(W24)`,
  `(`W62`, `W63`)`, `(W66 to W69)`, `(W65, Q-M1f.71)` → `(Q-M1f.71)`). Where a tag stood alone on a line, the
  sentence was re-joined (eleven lines of `.rs`; in the one `.ein`, the line kept empty).
- **A family named by its regressions or its witnesses** — about 120 lines by hand: `a control of **W22**` → `a
  control of regression `kw-and-in-assert`` (fourteen twins), `W17's witnesses` → the `forbid-chain-*` witnesses,
  `W16's second-shape witnesses` → `activator-all-int` and `activator-all-nested`, `the W27 witness` →
  `past-limit-lines.ein`, `W88's regression` → the `import-facts-*` regressions, `W40's six rows` →
  `einb-every-command/*`; the corpus notes' `W14's control for …`, `W22's twin of …` → `The control for …`, `The
  twin of …`, or the regression named (33 forms over 55 notes, by a reviewed table).
- **A defect a narrative named by its id: said in words** — `W1 at layer 0` → an unrecorded survivor at layer 0,
  `W3's clause` → the dead set not upward-closed under `absent`, `W4` → the fresh constant missing from H, `W62,
  W63, W64 and W65` → a nested `(or …)`, a clause past the register file, a goal's relation variable, and every
  command that reads a program.
- **An open row's id: the gap in words** — `W8` (`detector.rs`: *a stale fact can be in a dead node no run
  lists*; `refutation_census`: *the shape that can leave a stale fact*; `monotone.rs`: `features/07`'s `derive-p`;
  `stale-naf-fork-warn.ein`: *the stale fact is not contained*), `W10` (`kb.rs`' tag dropped), `W39`
  (`alive_invariant.rs`' tag dropped). No project file named W25, W36 or W87.
- **Names** — `region_render.rs`' tests `w1_…`, `w2_…`, `w3_…` lose the prefix (scratch tags `missing`, `root`,
  `hole`; nothing else names them), `models_grid.rs`' `const W1` is `FILE`, `cli_semantics.rs`' `w58` is `seeded`,
  `site_census`'s `c1_row`, JSON key `"c1"`, `c1_types`, `c1_drop` and its two printed `C1` labels are `typing`
  (nothing reads the key). **Messages**: `open_reserved.rs`' count, and `guide_transcripts.rs`' two messages'
  review id `CD-L2`, the one plan id the widened patterns found anywhere in the project.
- Found beside them and fixed: `examples/broken/load/import_as_and_symbols.ein`'s `(A1 D3)`, an M1a plan's labels;
  `detector.rs`' *the proposal's § 4.1, approved*; `corpus.toml`'s *under the ruling as before it*;
  `models_grid.rs`' *the right answer since the switch*; `import_invariance.rs`' *until the rung was read off …*.

Every regression a project line names resolves in `corpus/regressions.toml` (272 names and globs, a script over
every `regression(s) `…`` in the project). Every `.ein` changed, 330, keeps its line count and its code (each
compared with HEAD with its comments stripped); `stdlib/algebra.ein`'s one line re-wrote the manifest;
`fuzz_seeds.txt` moved by one seed, `empty_absent.ein`'s comment, re-blessed and checked comment-only (its text
with `;` comments stripped is the same, 19 lines either way). No printed string a golden digests carried a case id.

**The check, widened** (`utils/check_release_voice.py`), each widening measured with `--list` on the merged tree
before any fix:

| widening | what it found | fixed by |
|---|---|---|
| a register case, `W62`, `W8/features-07`, `W-22` | 699 lines over the old scope (361 `.ein` comments, 278 doc comments, 60 `corpus.toml` notes) | the two code commits below |
| a finding, followup or review id: `[EKL]-n`, `F<n>`, `XX-[HML]n` | 0 in the scope; one review id, `CD-L2`, in two test messages, outside it | the crates' commit |
| a process path: `utils/`, `run_tests.sh`, `.github/`, `AGENTS.md`, `CLAUDE.md`; `plans/` and `utils/` named alone | 0 (S1f.20.6 and parts A and B had left none) | — |
| every `.md` of the project (folder READMEs, fuzz notes, the six `ein-render` goldens) | 4 lines: `tests/README.md` ×2, `hrule-reads-not.md`, a witness directory's `README.md`, all case ids | the programs' commit |
| every comment of every crate (plain, trailing, block; `ein-corpus` and `ein-narration` too) | 78 plain or trailing `//` lines of the shipped crates, case ids; the dev-only crates 0 | the crates' commit |
| `ein.l`/`ein.y` comments, `#` comments of `ein.rs/**/*.toml`, `corpus/regressions.toml` whole | 2 `ein.l` lines, case ids; the rest 0 | the crates' commit |

Not added, and why: a plan's `D<n>` (`D1`, `D2` are `std.math.rat`'s denominators and its pages'; no project file
names a plan's D row now); a `C<n>` case (a chair, a contract clause); a commit hash (below). **The baseline and
the ratchet are deleted**: `utils/release_voice_baseline.txt`, `--bless`, the fell/rose report; any finding exits 1
with the files and their counts, `--list` prints the lines. The script no longer reads itself (process, as
`AGENTS.md` is), so its docstring carries no escape; `NOT_PROJECT` keeps `docs/history/` until C1's deletion lands,
when the entry is dead and may go. `history_census.py` imports the same names (the kinds `milestone` … `date` kept)
and calls its crate column *crates' comments*; `run_tests.sh`' header and step comment and `per-commit.yml`'s say
what the step holds now; `check_gate_steps.py` reads the same 16, 75, 79 steps.

**For the coordinator**: `utils/README.md`'s row (C1's file) could read, in place of the ratchet's text:

> | [`check_release_voice.py`](check_release_voice.py) | **the release voice**: no milestone, phase, stage or task id, no plan's finding, followup or review id (`E-33`, `F21`, `CD-H1`), no case of the wrong-answer register (`W62`, `W8/…`), no path into the process (`plans/`, `utils/`, `run_tests.sh`, `.github/`, `AGENTS.md`, `CLAUDE.md`) and no ISO date in the project's prose: every `.md` of the project, every comment of every crate (and `ein.l`, `ein.y`, the `.toml` files under `ein.rs/`), every `.ein` comment, and `corpus/corpus.toml` and `corpus/regressions.toml` whole. A decision id (`Q-M…`) passes, and so does a line marked `release-voice-ok: <reason>`. No baseline: any finding exits 1, and `--list [PATH …]` prints the lines | per-commit CI |

and `AGENTS.md`'s *The release voice is a ratchet* rule, and its `--list … --bless` command line, lose the
ratchet and the baseline the same way. Until one of them lands, `doc_audit.py --links --check --tracked` finds one
link, `utils/README.md:118`'s to the deleted baseline.

**Q-M1f.314's readings this changes**: (1) every comment of every crate, plain, trailing and block, the dev-only
crates and the grammar's sources too, not a shipped crate's doc comments; (4) `plans/` named alone fails, as
`utils/` does: the project names no process; (5) followup, finding, register and review ids are read now; what is
still not read is a `.ein` string (`:why`, `:source`, `:goal-text`), a Rust string (part B's scan, not a gate), a
plan's `D<n>`, a `C<n>` case and a commit hash; (6) moot: with no baseline nothing is banked, so nothing falls;
(7) the check does not read itself. The scope also widens from `docs/ein/`, `docs/decisions/` and `README.md` to
every `.md` of the project, and gains `corpus/regressions.toml`. Readings (2) and (3) stand.

**Commits**, on the worktree's branch after `git merge master`: `b5a4787f` (the programs, the stdlib, the corpus
and their pages, with the fuzz seeds' golden), `6e75f093` (the crates: comments, names, messages), `e1f00ddc` (the
check widened, the baseline deleted), and this record.

**Checks**: `check_release_voice.py` exits 0 over 1 718 files with no baseline, and 1 on a probe line
(`W8/features-07` appended to `README.md`, then restored); `doc_audit.py --links --check --tracked` 1 and 0, the one
link above; `cargo fmt --all --check`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`; under the gate
profile, `ein-ir` whole (`fuzz_properties` blessed, `grammar_pages`, `generated_parser`, `load_semantics`,
`dump_goldens`, the lib's manifest test), `ein-cli`'s `region_render`, `models_grid`, `cli_semantics`,
`open_reserved`, `guide_transcripts`, `goal_premises`, `einb_cli`, `import_facts`, `closed_stdout`,
`conclusion_shapes`, `import_invariance`, `hrule_reads_not`, `corpus_cli`, `ein-infer`'s lib, `stdlib_coverage`,
`alive_invariant`, `compile_semantics`, `detector`, `corpus_cores`, and `ein-render`'s `id_order_invariance` (no
shape moved by a comment); `check_gate_steps.py`; `history_census.py` runs. No `./run_tests.sh`.

**Found outside the part**:

- **Commit hashes in the witnesses' narratives**: 293 lines in 264 files under `examples/ein-bugs/` say what the
  engine answered *at `6a775f3d`* and the like, the defect's history in the past tense, as part A found of the
  corpus notes. No check reads a hash; a pattern for one (a backticked 7–12 hex) would find exactly these.
  `alt-cap-shared.ein:31` names one bare. Restating them is the witness headers' share of the sweep.
- **The `site_census` and `refutation_census` examples** print per-site census rows a process census reads
  (`utils/censuses/hole-census.md` cites `site_census --json`); they are instruments in a shipped crate's
  `examples/`.
- `NOT_PROJECT` in `check_release_voice.py` and in `doc_audit.py` list `docs/history/`, dead once C1's deletion
  lands.
