# S1f.6.25 — The findings fixed in this phase: E-5, E-8, E-11, E-20, E-22

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12. Not started. **An engine stage,** a task a finding.
**Depends on:** nothing in the phase. Independent of the stdlib layout. E-27, the sixth finding the same
ruling makes this phase's, is built with the detection rules
([S1f.6.12's build](s1f.6.12_malformed_numbers.md#the-build-as-ruled-2026-11-12), T1f.6.12.7).

## The ruling

[Section J of `questions.md`](questions.md#j-findings-that-need-an-owner) asked, for each finding with no
owner, one of *(a) fix in this phase; (b) a stage of a named later phase; (c) a followup; (d) leave as it
is*, and said: *One line a row; a line left empty takes the recommendation.* The user left every line empty
(2026-11-12). [Round 2, § Left empty](questions_round2.md#empty) reads that as the file said it would:

> That makes these **fixes in this phase**, each a stage: the trace's *Derives* line (58, E-5); a qualified
> import bringing the module's queries and config (59, E-8); the dumped program dropping a rule only a
> selection names (61, E-11); a flat import that leaves the lower theory's rules behind, said in one line
> (66, E-20); `ein solve` exiting 1 with no read-out past 2¹²⁸ commitments (68, E-22), filed as a register
> row first since an answer that exits non-zero is against your rule; and a second measure for guards (71,
> E-27), with the detection rules.

and asks: *If you did not mean to take them all, name the rows to drop.* The user left that reply empty too
(`dc88520b`), and said in chat on 2026-11-12: *"if something remain - pick the default and continue current
phase"*. **So each task below stands on the default, by that instruction,** and all five are built.

Each finding is read from its row of [`findings.md`](findings.md), which has the witness and the command.

## Tasks

### T1f.6.25.1 — E-5: `--trace`'s *Derives* line names every conclusion of a firing

Today the line names only a firing's first conclusion; a second is stored and not named (the witness: a rule
concluding `(division-by-zero …)` and `(false)`, row 58). The fix is in the trace's writer: the line lists
every conclusion the firing stored, in the order the rule writes them.

- Read first which surfaces share the line (`--trace`, the answer dump's state files, the review bundle's
  case pages) so all say the same.
- No answer moves: R, N, H, claims and exit codes are untouched. The goldens that hold trace text move for
  every corpus rule with more than one conclusion; **they are counted and named in the record before
  blessing**.
- The natural-language reading of a fact (`09_natural_language.md`) takes a fact's first recorded
  justification and is not changed.
- If the review bundle's writer changes, `utils/ci_container.sh` is part of done.

### T1f.6.25.2 — E-8: a qualified import brings no `(query …)`, `(solve …)` or `(config …)`

Today a bare or `:as` import keeps every form of the module, so the importer is asked the module's queries,
solved under the module's lattice and configured by the module's flags; two imports of a module that holds a
`(solve …)` are refused, *2 (solve …) forms — a program states its lattice once* (row 59). A flat import
brings none of the three.

- **A register row first.** The importer's R is computed under a lattice it did not state, and it answers a
  query it did not ask: an answer that is false of the program as written. Row 59 says *with a register row
  first if you call it a wrong answer*, and the line was left empty; **the default taken here is that it is
  one,** filed `open` with witnesses under `examples/ein-bugs/wrong-answers/` (a module with each of the
  three forms imported `:as`, and imported twice), `now` today's answer and `right` the importer's own.
- The fix, in `ein_ir::imports`: the three forms are a *program's*, and an import of any kind brings a
  module's declarations and facts and none of them.
- **The census before the fix:** every tracked module and every inline one in the Rust suites that is
  imported qualified or bare and holds one of the three forms. What would move is shown before it moves.
- The pin in `ein-ir/tests/imports_semantics.rs`; [`01_grammar.md` § Import](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#import)
  and `defined_behaviour.md` § 4.25 say it; the rows flip to `fixed` in the fixing commit.

### T1f.6.25.3 — E-11: the program a `--dump-answer` state file re-writes keeps a rule a selection names

Today the re-written program goes through the tree-shake, which keeps an imported declaration that a live
name reaches or a live premise activates; a rule with an empty `:match` that the importer lists in
`:symbols` is reached by neither, fired at root in the run, and is not in the re-written program (row 61).
`std.arith` works around it with one fact, `(arith-rule arith)`, and every module since has copied the
workaround.

- The fix, in `ein_ir::resolve_and_minimize` or its caller: a name an import's selection lists is live.
  It is the same function the inlining route of `stdlib/README.md` documents, so that route is covered too.
- **Is it a wrong answer?** The dumped file reloads to the state's facts but not to its program; whether
  the register takes it is decided by what `ein-cli/tests/answer_round_trip.rs` holds today. If a program
  can be written whose reloaded dump answers differently from the run that wrote it, that program is a
  register row, filed `open` first. T1 of this task tries to write one.
- `answer_round_trip.rs` gains the witness without the workaround: a library whose entry is a rule with an
  empty `:match`, imported by selection.
- **The workaround facts stay.** `(arith-rule arith)`, `(int-rule int-arith)` and their siblings are not
  removed here: removing them moves every state of every arithmetic program, for no answer. The module
  headers say the fact is no longer needed, and the layout move may drop them if the user wants.

### T1f.6.25.4 — E-20: one line where a selection brings a relation's declaration and no rule of its module

Today a flat import of a theory built on another theory leaves the lower theory's rules behind in silence:
the closure reaches the lower theory's relation declarations and not its entry's name. The symptom is far
from the cause (no request answered, the lower theory's relations open, 660 candidates, and E-22's exit).
It was met three times (row 66). The rule itself is as [`01_grammar.md` § Import](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#import)
states it and **is not changed**: a selection is a selection.

- **A diagnostic, not a refusal.** One line on stderr at load, naming the importing form, the module, the
  relation whose declaration came, and that no rule of that module which derives it was selected. Exit codes
  are unchanged; the program loads and answers as today.
- **Shown on real programs first.** The exact condition decides how noisy the line is. A library of facts
  has no rule to leave behind (`(import std.digits :symbols (fa nz __closed__))` must print nothing), and a
  program may select a relation on purpose. The task takes the census of every import in the corpus, the
  tests and the Rust suites under two or three candidate conditions (*the module has a rule that concludes
  the relation and none is kept*; *the module has an entry rule and it is not kept*), and the record shows
  which lines each would print. The condition is chosen on that table; if no condition is quiet on the
  corpus and loud on the three witnesses, the task stops and shows the user.
- Where it prints is the loader's, so every command that loads a program prints it once. `--json-summary`
  carries it if the summary already carries load diagnostics; otherwise that is a finding.
- `ein-cli/tests/corpus_cli.rs`'s rule that a refusal carries a diagnostic is unaffected; the line is
  pinned by a test of its own with the witness `bob1_nested_import.ein` rebuilt under `tests/`.

### T1f.6.25.5 — E-22: a count past 2¹²⁸ is an answer, exits 0, and says what the run established

Today, where one state is reached by more than 2¹²⁸ commitments, a bare `ein solve` prints one line to
stderr, *a state's commitments pass 2^128: the state route cannot count them*, nothing to stdout, and exits
1, though no claim was refuted and the program is not in error; `--minimal` answers at once (row 68). The
user's rule of 2026-10-16 is that every answer exits 0 and a run that stops says what it established.

- **A register row first,** as round 2 says: the witness `only_conv.ein` of `build/s1f.6.16/out/witness_2p128/`
  rebuilt under `examples/ein-bugs/wrong-answers/` at the smallest size that passes 2¹²⁸, `now` exit 1 with
  one stderr line and no read-out, `right` exit 0 with a read-out. If the register's schema cannot record a
  run's exit with an empty stdout, the schema moves first, as it has before.
- **What the read-out says** is the design, shown on the witness before it is built: the states R has
  (k_s is known to the state route), and for the count of commitments a value that says *more than 2¹²⁸*,
  or the count in a wider integer. Which of the two is a question under the next free id; the default is the
  first, since nothing reads a count that large as a number. `--count` and `--json-summary` say the same
  thing in their forms; `--count` on the witness had printed nothing when a 20 s limit ended it, which the
  task measures and reports and does not promise to cure.
- [`read_out.md`](../../../docs/kernel/inference/read_out.md), `ein_render::read_out::Row` and
  `ein-cli/tests/read_out_reference.rs` move in the same commit as the printer; `lever_invariance` holds the
  new read-out under every lever; the row flips to `fixed` in the fixing commit.

## Acceptance

- Each of the five findings is closed in [`findings.md`](findings.md) with what was done and its commit, or
  the user dropped its row and the task says so.
- E-8's and E-22's register rows are fixed, and any row T1f.6.25.3 filed.
- No answer of an existing corpus program moved without a register row; every golden re-banked was named
  before blessing.
- The kernel pages that describe a changed behaviour changed in the same commit.

## What it must not do

- **Not change the closure rule of a selection** (E-20 is a line, not a new semantics; the semantics of
  imports is [S1f.6.30](s1f.6.30_name_clashes_between_imported_theories.md)'s to propose).
- **Not remove a module's workaround facts.**
- **Not take the findings the table gave to other owners:** E-1 and E-7 (P1f.10), E-3, E-12, E-15, E-24,
  E-25 and E-26 (M1g's S1g.6.2), E-9, E-10 and E-13 (M1g's S1g.6.3), E-21 (F23), E-16 (Q-M1f.8), E-18 and
  E-19 ([S1f.6.26](s1f.6.26_what_root_owes.md)), E-30 and E-31
  ([S1f.6.27](s1f.6.27_ein_test_by_the_cheapest_route.md)).

## The standing rules

A wrong answer is a row in `corpus/wrong_answers.toml`, filed `open` before its fix. New findings go to
[`findings.md`](findings.md) under the next free id; questions to [`open_questions.md`](../open_questions.md)
under the next free id. A change to what a surface prints shows real computed examples first. Config never
changes an answer; exit non-zero only on error. The gate is `./run_tests.sh`, then `./run_tests.sh --nightly`
after each code commit; `utils/ci_container.sh` only if a DOT writer or the review bundle changes (T1 may
touch the bundle's writer). The five tasks may be five commits, each green.

## Record, 2026-11-13

Each task is a build commit of its own, so that one can be reverted without the others; a register row is
filed `open` in a commit before its fix. The baseline every sweep compares with is the release binary and
the stdlib at `5e5685b7`, kept as `build/s1f.6.25/ein-old` and `build/s1f.6.25/stdlib-old`, and the sweep is
`python3 build/s1f.6.25/sweep.py --diff`: `ein saturate --dump` and `ein solve` over every `.ein` file
`5e5685b7` tracks, stdout, stderr and exit, times masked.

### T1f.6.25.1 — E-5, done

**The surfaces, read first.** A firing's conclusions are named in three places.

| surface | what it printed | after |
|---|---|---|
| `--trace`'s step, `ein_render::trace::region::derives_line` | *Derives `c1`*, the firing's first conclusion and its statement | every conclusion, in the order the rule writes them, `; ` between two, each with its own statement |
| `--trace --reorder`'s step, the same module's `steps` | `` `rule` → `c1` `` | `` `rule` → `c1`, `c2`, … `` |
| a `--dump-answer` state file | a line a fact, each with its own `; rule r: <reason> — <statement>` comment | unchanged: every conclusion is a fact of the state and always had its line |

The review bundle's case pages carry the trace's text as written, so they say what the trace says; the
bundle's writer is not changed, and `utils/ci_container.sh` is not part of this task's done. One place in
it still reads the first conclusion alone, E-40 below.

**The fix** (`ein-render`'s `trace/ast.rs`, `linearize.rs`, `region.rs`). A `TraceStep` carries its firing's
other conclusions, `more`, each with its statement. A firing's `derived` already held every conclusion,
new or already known, in the rule's order (`ein_infer::firing::fire`), so nothing in the engine moved. A
step's section under `--reorder` is still its first conclusion's entity. The natural-language reading of a
fact is not changed. The form of the line is a default,
[Q-M1f.214](../open_questions.md#q-m1f214--the-derives-line-of-a-firing-with-several-conclusions).

**Before and after**, on `build/s1f.6.25/p/e5.ein`, a rule `two` with `:assert (and (q ?x) (flag ?x) (not (r
?x b)) (r ?x ?x))` beside `(p a)` (`ein solve FILE --trace T.md`, the binary at `5e5685b7` and the tree's,
2026-11-13):

```text
before   Derives `q(a)` — a is q
after    Derives `q(a)` — a is q; `flag(a)`; `not(r(a, b))`; `r(a, a)` — a is related to a
before   **Step 1** · `two` → `q(a)` — a is q and related to itself          (--reorder)
after    **Step 1** · `two` → `q(a)`, `flag(a)`, `not(r(a, b))`, `r(a, a)` — a is q and related to itself
```

**What moved, named before blessing.** `cargo test --workspace --no-fail-fast` on the fix
(`build/s1f.6.25/ws-t1.log`, 2026-11-13) failed two tests and no other:

- `ein-render/tests/golden/corpus_shapes.md5`: 173 `region-trace` digests of 23 765 renderings, and no
  digest of another kind (`build/s1f.6.25/t1-moved.txt`). They are the programs in which a rule of several
  conclusions fires: 97 under `tests/stdlib/`, 16 under `examples/word_problems/`, 15 `binary_logic`, 13
  `arith`, 9 `beyond_arith`, 7 `binary_arithmetic`, the six zebra programs (the stdlib's setup fan-outs
  conclude four facts a firing), 3 `domain_elim`, 2 `ein-bugs`, 1 `features` and four stdlib modules. No
  line count moved: a step keeps its one *Derives* line.
- `ein-render/tests/golden/trace_unconditional.md`, one line: *Derives `functional(color-loc)`.* reads
  *Derives `functional(color-loc)`; `injective(color-loc)`; `total(color-loc)`; `surjective(color-loc)`.*

**The pin**: `ein-cli/tests/region_modes.rs`'s `a_trace_step_names_every_conclusion_of_its_firing`, both
forms, and a rule of one conclusion printing the line it always did.

**The sweep** moved no answer: of 837 files one differs, `examples/broken/load/import_module_not_found.ein`,
by the path in its refusal, the old stdlib's directory, as in every sweep that points the old binary at a
copy (`build/s1f.6.25/sweep-t1.txt`, 2026-11-13). R, N, H, claims and exit codes are untouched; the zebra
programs' answers did not move, and their `--trace` files name three conclusions more a setup step.

**Page**: `09_natural_language.md`'s table row for a relation's `:why`.

### T1f.6.25.2 — E-8, done

**The census before the fix** (`python3 build/s1f.6.25/e8_census.py`, 2026-11-13, a text scan of every tracked
`.ein`): 837 files, 371 imports, **8 of them bare or `:as`**. Three are refusal fixtures of a module that
does not exist (`examples/broken/load/import_bare_std`, `import_module_not_found`,
`import_relative_not_found`); the other five import a module that holds no `(query …)`, `(solve …)`,
`(config …)` or `(trace …)`. So no tracked program's answer could move. In the Rust suites one test pinned
the old behaviour, `imports_semantics.rs`'s `what_each_import_form_brings_of_a_module`, whose first case
listed the module's query, solve form and config among what a qualified import brings; it is restated. No
other test moved (the tests named below, and the gate).

**The rows, filed `open` first** (`4faf47dc`), on the release binary at `5e5685b7`: W90, two rows and the
module they import, `examples/ein-bugs/wrong-answers/import-directives-module.ein`, which declares `is-a`,
states `(is-a Tea Drink)`, and holds `(solve :hypothesis-relations ())`, `(config
:record-alternative-justifications false)` and `(query :goal (is-a ?x Drink))`.

| witness | before | after |
|---|---|---|
| `import-directives-as.ein`: `(import … :as m)`, a relation `chosen` of its own, no lattice, flag or query | R {∅}, one state, no candidate, the module's lattice; the read-out answers `(m.is-a ?x Drink)`, *Tea is a drink*; `--dump-config` shows the module's flag | R four states, the cube over `(chosen Drink)` and `(chosen Tea)`; no query; the default configuration |
| `import-directives-twice.ein`: the same module `:as m` and `:as k` | `kb load error: 2 (solve …) forms — a program states its lattice once`, exit 1 | loads; R four states |

That it is a wrong answer, and that a module's `(trace …)` goes the way of the three, are defaults,
[Q-M1f.215](../open_questions.md#q-m1f215--a-qualified-import-and-what-a-module-states-as-a-program).

**The fix** (`ein_ir::imports`' `resolve`): where a module is imported bare or `:as`, its forms headed
`query`, `solve`, `config` and `trace` are dropped before the rest are qualified. A flat import never
brought one (`select`). The importer's own are untouched, and a module imported through a module loses
them at the inner import.

**The pins**: `imports_semantics.rs`'s `no_import_brings_a_modules_query_solve_or_config` (the three import
forms, the module twice under two names, a module through a module, the importer's own three kept, and the
loaded program's queries, lattice and configuration), and the restated table test.

**The register**: W90's two rows flipped to `fixed`, `[case.fixed]` the null id in the fixing commit and
the hash in the stage's records commit: 314 fixed, 16 open and 1 not wrong (`grep '^status'
corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-13).

**What moved, named before blessing**: `corpus_exits.txt`, `import-directives-twice`'s two cells from exit 1
to 0 and one new cell, its `saturate --dump`; `corpus_shapes.md5`, the two witnesses' digests and no other
file's. **The sweep** (`build/s1f.6.25/sweep-t2.txt`, 2026-11-13) moved no answer of a file `5e5685b7` tracks: the
refusal's path T1 names, and two `saturate:` timing lines whose column grew by a digit, which the mask
let through and has covered since. The witnesses are new files.

**Pages**: `01_grammar.md` § Import, `defined_behaviour.md` § 4.25 (the table's last row and a bullet),
`AGENTS.md`'s standing rule on imports, `examples/README.md`, `imports.rs`'s module header.
