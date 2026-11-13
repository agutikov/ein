# S1h.4.4 — The alternatives not taken, and *answer correct, solution not*: what Ein can say, and the stated limit

**Phase:** [P1h.4](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. An investigation on prototypes that **may end as a stated
limit**, by the milestone README's own words: nothing outside `plans/` and `build/` moves.
**Depends on:** [S1h.4.2](s1h.4.2_the_formula_as_an_object.md), since an alternative is a formula that was
not built; [S1h.3.4](../p1h.3_gaps_assumptions_and_readings/s1h.3.4_readings.md), the two states with one
answer (if that stage has not run, the two states are re-run here from
[S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md)'s
files or the investigation's, and the page says so).

## What it rests on

The user, 2026-11-12 (reply 31), in full:

> reading always chosen, it can't be not chosen: result depends on formula, formula depends on selected
> arith operations, selection depend on reading. The synthesized program must be able answer question about
> the solution, like "why you divided those numbers? why not 3/12, not 3+12, not 12-3, and so on" btw there
> could be the case when answer is correct, but solution is not, right? And finally the solution could be
> correct, but reasoning not - this case is M2 and Ein theory of itself or theory of solutions - so out of
> the scope here.

The milestone README's two questions:

- *The alternatives not taken. "why not 3/12, not 3+12, not 12-3" asks about formulas the theory did **not**
  build. A derivation explains what was derived; an explanation of what was not is a different object (each
  alternative, and the premise it lacks). What Ein can say there is the phase's hardest question and may end
  as a stated limit.*
- *Answer correct, solution not: how a program or a reader tells them apart, given two states with one
  answer; what a "wrong" solution is when both readings are admissible.*

And where it stops: *"the solution could be correct, but reasoning not" is M2's and a theory of solutions',
by the user's own line.*

## What it produces

Prototypes under `build/s1h.4.4/` and the fourth section of `formula_and_calculation.md`: two constructions
of *why not*, each tried on one problem; the cases of *answer correct, solution not*, each a program; and a
plain statement of what Ein can say, what it cannot, and where the user's line falls.

## Tasks

1. **T1h.4.4.1 — the alternatives, enumerated.** For the 12-apple problem, 12 and 3: the formulas of one
   operation over the two numbers, in both orders where order matters, which is the user's list and its
   *"and so on"*. Each with its value by the calculator, or its flag. A table: the formula, its value,
   whether any rule of the word theory builds it for this problem, and whether its value equals the
   answer.
2. **T1h.4.4.2 — *why not*, first construction: the rule that would have built it.** For each alternative,
   the rules of the theory whose conclusion has that operation, and for each such rule the premise that
   this problem does not satisfy: *a sum is built by "together" or by "was given", and the question does not
   ask for a "together" of 12 and 3*. This is S1h.2.3's object, a rule with the premise that fails, turned
   to a rule that concludes instead of one that refutes; the stage uses that prototype's matcher if it
   exists and writes the few lines it needs if not. Where **no** rule of the theory has the operation with
   those operands at all, the answer is a statement about the rule set, *the theory has no rule that reads
   these words as a subtraction*, which a program can say only if it can read its own rules. The theory does
   name its rules as facts (`(words-rule need-plan)`), and that is all it knows of them: the page says so
   and names [M12](../../m12_rules_as_data/README.md) as where the rest would come from.
3. **T1h.4.4.3 — *why not*, second construction: the alternative as a hypothesis.** Each alternative formula
   a candidate the search may commit to, and the word theory's meanings as restrictions on which formula a
   question may have. The states that commit to a wrong formula are dead, and each has a core: N, with one
   minimal dead set an alternative, is a list of *why not* answers, in the engine's own form. Tried on the
   one problem. What it needs that the theory does not have, rules that say when a formula is **not** the
   reading of a sentence, is written down with the count of such rules the one problem took; and whether
   those rules are still *what the words mean* or have become *how to judge a solution*, which is the line
   the user drew. The page puts the prototype on one side of that line or the other and says why.
4. **T1h.4.4.4 — answer correct, solution not: the cases.** Each a program, each with what a query on the
   answer shows and what a query on the formula shows (with S1h.4.2's formula fact):
   - **two admissible readings, one answer** (12 apples): two states, two formulas or one, and one class of
     the answer. Neither is wrong; the program can show that there are two and cannot rank them;
   - **a stated reading contradicted** (*Every apple must be packed*, and the full-boxes formula): the
     state is dead, and the right number does not save it;
   - **a wrong formula that is no reading of anything, with the right number** (*2 + 2* written where the
     problem means *2 × 2*): a claim on the answer holds. The task shows whether a program can call this
     solution wrong at all, and against what. The candidate is an expected formula somebody wrote down,
     claimed by an ordinary `:expect` over the formula fact, with no new syntax: the claim on the answer
     then holds and the claim on the formula fails.
5. **T1h.4.4.5 — the statement.** One table: each of the user's questions in the reply, what Ein says today,
   what it says with the formula as an object, what it could say with each construction, and what it cannot
   say, each *cannot* with the program or the missing piece that shows it. And the line: what falls under
   *"the solution could be correct, but reasoning not"*, with one example of the kind, handed to
   [M2](../../m2_nl_to_ir/README.md) by name and not pursued.
6. **T1h.4.4.6 — the page.**

## Acceptance

- The alternatives of one problem are a table on the page, each with its computed value.
- Each construction is tried on that problem and is on the page with its program and output, or with the
  point at which it could not be written and why.
- The three cases of T4 are each a program with the two queries' rows.
- The statement of T5 is on the page, every *cannot* with its evidence.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not build a theory of solutions.** If the second construction needs one, the page says so and stops.
- **Not enumerate formulas past the one problem's two numbers.** The list is the user's *3/12, 3+12, 12-3*
  and its neighbours, to show the object, not to search a space.
- **Not rank two admissible readings.**
- **Not change `:expect`, a claim form or the read-out.** A claim on a formula is an ordinary claim on a
  fact.
- **Not present a limit as a defect.** *May end as a stated limit* is the plan.

## The standing rules

A wrong answer of the engine met by a prototype is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. A prototype in which a hypothesis leads
to a request to the calculator stays out of the corpus (E-33). Every prototype run carries `-T`. Scratch
stays under `build/s1h.4.4/`. The link check is run before the commit, which changes pages only.
