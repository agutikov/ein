# D3 — Q-M1e.8: file the fix, or take it here?

**Decides:** whether M1e ships knowing that `ein solve -e -L
examples/lattice/02_genuine_3set_death.ein` states a falsehood.
**Touches:** [T1e.1.1.3](README.md#task-t1e113--q5-derive-lattice02-by-hand-against-the-ruling)
step 5, which currently says *file, do not take*.

## The defect, reproduced

```
$ ein solve -e --no-lookahead examples/lattice/02_genuine_3set_death.ein
  solutions (k)   0            exhausted = true      7 enterings, 3 layers
  verdict         No solution — the constraints are contradictory

  unsat core (3 facts)
    (a-prop X)  (b-prop X)  (c-prop X)
```

That program has **three** solutions under
[Q-M1e.6](../../open_questions.md#q-m1e6--what-is-a-solution-and-what-is-a-model)
— `{h₁,h₂}`, `{h₁,h₃}`, `{h₂,h₃}` — and the run **found all three states**: it
entered each pair, each survived, and it then proved every triple dead.
Nothing was truncated, so `exhausted` is honestly `true`; the lattice really
was walked to the end.

What failed is that no surviving pair was flagged `solved`, because
`complete()` with the lookahead off still proposes the third candidate — and
`finalise` reads only `lstate.nodes`, which is empty, so the `Contradiction`
arm unions the dead cores and asserts unsatisfiability.

## Why the fix is small

Q-M1e.6's operational form is *a solution is a maximal alive commitment — one
with no live child*, and **layer `n+1` already computes that**. A surviving
`C` at layer `n` is a solution iff none of its supersets survived at layer
`n+1`; the supersets apriori declined to generate are the ones a subset
already proved dead.

The engine computes it and throws it away — `a_layer` becomes `a_prev` and is
never asked which of its members had a live child. Retaining it is one bitset
per layer:

```rust
// sketch, inside the layer loop
let mut had_live_child = bitvec![0; a_prev.len()];
//   … in commit_entering, when an (n+1)-set is alive or solved:
//     mark every parent of it in a_prev
// … at the barrier:
for (i, c) in a_prev.iter().enumerate() {
    if !had_live_child[i] { /* C is a solution — record it */ }
}
```

No new fork, no new saturation. Free under `-e`; under `-n 1` it defers
recognising a model by one layer, which is a trade-off to **measure**, not to
assume.

## What it is not

It is **not** *"make the lookahead unconditional"*. That is still an
approximation of clause 3 — a better one, and 448× more expensive on
`branching/06`'s fast path — and it would leave the same defect at any state
whose remaining candidates need two firings to die. The maximality test is the
definition, not an approximation of it.

## Options

| | what happens | consequence |
|---|---|---|
| **A — file** (status quo) | S1e.1.1 records the derivation, the goldens and the fix sketch; Q-M1e.8 keeps *owner unassigned* | M1e closes with a known false verdict shipped and a written explanation of it. Defensible: M1e processes a review, and this is not one of the 63 |
| **B — take it in S1e.1.1** | the stage grows a fourth task | S1e.1.1 is a *probe* stage that changes no verdict; adding an engine change breaks its own acceptance bullet (*"Nothing in this stage changes a verdict"*) |
| **C — take it in P1e.2** | a new stage beside [S1e.2.1](../../p1e.2_high/s1e.2.1_correctness.md) | P1e.2 is already the phase that fixes *"the surface either honours its contract or refuses to ship"* — CO-H3(b) is the same seam, an evidence-free `Contradiction`. Natural home, and the phase is already budgeted for engine work |
| **D — new milestone** | Q-M1e.8 gets an M-number | honest if the fix turns out to need the layer bookkeeping reshaped, which T1e.1.1.3 cannot know in advance |

**Recommended: C.** The defect and CO-H3(b) are one seam — a `Contradiction`
that asserts more than the search established — and P1e.2 is where that seam
is already being worked. A leaves a known-false shipped surface for the sake
of a scope line; B breaks the stage's own contract.

## Goldens it would move

Predicted before it moves, per the milestone's rule. On the corpus:
`lattice/02`'s `-L` lever cell (verdict and `k`), and any entry where a
surviving commitment currently ends the search without being recorded. The
[layer census](../../../../docs/history/m1d_satisfiability/layer_census.md)'s
25 entries whose enterings are exactly `Σₖ C(alive, k)` are where to look
first. `corpus_shapes.md5` does **not** move — the KB shape is unchanged.
