# S1f.20.8 — The decisions, in `docs/decisions/`

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Status:** planned 2026-11-15 on the user's rulings of that day
([P1f.20 § The user's rulings](README.md#the-users-rulings-2026-11-15)). **The layout and the defaults ruled
2026-11-17** ([P1f.20 § The user's rulings at S1f.20.2's review](README.md#the-users-rulings-at-s1f202s-review-2026-11-17),
items 5 to 11: process decisions not carried; an open question the project cites is written as a decision; the
map fixed at Q-M1e.23–.26, Q-M1e2.29, Q-M1f.315–.340), T3 and T4 next. **T1 and T2 done 2026-11-16**
([§ Record](#record-2026-11-16-t1-and-t2)): the census is banked and the layout is
[proposed for the user](s1f.20.8_layout_proposal.md). T3 and T4 wait for the user's review at S1f.20.2's stop.
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

## Record 2026-11-16: T1 and T2

**T1, the census.** [`utils/decision_census.py`](../../../utils/decision_census.py) reads every ledger the stage
names and every other one the tree holds: the nine milestone ledgers (four under `docs/history/`), the M1e review's
ten, `plans/open_questions.md` with M1's absorbed questions, M2's two id forms, the Q rows of F1 to F4, P1f.6's
numbered questions with round 2 and the replies' index, and the six stage files that number questions of their own.
It classes each id **decision**, **open** or **not carried**, follows a chain to its last word, and gives each
decision a proposed subject. What the state's first words do not decide, and every row whose state names a chain,
is classed by hand in [`utils/decision_census.txt`](../../../utils/decision_census.txt), one row an id, which
`--check` holds complete: a ledger edit that adds an id or a chain fails it until a hand row classes it.

```sh
python3 utils/decision_census.py --counts    # re-takes the table below
python3 utils/decision_census.py --check     # 593 ids; 0 gaps
```

| ledger family | decision | open | not carried | all |
|---|---:|---:|---:|---:|
| `docs/history/` (M1a, M1c, M1d, M1e and the review's Q1–Q10) | 33 | 10 | 23 | 66 |
| the milestone ledgers under `plans/` (M1e2, M1f, M1g, M1h, M10, M2) | 325 | 50 | 18 | 393 |
| `plans/open_questions.md` (the global pool, M1's Q4–Q42) | 19 | 2 | 7 | 28 |
| the Q rows of `plans/followups/` | 0 | 15 | 1 | 16 |
| P1f.6's local numbers and the stage files' | 48 | 1 | 41 | 90 |
| **all** | **425** | **78** | **90** | **593** |

Per ledger: M1f's 313 ids are 299 decisions, 0 open, 14 not carried; M1e2's 28 are 23, 1, 4; the per-ledger table
is the proposal's [§ 1](s1f.20.8_layout_proposal.md#1-what-the-census-found). **225 of the 593 are classed by
hand**, 368 read off their state. Of the 425 decisions, **64 are about the plans only**, **4 are decided and not
built**, and **16 are flagged to check** against today's engine. **52 local numbers are decisions** and take the
next free id of their milestone by default 3, Q-M1e.23 to .26, Q-M1e2.29 and Q-M1f.315 to .361, the 21 process-only
ones last ([§ 5.5](s1f.20.8_layout_proposal.md#55-local-numbers-52-decisions-take-a-milestone-id-31-if--51-is-ruled-as-proposed)).
Renumbered: nothing; the map is a proposal. The map starts after the ledger's last id, which `--local-ids` reads
off the ledger rather than holding: Q-M1f.314, the release-voice check's readings that S1f.20.1 defaulted for
S1f.20.2's review, is in the census as a decision (a default stands until the user revises it), under `gate/`.

**T2, the layout.** [`s1f.20.8_layout_proposal.md`](s1f.20.8_layout_proposal.md), written to be shown at
S1f.20.2's stop as it is: the tree under default 1 with each folder's count (361 files in 20 folders, the largest
`language/` at 70), the slug rule with six real examples (`--slug` applies it), four decision files rendered in
full, what the census found on the way (ids used twice, rows that say *stands* and are superseded, decisions with no
id, open questions cited by code), and ten questions for the user, each with a default.
