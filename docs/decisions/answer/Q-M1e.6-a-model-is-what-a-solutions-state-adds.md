# Q-M1e.6 — A model is what a solution's state adds to the program

A **solution** is a node of R ([Q-M1e2.7](Q-M1e2.7-the-answer-is-a-region-r-of-the-hypothesis.md)): a commitment
whose saturated state is alive and discharged. Its **model** is the positive part of that state minus the positive
part of the initial KB, the program as loaded before its first saturation. A model carries no `(not …)`, nothing the
file states and nothing the loader writes: it is what the program did not say and the solution established.

**Why.** Two runs can agree on every model and differ in what their states hold besides, negations above all; the
model is the part of a solution that answers the puzzle, and the state is what the engine computed to reach it.

**Held by.** [Solution semantics](../../ein/reasoning/kb-evolution/solution-semantics.md) § 2 and § 3; `ein solve -P`
(`--print-final-positive`) prints each listed state's model; `ein-cli/tests/acceptance_cli.rs`:
`solve_zebra2_prints_the_answer_the_puzzle_asked_for`. What a listing's `(model …)` prints instead:
[Q-M1f.18](../read-out/Q-M1f.18-the-read-outs-model-is-the-claim-form-and-p.md).
