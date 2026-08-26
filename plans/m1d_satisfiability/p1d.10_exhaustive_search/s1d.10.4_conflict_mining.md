# S1d.10.4 — Conflict mining when a layer is barren

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 4 days → **0.5 day**: what is left is writing the refutation down
**Depends on:** [S1d.10.1](s1d.10.1_why_it_does_not_finish.md)
**Runs 3rd of six**, and it is the stage that closes rather than builds.

---

## The answer, 2026-08-26 — there are no conflicts down there

The stage's premise, in its own words below: *"deaths live **deeper**, where
enough hypotheses are committed to contradict. A cardinality-BFS reaches that
depth only by paying for every shallower combination first. A dive reaches it
directly, and the clause it brings back prunes the breadth-first frontier."*

Measured on `examples/zebra2-minus-15-obligations.ein`, at `-j16`, by reading
`dead_post` off three runs of the same file:

| layers | enterings | deaths |
|---|---:|---:|
| 1–5 | 618 076 | 19 121 |
| **6** | **865 757** | **8** |
| **7–22** | **15 720 759** | **0** |

`dead_post` is 19 129 at `-m 6` and 19 129 at `-m 38`, and `dead_pre` is 0 at
every cap. **Fifteen and a half million commitments are entered below layer 6
and not one of them is refuted.** A dive whose only product is clauses would
come back with none — not because the dive is badly aimed, but because the
region it dives into is consistent.

That is an answer to the question this stage asked, and it is the one
[T1d.10.4.5](#task-t1d1045--the-decision) named as legitimate: *record as
inert, with the number*. Three consequences worth stating with it:

1. **The reasoning in § Context is not wrong; the fact it assumed is.** Pruning
   *does* come from deaths, a barren layer *does* have none, and a dive *would*
   reach depth directly. What fails is *"deaths live deeper"* — on this puzzle
   they live at depths 2 and 3 and then stop, and the last one is at layer 6.
   The mechanism is sound and its fuel is absent.
2. **It generalises as far as the corpus lets anything generalise here.** Of
   the 51 cells that reach the search, **35 never learn a clause at all** and
   **42 never have one drop a candidate**
   ([layer census §5](layer_census.md#5-what-the-clause-store-is-worth), and the
   2026-08-26 re-take, where the two counts are 35 of 51 and 42 of 51 against
   the original's 35 of 49 and 41 of 49). The entry with the most clauses in the
   corpus — 11 577 on `zebra2-minus-15` at `-m 3` — is the one whose deep half
   has no deaths to mine.
3. **The cost side is refuted too, which removes the fallback.** The stage's
   note anticipated a partial win where the barren regime's cost was
   concentrated shallow. It is not: the per-entering cost **rises** with depth,
   0.0812 → 0.0845 → 0.0873 ms at a constant `-j16`
   ([S1d.10.2](s1d.10.2_depth_required.md)). There is no cheap deep region to
   harvest and no expensive shallow one to protect.

**What survives, and it is not small.** The stage's obligation (2) —
*"completeness of the surrounding search is unaffected, because the BFS still
visits everything the clauses do not exclude"* — is the argument that makes a
*bounded excursion off the frontier* admissible at all, and
[S1d.10.6](s1d.10.6_the_traversal.md) needs the same shape of argument for a
different reason: a per-obligation branch is complete at its node because the
alternatives are jointly exhaustive. Obligation (4) — *answer F9's E10, do not
ignore it* — transfers verbatim, and so does the note about clauses subsumed on
arrival. What does **not** transfer is the trigger, the dive policy and the
wasted-dive accounting, because there is nothing to trigger on.

**What would re-open it**, stated as a property of a corpus entry rather than as
a hope: an entry whose deaths per entering *rise* with depth. Every entry
measured so far falls, and the phase's own goes
**31.3 % → 23.0 % → 4.2 %** at layers 2–4, **0.0009 %** at layer 6 and **0**
after it. The census's `deaths` column is where a counter-example would show up
without anyone having to look for it.

---

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
   [design/08](../../../docs/history/m1a_rust/design/08_parallelism.md) §7 rejected parallel depth-first on
   exactly this ground. So the dive is a flag, and default-on requires the
   [Q-M1a.18](../../../docs/history/m1a_rust/open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)-shaped
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
- Interaction with [P1a.7](../../../docs/history/m1a_rust/README.md#p1a7--parallelism) stated: a dive is
  independent work and an obvious thing to run on a spare core, but it writes
  clauses to the shared store, which is exactly the mid-layer root mutation
  [S1a.7.0](../../../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit) found to be the
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
