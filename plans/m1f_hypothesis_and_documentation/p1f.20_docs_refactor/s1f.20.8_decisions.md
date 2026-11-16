# S1f.20.8 — The decisions, in `docs/decisions/`

**Phase:** [P1f.20](README.md) (`docs/ein/`: the tree a released system would have)
**Status:** **done 2026-11-17** ([§ Record: T3 and T4](#record-2026-11-17-t3-and-t4)). Planned 2026-11-15 on the
user's rulings of that day ([P1f.20 § The user's rulings](README.md#the-users-rulings-2026-11-15)); T1 and T2 done
2026-11-16 ([§ Record](#record-2026-11-16-t1-and-t2)), the layout [proposed](s1f.20.8_layout_proposal.md) and ruled at
S1f.20.2's review on 2026-11-17 ([P1f.20 § The user's rulings at S1f.20.2's review](README.md#the-users-rulings-at-s1f202s-review-2026-11-17),
items 5 to 11); T3 and T4 the same day. `./run_tests.sh` is the coordinator's, after the stage.
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

## Record 2026-11-17: T3 and T4

**T3, the files**, written by four groups in worktrees and merged (`99ae892d`, `384dd4ab`, `96399075`, `33ee5de1`,
the indexes `7e5cd548`, `ea24e379`): **348 decision files in 19 folders**, each with a `README.md` index, and
[`docs/decisions/README.md`](../../../docs/decisions/README.md) on what a decision is, its file and how one is cited.

| folder | files | folder | files |
|---|---:|---|---:|
| `language/` | 73 | `stdlib/` | 3 |
| `imports/` | 24 | `stdlib/math/` | 22 |
| `diagnostics/` | 14 | `stdlib/math/decimal/` | 25 |
| `answer/` | 28 | `stdlib/math/int/` | 6 |
| `hypotheses/` | 16 | `stdlib/math/rat/` | 6 |
| `read-out/` | 27 | `stdlib/math/gcd/` | 4 |
| `cli/` | 19 | `stdlib/math/binary/` | 5 |
| `engine/` | 23 | `stdlib/math/num9/` | 6 |
| `corpus/` | 28 | `stdlib/bool/` | 6 |
| `docs/` | 13 | **all** | **348** |

`python3 utils/decision_census.py --files` re-takes it. `gate/` and `process/` hold no files: their decisions are
the work's (item 6 and the boundary).

- **New ids** (item 8: the next free id of the milestone that ruled them): **Q-M1e2.30 to .37**, the standing rules
  with no id (no option changes R, the least work per question, no two commands disagree and the oracle the
  reference, every answer exits 0, listings share 100 lines, the default region budget, an empty `:match` fires
  once, a conclusion's variable is bound on every way); **Q-M1f.341 to .349**, the documentation rulings and the
  boundary, and **Q-M1f.350**, *a fixed wrong answer is a regression test*; **Q-M1f.361 to .366**, the rulings of
  P1f.6's `design.md`. Each is a hand row of the census flagged `given`, naming where the ruling is written.
- **Local numbers at their ruled ids** (item 7): Q-M1e.23 to .25 (the review's Q1, Q7, Q8; its Q10 is Q-M1e.26, a
  gate decision, no file), Q-M1e2.29 (S1e2.3.4's Q6), Q-M1f.315 to .336 and .338 (P1f.6's), Q-M1f.339 and .340
  (S1f.5.4's Q1 and Q4). **Q-M1f.337**, `std.math.crypt`, decided and not built, has no file: it is
  [F27](../../followups/f27_cryptarithm_theory.md)'s (task 5). Each local number's row in the census opens
  `as <id>;`.
- **Not carried, with why**: every decision the census files under `process` (73, of them 11 reclassified at T3:
  Q-M1a.16, Q-M1f.50, .55, .56, .99, .172, .188, .191, .212, .227, corpus subjects that are the work's, and
  Q-M1f.296, a generator's) and under `gate` (32): the work's, which stays with its plans; Q-M1a.17 (a mechanism
  declined on the deleted walk's measurement) and Q-M1f.205 (the operand pass, superseded by the numeral's check
  where it is written, Q-M1f.272 and .273); and, decided but not built, Q-M1g.1, Q-M1h.4, Q-M1h.21 and Q-M1f.337
  (`pending`, with their plans). The census's 92 not-carried rows (superseded, moot, merged) stay as T1 classed them.
- **Reclassified**: Q-M1e.7 from `read-out/` to `answer/` (its read-out half is Q-M1f.18); Q-M1a.3 carried as its
  chain's last word, (c), the grammar's error report (the ledger's index says (a)); Q21 carried for its built half,
  the fixed DOT shapes, its `dot → ir` half an open row of `plans/open_questions.md`.
- **Item 10**: the ten open ids the project cited are decision files, ids kept, each what holds today: Q-M1a.6,
  Q-M1a.23, Q-M1c.6, Q-M1c.7, Q-M1d.1, Q-M1e.10, Q-M1e.13, Q-M1e.14, Q-M1e.22, Q-M1e2.2. Q-M1e.19, open and cited by
  nothing in the project, moved to `plans/open_questions.md`, id kept; the census reads it there.
- **The verify rows**, each read against today's engine before its file: Q-M1d.6 and Q-M1f.243 hold; Q-M1c.2, Q1
  and Q40 carried as the part that holds; Q16, Q25, Q-M1f.146, .206 and .208 rewritten to today's stdlib; Q18
  written from `ein-core`'s provenance; Q-M1e.23 (the review's Q1) with today's argument for determinism under
  `--jobs`, the shared store gone; Q-M1a.3 and Q21 as above; Q-M1a.16 and Q-M1f.205 not carried. Q-M1e.4 carried
  with its date warrant gone (item 9).

**T4, the check.** `python3 utils/doc_audit.py --links --check --tracked`, the gate step it was, now holds every
decision id the **project** cites to exactly one `docs/decisions/**/<id>-<slug>.md`, naming referrer, line and id
(`02b8b540`). It reads every project `.md` whole, every Rust comment and string literal under `ein.rs/crates/` (the
binary prints some), a `.ein`'s comments and `corpus/*.toml`; nothing of the process, `docs/history/`, `docs/lib/`,
`c/`, `nl2smt/` or `ein.rs/vendor/`. An id is `Q-M<m>.<n>` with every `.n` continuing it written short (a list, a
range's ends), or a **bare `Q<n>` written as a decision id**: not inside a longer name (`Q1.5b.7`, `S1f.5.4#Q1`,
`Q23@M2`) and not where the words beside it name the ledger that numbered it (*the review's Q2*, *S1f.5.4's Q4*, *F4
Q30*, *M1 Q26*, *S1e.1.3 / Q2*, a file, a quoted reply); those are a plan's local numbers (finding L-14). It fails
too on an id two files carry and on a misnamed file under `docs/decisions/`. **One allow-list row**, Q-M1e.9 in
`naf_deps.rs`' warning text, which the shape golden digests, stands until the engine's own ids are re-aimed (L-16,
L-12's owner); a row whose file stops citing its id is a finding.

- **Probes.** On the tree: 4 149 citations in 1 688 project files, 1 let stand, 0 findings. With a made-up
  `Q-M1f.9999` and a list's `.9998` in `README.md`, a bare `Q32`, an id in a Rust string literal and in a comment
  after code, in a `.ein` comment and a `corpus.toml` comment, a second file for Q-M1f.313 and a misnamed file: 26
  findings, each named, every citation of Q-M1f.313 among them; *F4 Q32*, *the review's Q2*, `Q1.5b.7`, an id in a
  `.ein` string and one in `plans/open_questions.md` pass. A stale allow-list row is a finding.
- **The citations rewritten** (`212ee01b`, 160 files), the 626 the check first found, by kind: **232** superseded →
  the carrier (Q32 → Q-M1e2.28, Q33 → Q-M1e2.26, Q41 and Q15 → Q-M1f.313, Q30 → Q16, Q-M1a.8 → Q-M1e.16, Q-M1e.8 →
  Q-M1e2.7, Q-M1e.9 → Q-M1e2.5, Q-M1e.15 → Q-M1e2.11, Q-M1e2.13/.14 → Q-M1e2.22 or .24, Q-M1e2.4 → Q-M1e2.27 and
  Q-M1f.48, Q-M1e2.20 → Q-M1e2.24, Q-M1e.3 → Q-M1f.341, Q-M1f.4 → Q-M1f.313, Q-M1f.135, .141, .142, .217 → Q-M1f.247,
  Q-M1f.307 → Q-M1e.11); **367** process or gate ids dropped, the behaviour in plain words (the register's
  *question* lists keep decisions only; 186 of these in `wrong_answers.toml`, and Q-M1f.2 28 times in `corpus.toml`);
  **9** not carried or never ids dropped (Q-M1a.17, Q-M1d.4, Q-M1a.4/.5/.14/.15); **15** open questions dropped
  (M2's Q7 to Q11, F4's Q34 to Q38, Q26, Q28); **2** reworded bare numbers that were no id; **1** allow-listed. Beyond
  the findings: **27** gap citations restated as closed (Q-M1c.6, Q-M1c.7, Q-M1e.13 at every site G4 named, Q-M1e.22,
  Q-M1d.1, Q-M1a.23, Q-M1e2.2 where a page called it open); **18** local numbers cite their new ids; **5** bare
  numbers that resolved to the wrong question removed; and a ledger link on a rewritten line points at the file.
  Comments, docs, test messages and test data only; `cargo doc -D warnings` and `cargo check --all-targets` pass;
  18 files' release-voice counts fell, re-banked.
- **Task 3** (`1d8fea01`): 286 ledger rows under `plans/` point at their file, a `**Decision file:**` line under the
  row's heading, written by a script from the census (idempotent).
- **Task 4** (`a20e8341`): `decision_census.py --check` holds the census to `docs/decisions/` both ways: **617 ids**
  (593 ledger rows and 24 given), 0 gaps; **348 carried decisions, 348 files, 0 mismatches**.
- **Task 5** (`b070b0c6`): `std.math.crypt` is [F27](../../followups/f27_cryptarithm_theory.md).
- **Found outside the stage**: [L-14 to L-18](findings.md): qualified local numbers left in the project, 151
  ledger links of decision ids, the allow-listed warning text, the census's dependency on `docs/history/`, and the
  register outside the release-voice check's scope.


### The gate on the stage's batch (2026-11-17)

The batch from `aa2b7093` (S1f.20.1's last green gate) to T4's last commit: **green on `2d040e98`**,
`./run_tests.sh` exit 0, 163 test binaries, none failed. Two runs failed first, each on a side effect of task 2's
citation rewrite, and neither moved an answer:

- **`fb47433d`, `ein-cli/tests/wrong_answers.rs`**: W28's two fixed rows record their refusal's position, and the
  rewritten header comments of 27 `.ein` files had grown or shrunk by one to three lines, moving it (line 32 to 33,
  28 to 29). Fixed in `970defbe` by giving every such file its old line count back (a grown block's last lines
  joined, a shrunk one padded with empty comment lines).
- **`970defbe`, `ein-ir/tests/fuzz_properties.rs`**: `golden/fuzz_seeds.txt` quotes every fuzz seed's and finding's
  text, comments included, and one seed's header lost a process id. Re-blessed in `2d040e98` after checking that
  the quoted text differs in that one comment line and no parse line moved.

**For every later stage that rewrites `.ein` comments** (S1f.20.6, S1f.20.9): keep each file's line count, and
expect `fuzz_seeds.txt` to need a bless, checked to be comment-only. The background wrapper's exit code is not the
gate's: read `exit` from the log.
