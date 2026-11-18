# Q-M1f.415 — `ein show hyps` and `ein render h-space` of a saved file

## 1. The question

A saved file is a program plus one commitment of its H (a `(commit …)`), and `ein solve` reads it as that node of the
program's lattice: the run's universe is H **less** the node's choices, and its R is the part of the program's R above
the node. Two commands read hypotheses without searching: `ein show hyps` (H as Ein facts, one per line) and
`ein render h-space` (the structure of H: pairs that exclude each other, groups, the coupling of the questions). Given a
saved node file, should these two read **the program's H** (the node aside; today), or **the run's universe above the
node**, H less the choices, as `ein solve -H` of the same file lists it? The difference matters most for h-space: a
structure that only appears once a choice is made (a pair that excludes *at the node*) is shown by the second and not by
the first.

Recorded in [`../../open_questions.md` § Q-M1f.415](../../../open_questions.md#q-m1f415--ein-show-hyps-and-ein-render-h-space-of-a-saved-file).
The shipped page: [`10-answer-dump.md` § A saved file is a node of its program's lattice](../../../../../docs/ein/user/04-reference/10-answer-dump.md#a-saved-file-is-a-node-of-its-programs-lattice)
(fourth bullet).

## 2. What is so today

Program: [`programs/02_genuine_3set_death.ein`](programs/02_genuine_3set_death.ein), a copy of
`examples/lattice/02_genuine_3set_death.ein`. H is three candidates, `(a-prop X)`, `(b-prop X)`, `(c-prop X)`; a rule
kills a state that holds all three, so every pair is alive and R has seven states
([`program.solve.txt`](runs/program.solve.txt)). The saved node is state 2,
[`runs/program.dump/models/02.ein`](runs/program.dump/models/02.ein), whose file ends `(commit (a-prop X))`. Binary:
`build/review-bin/ein-head-b5fb8d70`; every exit is 0 ([`00-exit-codes.txt`](runs/00-exit-codes.txt)).

**`ein show hyps`: the node file prints the program's H, all three, byte for byte**
([`node.show-hyps.txt`](runs/node.show-hyps.txt); the diff with the program's,
[`diff.show-hyps.program-vs-node.diff`](runs/diff.show-hyps.program-vs-node.diff), is empty). Its stderr note differs
in the path only ([`diff.show-hyps-stderr.program-vs-node.diff`](runs/diff.show-hyps-stderr.program-vs-node.diff)):

<!-- output -->
```text
(a-prop X)
(b-prop X)
(c-prop X)
```

The node's choice, `(a-prop X)`, is among them and is not marked: the line is the same as on the program.

**`ein render h-space`: the node file draws the program's structure**, every surface differing from the program's only
in the path it names: the read-out ([`diff.h-space.program-vs-node.diff`](runs/diff.h-space.program-vs-node.diff)),
the read-out with `--supports`
([`diff.h-space-supports.program-vs-node.diff`](runs/diff.h-space-supports.program-vs-node.diff)), `--dump`'s
`h-space.json` and four graphs ([`diff.h-space-dump.program-vs-node.diff`](runs/diff.h-space-dump.program-vs-node.diff)),
and each `--view` DOT (`diff.h-space-{exclusion,cover,coupling,one-step}-dot.program-vs-node.diff`). Each `--view` DOT
is the file `--dump` writes for it ([`check.view-vs-dump.txt`](runs/check.view-vs-dump.txt)). The node's read-out, an excerpt (`…` marks lines left out):

<!-- output -->
```text
h-space      runs/program.dump/models/02.ein
H            3 candidates · 0 dead alone · 0 at root · built by the rung `hrules`
pairs        3 · 3 saturated · 0 excluded at root · 0 interacting · 0 implications · 0 equivalence classes
…
coupling     3 questions · 0 ties · 1 edge (1 of three questions or more, 0 a debt's alone) · 1 edge one application back · 0 debts that are edges (0 unlicensed)
```

The drawings: the exclusion graph is empty
([`renders/node.h-space.exclusion.svg`](renders/node.h-space.exclusion.svg)); the coupling hypergraph has one edge
over the three questions, the three-way rule ([`renders/node.h-space.coupling.svg`](renders/node.h-space.coupling.svg));
the cover is empty ([`renders/node.h-space.cover.svg`](renders/node.h-space.cover.svg)). The program's drawings are the
same, `renders/program.h-space.*.svg`.

**What a run of the node reads instead.** `ein solve runs/program.dump/models/02.ein -H` lists the run's universe, H
less the choice ([`node.solve-H.txt`](runs/node.solve-H.txt), against the program's in
[`diff.solve-H.program-vs-node.diff`](runs/diff.solve-H.program-vs-node.diff)):

<!-- output -->
```text
H — the universe: 2 candidates over 2 relations, 0 dead alone
  b-prop          1 candidate, 0 dead alone
    (b-prop X)
  c-prop          1 candidate, 0 dead alone
    (c-prop X)
```

and the summary says `root.commit` = `["(a-prop X)"]`, `region.universe.candidates` = 2 and `root.hypgen.emitted` = 3
([`node.summary.json`](runs/node.summary.json)). So on one saved file today, `ein solve -H` lists two candidates and
`ein show hyps` three.

## 3. The options

- **(a) as built**: both read the program's H, the node aside. Everything in § 2.
- **(b) both read the run's universe**, H less the choices, as `ein solve -H` lists it. **Mocks** below. To make them
  without changing code, a program was written whose root is the node's state and whose H is H less the choice:
  [`programs/variant_a_prop_stated.ein`](programs/variant_a_prop_stated.ein), the corpus file with `(a-prop X)` stated
  as a fact and its `guess` instance dropped from `(solve :hrules …)`. Today's binary was run on it
  (`runs/variant.*`, `renders/variant.*`), and each output's path was replaced by the saved file's; nothing else was
  edited ([`edits.MOCK.option-b.diff`](runs/edits.MOCK.option-b.diff)).
  - `ein show hyps` of the node would print two lines
    ([`node.show-hyps.MOCK.option-b.txt`](runs/node.show-hyps.MOCK.option-b.txt), against today in
    [`node.show-hyps.today-vs.MOCK.option-b.diff`](runs/node.show-hyps.today-vs.MOCK.option-b.diff)): `(b-prop X)`,
    `(c-prop X)`, what `ein solve -H` of the file lists.
  - `ein render h-space` of the node would show **a pair excluded at the node**, `(b-prop X)`–`(c-prop X)`, which the
    program's structure has no pair for: with `(a-prop X)` committed, the two together reach the three-way rule
    ([`node.h-space.MOCK.option-b.txt`](runs/node.h-space.MOCK.option-b.txt), against today in
    [`node.h-space.today-vs.MOCK.option-b.diff`](runs/node.h-space.today-vs.MOCK.option-b.diff);
    the exclusion graph [`renders/node.h-space.exclusion.MOCK.option-b.svg`](renders/node.h-space.exclusion.MOCK.option-b.svg),
    its DOT against today's in [`node.h-space-exclusion-dot.today-vs.MOCK.option-b.diff`](runs/node.h-space-exclusion-dot.today-vs.MOCK.option-b.diff);
    the coupling, now one tie between two questions,
    [`renders/node.h-space.coupling.MOCK.option-b.svg`](renders/node.h-space.coupling.MOCK.option-b.svg); the document
    [`node.h-space.MOCK.option-b.json`](runs/node.h-space.MOCK.option-b.json)).
  - **What the mock cannot decide.** In the variant `(a-prop X)` is a stated fact, so the pair is counted as *the
    puzzle's* (dashed, `1 the puzzle's`) and the small KB *drops 1 of 3 stated facts*. In a saved node the choice is a
    hypothesis, not a statement; whether (b) would count the pair as the puzzle's, the theory's or a third kind, and
    whether the note would still say *H, built by the rung `hrules`: 2 candidates* rather than name the universe as H
    less the node's choice, is for (b) to define.

## 4. The default

**(a)**, from the ledger: Q-M1f.402 (C) says *H is the program's*, and a run of a saved file is over H less the node's
choices; both commands describe H, so they read the program's. Built in `ein-cli/src/common.rs`' `program_of_saved`;
held by `saved_files.rs`' `a_choice_is_saved_as_a_commitment_and_loaded_as_the_node` (which checks `ein show hyps` of
`tests/saved/02`'s node prints `(note (bad a))`, its one candidate, which is also its choice).

## 5. Files

- [`rerun.sh`](rerun.sh) — every command below, in order, and the mocks' diffs; the mocks themselves are not touched by it.
- [`programs/02_genuine_3set_death.ein`](programs/02_genuine_3set_death.ein) — copy of `examples/lattice/02_genuine_3set_death.ein`.
- [`programs/variant_a_prop_stated.ein`](programs/variant_a_prop_stated.ein) — written for this review: the same with the node's choice stated and its hrule instance dropped; run only to make (b)'s mocks.
- [`runs/00-exit-codes.txt`](runs/00-exit-codes.txt) — every command run, with its exit code.
- [`runs/program.solve.txt`](runs/program.solve.txt) — the program's read-out, the run that saved `program.dump`.
- [`runs/program.dump/`](runs/program.dump/) — that save: [`index.json`](runs/program.dump/index.json), [`answer.txt`](runs/program.dump/answer.txt), [`answer.json`](runs/program.dump/answer.json), [`nogoods.json`](runs/program.dump/nogoods.json), `models/01.ein` … `models/07.ein`, of which [`models/02.ein`](runs/program.dump/models/02.ein) is the node read here.
- [`runs/program.show-hyps.txt`](runs/program.show-hyps.txt), [`.stderr.txt`](runs/program.show-hyps.stderr.txt) — `ein show hyps` of the program.
- [`runs/node.show-hyps.txt`](runs/node.show-hyps.txt), [`.stderr.txt`](runs/node.show-hyps.stderr.txt) — `ein show hyps` of the node.
- [`runs/diff.show-hyps.program-vs-node.diff`](runs/diff.show-hyps.program-vs-node.diff) — their stdout diffed (empty).
- [`runs/diff.show-hyps-stderr.program-vs-node.diff`](runs/diff.show-hyps-stderr.program-vs-node.diff) — their stderr diffed.
- [`runs/program.h-space.txt`](runs/program.h-space.txt), [`runs/node.h-space.txt`](runs/node.h-space.txt) — `ein render h-space` read-outs.
- [`runs/program.h-space.supports.txt`](runs/program.h-space.supports.txt), [`runs/node.h-space.supports.txt`](runs/node.h-space.supports.txt) — the same with `--supports`.
- [`runs/program.h-space.dump/`](runs/program.h-space.dump/), [`runs/node.h-space.dump/`](runs/node.h-space.dump/) — `--dump`: `h-space.json`, `exclusion.dot`, `cover.dot`, `coupling.dot`, `coupling_one_step.dot`.
- [`runs/diff.h-space.program-vs-node.diff`](runs/diff.h-space.program-vs-node.diff), [`runs/diff.h-space-supports.program-vs-node.diff`](runs/diff.h-space-supports.program-vs-node.diff), [`runs/diff.h-space-dump.program-vs-node.diff`](runs/diff.h-space-dump.program-vs-node.diff) — program vs node: read-out, with supports, the dump directories.
- [`runs/diff.h-space-exclusion-dot.program-vs-node.diff`](runs/diff.h-space-exclusion-dot.program-vs-node.diff), [`cover`](runs/diff.h-space-cover-dot.program-vs-node.diff), [`coupling`](runs/diff.h-space-coupling-dot.program-vs-node.diff), [`one-step`](runs/diff.h-space-one-step-dot.program-vs-node.diff) — program vs node, each `--view` DOT.
- [`runs/check.view-vs-dump.txt`](runs/check.view-vs-dump.txt) — each `--view` DOT compared with `--dump`'s file.
- [`runs/program.solve-H.txt`](runs/program.solve-H.txt), [`runs/node.solve-H.txt`](runs/node.solve-H.txt), [`runs/diff.solve-H.program-vs-node.diff`](runs/diff.solve-H.program-vs-node.diff) — `ein solve -H`: the program's universe, the node run's universe, diffed.
- [`runs/node.summary.json`](runs/node.summary.json) — the node run's `--json-summary` (`root.commit`, `region.universe`).
- [`runs/variant.show-hyps.txt`](runs/variant.show-hyps.txt), [`.stderr.txt`](runs/variant.show-hyps.stderr.txt), [`runs/variant.h-space.txt`](runs/variant.h-space.txt), [`runs/variant.h-space.dump/`](runs/variant.h-space.dump/), [`runs/variant.solve-H.txt`](runs/variant.solve-H.txt) — the variant program's real outputs, the mocks' source.
- [`runs/node.show-hyps.MOCK.option-b.txt`](runs/node.show-hyps.MOCK.option-b.txt), [`.stderr.txt`](runs/node.show-hyps.MOCK.option-b.stderr.txt) — **mock**, (b): `ein show hyps` of the node.
- [`runs/node.h-space.MOCK.option-b.txt`](runs/node.h-space.MOCK.option-b.txt), [`runs/node.h-space.MOCK.option-b.json`](runs/node.h-space.MOCK.option-b.json) — **mock**, (b): `ein render h-space` of the node, read-out and document.
- [`runs/edits.MOCK.option-b.diff`](runs/edits.MOCK.option-b.diff) — the edits that made every mock from the variant's outputs (the path only).
- [`runs/node.show-hyps.today-vs.MOCK.option-b.diff`](runs/node.show-hyps.today-vs.MOCK.option-b.diff), [`runs/node.h-space.today-vs.MOCK.option-b.diff`](runs/node.h-space.today-vs.MOCK.option-b.diff), [`runs/node.h-space-json.today-vs.MOCK.option-b.diff`](runs/node.h-space-json.today-vs.MOCK.option-b.diff), [`runs/node.h-space-exclusion-dot.today-vs.MOCK.option-b.diff`](runs/node.h-space-exclusion-dot.today-vs.MOCK.option-b.diff), [`runs/node.h-space-coupling-dot.today-vs.MOCK.option-b.diff`](runs/node.h-space-coupling-dot.today-vs.MOCK.option-b.diff) — today against (b)'s mock, per surface.
- `renders/program.h-space.{exclusion,cover,coupling,one-step}.{dot,svg}` — the program's four graphs: [exclusion](renders/program.h-space.exclusion.svg), [cover](renders/program.h-space.cover.svg), [coupling](renders/program.h-space.coupling.svg), [one-step](renders/program.h-space.one-step.svg) ([`.dot`](renders/program.h-space.exclusion.dot), [`.dot`](renders/program.h-space.cover.dot), [`.dot`](renders/program.h-space.coupling.dot), [`.dot`](renders/program.h-space.one-step.dot)).
- `renders/node.h-space.{exclusion,cover,coupling,one-step}.{dot,svg}` — the node's, today: [exclusion](renders/node.h-space.exclusion.svg), [cover](renders/node.h-space.cover.svg), [coupling](renders/node.h-space.coupling.svg), [one-step](renders/node.h-space.one-step.svg) ([`.dot`](renders/node.h-space.exclusion.dot), [`.dot`](renders/node.h-space.cover.dot), [`.dot`](renders/node.h-space.coupling.dot), [`.dot`](renders/node.h-space.one-step.dot)).
- `renders/variant.h-space.{exclusion,cover,coupling,one-step}.{dot,svg}` — the variant's real graphs: [exclusion](renders/variant.h-space.exclusion.svg), [cover](renders/variant.h-space.cover.svg), [coupling](renders/variant.h-space.coupling.svg), [one-step](renders/variant.h-space.one-step.svg) ([`.dot`](renders/variant.h-space.exclusion.dot), [`.dot`](renders/variant.h-space.cover.dot), [`.dot`](renders/variant.h-space.coupling.dot), [`.dot`](renders/variant.h-space.one-step.dot)).
- `renders/node.h-space.{exclusion,cover,coupling,one-step}.MOCK.option-b.{dot,svg}` — **mocks**, (b): the node's graphs: [exclusion](renders/node.h-space.exclusion.MOCK.option-b.svg), [cover](renders/node.h-space.cover.MOCK.option-b.svg), [coupling](renders/node.h-space.coupling.MOCK.option-b.svg), [one-step](renders/node.h-space.one-step.MOCK.option-b.svg) ([`.dot`](renders/node.h-space.exclusion.MOCK.option-b.dot), [`.dot`](renders/node.h-space.cover.MOCK.option-b.dot), [`.dot`](renders/node.h-space.coupling.MOCK.option-b.dot), [`.dot`](renders/node.h-space.one-step.MOCK.option-b.dot)).

## 6. Re-run

From the repo root, `bash plans/m1f_hypothesis_and_documentation/review/01-save-and-load/18-Q-M1f.415/rerun.sh`. The core commands:

```sh
B=/home/user/work/ein/build/review-bin/ein-head-b5fb8d70
cd plans/m1f_hypothesis_and_documentation/review/01-save-and-load/18-Q-M1f.415
$B solve programs/02_genuine_3set_death.ein --dump-answer runs/program.dump
$B show hyps programs/02_genuine_3set_death.ein
$B show hyps runs/program.dump/models/02.ein
$B render h-space programs/02_genuine_3set_death.ein --dump runs/program.h-space.dump
$B render h-space runs/program.dump/models/02.ein --dump runs/node.h-space.dump
$B render h-space runs/program.dump/models/02.ein --view exclusion > renders/node.h-space.exclusion.dot
dot -Tsvg renders/node.h-space.exclusion.dot -o renders/node.h-space.exclusion.svg
$B solve runs/program.dump/models/02.ein -H --json-summary runs/node.summary.json
$B render h-space programs/variant_a_prop_stated.ein          # the source of (b)'s mocks
```

## 7. Found while building

- **Two counts of one file's H.** On the same saved node, `ein solve -H` lists two candidates and `ein show hyps`
  three ([`node.solve-H.txt`](runs/node.solve-H.txt), [`node.show-hyps.txt`](runs/node.show-hyps.txt)). That is (a) as
  ruled, but neither surface says which of the three is the node's choice, nor that `-H`'s list is H less it beyond the
  `commit` row: a reader of `show hyps` alone cannot tell the node's choice from the file's output.
- The ledger's statements reproduce: both commands print, on the node, what they print on the program, but for the
  path.

## 8. Your call

- **(a)** both read the program's H, the node aside — **the default**.
- **(b)** both read the run's universe, H less the node's choices, as `ein solve -H` lists it.
