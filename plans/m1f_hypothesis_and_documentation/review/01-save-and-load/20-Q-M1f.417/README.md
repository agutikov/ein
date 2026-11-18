# Q-M1f.417 — what a run of a saved file says of its node, and the directory's schema

## 1. The question

A run of a saved node file answers the part of the program's R **above** the node: its universe is H less the node's
choices, and its `∅` is the node. Nothing in the read-out of a program says *which* node a run starts from, and the
state files no longer carry a header naming their run (that moved to `index.json`, Q-M1f.402). So: how should a run of
a saved file name its node, and how does the answer directory say which commitment each saved file holds? Today
(option a): a **`commit` row** under `universe` in the read-out; **`root.commit`** in `--json-summary` (with
`root.dead`, `root.owes`, `root.owed` the node's standing while `root.facts` stays the program root's fixpoint); a
per-file **`commit`** in `index.json` beside `region.states`' `from`; and the directory's schema bumped from
`ein-answer/2` to **`ein-answer/3`**, whose writer clears an earlier schema's directory as its own. Option (b): no row;
the read-out of a saved file is the read-out of a program.

Recorded in [`../../open_questions.md` § Q-M1f.417](../../../open_questions.md#q-m1f417--what-a-run-of-a-saved-file-says-of-its-node-and-the-directorys-schema).
The shipped pages: [`08-read-out.md` § 5.3](../../../../../docs/ein/user/04-reference/08-read-out.md#53-under-universe)
and [`10-answer-dump.md`](../../../../../docs/ein/user/04-reference/10-answer-dump.md) (§ The directory, the `root`
block's `root.commit` row).

## 2. What is so today

Program: [`programs/02_genuine_3set_death.ein`](programs/02_genuine_3set_death.ein), a copy of
`examples/lattice/02_genuine_3set_death.ein`: H is `(a-prop X)`, `(b-prop X)`, `(c-prop X)`, all three together die,
R is seven states. A smaller second program, [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein)
(copy of `tests/saved/02_…`), is used for the schema runs and once for the row. Binaries:
`build/review-bin/ein-head-17085b27` (today) and `build/review-bin/ein-a8d595eb` with its own standard library
(before save/load). Every exit is 0 ([`00-exit-codes.txt`](runs/00-exit-codes.txt)).

**The program, both binaries.** The read-out is the same byte for byte
([`diff.read-out.a8d-vs-today.diff`](runs/diff.read-out.a8d-vs-today.diff), empty), and so is the directory's file
list ([`today.dump.tree.txt`](runs/today.dump.tree.txt), [`a8d.dump.tree.txt`](runs/a8d.dump.tree.txt),
[`diff.tree.a8d-vs-today.diff`](runs/diff.tree.a8d-vs-today.diff), empty: `answer.json`, `answer.txt`, `index.json`,
`models/01.ein` … `07.ein`, `nogoods.json`). What moves:

- **`index.json`**: the schema `ein-answer/2` → `ein-answer/3`, and each `models[]` entry gains `commit`, the
  candidates its `(commit …)` names: `[]` for root's file, `["(a-prop X)"]` for state 2, two for each maximal state.
  Side by side: [`side-by-side.index.a8d-vs-today.txt`](runs/side-by-side.index.a8d-vs-today.txt); diffed:
  [`diff.index.a8d-vs-today.diff`](runs/diff.index.a8d-vs-today.diff). Files:
  [`a8d.dump/index.json`](runs/a8d.dump/index.json), [`today.dump/index.json`](runs/today.dump/index.json).
- **`answer.json`** (= `--json-summary`, [`check.answer-json-is-the-summary.txt`](runs/check.answer-json-is-the-summary.txt)):
  one key added, `root.commit: []` on a program ([`diff.answer-json.a8d-vs-today.diff`](runs/diff.answer-json.a8d-vs-today.diff),
  [`diff.summary.a8d-vs-today.diff`](runs/diff.summary.a8d-vs-today.diff)).
- **The state files**: the program reprinted plus the state's facts with their English, against the program as written
  plus `(commit (a-prop X))` ([`diff.state2-file.a8d-vs-today.diff`](runs/diff.state2-file.a8d-vs-today.diff)).

**A run of the saved node `{(a-prop X)}`** ([`today.dump/models/02.ein`](runs/today.dump/models/02.ein)):
`ein solve` of it ([`node.solve.txt`](runs/node.solve.txt)) prints the `commit` row:

<!-- output -->
```text
  universe          2 candidates, 0 dead alone
    commit          1 candidate of H, the node this saved file names: R is the part of its program's R above it, and the universe is the rest of H — (a-prop X)
```

and lists R above the node as `(model)`, `(model (b-prop X))`, `(model (c-prop X))`, each relative to the node.
Its directory ([`node.dump/`](runs/node.dump/)) saves each state with the **union** of the node and the run's
commitment, and `index.json` says both ([`node.commit-vs-from.txt`](runs/node.commit-vs-from.txt), read off
[`node.dump/index.json`](runs/node.dump/index.json) and [`node.summary.json`](runs/node.summary.json)):

<!-- output -->
```text
summary root.commit: ['(a-prop X)']
summary root.dead / owes / owed: False 0 []
summary root.facts: 10  root.hypgen.emitted: 3  region.universe.candidates: 2

file            index.json models[].commit          region.states[].from      commitments  minimal_generators
models/01.ein   ['(a-prop X)']                      []                        1            [[]]
models/02.ein   ['(a-prop X)', '(b-prop X)']        ['(b-prop X)']            1            [['(b-prop X)']]
models/03.ein   ['(a-prop X)', '(c-prop X)']        ['(c-prop X)']            1            [['(c-prop X)']]
```

`root.facts` is 10 on the node's run as on the program's: the program root's fixpoint, not the node's state
([`diff.summary.program-vs-node.diff`](runs/diff.summary.program-vs-node.diff)). The same row on the smaller program:
[`today.p02.node.solve.txt`](runs/today.p02.node.solve.txt), `— (note (bad a))`.

**`root.dead` is the node's standing.** A node written by hand with all three choices
([`dead-node.ein`](runs/dead-node.ein), from `models/05.ein`; no saved file of R is dead) reads `empty — every node
is dead`, its `commit` row naming three candidates ([`dead-node.solve.txt`](runs/dead-node.solve.txt)); its summary
says `dead=True` while `facts` stays the program root's 10 ([`dead-node.root-block.txt`](runs/dead-node.root-block.txt)).

**The node's root is the node.** `ein solve NODE --no-search --dump-answer` writes `root.ein`, the node file again, byte
for byte ([`check.node-root-is-the-node.txt`](runs/check.node-root-is-the-node.txt); its read-out with the `commit`
row, [`node.solve-no-search.txt`](runs/node.solve-no-search.txt)).

**The earlier binary on its own state file of the same node** answers the same R (the state file is a program in which
`(a-prop X)` is stated, so the hrule's instance is filtered) with no `commit` row; the read-outs differ in that row and
the `work` row ([`diff.node-read-out.a8d-vs-today.diff`](runs/diff.node-read-out.a8d-vs-today.diff),
[`a8d.node.solve.txt`](runs/a8d.node.solve.txt)).

**The schema bump and the writer's clearing** ([`over/summary.txt`](runs/over/summary.txt)):

- an `ein-answer/2` directory (the earlier binary's, seven state files) written again by today's binary with a
  two-state program: `models/03.ein` … `07.ein` are gone, the schema is `ein-answer/3`
  ([`over/a8d-then-today.before.tree.txt`](runs/over/a8d-then-today.before.tree.txt) →
  [`.after.tree.txt`](runs/over/a8d-then-today.after.tree.txt));
- the reverse, an `ein-answer/3` directory written again by the earlier binary: it does not know `/3`, deletes nothing,
  and five of today's state files stay beside its two
  ([`over/today-then-a8d.after.tree.txt`](runs/over/today-then-a8d.after.tree.txt);
  [`over/today-then-a8d/index.json`](runs/over/today-then-a8d/index.json) says `ein-answer/2` and lists two).

## 3. The options

- **(a) as built**: the `commit` row, `root.commit` (with `root.dead`/`owes`/`owed` the node's, `root.facts` the
  program root's), `index.json`'s per-file `commit` beside `from`, `ein-answer/3`. All of § 2.
- **(b) no row: the read-out of a saved file is the read-out of a program.** **Mock**:
  [`node.solve.MOCK.option-b.txt`](runs/node.solve.MOCK.option-b.txt), today's read-out of the node with the `commit`
  line deleted, nothing else ([`node.solve.today-vs.MOCK.option-b.diff`](runs/node.solve.today-vs.MOCK.option-b.diff)).
  It then reads as the earlier binary's state-file run but for the `work` row
  ([`node.solve.a8d-vs.MOCK.option-b.diff`](runs/node.solve.a8d-vs.MOCK.option-b.diff)): nothing in it says that
  `(model)` is a state holding `(a-prop X)`. The option names the row only; if it is read to drop the other two
  places too, the mocks are [`node.summary.MOCK.option-b.json`](runs/node.summary.MOCK.option-b.json) (`root.commit`
  deleted, [`diff`](runs/node.summary.today-vs.MOCK.option-b.diff)) and
  [`node.index.MOCK.option-b.json`](runs/node.index.MOCK.option-b.json) (every `commit` deleted,
  [`diff`](runs/node.index.today-vs.MOCK.option-b.diff)); then a reader of `index.json` can tell the node only by
  opening each saved file. Read, not run: the bump to `ein-answer/3` follows from the state files' new form as much as
  from `commit`, so (b) would not by itself keep `/2`.

## 4. The default

**(a)**, from the ledger: Q-M1f.402 moves what the old state-file header said of the run to `index.json`, and a run of
a saved file, the part of R above a node, has to say which node. Built in `ein-render/src/read_out.rs`,
`ein-cli/src/summary.rs`, `ein-render/src/dump/region_answer.rs`; the pages `08-read-out.md` § 5.3 and its example X21,
and `10-answer-dump.md`.

## 5. Files

- [`rerun.sh`](rerun.sh) — every command below, in order, and the mocks' diffs; the mocks themselves are not touched by it.
- [`programs/02_genuine_3set_death.ein`](programs/02_genuine_3set_death.ein) — copy of `examples/lattice/02_genuine_3set_death.ein`.
- [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein) — copy of `tests/saved/02_a_choice_with_a_fact_inside_it.ein`.
- [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) — every command run, with its exit code.
- [`runs/today.solve.txt`](runs/today.solve.txt), [`runs/today.summary.json`](runs/today.summary.json) — today's read-out and summary of the program.
- [`runs/today.dump/`](runs/today.dump/) — today's directory: [`index.json`](runs/today.dump/index.json), [`answer.txt`](runs/today.dump/answer.txt), [`answer.json`](runs/today.dump/answer.json), [`nogoods.json`](runs/today.dump/nogoods.json), `models/01.ein` … `07.ein` ([`02.ein`](runs/today.dump/models/02.ein) the node read below, [`05.ein`](runs/today.dump/models/05.ein) the source of `dead-node.ein`).
- [`runs/a8d.solve.txt`](runs/a8d.solve.txt), [`runs/a8d.summary.json`](runs/a8d.summary.json) — the earlier binary's read-out and summary.
- [`runs/a8d.dump/`](runs/a8d.dump/) — its directory, `ein-answer/2`: [`index.json`](runs/a8d.dump/index.json), [`answer.txt`](runs/a8d.dump/answer.txt), [`answer.json`](runs/a8d.dump/answer.json), [`nogoods.json`](runs/a8d.dump/nogoods.json), `models/01.ein` … `07.ein` ([`02.ein`](runs/a8d.dump/models/02.ein)).
- [`runs/today.dump.tree.txt`](runs/today.dump.tree.txt), [`runs/a8d.dump.tree.txt`](runs/a8d.dump.tree.txt), [`runs/diff.tree.a8d-vs-today.diff`](runs/diff.tree.a8d-vs-today.diff) — the two file lists and their diff (empty).
- [`runs/diff.read-out.a8d-vs-today.diff`](runs/diff.read-out.a8d-vs-today.diff) — the program's read-out, both binaries (empty).
- [`runs/side-by-side.index.a8d-vs-today.txt`](runs/side-by-side.index.a8d-vs-today.txt), [`runs/diff.index.a8d-vs-today.diff`](runs/diff.index.a8d-vs-today.diff) — `index.json`, side by side and diffed.
- [`runs/diff.answer-json.a8d-vs-today.diff`](runs/diff.answer-json.a8d-vs-today.diff), [`runs/diff.summary.a8d-vs-today.diff`](runs/diff.summary.a8d-vs-today.diff) — `answer.json` and `--json-summary`, diffed.
- [`runs/check.answer-json-is-the-summary.txt`](runs/check.answer-json-is-the-summary.txt) — `answer.json` against `--json-summary`.
- [`runs/diff.state2-file.a8d-vs-today.diff`](runs/diff.state2-file.a8d-vs-today.diff) — state 2's file, both binaries.
- [`runs/node.solve.txt`](runs/node.solve.txt), [`runs/node.summary.json`](runs/node.summary.json) — today's run of the saved node: the `commit` row, `root.commit`.
- [`runs/node.dump/`](runs/node.dump/) — its directory: [`index.json`](runs/node.dump/index.json), [`answer.txt`](runs/node.dump/answer.txt), [`answer.json`](runs/node.dump/answer.json), [`nogoods.json`](runs/node.dump/nogoods.json), `models/01.ein` … `03.ein` ([`02.ein`](runs/node.dump/models/02.ein) ends `(commit (a-prop X) (b-prop X))`).
- [`runs/node.commit-vs-from.txt`](runs/node.commit-vs-from.txt) — `index.json`'s `commit` beside `region.states`' `from`, and the summary's root block.
- [`runs/diff.summary.program-vs-node.diff`](runs/diff.summary.program-vs-node.diff) — the program's summary against the node's.
- [`runs/node.solve-no-search.txt`](runs/node.solve-no-search.txt), [`runs/node-root.summary.json`](runs/node-root.summary.json), [`runs/node-root.dump/`](runs/node-root.dump/) ([`root.ein`](runs/node-root.dump/root.ein), [`answer.txt`](runs/node-root.dump/answer.txt), [`answer.json`](runs/node-root.dump/answer.json)), [`runs/node-root.dump.tree.txt`](runs/node-root.dump.tree.txt) — the node under `--no-search`, its saved root.
- [`runs/check.node-root-is-the-node.txt`](runs/check.node-root-is-the-node.txt) — that `root.ein` against the node file.
- [`runs/dead-node.ein`](runs/dead-node.ein), [`runs/dead-node.solve.txt`](runs/dead-node.solve.txt), [`runs/dead-node.summary.json`](runs/dead-node.summary.json), [`runs/dead-node.root-block.txt`](runs/dead-node.root-block.txt) — a hand-made dead node: read-out, summary, root block against the program's.
- [`runs/a8d.node.solve.txt`](runs/a8d.node.solve.txt), [`runs/a8d.node.summary.json`](runs/a8d.node.summary.json) — the earlier binary on its state file of the node.
- [`runs/diff.node-read-out.a8d-vs-today.diff`](runs/diff.node-read-out.a8d-vs-today.diff) — that read-out against today's of the node.
- [`runs/node.events.jsonl`](runs/node.events.jsonl) ([`.md`](runs/node.events.md)), [`runs/a8d.node.events.jsonl`](runs/a8d.node.events.jsonl) ([`.md`](runs/a8d.node.events.md)), [`runs/node.layer-facts.txt`](runs/node.layer-facts.txt) — `--events` of the node's run, both binaries, and their `layer` events' facts.
- [`runs/today.p02.solve.txt`](runs/today.p02.solve.txt), [`runs/today.p02.dump/`](runs/today.p02.dump/) ([`index.json`](runs/today.p02.dump/index.json), [`models/02.ein`](runs/today.p02.dump/models/02.ein), [`models/01.ein`](runs/today.p02.dump/models/01.ein), [`answer.txt`](runs/today.p02.dump/answer.txt), [`answer.json`](runs/today.p02.dump/answer.json), [`nogoods.json`](runs/today.p02.dump/nogoods.json)) — the smaller program saved.
- [`runs/today.p02.node.solve.txt`](runs/today.p02.node.solve.txt), [`.stderr.txt`](runs/today.p02.node.solve.stderr.txt), [`runs/today.p02.node.summary.json`](runs/today.p02.node.summary.json) — its node's run: the row with `(note (bad a))`.
- [`runs/over/summary.txt`](runs/over/summary.txt) — the two cross-binary rewrites of one directory, what is left.
- [`runs/over/a8d-then-today/`](runs/over/a8d-then-today/), [`.before.tree.txt`](runs/over/a8d-then-today.before.tree.txt), [`.after.tree.txt`](runs/over/a8d-then-today.after.tree.txt), [`runs/over.a8d-first.txt`](runs/over.a8d-first.txt), [`runs/over.today-second.txt`](runs/over.today-second.txt) — an `/2` directory written over by today's binary, and the two read-outs.
- [`runs/over/today-then-a8d/`](runs/over/today-then-a8d/), [`.before.tree.txt`](runs/over/today-then-a8d.before.tree.txt), [`.after.tree.txt`](runs/over/today-then-a8d.after.tree.txt), [`runs/over.today-first.txt`](runs/over.today-first.txt), [`runs/over.a8d-second.txt`](runs/over.a8d-second.txt) — a `/3` directory written over by the earlier binary, and the two read-outs.
- [`runs/node.solve.MOCK.option-b.txt`](runs/node.solve.MOCK.option-b.txt), [`runs/node.solve.today-vs.MOCK.option-b.diff`](runs/node.solve.today-vs.MOCK.option-b.diff), [`runs/node.solve.a8d-vs.MOCK.option-b.diff`](runs/node.solve.a8d-vs.MOCK.option-b.diff) — **mock**, (b): the node's read-out with no row; against today and against the earlier binary.
- [`runs/node.summary.MOCK.option-b.json`](runs/node.summary.MOCK.option-b.json), [`runs/node.summary.today-vs.MOCK.option-b.diff`](runs/node.summary.today-vs.MOCK.option-b.diff) — **mock**, (b) read wide: no `root.commit`.
- [`runs/node.index.MOCK.option-b.json`](runs/node.index.MOCK.option-b.json), [`runs/node.index.today-vs.MOCK.option-b.diff`](runs/node.index.today-vs.MOCK.option-b.diff) — **mock**, (b) read wide: no per-file `commit`.

## 6. Re-run

From the repo root, `bash plans/m1f_hypothesis_and_documentation/review/01-save-and-load/20-Q-M1f.417/rerun.sh`. The core commands:

```sh
B=/home/user/work/ein/build/review-bin/ein-head-17085b27
A=/home/user/work/ein/build/review-bin/ein-a8d595eb; AS=/home/user/work/ein/build/wt-rev-a8d/stdlib
cd plans/m1f_hypothesis_and_documentation/review/01-save-and-load/20-Q-M1f.417
$B solve programs/02_genuine_3set_death.ein --dump-answer runs/today.dump --json-summary runs/today.summary.json
EIN_STDLIB=$AS $A solve programs/02_genuine_3set_death.ein --dump-answer runs/a8d.dump --json-summary runs/a8d.summary.json
diff -u runs/a8d.dump/index.json runs/today.dump/index.json
$B solve runs/today.dump/models/02.ein --dump-answer runs/node.dump --json-summary runs/node.summary.json
$B solve runs/today.dump/models/02.ein --no-search --dump-answer runs/node-root.dump
EIN_STDLIB=$AS $A solve runs/a8d.dump/models/02.ein
```

## 7. Found while building

- **The `work` row and `--region-budget` count the node's own choices on every saturation.** The node's run reads
  `4 saturations · 9 facts beyond root`; the earlier binary's run of its state file of the same node, the same R,
  reads `5 facts beyond root` ([`diff.node-read-out.a8d-vs-today.diff`](runs/diff.node-read-out.a8d-vs-today.diff)).
  The `layer` events say where: 1, 4, 4 facts per layer today against 0, 2, 3, one more in each of the four
  saturations, `(a-prop X)` ([`node.layer-facts.txt`](runs/node.layer-facts.txt), from
  [`node.events.jsonl`](runs/node.events.jsonl) and [`a8d.node.events.jsonl`](runs/a8d.node.events.jsonl)).
  *Beyond root* is beyond the program's root, as `root.facts` is ruled to be. Consistent with (a), but the budget a saved node's run spends grows with the node's size.
- **An older binary writing into an `ein-answer/3` directory leaves today's state files beside its own** (five stale
  files, [`over/today-then-a8d.after.tree.txt`](runs/over/today-then-a8d.after.tree.txt)). That is the earlier
  writer's rule (it clears only a schema it knows); noted only because it is one direction of the bump.
- The ledger's statements reproduce: the row, `root.commit` (`[]` on a program), `root.facts` the program root's,
  `index.json`'s `commit` beside `from`, `ein-answer/3`, and the clearing of an `/2` directory.
- **Re-taken with `ein-head-17085b27`** (the first look was `ein-head-b5fb8d70`): nothing moved. The read-outs,
  summaries, `index.json`, events and the `9 facts beyond root` are as they were, timings aside.

## 8. Your call

- **(a)** the `commit` row, `root.commit`, `index.json`'s `commit`, `ein-answer/3` — **the default**.
- **(b)** no row: the read-out of a saved file is the read-out of a program.
