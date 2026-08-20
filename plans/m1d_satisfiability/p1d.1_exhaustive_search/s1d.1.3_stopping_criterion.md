# S1d.1.3 — Is there a stopping criterion?

**Phase:** P1d.1 (Exhaustive search over many models)
**Estimate:** 4 days
**Depends on:** [S1d.1.2](s1d.1.2_depth_required.md)

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
which S1d.1.1's census should confirm before anyone builds one.

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

### Task T1d.1.3.1 — (a), argued to a conclusion
### Task T1d.1.3.2 — (b), tested against the census first

Cheap, and in the right order: if `alive` never shrinks in the regime, the
mechanism is dead before it is designed.

### Task T1d.1.3.3 — (c), as a mode with an honest verdict
### Task T1d.1.3.4 — Anything the census suggests

S1d.1.1 and S1d.1.2 may name a fourth. Leave room for it — the three above
were written from one puzzle's numbers, which is exactly the position this
phase exists to get out of.

### Task T1d.1.3.5 — The ledger

Whatever the outcome, the catalogue: idea, argument, measurement, disposition.
It belongs beside F9's, and it is the deliverable even if nothing ships.

## Notes

- The strongest reason to expect something here: the engine already learns
  clauses, and a clause is a proof that a region is empty. What it lacks is a
  way to notice that the clauses it holds already **cover** the remaining
  space. That framing — coverage of the residual lattice by learned clauses —
  is worth a day on its own before settling for (c).
