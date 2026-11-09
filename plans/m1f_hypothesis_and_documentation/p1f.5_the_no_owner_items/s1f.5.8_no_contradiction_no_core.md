# S1f.5.8 — A node with no contradiction has no core: `null`, not `[]`

**Phase:** [P1f.5](README.md) (The no-owner items)
**Was S1e2.5.8**, in M1e2's P1e2.5, until 2026-11-08 ([§ The ids](README.md#the-ids)).

**Status:** specified 2026-11-07 on the user's ruling of that day on D-14
([Q-M1e2.27](../../m1e2_correctness/open_questions.md#q-m1e227--what-do--and-null-mean-in-eins-json)). **Running since
2026-11-09**, on the user's go-ahead for P1f.5 of 2026-11-08 and the instruction of 2026-11-09 (*"just continue with
default decisions till the end of the phase"*), beside the second lane's S1f.5.18: T1f.5.8.1, every surface listed
(`2b63223d`, [its record](#t1-record-2026-11-09)); T1f.5.8.2, `null` ([its record](#t2-record-2026-11-09)). Two choices were taken by default and recorded:
[Q-M1f.28](../open_questions.md#q-m1f28--is-a-not-dead-nodes-core-written--a-wrong-answer), no register row, and
[Q-M1f.29](../open_questions.md#q-m1f29--a-dead-roots-core-under---count), a dead root's core under `--count`.
**Depends on:** nothing.

## The ruling

The agent put D-14 as a spelling: an owing hole is written `"core": []` where S1e2.3.9 spells *no core*
`null`. The user, 2026-11-07: *"[] - meaning is something like "empty set"; null - meaning is "some single
object does not exist"; which fits better in which case?"*

The agent's reading of that rule: a core is **one object, a set of facts**, the given and committed facts
that together derive a node's contradiction. A node that is not dead has no contradiction, so its core does
not exist: `null`. `[]` is the empty set, a core that exists and holds no fact, *"the empty set of facts
suffices"*. That is a different claim, kept for that case alone. The user: *"Yes, plan it that way"*.

| case | does the core exist? | spelled |
|---|---|---|
| a dead node | yes, a set of facts | a list: `["(p a)"]` |
| a dead node whose contradiction needs no fact | yes, the empty set | `[]` |
| a node that is not dead: in R, or owing | no | `null` |
| a dead node past `--region-limit` whose core the run did not keep | yes, not kept | `null`, beside `core_status: "not-kept"`, which says why (S1e2.3.9) |

`refuted_by` reads the same way. A dead node's is a list of rules, one per fact of its contradiction, with
`(given)` where the core holds the fact. A node that is not dead has none: `null`.

The summary already spells its holes this way, one level up: `holes.minimal: []` means *computed, and none*,
and `holes: null` means *not known*. D-14 is the one place the two meanings merged.

## The case

Taken on 2026-11-07 at `916d97ac`, on the release binary, on `lattice/09_activator_hole.ein`, whose R is
three states and whose one hole, `{(opens K1)}`, owes 1:

| surface | node | written today |
|---|---|---|
| `ein solve --json-summary`, and `answer.json` and `index.json` under `--dump-answer`: `region.holes.minimal` | the owing hole | `"core": [], "core_status": null` |
| the same files: `region.core` | root, alive | `"core": null` |
| `ein oracle --dump`'s `region.json`: `region.nodes` | every node that is not dead: the 3 members of R and the hole | `"core": [], "refuted_by": [], "core_status": null` |

The inventory found the first row, on 8 holes of 7 programs. The re-take found the third, on every node that is
not dead. A dead node is written its core and rules (`branching/01_saturate_only`'s dead nodes: a core,
`within-budget`).

**A core that is the empty set does not occur today.** A contradiction that needs no fact needs a rule with
no premise, and `:match ()` never fires. That is
[F12](README.md#f12-an-empty-match-never-fires), filed the same day and not this stage's.

## What it changes

- **The summary's holes** ([`region_answer.rs`](../../../ein.rs/crates/ein-render/src/dump/region_answer.rs)
  line 304): a hole that is not dead is written `"core": null`. `--json-summary` and `--dump-answer`'s two
  files take it from the one writer.
- **The oracle's nodes** (the region dump's writer): a node that is not dead is written `"core": null` and
  `"refuted_by": null`.
- **What stays.** A dead node's core and rules, `core_status` everywhere, and `nogoods.json`'s `not-kept` rows.
  No answer moves, and no register row.
- **What moves.** The goldens that hold an oracle dump or a summary with a node that is not dead. Step 1 lists
  each, including `ein-cli/tests/region_dump.rs`'s.

## The method

### Task T1f.5.8.1 — every surface listed

Every writer of `core` and `refuted_by` in `ein-render` and `ein-cli`, the files each reaches, and the goldens
that hold them. `utils/region.py` and `region_census.py` read the dump, and step 1 checks that neither reads a
non-dead node's `core`.

### T1 record, 2026-11-09

**Every writer**, read off the code at `e0162f7c` and found again by the sweep below. Five writers put a node's core
on a JSON surface, and every one but `nogoods.json`'s, whose nodes are all dead, wrote a node that is not dead as
`[]` where it writes one:

| surface | writer | the nodes | `core` | `refuted_by` | a node that is not dead, at `e0162f7c` |
|---|---|---|---|---|---|
| `--json-summary`, and `--dump-answer`'s `answer.json` and `index.json`; `ein oracle --json-summary`; `--events`' `region` event, `ein solve` and `ein test`, which carries the same block | `region_answer::region` | root (`region.core`), each minimal hole (`region.holes.minimal[]`) | yes | root's | root: `null`; an owing hole: `[]` |
| `--dump-answer`'s `nogoods.json` | `region_answer::nogoods` | each member of N, every one dead | yes | yes | none: every member is dead |
| `ein oracle --dump`'s `region.json` | `dump::region::region_json` | every node of the cube | yes | yes | `[]` and `[]` |
| `--dump-lattice`'s `nodes.json` | `dump::region_lattice::json` | every node the walk visits | yes | yes | `[]` and `[]` |
| `--events`' `enter` lines, `ein solve` and `ein test` | `region::narrate_node` (`ein-infer`) | every node the walk visits | yes | no | `[]` |

The spec named the first and third; the re-take found `nodes.json` and the `enter` lines, which read one `Visited`
record and wrote it the same way, and the sweep found the `region` event, the summary's block in the event stream.
Every text surface prints a core only where it is non-empty, so none prints one for a node that is not dead: `region.md`, `nodes.md`, `--trace`'s *N*, the read-out's `unsat core` rows, and `ein render region`'s
tooltips and table. The readers of the dumps: `ein render region` (`region_doc`) reads `core` and `refuted_by` through
`strings`, which reads `null` as no fact; `utils/region.py` reads no core; `utils/region_census.py` copies a minimal
hole's `refuted_by` into its row, which writes `null` for an owing hole after the stage and prints neither; the review
bundle (`utils/review/run.py`, `case.py`) reads both through `or []`. No checked-in golden holds `"core": []`; the
manifest `ein-render/tests/golden/corpus_shapes.md5` holds them in digests (below).

**The spec predates S1f.5.12**, which retired `not-kept` and gives every member of N its core after the walk: the
ruling's fourth row, a dead node *whose core the run did not keep*, and *what stays*'s `not-kept` rows no longer occur.
No writer of a node's core is left that writes a dead node with no core, and `region_answer::nogoods` refuses one.

**The case, re-taken** at `e0162f7c` on the release binary, on `lattice/09_activator_hole.ein` (R three states, one hole
`{(opens K1)}` owing 1; `build/s1f.5.8/case/`): the summary, `answer.json` and `index.json` write the hole `"core": [],
"core_status": null` and root `"core": null`; `region.json` writes its four nodes, none dead, `"core": [],
"refuted_by": []`; and the two surfaces the spec did not name, `nodes.json` and the `enter` lines, write each of the
four nodes the walk visits `"core": []`, `nodes.json` with `"refuted_by": []` beside it.

**The sweep** (`build/s1f.5.8/sweep.py`, gitignored, kept with its logs): every `.ein` under `examples/`, `tests/`,
`stdlib/` and `corpus/`, 427 files; on each, `ein solve` under a bare describe, `--maximal`, `--minimal`, `--single` and
`--count`, once with `--json-summary`, `--dump-answer` and `--trace` (`--count` refuses `--trace`) and once with
`--dump-lattice` and `--events`, the events read off a pipe; describe at `--region-limit 1` and `2` too on the 29
programs the static check flags; `ein oracle --dump --json-summary`, and on the 265 programs it answers `ein render
region` on its directory, with `--view table`, and with describe's two directories as `--visited` and `--answer`; and
`ein test --events --json-report`. 6 150 runs at `e0162f7c`, release (`python3 build/s1f.5.8/sweep.py --bin
build/s1f.5.8/ein-base --out build/s1f.5.8/base.jsonl`, then `python3 build/s1f.5.8/tally.py
build/s1f.5.8/base.jsonl`, 2026-11-09). Each surface that writes a node's core is read and each node tallied by its
deadness and its spelling:

| surface | a node that is not dead, written `[]` | a dead node, written a list |
|---|---:|---:|
| the summary's minimal holes (`--json-summary`, `answer.json`, `index.json`, and `ein solve --events`' `region` event) | 20 holes on 17 runs of 6 programs, every one owing; `ein test --events`' `region` event, 1 on `tests/region/07_cube` | none: no run meets a dead hole |
| `ein oracle --json-summary`'s minimal holes | 7 holes on 6 programs, the same | none |
| `region.json`'s nodes, `core` and `refuted_by` | 53 617 nodes on 235 programs | 244 491 on 96 programs |
| `nodes.json`'s nodes, `core` and `refuted_by` | 2 092 457 on 1 373 runs of 263 programs | 417 086 on 536 runs of 122 programs |
| `ein solve --events`' `enter` lines | 2 095 947 on 1 383 runs of 265 programs | 417 086 on 536 runs of 122 programs |
| `ein test --events`' `enter` lines | 293 on 79 programs | 187 on 52 programs |
| `nogoods.json` | none: every member is dead | 5 554 members on 136 runs of 122 programs |

The six programs with a hole are `lattice/04_obligation_trigger_hole`, `08_absent_witness_hole`, `09_activator_hole`,
`12_activator_chain_hole`, `lattice/synth/local_not_global` (two holes) and `tests/region/07_cube`: the inventory's
eight holes but `past-limit-lines`' at `--region-limit 17`, which the sweep does not run. **No dead node is written
`[]`** on any surface: a core that is the empty set does not occur, as the spec says (F12, S1f.5.9). Root's core is
`null` where root is not dead on every run (584 summaries of 263 programs, and the oracle's 233), and a list where it is
dead under describe (39 programs) and the oracle (32). **One dead node is written `null`**: under `--count`, which
prints no core on stdout, root's `region.core` and `region.refuted_by` are `null` beside `region.root: "dead"`, on all
39 programs whose root is dead; a mode writes `region.root: null` and no core either. That is the summary's `null`
for *does not apply*, which the ruling's *or is not known* covers, and it stays
([Q-M1f.29](../open_questions.md#q-m1f29--a-dead-roots-core-under---count), defaulted).

**Is it a register row? No** ([Q-M1f.28](../open_questions.md#q-m1f28--is-a-not-dead-nodes-core-written--a-wrong-answer),
defaulted (a)). The coordinator's brief asks whether `[]` on a node with no contradiction is a printer line that says
something false about an answer, which the register files before a fix. Read by the register's own rule, *an answer
that is false under the definitions the user has ruled on*, it is not: the register's answers are R, N, a dead node's
core and rules, claims, exit codes, refusals and a printed line that reads as another answer (W42's, a count printed
as another number). A node that is not dead has no core to be wrong about, so a row could state no `right` but its
absence, and every object that carries the `[]` carries its deadness beside it (`dead: false`, `status`, `kind`).
The inventory took D-14 as a spelling and the user ruled it as one, and the nearest precedent, M1e2 S1e2.3.9's empty
core on a member of N a run had not taken (N6, *reads as refuted by nothing*), moved no row. The spec says the same:
*no answer moves, and no register row*.

**Every move, on a prototype of T2** (`python3 build/s1f.5.8/diff.py build/s1f.5.8/base.jsonl
build/s1f.5.8/proto.jsonl`, the same sweep on the prototype's release build, 2026-11-09): **1 714 runs of 265
programs move, every one by D-14's move, and 0 findings.** By surface: the summary's holes on 23 runs of the six
programs (17 of `ein solve`, 6 of `ein oracle`), `answer.json` on 17 and `index.json` on 11; `region.json` on 235 runs
of 235 programs; `nodes.json` on 1 373 runs of 263; and the event stream on 1 462 runs of 265, its `enter` lines and,
on the six programs with a hole, its `region` event. No exit code, stdout, stderr, state file, `nogoods.json`, trace,
`answer.txt`, `nodes.md`, `region.md`, report or drawing moves. The diff holds each
surface that writes a node's core as the prototype wrote it against the base's with D-14's move applied, every node
that is not dead read `null` where it was `[]`, and every other file, stream and exit code as written: `answer.txt`,
the state files, `nogoods.json`, every trace, stdout and stderr, `nodes.md`, `region.md`, `ein test`'s report and
`ein render region`'s three drawings, wall-clock masked (`region.md`'s cost line, the report's `ms`, `ein test`'s
`(0.01 s)`). The sweep's first pass compared those three as written, and the 394 runs it flagged were the clock; it
also found the sixth surface, `--events`' `region` event, which carries the summary's `region` block and so its
holes, and moved as the summary did. After the move, every node that is not dead is written `null` on every surface,
every dead node a list, and `--count`'s dead root `null` as before (`python3 build/s1f.5.8/tally.py
build/s1f.5.8/proto.jsonl`).

**The goldens and tests that move** (`cargo test --workspace --release --no-fail-fast` on the prototype, 2026-11-09):
- `ein-render/tests/golden/corpus_shapes.md5`: 1 505 of 13 842 renderings, 1 285 of the 1 595 `cores[…]` (the
  summary's `region` block and `nodes.json`, under each of five questions) and 220 of the 319 `oracle` (`region.json`
  at the sweep's cube limit), and no other op. Each is D-14's move and nothing else: undoing it, every node that is not
  dead read back `[]` where it is now `null` (59 955 fields), gives back the banked digest of every one
  (`build/s1f.5.8/s1f58_shapes_probe.rs`, a scratch test copied into `ein-render/tests/`, run, and deleted,
  2026-11-09; its list is `build/s1f.5.8/shapes_moved.txt`).
- Nothing else fails. `cli_semantics`' `a_bare_underscore_names_nothing_in_every_place` failed under a `TMPDIR` whose
  path held `..`, which the loader's message resolves, and passes under the gate's; `answer_round_trip`,
  `region_dump`, `region_modes`, `answer_dump`, `read_out_reference`, `read_out_examples`, `lever_invariance`,
  `events_reference`, `jobs_invariance`, `id_order_invariance`, `wrong_answers` and the rest pass unchanged. No
  register row's `now` reads a non-dead node's core: the register's `core` is root's, and its `nogoods` N's members,
  every one dead.

**Found on the way, outside the brief:**
- **The text surfaces spell the empty set as nothing.** `region.md`, `nodes.md`, `ein render region`'s tooltips and
  table, and the read-out's `unsat core` rows print a core only where it is non-empty, so a dead node whose core is
  the empty set would print no core at all, where the JSON says `[]`. No program reaches one today; S1f.5.9 makes the
  first (`(rule r () :match () :assert (false))`, root dead, core `[]`, `refuted by r`), so it is that stage's to
  spell, and the coordinator is told, S1f.5.9 being the second lane's.
- **The glossary still said `--dump-answer` and `--trace` take every dead node's core they visit**, which S1f.5.12
  ended: they take each member of N's after the walk. T1f.5.8.3 corrects the sentence.

### Task T1f.5.8.2 — `null`

The two writers, and a test in `ein-cli/tests/answer_round_trip.rs`, which already reads every corpus dump:
over every corpus program, every node and hole that is not dead has `core` and `refuted_by` `null`, and every
dead one has a list.

### T2 record, 2026-11-09

**One rule, one function.** `ein_render::dump::region_answer::core_field(dead, status, list)` writes a node's core, or
its rules, as a list where the node is dead and the run took its core, and `null` otherwise; every writer in
`ein-render` calls it: the summary's root (`!alive`) and minimal holes (`dead`), `nogoods.json` (every member dead),
`region.json` (the node's `DEAD` bit) and `nodes.json` (`kind == "dead"`). The `enter` line is written in `ein-infer`,
which cannot call `ein-render`, and `region::narrate_node` applies the same test. A dead node whose core the run did
not take would be `null` beside a `null` `core_status`, the ruling's *not known*, where it would have been the `[]`
S1e2.3.9 removed; no surface the binary writes holds one, each asking for the cores it prints, and the test below
holds every dead node to a list.

**The test** is `ein-cli/tests/answer_round_trip.rs`' `no_core_unless_dead`, run on every corpus program the round
trip already dumps: `answer.json`'s and `index.json`'s root and minimal holes, `nogoods.json`'s members, and, where
`ein oracle --region-limit 10` answers, which bounds their cost, the oracle's summary, `region.json`, and one `ein
solve --dump-lattice --events` run's `nodes.json`, `enter` lines and `region` event, the `enter` lines held node for
node to `nodes.json`; `eager_cores` holds the `nodes.json` of every other program it dumps. Each node or hole that is
dead must have its fields lists and every other `null`, and present: 5 016 dead and 17 004 not dead over the eight
shards, each shard held to at least one of each (release, 2026-11-09, `cargo test --release -p ein-cli --test
answer_round_trip -- --nocapture`). With the old spelling put back in `core_field` and the `enter` line, the test fails
on every surface: `answer.json`, `index.json`, the oracle's summary and the `region` event on the seven holes,
`region.json` on 2 312 nodes, `nodes.json` on 27 872 and the `enter` lines on 1 107 (a scratch mutation, 2026-11-09).

**What moved** is T1's list and nothing else. `corpus_shapes.md5`'s 1 505 renderings (`EIN_BLESS=1 cargo test -p
ein-render --test corpus_shapes`, the manifest's diff read before it was kept: exactly the 1 505 lines T1 named, each
a digest and no line count). The kernel pages that describe the spelling say it in the same commit:
[`read_out.md` § 11](../../../docs/kernel/inference/read_out.md#11---json-summary-the-answer-blocks), where the
summary's keys are defined, carries the rule, `[]` the empty set and `null` an object that does not exist or that the
run does not state, with `--count`'s dead root
([Q-M1f.29](../open_questions.md#q-m1f29--a-dead-roots-core-under---count)); [`region_dump.md`](../../../docs/kernel/inference/region_dump.md)'s
`nodes` row, [`lattice_dump.md`](../../../docs/kernel/inference/lattice_dump.md)'s `nodes.json` and
[`events.md`](../../../docs/kernel/inference/events.md)'s `enter` row say a node that is not dead has `null`; and
[`glossary.md`](../../../docs/kernel/glossary.md) states the rule beside the core's definition.
[`answer_dump.md`](../../../docs/kernel/inference/answer_dump.md) defines no summary field of its own, deferring them to
`read_out.md` § 11, and its `nogoods.json` holds dead nodes alone, so it is unedited. No answer, exit code or register
row moved.

### Task T1f.5.8.3 — the pages

`region_dump.md`'s `nodes` row, `answer_dump.md`'s summary fields, and the live read-out reference's, [`read_out.md`](../../../docs/kernel/inference/read_out.md)
§ 11 (S1f.5.16), each
with the rule: `null`, the core does not exist; `[]`, the empty set. The inventory's D-14 says what fixed it.

## Acceptance

- Every JSON surface writes `"core": null` for a node that is not dead, and `"refuted_by": null` where it
  writes that field, on every corpus program.
- A dead node's core and rules are unchanged.
- No answer moves.
- `./run_tests.sh` is green.
