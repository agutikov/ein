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
