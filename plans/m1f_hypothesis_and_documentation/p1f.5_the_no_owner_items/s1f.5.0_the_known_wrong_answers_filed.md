# S1f.5.0 — The known wrong answers filed: W29 to W32, before any stage moves them

**Phase:** [P1f.5](README.md) (The no-owner items, then NL and `:priority`)
**Estimate:** 1 day.
- Half a day to bank the witnesses and capture each row's answer today.
- A quarter of a day for the corpus entries and the register's test.
- A quarter of a day for the four stages' step 1 and the record.

**Status:** specified 2026-11-08 on the user's instruction: *"plan adding rows in p1f.5"*.
**Not started, and not to start until the user says so.** No engine code, test, golden or fixture has
moved.
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

## Acceptance

- The register holds W29 to W32's twenty rows, each `open` and owned by the stage that fixes it.
- `wrong_answers.rs` holds each to the answer the engine gives today.
- No engine code moved.
- `./run_tests.sh` is green.
