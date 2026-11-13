# S1h.2.2 — The first opening question: assumptions as premises make the slice name them

**Phase:** [P1h.2](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A test on scratch copies: no source under `ein.rs/` and nothing
outside `plans/` and `build/` moves. Its answer is shown at
[S1h.2.4](s1h.2.4_what_used_means_and_the_stop.md).
**Depends on:** [S1h.2.1](s1h.2.1_what_a_slice_is_today.md)'s script.
**Read by:** [S1h.3.2](../p1h.3_gaps_assumptions_and_readings/s1h.3.2_assumptions_marked_supposed_and_universal.md)
and [S1h.3.3](../p1h.3_gaps_assumptions_and_readings/s1h.3.3_which_assumptions_a_model_has_to_state.md),
which takes the census of every rule's assumptions from where this stage leaves it.

## The question it rests on

The user, 2026-11-12, at the end of reply 33:

> Btw, if we add rules and ontology that restricts throwing things away and after that the solution will not
> touch those rules and facts (see my answer 27) - that means this is not required information?

[Round 2, item 33](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q33), to which
the user replied *"OK"*:

> If an assumption is a fact that the dependent rules take as a premise, then the slice of an answer (your
> phase from 27) contains exactly the assumptions that answer rests on, with no extra mechanism, and one the
> derivation never touches is not required for *that* answer.

So the answer is *yes, for that answer*, **provided** assumptions are written that way. The milestone README:
*The phase tests it on the banked problems.*

## What it starts from

[`words.ein`](../../../examples/word_problems/words.ein) states what its model leaves out once, as one fact,
`(model-counts-only)`, whose `:source` holds three clauses: things are counted by number alone; any thing of a
kind fits any container that holds that kind; nobody gives away what they do not have. Two rules take it as a
premise, `need-exact` and `gave-more-than-had-is-false`. The calculator's investigation recommended more
([`word_problems.md` § 5](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#5-the-models-boundary)):
*a theory that ships would have one per assumption, so that an answer cites only those it used.* On
2026-11-13 the `-l` trace of `boxes12.ein` names the fact and those of `bob1.ein`, `unused.ein` and
`bystander.ein` do not.

## What it produces

A scratch copy of the word theory under `build/s1h.2.2/` with the one fact split, the banked problems run
over it, and the second section of `slicing.md`: a table of problem against assumption, and the user's
question answered on a program.

## Tasks

1. **T1h.2.2.1 — one fact an assumption.** In the scratch copy, the three clauses become three facts, each
   with its own relation, `:why` and `:source`, and each rule that takes `(model-counts-only)` takes instead
   the one or two its conclusion depends on: *any thing fits* for `need-exact`; *nobody gives away what they
   do not have* for `gave-more-than-had-is-false`. No rule gains a premise it did not have in substance: the
   full census, rule by rule, is S1h.3.3's.
2. **T1h.2.2.2 — the table.** Every banked problem over the scratch theory: its answer is the banked one
   (`ein test` on scratch copies), and S1h.2.1's script gives, for the answer to its first query, which of
   the three assumption facts are in the used part. One row a problem, one column an assumption, *used* or
   *not used*; for `contra.ein`, whose root is dead, the column is read off its core.
3. **T1h.2.2.3 — the user's example, run.** *"rules and ontology that restricts throwing things away"*: a
   fourth assumption added to the scratch theory, with a relation for throwing away and a rule that depends
   on it, and two problems, one whose text has a throwing-away and one, `bob1`, whose text has none. The
   slice of `bob1`'s answer does not hold the new rule or its fact; the slice of the other's does. The page
   states the answer in the user's words and the round's: *not required information*, **for that answer**.
4. **T1h.2.2.4 — the control: an assumption as text only.** The same assumption written only in a relation's
   `:why` or a query's `:goal-text` (option (b) of the calculator's question 33). The script's cut of the
   same answer holds nothing of it, though the answer rests on it as much. So the proviso is real: the slice
   names an assumption **only** where it is a premise.
5. **T1h.2.2.5 — what the test does not show.** Two limits, written down and handed on: an assumption whose
   rule *tests* and does not conclude (*nobody gives away what they do not have*, on a problem where nobody
   does) is in no slice of `bob1`'s answer, which is
   [S1h.2.3](s1h.2.3_a_restriction_that_was_checked_and_held.md)'s question; and whether each rule has the
   right assumptions as premises is a modelling judgement no slice checks, which is S1h.3.3's.
6. **T1h.2.2.6 — the page.**

## Acceptance

- Every banked problem answers as banked over the scratch theory.
- The table has a row for every banked problem and a column for each assumption, each cell computed by the
  script.
- The throwing-away example is two programs on the page, with the cut of each.
- The control is a program on the page whose cut holds nothing of the assumption.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not edit `examples/word_problems/words.ein`.** Whether the banked theory takes one fact an assumption is
  [Q-M1h.14](../open_questions.md#q-m1h14--which-of-the-banked-forms-change), put at P1h.3's stop.
- **Not add an assumption because it sounds right.** The three are the banked fact's three clauses, and the
  fourth is the user's example.
- **Not claim the slice is complete.** T5 says where it is not.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every program states its lattice. Scratch
stays under `build/s1h.2.2/`. The link check is run before the commit, which changes pages only.
