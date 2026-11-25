# Q-M1f.612 — An `open` fact's provenance names what wrote it

An `open` fact the pass writes at a state's fixpoint is recorded as written by a rule, with no premise: `(open R)` by
the obligation that owes the instance, `(open F)` by the rule that concludes it or the obligation whose payment it
is, and by `open` itself, the kernel's relation, where the blind enumerator or a later debt wrote it. An `open` fact
the program writes keeps the loader's record, the place it is written at. The record is the engine's and the fact
carries none of it ([Q-M1f.390](Q-M1f.390-open-r-carries-no-justification-and-the-instance.md)): two writers of one
fact write one fact, recorded once.

What reads it is what says where a hypothesis came from: a refusal of the hypothesis mechanisms names the `open`
fact it disagrees with as *which rule 'guess' writes*, *which obligation 'total-owed' writes*, or *written at
FILE:L:C* ([Q-M1f.578](Q-M1f.578-the-two-lists-and-a-closure-configure-the-blind.md)).

**Why.** Every other fact a state holds is recorded with what put it there, and an `open` fact is a fact a state
holds ([Q-M1f.566](Q-M1f.566-what-a-state-owes-is-open-r-a-fact-and-open-f.md)). Recorded as its writer's, with no
premise, it says who supposed it without pretending to a derivation the pass did not make.

**Held by.** `ein_infer::open`'s `write`, which records each fact it writes as its writer's;
`ein_infer::mechanisms`, which reads the record into a refusal's line; `ein-cli/tests/mechanism_conflicts.rs`;
[`07-diagnostics.md`](../../ein/user/04-reference/07-diagnostics.md#226-a-programs-hypothesis-mechanisms-do-not-contradict-one-another)
§ 2.26.
