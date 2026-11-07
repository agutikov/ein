# S1f.5.0 — The known wrong answers filed: W29 to W32, before any stage moves them

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)
**Estimate:** 1 day.
- Half a day to bank the witnesses and capture each row's answer today.
- A quarter of a day for the corpus entries and the register's test.
- A quarter of a day for the four stages' step 1 and the record.

**Status:** specified 2026-11-08 on the user's instruction: *"plan adding rows in p1f.5"*, and run the same
day on the user's go-ahead (*"run plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items, each stage
in separate subagent"*). **The twenty rows are filed**: T1f.5.0.1 `0525444e` (the witnesses), T1f.5.0.2
`da58dda6` (the rows), T1f.5.0.3 (this record). **T1f.5.0.4 is next**: three more rows, W33 to W35, which
the user added the same day from S1f.5.5's census. No engine code moved
([§ What was filed](#what-was-filed-2026-11-08)).
**Depends on:** nothing. **It runs first in the phase**, before any stage that moves one of these answers,
so that each row holds today's wrong answer before its fix.

## The instruction

M1e2 closed on 2026-11-08 with about twenty wrong answers known and in no register row. Each is owned by a
stage of this phase, which planned to file its own rows as its step 1. The user's ruling of 2026-11-05,
[Q-M1e2.16](../../m1e2_correctness/open_questions.md#q-m1e216--what-does-the-close-do-with-the-inventorys-items)
(5), files a live wrong answer that has an owner and no row as an `open` row with that owner, so that a
close can name it. The agent asked whether to file them before M1e2's close or later. The user: *"plan adding
rows in p1f.5"*. So this stage files them, all at once and first, and M1e2 closed naming them.

## What a row is

One witness program in [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), with the answer
the engine gives today, the right one, its status and its owner. `ein-cli/tests/wrong_answers.rs` runs every
witness per commit. It holds an `open` row to its wrong answer, so a change to it fails the test, and a
`fixed` row to its right answer.

## The rows

Twenty rows in four cases. The case numbers are this stage's default, in the order of the stages that fix
them. Each row's `today` and `right` are in its owner's stage file, taken on 2026-11-07 at `916d97ac`.

| case | rows | what is wrong today | right | owner |
|---|---:|---|---|---|
| **W29** | 6 | a bare `_` in a fact, an `is-a`, a nested fact, an activator, a goal, and F3's pair, is dropped at load, in silence | refused at load | [S1f.5.1](s1f.5.1_bare_underscore_refused.md#what-it-changes) |
| **W30** | 3 | a rule's concluded `eq` or `neq` is stored as a fact: `(neq ?x ?x)` and `(eq ?x w)` leave ∅ in R, and `(neq w ?x)` adds a fact | a true conclusion adds nothing, and a false one kills the state | [S1f.5.7](s1f.5.7_eq_and_neq_are_relations.md#conclusions-g-21) |
| **W31** | 4 | a rule or hrule whose `:match` is `()` never fires | it fires, once or once per activator | [S1f.5.9](s1f.5.9_an_empty_match_fires.md#the-case) |
| **W32** | 7 | a keyword pair in a term is dropped, so a rule fires without its `:where` or answers a goal without its pair | refused at load | [S1f.5.11](s1f.5.11_closed_keywords.md#the-case) |

W29's six rows are the only ones that reach corpus programs: `syntax/arg-kinds` and `syntax/is-a-shapes`.
The other fourteen are probes.

**Each row:**
- `status = "open"`, owned by its stage, with `open_because` naming the stage that fixes it and the ruling
  behind its `right`: Q-M1e2.22, .26 or .28, or F12's;
- a witness under `examples/ein-bugs/wrong-answers/`, copied from the probe that showed it (the inventory's
  probes under `plans/m1e2_correctness/`, or the stage's own program), never cited in place;
- a `corpus.toml` entry with the runs that show it, and its digests banked.

**Not filed here**, as each stage already says:
- the rows a ruling *restates* rather than adds: W7's, W16's, W20's, W24's and W28's, each by the stage that
  moves its `right`;
- F13's unbound conclusion variable, which stops the run rather than giving an answer, so no row records it
  (S1f.5.10);
- the surfaces (S1f.5.6, S1f.5.8, S1f.5.12, S1f.5.13), whose fixes move no answer.

## What the four stages then do

Their step 1 starts from these rows instead of filing them: it re-takes each row on a fresh build, and every
move a fix makes must be one of them. Each stage still turns its rows `fixed` in the commit that fixes them.

## The method

### Task T1f.5.0.1 — the witnesses, and today's answers

Each witness banked, re-taken on a fresh release build, and its row's `today` captured from the run, not
copied from a stage file. A row whose answer differs from its stage file's is a finding, shown to the user.

### Task T1f.5.0.2 — the rows and their test

The rows, the corpus entries, `wrong_answers.rs` green holding each to its wrong answer, and `./run_tests.sh`
green. Schema `/13` if a field needs it.

### Task T1f.5.0.3 — the record

The four stages' step 1 restated to start from the rows, and the phase README's count of open rows.

### Task T1f.5.0.4 — W33 to W35, from S1f.5.5's census

Added 2026-11-08 on the user's ruling on S1f.5.5's census (*"Q3 - as recommended"*): the three wrong answers
its [§ Findings outside the grammar](s1f.5.5_census.md#findings-outside-the-grammar) found, filed `open` as
the twenty were, each `today` from a run on a fresh build.

| case | row | what is wrong today | right | owner |
|---|---|---|---|---|
| **W33** | `activator-nested-value` | an activator that binds a value parameter to a nested fact leaves it unbound: `(act (f a))` derives `(hit a)`, `(hit b)` and `(hit (f a))` | `(hit (f a))` only | S1f.5.18 |
| **W34** | `assert-empty` | `:assert ()` stores a fact named `@empty`, which joins H: R two states | an empty conclusion asserts nothing (F12): one state | [S1f.5.9](s1f.5.9_an_empty_match_fires.md) |
| **W35** | `absent-empty` | `(absent ())` holds, so the rule fires for every `p` | refused at load (the paper grammar's default 7) | [S1f.5.5](s1f.5.5_the_grammar.md) |

## Acceptance

- The register holds W29 to W32's twenty rows, and W33 to W35's three, each `open` and owned by the stage
  that fixes it.
- `wrong_answers.rs` holds each to the answer the engine gives today.
- No engine code moved.
- `./run_tests.sh` is green.

## What was filed, 2026-11-08

Every row's `today` was taken at `ec004a48`, whose engine is `916d97ac`'s, where the four stage files took
their cases, on a release build up to date with it and the debug build the register's test runs: `ein
saturate FILE --dump`, and `ein solve FILE` under describe, `--minimal` and `--maximal`, release and debug the
same on every witness, and describe and `ein oracle` the same on each. **Every `today` is its stage file's.**
Each witness is under `examples/ein-bugs/wrong-answers/`, with a header giving its hand derivation, and is a
`regression` corpus entry run under `solve` and `saturate`.

| row | witness | `now`, today | `right` |
|---|---|---|---|
| `W29/fact-arg` | `underscore-fact-arg.ein` | `--minimal`: `(model (r a b) (r a))` | refused |
| `W29/is-a-arg` | `underscore-is-a-arg.ein` | `--minimal`: R {∅}; root holds `(is-a T)` | refused |
| `W29/nested-fact-arg` | `underscore-nested-fact-arg.ein` | `--minimal`: `(model (r (f a) c))` | refused |
| `W29/activator-arg` | `underscore-activator-arg.ein` | `--minimal`: root holds `(q)` and no `hit` | refused |
| `W29/goal-arg` | `underscore-goal-arg.ein` | R {∅}; the goal one class, no claim | refused |
| `W29/core-two-ways` | `underscore-core-two-ways.ein`, and `-twin` | R empty; root's core four facts, the twin's two | refused |
| `W30/assert-neq-w-x` | `assert-neq-w-x.ein` | R 2 states, `(neq w a)` and `(neq w w)` stored | R {∅} |
| `W30/assert-neq-x-x` | `assert-neq-x-x.ein` | R {∅}, `(neq a a)` stored | root dead |
| `W30/assert-eq-x-w` | `assert-eq-x-w.ein` | R 2 states, `(eq a w)` stored | root dead |
| `W31/empty-match-false` | `empty-match-false.ein` | `--minimal`: ∅ in R | root dead |
| `W31/empty-match-fact` | `empty-match-fact.ein` | `--minimal`: `(model (q a))` | `(model (q a) (q b))` |
| `W31/empty-match-activator` | `empty-match-activator.ein` | `--minimal`: no `hit` | `(model (hit a))` |
| `W31/empty-match-hrule` | `empty-match-hrule.ein` | H empty, R {∅} | R ∅ and `{(q b)}` |
| `W32/where-in-and` | `where-in-and.ein` | `--minimal`: loads; root holds `(q a) (q b)` | refused |
| `W32/where-in-pattern` | `where-in-pattern.ein` | the same | refused |
| `W32/where-on-rule` | `where-on-rule.ein` | the same | refused |
| `W32/kwpair-in-pattern` | `kwpair-in-pattern.ein` | `--minimal`: `(model (p a) (p b) (q a) (q b))` | refused |
| `W32/kwpair-in-assert` | `kwpair-in-assert.ein` | the same | refused |
| `W32/kwpair-in-goal` | `kwpair-in-goal.ein` | 16 states, one class, the claim held | refused |
| `W32/kwpair-in-macro` | `kwpair-in-macro.ein` | `--minimal`: loads; root holds `(hit a) (hit b)` | refused |

The register then read **103 rows fixed, 30 open and 1 not-wrong**
(`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-08), and
`ein-cli/tests/wrong_answers.rs` holds each open row to its `now`.

**What the stage files did not say**, each recorded in its row and its owner's step 1:
- **A right that differs from its stage file's, put to the user.** `W30/assert-neq-w-x`'s state `{(p w)}`
  also stores the false `(neq w w)`: `w` is a name of the program (a rule's `:assert` names it), so `(p w)` is
  a candidate, and under G-21 that state is dead. The row is filed R {∅}, where
  [S1f.5.7 § Conclusions](s1f.5.7_eq_and_neq_are_relations.md#conclusions-g-21) reads *R 2 states*; R is
  {∅} under either reading of whether `w` is a name. The table stands until the user rules.
- **A stored `neq` writes a state file that does not load.** `--dump-answer` writes `(neq w a)` as a fact
  line, which the parser refuses, so `ein-cli/tests/answer_round_trip.rs` and `answer_dump.rs` name W30's two
  `neq` witnesses (`DO_NOT_LOAD`), each held to fail on every state file and for that reason alone, and
  S1f.5.7 empties both lists. `(eq …)` loads.
- **Three witnesses are not the inventory's probes.** `where_t3_example.ein` (D-15) is refused today for
  its relation variable `?r`, W24's refusal, so it cannot show the drop; `kwpair_in_pattern.ein` (G-15)
  writes a range, which S1f.5.3 retires; and `fact_wild_arg.ein` (W-22) writes `?x`, G-6's. Each row's
  witness is its stage's case program, and its note names the probe.
- **A dropped pair's value is still a name.** On `kwpair-in-pattern` and `kwpair-in-assert` the value `v`
  of the dropped `:k v` is a name of the program, so H has candidates over it and R is 24 states.
- **What moved by banking, outside the register**: `corpus_exits.txt` 42 cells, every one exit 0;
  `corpus_shapes.md5` 861 digests, none removed; `detector.rs`' flagged corpus programs 48 → 49
  (`kwpair-in-macro`, its `forall`'s `absent` over `q`); `alive_invariant.rs`' `SEEDED` 26 → 33 programs
  (`w`, `b`, `v` and `a`, names only a rule or a nested fact writes); `examples/README.md`'s catalog.
- **The corpus entries landed with the witnesses**, in T1f.5.0.1 rather than T1f.5.0.2: a `.ein` with no
  entry fails the completeness check.

**Schema `/13`**, which three rows needed: beside a `right` that is a refusal, a `now` may record the core
and the claims its run reports, what the refusal replaces (`W29/core-two-ways`' core, `W29/goal-arg`'s and
`W32/kwpair-in-goal`'s query), and a refused `now` records none. `ein-corpus`'s
`beside_a_refusal_now_records_what_today_reports` holds it.

`./run_tests.sh` was green before each commit: 13 min 37 s for T1f.5.0.1
(`build/gate-s1f.5.0-T1.log`) and 13 min 33 s for T1f.5.0.2 (`build/gate-s1f.5.0-T2.log`).
