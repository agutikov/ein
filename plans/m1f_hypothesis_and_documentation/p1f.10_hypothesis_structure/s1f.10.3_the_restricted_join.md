# S1f.10.3 — The join, restricted

**Phase:** [P1f.10](README.md)
**Estimate:** 3 days
**Depends on:** [S1f.10.2](s1f.10.2_groups.md) — the cover, and its one owner.
**Blocks:** [S1f.10.5](s1f.10.5_ordering.md) only.

## Context

This is the stage where the phase touches the engine, and it is one predicate
in one place. `generate_layer`
([`solve.rs:1659`](../../../ein.rs/crates/ein-infer/src/solve.rs), through
[`apriori::filter_candidate`](../../../ein.rs/crates/ein-infer/src/apriori.rs))
builds layer *L*'s candidates by prefix-joining `a_prev`, then drops a
candidate for exactly two reasons: an element is no longer in `alive`, or a
learned clause subsumes it. The stage adds a third:

> **a candidate containing two members of one group is refused before the
> fork.**

It is answer-preserving by the group's own definition: the two members exclude
each other, so the fork would have saturated to `(false)` and died. What
changes is that the death is not paid for.

## The two things that make it not a one-liner

**It changes the learned clause set.** A pair that never enters never emits its
width-2 no-good, so `nogoods_emitted` falls, and every counter downstream of
it moves. That is the *point* — the clause was re-derived work — but it means
the stage must say what happens to the clauses' other readers: the unsat core
(`union_dead_cores`), the `--trace` proof's `learned_nogoods`, and
`lstate.dead`. A death the search never suffered is not evidence, and a core
assembled from deaths is a core assembled from fewer of them.

**It changes what a `Contradiction` explains.** Where every candidate at a
layer is group-refused, the layer is empty for a *structural* reason rather
than through death, and `finalise`'s Contradiction arm unions over a dead list
that is now shorter. On an unsatisfiable puzzle this could produce a smaller
— or an empty — core, which is
[CO-H3](../../m1e_review_processing/README.md#the-findings)(b)'s defect arriving by a second route.
The stage owns this, and the safe shape is: **a group refusal is recorded**,
with its own kind, so the core can cite it.

## Acceptance

- **Every corpus model set is identical, fact for fact**, at each entry's
  declared runs and at `-e`. The comparison is
  [`tree_traversal.rs`](../../../ein.rs/crates/ein-infer/tests/tree_traversal.rs)'s
  — *never of `k`* — and it runs over the whole corpus, not a sample.
- Every corpus **verdict word, `k`, and `exhausted`** identical.
- The enterings delta is recorded **per entry**, and the entries where it is
  zero are named — [I-L02](README.md#the-instances) expected among them.
- The no-good and dead-commitment counters are expected to **move**, and the
  stage file says which goldens that moves *before* it moves them. A re-bless
  that was not predicted is a stop.
- A group refusal is **narrated** — a new kind on the existing `layer` census
  row, or a new event field; not silent. `layer_census.py`'s sixteen counters
  get a seventeenth or the census stops adding up.
- The unsat core on every corpus entry that reports `Contradiction` is
  **unchanged**, or the change is stated and defended.

## Tasks

### Task T1f.10.3.1 — The predicate, off by default

Add it behind a `SolverConfig` field — `enable-group-refusal`, defaulting to
**false** for the duration of this stage — so the A/B is one flag and the
corpus can be swept both ways before anything becomes the default. The
precedent is every lever in
[`config.rs`](../../../ein.rs/crates/ein-core/src/config.rs) and the reason is
`EIN_OBLIGATION_CHOICE`'s: a knob whose two settings are being compared must
not re-bless the shape goldens while the comparison is running.

**Then flip the default in the same stage, deliberately, once the sweep is
clean** — a lever nobody turns on is [DO-M1](../../m1e_review_processing/README.md#the-findings)'s
shape in the config table.

### Task T1f.10.3.2 — The sweep

Every corpus entry, both settings, `--json-summary` both times, model sets
diffed fact for fact. This is the stage's evidence and it is the same shape as
[S1d.10.6](../../../docs/history/m1d_satisfiability/README.md#s1d106--the-traversal)'s
tree-vs-lattice verification, which is the precedent for trusting it.

Report: enterings before/after, wall before/after, no-goods emitted
before/after, and the **model set delta, which must be empty everywhere**.

### Task T1f.10.3.3 — What a refusal owes the core

Decide and implement one of:

- **(a)** a refused candidate is recorded in `lstate.dead` with a synthetic
  core of the two group members and a `Kind` of its own, so the unsat core and
  the proof read as they did;
- **(b)** a refusal is counted and narrated but not recorded, and the
  Contradiction arm's core is documented as *"over the deaths the search
  suffered"* — which is what it already is, but is not currently a distinction
  anyone has had to make.

(a) is more work and keeps every downstream surface honest; (b) is one
sentence and makes the core weaker on exactly the puzzles where structure did
the most work. **(a) is required, not preferred**, and the reason is
[Q-M1e.6](../../m1e_review_processing/open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)'s
operational form: a solution is a surviving commitment **whose every superset
died**, so a superset that is *refused* rather than entered has to count as
dead or the maximality test reads a refusal as an unexplored child and stops
recognising solutions. It is dead — the two group members exclude — but the
refusal must say so where the test looks.

That coupling is worth stating in the stage rather than discovering: this
phase and [Q-M1e.8](../../m1e_review_processing/open_questions.md#q-m1e8--exhausted-certifies-the-lattice-not-the-model-set)'s
fix touch the same layer bookkeeping, and whichever lands second has to know
about the first.

### Task T1f.10.3.4 — The fixture

One `examples/` fixture whose only purpose is the refusal: a small bijection
where layer 2's join proposes `n` same-group pairs and the counter says `n`
were refused. It goes in the corpus with an `:expect`, and it is what fails if
a later milestone removes the predicate.

## Notes

The saving compounds with depth and the census should say so: a 3-set is
refused if **any** of its three pairs excludes, so the fraction removed at
layer *L* grows with *L* — which is the direction that matters, because
[layer_census](../../../docs/history/m1d_satisfiability/layer_census.md) found
the deep layers are where the powerset lives.

There is an alternative implementation this stage deliberately does not take:
**seed the no-good store with the group clauses at load**. It needs no new
predicate — `filter_candidate` already refuses a candidate a clause subsumes —
and it is three lines. It is rejected because a clause is *learned* evidence
with a provenance, and manufacturing 250 of them at load makes every proof,
every core and every `learned_nogoods` list a mixture of what the search
proved and what the loader asserted. Should the stage find the predicate
route unaffordable, that is the fallback and the cost is a paragraph in
[`docs/kernel/inference/`](../../../docs/kernel/inference/README.md) saying so.
