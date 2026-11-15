# S1f.20.8 — The decisions, in `docs/decisions/`

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Status:** planned 2026-11-15 on the user's rulings of that day
([P1f.20 § The user's rulings](README.md#the-users-rulings-2026-11-15)). Not started.
**Depends on:** [S1f.20.2](s1f.20.2_the_disposition_table.md), whose review it joins: the decisions' layout
is shown to the user with the table and the tree, before anything moves.
**Runs before** [S1f.20.3](s1f.20.3_user.md), [S1f.20.4](s1f.20.4_reasoning.md) and
[S1f.20.5](s1f.20.5_ein_rs.md), so that a page they rewrite cites a decision by its id rather than a plan.
**First half of the history sweep**; its second half is [S1f.20.9](s1f.20.9_the_history_sweep.md).

## What it builds

**`docs/decisions/`**: every decision the project has taken, **the final one only**, as the current state of
the project. A decision is a question that has an answer: ruled by the user, or a default that stands. Its
source is every question ledger in the tree:

- the milestone ledgers, `plans/*/open_questions.md` and [`plans/open_questions.md`](../../open_questions.md);
- the phase ledgers, such as P1f.6's `questions.md`, `questions_round2.md` and the replies;
- the Q rows of `plans/followups/` and of `plans/ideas/`;
- the records under `docs/history/`, whose ledgers (`Q-M1a.n` to `Q-M1e.n`) leave with that directory.

**What a decision file holds**: its id, a title that states the question as a statement, the decision, a
short reason, and what holds it (a test, a command, a page of `docs/ein/`). **It holds no history**: no option
not taken, no default later revised, no date, no name, no stage, phase or plan, no link into `plans/`.

**Ids keep their numbering** (the user: *"Q-M1f.304 remain while M1f completely gone"*). A decision's id is
its name, not a pointer: nothing resolves `Q-M1f.304` to a milestone once M1f's folder is gone.

**What it does not carry**: a question with no answer and no default stays in `plans/`, a work item; a
superseded, moot or reversed row is not a decision (only the one that stands is).

## The defaults, each the user's to revise at S1f.20.2's review

1. **One file a decision**, `docs/decisions/<subject>/<id>-<slug>.md`, in folders by subject (the language,
   the answer, the CLI, the stdlib by module, the corpus and its tests, the gate, the documentation), each
   folder with a `README.md` index of one line a decision. The user allowed either this or grouped files of
   a reasonable size; one file a decision keeps every id's link stable when a group grows.
2. **No mark for a default.** A default that stands is a decision like a ruling: every decision may be
   reopened, so the distinction is history.
3. **A decision with only a local number** (a phase ledger's *question 38*) takes the next free id of the
   milestone that asked it, and the map from old to new is in this stage's record.
4. **A decision is cited by its id**: in a page as a link to its file under `docs/decisions/`, in a code
   comment as the bare id. The link check resolves every id cited outside `plans/` to a decision file.

## The method

### Task T1f.20.8.1 — the census of questions

Every ledger read, every row classified: *decision* (with its final answer), *open* (stays in `plans/`), or
*not carried* (superseded, moot, reversed), by a script that lists every id and its class, banked under
`utils/`. The counts written into this file.

### Task T1f.20.8.2 — the layout, for the user

The subjects and the slug rule, shown at [S1f.20.2](s1f.20.2_the_disposition_table.md)'s stop with the
table and the tree.

### Task T1f.20.8.3 — the files

One per decision, written in the release voice, each with what holds it. The ledgers under `plans/` keep
their rows until S1f.20.9 deletes the finished folders; a row that is now a decision points at its file.

### Task T1f.20.8.4 — the check

`utils/doc_audit.py` resolves every decision id cited outside `plans/` (a `.md`, a doc comment, a `.ein`
comment, `corpus.toml`) to a file under `docs/decisions/`, and fails on one it cannot.

## Acceptance

- Every answered or defaulted question of every ledger has exactly one decision file, or is listed *not
  carried* with why; every open one is still in `plans/`.
- No decision file holds an option not taken, a date, a name or a reference into `plans/`.
- Every decision id cited outside `plans/` resolves, held by the link check in the gate.
- `./run_tests.sh` is green.
