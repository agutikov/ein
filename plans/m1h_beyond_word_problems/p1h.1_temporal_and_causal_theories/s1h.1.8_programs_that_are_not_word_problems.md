# S1h.1.8 — Each module used by a program that is not a word problem; and what the word problems take

**Phase:** [P1h.1](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. It is past the phase's stop: **its tasks are
written at [S1h.1.5](s1h.1.5_the_design_page_and_the_stop.md)'s re-plan,** to the user's rulings.
**Depends on:** [S1h.1.6](s1h.1.6_the_temporal_module.md) and
[S1h.1.7](s1h.1.7_the_causal_module_and_the_rule_that_joins_them.md); the rulings on
[Q-M1h.10](../open_questions.md#q-m1h10--the-programs-that-are-not-word-problems) (which programs) and
[Q-M1h.7](../open_questions.md#q-m1h7--an-event-as-an-instruction-or-a-relation) (what the word problems
take).

## What it rests on

The user, 2026-11-12: the two theories *"can be reused"*. The milestone's acceptance: *each used by a program
that is not a word problem*. And, of the word problems: *"In the Bob/Alice prototype - pick the simplest that
works - leave what is implemented already."*

## What it ships

- **A program for each module that is not a word problem,** the ones the user chose at the stop from
  [S1h.1.1](s1h.1.1_what_the_moments_cover_and_the_literature.md)'s candidates, under `examples/`, each with
  its `:expect`, its corpus entry and a line in [`examples/README.md`](../../../examples/README.md). Each
  imports the module and nothing of the word theory, which is what shows the module is reusable.
- **What the word problems take, as ruled on Q-M1h.7.** If the user rules that one of
  [S1h.1.2](s1h.1.2_an_event_as_an_instruction_or_a_relation.md)'s two ways lands under
  `examples/word_problems/`, it lands as a module beside `words.ein`, which is not rewritten, and
  `before.ein`'s claim moves to 7 in that commit, as
  [S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md)'s
  T5 has it. If the user rules that nothing lands, `before.ein` keeps its claim and its header is restated
  to say where the comparison is.
- **The phase's close:** every row of `findings.md` with a ruling, a fix or an owner; the phase's README and
  the milestone's phase table restated to what was built.

## Acceptance

Written with the tasks. At the least: each program is a corpus entry whose claim `ein test` holds; the
banked word problems answer as they did but for what Q-M1h.7's ruling moves, and each moved golden is named
before it is blessed; `./run_tests.sh`, then `./run_tests.sh --nightly`, green on the commit.

## What it must not do

- **Not rewrite a banked problem to the new theories.**
- **Not bank a planner's problem as something the modules solve.**
- **Not add a program whose answer was not first computed on the prototype and shown.**
