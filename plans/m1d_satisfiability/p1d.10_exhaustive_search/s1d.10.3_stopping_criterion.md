# S1d.10.3 — Is there a stopping criterion?

**Phase:** P1d.10 (Exhaustive search over many models)
**Estimate:** 4 days → **3 days**: one candidate is dead and a fourth arrived
**Depends on:** [S1d.10.2](s1d.10.2_depth_required.md) and, since 2026-08-26,
[S1d.10.6](s1d.10.6_the_traversal.md) — because the fourth candidate is that
traversal's termination argument and cannot be judged before it exists.
**Runs 5th of six.**

---

## Re-aimed 2026-08-26

The stage was written from one puzzle's numbers, which is the position it
existed to get out of. All three of its candidates have moved, the Note's own
idea is refuted, and a fourth candidate arrived from outside the lattice.

**(b) is dead, measured twice.** Its own text predicted this — *"a criterion
that depends on `alive` shrinking is inert in exactly the regime that needs
it"* — and both censuses agree: `alive` is 96 at every one of the phase entry's
five measured layers, and corpus-wide it ever shrinks in **4 of 48** multi-layer
cells, all four in the pruning handful. Nothing is left to design.

**The Note's fourth idea is refuted by counting, and it was the strongest one.**

> the engine already learns clauses, and a clause is a proof that a region is
> empty. What it lacks is a way to notice that the clauses it holds already
> **cover** the remaining space.

They cannot cover it, because the remaining space is not empty. Layers 7–22 of
the phase's entry are **15 720 759 enterings, 0 deaths, 0 new models**
([S1d.10.4](s1d.10.4_conflict_mining.md)): every one of those commitments is a
*consistent* extension that happens to lead nowhere new. A clause proves a
region holds no **model**-bearing state only by proving it holds no **state**,
and these regions hold plenty. By depth 5 the store had a clause for each of
19 121 deaths, and the entire rest of the lattice added **eight**. Coverage is
not two layers late; it is the wrong shape.

**Which promotes (a) from "probably dies" to "the only lattice-side candidate
left".** The deep half is not barren of deaths by accident — it is barren
because the search is re-walking the neighbourhood of models it already has.
That is exactly what subsumption is about, and the stage's own objection to it
stands unanswered: the argument covers `c`, not `c ∪ {h}`, and expanding is
where the cost is. Whether a version survives *"every extension of `c` within
`alive` is also within some found model"* is now the question the whole
lattice-side of this stage rests on, and a written refutation closes the
lattice side entirely.

**And a fourth candidate arrived, from outside the lattice.**

### (d) Termination by discharge

A per-obligation tree ([S1d.10.6](s1d.10.6_the_traversal.md)) does not stop
because the lattice was exhausted. It stops because **every obligation is
discharged and none can be created**: each commitment retires at least one owed
instance, the owed set is finite at root, and a node that owes nothing and
derives nothing false is a model. The reconnaissance's emulation bottoms out at
depth **6** on a puzzle whose lattice needs 22 layers, and `--max-set-size`
plays no part in the argument at all.

That is a stopping criterion in the sense this stage wanted — sound, and it
proves the same thing sooner — but it is **not a criterion over the search this
stage was written about.** It replaces the search. Two things follow and both
belong here rather than in S1d.10.6:

- **It proves a different proposition**, and the difference has a name the rung
  already reports. A tree that discharges every obligation has decided every
  relation *some obligation owes*; the models also depend on the `uncovered`
  relations, which saturation must determine. `exhausted = true` after a tree
  therefore means *the obligations are discharged and the rest is determined*,
  and the second clause is a claim that needs evidence per program.
- **The two guarantees must not print the same word.** This is the phase's own
  rule turned on its most attractive candidate: if the tree's claim is weaker,
  it says so; if it is equally strong, the stage writes down why. Either way
  [S1d.10.5](s1d.10.5_contract.md) owns the vocabulary and this stage owes it
  the argument.

**What does not move:** (c) is unchanged and still ships as a flag with a
different verdict word if it ships at all; the ledger (T1d.10.3.5) is still the
deliverable even if nothing ships, and it now has **five** rows — (a), (b), (c),
(d) and coverage.

