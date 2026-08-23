# S1d.10.4 — Conflict mining when a layer is barren

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 4 days
**Depends on:** [S1d.10.1](s1d.10.1_why_it_does_not_finish.md)

## Context

The user's proposal, and it addresses the mechanism S1d.10.1 identifies rather
than the symptom: **when a layer grows rapidly and emits no usable clauses,
dive depth-first to mine facts.**

The reasoning is sound in outline. Pruning comes from deaths; a barren layer
has none; deaths live *deeper*, where enough hypotheses are committed to
contradict. A cardinality-BFS reaches that depth only by paying for every
shallower combination first. A dive reaches it directly, and the clause it
brings back prunes the breadth-first frontier. That is conflict-driven
learning with restarts, and the engine already has the learning half — it is
the diving half that is missing.

**What it is not** is a change of search strategy. The BFS stays; the dive is a
*probe* whose only product is clauses.

## The obligations, up front

1. **Soundness of the learned clause.** A clause from a dived commitment `c`
   is `¬(⋀ c)`, and it is valid for the same reason every learned clause is:
   `sat(base ∪ c)` contains a contradiction and the KB is append-only. The
   dive does not need to be complete to learn soundly — it needs only to be
   *correct about the deaths it finds*.
2. **Completeness of the surrounding search is unaffected**, because the BFS
   still visits everything the clauses do not exclude. This is the property
   that makes the whole idea admissible where a straight depth-first search
   would not be.
3. **The counters move.** Enterings performed by the dive are enterings, and
   the clauses change which later candidates are filtered.
   [design/08](../../m1a_rust/design/08_parallelism.md) §7 rejected parallel depth-first on
   exactly this ground. So the dive is a flag, and default-on requires the
   [Q-M1a.18](../../m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)-shaped
   decision, taken explicitly.
4. **F9's E10 must be answered, not ignored** — "cardinality layering already
   *is* breadth-first deepening; there is no DFS depth bound to raise". True,
   and this is not iterative deepening: it is a bounded excursion *off* the
   BFS frontier to harvest clauses, with the BFS unchanged. The distinction has
   to be written down, because from a distance the two look alike.

## Acceptance

- A **trigger**, measured rather than guessed: what makes a layer "barren"
  (deaths per entering, clauses per death, next-layer filter rate — all from
  S1d.10.1's census) and what the threshold is.
- A **dive policy**: how deep, how many, which candidates to extend, and what
  happens when a dive finds a model instead of a conflict (it is a model —
  record it, deduped by `state_key`, exactly as the BFS would).
- **Measured on both regimes.** The number that matters is enterings-to-
  completion on `zebra2-minus-15 -e`, against the current "does not finish".
  The number that decides whether it ships by default is the cost on the
  determinate puzzles, where the trigger should essentially never fire — and
  if it does fire there, the trigger is wrong.
- **Wasted-dive accounting.** A dive that finds nothing is pure cost; report
  dives attempted, clauses harvested, and candidates those clauses filtered.
  Without that third number this is unfalsifiable.
- Interaction with [P1a.7](../../m1a_rust/p1a.7_parallelism/README.md) stated: a dive is
  independent work and an obvious thing to run on a spare core, but it writes
  clauses to the shared store, which is exactly the mid-layer root mutation
  [S1a.7.0](../../m1a_rust/p1a.7_parallelism/s1a.7.0_speculation_audit.md) found to be the
  hard case. Do not discover that twice.

## Tasks

### Task T1d.10.4.1 — The barren-layer trigger
### Task T1d.10.4.2 — The dive
### Task T1d.10.4.3 — Clause harvesting and integration

Where the harvested clauses land, and when. Integrating them mid-layer is the
same hazard P1a.7 measured; integrating at the layer barrier is the cheap and
obviously-sound option, and its cost is that the current layer does not
benefit from them.

### Task T1d.10.4.4 — Measure both regimes
### Task T1d.10.4.5 — The decision

Ship on, ship behind a flag, or record as inert. All three are acceptable
outcomes; only an unmeasured one is not.

## Notes

- The failure mode to design against is a dive that keeps finding the *same*
  conflict by a different route, harvesting clauses that are subsumed on
  arrival. `emit_nogood`'s subsumption check will report that honestly —
  `nogoods_subsumed` climbing while `nogoods_emitted` does not is the signal,
  and it should be a first-class number in the accounting rather than
  something noticed later.
