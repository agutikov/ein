# S1f.6.27 — `ein test` decides a claim by the cheapest route that decides it

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply in round 2. Not started. **An engine stage.**
**Depends on:** [S1f.6.25](s1f.6.25_the_section_j_fixes.md), which it runs after
([round 2, item 39](questions_round2.md#q39): *planned as a stage of this phase and run after the section J
fixes*). Independent of the stdlib layout. The cryptarithm theory of [reply 42](questions.md#q42) waits for
it.

## The ruling it rests on

[Question 39](questions.md#q39): *Should `ein test` decide a claim by the cheapest route that decides it?*
The user, 2026-11-12: *"I don't understand the question, explain in more details and examples"*.

[Round 2, item 39](questions_round2.md#q39) explains it and takes **(a), yes, as an engine stage,**
meanwhile. **The user's reply there, committed as `dc88520b`: *"(a)"*.**  The explanation's proposal:

> `ein test` reads the shape of each claim and runs the cheapest mode that decides it. A claim of one state
> is `--single`'s question; a claim of exactly k states is `-n k+1`'s (find up to k+1: a (k+1)-th refutes
> the claim); only a claim about the whole region describes it. It is your rule *minimum work per question*,
> applied to the runner.

Row 72 of [section J](questions.md#j-findings-that-need-an-owner) (E-31), left empty, took its suggestion:
*one engine stage with question 39*.

## The findings

- **E-30.** `ein test` always describes all of R and marks a claim NOT CHECKED, exit 1, when that stops at
  the budget; `--single` and `-n` take the obligation tree, which the runner never asks. SEND + MORE = MONEY
  with its answer claimed: `ein test`, *NOT CHECKED*, 7.5 s, exit 1; `ein solve --single` on the same file,
  *Solution — R is one state*, 0.5 s (2026-11-12, `questions.md` 39). No wrong answer: a claim that could be
  held is not checked, which is why the corpus holds I + BB = ILL and not SEND + MORE.
- **E-31.** Describing R spends its time on a set computation (minimal transversals,
  `ein_infer::transversal`), not on saturations: 94 % of 5.1 s on a five-digit equation (`questions.md` 72).
  Its row asks whether a surface that prints a count of N needs the sets.

## What it builds

**The runner asks the question the claim is.** For each claim shape the grammar has, one mode of
`ein solve` is the least work that decides it. T1 writes the table; the plan's expectation, to be confirmed
there and not assumed:

| the claim | the question | the mode |
|---|---|---|
| a query's `:expect` listing one class over all of R, or over the minimal line | is there exactly this, and no other | `--single`'s |
| a query's `:expect` listing k classes | are there these k, and no (k+1)-th | `-n k+1`'s |
| `(solve … :expect (false))` | is root dead | root's saturation |
| `(solve … :expect (region …))`, and any claim about owing or merged states | what R is | describe, as today |
| a query's `:expect (false)` | is R empty | whichever of the above decides emptiness first; E-18 ([S1f.6.26](s1f.6.26_what_root_owes.md)) is its neighbour |

A claim is about *projection classes* and a mode finds *states*: two states can fall in one class, so
*`-n k+1` found k+1 states* does not refute a claim of k classes by itself. T1 has to state, for each row,
exactly when the cheaper mode's answer decides the claim and when the runner must fall back to describe.
**The fall-back is always describe,** so no claim that is checked today becomes unchecked.

**Several claims in one file** are read off one run today. The stage keeps *one run where one run decides
them all*: the mode that decides every claim of the file, which is describe as soon as one claim needs it.
Running a mode a claim is a choice T1 prices and does not take by default.

**E-31** is taken as far as it is free: where a surface prints a *count* of N and not its sets, the task
measures whether the count needs the transversal, and removes the work only if every surface is
byte-identical after. Anything more is a finding with an owner, not this stage's.

## Tasks

1. **T1f.6.27.1 — read-only: the table, on real programs.** Every claim shape of
   [`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query), with the mode
   proposed for it, and for each a corpus program on which describe and the proposed mode are both run and
   compared: the outcome (held, FAILED, NOT CHECKED), the saturations, the time. SEND + MORE
   (`build/s1f.6.10/p/p3_send_expect.ein`) and the search programs of `examples/beyond_arith/` and
   `examples/binary_logic/` are the witnesses; the census of the corpus's claims by shape
   (`ein test examples tests stdlib --json-report`) says how many claims each row would move to another
   route. Shown before the runner changes.
2. **T1f.6.27.2 — the oracle's word.** For each row, a test that holds the proposed mode's verdict on a
   claim to describe's and to `ein oracle`'s on every corpus program the oracle reaches: **every answer
   agrees with the oracle** (the user's rule of 2026-10-16), and a claim's outcome may move only from NOT
   CHECKED to held or FAILED, never between held and FAILED.
3. **T1f.6.27.3 — the runner,** in `ein-cli`'s `test` command: the shape read, the mode chosen, the
   fall-back. `--json-report` gains the route each row was decided by (the report's schema number moves, as
   it has before). `ein test -v` says the route in a word.
4. **T1f.6.27.4 — a NOT CHECKED that is still one says why:** the route tried, and that describe did not
   exhaust. Exit codes are unchanged in meaning: 1 where a claim is false or not checked, 2 on a load error.
5. **T1f.6.27.5 — E-31, measured:** `perf record` on the row's witness; which surfaces need the sets and
   which only the count; the change, if it is free, with the sweep that shows no byte moved.
6. **T1f.6.27.6 — SEND + MORE = MONEY banked** under `examples/beyond_arith/` with its claim and a corpus
   entry, declared by column (E-29: declared as written it has no answer in 600 s, which is P1f.10's), once
   `ein test` holds it. It is the stage's witness in the corpus. Whether it is `slow` is measured, as every
   entry's is.
7. **T1f.6.27.7 — the pages:** [`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query)
   and the runner's page (what `ein test` runs for which claim); `tests/README.md`;
   [`beyond_arithmetic.md`](beyond_arithmetic.md) gets a closing note on its question 2;
   `AGENTS.md`, whose S1c.1.3 sentence *it exhausts* is restated; E-30 and E-31 closed or re-owned in
   [`findings.md`](findings.md).

## Acceptance

- SEND + MORE = MONEY's claim is held by `ein test`, in the corpus.
- Every claim the corpus held or failed before is held or failed the same after; some NOT CHECKED became
  held or FAILED, each listed.
- On every corpus program the oracle reaches, each claim's outcome under the chosen route equals its outcome
  under describe and under `ein oracle`.
- `ein test` never solves a query that carries no `:expect`, as before.

## What it must not do

- **Not change what a claim means.** *Naming a relation closes it*, the classes over `:over`, the region
  claim: all as they are. E-18's question stays asked and unanswered.
- **Not add a flag that picks the route.** The route is a function of the claim; config never changes an
  answer, and a route that could change an outcome would be a bug, not an option.
- **Not order the obligation tree by the state.** That is [P1f.10](../p1f.10_hypothesis_structure/README.md)'s
  ([reply 40](questions.md#q40)); this stage takes the tree as it branches today.
- **Not build the cryptarithm theory** (`std.math.crypt`, reply 42), which waits for this stage and for
  P1f.10.

## The standing rules

A wrong answer met on the way (two routes disagreeing on a claim is one) is a row in
`corpus/wrong_answers.toml`, filed `open` before its fix, as W27 was. Findings go to
[`findings.md`](findings.md) under the next free id; questions to [`open_questions.md`](../open_questions.md)
under the next free id. The design is shown on real computed examples first (T1). Config never changes an
answer; exit non-zero only on error or a claim not held. The gate is `./run_tests.sh`, then `./run_tests.sh
--nightly` after the code commit; `utils/ci_container.sh` is not needed.

## Record, 2026-11-13

The baseline every run below compares with is the release binary built at `9ac0f7ab`, kept as
`build/s1f.6.27/ein-old`; *the tree's* is the release binary of the commit a paragraph names. The scratch
programs are under `build/s1f.6.27/p/`, written from S1f.6.10's (`build/s1f.6.10/p/`) with a claim added:
`send.ein` is `p3_send_expect.ein`, SEND + MORE = MONEY with its answer claimed. The seconds of T1 were taken
while the previous stage's gate ran on the same machine, so they are upper bounds; the saturations are exact.

### T1f.6.27.1 — the table, on real programs, done

Read-only: no source moved before this section was committed.

**What the engine already does.** `ein solve --single FILE` and `ein solve -n N FILE` check a query's claim
where their answer lists the whole part the claim is about: `--single` that says `Solution`, `Contradiction`
or `Open`, and `-n N` that found fewer than N states (`ein_infer::read_out`'s `exact`). On `p/send.ein`,
`ein solve --single` prints `:expect holds` under its one state. So the routes exist and decide claims; the
runner asks describe and no other question.

**The corpus's claims by shape** (`build/s1f.6.27/ein-old test examples tests stdlib -q --json-report
build/s1f.6.27/report-old.json`, 2026-11-13: 742 files, 465 claims, 464 held, 1 FAILED, 0 not checked, 221
files that do not load, 256 that state no claim):

| the claim | claims | outcome today |
|---|---:|---|
| a query's, one class, `(model …)`, over all of R | 255 | 254 held, 1 FAILED (`ein-bugs/wrong-answers/goal-macro-expect-model.ein`) |
| a query's, k classes, `(or …)`, over all of R | 9 | held; k is 2 on seven, 3 on one, 8 on one |
| a query's, k classes, over the maximal line | 2 | held; k is 2 |
| a query's `(false)` | 3 | held; each beside a region claim |
| a query's, over the minimal line | 0 | |
| `(solve … :expect (false))` | 57 | held |
| `(solve … :expect (region …))` | 139 | held; 17 with `(owing …)`, 2 with `(merged …)` |

By file, 265 files state a claim: 139 state a region claim, 64 of them beside query claims; 57 state
`(solve :expect (false))` and nothing else; 69 state query claims alone, 67 over all of R and 2 over the
maximal line. Three of the 265 are programs the static check flags
(`tests/region/07_cube.ein` and the two `oblig-stdlib-total-owed` witnesses), and each states a region
claim.

**The table.** A file's claims are read by their shapes, and the file is asked one question, the least that
can decide every claim; what it leaves undecided is asked the describing question, today's. *P* is a part of
R: all of it, the minimal line or the maximal line.

| the file's claims | the question | its answer decides a claim | it falls back to describe |
|---|---|---|---|
| queries' claims, all over one part P, none listing more than one class: `(model …)`, `(false)` | `--single` over P | `Solution`, `Contradiction` or `Open`: P is one state or none, the classes are P's, and the claim holds or fails as under describe, with describe's lines. `Ambiguity`: a claim is refuted where one of the two states gives a class it does not list | `Ambiguity` with both states in the one class a claim lists; a stop at the budget |
| queries' claims, all over one part P, the most classes one lists being k ≥ 2 | `-n k+1` over P | fewer than k + 1 states found: the run listed all of P, so every claim of the file is decided, the ones that list fewer classes too. k + 1 found: a claim is refuted where a state gives a class it does not list | k + 1 states found that all fall in classes the claim lists; a stop at the budget |
| `(solve … :expect (false))` | describe | root is dead: describe's route is root's saturation alone (L0, L1), and the claim holds. Root is alive: the claim fails, with what R holds where describe exhausts, and at root where it does not | never: it is describe |
| `(solve … :expect (region …))`, in each of its forms: states, `(merged …)`, `(owing …)`, `(region)` | describe | as today | never |
| a solve claim beside queries' claims; queries' claims over more than one part | describe of all of R | as today: one run decides them all | never |
| a query's claim over one line, beside no other part | that line: `--minimal` or `--maximal`, with `--single` or `-n k+1` as above | as the first two rows | to that line's describe, `--minimal` or `--maximal`, which is today's question for it |
| a claim on a query with no `:goal` | none answers it | NOT CHECKED, as today | |

Three things the table rests on.

- **A claim is about classes and a mode finds states.** Several states can fall in one class, so k + 1
  states do not refute a claim of k classes by their number. What refutes is a class: every state a mode
  lists is a certified member of the part it was asked, on the monotone routes and on the cube, so a class
  of a state found that the claim does not list is a class of P the claim does not list, whatever the run
  left unvisited. And nothing short of all of P shows that the claim's classes are the only ones, so a claim
  is **held** by an exhausted answer alone. `tests/region/08_query_classes.ein` is the corpus's witness:
  two classes over seven states, `-n 3` finds three states in the two classes, and describe decides.
- **Refuting from a part is new.** Today a run that did not exhaust decides nothing, under either command.
  The runner takes the rule; `ein solve` under a mode keeps its note, *not checked*, and its exit 0
  ([Q-M1f.222](../open_questions.md#q-m1f222--a-claim-refuted-by-a-state-a-run-found), defaulted).
- **The fall-back is the describing question, asked of the program loaded again,** since a run saturates the
  root it is given. Where it decides a claim its report stands, so a claim describe decides reads as it does
  today, line for line. Where it does not and the first question did, the first stands. Two questions that
  decide one claim differently are an engine defect and the file is an ERROR, never an outcome.

**Each row on a real program** (`python3 build/s1f.6.27/t1_witness.py`, 2026-11-13, on `ein-old`; a cell is
saturations · wall · what the run says of the claim; *p/* is `build/s1f.6.27/p/`):

| the program | the claim | `ein test` today | describe | the question | its answer |
|---|---|---|---|---|---|
| `beyond_arith/cryptarithm.ein` | one class | held, 0.03 s | 417 · 0.03 s · held | `--single` | 119 · 0.02 s · `Solution` · held |
| `beyond_arith/equation_by_search.ein` | one class | held, 0.02 s | 251 · 0.03 s · held | `--single` | 31 · 0.02 s · `Solution` · held |
| `p/send.ein`, SEND + MORE = MONEY | one class | **NOT CHECKED, exit 1, 7.97 s** | 336 101 · 7.66 s · not checked, stopped at the budget | `--single` | **17 747 · 0.46 s · `Solution` · held** |
| `p/send_wrong.ein`, the same claiming Y = 3 | one class, false | NOT CHECKED, exit 1, 7.63 s | 336 101 · 7.63 s · not checked | `--single` | 17 747 · 0.46 s · `Solution` · failed |
| `p/mul7_5.ein`, x × 7 at five digits, E-31's witness | one class | held, 5.21 s | 16 331 · 5.15 s · held | `--single` | 51 · 0.02 s · `Solution` · held |
| `p/mul7_6.ein`, the same at six digits | one class | no answer in 120 s | no answer in 120 s | `--single` | 61 · 0.03 s · `Solution` · held |
| `p/two_one.ein`, TWO + TWO = FOUR claiming one of its seven | one class, false | FAILED, 32.51 s | 67 046 · 32.01 s · failed | `--single` | 1 554 · 0.05 s · `Ambiguity`: the second state's class is not listed, which refutes |
| `binary_logic/satisfiable.ein` | one class | held, 0.01 s | 30 · 0.01 s · held | `--single` | 17 · 0.01 s · `Solution` · held |
| `arith/add.ein` | one class each, two claims | held, 0.03 s | 1 · 0.03 s · held, held | `--single` | 1 · 0.03 s · `Solution` · held, held |
| `beyond_arith/equation_two_unknowns.ein` | three classes | held, 0.47 s | 1 466 · 0.41 s · held | `-n 4` | 1 165 · 0.37 s · 3 states, all of them · held |
| `binary_logic/truth_table.ein` | eight classes | held, 0.02 s | 30 · 0.03 s · held | `-n 9` | 27 · 0.03 s · 8 states, all of them · held |
| `binary_logic/knights.ein` | one class and two, two claims | held, 0.02 s | 26 · 0.02 s · held, held | `-n 3` | 13 · 0.03 s · 2 states, all of them · held, held |
| `p/two_seven.ein`, TWO + TWO = FOUR | seven classes | held, 47.95 s | 67 046 · 32.66 s · held | `-n 8` | 1 878 · 0.06 s · 7 states, all of them · held |
| `p/two_six.ein`, the same claiming six of the seven | six classes, false | FAILED, 32.46 s | 67 046 · 32.68 s · failed | `-n 7` | 1 724 · 0.06 s · 7 states, not exhausted: the seventh's class is not listed, which refutes |
| `tests/region/08_query_classes.ein` | two classes over seven states | held, 0.00 s | 8 · 0.00 s · held | `-n 3` | 4 · 0.00 s · 3 states in the two classes, not exhausted: **falls back** |
| `p/eq862_false.ein`, x × 7 = 862 | a query's `(false)` | held, 0.05 s | 251 · 0.05 s · held | `--single` | 31 · 0.02 s · `Open` · held |
| `features/11_expect_ambiguity.ein` | two classes over the maximal line | held, 0.00 s | `--maximal`: 10 · 0.00 s · held | `--maximal -n 3` | 10 · 0.00 s · 2 states, all of them · held |
| `tests/region/09_query_over_maximal.ein` | two classes over the maximal line | held, 0.00 s | `--maximal`: 10 · 0.00 s · held | `--maximal -n 3` | 10 · 0.00 s · 2 states, all of them · held |
| `beyond_arith/equation_refuted.ein` | `(solve :expect (false))` | held, 0.02 s | 1 · 0.02 s · held | describe | the same run |
| `binary_logic/unsatisfiable.ein` | a region claim, `(owing …)`, and a query's `(false)` | held, 0.01 s | 28 · 0.01 s · held, held | describe | the same run |
| `beyond_arith/equation_no_solution.ein` | a region claim, `(owing …)` | held, 0.03 s | 76 · 0.03 s · held, held | describe | the same run |
| `tests/region/01_states.ein` | a region claim, seven states | held, 0.00 s | 8 · 0.01 s · held | describe | the same run |

On the old binary the two refutations read *not checked* under `ein solve --single` and `ein solve -n 7`,
since no run refutes from a part today; the cell says what the states found show.

- **Where the claim holds on a search, the question is the search and describe is every alive node.** SEND +
  MORE: 17 747 saturations against 336 101 and a stop. x × 7 at six digits: 61 against no answer.
- **`(solve :expect (false))` has no cheaper question.** Describe saturates root alone on each of the
  corpus's 57 (57 saturations in all, `python3 build/s1f.6.27/t1_price.py`).
- **A line's claim gains nothing where it holds:** the whole line has to be listed either way, 10
  saturations and 10. It is asked `-n k+1` for the claim that fails.

**One run a file against a mode a claim, priced** (`python3 build/s1f.6.27/t1_price.py --report
build/s1f.6.27/report-old.json`, 2026-11-13, on `ein-old`, over the 69 files whose claims are queries'
alone; a mode a claim is each claim's own question in its own run):

| | saturations | runs |
|---|---:|---:|
| describe, today | 2 357 | 69 |
| one run a file: the question of the file's largest claim | 1 494 | 69, and one fall-back |
| a mode a claim | 1 590 | 156 |

58 of the 67 files over all of R ask `--single`, seven `-n 3`, one `-n 4` and one `-n 9`; the two over the
maximal line ask `--maximal -n 3`. The one run saturates no more than describe on any of the 69, and
leaves one file undecided, `tests/region/08_query_classes.ein`. A mode a claim saturates more, since each
run saturates root again, and loads the program once a claim: on these programs a run's wall is its load
(`arith/add.ein`, 29 ms under `ein solve`, 26 ms under `ein saturate`, best of 15, 2026-11-13). **One run a
file is taken** ([Q-M1f.221](../open_questions.md#q-m1f221--one-question-a-file-and-the-fall-back),
defaulted): it is the stage file's own preference, it is cheaper, and a claim that lists fewer classes than
the file's largest is decided by the same run whenever that run is exhausted.

**A flagged program.** Its modes are the cube in layer order, so the least question saturates what describe
does where a claim holds, and stops early where a state refutes one. No corpus claim a mode would be asked
is on a flagged program; T2's test writes claims on every one the oracle reaches.

**What will move.** No claim of the corpus is NOT CHECKED today, so no corpus outcome can move; the claims
that move are the scratch witnesses': `p/send.ein` from NOT CHECKED to held, `p/send_wrong.ein` from NOT
CHECKED to FAILED, `p/mul7_6.ein` from no answer to held. T6 banks the first.

**Defaults taken here**, each the user's to revise: Q-M1f.221, one question a file and then the describing
one; Q-M1f.222, a claim is refuted by a state a run found, under `ein test` and not under `ein solve`;
[Q-M1f.223](../open_questions.md#q-m1f223--what-the-surfaces-of-ein-test-say-of-the-question), what `-v`,
`--json-report`, `--events` and `--json-summary` say when a file was asked two questions.
