# S1f.21.26 — the key of a state at its own size

**Phase:** [P1f.21](README.md), part 2 ([the order](README.md#the-order-two-parts))
**Status:** planned 2026-11-25 on the user's answer to review item 78, *"(b)"*
([the README](README.md#the-answers-to-part-2s-items-and-item-87)); **not started**, on the user's word.
**Writes:** `ein-infer/src/region.rs` (the node function) and what reads a state's key whole; their tests; the
costs page's final table, re-taken.

## What it is

[V-86](findings.md#v-86--a-states-key-sorts-every-fact-the-state-holds-roots-among-them). At every state the node
function takes the state's key, the sorted list of its facts beyond root's, which is how two commitments that reach
one state are told from two that do not (`region.rs`, `classes.entry(out.state)`; `canon::distinct_by_state`). It
makes the key the long way (`region.rs`, `node_in_region`):

```rust
let state = crate::canon::state_key(&fork);          // every fact the state holds, root's fixpoint too, sorted
let delta: Vec<FactId> = state.iter().copied()
    .filter(|f| !axes.root_facts.contains(f))        // root's filtered back out
    .collect();                                      // the key
```

On `send_more_money.ein` the key is 38 to 39 % of the instructions and the filter 9 to 10 %; on
`equation_two_unknowns.ein` about 11 % and 2 to 3 % (review item 78's profile, in `git log` at `bc5146bb`). The engine
did so before the phase.

**The move is S1f.21.23's** ([Q-M1f.674](../open_questions.md#q-m1f674--the-facts-a-resumed-state-added-read-off-its-own-layers)):
a fork's own facts are its layers above root's mark, `Kb::facts_above`, which `Saturator::new_facts_of` already reads,
filtering the few against root's set (an `open` fact the fork holds as root did is root's). So the key is

```rust
let mut delta = snapshot.new_facts_of(&fork);        // the fork's own facts, at their own size
delta.sort_unstable();                               // still sorted: derivation order differs between commitments
```

**the same set in the same order**, so every key, every class of states, every answer and every firing is as it was.

## Tasks

1. **Who reads a key whole**, root's facts in it, settled first: the describe route's splits (`region.rs`,
   `split_in_fork`), the read-out's check of paths (`read_out.rs`, `crate::canon::state_key(kb)`), `ein-einb`'s
   solutions, `canon::distinct_by_state` and its callers. Each either takes the key above root, which tells states
   apart as exactly, every state holding root's facts, or keeps the whole one where it is no state's cost; recorded,
   one line each.
2. **The node function** takes its key above root's mark as above, the filter gone. On a build with debug assertions
   (the gate's) the new key is held to the old at every state, as S1f.21.23's read is held to the walk.
3. **What holds it**, as S1f.21.21 and S1f.21.23 held theirs: `ein solve -p` and the digest of the events on every
   corpus file, before and after, identical; the gate.
4. **The cost taken again**: the profile of item 78 on both programs, and the final table of
   [the costs page](costs/README.md#the-final-table-before-after-improved-after) re-taken with a column for this
   stage, on an idle machine on AC, through `utils/bench_env.sh`.

## What must hold

- **No key moves**, so no class, no answer, no firing: task 3.
- The defaults a stage takes are recorded with ids the coordinator gives, and its findings likewise.

## Record

*Not started.*
