# S1h.1.2 — An event as an instruction or a relation: the two ways to answer 7, compared on programs

**Phase:** [P1h.1](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A comparison on prototypes: nothing outside `plans/` and
`build/` moves. Its question,
[Q-M1h.7](../open_questions.md#q-m1h7--an-event-as-an-instruction-or-a-relation), is put to the user at
[S1h.1.5](s1h.1.5_the_design_page_and_the_stop.md).
**Depends on:** [S1h.1.1](s1h.1.1_what_the_moments_cover_and_the_literature.md)'s problem set;
`examples/word_problems/before.ein`, which
[S1f.6.28](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.28_word_problems_second_pass.md)
banks. If that stage has not run, this one writes the three facts beside scratch copies of `words.ein` and
`gaps.ein`, as the planning did on 2026-11-13 (`build/m1h/before/before.ein`), and says so.
**Read with:** [M1g's P1g.9](../../m1g_logic/p1g.9_one_equation_three_methods/README.md), whose S1g.9.1 lists
this problem as a witness, and [S1g.10.4](../../m1g_logic/p1g.10_school_algebra/s1g.10.4_linear_equations.md).

## The question it rests on

The user, 2026-11-12, on the banked word problems:

> what is just simply change the order: t1 Bob gave Alice 2 apples; t2 Bob has 5 apples; How many apples
> Bob had at t0 (before t1); would the same theory infer plus from gave reverted in time? Or even reverted
> plus directly with reverse arithmetic?

On the banked theory the answer is no. The rules read *gave* forwards only, *after = before − x*, and plan
only from a later moment to an earlier one; the question has no row, and the amount at `t0` is named as not
stated. The milestone README sets out two ways to make it answer 7, *for this phase to compare on programs*:

- **Each word written twice,** once in each direction of time: *if the amount before a giving is wanted and
  the amount after is known, it is after + x*. It works over the calculator as shipped; the planning then
  runs both ways along the timeline.
- **The event stated once, as a relation between the moments on both sides of it,** *before = after + x*,
  and whichever of the three is unknown asked for. The direction is then in no word rule. Finding the unknown
  is either a rule that knows the inverse of a sum, which is the algebra step, or the addition table read
  backwards, the user's *"reverted plus directly with reverse arithmetic"*.

And what follows from either: *once an event is read both ways, a problem that states an amount on both sides
of it has two derivations of one quantity, and where they disagree the problem contradicts itself. Nothing
checks that today.*

## What it produces

Prototypes under `build/s1h.1.2/`, each a scratch module beside an unedited copy of the banked ones, and one
section of the phase's `design.md`:

- **Way A, `words_both.ein`:** `words.ein` with a second set of rules for each word that has a direction
  (*gave* for the giver and for the receiver, and the bystander's rule), read from the later moment to the
  earlier.
- **Way B, in two variants:** the event as one relation between the amount before, the amount after and the
  amount moved, with the unknown one found by **B1**, a rule that knows the inverse of a sum, and **B2**, the
  addition table read backwards, from
  [`examples/beyond_arith/inverse_addition.ein`](../../../examples/beyond_arith/inverse_addition.ein).
- **The table** that sets A, B1 and B2 side by side on the problem set.

## Tasks

1. **T1h.1.2.1 — today's answer, re-taken.** The backwards problem and the two problems of S1h.1.1's set
   that state an amount on both sides of one event, on the banked theory, under `ein solve`, `ein test` and
   `ein saturate --dump`: what is answered, what is named as a gap, and that the inconsistent one is not
   noticed. Each with its command and date.
2. **T1h.1.2.2 — way A.** The rules written; how many are added for each word; the backwards problem
   answers 7; every banked problem answers as it did (`ein test` over scratch copies that import
   `words_both`); the plan now runs both ways, so the task shows on the dump that the `wanted` and `needs`
   facts stay a finite set and what they hold that the forward theory's did not.
3. **T1h.1.2.3 — way B.** The relation written once for *gave*; the three questions it can be asked (the
   amount after, the amount before, the amount moved), each on a problem; B1 and B2 each answering 7. For B1
   the one rule is [S1g.10.4](../../m1g_logic/p1g.10_school_algebra/s1g.10.4_linear_equations.md)'s step and
   the page says so; for B2 the rules are those of the banked inverse addition. If
   [S1g.9.1](../../m1g_logic/p1g.9_one_equation_three_methods/README.md) has run, its programs are read first
   and not rewritten.
4. **T1h.1.2.4 — two derivations of one quantity.** Under A and under B, the problem that states both sides
   consistently (*had 7, gave 2, has 5*) and the one that does not (*had 7, gave 2, has 4*): whether the
   state holds two values for one quantity, and what makes that a contradiction. The banked theory's pattern
   for a conflict is a flag and then `(false)` (`giver-gave-too-many`, `gave-more-than-had-is-false`); the
   task writes the check in that pattern and shows root dead, with its core, on the second problem. **Whose
   rule the check is,** the temporal theory's (*an event relates the amounts on its two sides*) or the word
   theory's (*a quantity has one value*), is written as the two candidates with the program of each, and
   left to the stop.
5. **T1h.1.2.5 — the table.** For A, B1 and B2 on the same problems: rules added a word; root firings and
   trace steps, the word theory's among them; whether each is search-free (no candidate, k_s 1, R {∅}) and
   monotone (`ein solve --monotone`); what the trace of the backwards problem reads as, step by step; what
   each does to `contra.ein` and `gap.ein`.
6. **T1h.1.2.6 — the page.** The section, with Q-M1h.7's four parts each set beside the programs that bear
   on it, and no recommendation the table does not support.

## Acceptance

- The backwards problem answers 7 under A, B1 and B2, each a program on the page with its read-out.
- Every banked problem answers as banked under each way (`ein test` on the scratch copies).
- The inconsistent problem is a dead root under each way, with its core on the page.
- The table of T5 is on the page, every figure with its command and date.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not edit `examples/word_problems/`.** *"leave what is implemented already."* Whether a module is added
  beside `words.ein`, and `before.ein`'s claim moved to 7, is Q-M1h.7.
- **Not add inverse requests to the stdlib.** Round 2 of the calculator's questions: they are *not* added
  now, and [P1g.9](../../m1g_logic/p1g.9_one_equation_three_methods/README.md)'s S1g.9.4 is where a proposal
  would be put.
- **Not decide *arithmetic or algebra*.** That is P1g.9's question; this stage shows the same choice met
  from a word problem.
- **Not design the temporal theory.** What an event relates is settled here only as far as the two ways need;
  the relations between moments are [S1h.1.3](s1h.1.3_the_temporal_theory_on_a_prototype.md)'s.

## What it may find

- **That way A does not stay finite** once two events share a moment, the plan running forwards from one and
  backwards from the other. Then the page shows the program and says what bounds it.
- **That way B needs the expression data of P1g.10** to state *whichever of the three is unknown*. Then the
  prototype declares what it needs in a scratch file and the page names the dependency
  ([Q-M1h.2](../open_questions.md#q-m1h2--whether-it-follows-p1g10)).

## The standing rules

A wrong answer of the engine met by a prototype is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every prototype run carries `-T`. Scratch
stays under `build/s1h.1.2/`. The link check is run before the commit, which changes pages only.
