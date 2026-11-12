# S1f.6.26 — A read-out row for what root owes

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-12, on the user's reply of that day. Not started. **An engine stage,** on the
read-out.
**Depends on:** nothing in the phase. Independent of the stdlib layout.
[S1f.6.28](s1f.6.28_word_problems_second_pass.md) reads its row.

## The ruling

[Question 30](questions.md#q30) asked which fix to wait for where a named gap stands beside a supposition:
*(a) a read-out row for what root owes when R is not empty. A small change to the read-out*, or (b) F21's
ruling on non-monotone programs. The user, 2026-11-12:

> that simple problem couldn't have non-monotone solution, I mean the problem is too simple and too obvious,
> or not? or that means that simple problem, containing ambiguity, could already be non-monotone? Or that
> means ambiguity == non-monotone solution == required additional information? Again - those questions go
> into M1h. Here I don't care - so pick (a).

So (a) is built. The questions in the reply are answered in [round 2, item 30](questions_round2.md#q30) and
are [M1h's P1h.3](../../m1h_beyond_word_problems/README.md#p1h3--gaps-assumptions-and-readings); nothing of
them is built here.

Row 65 of [section J](questions.md#j-findings-that-need-an-owner) (E-18) was left empty and so took its
suggestion, *decided with question 24 and question 30, as one read-out stage*. Question 24 was not ruled
(round 2, items 24, 28 and 37: *nothing is built from 24*), so **this stage takes of E-18 only what a row
can say, and changes no claim.**

## The two findings

- **E-19.** With R empty the `owing` rows print each owed instance with its rule's `:why`. With R not empty
  the `∅` row reads *not in R — root owes 1 instance* and neither the read-out nor `--trace` says which
  instance or why: it is in `--events` alone, the `owe` line. The witnesses are a word problem whose root
  owes a reading of *need* and one whose root owes a quantity. The count is right; the `:why` of the debt,
  which is the named gap, is not printed.
- **E-18.** A query cannot tell an R that is empty because root owes from one that is empty because root is
  dead: each query's row reads *R is empty: no state to answer from*, and a query's `:expect (false)`
  **holds** on a program that only owes, though root is alive and `ein solve --single` says `Open`. Only
  the region claim tells the two apart.

## What it builds

**The row.** Where root is alive and owes, the read-out names each instance root owes, with its rule's
`:why` rendered, whether R is empty or not: the same rows the empty case prints today, under the `∅` row.
[Round 2, item 30](questions_round2.md#q30) is the reason it matters: *the gap written as a debt is
monotone, because it is the engine and not a rule that says "missing"*, and with this row the monotone form
says what the non-monotone one says.

**A query's row over an empty R says which empty.** Where R is empty and root is alive, a query's row says
that root owes and how many instances, in place of the bare *no state to answer from*; where root is dead it
says so. No claim's outcome moves.

**What it does not build:** the N row for an R that is empty by owing (question 24's option (a)); a core for
an open state; any change to what `Open` or `(false)` mean. Those are
[M1g's P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md).

## Tasks

1. **T1f.6.26.1 — the rows shown first, on real programs,** before the printer moves: today's read-out and
   the proposed one side by side for E-19's two witnesses and E-18's (rebuilt from `build/s1f.6.20/` or
   taken from `examples/word_problems/` where banked), and for one zebra-family program where root owes many
   instances, so the row's length is seen. The listing shares the read-out's line budget (the user's rule of
   2026-10-16: stdout listings share 100 lines, and `--dump-answer` writes the whole read-out); the table
   shows where it is cut and what the cut line says.
2. **T1f.6.26.2 — the printer.** A variant of `ein_render::read_out::Row` for the owed instances under an
   alive root that is not in R, and the query row's two wordings. **In the same commit:**
   [`read_out.md`](../../../docs/kernel/inference/read_out.md)'s table,
   [`read_out_examples.md`](../../../docs/kernel/inference/read_out_examples.md) re-written by
   `utils/read_out_examples.py`, and `ein-cli/tests/read_out_reference.rs`, which fails otherwise.
3. **T1f.6.26.3 — the other surfaces agree.** `--trace` names the same instances; `--json-summary` already
   carries the owing holes, and the task checks that root's are among them and adds nothing if so;
   `--dump-answer`'s read-out file has the rows uncut. `--single`, `--minimal`, `--maximal` and `-n` print
   the row wherever they print the `∅` row.
4. **T1f.6.26.4 — E-18's claim, asked and not changed.** Whether a query's `:expect (false)` should *fail*
   on a program that only owes is a change to what a claim means, and it is question 24's subject. The task
   files it under the next free id with the two witnesses and the default *no change*, and points it at
   M1g's P1g.7.
5. **T1f.6.26.5 — the goldens.** Every corpus program whose root is alive and owes, with R not empty, gains
   rows: they are counted with a command and named in the record before blessing. `lever_invariance` holds
   the read-out under every lever.
6. **T1f.6.26.6 — the pages and the rows:** E-19 closed and E-18 marked *the row built, the claim asked* in
   [`findings.md`](findings.md); `AGENTS.md`; [`word_problems.md`](word_problems.md)'s § 3.5 gets a pointer
   from the stage that edits that page next, not from this one.

## Acceptance

- On E-19's witnesses the read-out names what root owes and why, with R not empty.
- A query's row over an empty R says whether root owes or is dead.
- R, N, H, every claim's outcome and every exit code are the same before and after, on every corpus program
  (the sweep).
- `read_out_reference.rs` holds the page to the printer.

## What it must not do

- **Not change a verdict word or a claim's outcome.**
- **Not print N** in the read-out of an R that is empty by owing: not ruled.
- **Not take a core** where no surface prints one (*minimum work per question*): the row prints the
  obligation's `:why`, which needs none.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` before its
fix. Findings go to [`findings.md`](findings.md) under the next free id; questions to
[`open_questions.md`](../open_questions.md) under the next free id. A change to what a surface prints shows
real computed examples first (T1). Config never changes an answer; every answer exits 0. The gate is
`./run_tests.sh`, then `./run_tests.sh --nightly` after the code commit; `utils/ci_container.sh` if the
review bundle's pages change with the read-out.
