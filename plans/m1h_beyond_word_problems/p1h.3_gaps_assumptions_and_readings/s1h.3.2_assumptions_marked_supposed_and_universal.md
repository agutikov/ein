# S1h.3.2 — Assumptions: a marked fact against a hypothesis, the universal ones, and how an answer reports what it rests on

**Phase:** [P1h.3](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A comparison on prototypes: nothing outside `plans/` and
`build/` moves. Its question,
[Q-M1h.15](../open_questions.md#q-m1h15--universal-assumptions-as-a-module), is put to the user at
[S1h.3.6](s1h.3.6_the_page_and_the_stop.md).
**Depends on:** [S1h.3.1](s1h.3.1_the_three_forms_of_a_gap.md)'s problems and table;
`examples/word_problems/listed_is_all.ein`, which
[S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md)
banks (if that stage has not run, this one writes the module in scratch, to the user's reply below, and says
so); [S1h.2.1](../p1h.2_slicing/s1h.2.1_what_a_slice_is_today.md)'s script, for the slice.

## What it rests on

The user, 2026-11-12, on how an assumption is written and reported (reply 29, where option (a) reads *A
marked fact, `assumed`, and a `rests-on` fact beside the answer* and option (b) *A hypothesis the search may
commit to*):

> again - for this stage (a) is OK, return back to investigations of this in M1h

On whether *Alice has 3 pears* is also *Alice has no apples* (reply 32):

> what is not listed is zero - good idea, it could be a reusable universal assumption.

and in round 2, on building it as a module a problem imports (`dc88520b`):

> Yes, relation arguments are marked with some objects that can be treated as types. So can add a rule that
> assigns zero to every unknown and deduce (has Alice apples 0). It doesn't work in zebra for example - if
> nobody lives in the house means we yet don't know who lives there.

The milestone README: *Assumptions: marked facts against hypotheses the search may commit to; which of a
theory's assumptions are "universal" and reusable across theories ("what is not listed is zero" is the closed
world for one relation; the user calls it "a reusable universal assumption" and names where it fails: "It
doesn't work in zebra", where an empty slot is "not known yet"; so "universal" means reusable, adopted
relation by relation, and the phase says what decides whether a relation may adopt it), and whether such
assumptions are a stdlib module; how an answer reports what it rests on (P1h.2's slice).*

## What it produces

Scratch programs under `build/s1h.3.2/` and the second section of `gaps_assumptions_readings.md`: the two
ways to write an assumption on the same problems; the three ways an answer can report one; *what is not
listed is zero* on a relation that may adopt it and on one that may not; a stated condition for adopting it;
and the evidence for Q-M1h.15.

## Tasks

1. **T1h.3.2.1 — a marked fact against a hypothesis.** *Assume that Alice had none*, on the problems of
   S1h.3.1: as a marked fact (`gap_assume.ein`, banked) and as a hypothesis the search may commit to (the
   investigation's `gap_assume_h`, and with each form of the gap, `_ha` and `_hb`), each on the day's
   binary with every surface. What differs: one state or two; whether the problem as stated, with no
   assumption, is still answered; what is a fact of the state and what is a commitment.
2. **T1h.3.2.2 — how an answer reports what it rests on.** The same answer, 5, reported three ways, side by
   side: the `rests-on` fact as a query row (the marked fact's); the state's `from {…}` line and its place
   in the `(region …)` (the hypothesis's); and the slice of the answer by S1h.2.1's script, in which the
   assumption is a used fact with its `:source`. Which of the three needs a rule written for it, which needs
   the author to ask, and which comes with no extra mechanism.
3. **T1h.3.2.3 — *what is not listed is zero*, where it holds.** `listed_is_all.ein` as the user's reply has
   it: over the typed arguments of `has`, a rule that derives a zero amount for every holder and kind with
   none listed, the derived fact marked as resting on the assumption. The apples-and-pears problem with the
   import and without. What `ein solve --monotone` and *forbid* say of a program that imports it, since it
   reads `absent` over `has`.
4. **T1h.3.2.4 — where it fails.** The user's case, run: the same rule over a relation of a Zebra program,
   where an empty slot is *not known yet*. A small program of the Zebra family with the assumption adopted
   for its location relation: what the engine does (a refusal by *forbid*, a dead root, a wrong region), set
   beside the same program without it. The witness is a scratch program; if it shows the engine giving a
   wrong answer where it should refuse, that is a register row, filed before anything else.
5. **T1h.3.2.5 — what decides whether a relation may adopt it.** A condition, stated so that it can be
   tested, from T3 and T4. The candidates to try, none chosen before the runs: no commitment can extend the
   relation (it is no hypothesis relation, which is the condition *forbid* already reads); nothing derives
   it (only the problem's own facts hold it); the problem says so (the assumption is a sentence of the
   text, or the author's). For each: does it admit `has` in the word problems, does it refuse the Zebra
   relation, and does anything in the engine check it today or is it the author's to know.
6. **T1h.3.2.6 — which other assumptions are universal.** Of the assumptions the word theory has (the three
   clauses of `model-counts-only`, and [S1h.3.3](s1h.3.3_which_assumptions_a_model_has_to_state.md)'s census
   if it has run): which would a second theory adopt unchanged, and which is the word theory's own. A second
   theory is needed to say so; the page names the one it tried against (the Zebra programs; the calculator)
   and leaves the rest as *not tested*.
7. **T1h.3.2.7 — the page,** with Q-M1h.15's three parts each beside its evidence: whether such assumptions
   are a module; where it lives; whether the condition of T5 is checked by anything.

## Acceptance

- The assumption is on the page as a marked fact and as a hypothesis, on the same problem, each with its
  read-out.
- The three reports of T2 are side by side for one answer.
- *What is not listed is zero* answers the apples-and-pears problem with the import and leaves a gap without
  it; the Zebra witness is on the page with what the engine did.
- A condition for adopting it is stated and tested on one relation that meets it and one that does not.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not make *what is not listed is zero* the word theory's meaning.** It is an assumption a problem adopts,
  relation by relation.
- **Not move anything into the stdlib,** and not name a module. Q-M1h.15.
- **Not edit a banked problem or `listed_is_all.ein`.**
- **Not extend *forbid* or the static check.** If T5's condition should be checked and is not, that is a
  finding and a part of Q-M1h.15.

## The standing rules

A wrong answer of the engine met by a prototype is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every program states its lattice. Scratch
stays under `build/s1h.3.2/`. The link check is run before the commit, which changes pages only.
