# S1h.4.2 — The formula as an object: the word theory deriving it as a fact, *before* and *instead of*

**Phase:** [P1h.4](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A prototype: nothing outside `plans/` and `build/` moves. Its
question, [Q-M1h.17](../open_questions.md#q-m1h17--the-formula-as-an-object), is put to the user at
[S1h.4.5](s1h.4.5_the_page_and_the_stop.md).
**Depends on:** [S1h.4.1](s1h.4.1_the_two_parts_and_a_well_formed_formula.md);
[M1g's S1g.10.2](../../m1g_logic/p1g.10_school_algebra/s1g.10.2_expressions_and_evaluation.md), expressions
as data, **if it has run** ([§ If P1g.10 has not run](#if-p1g10-has-not-run)).

## What it rests on

The user, 2026-11-12 (reply 31): *"result depends on formula, formula depends on selected arith operations,
selection depend on reading. The synthesized program must be able answer question about the solution, like
"why you divided those numbers? why not 3/12, not 3+12, not 12-3, and so on"*. And reply 36: *"inference of
a formula and calculations"*.

The milestone README: *The formula as an object. Today the formula of a word problem exists nowhere: it is
the order in which rules ask the calculator. P1g.10's expressions (`(calc E)`, expression data) are where a
formula can be a value, so the phase reads what P1g.10 built before proposing its own.* And the phase's second
question: *the word theory deriving the formula as a fact before, or instead of, asking the calculator step
by step; then "why divided and not subtracted" is a question about which rule built which node of the
formula, answerable from its justification.*

## What it produces

Two scratch variants of the word theory under `build/s1h.4.2/`, each beside an unedited copy of the banked
one, the banked problems run over both, and the second section of `formula_and_calculation.md`.

- **Variant *before*:** each word's rule concludes the formula of its quantity, a fact such as
  *the formula of how many boxes they need is (5 + 7) over 3*, built from the formulas of the quantities it
  needs; the value is then computed as today, by the step-by-step requests. The formula is an extra fact
  beside the answer.
- **Variant *instead of*:** the word rules conclude formulas only; one evaluator turns a formula into its
  value through the calculator. The word theory asks the calculator nothing itself.

## Tasks

1. **T1h.4.2.1 — the formula's data.** Read from S1g.10.2 if it has run: its heads, its `(evaluate E)` and
   `(value-of E N)`. The stage writes down which of them a word problem's formula uses (a number, a sum, a
   difference, a product, a quotient with its remainder, a comparison) and which the word problems need
   that the algebra's expressions may lack: a quotient *and* its remainder are two results of one node
   (`each-gets-the-quotient` concludes both), and a comparison has no value. Each is shown on a problem and
   is a note to P1g.10, not a second design.
2. **T1h.4.2.2 — variant *before*.** The rules written; for `bob1`, `boxes12`, `pencils` and `more`, the
   formula fact of the question, as the run derives it, beside the formula a pupil would write. Every banked
   problem answers as banked (`ein test` on scratch copies). What a formula adds to the plan's `(needs Q Q')`
   facts, which already hold the shape without the operations: the two side by side on one problem.
3. **T1h.4.2.3 — variant *instead of*.** The same four problems. The word theory's rules are counted
   against the banked theory's (`words.ein` has 37; a word is there a rule that plans, one that asks the
   calculator and one that reads its answer): which of those fold into one rule that builds a node, and
   what the evaluator costs in rules and firings. Whether
   the plan, backwards from the question, survives: a formula built bottom-up needs the sub-formulas first,
   so the task shows what makes a formula wanted.
4. **T1h.4.2.4 — *why divided and not subtracted*.** On `boxes12`, under each variant: the node of the
   formula that is a division, the rule that concluded it, and that rule's `:why` and premises, read off
   `ein saturate --dump`'s `:rule` and the trace's step. Written as the answer to the user's question as far
   as it goes: *divided, because the question asks how many boxes and a box holds 3* is one rule's `:why`
   with its bindings. What it does not answer, *why not subtracted*, is
   [S1h.4.4](s1h.4.4_the_alternatives_not_taken.md)'s.
5. **T1h.4.2.5 — a formula a reading.** The 12-apple and 13-apple problems under the rebuilt readings
   ([S1h.3.4](../p1h.3_gaps_assumptions_and_readings/s1h.3.4_readings.md)): the formula fact in each of the
   two states. With 12 apples the two states have one answer; the page shows whether they have one formula
   or two, which is what a query on the formula would then tell apart where a query on the answer cannot.
6. **T1h.4.2.6 — *well-formed*, once the formula is data.** S1h.4.1's table re-read: which checks become
   rules over the formula's shape, run before anything is calculated, and which stay the calculator's at the
   call. One of them written as a rule on the prototype, a division whose divisor's formula is a stated
   zero, and its outcome beside today's.
7. **T1h.4.2.7 — what each variant costs.** Firings and trace steps a problem, beside the banked theory's;
   the joins a rule over a nested formula makes (E-3 of the calculator's findings; noted, not measured in a
   batch and not fixed).
8. **T1h.4.2.8 — the page,** with Q-M1h.17's three parts each beside the programs: which variant; whether
   the data is P1g.10's and nothing else; whether a module is banked beside `words.ein`.

## If P1g.10 has not run

[Q-M1h.2](../open_questions.md#q-m1h2--whether-it-follows-p1g10) is open: nothing says this phase follows
P1g.10. If S1g.10.2 has not run when this stage starts, the prototype declares the few heads it needs in a
scratch file of its own, in that stage's planned spelling (`num`, `Plus`, `Minus`, `Times`, `Over`), with an
evaluator of the few rules its problems use. The page then says, at its head, that the formula's data is
provisional and S1g.10.2's to settle, and T1's notes are written as questions to that stage. Nothing of the
scratch term language is proposed for shipping.

## Acceptance

- The formula of each of the four problems is a fact the run derives, on the page under both variants.
- Every banked problem answers as banked under both variants.
- *Why divided* is answered on the page from one rule's justification.
- The two states of the 12-apple problem are shown with their formulas.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not edit `examples/word_problems/`,** and not the calculator.
- **Not ship a term language.** It is P1g.10's.
- **Not add an expression layer to the calculator's core** (reply 10 of 2026-11-12 put it in P1g.10).
- **Not explain a formula by anything but its derivation.** A reason no rule states is the reasoning the
  user ruled out of scope.

## The standing rules

A wrong answer of the engine met by a prototype is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every program states its lattice; every
prototype run carries `-T`. Scratch stays under `build/s1h.4.2/`. The link check is run before the commit,
which changes pages only.
