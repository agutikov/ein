# S1f.22.5 — a saved solution loads back as itself, in both directions

**Phase:** [P1f.22](README.md)
**Status:** planned 2026-11-17; **started 2026-11-17** on the user's word (*"start s1f.22.5"*). Its first task
ends with the user's look at the format on real programs; nothing is built before it. **Task 1 is done
(2026-11-18)**: [`s1f.22.5_the_format.md`](s1f.22.5_the_format.md), Q-M1f.400 to .403 put. **The user looked and
ruled** (2026-11-18, *"Q-M1f.400 (a), Q-M1f.401 (a), Q-M1f.402 (C), Q-M1f.403 (a)"*): the defaults but for Q-M1f.402's save of a loaded file, which is (C), **a saved file is
a node of its program's lattice**. Tasks 2 to 4 are started on that ruling.
**Rulings:** [Q-M1f.392](../open_questions.md#q-m1f392--a-state-file-that-reads-back-dead-through-written);
[Q-M1f.400](../open_questions.md#q-m1f400--how-a-saved-file-states-the-choices-made) (a),
[Q-M1f.401](../open_questions.md#q-m1f401--how-the-program-and-its-modules-are-in-one-file-unchanged) (a),
[Q-M1f.402](../open_questions.md#q-m1f402--what-save-and-load-are-as-commands-the-dump-they-are-held-to-and-a-files-first-lines)
(C), [Q-M1f.403](../open_questions.md#q-m1f403--the-programs-claims-in-a-saved-file) (a).

## The requirement, in the user's words

> save and load must in both directions must have identical results: save -> load -> save byte identical,
> load -> save -> load - have identical dumps; both save and load must not modify the program, must not add or
> remove any bit of information

and, asked the two points of design: **a saved solution is the program unchanged with the choices made**, the
state derived again on load (*"(a)"*), and **a saved file loads with no other file beside it** (*"yes"*).

## What is so today

Measured on the gate build ([`q392_state_file_and_written.md`](q392_state_file_and_written.md) and the question's
entry): a state file is the program as loaded with its imports inlined, then every fact of the state as a typed
fact, its origin in a comment. A second save of it differs from the first and grows by a copy of the state each
round; a reloaded state holds its derived facts as given and `written` facts the state did not hold; and for the
program of the question it loads dead. The round-trip test compares by value and takes `written` as a superset.

## What follows from the two rulings

A state is a function of the program and of the hypotheses committed, so a file that holds exactly those two
loses nothing and adds nothing: the facts are derived again, by the rules that derived them, with the origins they
had. Three things are then to be designed, and are the first task's:

1. **How a file states the choices made.** No form does today; a state file writes a hypothesis as a typed fact
   with `; hypothesis N` in a comment.
2. **How the program is in the file unchanged and with no other file beside it.** These pull apart where a
   program imports: inlining the modules rewrites the program (and changes the spelling a state prints a name
   in), and leaving the import lines as written needs the modules. The way that keeps both is the modules'
   texts carried in the file as they are, beside the program's own text as it is.
3. **What *save* and *load* are, as commands, for each of the two invariances**, and what a file's first lines may
   say: a line that names the state's place in an answer, or the path it was read from, is not reproduced by a
   second save.

## Tasks

1. **The format, on real programs, for the user's look**: the three points above decided with their alternatives,
   and a page that shows a saved solution of at least a program with no import, one that imports the standard
   library, one with a local module and one whose state rests on a commitment, each beside the two invariances
   stated as the commands that check them. Nothing is built for it but what the page needs to be honest.
2. **The build**, on the user's word: the save, the load, the form for the choices made.
3. **The tests**: the two invariances over **every corpus program**, byte for byte and dump for dump, in the
   place of the comparison by value; [`q392_demo.ein`](q392_demo.ein) and the standard library's twin of it as
   the simple unit test the user asked for.
4. **The pages and the decisions, in the same commits**: `10-answer-dump.md`, the header a state file carries,
   what `03-reserved-names.md` and the decisions on `written` say of a state file, the decision Q-M1f.392.

## Acceptance

`./run_tests.sh` exit 0 on the last commit, the two invariances among its tests; the nightly's deep-corpus steps,
since every dumped state of every program is read again.

## Record

**Task 1, the format on real programs** (2026-11-18). Done; it ends here, with the user's look.

- **The page**: [`s1f.22.5_the_format.md`](s1f.22.5_the_format.md), with the files, raw, in
  [`s1f.22.5_the_format/`](s1f.22.5_the_format/): for eight programs (no import; a choice with a fact inside it;
  the standard library's check; one choice and its derived facts; zebra, the standard library and one choice; a
  local module; an owing node; a corpus program that has the bug today), everything today's `--dump-answer`
  writes, today's second save, the KB dumps and their diffs, the proposed file, and a `commands.txt` of every
  command with its exit code and output; and `00_survey/`, every writer of a state run once. The proposed files are
  assembled by [`s1f.22.5_propose.py`](s1f.22.5_propose.py) from the programs' and modules' bytes and the
  commitment today's file names; no `ein` reads one today. The folders are written by
  [`s1f.22.5_showcase.py`](s1f.22.5_showcase.py).
- **The measurement**: [`s1f.22.5_measure.py`](s1f.22.5_measure.py) over the corpus entries the round trip reads
  per commit, on the gate build, its numbers in the page's § 2. The writer's state is a function of the program
  and the commitment (28 311 files byte-identical on a second run and with `-j 4`); the program and the choices
  saturated at once are not (facts in 6 programs, a first rule in 8); today's load is not the state (every derived
  and committed fact typed, `written` beyond root's in 136 states, dead in 2); today's second save is byte-identical
  for 0 of 1 267.
- **The questions**, each with a default, in [`open_questions.md`](../open_questions.md):
  [Q-M1f.400](../open_questions.md#q-m1f400--how-a-saved-file-states-the-choices-made) (the choices: `(commit F
  …)`), [Q-M1f.401](../open_questions.md#q-m1f401--how-the-program-and-its-modules-are-in-one-file-unchanged) (the
  program and its modules: `(program …)` and `(module NAME …)` around each file's bytes; the file reads no other),
  [Q-M1f.402](../open_questions.md#q-m1f402--what-save-and-load-are-as-commands-the-dump-they-are-held-to-and-a-files-first-lines)
  (no new command; the dump `ein saturate --dump`'s KB section; a loaded file a program whose root is its state;
  one fixed first line), [Q-M1f.403](../open_questions.md#q-m1f403--the-programs-claims-in-a-saved-file) (the
  program's claims, which 188 of 826 state files with a commitment fail today: not checked on such a file).
- **Found outside the task**: N-29 to N-32 in [`findings.md`](findings.md), N-30 the notable one: two corpus
  programs, `tests/stdlib/math/{decimal,binary}/0?_what_stays_silent.ein`, already have the bug, and the round-trip
  test does not see it (N-31).
- **What it leaves for the user**: the look at the format, and the four questions. Task 2, the build, waits on
  them.

**The user's look** (2026-11-18): *"Q-M1f.400 (a), Q-M1f.401 (a), Q-M1f.402 (C), Q-M1f.403 (a)"*. Every default stands but Q-M1f.402's save of a loaded file: (C), a saved
file is a node of its program's lattice, so `ein solve F` describes the part of the program's R above it, H is the
program's, and a state found above it is saved as the program with that state's commitment in one `(commit …)`.
Tasks 2 to 4 start on it.
