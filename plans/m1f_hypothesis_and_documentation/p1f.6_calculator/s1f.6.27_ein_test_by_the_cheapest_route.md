# S1f.6.27 — `ein test` decides a claim by the cheapest route that decides it

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply in round 2. **Done 2026-11-13:** [the record](#record-2026-11-13).
**An engine stage.**
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

### T1f.6.27.2 — the oracle's word, done

**Built, in the library, before the runner asks** (`fad716e6`; `ein_infer::read_out`, `ein_infer::explore`).

- `read_out::least(root, ast)`: the least question that can decide every claim a program states, read off
  the claims' shapes and nothing else, as T1's table has it. `read_out::describing(root, ast)`: all of R, or
  the one line every query claim is over, the one question the runner asked until now.
- `read_out::stands(first, fallback)`: which of the two runs a claim is read off. The describing run's
  wherever it decided; the first's where it decided and the description stopped; and where the two decide
  one claim differently, a refusal, *two questions decide one claim differently, which no two routes may —
  an engine defect*.
- `explore::Options::refute`, off by default and under `ein solve`: with it a claim whose part the answer
  did not exhaust is refuted by a class of a state found that the claim does not list, and `(solve …
  :expect (false))` by an alive root on a partial description. No route and no answer reads it.
- `Mode::flags()`: a question as `ein solve`'s flags, `describe` for the bare run.

**The test**, `ein-infer/tests/claim_routes.rs`. The corpus states 465 claims and all but one hold, so the
corpus's own claims cannot show that a route fails a false claim. The test writes its claims from the
oracle. On every corpus file and register witness whose cube the oracle enumerates, for every goal the
program asks and each of the three parts of R, the query's classes are read off the cube, and three claims
are written from them: the one that lists them, which holds; one with a class fewer and one with `(model)`
more, which fail. The program's own queries and its solve claim are taken out of its text first.

- Each claim is asked **alone, as `ein test` reads a file that states it**: the least question, then the
  describing one where the first left it undecided, unbudgeted and at a budget of 8 facts.
- All the claims of a program are asked of **one describing run**.
- **Held:** the least question's word where it decides, the fall-back's, what stands, and the
  description's each equal the oracle's; a stopped run that decides, decides as the oracle does; unbudgeted,
  what stands is never *not checked*.
- `(solve … :expect (false))`, spliced into the program's `(solve …)`, holds exactly when the oracle's root
  is dead, unbudgeted and at budget 8.

Counted (`cargo test -p ein-infer --test claim_routes -- --nocapture`, 2026-11-13):

| | |
|---|---:|
| programs the oracle reaches, and those of them that ask a query | 459, 296 |
| claims written and asked, and those over a line | 1 710, 504 |
| asked `--single`, asked `-n k+1` | 1 117, 593 |
| held by an exhausted least question | 580 |
| failed by an exhausted least question | 958 |
| refuted by a state found, nothing exhausted | 108 |
| fell back to the description | 64 |
| at budget 8: decided, of which refuted by what a stopped run found; not checked | 437, 59; 305 |
| `(false)`: decided; of which refuted at an alive root by a stopped run | 851; 67 |

**No disagreement, on any claim, so no register row was filed.** A program with no candidate is asked over
all of R alone and at one budget, since its three parts are one and no budget stops a run before root. The
sweep runs a program a thread, 24 s on this machine; its first form, a thread and every part on every
program, took 500 s. Two smaller tests: `the_least_question_is_read_off_the_claims_shapes`, T1's table row
by row on one program whose claims change, and `stands` on its nine cases;
`a_program_less_its_claims_keeps_everything_else`, the test's own text surgery.

### T1f.6.27.3 and .4 — the runner, and a NOT CHECKED that says why, done

**Built** (`9116a3f3`; `ein-cli`'s `test.rs`). A file is loaded and its claims found as before; then it is asked
`least`'s question with `refute` set. Where that is not the describing question and left a query's claim
undecided, the program is loaded again and asked `describing`'s, and each claim is read off the run
`stands` names. A solve claim is asked describe alone. The file's deadline, `-T`, is one instant for both
runs, and `--region-budget` is each run's. Two runs that decide a claim differently make the file an ERROR,
exit 2.

- **`-v`** names the question in a query claim's line, before what the run found: *holds (--single: all
  of R, 1 class over 1 states)*, *holds (-n 4: all of R, 3 classes over 3 states)*, *holds (--maximal -n
  3: the maximal line, 2 classes over 2 states)*, *holds (describe: all of R, 1 class over 1 states)* and,
  on the one corpus file that falls back, *holds (describe, after -n 3: all of R, 2 classes over 7
  states)*. A solve claim's line is as it was.
- **`--json-report`** is `ein-test-report/5`: every row carries `route` and `fell_back_from`, `null` on a
  row no run checked. The option's help says so.
- **T4.** A claim neither question decided keeps the describing run's line and gains one: *asked first:
  --maximal -n 3, which stopped at --region-budget*, or *asked first: -n 3, which found 3 states, each in
  a class the claim lists, and did not exhaust what the claim is about*. Exit 1, as before.
- **A refutation from a part** reads *the claim lists 1 class, and all of R has a class it does not list:
  (model)*, each further unlisted class on a line, and then *the run asked whether it is one state
  (--single) and did not exhaust all of R: what it found refutes the claim, and is not every difference*.
- **`--events` and `--json-summary`**, where a file was asked twice, are the last run's.

**The sweep** (`python3 build/s1f.6.27/sweep.py --new build/s1f.6.27/ein-t3b --diff`, 2026-11-13,
`build/s1f.6.27/sweep-t3b.txt`): the release binary at `9ac0f7ab` against the release binary of the tree
this section was committed with, kept as `build/s1f.6.27/ein-t3b`, over the 849 `.ein` files `9ac0f7ab`
tracks. A first sweep, on a binary built before the runner's last edits (`sweep-t3.txt`), said the same
line for line.

- **The surfaces that must not move did not:** `ein saturate --dump`, `ein solve`, `ein solve --single`,
  `ein solve -n 2`, `ein solve --count`, stdout, stderr and exit, and the whole of `ein solve
  --json-summary`: 0 files of 849.
- **`ein test FILE`: 0 files moved,** no line and no exit code.
- **No claim's outcome moved:** 0 of the 465. Every report is the old one once `route`, `fell_back_from`,
  `region.ms` and the schema are taken out.
- **`ein test FILE -v`: 133 files moved, each by the question named in a claim's line alone,** 269 lines:
  144 gained `--single`, 113 `describe` (a query's claim beside a solve claim), 7 `-n 3`, 2 `--maximal -n
  3`, 1 `-n 4`, 1 `-n 9`, and 1 `describe, after -n 3`.
- **The question each of the 465 claims was read off:** describe 309, of which 196 are solve claims;
  `--single` 144; `-n 3` 7; `--maximal -n 3` 2; `-n 4` 1; `-n 9` 1; describe after `-n 3` 1.

**The goldens, counted before they were blessed.** `help_shape.txt`: 4 lines, the help of `ein test`'s
`--json-report`, `--json-summary`, `--region-budget` and `-v` (`diff build/s1f.6.27/help_shape.before.txt
ein.rs/crates/ein-cli/tests/golden/help_shape.txt | grep -c '^>'`). No other golden moved: `ein test`
writes none, and no digest of `corpus_shapes.md5` reads it.

**The pins** (`ein-cli/tests/test_cli.rs`): `a_claim_is_asked_the_least_question_and_the_surfaces_say_which`,
four corpus files of four shapes and one that states no claim, the `-v` lines and the rows' `route`;
`a_claim_on_a_search_is_held_where_describing_r_stops`, I + BB = ILL at a budget of 1 200 facts, held by
`--single` where `ein solve` at the same budget stops and notes the claim;
`an_undecided_claim_falls_back_to_the_description`, `tests/region/08_query_classes.ein`, its line, its row
and the summary's `asked`; `a_state_found_refutes_a_claim_that_does_not_list_its_class`, the lines, the
row's `exact: false`, and `ein solve --single` on the same file keeping its note and its exit 0;
`a_claim_still_not_checked_says_what_was_asked_first`, both reasons;
`the_regions_false_is_refuted_at_an_alive_root`, exhausted and at a budget of 1;
`the_pages_name_the_reports_schema`, new: `defined_behaviour.md`, `configuration.md` and the option's help
name the schema the binary writes. `the_verbose_header_and_the_report_row_agree_field_for_field` reads the
question off the row, so the two surfaces are held to each other on it over the corpus.

**The pages, in the same commit:** [`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query),
the table and why a claim is held only by an exhausted answer; `defined_behaviour.md`, the *not checked*
paragraph and the report's schema, which the page had at `/2` while the binary wrote `/4`
([E-46](findings.md), filed and fixed here); `configuration.md`; `events.md` and `read_out.md` § 7 and § 12;
[`tests/README.md`](../../../tests/README.md) § What may be claimed here; and `AGENTS.md`'s rule *`ein test`
exhausts*, restated in place, with its sentence on the stdlib conformance gate. That gate,
`ein-infer/tests/stdlib_coverage.rs`, keeps the description: its question is what a program can activate,
which a claim's least question need not visit, and its doc comment says so.

### T1f.6.27.5 — E-31, measured, done

**What the profile shows.** `perf record` of the release binary at `9ac0f7ab` describing x × 7 at five digits
(`build/s1f.6.27/p/mul7_5.ein`, 16 331 saturations; `build/s1f.6.27/perf-b5.data`, 2026-11-13): 19 619 of the
20 746 samples on the machine's performance cores, 94.6 %, are `ein_infer::transversal::Mmcs::rec`, and 0.8 %
more `transversal::minimal_edges`; the 161 samples on its efficiency cores are the parse. Re-taken on this section's binary, the same
run is 87.0 % and 98.7 % `Mmcs::rec` on the two kinds of core (`perf-t5-describe.data`, taken while the tests
ran, so its 9.0 s are an upper bound), and `ein solve --count` on the same file is 307 ms with one sample of
`transversal` in 1 263 (`perf-t5-count.data`): its time is the walk's saturations and the parse.

**Which surfaces need N.** On the state route `N` is the one part of an exhausted description that is not read
off the states the walk met: it is searched for after the walk, the minimal transversals of the maximal alive
states' complements. Its **count** is how many transversals there are, so a surface that prints the count
needs the search as much as one that prints the sets, and E-31's question, *whether a surface that prints a
count of N needs the sets*, is answered yes.

| the surface | prints of N |
|---|---|
| a bare `ein solve`'s `N` row; `--trace`'s N; `--dump-answer`'s `nogoods.json` | the sets, or their count and which a partial run holds |
| the summary's `region.nogoods`, so `--json-summary`, `--dump-answer`'s `answer.json` and `index.json`, and `--events`' `region` event | the count |
| `--dump-lattice`'s `nodes.json` and `nodes.md`, the nodes the walk visited | neither |
| `ein solve --count`'s rows; every line of `ein test`, and `--json-report` | neither |

**Built.** `explore::Options::nogoods`, on by default: off, an exhausted description on the state route does
not search for `N` (`states::describe_with`), and `Described::nogoods_skipped` says so. Route 1 and the cube
read `N` off what they visited and take it regardless. `ein solve --count` asks without it unless it writes
`--json-summary`, `--events` or `--dump-answer` (it refuses `--trace`); `ein test` asks without it unless it
writes `--json-summary` or `--events`. No surface of a run that skipped `N` prints it; a printer handed such a
description says *not taken* in the `N` row and writes `null` for the summary's count, which no command
reaches. The flag is no option of any command and chooses no route, as `Cores` does not: it is *a core is taken
only where a surface prints one* (the user's ruling of 2026-11-05) applied to `N`.

**The pins.** `ein-infer/tests/explore.rs`'s `a_description_asked_without_n_is_the_description_less_n`: on
every corpus file and register witness outside the slow set, under describe and `--count`, the description
asked without `N` equals the one asked with it in every field once `N` is taken out of both, the states,
counts, lines, saturations, facts and verdict; only an exhausted description on the state route skips `N`; and
where nothing was skipped the two answers are equal whole: 976 runs, 314 of which skipped N, which held 5 688
sets where it was taken (`cargo test -p ein-infer --test explore a_description_asked_without_n -- --nocapture`,
2026-11-13). `ein-cli/tests/test_cli.rs`'s
`a_run_that_prints_no_n_takes_none_and_says_the_same`: on `tests/region/01_states.ein`, whose N is one set,
`--count` prints the same rows with a summary and without, `ein test -v` the same lines, and each summary's
`region.nogoods` is the bare description's.

**The sweep** (`python3 build/s1f.6.27/sweep.py --old build/s1f.6.27/ein-t3b --new build/s1f.6.27/ein-t5
--diff`, 2026-11-13, `build/s1f.6.27/sweep-t5.txt`): T3's binary against this section's, over the 849 files:
`ein saturate --dump`, `ein solve` bare, `--single`, `-n 2` and `--count`, stdout, stderr and exit, and the
whole `--json-summary`, 0 files moved; `ein test FILE` and `ein test FILE -v`, 0 files moved; the reports,
0 differ; no claim's outcome moved. The two artefacts `--count` writes beside it, `ein solve FILE --count
--dump-lattice DIR` and `--count --dump-answer DIR`, every file each writes, stdout, stderr and exit: 0 files
moved (`python3 build/s1f.6.27/lattice_sweep.py build/s1f.6.27/ein-t3b build/s1f.6.27/ein-t5`,
`build/s1f.6.27/sweep-t5-dumps.txt`).

**What it costs and saves** (`python3 build/s1f.6.27/t5_time.py build/s1f.6.27/ein-t3b build/s1f.6.27/ein-t5
--runs 3`, best of three, 2026-11-13, `build/s1f.6.27/t5-time.txt`; taken while the tests ran, so upper
bounds; every row's stdout, stderr and exit the same on the two binaries):

| the command | T3's binary | this section's |
|---|---:|---:|
| `ein solve p/mul7_5.ein --count` | 5.02 s | 0.21 s |
| `ein solve p/two_seven.ein --count`, TWO + TWO = FOUR | 33.71 s | 1.00 s |
| `ein solve p/mul7_5.ein`, a bare description | 5.04 s | 5.13 s |
| `ein test examples tests stdlib -q` | 5.16 s | 5.16 s |

The corpus's claims are on programs whose N is small, so `ein test` over the corpus gains nothing measurable;
what gains is a count on a search program, and a solve claim on one.

**What is left of E-31** is a bare `ein solve` on such a program, which prints `N`'s count and pays for it:
5.1 s of x × 7 at five digits, where `--count` takes 0.21 s. Removing it moves a row of every description, so
it is not free by this task's rule. Filed as
[Q-M1f.224](../open_questions.md#q-m1f224--what-a-bare-description-pays-for-n), defaulted to no change, the
user's to revise; E-31's row says so.

**The page:** [`read_out.md`](../../../docs/kernel/inference/read_out.md) § 2, the `--count` paragraph: it
prints no `N` row and takes no N where it writes no artefact that carries N's count.

### T1f.6.27.6 — SEND + MORE banked, done

**The program**, [`examples/beyond_arith/send_more_money.ein`](../../../examples/beyond_arith/send_more_money.ein):
`build/s1f.6.27/p/send.ein`, which is S1f.6.10's `p3_send_expect.ein`, with a header. The letters are declared by
column, units first, as E-29 requires; its claim is the one state, `(model (val D 7) (val E 5) (val M 1) (val N
6) (val O 0) (val R 8) (val S 9) (val Y 2))`, one class over all of R.

| `ein test examples/beyond_arith/send_more_money.ein -v` | the claim | saturations | wall | exit |
|---|---|---:|---:|---:|
| the release binary at `9ac0f7ab` (`build/s1f.6.27/ein-old`) | NOT CHECKED, *the claim is about all of R, and this run did not exhaust it* | 336 101 | 7.39 s | 1 |
| this stage's | holds (`--single`: all of R, 1 class over 1 states) | 17 747 | 0.46 s | 0 |

(`python3 build/s1f.6.27/t.py`, 2026-11-13; the saturations are `ein solve FILE` and `ein solve FILE --single`'s
`work` rows.) A bare `ein solve` on it describes R to the budget and stops after 336 101 saturations, 7.5 s and
529 MiB, the claim a note, *not checked*, exit 0, as before.

**The corpus entry.** `runs = ["solve", "solve --single", "test"]`, as `cryptarithm.ein` declares its own: `test`
is the stage's claim, `solve --single` the question it asks, and the bare `solve` the other half of what the file
shows, that describing this R stops. Measured (`utils/bench_env.sh python3 utils/corpus_cost.py -k
send_more_money --runs 3 --json build/s1f.6.27/cost-smm.json`, 2026-11-13): `solve` 7.47 s, `solve --single`
465 ms, `test` 465 ms, 8.40 s together, so **`slow = true`, `cost_ms = 8404`**. The slow set is fifteen:
`ein-corpus`'s pin is `the_slow_set_is_exactly_these_fifteen`, its name moving with the set as it has before,
and `corpus/README.md`, `corpus.toml`'s header, `corpus_cli.rs`'s comment and `AGENTS.md`'s command line name it.
Its describe dump writes four files (`ein solve FILE --dump-answer DIR`), so it is not `heavy_dump`.

**The tests that read the corpus, run before relying on it** (2026-11-13; T5's tree's time in brackets, from
`build/s1f.6.27/t5b-*.log`):

- `cargo test -p ein-infer --test region`, **alone**, as `AGENTS.md` has it for E-33: 28 passed, 39.76 s
  (40.32 s). The program requests nothing of `std.arith`: it reads `std.digits`' table `fa`.
- `-p ein-infer --test alive_invariant --test claim_routes`: 3 and 3 passed, 19.59 s (19.91 s) and 22.25 s
  (23.50 s). The oracle does not reach 80 candidates, so `claim_routes` does not write claims on it.
- `-p ein-cli --test corpus_cli --test lever_invariance --test answer_round_trip --test test_cli`: 7, 4, 9 and
  43 passed, 57.23 s (56.70 s), 55.49 s (55.35 s), 27.50 s (27.58 s), 9.49 s (8.70 s): a slow entry is out of the
  per-commit sweep of each.
- `EIN_CORPUS_SLOW=1 cargo test -p ein-cli --test corpus_cli`, the nightly's half: 7 passed, 184.94 s, the new
  cells' exits as banked and the `slow` flag held to the sweep.
- `-p ein-render --test corpus_shapes --test dot_wellformed --test golden_trace`: `corpus_shapes` failed, *42 of
  24269 renderings moved*, each `(new)`, 345.88 s (343.44 s); the other two passed.

**The goldens, counted before they were blessed.** `corpus_shapes.md5`: 42 digests new, every one the new
file's, one of each of the 42 kinds; 0 moved and 0 gone, so `cryptarithm.ein`'s header, rewritten, moved no
rendering (`EIN_BLESS=1 cargo test -p ein-render --test corpus_shapes`, then `python3
build/s1f.6.27/shapes_moved.py`, the golden at HEAD against the tree's). `corpus_exits.txt`: three lines added
by hand, the new entry's three cells, each exit 0, which the slow-tier run above holds. No other golden moved.

**The pages.** `cryptarithm.ein`'s header and its corpus note said SEND + MORE was not in the corpus;
[`examples/README.md`](../../../examples/README.md)'s line for `beyond_arith/` (seventeen programs, five that
search); [`tests/README.md`](../../../tests/README.md) § What may be claimed here names the file;
[`beyond_arithmetic.md`](beyond_arithmetic.md) gets its closing note on question 2.

### T1f.6.27.7 — the pages, done

The pages of each task are in that task's section and commit: T3's the grammar, `defined_behaviour.md`,
`configuration.md`, `events.md`, `read_out.md`, `tests/README.md` and `AGENTS.md`; T5's `read_out.md`'s
`--count` paragraph; T6's the examples' pages and `beyond_arithmetic.md`. This task adds the rows:
[`findings.md`](findings.md), **E-30 closed** and E-31 as T5 left it, the free part built and the rest
Q-M1f.224's; and the phase [README](README.md)'s row. `AGENTS.md`'s edits are the rule *`ein test` exhausts*
restated in place, its sentence on the stdlib conformance gate, and the slow pin's name in the command list,
each a rule or a command that changed; no line says what the stage did.

### The stage, closed

**The commits**, in order:

| commit | what |
|---|---|
| `f032b026` | T1: the table, on real programs, and Q-M1f.221 to .223. No source |
| `fad716e6` | T2: `read_out::least`, `describing`, `stands`, `explore::Options::refute`, `Mode::flags`, and `claim_routes.rs` |
| `9116a3f3` | T3 and T4: the runner, its surfaces, the pins, `help_shape.txt`, the pages, E-46 |
| `7748006b` | T5: E-31 measured, N taken only where it is printed, Q-M1f.224 |
| `4d2187e7` | T6: SEND + MORE banked, its entry, the slow pin, the goldens, the examples' pages |
| this section's | T7: the findings, the phase's row, and this record's sections for T6 and T7 |

**The acceptance.**

- *SEND + MORE = MONEY's claim is held by `ein test`, in the corpus*: T6, `--single`, 0.46 s, where it was NOT
  CHECKED, exit 1, after 7.39 s.
- *Every claim the corpus held or failed before is held or failed the same after; some NOT CHECKED became held or
  FAILED, each listed*: the sweeps of T3 and T5, 0 of 465 claims moved. No corpus claim was NOT CHECKED, so none
  could move; the claims that moved are the scratch witnesses' (`build/s1f.6.27/witnesses-final.txt`, `ein test
  FILE -v` under the binary at `9ac0f7ab` and this stage's, 2026-11-13, taken while tests ran):

  | `build/s1f.6.27/p/` | at `9ac0f7ab` | this stage's |
  |---|---|---|
  | `send.ein` | NOT CHECKED, exit 1, 8.03 s | held, `--single`, 0.49 s |
  | `send_wrong.ein`, Y = 3 claimed | NOT CHECKED, exit 1, 7.98 s | FAILED, `--single`, 0.47 s |
  | `mul7_6.ein` | no answer in 120 s | held, `--single`, 0.03 s |
  | `mul7_5.ein` | held, 5.15 s | held, `--single`, 0.03 s |
  | `two_seven.ein` | held, 33.21 s | held, `-n 8`, 0.07 s |
  | `two_one.ein`, `two_six.ein` | FAILED, 33.25 s and 33.61 s | FAILED, `--single` and `-n 7`, 0.05 s and 0.06 s |
  | `eq862_false.ein` | held, 0.05 s | held, `--single`, 0.03 s |

- *On every corpus program the oracle reaches, each claim's outcome under the chosen route equals its outcome
  under describe and under `ein oracle`*: `claim_routes.rs`, T2, 1 710 claims written from the oracle's classes
  on 459 programs, no disagreement; run again on this stage's last tree.
- *`ein test` never solves a query that carries no `:expect`*: the runner's early return is unchanged, and
  `a_claim_is_asked_the_least_question_and_the_surfaces_say_which` holds that a row with no claim names no
  question.

**Defaults taken**, each the user's to revise: Q-M1f.221, one question a file and then the describing one;
Q-M1f.222, a claim refuted by a state a run found, under `ein test` alone; Q-M1f.223, what `-v`,
`--json-report`, `--events` and `--json-summary` say of the question; Q-M1f.224, a bare description keeps
paying for N's count.

**Found outside the tasks**: E-46, the kernel page named the report's schema two versions behind the binary,
fixed with T3. **No wrong answer**: no two routes decided a claim differently, on the corpus, on the register's
witnesses or on the claims T2 writes, so no register row was filed.

**What moved of what already existed**, each intended: `ein test -v`'s claim lines in 133 files, by the question
named; `--json-report`'s schema, `/5`, and two keys on every row; `help_shape.txt`'s four lines of `ein test`'s
help; `corpus_exits.txt` and `corpus_shapes.md5` by the new file's cells and renderings alone; the slow set, by
one. No claim's outcome, verdict word, exit code, state, count, core or summary moved.

**Not built, as this file rules**: a flag that picks the route; an order of the obligation tree by the state
(P1f.10); the cryptarithm theory (reply 42). Not built by default: the rule refuting from a part under `ein
solve` (Q-M1f.222's (b)); a cheaper N for a bare description (Q-M1f.224's (b) and (c)).

**Not run by this stage**: `./run_tests.sh` and `./run_tests.sh --nightly`, which run on the stage's last commit
in detached worktrees, their outcome written under this line by the records commit; `utils/ci_container.sh`, which
the stage file does not need: no DOT writer, review bundle script or CI install changed.

**The gate and the nightly, run by the coordinator** (2026-11-13), each in a detached worktree on
`8f987963`, the stage's last commit, whose code is `4d2187e7`'s (T7 moved plan pages only):

| run | outcome |
|---|---|
| `./run_tests.sh` | exit 0 (`build/s1f.6.27/gate.log`), its link check over 553 pages with no finding |
| `./run_tests.sh --nightly` | exit 0, every step of its three jobs (`build/s1f.6.27/nightly.log`); `utils/stdlib_mutants.py --check` among them, 437 of 500 killed, the bank held |

**The new slow entry in the nightly.** `send_more_money.ein` is described in the nightly's slow halves, and no
step moved toward its budget of 5 min on the runner. Each step's `finished in`, on this machine with a gate
running beside each nightly, against S1f.6.26's nightly on `9ac0f7ab`, taken the same way
(`build/s1f.6.26/nightly.log`):

| step | `9ac0f7ab` | `8f987963` |
|---|---:|---:|
| `corpus_cli`, the slow entries | 130.6 s | 144.0 s |
| `answer_dump` | 43.0 s | 45.3 s |
| `answer_round_trip`, the slow and `heavy_dump` entries | 193.8 s | 143.0 s |
| `lever_invariance`, the slow cells | 55.2 s | 42.5 s |
| `corpus_cores`, the slow entries | 227.9 s | 225.3 s |
| `region`, the slow half | 56.3 s | 51.4 s |
| `obligation_rung`, the slow half | 72.2 s | 69.2 s |

The runner's times are not these; the next nightly in CI is what shows them.
