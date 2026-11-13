# P1h.3 — Gaps, assumptions and readings: what a problem leaves out, and how a program says so

**Milestone:** [M1h](../README.md)
**Status:** planned 2026-11-12 as a section of the milestone's README, on the user's replies of that day
([§ Where it comes from](#where-it-comes-from)); **written out as this directory on 2026-11-13,** on the
user's instruction of that day (*"materialize M1h phases"*). Not started. It carries no estimate (the user's
ruling of 2026-11-08). It is an investigation on programs: five stages each compare the forms of one thing on
the same problems, the sixth **stops for the user**, and the seventh, whose tasks are written at the stop,
builds what the user rules.
**Depends on:** [M1f](../../m1f_hypothesis_and_documentation/README.md)'s calculator phase:
[S1f.6.26](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.26_what_root_owes.md), the read-out
row for what root owes (done 2026-11-13), and
[S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md),
which rebuilds the readings to *always owed* and banks `listed_is_all.ein` (done 2026-11-13: the files are
banked under `examples/word_problems/`).
**Reads:** [P1h.2](../p1h.2_slicing/README.md)'s slice, for how an answer reports what it rests on;
[M1g's P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md), which rules open against false;
[F21](../../followups/f21_non_monotone_programs_decided.md), which owns what a non-monotone program means.
None is waited for ([§ The order](#the-order)).
**Blocks:** [P1h.4](../p1h.4_the_formula_and_the_calculation/README.md) reads its page on readings.

The milestone README's section, [§ P1h.3](../README.md#p1h3--gaps-assumptions-and-readings), is the phase's
source and is kept as written. This page carries the user's replies in full, what is banked today, and the
stages.

## Where it comes from

Six replies of 2026-11-12 to the calculator's
[`questions.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions.md#q28), on what a problem
leaves out. Each in full.

**Question 28, how a missing quantity is detected** (*Bob has 5, gave 2 to Alice; how many together?* Nothing
says what Alice had):

> see my answer in question 24; in general - need to discuss, analyze, needs a separate stage. But I want
> already written word problems banked as is, so pick recommended, and note that is is not only and not
> final.

**Question 29, how an assumption is written and reported** (*Assume that Alice had none.*):

> again - for this stage (a) is OK, return back to investigations of this in M1h

**Question 30, a named gap together with a supposition:**

> that simple problem couldn't have non-monotone solution, I mean the problem is too simple and too obvious,
> or not? or that means that simple problem, containing ambiguity, could already be non-monotone? Or that
> means ambiguity == non-monotone solution == required additional information? Again - those questions go
> into M1h. Here I don't care - so pick (a).

**Question 31, a word with two readings** (*13 apples, a box holds 3: how many boxes do they need?*):

> reading always chosen, it can't be not chosen: result depends on formula, formula depends on selected
> arith operations, selection depend on reading. The synthesized program must be able answer question about
> the solution, like "why you divided those numbers? why not 3/12, not 3+12, not 12-3, and so on" btw there
> could be the case when answer is correct, but solution is not, right? And finally the solution could be
> correct, but reasoning not - this case is M2 and Ein theory of itself or theory of solutions - so out of
> the scope here.

**Question 32, is *Alice has 3 pears* also *Alice has no apples*:**

> what is not listed is zero - good idea, it could be a reusable universal assumption.

and, in round 2, to the proposal that it be a module a problem imports when it adopts it (`dc88520b`):

> Yes, relation arguments are marked with some objects that can be treated as types. So can add a rule that
> assigns zero to every unknown and deduce (has Alice apples 0). It doesn't work in zebra for example - if
> nobody lives in the house means we yet don't know who lives there.

**Question 33, how the theory states what its model leaves out:**

> Is this a finite set of assumptions? What is it limited by? Used names/words? Probably we don't need to say
> that things are not alive and don't run out of the boxes, becasue the problem doesn't contain words "alive"
> and "run", or do we?
> But the same time we assume things fit boxes, while problem doesn't have this word.
> Btw, if we add rules and ontology that restricts throwing things away and after that the solution will not
> touch those rules and facts (see my answer 27) - that means this is not required information?

**What round 2 answered in outline,** which the user accepted there (`dc88520b`: *"OK (a)"* on item 30, *"OK"*
on item 33) and which the phase starts from and re-takes on programs:

- [Item 30](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q30), on the user's
  chain *ambiguity == non-monotone solution == required additional information*: *Not quite, and the
  difference is worth having for M1h.* **Ambiguity is monotone:** R has several states and nothing is
  retracted. What is non-monotone is one sentence, *Alice's amount is **not stated***, true of the problem as
  given and false as soon as anything supposes or supplies the amount. The gap written **as a fact** is
  non-monotone, for every problem, however simple; the gap written **as a debt** is monotone, because it is
  the engine and not a rule that says *missing*. So: *required additional information = a debt; ambiguity =
  several states; neither needs non-monotone rules unless the program wants the missing thing as a fact to
  reason from.*
- [Item 33](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q33), on whether the
  set of assumptions is finite: *In the world the set is not finite (the frame problem). In a theory it is
  bounded by the rules, not by the text's words. An assumption has to be stated exactly where a rule's
  conclusion depends on it.* *Things fit boxes* is needed although the text never says *fit*, because the
  rule that reads *a box holds 3* as a division is valid only under it; *things are not alive* is needed by
  no rule. *The set grows with the vocabulary the theory covers, a few per rule.*

These outlines are the coordinator's answers, accepted by the user in a word each. They are **claims to be
held to programs**, and the stages below are where each is held or corrected.

## What it is

Five questions, each answered with programs, as the milestone README lists them:

| the question | stage |
|---|---|
| **The three forms of a gap, compared properly:** a debt, a fact by `absent`, a hypothesis over a list. What each says in the read-out, what each lets a rule reason from, and what each costs; with the open-against-false question | [S1h.3.1](s1h.3.1_the_three_forms_of_a_gap.md) |
| **Assumptions:** marked facts against hypotheses the search may commit to; which are *universal* and reusable, and what decides whether a relation may adopt one; whether such assumptions are a stdlib module; how an answer reports what it rests on | [S1h.3.2](s1h.3.2_assumptions_marked_supposed_and_universal.md) |
| **Which assumptions a model has to state at all:** for each rule of the word theory, the assumption that makes its reading valid, written as a premise; then the count, and whether *a few per rule* holds | [S1h.3.3](s1h.3.3_which_assumptions_a_model_has_to_state.md) |
| **Readings:** *a reading is always chosen*, and its consequence, two states with one answer. What tells a right answer by the wrong reading from a right solution | [S1h.3.4](s1h.3.4_readings.md) |
| **Non-monotone or not:** round 2's distinction held to programs, with F21 | [S1h.3.5](s1h.3.5_non_monotone_or_not.md) |

## What it is not

- **Not the ruling on open against false.** That is M1g's P1g.7
  ([Q-M1g.13](../../m1g_logic/open_questions.md#q-m1g13--may-a-finished-search-call-an-unpayable-debt-dead)).
  The phase shows a missing amount and an equation with no root side by side and decides nothing.
- **Not a decision of F21's.** The user's direction of 2026-11-02: detect, document, warn, and decide
  nothing. The phase detects, on its own programs, and hands what it finds to F21.
- **Not a redesign of the banked word problems before the stop.** They were banked *"as is"*, and what they
  become is [Q-M1h.14](../open_questions.md#q-m1h14--which-of-the-banked-forms-change).
- **Not an enumeration of words or of the world's assumptions.** Reply 35 of the same day: *"We don't
  enumerate them all, at least now."* The census of S1h.3.3 is over the rules the word theory has.
- **Not a theory of solutions.** *"the solution could be correct, but reasoning not - this case is M2 and Ein
  theory of itself or theory of solutions - so out of the scope here."*
- **Not new syntax, and not a change to what a surface prints.** If a form wants a row the read-out lacks,
  that is a finding, shown.

## What it starts from

As the tree stands on 2026-11-13. Each is re-taken by the stage that uses it.

- **What is banked, and is *"not only and not final"*:**
  the gap as a derived fact by `absent`, in a module of its own
  ([`gaps.ein`](../../../examples/word_problems/gaps.ein): `not-stated`, `unanswered`, `rests-on`); an
  assumption as a marked fact, `(assumed QUANTITY NUMBER)`, with a `rests-on` fact beside the answer
  (`gap_assume.ein`, `kinds_assume.ein`); readings as hypotheses
  ([`readings.ein`](../../../examples/word_problems/readings.ein), with nothing owed today and always owed
  after S1f.6.28); *what is not listed is zero* as a module a problem imports (`listed_is_all.ein`, banked by
  S1f.6.28); the model's boundary as one fact with a `:source`, a premise of two rules
  ([`words.ein`](../../../examples/word_problems/words.ein)'s `model-counts-only`).
- **The read-out row for what root owes** (S1f.6.26, done 2026-11-13): where root is alive, owes, and R is
  not empty, `owed` rows under `∅` name each instance through its rule's `:why`
  ([`read_out.md` § 2](../../../docs/kernel/inference/read_out.md#2-describe-a-bare-ein-solve---count-and-ein-oracle)).
  So the calculator's table of what each form of a gap prints
  ([`word_problems.md` § 3.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#36-what-each-mechanism-does))
  is out of date in one cell, *the gap is not printed*, and S1h.3.1 re-takes it whole.
- **The forms that were tried and not banked.** The investigation's prototypes, `gap_a` (an obligation),
  `gap_c` (a hypothesis over a stated list), `gap_assume_h`, `_ha` and `_hb` (the assumption as a
  hypothesis, alone and with each form of the gap), `readings_always`, `readings_both`, `readings_gap`, stand
  under `build/s1f.6.20/p/` on 2026-11-13. That directory is scratch and not tracked: where it is gone, each
  is rebuilt from [`word_problems.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md),
  which carries the theory in full (§ 12) and each variant's rule.
- **The stale fact.** With the gap as a fact and the assumption as a hypothesis (`gap_assume_hb`), the state
  that supposes Alice's amount still holds *not stated*, and the read-out's warning names it
  ([`word_problems.md` § 3.5](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#35-the-gap-closed-by-a-stated-assumption)).
  It is F21's open question met on a program a user would write.
- **Two findings of the calculator's phase** that bear on a gap as a debt: E-18, a query's `:expect (false)`
  holds on a program that only owes
  ([Q-M1f.219](../../m1f_hypothesis_and_documentation/open_questions.md#q-m1f219--a-querys-false-on-a-program-that-only-owes),
  defaulted to no change and P1g.7's), and E-44, on an R empty by owing root's own debts are on no stdout row
  where root is not on the frontier
  ([`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md)).

**Names.** [S1f.6.31](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md)
moves the calculator's modules under `std.math`; where this phase's files say `std.arith`, read
`std.math.decimal-arith` once that stage has run.

## The stages

| ID | title | depends on | stops for the user |
|---|---|---|---|
| [S1h.3.1](s1h.3.1_the_three_forms_of_a_gap.md) | The three forms of a gap, on the same problems: a debt, a fact by `absent`, a hypothesis over a list | S1f.6.26 | no |
| [S1h.3.2](s1h.3.2_assumptions_marked_supposed_and_universal.md) | Assumptions: a marked fact against a hypothesis, the universal ones, and how an answer reports what it rests on | S1h.3.1; S1f.6.28 for `listed_is_all.ein`; [S1h.2.1](../p1h.2_slicing/s1h.2.1_what_a_slice_is_today.md)'s script | no; [Q-M1h.15](../open_questions.md#q-m1h15--universal-assumptions-as-a-module) is put at S1h.3.6 |
| [S1h.3.3](s1h.3.3_which_assumptions_a_model_has_to_state.md) | Which assumptions a model has to state: every rule of the word theory with the assumption that makes its reading valid, and the count | [S1h.2.2](../p1h.2_slicing/s1h.2.2_assumptions_as_premises.md) | no |
| [S1h.3.4](s1h.3.4_readings.md) | Readings: a reading is always chosen, two states with one answer, and what tells them apart | S1f.6.28's rebuilt readings | no |
| [S1h.3.5](s1h.3.5_non_monotone_or_not.md) | Non-monotone or not: round 2's distinction held to programs, and the three questions of reply 30 | S1h.3.1 to S1h.3.4 | no |
| [S1h.3.6](s1h.3.6_the_page_and_the_stop.md) | The page and the stop | S1h.3.1 to S1h.3.5 | **yes: the phase's stop.** [Q-M1h.14](../open_questions.md#q-m1h14--which-of-the-banked-forms-change), [Q-M1h.15](../open_questions.md#q-m1h15--universal-assumptions-as-a-module). Nothing outside `plans/` and `build/` is written before the user has read its page |
| [S1h.3.7](s1h.3.7_what_the_rulings_build.md) | What the rulings build | the rulings of S1h.3.6 | no; **its tasks are written at S1h.3.6** |

No stage carries an estimate. Findings go to `findings.md` in this directory, opened at the first one
([M1h § Stage, phase, milestone](../README.md#stage-phase-milestone)).

## The order

S1h.3.1 first, since the other four use its problems and its table. S1h.3.2, S1h.3.3 and S1h.3.4 after it, in
any order. S1h.3.5 after those three, since it puts every form they wrote through one check. S1h.3.6, and it
stops. S1h.3.7 on the rulings.

Against other phases and milestones, stated and not resolved:

- **P1h.2.** S1h.3.2 and S1h.3.3 read the slice. They need S1h.2.1's script and S1h.2.2's split of the
  model's fact; they do not need a surface. If P1h.2 has not started, S1h.3.3 does S1h.2.2's first task
  itself and says so, and S1h.2.2 then starts from it.
- **M1g's P1g.7.** If [Q-M1g.13](../../m1g_logic/open_questions.md#q-m1g13--may-a-finished-search-call-an-unpayable-debt-dead)
  has been ruled, S1h.3.1 shows the gap as a debt under the ruling. If not, it shows the debt as the engine
  reports it today and sets beside it the two reasons a debt goes unpaid, which is the evidence P1g.7's third
  stage asks for; it decides nothing.
- **P1h.1.** *Soft* and *unknown* in time are this phase's forms of a gap and of a reading. Whichever of
  [S1h.1.3](../p1h.1_temporal_and_causal_theories/s1h.1.3_the_temporal_theory_on_a_prototype.md) and S1h.3.1
  runs first writes the forms; the other re-reads them.
- **F21.** S1h.3.5 reads its analysis and its decisions D1 to D7 and takes none.
- **Where M1h sits against M1g** is [Q-M1h.1](../open_questions.md#q-m1h1--where-m1h-sits-against-m1g).

## Acceptance

- **S1h.3.6's page was read and ruled on by the user** before any file outside `plans/` and `build/` was
  written.
- **The forms of a gap, of an assumption and of a reading are each compared on the same problems,** every
  cell of every table read off a run.
- **The user's questions of reply 30 are answered with programs,** each of its three sentences with the
  program that answers it.
- **The user's questions of reply 33 are answered with programs:** whether the set is finite and what limits
  it, on the census; *fit* and *alive*, each on a rule or on the absence of one; and the last, on a slice.
- **What decides whether a relation may adopt *what is not listed is zero*** is stated as a condition and
  tested on a relation that may and on a Zebra relation that may not.
- **Round 2's two outlines are each marked held, or corrected,** with the program.
- **The repo's discipline:** a wrong answer of the engine is a row of
  [`corpus/wrong_answers.toml`](../../../corpus/wrong_answers.toml), filed `open` before its fix; every other
  finding is a row of this phase's `findings.md`, shown; every number carries its command and date.

## Risks, each with what checks it

- **The comparison is of prototypes that have drifted.** The investigation's files are scratch of
  2026-11-11, and the read-out and the calculator have changed since. Every program is re-run on the day's
  binary by the stage that uses it, and a cell that changed is marked.
- **An assumption is a judgement.** Which assumption makes a rule's reading valid is written by whoever
  writes the census, and no run checks it. S1h.3.3 puts the whole list on the page for the user to read, and
  counts nothing it does not list.
- **A form that reads best is not monotone.** The gap as a fact beside a hypothesis leaves a stale fact.
  S1h.3.5 puts every form through `ein solve --monotone` and the cube, and the stop does not recommend a
  flagged form without saying so.
- ***Universal* is taken for *true everywhere*.** The user names where it fails. S1h.3.2 states the
  condition for adopting it and tests the failing case, so that the module, if there is one, cannot be read
  as a default.
- **Two phases answer one question twice.** *Soft* and *unknown* (P1h.1), *right answer, wrong reading*
  (P1h.4). Each is asked once and cited; [§ The order](#the-order) says by whom.

## Connections

- **[P1f.6](../../m1f_hypothesis_and_documentation/p1f.6_calculator/README.md):** replies 28 to 33, round 2's
  items 30 to 33, the investigation's page and prototypes, S1f.6.26's row and S1f.6.28's rebuild.
- **[P1h.2](../p1h.2_slicing/README.md):** the slice, through which an answer reports its assumptions.
- **[P1h.4](../p1h.4_the_formula_and_the_calculation/README.md):** *answer correct, solution not* is its
  subject; S1h.3.4 hands it the two states with one answer.
- **[M1g](../../m1g_logic/README.md):** P1g.7, unsatisfiable against owing; P1g.8, a theory of types, which
  the user's *"relation arguments are marked with some objects that can be treated as types"* touches.
- **[F21](../../followups/f21_non_monotone_programs_decided.md):** non-monotone programs.
- **[F26](../../followups/f26_consistency_and_completeness.md):** its senses of *complete*, which a text
  that leaves things unsaid meets at once.
- **[Idea 03](../../ideas/03-three-task-classes.md):** a gap is class B.
- **[`open_questions.md`](../open_questions.md):** Q-M1h.14 and Q-M1h.15.
