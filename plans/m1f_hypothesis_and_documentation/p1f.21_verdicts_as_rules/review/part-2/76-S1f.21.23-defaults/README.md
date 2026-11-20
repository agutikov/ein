# 76 · The defaults S1f.21.23 took

[← part 2](../README.md) · [← the phase](../../../README.md) · [your answers](../ANSWERS.md) ·
[the stage's record](../../../s1f.21.23_the_delta_of_a_resumed_state.md#record) · [the cost page](../../../costs/README.md)

S1f.21.23 is done, on your word *"of course build"* (Q-M1f.674): the facts a resumed state added are read off
the state's own layers, above root's, and cost their own count, where the resume walked every fact the state
holds. **No firing moved**: `ein solve -p` and the digest of the events are byte for byte the same on all 942
corpus files. These are its four defaults.

## What you should know first

- **What it saves**: 8.9 % of `send_more_money.ein`'s instructions, 3.2 % of `equation_two_unknowns.ein`'s, 0.3 to
  0.5 % of three others. About half of what the finding said: its 17 % was one cost counted twice.
- **How a state is known to have grown from root** (default 1): each layer of a knowledge base carries a stamp,
  the snapshot keeps root's, and a state whose lower layers are root's is read above them. Any other knowledge
  base gets the walk as before (default 3); no path of the engine takes it.
- **The next cost of the same kind is larger**
  ([V-86](../../../findings.md#v-86--a-states-key-sorts-every-fact-the-state-holds-roots-among-them)): at every
  state the key of the state sorts every fact the state holds, root's among them, and then filters root's back
  out. On `send_more_money.ein` that is about 47 % of the instructions the engine now executes. The engine did
  so before the phase. **Not built; yours to call, in an item of its own: [78](../78-the-key-of-a-state/README.md).**

## The four

| # | question | what was asked | the default taken | the other reading |
|---|---|---|---|---|
| 1 | [Q-M1f.675](../../../../open_questions.md#q-m1f675--how-a-knowledge-base-grown-from-the-snapshots-is-recognised) | how a KB grown from the snapshot's is recognised | a stamp a layer, fresh where a layer is made or copied and kept where it is moved, and a mark of stamps and lengths, compared at every read (the count of root's layers) | a fact count alone, `Kb::facts_from(n)`, trusting every caller to hand a fork of the snapshot's KB (a copy of root would read a wrong delta, in silence); or root sealed when the snapshot is taken and its layers compared by `Arc` |
| 2 | [Q-M1f.676](../../../../open_questions.md#q-m1f676--whether-the-snapshot-keeps-its-set-of-facts) | whether the snapshot keeps its fact set | kept: the facts above the mark are asked of it, so the `open` facts the empty commitment's fork holds as root did, and any fact root held in its `open` layer that a fork writes into its stack, stay out exactly as the walk had them; the set is built once a run and copied with the snapshot, as before | the set dropped and root's `open` facts kept apart, the read above the mark taken as it stands |
| 3 | [Q-M1f.677](../../../../open_questions.md#q-m1f677--what-a-knowledge-base-that-did-not-grow-from-the-snapshots-gets) | what a KB that did not grow from the snapshot's gets | the walk, as before: no engine path in the corpus takes it (§ 1), the tests' copy does | refused, a panic: a caller legal today would fail |
| 4 | [Q-M1f.678](../../../../open_questions.md#q-m1f678--where-a-layers-stamp-comes-from) | where a stamp comes from | a process-wide atomic counter, relaxed: two draws where a KB branches, one where a layer is sealed; a stamp is compared and nothing else, and its debug form is a constant, so no output reads the number | a counter per thread with the thread's id; or the stamps kept by the KB beside each `Arc` |

## Your call

A number and what you want otherwise, in [`ANSWERS.md`](../ANSWERS.md) under item 76. Blank keeps the four as built.
