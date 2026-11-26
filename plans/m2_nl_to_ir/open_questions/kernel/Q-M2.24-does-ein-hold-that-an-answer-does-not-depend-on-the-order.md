# Q-M2.24 — Does Ein hold that an answer does not depend on the order of a program's forms?

**Status:** open

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

**The default taken**: yes, as a proposal: a test beside `ein-cli/tests/import_invariance.rs` that writes every
corpus program's top-level forms in synthesis order and in reverse, and holds each answer (R, every question's
outcome, the queries' classes and rows, H) to the original's, with a row of § 1.2 and a decision under
`docs/decisions/` written with it. It is not written by this stage: which stage writes it, and when, is the user's.
Until then the synthesis order rests on this stage's measurement.

**What the default costs.** A sweep over the corpus in the gate (each program loaded three times, every question
asked); a decision that fixes what the loader may never change (reading forms in passes is today's structure, not a
rule).

**The alternatives.**

1. **The test and the decision** (the default, when the user says).
2. **A decision only**, held by the measurement of this stage's probes.
3. **Nothing**: the steps reorder what they write into the usual order before the kernel reads it.
