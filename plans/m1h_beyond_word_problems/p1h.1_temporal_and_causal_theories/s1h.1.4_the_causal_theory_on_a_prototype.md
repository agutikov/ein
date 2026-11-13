# S1h.1.4 — The causal theory on a prototype: a causal tree beside the derivation tree, and the rule that joins the two theories

**Phase:** [P1h.1](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. A prototype: nothing outside `plans/` and `build/` moves. Its
question, [Q-M1h.9](../open_questions.md#q-m1h9--what-a-causal-tree-is), is put to the user at
[S1h.1.5](s1h.1.5_the_design_page_and_the_stop.md).
**Depends on:** [S1h.1.3](s1h.1.3_the_temporal_theory_on_a_prototype.md), whose order the joining rule
reads; [S1h.1.1](s1h.1.1_what_the_moments_cover_and_the_literature.md)'s problem set.
**Read with:** [P1h.2](../p1h.2_slicing/README.md) and
[P1h.4](../p1h.4_the_formula_and_the_calculation/README.md): if a causal tree turns out to be a reading of
the trace, it is their slice and their pupil's page met from this side.

## What it rests on

The user, 2026-11-12: *"And there is a causal tree. Both are separate theories, can be reused and deserve
separate files in stdlib, right?"*

The milestone README: *What happened because of what: "Bob has 3 because he had 5 and gave 2". In Ein a
derivation is already a tree of justifications; the phase's first question is what a causal tree "in the
problem's world" is beside the derivation tree of the answer, and whether it is a relation a program states,
one the theory derives, or a reading of the trace.* And, of the two theories: *a cause is before its effect,
which is one rule joining them.*

## What it produces

Prototypes under `build/s1h.1.4/` and one section of the phase's `design.md`: the derivation tree of an
answer as it is today, three candidate forms of a causal tree on the same problems, the joining rule, and
Q-M1h.9 with what the runs show.

## Tasks

1. **T1h.1.4.1 — the derivation tree, read.** On `bob1.ein`, from `ein solve FILE -G --trace T.md` and the
   `fire` lines of `--events`: the step that concludes *Bob has 3 at t1* (`giver-has-the-difference`) and its
   four premises, each put in one of three kinds: something that happened or held in the problem's world
   (*Bob had 5*, *Bob gave 2*), a step of the plan (`(wanted …)`, which is the question's and no cause), and
   the calculator's (`(diff-of 5 2 3 z)`, and under it the table's rows). The page fixes two words on this
   program before any later page uses them: a **premise** is what a rule matched; a **cause** is a premise of
   the first kind.
2. **T1h.1.4.2 — three candidate forms, on the same problems.** The why-question of S1h.1.1's set, the
   backwards problem, and one problem where the text itself states a cause that no rule of the theory would
   give:
   - **(a) a relation a program states:** the text's own *because*, written as a fact, with its sentence as
     `:source`;
   - **(b) a relation the theory derives:** a rule for each kind of event that concludes what an amount
     after it is because of, so that *why* is an ordinary query with rows;
   - **(c) a reading of the trace:** nothing in the program; the tree is the answer's justification with the
     plan's steps and the calculator's left out, taken by a script over `--events`
     ([S1h.2.1](../p1h.2_slicing/s1h.2.1_what_a_slice_is_today.md)'s, if that stage has run; a few lines
     written here if not).
   For each: what the why-question's answer looks like; what it cannot say (a stated cause has no derivation;
   a derived cause is only as good as its rule; a trace knows nothing the rules did not derive); what it
   costs in rules and facts.
3. **T1h.1.4.3 — a tree, or not.** A quantity with two causes (an amount after two events), and a chain (the
   cause of a cause): whether the object is a tree, what a query for *every cause, however far back* costs,
   which is the closure of S1h.1.3 again, and whether it is asked on demand.
4. **T1h.1.4.4 — the rule that joins the two theories.** *A cause is before its effect*: written once, over
   the prototype of S1h.1.3 and form (a) or (b). What it derives where a cause is stated and the order is
   not; what it refutes where a text states a cause after its effect, as a flag and then `(false)`, with the
   core on the page. And where the rule would live if the two theories are two modules: in the causal one,
   which then imports the temporal one, or in a third
   ([Q-M1h.5](../open_questions.md#q-m1h5--where-the-temporal-and-causal-modules-live-and-their-names)).
5. **T1h.1.4.5 — outside word problems.** The causal candidate of
   [Q-M1h.10](../open_questions.md#q-m1h10--the-programs-that-are-not-word-problems) tried on the prototype
   as far as it goes, so that the stop can judge it: a small circuit, where an output has its value because
   of its inputs, under each of the three forms.
6. **T1h.1.4.6 — the page.** The section, and Q-M1h.9 with the three forms side by side. If the runs show
   that (c) answers everything the problem set asks, the page says that the causal theory may be a view and
   no module, and sets that beside the reply's *"separate files in stdlib"*; it does not choose.

## Acceptance

- The derivation of one answer is on the page with every premise classified, and the two words are defined
  on it.
- Each of the three forms has a program, or a script's output, that answers the why-question, on the page
  with what it cannot say.
- The joining rule derives and refutes, each on a program with its read-out.
- Every program is monotone by `ein solve --monotone`, or the page shows the site.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not call a premise a cause** on any page past T1 without the classification that makes it one.
- **Not build a theory of causation** past what the problem set asks: no counterfactuals, no interventions,
  no probabilities. Where the literature of S1h.1.1 has them, the page names them as left alone.
- **Not change `--trace` or any surface.** Form (c) is a script here; a view of the trace is P1h.4's
  question ([Q-M1h.16](../open_questions.md#q-m1h16--the-pupils-page)).
- **Not name the module or place it.**

## The standing rules

A wrong answer of the engine met by a prototype is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Every prototype run carries `-T`; every
program states its lattice. Scratch stays under `build/s1h.1.4/`. The link check is run before the commit,
which changes pages only.
