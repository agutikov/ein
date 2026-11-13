# S1h.1.3 — The temporal theory on a prototype: points and ranges, what each relation costs, and what *soft* and *unknown* are

**Phase:** [P1h.1](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A prototype: nothing outside `plans/` and `build/` moves. Its
questions, [Q-M1h.8](../open_questions.md#q-m1h8--the-temporal-theorys-first-cut-and-what-soft-and-unknown-are)
and the evidence for
[Q-M1h.5](../open_questions.md#q-m1h5--where-the-temporal-and-causal-modules-live-and-their-names), are put to
the user at [S1h.1.5](s1h.1.5_the_design_page_and_the_stop.md).
**Depends on:** [S1h.1.1](s1h.1.1_what_the_moments_cover_and_the_literature.md) (the problem set, the
stdlib's baseline) and [S1h.1.2](s1h.1.2_an_event_as_an_instruction_or_a_relation.md) (what an event
relates).
**Shares a question with:** [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md)
([§ The shared question](#the-shared-question)).

## What it rests on

The user, 2026-11-12: *"order of events on a theoretical/virtual timeline - they can be points, they can be
ranges, they can be hard, soft, unknown etc."* And *"Both are separate theories, can be reused and deserve
separate files in stdlib"*.

The milestone README's list of what the phase has to find out on programs before either module is designed;
this stage has three of its four items:

- *which of the temporal relations between points and ranges (before, during, overlapping) the phase's
  problems need, and what each costs as rules: an order is a transitive relation, and a transitive closure
  over a text's events is the first thing that will not stay small;*
- *what "soft" and "unknown" are in a language whose facts hold or do not: a soft order as a hypothesis with
  nothing owed, an unknown order as a debt, or as states of R (the two orders of two events are two states).
  This is P1h.3's question about gaps and readings met again in time, and the two phases share the answer;*
- *whether "hard, soft, unknown" is a property of a temporal fact or the general distinction between a fact,
  a hypothesis and an obligation that Ein already has.*

## What it produces

A prototype theory under `build/s1h.1.3/`, the problems of S1h.1.1's set that need an order written over it,
and one section of the phase's `design.md`. The module has no name yet: the stage files write *the temporal
theory*, and the prototype's file name is scratch.

## Tasks

1. **T1h.1.3.1 — points.** Moments as names with a stated order between them, where the banked theory has
   only the two names on an event. The relations the problem set uses and no others. Each written two ways:
   closed forward, over [`std.algebra`](../../../stdlib/algebra.ein)'s `transitive` where that fits, and
   demand-driven, answering *is a before b* only when asked, as the calculator answers a request. The cost
   table at n = 5, 10, 20 and 40 events: facts, firings, wall clock through `utils/bench_env.sh`, beside
   S1h.1.1's baseline.
2. **T1h.1.3.2 — ranges.** A range as two points, and the relations between a point and a range and between
   two ranges that the problem set uses (*during*, *overlapping*, one before the other), each a rule or two
   over the points' order. The page says which of Allen's thirteen relations the set reached and which it
   did not, and builds none of the second kind.
3. **T1h.1.3.3 — *unknown*: an order the text does not give.** The same two-event problem in three forms,
   each run under `ein solve`, `--single`, `ein test` and `ein solve --monotone`, with every surface:
   - nothing stated, and no rule asks: what the read-out says and does not;
   - **a debt:** the program states that any two events are ordered, and the state owes the order of these
     two (an obligation rule; the read-out's `owed` row, built by
     [S1f.6.26](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.26_what_root_owes.md), names
     it);
   - **states of R:** each order a hypothesis, the two orders two states, and the query's classes one or two
     according to whether the answer depends on the order.
4. **T1h.1.3.4 — *soft*: an order that is preferred.** Two forms on one problem: a hypothesis with nothing
   owed, so that R holds the state with it and the state without; and a marked fact with what rests on it
   named, as the banked `assumed` and `rests-on` do for a quantity
   ([`gaps.ein`](../../../examples/word_problems/gaps.ein)). What each prints, and what
   `ein solve --monotone` says of each.
5. **T1h.1.3.5 — the shared question, answered on T3 and T4's programs.** Is *hard, soft, unknown* a
   property a temporal fact carries (a third argument, a marker relation), or is it Ein's own distinction: a
   fact, a hypothesis the search may commit to, an obligation? The page sets the two readings side by side on
   the programs and says what a temporal vocabulary for it would add that the engine's does not already say.
6. **T1h.1.3.6 — the word problems over it.** A scratch variant of the word theory whose moments are the
   prototype's, on the problem set: the banked problems answer as banked; the problems that need an order
   the banked theory cannot state are answered or named as gaps. No banked file is edited.
7. **T1h.1.3.7 — the page.** The section: the relations proposed for a first module with the cost of each;
   the names it would occupy in a program that imports it flat, each checked against the word theory's and
   the stdlib's (a clash is refused at load, and
   [S1f.6.30](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.30_name_clashes_between_imported_theories.md)
   owns what an import could do about it); and Q-M1h.8 with what the runs support.

## The shared question

What *soft* and *unknown* are is the question
[S1h.3.1](../p1h.3_gaps_assumptions_and_readings/s1h.3.1_the_three_forms_of_a_gap.md) asks of a missing
quantity (a debt, a fact by `absent`, a hypothesis over a list) and
[S1h.3.4](../p1h.3_gaps_assumptions_and_readings/s1h.3.4_readings.md) of a word with two readings. It is asked
once. If P1h.3's stages have run, T3 and T4 start from their tables and add the temporal problems; if not,
this stage writes the forms on its own problems and P1h.3 re-reads them. Either way the two pages cite one
another and do not restate.

## Acceptance

- Each relation proposed is used by a problem of the set, and the page names the problem.
- The cost table has both ways of writing an order at four sizes, each figure with its command and date.
- *Unknown* is on the page in three forms and *soft* in two, each a program with its read-out and its
  `--monotone` verdict.
- The banked problems answer as banked over the prototype (`ein test` on the scratch copies).
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not build a relation no problem of the set uses,** and no decision procedure over the relations.
- **Not write a non-monotone theory in silence.** A form the static check flags is shown with its site
  ([`monotonicity.md` § 5](../../../docs/kernel/inference/monotonicity.md#5-detection)) and is not proposed
  for the module without saying so.
- **Not name the module or place it.** Q-M1h.5.
- **Not introduce a number for a time.** If a problem of the set needs a duration, it is the calculator's
  number, and the need is a finding.

## The standing rules

A wrong answer of the engine met by a prototype is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every prototype run carries `-T`; every
program states its lattice. Scratch stays under `build/s1h.1.3/`. The link check is run before the commit,
which changes pages only.
