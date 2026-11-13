# S1h.3.4 — Readings: a reading is always chosen, two states with one answer, and what tells them apart

**Phase:** [P1h.3](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A reading of programs that exist, with scratch variants: nothing
outside `plans/` and `build/` moves.
**Depends on:** [S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md),
which rebuilds `readings.ein` to *always owed*, makes `boxes12.ein` two states with one answer, and brings
back the 13-apple problem. If that stage has not run, this one uses the investigation's `readings_always`
and its two problems from `build/s1f.6.20/p/`, or rebuilds them from
[`word_problems.md` § 4](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#4-a-missing-reading),
and says so.
**Hands on to:** [S1h.4.4](../p1h.4_the_formula_and_the_calculation/s1h.4.4_the_alternatives_not_taken.md),
*answer correct, solution not*.

## What it rests on

The user, 2026-11-12, on a word with two readings (reply 31):

> reading always chosen, it can't be not chosen: result depends on formula, formula depends on selected
> arith operations, selection depend on reading. The synthesized program must be able answer question about
> the solution, like "why you divided those numbers? why not 3/12, not 3+12, not 12-3, and so on" btw there
> could be the case when answer is correct, but solution is not, right? And finally the solution could be
> correct, but reasoning not - this case is M2 and Ein theory of itself or theory of solutions - so out of
> the scope here.

[Round 2, item 31](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q31), to which
the user replied *"yes"*: *with 12 apples and boxes of 3 both readings answer 4, so R is **two states with
one answer**: two solutions, one of which may be the wrong reasoning with the right number. The query's
projection shows one class, 4; the states show the two readings.*

The milestone README: *Readings: "a reading is always chosen", and its consequence, two states with one
answer. What tells a right answer by the wrong reading from a right solution, which is P1h.4's subject from
the other side.*

**The ruling is not re-opened here.** *A reading is always chosen* is the user's. The stage reads what it
gives.

## What it produces

The fourth section of `gaps_assumptions_readings.md`: the two problems under every surface, a table of what
differs between two states that agree on the answer, and the forms that were not chosen set beside the one
that was, on the same problems.

## Tasks

1. **T1h.3.4.1 — the two problems, on every surface.** *12 apples, boxes of 3* and *13 apples, boxes of 3*
   under `ein solve`, `--single`, `--minimal`, `--maximal`, `ein test -v`, `ein oracle`, `--trace` and
   `--dump-answer`: the states, the classes of the query, the `owed` row that names the owed reading
   (S1f.6.26). The first is two states and one class; the second two states and two classes, 4 and 5.
2. **T1h.3.4.2 — what differs between two states with one answer.** For the 12-apple problem, state by
   state: the commitment (`(reads-as need whole-only)`, `(reads-as need pack-all)`); the facts one state
   holds and the other does not; the rule that concluded the answer in each (`ein saturate --dump` on each
   state's file, or its trace); the slice of the answer in each, by
   [S1h.2.1](../p1h.2_slicing/s1h.2.1_what_a_slice_is_today.md)'s script if it exists. And what does *not*
   differ: every row a query on the answer prints. So a reader of the query alone sees one answer and no
   sign of two solutions, and the page says which rows of the read-out do show it.
3. **T1h.3.4.3 — what could make one of them wrong.** Three things a problem can add, each a scratch
   variant, each run:
   - **a sentence that states the reading** (*Every apple must be packed*, the vocabulary's
     `(reads-as need READING)` written as a fact): the other reading's state is dead, with its core;
   - **a second question whose answer differs by reading** (*how many apples are left unpacked?*): the
     classes part, and the two states are told apart by what they answer;
   - **nothing:** both readings are admissible, R holds both, and neither is wrong. The page says this
     plainly: with nothing stated, *the wrong reading* has no meaning inside the program.
4. **T1h.3.4.4 — the forms not chosen, for the record.** The investigation built three ways to write
   readings as hypotheses and two that guess nothing (never owed; owed where it matters; always owed; both
   answers derived; a named gap and no answer). One table over the two problems: states, classes, what
   names the gap, monotone or flagged. It is here so that the comparison the README's acceptance asks for,
   *the forms of a reading compared on the same problems*, is on one page; it recommends nothing, since the
   form is ruled.
5. **T1h.3.4.5 — the page,** closing with what is handed to P1h.4: two states, one answer, and the fact that
   what tells them apart today is the commitment and the rule, and would be the formula if the formula were
   an object ([S1h.4.2](../p1h.4_the_formula_and_the_calculation/s1h.4.2_the_formula_as_an_object.md)).

## Acceptance

- Both problems are on the page with the read-out of every surface named in T1, taken on the day's binary.
- The table of T2 is on the page: what differs between the two states and what does not.
- Each of T3's three variants is a program with its read-out.
- T4's table has every form the investigation built, on both problems.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not re-open the ruling,** and not recommend a form.
- **Not say which reading of *need* is right.** With nothing stated, the program does not know, and the page
  says so.
- **Not add a reading or a word** (reply 35).
- **Not write a theory of solutions.** *"out of the scope here"*.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. A scratch program in which a hypothesis
leads to a request to the calculator stays out of the corpus (E-33; `AGENTS.md` § Edits that go together).
Scratch stays under `build/s1h.3.4/`. The link check is run before the commit, which changes pages only.
