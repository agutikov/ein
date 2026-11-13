# S1h.1.5 — The design page and the stop

**Phase:** [P1h.1](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. **It is the phase's stop:** nothing outside `plans/` and `build/`
is written before the user has read its page and ruled.
**Depends on:** [S1h.1.1](s1h.1.1_what_the_moments_cover_and_the_literature.md) to
[S1h.1.4](s1h.1.4_the_causal_theory_on_a_prototype.md), whose sections it assembles.

## What it rests on

The user's rule for a new subject (2026-10-16): real programs first, their answers computed, before anything
is implemented. The milestone README: *Each phase starts with real programs whose answers the binary
computed, shown to the user before a module ships.* And the reply this phase comes from ends its claim with a
question, *"Both are separate theories, can be reused and deserve separate files in stdlib, right?"*, which
this page is where the user answers on evidence.

## What it produces

`design.md` in this directory, whole: the four stages' sections brought to one page, the proposals, and the
questions. The model is the calculator's
[`design.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/design.md): target programs, each run
on the release binary and shown with what the run wrote, then the conventions proposed, then the questions,
each with a recommendation that a run supports or with none. The commit at the stop is pages only.

## Tasks

1. **T1h.1.5.1 — the page assembled.** One reading order: what the banked moments cover; the opening
   question, two ways; the temporal prototype; the causal prototype and the joining rule; the problem set
   with each problem's answer under the prototypes. Every program on the page is re-run on the binary of the
   day and every number carries its command and date; a figure taken by an earlier stage on another binary
   is re-taken or marked with its own date.
2. **T1h.1.5.2 — the proposals.** For each of the two theories: the relations of a first module, each with
   its `:why` as it would read, the problem that needs it and what it costs; what is left out and why; the
   tests such a module would carry. For the pair: the joining rule and where it would live.
3. **T1h.1.5.3 — the questions,** each with the programs that bear on it:
   - [Q-M1h.5](../open_questions.md#q-m1h5--where-the-temporal-and-causal-modules-live-and-their-names),
     where the modules live and what they are called, with the names each would occupy;
   - [Q-M1h.7](../open_questions.md#q-m1h7--an-event-as-an-instruction-or-a-relation), an event as an
     instruction or a relation, and whether either lands beside the banked word theory;
   - [Q-M1h.8](../open_questions.md#q-m1h8--the-temporal-theorys-first-cut-and-what-soft-and-unknown-are),
     the first cut, and what *soft* and *unknown* are;
   - [Q-M1h.9](../open_questions.md#q-m1h9--what-a-causal-tree-is), what a causal tree is;
   - [Q-M1h.10](../open_questions.md#q-m1h10--the-programs-that-are-not-word-problems), the programs that
     are not word problems.
   A question the four stages raised that is not among these is filed under the next free id first.
4. **T1h.1.5.4 — the findings shown.** Every row of this phase's `findings.md` so far, each with its witness.
5. **T1h.1.5.5 — the stop.** The user reads the page. The rulings are recorded on it in the user's words,
   under a heading of their own, and in [`open_questions.md`](../open_questions.md) against each id.
6. **T1h.1.5.6 — the re-plan.** [S1h.1.6](s1h.1.6_the_temporal_module.md),
   [S1h.1.7](s1h.1.7_the_causal_module_and_the_rule_that_joins_them.md) and
   [S1h.1.8](s1h.1.8_programs_that_are_not_word_problems.md) are restated to the rulings and given their
   tasks; the phase's README is restated where a ruling changes it. If the user rules that one of the two is
   not a module, its stage is restated to what was ruled and keeps its number.

## Acceptance

- The page exists, every program on it was run by the binary, and every number carries its command and date.
- Each of the five questions is on the page beside its programs.
- The user has ruled, or has said which questions wait; the page and `open_questions.md` record it in the
  user's words.
- The three later stage files have tasks, written to the rulings.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not write a module, a test program or an example into the tree.** That is what the stop is before.
- **Not take a default** on a question the user has not answered: a later stage that needs it waits.
- **Not summarise a run it did not re-take** as if it were of the day.

## The standing rules

Questions go to [`open_questions.md`](../open_questions.md) under the next free id; findings to this phase's
`findings.md`. A ruling is recorded as the user gave it and may be reopened by a later finding. The link check
is run before the commit, which changes pages only.
