# S1h.3.1 — The three forms of a gap, on the same problems: a debt, a fact by `absent`, a hypothesis over a list

**Phase:** [P1h.3](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A comparison on prototypes: nothing outside `plans/` and
`build/` moves.
**Depends on:** [S1f.6.26](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.26_what_root_owes.md),
the read-out row for what root owes (done 2026-11-13).
**Read with:** [M1g's P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md), which rules open
against false, and [S1h.1.3](../p1h.1_temporal_and_causal_theories/s1h.1.3_the_temporal_theory_on_a_prototype.md),
which meets the same three forms in time.

## What it rests on

The user, 2026-11-12, on how a missing quantity is detected (reply 28):

> see my answer in question 24; in general - need to discuss, analyze, needs a separate stage. But I want
> already written word problems banked as is, so pick recommended, and note that is is not only and not
> final.

The answer in question 24 it points to, in full:

> solution of `x × 7 = 862` belongs yet to arithmetic or already to algebra? this question open vs. false is
> deep. What would give more benefits, I expect `open` will, but why? Can we change it quickly later? We
> already have core for dead state, do we have anything like this for open? What it could be used for, I
> remember we already use it for hypothesis generation? It is not the case here - we would not generate
> hypothesis like (let x <number>) for every numbers already present in KB. It looks like an entire separate
> phase.

The milestone README: *The three forms of a gap, compared properly: a debt, a fact by `absent`, a hypothesis
over a list. What each says in the read-out, what each lets a rule reason from, and what each costs; with the
open-against-false question (the user's "see my answer in question 24"), which is ruled in M1g's P1g.7: a
missing amount that is simply not stated and an equation with no root both leave a debt unpaid, for different
reasons.*

## What it starts from

The calculator's investigation ran the three forms once, on its prototype of 2026-11-11
([`word_problems.md` § 3](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#3-a-missing-quantity)):
`gap_b` (a fact by `absent`, banked as `gap.ein` over
[`gaps.ein`](../../../examples/word_problems/gaps.ein)), `gap_a` (an obligation, the rule `quantity-owed`
concluding `(open value)`), `gap_c` (a hypothesis over a list of numbers the problem file states), and their
table, § 3.6. Since then the read-out names what root owes beside the states that pay it (S1f.6.26), so the
table's cell *the gap is not printed* is out of date.

## What it produces

The three forms as scratch programs under `build/s1h.3.1/`, on the same problems, and the first section of
`gaps_assumptions_readings.md` in this directory: one table, re-taken whole, with three columns the
investigation's did not have.

**The problems.** *Bob has 5 apples. He gave 2 to Alice. How many apples do they have together?* (the
user's); *Bob has 5 apples, Alice has 3 pears. How many apples together?*; and one from outside arithmetic's
gaps, an equation with no root, from
[`examples/beyond_arith/equation_no_solution.ein`](../../../examples/beyond_arith/equation_no_solution.ein).

## Tasks

1. **T1h.3.1.1 — the three forms, re-run.** Each problem in each form on the day's binary, under
   `ein solve`, `ein solve --single`, `ein test -v`, `ein saturate --dump` and `--trace`, every output kept
   under `build/s1h.3.1/out/`. The prototypes are taken from `build/s1f.6.20/p/` where that scratch
   directory stands, and rebuilt from the investigation's page where it does not.
2. **T1h.3.1.2 — what each says in the read-out.** For each form: is what *is* known answered (Bob's 3); is
   *together* said to have no answer; is the gap named, and on which row (a query row for the fact; the
   `owed` rows or the `owing` rows for the debt; nowhere for a bare hypothesis). The investigation's § 3.6
   re-taken cell by cell, each changed cell marked with what changed it.
3. **T1h.3.1.3 — what each lets a rule reason from.** One rule that wants *this amount is missing* as a
   premise, to conclude that the question has no answer and to say which sentence would supply it, tried
   against each form:
   - the fact: `(not-stated Q)` is a premise like any other;
   - the debt: an obligation rule asserts `open`, derives nothing and is in no saturation agenda, so no rule
     can match what a state owes. The task shows the attempt and what the loader or the run says;
   - the hypothesis: each guess is a state; *missing* is the state with no guess, and no fact in it says so.
4. **T1h.3.1.4 — what each costs.** Strata; candidates; saturations; the static check's verdict
   (`ein solve --monotone`); and what happens when the same file also holds a supposition, which is where
   the forms part: the fact leaves a stale fact, the debt stays monotone
   ([S1h.3.5](s1h.3.5_non_monotone_or_not.md) takes the reason).
5. **T1h.3.1.5 — two reasons a debt goes unpaid.** The word problem's debt (no sentence states the amount,
   and no candidate would pay it) beside the equation's (candidates exist, and every complete way of paying
   is dead): what the read-out, `--single`, the `owed` and `owing` rows and `nogoods.json` say of each, and
   whether anything a run prints tells the two apart. Each of the user's four questions in the reply above
   is answered on these two programs as far as they reach: what `Open` keeps; what there is for an open
   state as a core is for a dead one; what the obligations are used for; why the generator proposes no
   number from the KB. **If P1g.7 has ruled,** the task reads its ruling and shows the two programs under
   it. **If it has not,** the task decides nothing: the section is written so that P1g.7's third stage can
   cite it as its word-problem witness.
6. **T1h.3.1.6 — the page.** The table, the three attempts of T3, and one paragraph a form on when it is the
   one to write.

## Acceptance

- Each problem is on the page in each form it can be written in, with its read-out, or with the reason it
  cannot.
- The table has every column of the investigation's § 3.6 and three more: *a rule can read it*, *monotone
  beside a supposition*, *tells not-stated from no-root*.
- Every cell is read off a run of the day, with the command.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not change `gaps.ein` or any banked problem.** The banked form is *"not only and not final"*, and what
  it becomes is [Q-M1h.14](../open_questions.md#q-m1h14--which-of-the-banked-forms-change).
- **Not rule on open against false,** and not on whether a query's `(false)` should fail on a program that
  only owes (Q-M1f.219). Both are P1g.7's.
- **Not make a rule read a debt by a detour** (a fact that mirrors what the obligation asserts) and call the
  debt readable. If the detour is the only way, the page shows it as a fourth form and says what it costs.

## The standing rules

A wrong answer of the engine met by a prototype is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to `findings.md` in this directory, opened at the first;
questions to [`open_questions.md`](../open_questions.md) under the next free id. Every program states its
lattice. Scratch stays under `build/s1h.3.1/`. The link check is run before the commit, which changes pages
only.
