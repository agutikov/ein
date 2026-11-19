# S1f.21.15 — *forbid* is removed, and nothing validates a non-monotone program's answer

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** **done 2026-11-19**: `1f063ed9` (task 1), `15689488` (task 2), `1b9f87f0` (the warning) and the
record's commit ([§ Record](#record)). Planned the same day on the user's ruling, as *forbid becomes a warning*, and
started on the user's word (*"Remove this *forbid* from the code and docs completely"*), its scope as the second
ruling gives it. The file keeps its first name. **Left to the coordinator**: the gate, the nightly and the
container on the last commit, which the stage was told not to run; nine links in plan files that point at what it
removed, which fail the gate's link check until re-aimed; and one question for the user, the row
`W8/fifo-gap-root` (the record, § Task 2).
**Waits on:** no decision: [Q-M1f.564](../open_questions.md#q-m1f564--a-non-monotone-program-loads-with-a-warning-forbids-refusal-goes).
It runs before [S1f.21.11](s1f.21.11_kill_and_three_conventions.md), which then has no *forbid* to re-aim.
**Writes:** `ein-infer/src/forbid.rs` and what calls it, gone; the read-out and the summary, a warning; the tests
that pin a refusal or a flagged program's answer; `corpus/regressions.toml`, the shape golden, the register of
wrong answers; the decisions; `AGENTS.md`; every page of `docs/` and every comment that speaks of the refusal.

## The rulings

The user, 2026-11-19, verbatim, first: *"I tell you what I expect - it loads, ein print huge WARNING, and then we guarantee nothing about it's output. NO!  Q-M1e2.5 IS WRONG! What the hell this `forbid` is? Where did it came from?"*

And then: *"Ok, I cancel those my listed decisions regarding absent -> false.  We allow to load non-monotone programs, but we don't claim about solution. That means - no tests validate the result of the solution of non-monotone programs. Remove this *forbid* from the code and docs completely."*

So:

1. **The user's rulings of 2026-11-02 and 2026-11-04 on a refutation under an `absent` are cancelled**, *forbid* and
   its derived case with it (Q-M1e2.5, whose decision file this stage removed).
2. **A non-monotone program loads.** Ein warns, loudly, and claims nothing of its answer.
3. **No test validates the result of a non-monotone program's solution.**
4. ***Forbid* goes from the code and from the pages, completely.**

## What it touches, counted before the stage

On the gate build of `946ee788`, over the 893 programs under `examples/` and `tests/` (`ein solve FILE --no-search
--json-summary`, the summary's `monotonicity.monotone`; `ein show hyps` for the refusal's words):

- **20 programs are refused by *forbid*** (16 under `examples/ein-bugs/`, 2 under `examples/branching/`, 2 under
  `examples/lattice/`). 15 are the witness of a row of `corpus/regressions.toml`, whose right answer is the refusal;
  2 state an `:expect`.
- **27 programs load and are flagged** by the static check (12 under `examples/ein-bugs/`, 8 `examples/lattice/`, 3
  `examples/features/`, 3 `examples/saturation/`, 1 `tests/region/`). Of their results, today: 4 state an `:expect`;
  9 are the witness of a row of `corpus/regressions.toml`; the shape golden holds, for each, a line of every kind,
  among them the kinds that are an answer (`read-out`, `cores`, `states`, `region-lattice`, `oracle`, `explain`,
  `region-trace`); and the register of wrong answers has five rows of them (`W8`), each held to what the binary
  answers today.

## Task 1 — *forbid* goes

1. **First, that each of the 20 is flagged by the static check once it loads.** A route's licences are given up on
   a flagged program, and its routes are the cube's. A program that loads unflagged would be searched with licences
   that do not hold of it. If one of the 20 is not flagged, the stage stops there and says so before anything else.
2. The check, its module, its callers, its tests and its words go: no command refuses a program for an `absent`.
   The messages that told a user how to rewrite a program so as not to be refused go with it.
3. **The warning.** A flagged program is warned of as it is today, `lattice not monotone`, its sites and its stale
   facts; **Reading 1**, default: and louder, one block at the head of the read-out and one line on stderr, in
   capitals, that says the program is not monotone and that **nothing is claimed of this answer**, on every command
   that answers about a commitment. The other reading: the rows as they are.
4. The pages and the standing rules: the decision of Q-M1f.564 written, Q-M1e2.5's removed and every citation of
   it re-aimed; `AGENTS.md` § The language, *forbid*'s rule replaced by the ruling; the diagnostics' § 2.7 gone;
   the top `README.md`, the reasoning's pages on `absent` and on monotonicity, the read-out's page, the glossary,
   and every comment in `ein.rs/`, `stdlib/`, `examples/` and `tests/` that says a program is refused for it.

## Task 2 — no test validates a non-monotone program's result

Of the 47 programs, the 20 and the 27. **What goes**:

| what validates today | what becomes of it |
|---|---|
| an `:expect` a flagged program states of its answer | **kept in the program**, on the user's correction of this row (*":expect claims can present in non-monotone programs - just don't put it into the test suite gate and CI."*): what goes is every reading of it by the suite, the gate and CI, the program's `test` run in `corpus/corpus.toml` and any sweep that runs `ein test` over it; a flagged program under `tests/`, which is the suite, moves to `examples/` with its claim |
| a row of `corpus/regressions.toml` whose witness is one of the 47 | removed; the witness stays a program of the corpus. The rows are listed in the record, for M1n |
| the shape golden's lines of an answer's kind for a flagged program (`read-out`, `cores`, `states`, `region-lattice`, `oracle`, `explain`, `explain-noalts`, `region-trace`) | not banked and not compared |
| a test in a crate that asserts what a flagged program answers: a region, a count, a core, a model | the assertion goes; listed in the record |
| the register's rows of non-monotone programs (`W8`), held to what the binary answers | removed from the register; listed in the record, for M1n, whose rows they were |
| the expected refusals of the 20 (exit codes, messages) | the programs load: exit 0 |

**What stays**, since it validates no answer: that each of the 47 loads and every command exits 0 on it; that the
static check flags it and the warning is printed; that every command gives the same $R$ of it, which is the user's
standing rule of one answer and compares a run with a run; the lines of the shape golden that are no answer
(`load`, `ir`, `plan`, `match`, `naf`, `saturate`, the drawings of the program).

**Reading 2**, default: `ein test` is not changed: on a flagged program it checks a claim as it does today. Whether a
claim of a flagged program is *not checked* is put to the user and not built.

## What must hold

- **Every answer of a monotone program**: $R$ of every corpus program the static check does not flag, by the corpus
  sweep and the oracle; `ein-cli/tests/lever_invariance.rs`; the register's other rows and every other row of the
  regressions.
- **No licence on a flagged program** (task 1.1), held by a test over the 20.
- **Nothing is claimed**: no page, read-out, comment or test says an answer of a flagged program is right, sound or
  complete, or that such a program is refused.
- The goldens move for the 47 and where a refusal's line was pinned; the diff is read.
- A large change to what loads: the gate and the nightly on the stage's last commit.

## Acceptance

`grep -ri forbid` finds the word in no file of `ein.rs/`, `docs/`, `stdlib/`, `examples/`, `tests/`, `corpus/`,
`README.md` or `AGENTS.md` in the sense of this check; each of the 20 loads, is flagged and prints the warning; no
regression row, no golden line of an answer's kind and no assertion is left on any of the 47, and no step of the
gate or of CI checks an `:expect` of one, which stays written in its program.

## Record

The stage ran on 2026-11-19 in the main tree, in four commits. The order was task 1, task 2, the warning, where
the plan had the warning with task 1: the warning moves the read-out of every flagged program, and with task 2
first the shape golden banks none of those read-outs, so no golden was re-banked twice.

| commit | what |
|---|---|
| `1f063ed9` | task 1: *forbid* removed from the code and the pages; the 20 load |
| `15689488` | task 2: nothing validates what a flagged program answers |
| `1b9f87f0` | the warning, Reading 1 |
| this one | the record |

### Step 0: each of the 20 is flagged once it loads

Taken before anything was removed: the refusal's block in `region::prepare_as` switched off, the gate binary
built, and for each program `ein solve FILE --no-search --json-summary F`. All twenty: `monotonicity.monotone`
false, a `lattice  not monotone` row, exit 0. It is also so by construction: the removed check returned nothing on
a program with no site. The columns below were re-taken on the binary of `1f063ed9`: the sites, and R's two counts
and where root stands as a bare `ein solve` prints them, **of which nothing is claimed**; then the exit code of
each command. `ein test` exits 2 on all twenty: none states a claim, *nothing to check*.

| program | monotone | sites | k_s | k_c | root | exits (solve, --minimal, --maximal, --single, -n 2, --count, --monotone, oracle, show hyps, test) |
|---|---|---|---|---|---|---|
| `examples/branching/13_lookahead_naf_world.ein` | false | 1: self-block (absent) | 3 | 4 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/branching/14_lookahead_unjudgeable.ein` | false | 1: premature-signoff (absent) | 3 | 4 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/activator-all-int.ein` | false | 2: flag (absent); watch (activator) | 0 | 0 | dead | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/activator-all-nested.ein` | false | 2: flag (absent); watch (activator) | 0 | 0 | dead | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/activator-all-symbol.ein` | false | 2: flag (absent); watch (activator) | 0 | 0 | dead | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/alive-empty-interlayer.ein` | false | 1: totality (absent) | 4 | 4 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/d3-unsat-core.ein` | false | 1: fire-1 (absent) | 20 | 20 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/naf-upward-closure.ein` | false | 1: bad (absent) | 3 | 3 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/activator-chain-0.ein` | false | 2: mk (absent); q (activator) | 3 | 3 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/activator-chain-1.ein` | false | 2: mk (absent); q (activator) | 3 | 3 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/activator-chain-2.ein` | false | 2: mk (absent); q (activator) | 3 | 3 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/activator-chain-node-absent.ein` | false | 1: q (activator) | 3 | 3 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/activator-chain-node-writes.ein` | false | 1: q2 (activator) | 3 | 3 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/activator-chain-param-absent.ein` | false | 1: watch (activator) | 3 | 3 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/naf-nowb.ein` | false | 1: bad (absent) | 3 | 3 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/rung-connex-import.ein` | false | 1: connex (absent) | 0 | 0 | dead | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/stale-naf-fork.ein` | false | 1: derive-p (absent) | 1 | 1 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/ein-bugs/wrong-answers/stale-naf-fork-warn.ein` | false | 1: derive-p (absent) | 1 | 1 | in-r | 0 0 0 0 0 0 0 0 0 2 |
| `examples/lattice/synth/three_components.ein` | false | 2: dead-3 (absent); dead-7 (absent) | 13 | 13 | owing | 0 0 0 0 0 0 0 0 0 2 |
| `examples/lattice/synth/trigger_hole.ein` | false | 3: dead-1 (absent); dead-2 (absent); owe-2 (obligation-trigger) | 9 | 9 | owing | 0 0 0 0 0 0 0 0 0 2 |

On the stage's last binary every one of the 47 flagged programs gives one R on every command
(`ein solve`, `--count`, `--single`, `ein oracle --dump`, k_s and k_c compared): 40 whole, the 20 among them (describe exhausts, its k_s and k_c are `--count`'s and those of the oracle's cube, and all four commands exit 0), and 7 that stop at the default `--region-limit`, where describe and `--count` give one bound and `ein oracle` declines a cube over the limit, as it does of any program.

### Task 1: what went, and what stayed under another name

**Gone**: `ein-infer/src/forbid.rs` whole (the check, its causes, its message and remedies, `forbid::studying`);
its call in `region::prepare_as`; `ein-infer/tests/refutation_under_absent.rs`; the refusal's columns and options in
the census examples (`rung_census`'s refusals column, `refutation_census`'s `--option-e`); the decision
`Q-M1e2.5`'s file and every citation of it; the diagnostics' § 2.7 (the page's last section, § 2.32, took its
number, so no other section moved); the monotonicity page's § 10 (§ 11 and § 12 are § 10 and § 11); the stdlib
page's section on a refused program; the remedies in the NAF map's line; `utils/check_region_routes.py`'s list of
refused programs; 16 regression rows whose right answer was the refusal (below).

**Kept, under a name of its own**, because another reader uses it and it refuses nothing:
`ein-infer/src/extendable.rs`: `relations`, the relations a commitment can extend followed through each rule's
activator (D⁺), which the censuses read; and `derivable_activators`, which the obligations rung
(`oblgen.rs`) reads to know what a commitment can bring a rule to owe. `SolveError::Refused` stays for a saved
file whose `(commit …)` names no node. The static check (`ein_infer::monotone`) is untouched: detection stays.

**Renamed**: seven witnesses, `examples/ein-bugs/wrong-answers/forbid-chain-{0,1,2,no-absent,node-absent,node-writes,param-absent}.ein`
to `activator-chain-*`, with every reference and the regression id `forbid-chain-no-absent`.

**Written**: the decision
[Q-M1f.564](../../../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md);
`ein-infer/tests/non_monotone_loads.rs`, which holds the 20 and the two written activator programs to *loads,
flagged, every mode answered by the cube (or by root alone where root is dead) with no licence that is a theorem
about the program*; AGENTS.md's standing rule, under § The answer.

Two mutants of `std.bijection`'s `bijective-setup` that only the refusal killed survive now and are banked
(`tests/mutation_survivors.txt`, 119 survivors of 637).

### The warning, Reading 1

On a flagged program, `ein solve FILE` (every question, `--count` and `--no-search` among them) and `ein oracle FILE`
print, as the read-out's first row:

```text
solve · examples/ein-bugs/naf-upward-closure.ein --maximal
──────────────────────────────────────────────────────────────
  WARNING           THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER.
                    IT IS NOT CLAIMED TO BE RIGHT, SOUND OR COMPLETE (Q-M1f.564).
                    THE `lattice` ROWS BELOW NAME THE SITES.
  asked             the maximal line — every state on it
  …
  lattice           not monotone: 1 site (the static check) — 1 hole
```

and, first on stderr, as do `ein test FILE` (once a file), `ein show hyps FILE` and `ein render lattice FILE`:

```text
WARNING: examples/ein-bugs/naf-upward-closure.ein: THIS PROGRAM IS NOT MONOTONE: NOTHING IS CLAIMED OF THIS ANSWER (Q-M1f.564)
```

The JSON summary's `monotonicity.warning` is the sentence, `null` on a monotone program. `--monotone`, whose answer
is the verdict, prints neither. The `lattice` rows are as they were. One constant, `ein_infer::detector::WARNING`;
`ein-cli/tests/non_monotone_warning.rs` holds the three on every corpus program (flagged exactly when warned) and
the commands on a probe. `08-read-out.md` has the row in its three tables, § 5.1 and § 10;
`09-read-out-examples.md` was written again by the binary (46 lines added, the row and the stderr line on its ten
read-outs of flagged programs, nothing else moved).

**Reading 2**, taken: `ein test` is not changed. On a flagged file it checks a claim as it checks any, and writes
the stderr line.

### Task 2: what no longer validates a flagged program's answer

The set is the 47 corpus programs the static check flags, computed by each reader that can compute it
(`ein_infer::explore::check_monotone`): no test leaves a program out by a hand-written list.

**The claims stay.** Three flagged programs state an `:expect` (the plan counted six; in three of them the word
is in a comment). Each is kept as written, and no step of the gate or of CI reads it:

| program | its claim | what read it before | now |
|---|---|---|---|
| `examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed.ein` | `(solve … :expect (region …))` | `corpus_cli`'s declared run `solve` (a refuted claim exits 1) and the regression row `oblig-stdlib-total-owed` | declares `solve --count`, which reads no state and checks no claim; the row is removed |
| `examples/ein-bugs/wrong-answers/oblig-stdlib-total-owed-declared.ein` | the same | `corpus_cli`'s declared runs `solve` and `test` | declares `solve --count` |
| `examples/lattice/17_flagged_region_claim.ein`, which was `tests/region/07_cube.ein` | `(solve … :expect (region …))` | `ein test tests/` and the stdlib conformance pair (`stdlib_coverage.rs`), `corpus_cli`'s declared run `test`, and two tests that ran `ein test` on it (`test_cli.rs`, `read_out.rs`) | moved out of `tests/`; declares `solve --count` and `saturate`; the two tests read a monotone program |

`ein-corpus`'s `a_flagged_program_declares_no_run_that_checks_a_claim` holds it: no flagged entry declares a
`test` run, and one whose program states a claim declares no `solve` run that reads it; no program under `tests/`
is flagged. `ein test` over `tests/` held 1 claim more before the move (the moved program's one).

**Regression rows removed**, `corpus/regressions.toml`, 27 of 329 (302 left); every witness stays a corpus program.
In `1f063ed9`, the 16 whose right answer was the refusal:

| id | witness | run |
|---|---|---|
| `alive-empty-interlayer` | `examples/ein-bugs/alive-empty-interlayer.ein` | `solve --maximal` |
| `d3-unsat-core/d3-unsat-core` | `examples/ein-bugs/d3-unsat-core.ein` | `solve --maximal` |
| `d3-unsat-core-id-order` | `examples/ein-bugs/d3-unsat-core.ein` | `solve --maximal` |
| `naf-upward-closure` | `examples/ein-bugs/naf-upward-closure.ein` | `solve --maximal` |
| `naf-nowb-L` | `examples/ein-bugs/wrong-answers/naf-nowb.ein` | `solve --maximal` |
| `three-components` | `examples/lattice/synth/three_components.ein` | `solve --maximal` |
| `trigger-hole` | `examples/lattice/synth/trigger_hole.ein` | `solve --maximal` |
| `stale-naf-fork` | `examples/ein-bugs/wrong-answers/stale-naf-fork.ein` | `solve --maximal` |
| `activator-all-int` | `examples/ein-bugs/activator-all-int.ein` | `solve` |
| `activator-all-nested` | `examples/ein-bugs/activator-all-nested.ein` | `solve` |
| `forbid-chain-1` | `…/wrong-answers/activator-chain-1.ein` | `solve` |
| `forbid-chain-2` | `…/wrong-answers/activator-chain-2.ein` | `solve` |
| `forbid-chain-node-absent` | `…/wrong-answers/activator-chain-node-absent.ein` | `solve` |
| `forbid-chain-param-absent` | `…/wrong-answers/activator-chain-param-absent.ein` | `solve` |
| `forbid-chain-node-writes` | `…/wrong-answers/activator-chain-node-writes.ein` | `solve` |
| `connex-import` | `…/wrong-answers/rung-connex-import.ein` | `solve` |

In `15689488`, the 11 whose witness is flagged and whose right answer was an answer:

| id | witness | run |
|---|---|---|
| `oblig-stdlib-total-owed` | `…/wrong-answers/oblig-stdlib-total-owed.ein` | `solve --maximal` |
| `activator-chain-no-absent` | `…/wrong-answers/activator-chain-no-absent.ein` | `solve` |
| `core-activator-committed` | `…/wrong-answers/core-activator-committed.ein` | `solve` |
| `core-activator-guard-only` | `…/wrong-answers/core-activator-guard-only.ein` | `solve` |
| `kw-neq-in-absent` | `…/wrong-answers/kw-neq-in-absent.ein` | `solve --minimal` |
| `oblig-derived-activator` | `…/wrong-answers/oblig-derived-activator.ein` | `solve` |
| `past-limit-lines-minimal` | `…/wrong-answers/past-limit-lines.ein` | `solve --minimal` |
| `past-limit-lines-maximal` | `…/wrong-answers/past-limit-lines.ein` | `solve --maximal` |
| `fifo-gap-limit-1-minimal` | `examples/lattice/10_fifo_gap.ein` | `solve --minimal --region-limit 1` |
| `fifo-gap-limit-1-maximal` | `examples/lattice/10_fifo_gap.ein` | `solve --maximal --region-limit 1` |
| `class-label-04-open` | `examples/features/04_open.ein` | `solve --region-limit 12` |

Several of those rows recorded a defect that is not about monotonicity and whose only witness happens to be
flagged (`kw-neq-in-absent`, the parser splitting a name; `class-label-04-open`, a label's column; the
`past-limit-lines` and `fifo-gap-limit-1` pairs, a line mode past the limit; the three core rows). They went by the
ruling's criterion, the witness, and are listed here so that a monotone witness can be written for each where
the defect still wants a row.

**The shape golden**, `ein-render/tests/golden/corpus_shapes.md5`: no line of an answer's kind is banked or
compared for a flagged program (`corpus_ops::Op::is_answer`, `Unit::flagged`, both computed): `read-out` (five
lines a program), `cores` (five), `states`, `region-lattice`, `region-trace`, `oracle`, `explain`,
`explain-noalts`, 16 lines a program. Its other renderings are banked as any program's. A flagged program's
answers are still rendered, and held to the permuted run and to the run at another job count: a run against a run.

**Assertions removed**, by file (each was a literal of what a flagged program answers, or a property read off a
flagged cube):

- ein-cli/tests/cli_semantics.rs `every_member_of_n_has_its_core_past_the_limit_too`: the literal sizes of N of `lattice/06_friends_tables.ein` (68) and `past-limit-lines.ein` (15, twice, and `eager == 15`); the test now holds each run's N to its own surfaces (trace, nodes.json) and the stopped run to the exhausted one.
- ein-cli/tests/test_cli.rs `the_report_row_carries_what_h_leaves_out_where_the_text_prints_it` and ein-cli/tests/read_out.rs `the_universes_scope_is_one_sentence_on_every_surface`: both ran `ein test` on the flagged `tests/region/07_cube.ein` and required its claim to hold (exit 0); the witness of the hrules rung is now the monotone `examples/lattice/13_witness_of_two.ein` (`fed`, no hrule proposes it).
- ein-infer/tests/region.rs:
  - `the_seven_programs_have_the_tables_regions` → `the_six_monotone_programs_have_the_tables_regions`: the row of `naf-upward-closure.ein` (|R| 3, lines, the hole {(p A)}, N).
  - `the_duality_holds_without_holes_and_fails_on_the_hole` → `the_duality_holds_without_holes`: the duality's failure on `naf-upward-closure.ein` (`maximal_from_n` = {(q A)}).
  - `the_sub_cubes_are_the_second_tables`: `naf-upward-closure.ein`'s two sub-cubes.
  - `monotone_programs_show_no_violation_and_the_non_monotone_three_are_flagged`: `lattice/04`'s hole {(seats Ann C1)} and its discharged-not-upward-closed count; the three programs' site kinds stay.
  - `each_cause_has_a_fixture_whose_cube_shows_its_hole` → `each_cause_has_a_fixture_whose_only_site_is_of_its_kind`: the holes of `lattice/08`, `/04`, `/09` and their violation counts; `ACTIVATOR_REFUTES`' hole {(p A)} and `refuted_by`; the site kinds stay.
  - `across_a_hole_the_one_fact_test_misreads_the_lines`: the whole test (`synth/local_not_global.ein`'s holes, maximal line, local lines).
  - `fact_grain_never_calls_a_violated_cube_monotone`: no cube of a program the shipped check flags is read any more (the prototype's verdict against flagged cubes, the corpus's and the six written programs'); the half on monotone programs (a Monotone verdict implies no cube violation) and the give-up check stay.
  - `fact_grain_on_programs_written_to_test_it`: the column *its cube violated* of the six written programs, and the cube it was read from; the static columns stay.
  - `a_monotone_verdict_makes_saturation_a_closure_operator`: the written flagged program `CL_NOT_MONOTONE` and the assertion that `cl` is not monotone on it.
  - `every_synthetic_program_has_the_region_it_was_written_for`: the node-for-node comparison for the synthetic programs written for a non-monotone region (`local_not_global`, `maximal_not_top`, `three_components`, `trigger_hole`); their static verdict against the target stays.
  - `an_untainted_death_is_dead_in_every_superset` with its helper `deaths`: the whole test, a property of flagged cubes (no untainted dead node has an alive superset), and `ACTIVATOR_UNDER_ABSENT`'s dead hole.
  - `a_dead_nodes_core_follows_an_activator_only_a_node_derives` → `a_dead_nodes_core_follows_no_activator_of_roots`: the cores of nine flagged programs (`activator-chain-no-absent`, `core-activator-committed`, `core-activator-guard-only`, `activator-chain-0`, `-1`, `-2`, `-node-absent`, `-param-absent`, `-node-writes`); the two monotone rows stay.
- ein-infer/tests/detector.rs (the detector's dynamic half on flagged programs, three tests removed whole, with the helpers `warned` and `stale` and the written program `ROOT_DEAD_PAST_THE_LIMIT`):
  - `the_stale_check_reads_every_node_the_run_computes`: the stale facts of `stale-naf-fork`, `features/07`, `lattice/10_fifo_gap`, none on `naf-upward-closure` and `lattice/08`, and the early question's node count.
  - `the_status_names_the_sites_its_violations_are_attributed_to`: the status (exercised by which rules, with the counts of holes and violations; conservative; not known) of the flagged programs it names (fifteen mentions of corpus files) and of a written one.
  - `the_spot_check_reads_three_parents_of_each_printed_state`: the parent-path counts and the differing paths of nine flagged programs.
- ein-cli/tests/region_dump.rs:
  - `region_states_keeps_every_nodes_state`: `lattice/04`'s node {(seats Ann C1)} (its state, `owing`, `hole`); the fixture is now the monotone `lattice/05_owed_seats_bounded.ein`.
  - `the_warning_says_what_is_assumed_where_there_is_a_site`: the status bit of `lattice/08` (exercised), `features/07` (conservative), `lattice/04` (exercised); the read-out's bit is now held to the summary's.
- ein-cli/tests/region_modes.rs:
  - `where_empty_is_in_r_every_minimal_question_stops_at_root`: the whole test, on the flagged `past-limit-lines.ein` (∅ in R, the minimal line of one state, the verdict `Solution`, the one state listed). The ∅ rule on flagged programs stays held against the oracle by `ein-infer/tests/explore.rs`.
  - `the_minimal_line_row_names_empty_and_no_clause`: the two cells of `past-limit-lines.ein` (`--region-limit 1`, `2`); the monotone programs stay.
- ein-cli/tests/region_render.rs `the_hole_is_drawn_and_an_exclusion_the_cube_refutes_is_named`: the literals of `naf-nowb.ein`'s cube ({(p A)} the hole, {(p A), (q A)} above it, the repair edge `+(q A)`, `state 1`, 4 of 4 nodes); the drawing is now held to the dump it draws, whichever node the dump marks a dead hole.
- ein-render/tests/presentation_semantics.rs `a_dead_hole_is_read_out_as_dead_with_its_core`: the literal rows of `naf-upward-closure.ein` (the hole {(p A)} and its core, `exercised, by bad`, the counts of holes and dead nodes); the rows' shapes stay.
- ein-infer/tests/pair_layer.rs `a_flagged_program_gets_no_licence`: that `cul-de-sac.ein`'s layer finds an implication; that it is flagged, complete, saturates every pair and names no licence stays.
- ein-infer/tests/corpus_cores.rs, both sweeps (`every_core_names_its_commitment_and_derives_its_contradiction`, `no_core_is_printed_unqualified_from_a_cut_search`): a program the static check flags is read by neither any more (computed, `flagged`); the floor *one member of N past the limit* went with it, since only a flagged program stops at the limit.
- ein-corpus/src/regressions.rs `a_line_past_the_limit_counts_what_it_does_not_list`: read its two rows from corpus/regressions.toml (`past-limit-lines-minimal`, `-maximal`, removed); it now reads two rows written for the format in the test.
- ein-cli/tests/answer_dump.rs `every_recorded_state_loads_to_its_own_state`: the floor *a recorded run is a heavy_dump entry's* (the removed row `class-label-04-open` was the one; every heavy_dump entry is a flagged program).
- ein-infer/tests/later_debts.rs `r_over_the_readings_universe_is_the_oracles`: R's members written for each probe are held only where the static check calls the probe monotone (computed); a probe in which a commitment derives an activator is flagged. Describe against the cube and `--single` against the cube stay on every probe.

**Register rows removed**, `plans/wrong_answers/register.toml`, 6 of 16 (10 left); the witnesses stay under
`plans/wrong_answers/witnesses/` and in the corpus:

| row | witness | owner |
|---|---|---|
| `W8/features-07` | `examples/features/07_unstratifiable.ein` and `f07-fpoff.ein` | S1n.2.1 |
| `W8/f07-rules-swapped` | `examples/ein-bugs/wrong-answers/f07-rules-swapped.ein` | S1n.2.1 |
| `W8/fifo-gap` | `examples/lattice/10_fifo_gap.ein` | S1n.2.1 |
| `W8/corner-house` | `examples/saturation/square-unique/corner-house.ein` | S1n.2.1 |
| `W8/fifo-gap-root` | `plans/wrong_answers/witnesses/fifo-gap-root.ein` and its two twins | S1n.2.1 |
| `W25/guard-only-second` | `plans/wrong_answers/witnesses/guard-only-second.ein` and `guard-only-first.ein` | F22 |

Two of these want a second look. **The witness of `W8/fifo-gap-root` and its two twins are not flagged.** Each
states `(solve :no-search)` and has no candidate, and the static check reads what a commitment can extend, so it
finds no site (`ein solve FILE --monotone`: *yes, no site*). What the row recorded is at root: `r2` reads
`(absent (q ?x))` and fires before `r1` derives `q`, so root's fixpoint depends on the agenda's order. The row was
removed because the stage's instruction names the `W8` rows as the non-monotone programs' rows; but **no surface
warns of this program**, the loud warning being the static check's, so it is answered as a monotone program is and
nothing now holds or tracks its answer but this record and M1n. Whether such a program is *non-monotone* in the
ruling's sense, and so wants the warning (a detection the engine does not have), or is a monotone program with an
open wrong answer, and so wants its row back, is the user's; the row is in
`git show 15689488^:plans/wrong_answers/register.toml`. **`W25/guard-only-second`** is not a `W8` row and its owner
is F22; it was removed because its witness and its twin are flagged (an activator site).
`python3 utils/wrong_answers.py` passes on the ten.

**Other readers of a flagged answer, changed**: `utils/check_region_routes.py` holds no flagged program to a
second route (the list of holes is gone; 8 programs), and `utils/region_synth.py --check` compares nodes only of a
monotone program.

**What stays on a flagged program**, since it states no answer: it loads and every command exits 0
(`corpus_exits.txt`); the static check's verdict, sites and kinds (`region.rs`, `detector.rs`'s static half,
`non_monotone_loads.rs`); the route, the cube with no licence; the warning (`non_monotone_warning.rs`); and every
comparison of a run with a run: every mode against the oracle's cube (`explore.rs`' `every_mode_is_the_oracle`),
id order, job count, levers, import form, renaming, the answer dump's round trip, describe against `--count`.
**No such comparison failed on a program that newly loads**, and none was excluded from one.

### The old binary against the new

`ein solve FILE --json-summary F` by the binary of the stage's parent (`d9bdc5a2`) and by the stage's last, over
the 920 `.ein` files under `examples/`, `tests/` and `stdlib/`, comparing exit code, stdout, stderr and the
summary's `region`, `queries`, `root`, `verdict`, `claim` and `monotonicity` (the new key `monotonicity.warning`
read apart: `null` exactly on the unflagged):

| programs | how many | what differs |
|---|---|---|
| load, unflagged | 645 | nothing: the same exit code, stdout, stderr and summary, `monotonicity.warning` `null` |
| not answered by `ein solve` (a load error, a module, a fixture that is no program) | 228 | nothing |
| flagged, answered before | 27 | the `WARNING` row on stdout and its line on stderr; nothing of the summary but the new key |
| flagged, refused before | 20 | answered: exit 1 to 0, a read-out and a summary where there was the refusal's line |

After task 1 alone (the binary of `1f063ed9`) the same comparison gave no difference on the 645 and on those not
answered, the 27 unchanged and the 20 answered. The commands, from the scratch directory of the session:
`python3 cmp2.py ein-old ein-final all3.txt out.json` (old against new, ten at a time) and
`python3 agree.py ein-final flagged3.txt` (one R on every command); both are a dozen lines around
`ein solve FILE --json-summary F`, `--count`, `--single` and `ein oracle FILE --dump D`.

### Goldens that moved

- **`ein-render/tests/golden/corpus_shapes.md5`**, re-banked with `EIN_BLESS=1 EIN_CORPUS_SLOW=1` on
  `id_order_invariance`, every corpus program, 30 227 lines before the stage and 29 475 after:
  - `1f063ed9`: 301 lines moved, none added or removed. 280 of the 20: the refusal's line gone from `cores`,
    `read-out`, `oracle`, `region-lattice`, `region-trace` and `states`. 12 of `activator-chain-no-absent.ein`, its
    path in `cores`, `read-out`, `oracle` and `region-lattice`. 9 `naf` lines whose warning lost its remedy, 8 of
    the 20 and `examples/syntax/rule-forall-and-not.ein`. No `load`, `ir`, `saturate`, `plan`, `match`, `explain`
    or drawing moved.
  - `15689488`: 776 lines removed, 24 added, none moved. 752 are the 16 answer lines of each of the 47; the other
    24 are the lines of `tests/region/07_cube.ein` that are no answer, added again under
    `examples/lattice/17_flagged_region_claim.ein`.
  - `1b9f87f0`: 646 lines moved, none added or removed: the `oracle` line of every program that has one, each one
    line longer, `"warning": null` in the cube's `monotonicity` block. No flagged program has such a line.
- **`ein-cli/tests/golden/corpus_exits.txt`**: `1f063ed9`, 26 cells from 1 to 0, the 20's declared runs, and 8
  under the witnesses' new names; `15689488`, five lines out (the `solve` runs of the two `oblig-stdlib-total-owed`
  programs, the `test` run of the `-declared` one, `tests/region/07_cube.ein`'s `saturate` and `test`) and four in
  (their `solve --count`, and the moved program's `saturate` and `solve --count`).
- **`ein-ir/tests/golden/grammar_table.txt`**: one line, the retired flag `warn-derived-naf`'s note (`1f063ed9`).
- **`docs/ein/user/04-reference/09-read-out-examples.md`**, written by the binary: `1b9f87f0`, 46 lines added, the
  row and the stderr line on ten read-outs of five flagged programs
  (`lattice/04_obligation_trigger_hole.ein`, `lattice/06_friends_tables.ein`, `features/07_unstratifiable.ein`,
  `features/01_not_and_absent.ein`, `ein-bugs/wrong-answers/oblig-derived-activator.ein`).
- **`tests/mutation_survivors.txt`**: two lines added in `1f063ed9`
  (`bijection.ein::bijective-setup::swap-assert::(surjective ?R ?isa)` and `…(total ?R ?isa)`); the whole census
  after `15689488`, 518 of 637 killed, its 119 survivors the banked set, so the program that left `tests/` was no
  mutant's only killer.
- **`stdlib/MANIFEST.sha256`**: `stdlib/contradiction.ein`'s comment (`1f063ed9`). The search design page's
  diagram, drawn again in the same commit.

### Checks

On `1b9f87f0`, unless a commit is named:

| check | result |
|---|---|
| `cargo test --manifest-path ein.rs/Cargo.toml --profile gate --workspace --no-fail-fast` | 166 test runs, three failures, each the warning's and fixed in the commit: two tests that found `--count`'s `asked` row by position (`lever_invariance.rs`, `read_out.rs`), and the shape golden's `oracle` lines. Each was run again and passes |
| the same on the tree of `15689488` | 165 test runs, one failure, `answer_dump`'s floor on a `heavy_dump` regression row, fixed in that commit and run again |
| `EIN_CORPUS_SLOW=1` on `id_order_invariance`, `jobs_invariance`, `renaming_invariance`, `lever_invariance`, `answer_round_trip`, `corpus_cli`, `answer_dump` | pass |
| `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `RUSTDOCFLAGS="-D warnings" cargo doc -q --workspace --no-deps` | failed on `1f063ed9`'s `extendable.rs`, whose module page linked a crate-private function; fixed in `1b9f87f0`, passes |
| `utils/doc_audit.py --links --check --tracked` | no finding in the project; **10 in plan files**, nine in files this stage may not edit (below), the tenth fixed in this commit |
| `utils/check_release_voice.py`, `check_doc_blocks.py`, `render_diagrams.py --check`, `stdlib_manifest.py`, `check_tool_mirrors.py`, `check_gate_steps.py`, `check_hashmap_iteration.py` | pass |
| `utils/check_generated.py --bin ein.rs/target/gate/ein` | 9 generators, every file as written |
| `utils/wrong_answers.py` | 10 open rows, each still answered as recorded |
| `utils/check_region_routes.py --bin ein.rs/target/gate/ein` | pass, 8 programs and 300 random regions |
| `utils/stdlib_mutants.py --bin ein.rs/target/gate/ein --check`, the whole census, on `15689488` | 518 of 637 killed; the survivors are the banked 119 |
| `utils/review_bundle.py --smoke`; `cargo bench --bench engine -- --test` | pass |
| `ein test tests/` | 296 files, 553 expectations: 551 held, 0 failed, 2 not checked; no warning line, no program under `tests/` being flagged |
| the calculator's batch (`arith_batch`) | not run: no rule of the calculator changed |
| `./run_tests.sh`, `./run_tests.sh --nightly`, `utils/ci_container.sh` | not run, as told |

### Not done, and found outside the scope

- **`ein test` is not changed** (Reading 2). Whether a claim of a flagged program is *not checked* is the user's.
- **`ein render h-space` writes no stderr line.** Its read-out already says, in `domain` rows, that the static
  check flags the program and what its statements then cover. It was left as it is; the warning's stderr line can
  be added as `ein show hyps` has it if the user wants every surface alike.
- **`09-read-out-examples.md` shows ten read-outs of five flagged programs**, written by the binary and held to
  it by `utils/check_generated.py` and `read_out_reference.rs`. The page is where the rows of the warning are
  shown, and a row that only a flagged program prints has no other example; it is a golden of what those runs
  print, and it was kept. It says nowhere that an answer is right.
- **A program written inline in a test** was read where a test named a flagged corpus file, and where a test
  failed once the refusal went; the inline programs were not swept one by one for being flagged. A sweep over
  the corpus computes the flag, so it has no such gap.
- **The headers of the other flagged examples** say what is dead, alive or owed under the definition, which is
  how each explains why it is not monotone; none says an answer is right. The one that stated counts of its R,
  `examples/lattice/06_friends_tables.ein`, no longer does. The four programs `utils/region_synth.py` writes for a
  non-monotone region keep their *target* in the header, the region each was written for; nothing holds their
  nodes to it any more.
- **Nine links in plan files this stage may not edit point at what it removed**, so `utils/doc_audit.py --links
  --check --tracked`, a gate step, fails until they are re-aimed: the decision `Q-M1e2.5`'s file
  (`plans/m1f_hypothesis_and_documentation/README.md:25`, `open_questions.md:10508` and `:10712`,
  `p1f.21_verdicts_as_rules/design.md:363` and `:455`, `p1f.21_verdicts_as_rules/findings.md:38`), the
  diagnostics' old § 2.7 (`plans/m1g_logic/p1g.4_first_order_logic/s1g.4.1_the_fragment.md:40`,
  `s1g.4.2_relation_algebra.md:66`), and the removed test
  (`plans/m1n_non_monotone_programs/f21_non_monotone_programs_decided.md:532`). The tenth, in this file, is fixed
  here: the citation of Q-M1e2.5 in § The rulings is its id, the file being gone.
