# S1h.1.1 — What the banked moments already cover, the literature named, and the problems the phase is held to

**Phase:** [P1h.1](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. It writes pages and scratch programs only: nothing outside
`plans/` and `build/` moves. Its page is part of what [S1h.1.5](s1h.1.5_the_design_page_and_the_stop.md) shows
the user.
**Depends on:** the word problems as banked under
[`examples/word_problems/`](../../../examples/word_problems/README.md) (`f3fe6ff0`, and
[S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md)'s
changes if that stage has run).

## What it rests on

The user's reply of 2026-11-12 to question 26, quoted in full in the phase's
[README](README.md#where-it-comes-from): *"order of events on a theoretical/virtual timeline - they can be
points, they can be ranges, they can be hard, soft, unknown etc. And there is a causal tree."* And two
sentences of the milestone README's section: *What the banked problems have today is the simplest case, named
moments, `t0` and `t1`, with every event carrying two names; the phase starts by saying what of the reply's
list that already covers*; and *The literature each of those belongs to is surveyed in the phase's first stage
and named*.

## What it produces

The first three sections of `design.md` in this directory, and scratch programs under `build/s1h.1.1/`:

1. **A table: the reply's list against the banked problems.** One row for each of *points*, *ranges*,
   *hard*, *soft*, *unknown*, *a causal tree*; in each, what the banked theory has of it, with the program and
   the lines of its output that show it, or *nothing*.
2. **What the stdlib already has for an order,** measured.
3. **The literature, named.**
4. **The problem set:** the problems every later stage of the phase is run on.

## Tasks

1. **T1h.1.1.1 — the banked moments, read.** On `bob1.ein`, `gap.ein`, `bystander.ein`, `unused.ein` and
   `pencils.ein`, from `ein saturate FILE --dump` and `ein solve FILE -G --trace T.md`: what a moment is (a
   name; a point); what orders two moments (the two names on one `gave` fact, and nothing else: no fact
   relates `t0` and `t1`); what carries a state across an event (`bystander-plan`, `bystander-keeps`, a rule a
   kind of event); in which direction the plan and the computation run. Then the same reading of the two
   forms the calculator's investigation ran and did not bank, nested events and a chain of sentences
   ([`word_problems.md` § 1](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#1-how-a-problem-is-written)):
   each is already an order of another kind, and the table says which.
2. **T1h.1.1.2 — the stdlib's order, measured.** A scratch program with a chain of n moments and one stated
   relation between neighbours, under [`std.algebra`](../../../stdlib/algebra.ein)'s `transitive`, with
   `irreflexive` and `asymmetric`, at n = 5, 10, 20 and 40: the facts derived and the firings
   (`ein saturate`), and the wall clock through `utils/bench_env.sh`. The same chain as
   [`std.slots`](../../../stdlib/slots.ein)'s adjacency, where it can be stated. This is the baseline
   [S1h.1.3](s1h.1.3_the_temporal_theory_on_a_prototype.md)'s cost table starts from; nothing is designed
   here.
3. **T1h.1.1.3 — the literature, named.** For each of the three the README names as what the phase is *not*
   (Allen's interval algebra as a decision procedure; a planner; the event calculus), and for whatever the
   stage finds *a causal tree* belongs to: what it is in two or three sentences, what of it a school problem
   or a story uses, and what of it the phase leaves alone, with a link to a primary source. The catalogue is
   read first ([`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md) has Allen's algebra;
   [`docs/lib/README.md`](../../../docs/lib/README.md) lists the files). The survey is a section of the page;
   it adds no entry to `docs/lib/` unless the stop asks for one.
4. **T1h.1.1.4 — the problem set, proposed.** Each problem written as its sentences, with the item of the
   reply's list it needs and the answer worked by hand. At the least:
   - the banked problems, which the theories must leave answering as they do;
   - `before.ein`'s problem, the event read backwards;
   - a problem that states an amount on both sides of one event, once consistently and once not;
   - two events whose order the text does not give, where the answer depends on the order and where it does
     not;
   - a problem with a range: something that holds *while* or *until* something else;
   - a question that asks *why*: *Why does Bob have 3 apples?*;
   - **programs that are not word problems,** one for each theory
     ([Q-M1h.10](../open_questions.md#q-m1h10--the-programs-that-are-not-word-problems)): the candidates and
     the reason each is a fair test, from what the tree holds (an ordering puzzle of the Zebra family; a
     circuit of [`examples/binary_logic/`](../../../examples/binary_logic/README.md)).
   A problem is in the set because a later stage runs it. The set is the phase's boundary: a relation no
   problem of the set uses is not built.
5. **T1h.1.1.5 — the page.** The four sections, every number with its command and date.

## Acceptance

- The table of T1 has a row for each item of the reply's list, each with a program and its output or the
  word *nothing*.
- T2's figures are on the page at four sizes, each with its command.
- Each of the three subjects the README names is on the page with what the phase takes and leaves.
- The problem set is on the page, each problem with its sentences, the item it needs and its answer by hand.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not design a relation of either theory.** It reads and it lists.
- **Not edit a file under `examples/word_problems/`.**
- **Not write a planner's problem into the set as something the phase will solve.** The state-transition
  puzzles of [idea 09](../../ideas/09-puzzles-beyond-zebra.md) are named where the phase stops.

## The standing rules

A wrong answer of the engine met by a scratch program is a row in `corpus/wrong_answers.toml`, filed `open`
with its witness before anything fixes it. Findings go to `findings.md` in this directory, opened at the
first; questions to [`open_questions.md`](../open_questions.md) under the next free id. Scratch stays under
`build/s1h.1.1/`. No gate run is needed for a commit that changes pages only, but the link check,
`python3 utils/doc_audit.py --links --check` over the gate's paths, is run before it.
