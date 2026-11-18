# Q-M1f.416 — what `ein test` asks of a saved file's node, and how its claims exit

## 1. The question

A saved file carries its program's claims (`(solve … :expect …)`, `(query … :expect …)`), but a file with a
`(commit …)` is a node of the program's lattice, and those claims are about the program's whole R, so on a node they
are never checked: each is NOT CHECKED and the run exits 0 (Q-M1f.403). `ein test` asks each file the least question
that decides its claims. On a node no question decides any of them; so what should `ein test` run, if anything? Today
it runs **root alone, the node** (as `--no-search` does), prints each claim NOT CHECKED with the saved-file line, and
none of them moves the exit code. The alternative is **no run at all**: each claim NOT CHECKED and no run's data in
`--json-report`.

Recorded in [`../../open_questions.md` § Q-M1f.416](../../../open_questions.md#q-m1f416--what-ein-test-asks-of-a-saved-files-node-and-how-its-claims-exit).
The shipped page: [`10-answer-dump.md` § A saved file is a node of its program's lattice](../../../../../docs/ein/user/04-reference/10-answer-dump.md#a-saved-file-is-a-node-of-its-programs-lattice)
(fifth bullet).

## 2. What is so today

Programs: [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein) (copy of
`tests/saved/02_…`: two claims, a `(solve :expect (region (model) (model (note (bad a)))))` and a query on
`(flagged ?x)`; R is two states) and its saved node
[`programs/04_a_saved_node.ein`](programs/04_a_saved_node.ein) (copy of `tests/saved/04_a_saved_node.ein`, the state
`{(note (bad a))}` saved; byte for byte what `ein solve 02 --dump-answer` writes as `models/02.ein`,
[`check.node-is-the-programs-save.txt`](runs/check.node-is-the-programs-save.txt)). Binary:
`build/review-bin/ein-head-b5fb8d70`. Exit codes: [`00-exit-codes.txt`](runs/00-exit-codes.txt).

**The program: both claims hold, exit 0** ([`program.test-v.txt`](runs/program.test-v.txt); an excerpt, `…` marks the *outside the universe* lines left out):

<!-- output -->
```text
ok           programs/02_a_choice_with_a_fact_inside_it.ein
  solve · :expect (region …) — holds (state route, k_s = 2, k_c = 2)
  …
  :goal (flagged ?x) — holds (describe: all of R, 2 classes over 2 states)
  …
1 file, 2 expectations: 2 held, 0 FAILED, 0 not checked, 0 errors  (0.00 s)
```

**The saved node: both claims NOT CHECKED, exit 0** ([`node.test-v.txt`](runs/node.test-v.txt); plain
[`node.test.txt`](runs/node.test.txt), quiet [`node.test-q.txt`](runs/node.test-q.txt)):

<!-- output -->
```text
NOT CHECKED  programs/04_a_saved_node.ein
  solve · :expect (region …) — NOT CHECKED (a saved file's node: root alone)
    the claim is the program's, about the program's R, and this saved file is a node of its lattice: the program checks it
    outside the universe: flagged — no candidate: no hrule proposes it. R holds of it only what the program states and saturation derives
  :goal (flagged ?x) — NOT CHECKED (--no-search: all of R, 1 class over 1 states)
    the claim is the program's, about the program's R, and this saved file is a node of its lattice: the program checks it
    outside the universe: flagged — no candidate: no hrule proposes it. R holds of it only what the program states and saturation derives
1 file, 2 expectations: 0 held, 0 FAILED, 2 not checked, 0 errors  (0.00 s)
```

**What is run is root alone, the node, one saturation.** The report's rows say `region.route` `"root alone"` and
`route` `"--no-search"` ([`node.test.report.json`](runs/node.test.report.json)); the summary says
`asked.ask` `"no-search"` and `region` `null` ([`node.test.summary.json`](runs/node.test.summary.json), against the
program's in [`diff.summary.program-vs-node.diff`](runs/diff.summary.program-vs-node.diff)); the events are one
saturation, the choice committed, `flag` firing, one `enter` at layer 0 and a `region` with `ask` `no-search`
([`node.test.events.md`](runs/node.test.events.md), against the program's
[`program.test.events.md`](runs/program.test.events.md)).

**It is `--no-search`'s run with the saved-file reason.** The program under `ein test --no-search` and the node under
`ein test` differ in the path, the solve claim's parenthesis and the reason line
([`diff.test-v.program-no-search-vs-node.diff`](runs/diff.test-v.program-no-search-vs-node.diff)); their reports differ
in the path and a timing ([`diff.report.program-no-search-vs-node.diff`](runs/diff.report.program-no-search-vs-node.diff)).
`--no-search` on the node changes nothing ([`diff.test-v.node-vs-node-no-search.diff`](runs/diff.test-v.node-vs-node-no-search.diff),
empty).

**A claim not checked moves no exit code.** A copy of the program with its region claim made wrong,
[`programs/02_wrong_claim.ein`](programs/02_wrong_claim.ein), FAILS under `ein test`, exit 1
([`wrong.test-v.txt`](runs/wrong.test-v.txt)); its saved node [`runs/wrong.dump/models/02.ein`](runs/wrong.dump/models/02.ein)
is NOT CHECKED, exit 0 ([`wrong-node.test-v.txt`](runs/wrong-node.test-v.txt)); the two in one selection exit 1, from
the program alone ([`both.test.txt`](runs/both.test.txt): `1 held, 1 FAILED, 2 not checked`).

**Root's saved file is the program.** `ein test` of `models/01.ein` (no `(commit …)`) checks both claims as the
program does, both hold ([`saved-root.test-v.txt`](runs/saved-root.test-v.txt)).

**The node is still loaded and checked as a node.** A copy of the node whose `(commit …)` names `(note (bad b))`, no
candidate of H ([`no-candidate.ein`](runs/no-candidate.ein)), is an ERROR, exit 2, the command's load-error exit
([`no-candidate.test.txt`](runs/no-candidate.test.txt)).

**`ein solve` of the same node** describes the part of R above it (one state) and names both claims NOT CHECKED, exit
0, with a `note:` each on stderr ([`node.solve.txt`](runs/node.solve.txt), [`node.solve.stderr.txt`](runs/node.solve.stderr.txt));
with `--no-search`, [`node.solve-no-search.txt`](runs/node.solve-no-search.txt). The program's `--no-search`
read-out for comparison: [`program.solve-no-search.txt`](runs/program.solve-no-search.txt).

**The earlier binary (`ein-a8d595eb`).** Its state file of the same node is the state read as a program
([`a8d.dump/models/02.ein`](runs/a8d.dump/models/02.ein)), so `ein test` checks the program's claims against R = `{∅}`
of that program, and both FAIL, exit 1 ([`a8d.state2.test-v.txt`](runs/a8d.state2.test-v.txt)).

## 3. The options

- **(a) as built — root alone, the node.** One saturation; each claim NOT CHECKED with Q-M1f.403's line, none moving
  the exit, as under `--no-search`; the verbose header says *(a saved file's node: root alone)*; the report carries the
  run's data (`region`, `answer`, `outside`, `route`). All of § 2.
- **(b) no run at all.** Each claim NOT CHECKED, no row of a run in `--json-report`. **Mocks**, hand-written:
  - `ein test -v` of the node: [`node.test-v.MOCK.option-b.txt`](runs/node.test-v.MOCK.option-b.txt), today's output
    with both parentheses replaced by *(a saved file's node: not run)* and the two *outside the universe* lines removed,
    since no saturation reads the universe ([`node.test-v.today-vs.MOCK.option-b.diff`](runs/node.test-v.today-vs.MOCK.option-b.diff)).
    The tally and the exit, 0, are the same.
  - `--json-report`: [`node.test.report.MOCK.option-b.json`](runs/node.test.report.MOCK.option-b.json), today's rows
    with `region`, `answer` and `route` set to `null` and `outside` to `[]`
    ([`node.test.report.today-vs.MOCK.option-b.diff`](runs/node.test.report.today-vs.MOCK.option-b.diff)).
  - Not mocked, read, not run: what `--json-summary` (*the file's run, the last question it was asked*) and `--events`
    would write with no run is not said by the option; the events after `load` (the saturation, `enter`, `region`)
    would be absent. A node whose `(commit …)` names no candidate is refused at the load under (a); whether (b) loads
    the file at all is also not said.

What neither option changes: the claims are NOT CHECKED, the exit code is 0 unless something else in the selection
fails or errors.

## 4. The default

**(a)**, from the ledger: `ein test` asks a file the least question that decides its claims; no question about the node
decides a claim of its program, so the least is root's, and the run is the one `--no-search` makes. Built in
`ein-cli/src/test.rs`; held by `saved_files.rs` and `test_cli.rs`' `the_verbose_header_and_the_report_row_agree_field_for_field`
on `tests/saved/04_a_saved_node.ein`.

## 5. Files

- [`rerun.sh`](rerun.sh) — every command below, in order, and the mocks' diffs; the mocks themselves are not touched by it.
- [`programs/02_a_choice_with_a_fact_inside_it.ein`](programs/02_a_choice_with_a_fact_inside_it.ein) — copy of `tests/saved/02_a_choice_with_a_fact_inside_it.ein`.
- [`programs/04_a_saved_node.ein`](programs/04_a_saved_node.ein) — copy of `tests/saved/04_a_saved_node.ein`, its saved node.
- [`programs/02_wrong_claim.ein`](programs/02_wrong_claim.ein) — written for this review: 02 with its region claim made wrong.
- [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) — every command run, with its exit code.
- [`runs/program.test.txt`](runs/program.test.txt), [`runs/program.test-v.txt`](runs/program.test-v.txt), [`runs/program.test-q.txt`](runs/program.test-q.txt) — `ein test` of the program: plain, `-v`, `-q`.
- [`runs/program.test.report.json`](runs/program.test.report.json), [`runs/program.test.summary.json`](runs/program.test.summary.json), [`runs/program.test.events.jsonl`](runs/program.test.events.jsonl), [`.md`](runs/program.test.events.md) — its report, summary, events (raw and as a table).
- [`runs/node.test.txt`](runs/node.test.txt), [`runs/node.test-v.txt`](runs/node.test-v.txt), [`runs/node.test-q.txt`](runs/node.test-q.txt) — `ein test` of the saved node: plain, `-v`, `-q`.
- [`runs/node.test.report.json`](runs/node.test.report.json), [`runs/node.test.summary.json`](runs/node.test.summary.json), [`runs/node.test.events.jsonl`](runs/node.test.events.jsonl), [`.md`](runs/node.test.events.md) — its report, summary, events.
- [`runs/diff.test-v.program-vs-node.diff`](runs/diff.test-v.program-vs-node.diff), [`runs/diff.report.program-vs-node.diff`](runs/diff.report.program-vs-node.diff), [`runs/diff.summary.program-vs-node.diff`](runs/diff.summary.program-vs-node.diff) — program vs node: `-v`, report, summary.
- [`runs/program.test-v-no-search.txt`](runs/program.test-v-no-search.txt), [`runs/program.test-no-search.report.json`](runs/program.test-no-search.report.json) — the program under `ein test --no-search`.
- [`runs/node.test-v-no-search.txt`](runs/node.test-v-no-search.txt), [`runs/node.test-no-search.report.json`](runs/node.test-no-search.report.json) — the node under `ein test --no-search`.
- [`runs/diff.test-v.program-no-search-vs-node.diff`](runs/diff.test-v.program-no-search-vs-node.diff), [`runs/diff.report.program-no-search-vs-node.diff`](runs/diff.report.program-no-search-vs-node.diff) — the program under `--no-search` vs the node.
- [`runs/diff.test-v.node-vs-node-no-search.diff`](runs/diff.test-v.node-vs-node-no-search.diff) — the node with and without `--no-search` (empty).
- [`runs/program.solve.txt`](runs/program.solve.txt), [`runs/program.dump/`](runs/program.dump/) — the program's read-out and its save ([`index.json`](runs/program.dump/index.json), [`answer.txt`](runs/program.dump/answer.txt), [`answer.json`](runs/program.dump/answer.json), [`nogoods.json`](runs/program.dump/nogoods.json), [`models/01.ein`](runs/program.dump/models/01.ein), [`models/02.ein`](runs/program.dump/models/02.ein)).
- [`runs/check.node-is-the-programs-save.txt`](runs/check.node-is-the-programs-save.txt) — the node file compared with that save's `models/02.ein`.
- [`runs/node.solve.txt`](runs/node.solve.txt), [`.stderr.txt`](runs/node.solve.stderr.txt), [`runs/node.solve-no-search.txt`](runs/node.solve-no-search.txt), [`.stderr.txt`](runs/node.solve-no-search.stderr.txt) — `ein solve` of the node, and with `--no-search`.
- [`runs/program.solve-no-search.txt`](runs/program.solve-no-search.txt), [`.stderr.txt`](runs/program.solve-no-search.stderr.txt) — `ein solve --no-search` of the program.
- [`runs/saved-root.test-v.txt`](runs/saved-root.test-v.txt) — `ein test -v` of the saved root file: both claims hold.
- [`runs/wrong.test-v.txt`](runs/wrong.test-v.txt) — `ein test -v` of the wrong-claim program: FAILED, exit 1.
- [`runs/wrong.solve.txt`](runs/wrong.solve.txt), [`.stderr.txt`](runs/wrong.solve.stderr.txt), [`runs/wrong.dump/`](runs/wrong.dump/) — its read-out and save ([`models/02.ein`](runs/wrong.dump/models/02.ein) the node).
- [`runs/wrong-node.test-v.txt`](runs/wrong-node.test-v.txt), [`runs/wrong-node.test.report.json`](runs/wrong-node.test.report.json) — `ein test` of its node: NOT CHECKED, exit 0.
- [`runs/both.test.txt`](runs/both.test.txt) — the wrong program and its node in one selection: exit 1.
- [`runs/no-candidate.ein`](runs/no-candidate.ein), [`runs/no-candidate.test.txt`](runs/no-candidate.test.txt), [`.stderr.txt`](runs/no-candidate.test.stderr.txt) — a node naming no candidate: ERROR, exit 2.
- [`runs/a8d.program.solve.txt`](runs/a8d.program.solve.txt), [`runs/a8d.dump/`](runs/a8d.dump/) — the earlier binary's read-out and save of 02 (`ein-answer/2`; [`models/02.ein`](runs/a8d.dump/models/02.ein) its state file).
- [`runs/a8d.state2.test-v.txt`](runs/a8d.state2.test-v.txt) — the earlier binary's `ein test -v` of that state file: both claims FAILED, exit 1.
- [`runs/node.test-v.MOCK.option-b.txt`](runs/node.test-v.MOCK.option-b.txt), [`runs/node.test-v.today-vs.MOCK.option-b.diff`](runs/node.test-v.today-vs.MOCK.option-b.diff) — **mock**, (b): `ein test -v` of the node, and its edits.
- [`runs/node.test.report.MOCK.option-b.json`](runs/node.test.report.MOCK.option-b.json), [`runs/node.test.report.today-vs.MOCK.option-b.diff`](runs/node.test.report.today-vs.MOCK.option-b.diff) — **mock**, (b): its `--json-report`, and its edits.

## 6. Re-run

From the repo root, `bash plans/m1f_hypothesis_and_documentation/review/01-save-and-load/19-Q-M1f.416/rerun.sh`. The core commands:

```sh
B=/home/user/work/ein/build/review-bin/ein-head-b5fb8d70
cd plans/m1f_hypothesis_and_documentation/review/01-save-and-load/19-Q-M1f.416
$B test -v programs/02_a_choice_with_a_fact_inside_it.ein --json-report runs/program.test.report.json
$B test -v programs/04_a_saved_node.ein --json-report runs/node.test.report.json \
   --json-summary runs/node.test.summary.json --events runs/node.test.events.jsonl      # exit 0
$B test -v --no-search programs/02_a_choice_with_a_fact_inside_it.ein
$B solve programs/02_wrong_claim.ein --dump-answer runs/wrong.dump                     # exit 1
$B test -v runs/wrong.dump/models/02.ein                                                # exit 0
EIN_STDLIB=/home/user/work/ein/build/wt-rev-a8d/stdlib /home/user/work/ein/build/review-bin/ein-a8d595eb \
   test -v runs/a8d.dump/models/02.ein                                                  # exit 1
```

## 7. Found while building

- **The query's verbose header names `--no-search`, which was not given.** On the node, the solve claim's line says
  *(a saved file's node: root alone)*, as the ledger says, but the query claim's says
  *(--no-search: all of R, 1 class over 1 states)* ([`node.test-v.txt`](runs/node.test-v.txt)); the report's `route`
  is `"--no-search"` on both rows ([`node.test.report.json`](runs/node.test.report.json)). The ledger's sentence
  *the verbose header says (a saved file's node: root alone)* is true of the solve claim only. `test_cli.rs` rebuilds
  the header from the row, so it holds this as it is.
- **The query's answer reads *all of R* over the node alone.** The same row's `answer` is
  `{"over": "all", "exact": false, "classes": 1, "states": 1}`: the node's one state, labelled all of R, `exact`
  false ([`node.test.report.json`](runs/node.test.report.json)). It is the `--no-search` row's shape on a program
  ([`program.test-no-search.report.json`](runs/program.test-no-search.report.json)).

## 8. Your call

- **(a)** root alone, the node: one saturation, each claim NOT CHECKED, no exit moved — **the default**.
- **(b)** no run at all: each claim NOT CHECKED, no run's data in `--json-report`.
