# D3 — Q-M1e.8: **B**, and what B takes is the check

> **Decided 2026-08-28 by the user: option B — and B's content is a
> double-check that the existing implementation meets the solution criteria of
> [`solution_semantics.md`](../../../../docs/kernel/inference/solution_semantics.md)
> § 2, not the maximality fix.** That answers the objection this file raised
> against B: a conformance check changes no verdict, so the stage keeps its
> acceptance. The **fix** still has C's home — see
> [§ What the check found](#what-the-check-found), which also shows the fix
> menu forks on a question this stage does not own.

**Decides:** whether M1e ships knowing that `ein solve -e -L
examples/lattice/02_genuine_3set_death.ein` states a falsehood.
**Touches:** [T1e.1.1.3](README.md#task-t1e113--q5-derive-lattice02-by-hand-against-the-ruling)
step 5 (*file, do not take* — unchanged for the fix) and
[T1e.1.1.4](README.md#task-t1e114--the-record-site-conformance-check), which is
the check.
**Probes:** [`probes/run_record_sites.sh`](probes/run_record_sites.sh) and the
three fixtures beside it.

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

## What the check found

`solution_semantics.md` § 2 states three conjuncts:

```
solution(S)  ≡  S is saturated
              ∧ S is consistent
              ∧ ( S owes nothing  ∨  ∀ h ∈ alive₀ \ integrated(S): S ∪ {h} is dead )
```

`record_node` has **four** callers, and the check is one row each — *which
conjunct does this caller establish, and when?*

| caller | saturated? | consistent? | third conjunct |
|---|---|---|---|
| `:1977` — the normal layer path, **every corpus solve** | **no.** `complete()` at `:1895` runs the whole generation pipeline, and its lookahead kill cache writes `(not h)` into `result.kb` — the fork recorded 80 lines later. The comment above it calls that fork *"unmutated"* | established by `try_commitment_set` **before** that write, never re-checked | `complete()` — § 6's own admitted approximation |
| `:1030` — `tree_node` | **no**, same shape (`complete()` at `:1024`) | same | `complete()`, on top of the root-only rung premise ([Q6](../../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md)) |
| `:1118` — phase 1, `alive = ∅` | **no.** `compute_alive` at `:1098` writes the kill cache into **root** | checked at `:1091`, **before** that write | `alive = ∅` |
| `:1550` — between layers, `alive = ∅` | **no.** the layer's writebacks and `compute_alive` at `:1534` | last checked at phase 1, or inside the cascade | `alive = ∅` |

**The first conjunct is established at no record site, and the second is
established at every site before the last write into the KB it guards.** Said
without the table: **the engine records `S ∪ K`, and checked the criteria
against `S`** — where `K` is what the search wrote into that KB for its own
bookkeeping. `(not P)` is an ordinary match form
([`02_patterns.md`](../../../../docs/kernel/ir/03-ein-lang/02_patterns.md)), so
a program may read `K`; when it does, `S ∪ K` is not saturated and can be
inconsistent.

Three of the four are witnessed, by `sh probes/run_record_sites.sh`
(2026-08-28, `a3f4e7b`) — each recorded model's own negatives fed back into its
own program, one model at a time:

| fixture | site | default | that model, re-saturated |
|---|---|---|---|
| [`alive-empty-phase1.ein`](../../../../examples/ein-bugs/alive-empty-phase1.ein) | `:1118` | `Solution` k=1 | **`Contradiction`** |
| [`alive-empty-interlayer.ein`](../../../../examples/ein-bugs/alive-empty-interlayer.ein) | `:1550` | `Solution` k=1 | **`Contradiction`** |
| [`complete-records-stale.ein`](../../../../examples/ein-bugs/complete-records-stale.ein) | `:1977` | `Ambiguity` k=2 | model 1 **`Contradiction`**, model 2 `Solution` |

The fourth, `tree_node`, is the same two lines and is
[S1e.1b.6](../../p1e.1b_hypothesis_structure/s1e.1b.6_obligations_under_hypothesis.md)'s
to witness.

### What that does to § 6's table

| § 6 says | the check says |
|---|---|
| `complete(S) ⇒ solution(S)` — **yes**, so *"the engine never records a false model"* | **no.** Three witnesses, stock config, three different record sites |
| `solution(S) ⇒ complete(S)` — **no**, the engine under-reports | unchanged, and it is this decision's original subject |

So the engine is **neither** sound nor complete against § 2, and the page
claims the half that is false. The page edit is
[D9](d9_kernel_page_overclaims.md)'s; the row it has to rewrite is the one
marked *yes*.

### And the fix menu forks on Q-M1e.7

Three candidate fixes, and only one covers all three witnesses:

| | fix | phase 1 | inter-layer | normal path |
|---|---|:---:|:---:|:---:|
| **i** | the kill cache goes fork-local — nothing it writes reaches a recorded KB | ✓ | ✗ | ✓ |
| **ii** | re-saturate and re-check before recording (D1's dirty bit) | ✓ | ✓ | ✓ |
| **iii** | record `S`, not `S ∪ K` — strip engine-written negatives from the model | ✓ | ✗ | ✓ |

The inter-layer column is what separates them: its trigger is the **singleton
writeback**, not the cache, and design/08 holds that a writeback's negative is
*entailed* — so (i) and (iii) do not reach it.

And (ii) and (iii) **give different answers**, which is the finding that makes
this more than a bug list. On `complete-records-stale.ein`, (ii) refuses model 1
and reports `k = 1`; (iii) keeps both and reports `k = 2` with model 1 rendered
as `{(q A)}` — which is what `-K` already prints, and which is *correct* under
§ 2 read on `S` alone. Whether `K` is part of the state is exactly
[Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)
— *the read-out prints the solution **KB** and calls it a model*. So the
check's last product is a dependency, stated: **the fix cannot be chosen before
Q-M1e.7 is.** It was unowned when the check ran, was assigned on 2026-08-28 to **T1e.1.1.4
step 4**, and was **ruled the same day: A** — the recorded object is the
*state*, `model` is a projection of it, and § 2 is evaluated on the state.

**So the fix is chosen: (ii), re-saturate and re-check.** What settled it was
entailment, not cost: a kill-cache negative means *`S ∪ {h}` derives `(false)`
in one firing* and a writeback negative means *the fork for `{h}` died*, so
both are **consequences of `S`**. A rule reading one is reading something true,
and a state its own rules refute is inconsistent — (iii) would hide an entailed
contradiction, and (i) never reaches the inter-layer site.

That also disposes of the objection this table raised against (ii). `-K`
reports `k = 2` on `complete-records-stale.ein` where (ii) reports `k = 1`, and
that is **not** a lever changing the answer: `{(q A)}` entails both negatives
through `kill-p` and thence `(false)` through `totality`, so `k = 1` is right
and `-K` is the cache being *less complete*. Under (ii) all three probes answer
what `-L` answers, which is what the hand derivations support.

## Options

| | what happens | consequence |
|---|---|---|
| **A — file** (status quo) | S1e.1.1 records the derivation, the goldens and the fix sketch; Q-M1e.8 keeps *owner unassigned* | M1e closes with a known false verdict shipped and a written explanation of it. Defensible: M1e processes a review, and this is not one of the 63 |
| **B — take it in S1e.1.1** ✅ | the stage grows a fourth task — **the check, not the fix** | as *the fix*, B breaks the stage's own acceptance bullet (*"Nothing in this stage changes a verdict"*), which is what this file argued. As **a conformance check of the implementation against § 2** it does not: it produces a matrix, a ruling and three probes, and moves nothing. Chosen 2026-08-28 |
| **C — take it in P1e.2** | a new stage beside [S1e.2.1](../../p1e.2_high/s1e.2.1_correctness.md) | P1e.2 is already the phase that fixes *"the surface either honours its contract or refuses to ship"* — CO-H3(b) is the same seam, an evidence-free `Contradiction`. Natural home, and the phase is already budgeted for engine work |
| **D — new milestone** | Q-M1e.8 gets an M-number | honest if the fix turns out to need the layer bookkeeping reshaped, which T1e.1.1.3 cannot know in advance |

**Chosen: B for the check, and C stays the recommendation for the fix** —
which fix, decided on 2026-08-28 with
[Q-M1e.7](../../open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model),
is **(ii) re-saturate and re-check**.
The original argument for C stands and is kept: the defect and CO-H3(b) are one
seam — a `Contradiction`
that asserts more than the search established — and P1e.2 is where that seam
is already being worked. A leaves a known-false shipped surface for the sake of
a scope line — and what B turned out to be able to take without breaking the
stage's contract is the check below, which is why A is no longer the fallback.

## Goldens it would move

Predicted before it moves, per the milestone's rule. On the corpus:
`lattice/02`'s `-L` lever cell (verdict and `k`), and any entry where a
surviving commitment currently ends the search without being recorded. The
[layer census](../../../../docs/history/m1d_satisfiability/layer_census.md)'s
25 entries whose enterings are exactly `Σₖ C(alive, k)` are where to look
first. `corpus_shapes.md5` does **not** move — the KB shape is unchanged.
