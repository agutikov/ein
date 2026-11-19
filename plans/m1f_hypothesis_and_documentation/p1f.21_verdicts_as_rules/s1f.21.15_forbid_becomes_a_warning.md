# S1f.21.15 — *forbid* is removed, and nothing validates a non-monotone program's answer

**Phase:** [P1f.21](README.md), part 1 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-19 on the user's ruling, as *forbid becomes a warning*; **started the same day on the
user's word** (*"Remove this *forbid* from the code and docs completely"*), its scope as the second ruling gives it.
The file keeps its first name.
**Waits on:** no decision: [Q-M1f.564](../open_questions.md#q-m1f564--a-non-monotone-program-loads-with-a-warning-forbids-refusal-goes).
It runs before [S1f.21.11](s1f.21.11_kill_and_three_conventions.md), which then has no *forbid* to re-aim.
**Writes:** `ein-infer/src/forbid.rs` and what calls it, gone; the read-out and the summary, a warning; the tests
that pin a refusal or a flagged program's answer; `corpus/regressions.toml`, the shape golden, the register of
wrong answers; the decisions; `AGENTS.md`; every page of `docs/` and every comment that speaks of the refusal.

## The rulings

The user, 2026-11-19, verbatim, first: *"I tell you what I expect - it loads, ein print huge WARNING, and then we guarantee nothing about it's output. NO!  Q-M1e2.5 IS WRONG! What the hell this `forbid` is? Where did it came from?"*

And then: *"Ok, I cancel those my listed decisions regarding absent -> false.  We allow to load non-monotone programs, but we don't claim about solution. That means - no tests validate the result of the solution of non-monotone programs. Remove this *forbid* from the code and docs completely."*

So:

1. **The user's rulings of 2026-11-02 and 2026-11-04 on a refutation under an `absent` are cancelled**, *forbid* and
   its derived case with it ([Q-M1e2.5](../../../docs/decisions/language/Q-M1e2.5-a-refutation-may-not-rest-on-an-absent.md)).
2. **A non-monotone program loads.** Ein warns, loudly, and claims nothing of its answer.
3. **No test validates the result of a non-monotone program's solution.**
4. ***Forbid* goes from the code and from the pages, completely.**

## What it touches, counted before the stage

On the gate build of `946ee788`, over the 893 programs under `examples/` and `tests/` (`ein solve FILE --no-search
--json-summary`, the summary's `monotonicity.monotone`; `ein show hyps` for the refusal's words):

- **20 programs are refused by *forbid*** (16 under `examples/ein-bugs/`, 2 under `examples/branching/`, 2 under
  `examples/lattice/`). 15 are the witness of a row of `corpus/regressions.toml`, whose right answer is the refusal;
  2 state an `:expect`.
- **27 programs load and are flagged** by the static check (12 under `examples/ein-bugs/`, 8 `examples/lattice/`, 3
  `examples/features/`, 3 `examples/saturation/`, 1 `tests/region/`). Of their results, today: 4 state an `:expect`;
  9 are the witness of a row of `corpus/regressions.toml`; the shape golden holds, for each, a line of every kind,
  among them the kinds that are an answer (`read-out`, `cores`, `states`, `region-lattice`, `oracle`, `explain`,
  `region-trace`); and the register of wrong answers has five rows of them (`W8`), each held to what the binary
  answers today.

## Task 1 — *forbid* goes

1. **First, that each of the 20 is flagged by the static check once it loads.** A route's licences are given up on
   a flagged program, and its routes are the cube's. A program that loads unflagged would be searched with licences
   that do not hold of it. If one of the 20 is not flagged, the stage stops there and says so before anything else.
2. The check, its module, its callers, its tests and its words go: no command refuses a program for an `absent`.
   The messages that told a user how to rewrite a program so as not to be refused go with it.
3. **The warning.** A flagged program is warned of as it is today, `lattice not monotone`, its sites and its stale
   facts; **Reading 1**, default: and louder, one block at the head of the read-out and one line on stderr, in
   capitals, that says the program is not monotone and that **nothing is claimed of this answer**, on every command
   that answers about a commitment. The other reading: the rows as they are.
4. The pages and the standing rules: the decision of Q-M1f.564 written, Q-M1e2.5's removed and every citation of
   it re-aimed; `AGENTS.md` § The language, *forbid*'s rule replaced by the ruling; the diagnostics' § 2.7 gone;
   the top `README.md`, the reasoning's pages on `absent` and on monotonicity, the read-out's page, the glossary,
   and every comment in `ein.rs/`, `stdlib/`, `examples/` and `tests/` that says a program is refused for it.

## Task 2 — no test validates a non-monotone program's result

Of the 47 programs, the 20 and the 27. **What goes**:

| what validates today | what becomes of it |
|---|---|
| an `:expect` a flagged program states of its answer | removed from the program; a program under `tests/`, which must state one, moves to `examples/` |
| a row of `corpus/regressions.toml` whose witness is one of the 47 | removed; the witness stays a program of the corpus. The rows are listed in the record, for M1n |
| the shape golden's lines of an answer's kind for a flagged program (`read-out`, `cores`, `states`, `region-lattice`, `oracle`, `explain`, `explain-noalts`, `region-trace`) | not banked and not compared |
| a test in a crate that asserts what a flagged program answers: a region, a count, a core, a model | the assertion goes; listed in the record |
| the register's rows of non-monotone programs (`W8`), held to what the binary answers | removed from the register; listed in the record, for M1n, whose rows they were |
| the expected refusals of the 20 (exit codes, messages) | the programs load: exit 0 |

**What stays**, since it validates no answer: that each of the 47 loads and every command exits 0 on it; that the
static check flags it and the warning is printed; that every command gives the same $R$ of it, which is the user's
standing rule of one answer and compares a run with a run; the lines of the shape golden that are no answer
(`load`, `ir`, `plan`, `match`, `naf`, `saturate`, the drawings of the program).

**Reading 2**, default: `ein test` is not changed: on a flagged program it checks a claim as it does today. Whether a
claim of a flagged program is *not checked* is put to the user and not built.

## What must hold

- **Every answer of a monotone program**: $R$ of every corpus program the static check does not flag, by the corpus
  sweep and the oracle; `ein-cli/tests/lever_invariance.rs`; the register's other rows and every other row of the
  regressions.
- **No licence on a flagged program** (task 1.1), held by a test over the 20.
- **Nothing is claimed**: no page, read-out, comment or test says an answer of a flagged program is right, sound or
  complete, or that such a program is refused.
- The goldens move for the 47 and where a refusal's line was pinned; the diff is read.
- A large change to what loads: the gate and the nightly on the stage's last commit.

## Acceptance

`grep -ri forbid` finds the word in no file of `ein.rs/`, `docs/`, `stdlib/`, `examples/`, `tests/`, `corpus/`,
`README.md` or `AGENTS.md` in the sense of this check; each of the 20 loads, is flagged and prints the warning; no
`:expect`, no regression row, no golden line of an answer's kind and no assertion is left on any of the 47.

## Record

Nothing yet.
