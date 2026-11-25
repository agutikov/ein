# Q10 — Direct LLM → constraint?

**Status:** open

Per [idea 04 §Open questions point 4](../../../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: never the *default*, and with the SMT milestone
dropped (2026-08-18) there is no in-repo solver path to compare against
either. If the question is revisited it is as a *diagnostic*: prompt the
LLM to emit constraints for an external solver and check that its answer
agrees with the IR pipeline's; any difference is a bug in the pipeline.

**Re-homed to Stage L (2026-08-23).** The research plan asks this question
in the form it deserves — *which representation is easiest for an LLM to
synthesise, which produces the most useful repair diagnostics, which detects
semantic incompleteness, which yields the highest verified-answer precision,
which needs the least generated text* — with Ein, an SMT encoding, Datalog,
general-purpose code and a proof assistant as the candidates. That is
[P2.8](../../p2.8_representations/README.md), and it is an experiment about
*targets*, not a back-end: the no-solver-back-end decision
([Q2](../../../open_questions.md#q2--when-does-the-graph-engine-hand-off), M3
dropped) stands, and [M10](../../../m10_external_benchmarks/README.md)'s
hand-written encodings are the seed. The old working answer's *diagnostic*
use — agreement between two targets as a check on either — is one of P2.8's
readings.
