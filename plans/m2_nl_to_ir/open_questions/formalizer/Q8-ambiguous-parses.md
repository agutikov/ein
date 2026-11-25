# Q8 — Ambiguous parses

**Status:** open

Per [idea 04 §Multiple-variant complication](../../../ideas/04-nlp-to-graph-to-solver-pipeline.md#multiple-variant-complication)
and [idea 04 §Open questions point 3](../../../ideas/04-nlp-to-graph-to-solver-pipeline.md#open-questions).

**Working answer**: branched on the IR. The NL frontend emits all plausible parses, and the engine treats each as
it treats any other hypothesis.

**The user, 2026-11-25:** *"there is no `hrule` anymore, rules emitting open facts replaced them"*.

So the form is settled, and it is nothing new: an alternative is a hypothesis, `(open F)`, which a rule concludes
([Q-M1f.567](../../../../docs/decisions/hypotheses/Q-M1f.567-a-hypothesis-rule-is-a-rule-that-concludes-open.md)) or the
program writes at root. The `(hypothesis-parse ?id …)` wrapper this entry proposed, its hypothesis levels and the
`hrule` it named are gone. The corpus already encodes a sentence with two readings this way, each reading a
candidate and root owing one of them ([`readings.ein`](../../../../examples/word_problems/readings.ein),
[`boxes12.ein`](../../../../examples/word_problems/boxes12.ein), [`boxes13.ein`](../../../../examples/word_problems/boxes13.ein);
[Q-M1f.332](../../../../docs/decisions/corpus/Q-M1f.332-a-word-with-two-readings-always-has-one-chosen.md)).

**What is still open is the experiment the reshape added.** A reading the formalizer could not resolve may be
emitted as alternatives and pruned by the kernel (the working answer), **or** committed one way and caught by the
loop when the kernel answers several states or none and the feedback names the sentence
([S2.4.3](../../p2.4_loop/s2.4.3_feedback_ladder.md)). These are two strategies for the same sentence, and which one the
loop should prefer is measured, not decided: [S2.4.5](../../p2.4_loop/s2.4.5_alternatives_as_hypotheses.md) is the stage
and [P2.6](../../p2.6_ablations/README.md) is where the two are compared. S2.4.5 no longer has a form to choose.
