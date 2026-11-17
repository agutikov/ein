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
