# S1f.22.5 — a saved solution loads back as itself, in both directions

**Phase:** [P1f.22](README.md)
**Status:** planned 2026-11-17; **started 2026-11-17** on the user's word (*"start s1f.22.5"*). Its first task
ends with the user's look at the format on real programs; nothing is built before it.
**Ruling:** [Q-M1f.392](../open_questions.md#q-m1f392--a-state-file-that-reads-back-dead-through-written).

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

Not started.
