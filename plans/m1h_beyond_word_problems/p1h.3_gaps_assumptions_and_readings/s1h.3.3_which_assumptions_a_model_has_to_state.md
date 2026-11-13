# S1h.3.3 — Which assumptions a model has to state: every rule of the word theory with the assumption that makes its reading valid, and the count

**Phase:** [P1h.3](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A census on a scratch copy: nothing outside `plans/` and
`build/` moves.
**Depends on:** [S1h.2.2](../p1h.2_slicing/s1h.2.2_assumptions_as_premises.md), which splits the model's one
fact into one an assumption and runs the slice over it. If that stage has not run, this one does its first
task itself, says so, and S1h.2.2 then starts from it.

## What it rests on

The user, 2026-11-12, on how the theory states what its model leaves out (reply 33):

> Is this a finite set of assumptions? What is it limited by? Used names/words? Probably we don't need to say
> that things are not alive and don't run out of the boxes, becasue the problem doesn't contain words "alive"
> and "run", or do we?
> But the same time we assume things fit boxes, while problem doesn't have this word.
> Btw, if we add rules and ontology that restricts throwing things away and after that the solution will not
> touch those rules and facts (see my answer 27) - that means this is not required information?

[Round 2, item 33](../../m1f_hypothesis_and_documentation/p1f.6_calculator/questions_round2.md#q33), to which
the user replied *"OK"*:

> **In the world the set is not finite** (the frame problem). **In a theory it is bounded by the rules, not
> by the text's words.** An assumption has to be stated exactly where a rule's conclusion depends on it.
> *Things fit boxes* is needed although the text never says *fit*, because the rule that reads *a box holds
> 3* as a division is valid only under it. *Things are not alive* is needed by no rule, so it is not stated.
> Each rule is a modelling decision about what a word means, and its assumptions are what make that reading
> valid: the set grows with the vocabulary the theory covers, a few per rule.

The milestone README: *Which assumptions a model has to state at all: the user's question 33, tested: for
each rule of the word theory, the assumption that makes its reading valid, written as a premise; then the
count, and whether "a few per rule" holds.*

## What it starts from

[`words.ein`](../../../examples/word_problems/words.ein) has 37 rules, one of them its closing rule;
[`gaps.ein`](../../../examples/word_problems/gaps.ein) 7; and
[`readings.ein`](../../../examples/word_problems/readings.ein) 6 and one hrule
(`grep -c '^(rule ' FILE`, 2026-11-13). One fact states the model, `(model-counts-only)`, and two rules take
it as a premise. Every other rule's reading rests on something nobody wrote down: that is what the census
finds.

## What it produces

A scratch copy of the three modules under `build/s1h.3.3/` in which every rule takes the assumptions its
reading depends on as premises, each assumption a fact with its own relation, `:why` and `:source`; a script
that counts; and the third section of `gaps_assumptions_readings.md`: the census as a table, the counts, and
the user's questions answered on it.

## Tasks

1. **T1h.3.3.1 — the census.** Rule by rule: what the rule concludes from what, and what has to be true of
   the world for that reading of the word to be valid. Written as a table first, on the page: the rule, its
   `:why`, the assumption or assumptions in a sentence each, and whether the text of any banked problem says
   it. Some rows will say *none*: a rule that only restates a sentence (`has-states`) or only plans. The page
   says so and does not invent one. Examples of the kind expected, to be confirmed or struck rule by rule and
   not assumed: *gave means less* rests on the giver no longer having what was given; `bystander-keeps` on
   nothing else having happened between the two moments; *together means a sum* on the two having no apple
   in common; *holds means divide* on any thing fitting any box.
2. **T1h.3.3.2 — the assumptions as premises.** In the scratch copy, each distinct assumption of T1 becomes
   one fact, and each rule takes its own as premises. Every banked problem answers as banked over the
   scratch theory (`ein test` on scratch copies), and the firings and trace steps of each are counted beside
   the banked theory's, since every rule now has a premise or two more.
3. **T1h.3.3.3 — the count.** By the script, from the scratch files: the distinct assumptions; for each, how
   many rules take it; for each rule, how many it takes; the largest and the mean. **Whether *a few per
   rule* holds** is read off that table and said plainly. And how the set grew: the assumptions that the
   first word's rules needed, and what each further word added, in the order the module writes them.
4. **T1h.3.3.4 — the user's questions, on the census.**
   - *"Is this a finite set of assumptions? What is it limited by? Used names/words?"* The set the census
     found, and what bounds it: a row of the table exists because a rule exists, and the page shows a word
     of a problem's text that brings no assumption and an assumption that no word of any text names.
   - *things are not alive and don't run out of the boxes*: no row of the census, and the page says which
     rule would have to exist for there to be one.
   - *we assume things fit boxes, while problem doesn't have this word*: the row, and the rule that needs
     it.
   - the last question is [S1h.2.2](../p1h.2_slicing/s1h.2.2_assumptions_as_premises.md)'s, cited.
5. **T1h.3.3.5 — what an answer cites.** For each banked problem, by the slice script: how many of the
   census's assumptions the answer rests on, and which. One table, problems down the side.
6. **T1h.3.3.6 — the page,** with one more thing said: the census is a modelling judgement, written by the
   stage and checked by no run. The whole list is on the page so that the user can strike or add a row, and
   round 2's outline is marked *held* or *corrected* against the table.

## Acceptance

- Every rule of the three modules has a row, with an assumption or the word *none*.
- Every banked problem answers as banked over the scratch theory.
- The counts of T3 are on the page, each by the script, with its command and date.
- Each of the user's four questions has its answer beside the row or rows that give it.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not edit the banked modules.** One fact or one an assumption in the banked theory is
  [Q-M1h.14](../open_questions.md#q-m1h14--which-of-the-banked-forms-change).
- **Not enumerate the world.** An assumption is on the list because a rule needs it, and for no other
  reason.
- **Not add a rule or a word to make the census richer** (reply 35: *"Take only needed right now."*).

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every program states its lattice. Scratch
stays under `build/s1h.3.3/`. The link check is run before the commit, which changes pages only.
