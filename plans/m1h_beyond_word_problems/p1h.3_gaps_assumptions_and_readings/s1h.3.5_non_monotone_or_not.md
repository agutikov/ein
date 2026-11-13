# S1h.3.5 — Non-monotone or not: round 2's distinction held to programs, and the three questions of reply 30

**Phase:** [P1h.3](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A check of programs the earlier stages wrote: nothing outside
`plans/` and `build/` moves.
**Depends on:** [S1h.3.1](s1h.3.1_the_three_forms_of_a_gap.md) to [S1h.3.4](s1h.3.4_readings.md), whose
programs it runs through one check.
**Read with:** [F21](../../followups/f21_non_monotone_programs_decided.md), which owns what a non-monotone
program means, and [`monotonicity.md`](../../../docs/kernel/inference/monotonicity.md), the kernel page.

## What it rests on

The user, 2026-11-12, on a named gap together with a supposition (reply 30):

> that simple problem couldn't have non-monotone solution, I mean the problem is too simple and too obvious,
> or not? or that means that simple problem, containing ambiguity, could already be non-monotone? Or that
> means ambiguity == non-monotone solution == required additional information? Again - those questions go
> into M1h. Here I don't care - so pick (a).

[Round 2, item 30](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q30), to which
the user replied *"OK (a)"*:

> Not quite, and the difference is worth having for M1h. **Ambiguity is monotone**: R has several states and
> nothing is retracted. What is non-monotone is one sentence: *Alice's amount is **not stated***. It is true
> of the problem as given and false as soon as anything supposes or supplies the amount; any statement of
> the form *X is unknown* is retracted by learning X. So:
>
> - the gap written **as a fact** (*not stated*, by `absent`) is non-monotone, for every problem, however
>   simple;
> - the gap written **as a debt** (the state owes the amount) is monotone, because it is the engine and not
>   a rule that says *missing*, outside the facts.
>
> […] So: *required additional information* = a debt; *ambiguity* = several states; neither needs
> non-monotone rules unless the program wants the missing thing as a fact to reason from.

The milestone README: *Non-monotone or not: round 2's distinction held to programs, with F21, which owns what
a non-monotone program means.* And the user's direction on the subject (2026-11-02, F21's first lines):
detect, document and warn now; decide nothing; *"leave further investigation of non-monotonic lattices as
followup"*.

## What it produces

The fifth section of `gaps_assumptions_readings.md`: one table over every form the phase wrote, the three
sentences of the reply each with the program that answers it, and a list handed to F21.

## Tasks

1. **T1h.3.5.1 — every form through one check.** Each program of S1h.3.1 to S1h.3.4 (the three forms of a
   gap, with a supposition and without; the two ways to write an assumption; *what is not listed is zero*;
   each form of a reading) under `ein solve --monotone`, and, where the static check flags it, under a bare
   `ein solve`, whose `lattice` block says what the cube showed: the sites, what each puts at stake, a hole,
   a stale fact ([`read_out.md` § 5.1](../../../docs/kernel/inference/read_out.md#51-under-lattice-and-monotone-the-warning)).
   One table: the form, the verdict, the site if any, and whether the answer a reader takes from the
   read-out is affected.
2. **T1h.3.5.2 — the reply's three sentences, each on a program.**
   - *"that simple problem couldn't have non-monotone solution, I mean the problem is too simple and too
     obvious, or not?"*: the simplest problem of the set
     in the form that is flagged, with what the flag is about (the rule that reads an `absent`, and the
     relation a commitment can change) and whether its region has a hole or a stale fact.
   - *"simple problem, containing ambiguity, could already be non-monotone?"*: a problem that is ambiguous
     and monotone (the readings, always owed), and one that is ambiguous and flagged, side by side, with
     what makes the difference: not the ambiguity.
   - *"ambiguity == non-monotone solution == required additional information?"*: the three, each as the
     program that has it and not the other two, where such a program can be written: several states and no
     debt and no flag; a flag and one state; a debt and no flag.
3. **T1h.3.5.3 — round 2's outline, held or corrected.** Its three claims, each against T1's table: the gap
   as a fact is non-monotone *for every problem, however simple* (is it flagged where no hypothesis exists
   at all, and does the flag then matter); the gap as a debt is monotone; *neither needs non-monotone rules
   unless the program wants the missing thing as a fact to reason from*. Each marked *held* or *corrected*,
   with the program.
4. **T1h.3.5.4 — what the program wants the fact for.** [S1h.3.1](s1h.3.1_the_three_forms_of_a_gap.md)'s
   third task showed which forms a rule can read. Here the two findings are joined: the form a rule can
   reason from is the one that is flagged beside a supposition. The page says what a theory gives up by
   staying monotone (which sentences of a read-out become rows of the engine's and stop being facts of the
   program), on the banked `gap.ein` and its debt twin.
5. **T1h.3.5.5 — what is handed to F21.** Read F21's analysis and its decisions D1 to D7. For each program
   of this phase that the static check flags: which of F21's decisions would change what it prints, and
   how, as far as F21's page says. A list, on this phase's page; F21's file gets one line that points to it
   and nothing else. **No decision of F21's is taken or proposed.**
6. **T1h.3.5.6 — the page.**

## Acceptance

- Every form the phase wrote is in T1's table with the static check's verdict, taken on the day's binary.
- Each of the reply's three sentences has a program beside it, or the page says that no such program can be
  written and why.
- Round 2's three claims are each marked held or corrected.
- The list for F21 is on the page.
- Nothing outside `plans/` and `build/` moved, but for one pointer line in F21's file.

## What it must not do

- **Not decide what a non-monotone program means,** nor which node function is right, nor what is reported
  where R has holes. F21's, and the user's.
- **Not call the lattice non-monotone.** The user's correction of 2026-11-02 stands: the lattice is the
  domain; what is monotone or not is a function of its node, or the program.
- **Not change the static check, *forbid* or a warning's wording.**
- **Not ask the user F21's questions at this phase's stop.** They are parked where they will be revisited.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Scratch stays under `build/s1h.3.5/`. The
link check is run before the commit, which changes pages only.
