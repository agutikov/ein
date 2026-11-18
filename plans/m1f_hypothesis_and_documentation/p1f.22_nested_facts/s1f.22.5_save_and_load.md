# S1f.22.5 — a saved solution loads back as itself, in both directions

**Phase:** [P1f.22](README.md)
**Status:** planned 2026-11-17; **started 2026-11-17** on the user's word (*"start s1f.22.5"*). Its first task
ends with the user's look at the format on real programs; nothing is built before it. **Task 1 is done
(2026-11-18)**: [`s1f.22.5_the_format.md`](s1f.22.5_the_format.md), Q-M1f.400 to .403 put. **The user looked and
ruled** (2026-11-18, *"Q-M1f.400 (a), Q-M1f.401 (a), Q-M1f.402 (C), Q-M1f.403 (a)"*): the defaults but for Q-M1f.402's save of a loaded file, which is (C), **a saved file is
a node of its program's lattice**. Tasks 2 to 4 were started on that ruling and **paused 2026-11-18** on the user's word, their work in progress on the branch `s1f.22.5-save-load` (`db562a86`), while [S1f.22.6](s1f.22.6_written_erased.md) erases `written`; they resume after it. **Resumed and done 2026-11-18** on the branch, master merged into it: the record's last part.
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

**Tasks 2 to 4, the build, the tests and the pages** (2026-11-18). Done on the branch `s1f.22.5-save-load`, master
merged into it before and after (N-35 ended). Of the work in progress (`db562a86`), the grammar's three forms,
`ein_ir::saved`, the carried resolver, `Program::sources` and `commit`, the node read as a floor of the routes
(`region::above`, `Floor`, `saved_node`) and the claims' NOT CHECKED were kept and finished; its `load_held` and
`parse_held` through `saved::load_text` kept; nothing of it kept `written` away from a line, so nothing went with
`written`.

- **What was built.** `ein_ir::saved` writes and reads a saved file; `ein.y` takes `(program …)`, `(module NAME …)`
  and `(commit F …)` at the top of a file (`Saved`, the grammar page's § 2.16, six rows of its table); the loader
  refuses a wrapper in a program, and a load handed a saved file's forms reads it as its text is read
  (`saved::load_forms`, from `load_query_by`). The resolver reads a saved file's imports from the modules it carries
  and never the disk (`Resolver::carrying`), and records what a load read (`modules_read`), which
  `ein_core::Program::sources` holds beside the `(commit …)`'s texts, `Program::commit`. The engine reads the
  commitment as a node: `region::Floor` on the snapshot, written into every fork beside a node's own facts in H's
  order, the run's universe H less it, root's standing the node's (`region::above`), the node's state for `ein
  saturate` and `ein render kb` (`region::saved_node`), and a state found above it saved with the union
  (`RootStats::commitment`). The answer directory's writer (`ein-answer/3`) writes each state as the program's text,
  its modules' and the commitment, with no saturation, and `root.ein` the node again. The read-out's `commit` row,
  the summary's `root.commit`, `index.json`'s `commit`; a saved file's claims NOT CHECKED under `ein solve` and
  `ein test` (root alone, no exit code); `ein show hyps` and `ein render h-space` read the program's H; `ein kb save`
  stores a root's file from its carried modules and refuses a node. The nesting limit takes a saved file's wrappers'
  level (Q-M1f.185 rewritten). `dump_canonical` prints a wrapper as the file writes it.
- **The format as built, against the ruled one.** As ruled: one fixed first line, `;;; ein-saved/1 — the program and
  every module it imports, each as written, and the choices made: this file reads no other.`; `(program` a line
  break, the program file's bytes, a line break `)`; `(module NAME` the same, every module the load read, once,
  sorted by name; `(commit` then each candidate on a line of its own, two spaces in, the last closed `)`, in H's
  order, absent at root. The commitment is the one the state was read from; a `.einb`'s state carries its stored
  program and no module. What the run said of a state moved to `index.json` (`commit`, beside `region.states`'
  `from`, `commitments`, `minimal_generators`). Differs from the format page's proposal in one line: the first line
  is one, where the page's proposed files had two.
- **The invariances, and their counts.** `ein-cli/tests/answer_round_trip.rs`, per commit, every corpus entry
  neither `slow` nor `heavy_dump`: **631 programs, 28 319 state files**, each held three ways, and 639 of them through the
binary. Each file's first load is the run's state, fact for fact and
  origin for origin (`ein_cli::saturate::facts_section`); its root saved again is the file byte for byte; the two
  loads' `ein saturate --dump` KB sections are one text; and the binary on each program's first and last file
  (`ein solve F --no-search --dump-answer D`, `cmp`, `ein saturate --dump`). Under `EIN_CORPUS_SLOW=1`, every
  entry, over its four nightly shards: **650 programs, 492 717 state files**, each held three ways, 673 through the
binary, each shard 180, 194, 55 and 211 s on four P-core threads at four workers (N-41). No program is excluded: a program the loader refuses, or whose run writes no state, has no
  file to hold. `ein-infer/tests/explore.rs`' `every_mode_is_the_oracle_above_a_saved_node`: a saved file at
  362 nodes of corpus programs' cubes (245 states of R, 83 dead nodes, 34 owing; 47 on flagged programs, 315 on
  monotone ones), 5 792 cells, each file's cube the program's above the node, node for node,
  and every cell of `ein solve`'s two axes that cube. `ein-cli/tests/saved_files.rs` on `tests/saved/`: the
  demo's twin (`01`, root), a choice with a fact inside it (`02`), the standard library's twin (`03`, five modules
  carried, a load with `$EIN_STDLIB` pointing nowhere), a saved file in the corpus (`04`, `02`'s state, held to
  be what `02`'s dump writes, and every corpus sweep reads it as the node it names), and every refusal.
- **The questions defaulted**: Q-M1f.413 (`ein kb save` of a saved file), Q-M1f.414 (a fact's English, which a
  state file's comments carried), Q-M1f.415 (`ein show hyps`, `ein render h-space` read the program's H),
  Q-M1f.416 (what `ein test` asks a node), Q-M1f.417 (what a run of a saved file says of its node, `ein-answer/3`),
  Q-M1f.418 (the nesting limit and the wrappers). Q-M1f.392 and Q-M1f.400 to .403 are decision files; Q-M1f.58 is
  rewritten to the trace (`a-fact-reads-in-english-in-the-trace`), Q-M1f.185 to a state deeper than the limit, saved
  and loaded again, and Q-M1f.27 to the program a container's state file carries.
- **The checks, with exits.** On `339e592d` (master merged): `./run_tests.sh` exit 0 (20 min 13 s), and the first
  `./run_tests.sh --nightly` exit 101 at the deep-corpus `dot_wellformed`, `tests/saved/` not among its `SKIPPING`
  directories (`a5de14a1`); on `a5de14a1`, `./run_tests.sh --nightly` exit 0 (43 min 31 s). On the last code commit,
  `88ba8cde` (master merged again): `./run_tests.sh` exit 0 (1 147 s), `./run_tests.sh --nightly` exit 0
  (2 129 s). `utils/ci_container.sh` was not run: no DOT writer, the
  review bundle or what CI installs was changed.
- **Found outside the task**: N-36 to N-41 in [`findings.md`](findings.md).
