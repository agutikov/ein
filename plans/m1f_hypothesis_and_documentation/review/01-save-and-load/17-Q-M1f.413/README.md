# Q-M1f.413 — `ein kb save` of a saved file

## 1. The question

`ein kb save FILE OUT.einb` writes a `.einb`, the binary cache of a program: the program resolved and with its imports
flattened, and root's sections (root's fixpoint). A **saved file** (what `ein solve --dump-answer` now writes per
state) is a program, every module it imports, and at most one `(commit …)`, the choices of H that make the state. A
container has no place for a commitment. So what should `kb save` do when it is given a saved file? Today it stores a
saved file **of root** (no `(commit …)`) as the program it carries, its imports read from the modules the file
carries and never from the disk, and **refuses** a saved file that has a `(commit …)`. The alternatives are to refuse
both, or to give the container a section for the commitment so that it opens as that node.

Recorded in [`../../open_questions.md` § Q-M1f.413](../../../open_questions.md#q-m1f413--ein-kb-save-of-a-saved-file).
The shipped pages: [`10-answer-dump.md` § A saved file is a node of its program's lattice](../../../../../docs/ein/user/04-reference/10-answer-dump.md#a-saved-file-is-a-node-of-its-programs-lattice)
(last bullet) and [`07-diagnostics.md` § 2.30](../../../../../docs/ein/user/04-reference/07-diagnostics.md#230-a-saved-file-is-its-program-its-modules-and-one-commitment-of-h).

## 2. What is so today

Programs: [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein) (copy of
`tests/saved/02_…`: one item `a`, an hrule that proposes `(note (bad a))`, a rule `flag`; R is two states, root and
`{(note (bad a))}`) and [`programs/03_the_standard_library.ein`](programs/03_the_standard_library.ein) (copy of
`tests/saved/03_…`: imports `std.math.decimal-arith`, five modules carried; R is root alone). Binary:
`build/review-bin/ein-head-17085b27`. Every exit code is in [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt).

**The save.** `ein solve 02 --dump-answer runs/02.dump` writes root's file
[`models/01.ein`](runs/02.dump/models/01.ein) (no `(commit …)`) and the node's file
[`models/02.ein`](runs/02.dump/models/02.ein), which ends:

<!-- output -->
```text
)
(commit
  (note (bad a)))
```

**A saved root file is stored**, exit 0, no output. The container answers as the program does: the read-outs of the
program, of the saved root file and of the container differ only in the path on the first line
([`02.diff.solve.program-vs-root-einb.diff`](runs/02.diff.solve.program-vs-root-einb.diff),
[`02.diff.solve.saved-root-vs-root-einb.diff`](runs/02.diff.solve.saved-root-vs-root-einb.diff)); the summaries differ
only in `source` ([`02.diff.summary.program-vs-root-einb.diff`](runs/02.diff.summary.program-vs-root-einb.diff)); and
`ein saturate --dump` holds the same facts with the same origins, only the input and timing lines moving
([`02.diff.saturate.saved-root-vs-root-einb.diff`](runs/02.diff.saturate.saved-root-vs-root-einb.diff)).

**The container of a saved root file is the program's container, but for the source's digest.** Saved under one input
name (`p.ein`, once the program and once its saved root file,
[`runs/same-name/`](runs/same-name/)), the two `.einb` are the same length and differ in exactly 64 bytes: the
header's digest over the body (bytes 25–56) and the 32-byte BLAKE3 digest of the input file recorded after the name
`p.ein` (bytes 330–361), because the two input files' bytes differ
([`same-name/cmp.txt`](runs/same-name/cmp.txt)). The program text inside is the same.

**A saved node file is refused**, exit 1 ([`02.node.kb-save.stderr.txt`](runs/02.node.kb-save.stderr.txt)):

<!-- output -->
```text
kb save error: runs/02.dump/models/02.ein is a saved file with a (commit …), a node of its program's lattice, and a .einb holds a program's root — save the program, or the saved file of its root
```

What the node file reads as, which no container can hold today: `ein solve runs/02.dump/models/02.ein` describes the
part of R above the node, one state, with a `commit` row and the claims NOT CHECKED
([`02.saved-node.solve.txt`](runs/02.saved-node.solve.txt); diffed against the root container's read-out in
[`02.diff.solve.root-einb-vs-saved-node.diff`](runs/02.diff.solve.root-einb-vs-saved-node.diff)).

**The imports come from the file, not the disk.** Program 03's saved root file is stored, and its container answers
as the program does ([`03.diff.solve.program-vs-root-einb.diff`](runs/03.diff.solve.program-vs-root-einb.diff)). With
`EIN_STDLIB=/nonexistent` the program itself is refused
([`03.no-stdlib.program.kb-save.stderr.txt`](runs/03.no-stdlib.program.kb-save.stderr.txt):
`kb load error: (import std.math.decimal-arith) — $EIN_STDLIB names /nonexistent, which is not a directory (None)`),
and its saved root file is still stored, exit 0, from the five modules it carries, with one `note:` per module
([`03.no-stdlib.root.kb-save.stderr.txt`](runs/03.no-stdlib.root.kb-save.stderr.txt)); that container answers the same
([`03.diff.solve.root-einb-vs-no-stdlib-root-einb.diff`](runs/03.diff.solve.root-einb-vs-no-stdlib-root-einb.diff)).
A saved root file whose carried `std.math.arith` has a comment added
([`03.moved-module.ein`](runs/03.moved-module.ein)) is stored with a `note:` naming it
([`03.moved-module.kb-save.stderr.txt`](runs/03.moved-module.kb-save.stderr.txt)), and its container then answers
with no note ([`03.moved-module-einb.solve.txt`](runs/03.moved-module-einb.solve.txt), no stderr file).

**The drawings.** `ein render program` of the root container draws the program it stores in its three parts, four
graphs ([`renders/02.root-einb.render-program.dot`](renders/02.root-einb.render-program.dot), drawn as
`02.root-einb.render-program.1.svg` … `.4.svg`: the digraph `kb`, the four declarations and `(item a)`; the hrule
`guess`; the rule `flag`; the digraph `query`, the `(query …)` a cluster and the `(solve …)` a note). `ein render kb`
of the root container draws root: the four declarations, the four facts `(relation r)` the loader adds and
`(item a)`, every fact an octagon with no name in it, its arrow `1` at its relation's box
([`renders/02.root-einb.render-kb.svg`](renders/02.root-einb.render-kb.svg)); of the saved node file, the node's
state: root's facts, the committed `(note (bad a))`, dashed, whose arrow `2` is at `(bad a)` in the frame `note ctx`
(not held), and `(flagged a)`, dashed, `by flag` beside it
([`renders/02.saved-node.render-kb.svg`](renders/02.saved-node.render-kb.svg)):

![ein render kb of the saved node file: every fact an octagon with no name in it; the committed note fact, dashed, points at the fact bad a in the frame note ctx; flagged a, dashed, by flag](renders/02.saved-node.render-kb.svg)

**The earlier binary (`ein-a8d595eb`, before save/load was merged).** Its state file is the program reprinted and then
every fact of the state, each with its English in a comment, `ein-answer/2`
([`a8d.02.dump/models/02.ein`](runs/a8d.02.dump/models/02.ein); the change to today's file:
[`diff.state2-file.a8d-vs-today.diff`](runs/diff.state2-file.a8d-vs-today.diff)). That file is an ordinary program,
so its `kb save` stores it, exit 0, and the container answers as the file does
([`a8d.02.diff.solve.state2-vs-state2-einb.diff`](runs/a8d.02.diff.solve.state2-vs-state2-einb.diff): the path
only). What it answers is **the state read as a program**: `(note (bad a))` is a stated fact, the hrule proposes
nothing, R is `{∅}`, and the program's own claims are checked against that and FAIL, exit 1
([`a8d.02.state2-einb.solve.txt`](runs/a8d.02.state2-einb.solve.txt),
[`a8d.02.state2-einb.solve.stderr.txt`](runs/a8d.02.state2-einb.solve.stderr.txt)):

<!-- output -->
```text
runs/a8d.02.state2.einb: query :expect FAILED — the claim lists 2 classes, the query has 1: (model) is no class of all of R
runs/a8d.02.state2.einb: (solve :expect) FAILED — the claim lists 2 states and R has 1
```

## 3. The options

- **(a) as built.** A saved file of root is stored as the program it carries, imports from its carried modules; a
  saved file with a `(commit …)` is refused, exit 1, with the line above. All of § 2 is (a).
- **(b) both refused**: a container is made from a program file only. `kb save runs/02.dump/models/01.ein` would exit
  1 too. Mock: [`02.root.kb-save.MOCK.option-b.stderr.txt`](runs/02.root.kb-save.MOCK.option-b.stderr.txt), a line
  written by hand in the shape of today's refusal (no binary prints it). Nothing else moves: the program is still
  stored from its own file ([`02.program-einb.solve.txt`](runs/02.program-einb.solve.txt)). What is lost is storing a
  program from its saved file where its modules are not on the disk, as with `EIN_STDLIB=/nonexistent` above.
- **(c) the container carries the commitment**, a section of its own, and a container of a node opens as that node.
  `kb save runs/02.dump/models/02.ein runs/02.node.einb` would exit 0
  ([`02.node.kb-save.MOCK.option-c.txt`](runs/02.node.kb-save.MOCK.option-c.txt), written by hand), and
  `ein solve runs/02.node.einb` would print the saved node's read-out:
  [`02.node-einb.solve.MOCK.option-c.txt`](runs/02.node-einb.solve.MOCK.option-c.txt) and its stderr
  [`02.node-einb.solve.MOCK.option-c.stderr.txt`](runs/02.node-einb.solve.MOCK.option-c.stderr.txt), **mocks**: today's
  [`02.saved-node.solve.txt`](runs/02.saved-node.solve.txt) with the path, and *saved file* replaced by *container*
  in the `commit` row and the NOT CHECKED lines; the edits are
  [`02.node-einb.solve.MOCK.option-c.edits.diff`](runs/02.node-einb.solve.MOCK.option-c.edits.diff). It needs a new
  `.einb` section and a version bump of the container (`einb/2.0` today), read, not run.

## 4. The default

**(a)**, from the ledger: a `.einb` stores a program and its root, a saved file is a program with one commitment
(Q-M1f.402), and the container has no place for the commitment; resolving the saved file's imports from the disk
would read other files than the saved one, so the carried modules are used. Built in `ein-cli/src/kb.rs` and
`ein_einb::sections::Carried`; held by `ein-cli/tests/saved_files.rs`' `what_a_saved_file_is_refused_for`.

## 5. Files

- [`rerun.sh`](rerun.sh) — every command below, in order; the mocks are not touched by it.
- [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein) — copy of `tests/saved/02_a_choice_with_a_fact_inside_it.ein`.
- [`programs/03_the_standard_library.ein`](programs/03_the_standard_library.ein) — copy of `tests/saved/03_the_standard_library.ein`.
- [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) — every command run, with its exit code.
- [`runs/02.program.solve.txt`](runs/02.program.solve.txt) — read-out of program 02 (the run that saved `02.dump`).
- [`runs/02.dump/`](runs/02.dump/) — that save: [`index.json`](runs/02.dump/index.json), [`answer.txt`](runs/02.dump/answer.txt), [`answer.json`](runs/02.dump/answer.json), [`nogoods.json`](runs/02.dump/nogoods.json), [`models/01.ein`](runs/02.dump/models/01.ein) (root's file), [`models/02.ein`](runs/02.dump/models/02.ein) (the node's file).
- [`runs/02.root.einb`](runs/02.root.einb) — `kb save` of the saved root file.
- [`runs/02.program.einb`](runs/02.program.einb) — `kb save` of the program, for comparison.
- [`runs/02.node.kb-save.stderr.txt`](runs/02.node.kb-save.stderr.txt) — the refusal of the saved node file.
- [`runs/02.program.solve.txt`](runs/02.program.solve.txt), [`runs/02.saved-root.solve.txt`](runs/02.saved-root.solve.txt), [`runs/02.root-einb.solve.txt`](runs/02.root-einb.solve.txt), [`runs/02.program-einb.solve.txt`](runs/02.program-einb.solve.txt) — `ein solve` of the program, the saved root file, its container, the program's container.
- [`runs/02.program.summary.json`](runs/02.program.summary.json), [`runs/02.saved-root.summary.json`](runs/02.saved-root.summary.json), [`runs/02.root-einb.summary.json`](runs/02.root-einb.summary.json), [`runs/02.program-einb.summary.json`](runs/02.program-einb.summary.json) — their `--json-summary`.
- [`runs/02.program.saturate-dump.txt`](runs/02.program.saturate-dump.txt), [`runs/02.saved-root.saturate-dump.txt`](runs/02.saved-root.saturate-dump.txt), [`runs/02.root-einb.saturate-dump.txt`](runs/02.root-einb.saturate-dump.txt), [`runs/02.program-einb.saturate-dump.txt`](runs/02.program-einb.saturate-dump.txt) — their `ein saturate --dump`.
- [`runs/02.saved-node.solve.txt`](runs/02.saved-node.solve.txt), [`.stderr.txt`](runs/02.saved-node.solve.stderr.txt), [`runs/02.saved-node.summary.json`](runs/02.saved-node.summary.json), [`runs/02.saved-node.saturate-dump.txt`](runs/02.saved-node.saturate-dump.txt) — the saved node file read: read-out, its NOT CHECKED notes, summary, state.
- [`runs/02.diff.solve.program-vs-root-einb.diff`](runs/02.diff.solve.program-vs-root-einb.diff) — read-out, program vs root container.
- [`runs/02.diff.solve.saved-root-vs-root-einb.diff`](runs/02.diff.solve.saved-root-vs-root-einb.diff) — read-out, saved root file vs its container.
- [`runs/02.diff.summary.program-vs-root-einb.diff`](runs/02.diff.summary.program-vs-root-einb.diff) — summary, program vs root container.
- [`runs/02.diff.saturate.saved-root-vs-root-einb.diff`](runs/02.diff.saturate.saved-root-vs-root-einb.diff) — `saturate --dump`, saved root file vs its container.
- [`runs/02.diff.solve.root-einb-vs-saved-node.diff`](runs/02.diff.solve.root-einb-vs-saved-node.diff) — read-out, root container vs the saved node (what (c) would open).
- [`runs/same-name/cmp.txt`](runs/same-name/cmp.txt) — the two containers under one input name, byte by byte; [`program/p.ein`](runs/same-name/program/p.ein), [`program/p.einb`](runs/same-name/program/p.einb), [`saved/p.ein`](runs/same-name/saved/p.ein), [`saved/p.einb`](runs/same-name/saved/p.einb) are the inputs and outputs.
- [`runs/03.program.solve.txt`](runs/03.program.solve.txt) — read-out of program 03 (the run that saved `03.dump`).
- [`runs/03.dump/`](runs/03.dump/) — that save: [`index.json`](runs/03.dump/index.json), [`answer.txt`](runs/03.dump/answer.txt), [`answer.json`](runs/03.dump/answer.json), [`nogoods.json`](runs/03.dump/nogoods.json), [`models/01.ein`](runs/03.dump/models/01.ein) (root's file, five modules carried).
- [`runs/03.root.einb`](runs/03.root.einb), [`runs/03.root-einb.solve.txt`](runs/03.root-einb.solve.txt), [`runs/03.diff.solve.program-vs-root-einb.diff`](runs/03.diff.solve.program-vs-root-einb.diff) — its container, read, and diffed with the program's read-out.
- [`runs/03.no-stdlib.program.kb-save.stderr.txt`](runs/03.no-stdlib.program.kb-save.stderr.txt) — the program refused with no standard library reachable.
- [`runs/03.no-stdlib.root.kb-save.stderr.txt`](runs/03.no-stdlib.root.kb-save.stderr.txt), [`runs/03.no-stdlib.root.einb`](runs/03.no-stdlib.root.einb) — its saved root file stored then, the notes, and the container.
- [`runs/03.no-stdlib.root-einb.solve.txt`](runs/03.no-stdlib.root-einb.solve.txt), [`.stderr.txt`](runs/03.no-stdlib.root-einb.solve.stderr.txt), [`runs/03.diff.solve.root-einb-vs-no-stdlib-root-einb.diff`](runs/03.diff.solve.root-einb-vs-no-stdlib-root-einb.diff) — that container read (with its `StaleStdlib` warning), and diffed.
- [`runs/03.carried-vs-checkout.txt`](runs/03.carried-vs-checkout.txt) — each module 03's saved root file carries, compared with the checkout's `stdlib/` file.
- [`runs/03.moved-module.ein`](runs/03.moved-module.ein) — 03's saved root file with a comment added to the carried `std.math.arith`.
- [`runs/03.moved-module.kb-save.stderr.txt`](runs/03.moved-module.kb-save.stderr.txt), [`runs/03.moved-module.einb`](runs/03.moved-module.einb) — its `kb save`, the note, the container.
- [`runs/03.moved-module.solve.txt`](runs/03.moved-module.solve.txt), [`.stderr.txt`](runs/03.moved-module.solve.stderr.txt) — the file read directly: the note again.
- [`runs/03.moved-module-einb.solve.txt`](runs/03.moved-module-einb.solve.txt) — its container read: no note.
- [`runs/a8d.02.program.solve.txt`](runs/a8d.02.program.solve.txt) — the earlier binary's read-out of program 02.
- [`runs/a8d.02.dump/`](runs/a8d.02.dump/) — the earlier binary's save, `ein-answer/2`: [`index.json`](runs/a8d.02.dump/index.json), [`answer.txt`](runs/a8d.02.dump/answer.txt), [`answer.json`](runs/a8d.02.dump/answer.json), [`nogoods.json`](runs/a8d.02.dump/nogoods.json), [`models/01.ein`](runs/a8d.02.dump/models/01.ein), [`models/02.ein`](runs/a8d.02.dump/models/02.ein).
- [`runs/a8d.02.state2.einb`](runs/a8d.02.state2.einb) — the earlier binary's `kb save` of its state 2 file (exit 0).
- [`runs/a8d.02.state2-einb.solve.txt`](runs/a8d.02.state2-einb.solve.txt), [`.stderr.txt`](runs/a8d.02.state2-einb.solve.stderr.txt) — that container read: R `{∅}`, the claims FAILED, exit 1.
- [`runs/a8d.02.state2.solve.txt`](runs/a8d.02.state2.solve.txt), [`.stderr.txt`](runs/a8d.02.state2.solve.stderr.txt) — the state file read directly, the same.
- [`runs/a8d.02.diff.solve.state2-vs-state2-einb.diff`](runs/a8d.02.diff.solve.state2-vs-state2-einb.diff) — the two, diffed: the path only.
- [`runs/diff.state2-file.a8d-vs-today.diff`](runs/diff.state2-file.a8d-vs-today.diff) — state 2's file, the earlier binary's against today's.
- [`runs/02.root.kb-save.MOCK.option-b.stderr.txt`](runs/02.root.kb-save.MOCK.option-b.stderr.txt) — **mock**, option (b): the saved root file refused.
- [`runs/02.node.kb-save.MOCK.option-c.txt`](runs/02.node.kb-save.MOCK.option-c.txt) — **mock**, option (c): the saved node file stored.
- [`runs/02.node-einb.solve.MOCK.option-c.txt`](runs/02.node-einb.solve.MOCK.option-c.txt), [`.stderr.txt`](runs/02.node-einb.solve.MOCK.option-c.stderr.txt) — **mock**, option (c): its container read as the node.
- [`runs/02.node-einb.solve.MOCK.option-c.edits.diff`](runs/02.node-einb.solve.MOCK.option-c.edits.diff) — the edits that made that mock from the real read-out.
- [`renders/02.root-einb.render-program.dot`](renders/02.root-einb.render-program.dot) — `ein render program` of the root container, four graphs, drawn as [`.1.svg`](renders/02.root-einb.render-program.1.svg) (`kb`, fdp), [`.2.svg`](renders/02.root-einb.render-program.2.svg) (hrule `guess`), [`.3.svg`](renders/02.root-einb.render-program.3.svg) (rule `flag`), [`.4.svg`](renders/02.root-einb.render-program.4.svg) (`query`, with the `(solve …)` note).
- [`renders/02.root-einb.render-kb.dot`](renders/02.root-einb.render-kb.dot), [`.svg`](renders/02.root-einb.render-kb.svg) — `ein render kb` of the root container (fdp, the layout the view suggests).
- [`renders/02.saved-node.render-kb.dot`](renders/02.saved-node.render-kb.dot), [`.svg`](renders/02.saved-node.render-kb.svg) — `ein render kb` of the saved node file (fdp).

## 6. Re-run

From the repo root, `bash plans/m1f_hypothesis_and_documentation/review/01-save-and-load/17-Q-M1f.413/rerun.sh`. The binaries are
`build/review-bin/ein-head-17085b27` (today) and `build/review-bin/ein-a8d595eb` with
`EIN_STDLIB=build/wt-rev-a8d/stdlib` (before save/load). The core commands:

```sh
B=/home/user/work/ein/build/review-bin/ein-head-17085b27
cd plans/m1f_hypothesis_and_documentation/review/01-save-and-load/17-Q-M1f.413
$B solve programs/02_a_choice_with_a_fact_inside_it.ein --dump-answer runs/02.dump
$B kb save runs/02.dump/models/01.ein runs/02.root.einb          # exit 0
$B kb save runs/02.dump/models/02.ein runs/02.node.einb          # exit 1, refused
$B solve runs/02.root.einb --json-summary runs/02.root-einb.summary.json
$B solve programs/03_the_standard_library.ein --dump-answer runs/03.dump
EIN_STDLIB=/nonexistent $B kb save programs/03_the_standard_library.ein x.einb    # exit 1
EIN_STDLIB=/nonexistent $B kb save runs/03.dump/models/01.ein runs/03.no-stdlib.root.einb   # exit 0
A=/home/user/work/ein/build/review-bin/ein-a8d595eb; export EIN_STDLIB=/home/user/work/ein/build/wt-rev-a8d/stdlib
$A solve programs/02_a_choice_with_a_fact_inside_it.ein --dump-answer runs/a8d.02.dump
$A kb save runs/a8d.02.dump/models/02.ein runs/a8d.02.state2.einb   # exit 0
$A solve runs/a8d.02.state2.einb                                    # exit 1, claims FAILED
```

## 7. Found while building

- **A container stored with no standard library reachable warns on every later run.** Stored under
  `EIN_STDLIB=/nonexistent`, program 03's saved root file reads no standard library at all (it uses the five modules it
  carries), yet the container records the environment's standard-library digest, and each later `ein solve` of it, in
  the normal environment, prints `warning: runs/03.no-stdlib.root.einb was written under different inputs
  (StaleStdlib)` ([`03.no-stdlib.root-einb.solve.stderr.txt`](runs/03.no-stdlib.root-einb.solve.stderr.txt)). The
  answer is the same.
- **The other way round, no word.** A saved root file whose carried module is *not* the binary's
  ([`03.moved-module.ein`](runs/03.moved-module.ein)) gets its `note:` at `kb save` only; the container records the
  binary's standard-library digest and is read later with no note and no warning
  ([`03.moved-module-einb.solve.txt`](runs/03.moved-module-einb.solve.txt)), though it holds the moved text.
- **The `note:` wording where no standard library can be read.** Under `EIN_STDLIB=/nonexistent` each carried module
  gets *"the std.math.arith it carries is not this binary's standard library's"*
  ([`03.no-stdlib.root.kb-save.stderr.txt`](runs/03.no-stdlib.root.kb-save.stderr.txt)), though each is byte for byte
  the checkout's ([`03.carried-vs-checkout.txt`](runs/03.carried-vs-checkout.txt)); what is true is that no standard library was reachable to compare with.
- The ledger's statements reproduce: the saved root file is stored, the node file refused with the line the ledger
  gives, exit 1.
- **Re-taken with `ein-head-17085b27`** (the first look was `ein-head-b5fb8d70`): every read-out, summary, dump,
  refusal, note and diff is as it was, timings aside. What moved is the drawings, now in the canonical form and the
  program view in three parts (four graphs where there were five: the ontology and the `(solve …)` graph are gone,
  the declarations in `kb` and the `(solve …)` a note of `query`), and every `.einb` today's binary writes is 24 bytes
  shorter (`p.einb` 1 752 bytes, 1 776 at the first look; the container `einb/2.0` both), the two `p.einb` still
  differing in the same 64 bytes.
- **A committed fact is drawn as a derived one.** In `ein render kb` of the saved node, `(note (bad a))`, a choice of
  H, is dashed like `(flagged a)`, and has nothing beside it where a derived fact has `by flag`
  ([`renders/02.saved-node.render-kb.dot`](renders/02.saved-node.render-kb.dot)); the drawing does not say it is the
  node's commitment. The drawings' question (Q-M1f.424), not this one's.

## 8. Your call

- **(a)** store a saved root file from what it carries, refuse one with a `(commit …)` — **the default**.
- **(b)** refuse both; a container is made from a program file.
- **(c)** give the container a section for the commitment, so a node's container opens as that node.
