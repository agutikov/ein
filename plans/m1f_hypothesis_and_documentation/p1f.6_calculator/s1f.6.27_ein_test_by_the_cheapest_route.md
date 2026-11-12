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