---

## Context

If every model is found by depth 3 and the search runs to depth 5 to prove it,
the prize is a **sound** argument that lets it stop at 3 — one that proves the
same thing, sooner. Failing that, an explicit *unsound* mode that says so in
the verdict.

Three candidates, in decreasing order of how much they would be worth and
increasing order of how likely they are to survive:

### (a) Subsumption by a found model

If a commitment `c` satisfies `c ⊆ facts(M)` for a model `M` already found,
then `sat(base ∪ c) ⊆ sat(base ∪ facts(M)) = M` by monotonicity, so entering
`c` cannot produce a model other than `M`.

**The obligation, and it is where this probably dies:** that argument covers
`c` itself, not its *extensions*. `c ∪ {h}` for `h ∉ M` is not a subset of `M`
and may be a live path to a different model. So the prune is on *recording*,
not on *expanding*, and expanding is where the cost is. Whether a version
survives that observation — perhaps "every extension of `c` within `alive` is
also within some found model" — is the stage's first question, and a negative
answer written down is a legitimate outcome.

### (b) An exhaustion argument over the alive set

The search is complete when no unexplored commitment can yield a new model.
`alive` is the set of hypothesis facts still live, and on this puzzle it never
shrinks — 96 at every layer, because nothing is refuted. **A criterion that
depends on `alive` shrinking is inert in exactly the regime that needs it**,
which S1d.10.1's census should confirm before anyone builds one.

### (c) Quiescence as a *reported* heuristic

"No new model for k consecutive layers." Not a proof, and it must never set
`exhausted = true`. Shipped, it is a `--stop-when-quiet k` that returns
`Ambiguity (not certified)` — the same honesty `stop_after` already has, where
`k = 1` is "a model", not "the model".

## Acceptance

- Each candidate gets a **written soundness argument or a written refutation**,
  and the refutations are as valuable as the proofs. F9's catalogue is the
  model: a rejected idea with a number beside it stays rejected.
- Anything sound is **measured against the census**: what it saves in the
  under-determined regime, and what it costs in the determinate one. A
  criterion that costs 2 % on `zebra -e` to save nothing on the corpus is
  inert, and inert means not shipped.
- Anything unsound that ships is a **flag**, is off by default, and changes the
  *reported verdict*. `exhausted` keeps its meaning: the lattice was exhausted.
- The counters are stated: a criterion that stops early changes
  `enterings_total` and `layers_explored`, and every golden that pins them
  moves. That is a re-baseline with an argument, not a surprise.

## Tasks

### Task T1d.10.3.1 — (a), argued to a conclusion
### Task T1d.10.3.2 — (b), tested against the census first

Cheap, and in the right order: if `alive` never shrinks in the regime, the
mechanism is dead before it is designed.

### Task T1d.10.3.3 — (c), as a mode with an honest verdict
### Task T1d.10.3.4 — (d), the discharge argument

The fourth the section above names. Not "does the tree terminate" — it does, and
the bound is the owed count — but **what its termination licenses the engine to
say**, which is the `uncovered` clause and is a claim per program rather than a
theorem.

### Task T1d.10.3.6 — The coverage idea, refuted in writing

The Note's own candidate, closed by counting rather than by design
(§ Re-aimed). It gets a row in the ledger like the other four, because it is the
idea a reader of this file is most likely to have again.

### Task T1d.10.3.5 — The ledger

Whatever the outcome, the catalogue: idea, argument, measurement, disposition.
It belongs beside F9's, and it is the deliverable even if nothing ships.

## Notes

- The strongest reason to expect something here: the engine already learns
  clauses, and a clause is a proof that a region is empty. What it lacks is a
  way to notice that the clauses it holds already **cover** the remaining
  space. That framing — coverage of the residual lattice by learned clauses —
  is worth a day on its own before settling for (c).
  > **It is not, and § Re-aimed is why**: the residual is not empty of states,
  > only of *new models*, so there is nothing for a clause to be a proof of.
  > Kept above rather than deleted because the framing is right and only the
  > premise is wrong, and a reader who deletes it will re-derive it.