- **The process's own pages that still speak of the refusal**: `AGENTS.md`'s pointer to this stage (the
  coordinator's line); `utils/censuses/hole-census.md`, `node-census.md` and `taint-census.md`, measurements taken
  while the refusal stood; `ein-infer/examples/hole_census.rs` and `node_census.rs` still draft an option *refuse*
  among M1e2's alternatives, which is M1n's material; the plans' own tables.
- **For M1n**: the 27 regression rows and the 6 register rows above are its material, and
  `f21_non_monotone_programs_decided.md` and S1n.2.1 still speak of the register's `W8` rows. The one finding:
  **a program with no candidate is never flagged**, whatever its rules read under `absent`
  (`plans/wrong_answers/witnesses/fifo-gap-root.ein`), so the loud warning does not reach a program whose root
  fixpoint depends on the agenda's order.
- **Not run**: `./run_tests.sh`, `./run_tests.sh --nightly`, `utils/ci_container.sh`, as the stage was told. The
  plan asks the gate and the nightly on the stage's last commit; they are the coordinator's.

### The coordinator's part

- **The nine links** are re-aimed: the six to the removed decision's file now say that Q-M1e2.5 is cancelled and
  link Q-M1f.564's; M1g's two lines say what is true now, that such a program loads flagged; M1n's names the
  removed test in words.
- **`W8/fifo-gap-root` is back in the register**, by default and for the user to overrule
  ([V-22](findings.md#v-22--a-program-with-no-candidate-is-never-flagged)): its program is not flagged, so the
  engine claims its answer, and the register's rule is that a known wrong answer still open is a row. The register
  has 11 rows. The other five that went stay out; M1n's page and F22's say where each is.
- **The findings** of § *Not done, and found outside the scope* are V-22 to V-27 of the phase's page.
- **The gate and the nightly, in the runner's image**, on the batch's last commits, after `std.debts` was merged
  (Q-M1f.560, ruled the same day): `utils/ci_container.sh` on `edbe6c3b`, the last commit that changes code,
  **exit 0**; `utils/ci_container.sh --nightly` on `c125113f`, the same code with the merge's record, **exit 0**.
  The logs are `build/ci-logs/gate-edbe6c3b.log` and `build/ci-logs/nightly-c125113f.log`, on this machine. No
  second run was made outside the container.

