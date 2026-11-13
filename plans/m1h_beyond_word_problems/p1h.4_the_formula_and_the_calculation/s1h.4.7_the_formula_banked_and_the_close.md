# S1h.4.7 — The formula in the banked word problems, the page on what Ein can and cannot say about a solution, and the close

**Phase:** [P1h.4](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. It is past the phase's stop: **its tasks are
written at [S1h.4.5](s1h.4.5_the_page_and_the_stop.md)'s re-plan,** to the user's rulings, and nothing below
designs the module.
**Depends on:** the ruling on [Q-M1h.17](../open_questions.md#q-m1h17--the-formula-as-an-object);
[S1h.4.6](s1h.4.6_the_pupils_page_built.md), for the page a banked problem's README can then show; P1g.10's
[S1g.10.2](../../m1g_logic/p1g.10_school_algebra/s1g.10.2_expressions_and_evaluation.md), if the ruling makes
the formula's data that stage's.

## What it rests on

The milestone's acceptance for this phase: *a solved problem's formula is an object a reader can see apart
from its calculation, and the page says what Ein can and cannot say about a solution.* And the user,
2026-11-12, of the word theory: *"leave what is implemented already"* (reply 26) and *"examples for now"*
(reply 34).

## What it ships, by the ruling

- **The formula, banked.** The variant the user chose, *before* or *instead of*, as a module beside
  `words.ein` under `examples/word_problems/`, which is not rewritten; the problems that import it, each
  with a query on its formula and a claim; a corpus entry a file; the goldens expected to move counted and
  named before blessing; `examples/word_problems/README.md` restated. If the formula's data is P1g.10's and
  that stage has not run, this part waits for it and the stage file says so at the re-plan.
- **The witness of *answer correct, solution not*,** with its two queries, one on the answer and one on the
  formula, if [S1h.4.4](s1h.4.4_the_alternatives_not_taken.md)'s third case was accepted as an example to
  bank.
- **Nothing,** if the user rules that the formula stays a prototype. Then the page keeps the prototype's
  programs in full, as the calculator's investigation page keeps its own.

## The page, and the close

- **`formula_and_calculation.md` gets its closing section:** what was ruled and built; and the statement of
  S1h.4.4's fifth task in its final form, *what Ein can and cannot say about a solution*, each *cannot* with
  its program.
- **Handed on by name:** to [M2](../../m2_nl_to_ir/README.md), *"the solution could be correct, but
  reasoning not"*, with the example; to [M12](../../m12_rules_as_data/README.md), *why not this formula* as
  a question about a rule set; to P1g.10, the notes on what a word problem's formula needs of its
  expression data.
- Every row of this phase's `findings.md` has a ruling, a fix or an owner.
- The phase's README and the milestone's phase table are restated to what was built.

## Acceptance

Written with the tasks. At the least: the milestone's line above, on the banked two-step problem or on the
kept prototype; every banked problem that was not named by the ruling answers as it did; where a tracked
file outside `plans/` changed, `./run_tests.sh`, then `./run_tests.sh --nightly`, green on the commit.

## What it must not do

- **Not rewrite `words.ein`** or a banked problem that the ruling did not name.
- **Not move the word theory or the formula module into the stdlib** (reply 34).
- **Not bank a program in which a hypothesis leads to a request to the calculator** without running it alone
  against `region.rs`'s monotonicity test first (E-33; `AGENTS.md` § Edits that go together).
- **Not write a theory of solutions into the closing page.** It says where the line is.
