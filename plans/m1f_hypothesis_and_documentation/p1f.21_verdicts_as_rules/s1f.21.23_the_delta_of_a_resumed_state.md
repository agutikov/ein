# S1f.21.23 — the delta of a resumed state, read off its own layers

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-20 by the coordinator on the user's word, *"of course build"*
([Q-M1f.674](../open_questions.md#q-m1f674--the-facts-a-resumed-state-added-read-off-its-own-layers)); **not
started**.
**Writes:** `ein-infer/src/saturator.rs` (`Snapshot::new_facts_of` and what it reads), its callers in
`region.rs`, `ein-core/src/kb.rs` if the read needs one, their tests; two comments of `obligations.rs`.

## What it is

[V-78](findings.md#v-78--a-resumed-state-walks-every-fact-it-holds-to-find-what-is-new). A state of a
search is root's fixpoint branched, the commitment's facts written, and the saturation resumed from root's
snapshot. To resume, the saturator takes the facts the state holds and root did not. `Snapshot::new_facts_of`
finds them by walking every fact the state holds and asking the snapshot's set for each: root's whole fixpoint,
at every state, to find the handful the commitment wrote a moment before. On `send_more_money.ein` root holds
1 867 facts and the search builds 18 129 states; a profile puts the walk at about 17 % of its instructions. The
engine did so before the phase.

## Tasks

1. **The delta read off the layers above the snapshot's.** The facts a state added are in its own layers, above
   root's sealed ones, in the order they were written. `Kb` already reads its facts from a position on and skips
   whole layers below it. The delta costs its own size.
   - **The same facts in the same order**: the order of the delta is the order a state's derivations arrive in,
     which a core and an explanation read. A debug check on the gate's build holds the new read to the walk it
     replaces, a check a state and bounded as the earlier stages' are.
   - **Every caller**: the node function, the trace's, the saved node's, and the tests' own.
   - A fact the commitment names that root already held is no part of the delta, as today.
2. **[V-84](findings.md#v-84--two-comments-of-the-obligations-reader-name-forms-the-parser-refuses)**:
   the two comments of `obligations.rs` that name forms the parser refuses, reworded.
3. **Measured**: instructions executed, one worker, pinned, on the cost page's five programs, before and after.

## What must hold

Every corpus answer as before, and the order of every firing: `ein solve -p` and the digest of `--events`
identical on every corpus file, as S1f.21.21 checked its own. No golden moves. The gate on the last commit, and
the two slow render sweeps.

## Record

Nothing yet.
