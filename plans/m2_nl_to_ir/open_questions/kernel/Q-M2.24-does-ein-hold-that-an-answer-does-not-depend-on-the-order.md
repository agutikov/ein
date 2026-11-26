# Q-M2.24 — Does Ein hold that an answer does not depend on the order of a program's forms?

**Status:** ruled by the user, 2026-11-27

**Ruled by the user, 2026-11-27:** *"I think it already guarantees that, because 1 - syntax and sematics: all forms
are their own identities, every unique name mean the same objects, rules with the same name are load errors,
etc... 2 - the loading step that connects names and makes the runtime representation of an entire program (compile
rules, indexes, registries, ...) works on the entire program, no runtime loadable modules possible, nothing depends
on lines order; So for now let's state it and record as a known fact, a guarantee, so if will ever see different
behaviour it would be considered a bug"*.

**Read as** (the coordinator's reading, which the user may correct):

- **Ein guarantees it**: a program's answer does not depend on the order its forms are written in. It follows from
  the language (a form is its own object, a name means one object wherever it is written, a rule's name is one
  rule's) and from the load (the step that connects names and builds the runtime representation of a program, its
  compiled rules, indexes and registries, reads the whole program, and no module is loaded at run time).
- **A run that shows otherwise is a bug**: a wrong answer, filed and fixed by the register's rules
  ([`plans/wrong_answers/README.md`](../../../wrong_answers/README.md)).
- **It is stated, not tested, for now**: a decision under `docs/decisions/` under this id, carrying the user's two
  reasons, and a row of [`01-determinism-and-order.md` § 1.2](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#12-what-it-may-not-depend-on),
  the list of what a run may not depend on. No sweep of the corpus in another order is written; what has measured
  it is S2.2.0a's probes. *For now* leaves the test to a later word of the user's.
- **What may move** under another order is what § 1.3 of the same page already allows (the trace and the event
  stream, the order rules fire in), as the probes found; the answer is R, every question's outcome, the queries'
  classes and rows, and H.

**What it changes in the plan.** [S2.2e.4](../../p2.2e_kernel_fixes/s2.2e.4_order_and_table_rows.md)'s first half
writes the decision and the row, and no test; [E-112](../../evaluation/language/E-112-a-program-s-forms-may-be-written-in-the-order-they-are-made.md)
becomes a strength Ein states. The steps write a program in synthesis order and need not reorder it.

**Raised 2026-11-26 by [S2.2.0a](../../p2.2_formalizer/s2.2.0a_zebra_formulations.md)**, T2.2.0a.3, which wrote each of the six
Zebra encodings in synthesis order (the statements, then the ontology, then the theory) and in the theory-first
order, and six small programs with one form before another
([`zebra_formulations/` § The program in the order it is made](../../p2.2_formalizer/zebra_formulations/README.md#6-the-program-in-the-order-it-is-made);
[E-112](../../evaluation/language/E-112-a-program-s-forms-may-be-written-in-the-order-they-are-made.md)).

**The question.** Ein reads a program's forms in any order, and on these programs answers alike in every order. No
page says so ([`01-determinism-and-order.md` § 1.2](../../../../docs/ein/ein.rs/03-design/01-determinism-and-order.md#12-what-it-may-not-depend-on)
lists what a run may not depend on, and the written order of a program's forms is not among them), and no test of
Ein writes a program's forms in another order. The steps make a program in synthesis order and assemble it so
([the phase's design questions](../../p2.2_formalizer/README.md#five-design-questions-each-with-its-default)). Should Ein state it, and
hold it with a test?

The options it weighed (the test and the decision; a decision only; nothing, the steps reordering what they
write) are in `git show f281d35a:plans/m2_nl_to_ir/open_questions/kernel/Q-M2.24-does-ein-hold-that-an-answer-does-not-depend-on-the-order.md`.
