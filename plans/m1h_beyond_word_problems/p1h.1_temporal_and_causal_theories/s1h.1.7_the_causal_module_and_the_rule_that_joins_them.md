# S1h.1.7 — The causal module, and the rule that joins the two

**Phase:** [P1h.1](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. It is past the phase's stop: **its tasks are
written at [S1h.1.5](s1h.1.5_the_design_page_and_the_stop.md)'s re-plan,** to the user's rulings, and nothing
below designs the module.
**Depends on:** [S1h.1.6](s1h.1.6_the_temporal_module.md), which the joining rule reads; the rulings on
[Q-M1h.9](../open_questions.md#q-m1h9--what-a-causal-tree-is) (what a causal tree is) and
[Q-M1h.5](../open_questions.md#q-m1h5--where-the-temporal-and-causal-modules-live-and-their-names) (where the
module and the joining rule live).

## What it rests on

The user, 2026-11-12: *"And there is a causal tree. Both are separate theories, can be reused and deserve
separate files in stdlib, right?"* The milestone README: *how the two theories connect: a cause is before its
effect, which is one rule joining them.*

## What it ships

Whichever the user ruled at the stop, on [S1h.1.4](s1h.1.4_the_causal_theory_on_a_prototype.md)'s three
forms:

- **if a causal tree is a relation a program states or one the theory derives:** the causal module, and the
  joining rule in the place Q-M1h.5's ruling gives it, with everything
  [S1h.1.6 § What it carries](s1h.1.6_the_temporal_module.md#what-it-carries-whatever-is-ruled) lists;
- **if it is a reading of the trace:** no module is written here. The stage is then restated at the re-plan
  to what was ruled, and what remains of it, a view of an answer's justification with the plan's steps and
  an imported theory's left out, is handed by name to
  [P1h.2](../p1h.2_slicing/README.md) and [P1h.4](../p1h.4_the_formula_and_the_calculation/README.md), whose
  surfaces it would be. The milestone's acceptance line, *two stdlib modules*, is then restated in the
  milestone README with the user's words.

## Acceptance

Written with the tasks. At the least, where a module ships: the stdlib's gates as S1h.1.6 lists them; the
joining rule has a program where it derives an order from a stated cause and a pair where a cause stated
after its effect is refuted and where one stated before it is accepted; `./run_tests.sh`, then
`./run_tests.sh --nightly`, green on the commit.

## What it must not do

- **Not ship a theory of causation past what the problem set asks:** no counterfactuals, no interventions,
  no probabilities.
- **Not change `--trace` or any surface.** A view is P1h.4's question
  ([Q-M1h.16](../open_questions.md#q-m1h16--the-pupils-page)).
- **Not edit `examples/word_problems/`.**
