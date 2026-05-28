# P1.5b — Per-candidate processing in layer N (lattice engine)

Companion to [`diagrams/algorithm_layer_n.dot`](diagrams/algorithm_layer_n.dot).
Renders the per-candidate flow inside one BFS layer of the
**lattice engine**'s shared private `_explore_layers`
helper, used by both public entries (`gaps_solve`,
`contradictions_solve`). The layer-wrapping loop (enumerate
→ process → close layer → recompute alive → next layer) sits
one level above.

The monotonic engine's per-candidate flow is documented in
[`s1.5b.5_monotonic_backbone.md`](s1.5b.5_monotonic_backbone.md);
it shares the core shape with the lattice engine but adds
early-terminate on first goal-sat and omits both
`LatticeProof` collection and per-set storage.

**Design context — 2026-05-28.** Earlier drafts of this
document described the lattice flow with multi-parent
integrate, per-set kb storage of every visited commitment,
per-parent `is_unconditional_at` checks, and a runtime
sanity step. Under M1's monotone + order-commutative
saturation, those steps reduce to **idempotent multi-writes
of the same nogood/forced-positive** or to **always-True
predicates** — i.e., mechanical redundancy. The flow
described below uses **flat root-writes** (single write to
`root._nogoods` / `root.facts` instead of per-parent bubble)
and bounded per-set storage (SetNodes only when
`store_lattice=True`; SOLUTION kbs always; dead unsat-cores
under `contradictions_solve`). A separate engine-design
correction the same day clarified that the phase ships
**two engines, three entries** — `monotonic_solve` (solution
mode only), `gaps_solve`, `contradictions_solve` — rather
than a single `solve(mode=…)` dispatcher. See
[`project-set-search-unified` memory] + the 2026-05-28
conversation; [README](README.md) for the phase shape.

## Render

```sh
dot -Tsvg diagrams/algorithm_layer_n.dot -o diagrams/algorithm_layer_n.svg
# or
dot -Tpng diagrams/algorithm_layer_n.dot -o diagrams/algorithm_layer_n.png
```

The diagram uses six colour bands:

| colour            | meaning                                                |
|-------------------|--------------------------------------------------------|
| light blue        | setup / data flow / `try_commitment_set`                |
| light yellow      | decision diamonds                                       |
| light gray        | optional / opt-in feature (hash dedup, sanity)          |
| pale green        | ALIVE outcome (intermediate)                            |
| medium sea green  | SOLUTION outcome (collected; terminal under `early_terminate`) |
| salmon            | DEAD outcome                                            |
| plum              | flat root-write (nogood emit or unconditional merge)    |

## Engine surface — three entries

```python
# Engine 1: monotonic — solution mode only, no storage.
def monotonic_solve(
    root_kb,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    dumper: MonotonicDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Verdict, MonotonicStats]:
    """Solution mode. Early-terminate on first goal-sat.
    Per-candidate flow is the §3 algorithm below WITH
    early-terminate at §3c.ii AND no LatticeProof collection.
    Documented in s1.5b.5_monotonic_backbone.md."""

# Engine 2: lattice — two separate entries sharing
# _explore_layers.

def gaps_solve(
    root_kb,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,                # orthogonal storage toggle
    dumper: LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Ambiguity, LatticeStats]:
    """GAPS contract: always Ambiguity. Per-candidate flow
    is the §3 algorithm below WITHOUT early-terminate,
    collecting every satisfying commitment into
    proof.solutions. State-hash dedup MERGE is auto-disabled
    under gaps_solve (correctness — distinct satisfying
    commitments must register separately) even when
    store_lattice=True."""

def contradictions_solve(
    root_kb,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,
    dumper: LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Contradiction, LatticeStats]:
    """CONTRADICTIONS contract: always Contradiction. Per-candidate
    flow is the §3 algorithm below WITHOUT early-terminate,
    collecting every dead commitment into proof.dead_commitments.
    State-hash dedup merge is safe here (refutation map is
    multi-label by construction)."""
```

`store_lattice` toggles whether `LatticeProof.kb_index`
(per-visited-commitment `SetNode` storage) is built.
Independent of which lattice entry is called.

## Algorithm

For layer ``N`` with frozen ``A_1, …, A_{N-1}`` and the current
``root._nogoods``:

### 1. Generate candidates (Apriori prefix-join)

Sort every set in ``A_{N-1}`` canonically (Q1.5b.3 — tuple of
sorted ``FactId``s); for each prefix-class (sets agreeing on
their first ``N-2`` elements), pair every two members
``(a_1, …, a_{N-2}, b)`` and ``(a_1, …, a_{N-2}, c)`` with
``b < c``; emit ``(a_1, …, a_{N-2}, b, c)`` once. Each
size-``N`` candidate produced exactly once.

### 2. Filter

Drop ``C`` if any of:

- some ``(N-1)``-subset of ``C`` is **not** in ``A_{N-1}``
  (covered by construction with prefix-join on closed
  ``A_{N-1}``);
- ``matches_any_nogood(C, root._nogoods)`` — some learned
  clause is a subset of ``C``, so committing all of ``C``
  triggers the known contradiction;
- some element of ``C`` is no longer in the current alive set
  (a ``(not h)`` landed at root since layer-``N-1`` closed —
  e.g. from a forced-positive bubble or another layer-``N``
  candidate's death within this same layer under Option A
  cadence).

### 3. For each surviving candidate ``C`` of size ``N``

#### 3a. `try_commitment_set(root, C)`

The single all-in-one primitive
([`commitment.py`](../../../ein.py/src/ein_bot/inference/commitment.py)):

```
fork ← root.fork()
for (rn, args) ∈ C:
    fork.add_fact(Fact(rn, args, layer=REASONING, prov=from_hypothesis))
if ContradictionDetector(fork).detect():
    return CommitmentSetResult(kind="dead-pre", unsat_core=…)
firings ← Saturator(fork).saturate()
if ContradictionDetector(fork).detect():
    return CommitmentSetResult(kind="dead-post", firings=…, unsat_core=…)
return CommitmentSetResult(
    kind="alive",
    firings=firings,
    unconditional_facts=extract_unconditional(fork, root, C),
)
```

**Why fork from root, not from a parent.** Saturation
commutativity guarantees ``sat(root ∪ C) = sat(P_i.kb ∪ (C \ P_i))``
for any ``P_i ⊊ C``. Earlier designs forked from a parent's
kb_snapshot to reuse its firings. Under flat root-writes the
root accumulates unconditional consequences of every prior
alive entering, so root.kb is closer to kb(C) than any
specific stored P_i.kb (whose snapshot froze at its layer).
Forking from root is at least as fast and avoids the storage
cost.

**Unconditional-fact extraction.** A fact `f ∈ kb(C) \ root` is
*unconditional w.r.t. C* iff its provenance chain (transitive
walk of `provenance.premises_raw`) never hits any
`h_id ∈ frozenset(C)`. Equivalently — the derivation is valid
in `(root, rules)` alone. The walker terminates at non-rule
provenances (raw facts, hypothesis facts, forced-positive
promotions). See [`commitment._reaches_commitment`](../../../ein.py/src/ein_bot/inference/commitment.py).

Note (user, 2026-05-25): under M1 all hypotheses are positive
facts; negative hypotheses are a follow-up. The pre-sat
contradiction filter still applies but its yield is small —
most deaths surface only after saturation.

#### 3b. (Opt-in) State-hash dedup checkpoint

**Only when** `store_lattice=True` **and** the caller is
`contradictions_solve` (NOT `gaps_solve`, NOT
`monotonic_solve`). Under `gaps_solve`, two distinct
satisfying commitments must register separately even when
their saturated kbs collide — the merge is incorrect there
(the GAPS contract is "enumerate every satisfying commitment
set"). Under `monotonic_solve`, `store_lattice` doesn't
exist and there's no `kb_index` to look into.

```
h ← state_hash(result.kb)
if h ∈ proof.kb_index:
    E ← proof.kb_index[h]                # existing SetNode
    E.labels += (C,)                     # multilabel grow
    record audit row marking C → merged_into = E
    SKIP downstream — everything already recorded on E
    advance to next candidate
else:
    proof.kb_index[h] ← SetNode(canonical_set=C, …)
```

Even under `gaps_solve` with `store_lattice=True`, the
`SetNode` is created and added to `kb_index` but the merge
step is skipped — each commitment gets its own SetNode
record so distinct satisfying commitments stay distinct.

Cost: one `state_hash(fork)` (O(|fork.facts|)) per arrival;
one dict-by-int lookup. Free relative to the saturation we
just paid. Hash dedup pays off when:

1. The puzzle has frequent cross-set kb-convergence
   (`examples/zebra2-hints.ein` is a candidate fixture per
   [Q1.5b.4.c](open_questions.md#q15b4--set-equivalence-dedup--state-hash-dedup));
2. Downstream processing (per-set storage + dumper writes)
   is non-trivial — i.e., `store_lattice=True`.

Empirical evaluation lives in S1.5b.28 (fixtures) +
S1.5b.30 (perf round).

#### 3c. Match on `result.kind`

##### 3c.i. DEAD (`kind ∈ {"dead-pre", "dead-post"}`)

Flat root-write — no per-parent bubble. Saturation
commutativity makes "bubble through all parents" idempotent
(every ancestor would receive the same clause, all eventually
reaching root), so we write to root directly.

```
emit_nogood(root, frozenset(C), min_size=1)
    # Subsumption-aware (nogoods.py) — drops the clause if a
    # strict subset is already in _nogoods; removes existing
    # superset clauses; otherwise inserts.

if |C| == 1:
    # Single-element death → also write (not h) into
    # root._negated_facts so the next compute_alive() prunes h.
    # If h's relation is (symmetric R), write the mirror too.
    write_negated_fact(root, C[0])

# Entry-specific collection:
if caller is contradictions_solve:
    proof.dead_commitments.append(DeadCommitment(
        commitment=C, unsat_core=result.unsat_core,
        learned_clause=frozenset(C), layer=N, kind=result.kind,
    ))
# else (monotonic_solve, gaps_solve): nothing further —
# the root._nogoods clause is sufficient for downstream pruning.
```

The unsat_core is the witness facts the detector returned
(pre-sat: the new hypothesis + the existing facts that
contradict it; post-sat: the conflict pair or `(false)`
witness with its provenance chain).

Why doesn't `gaps_solve` collect dead commitments? Because
the GAPS contract is "give me every satisfying commitment"
— the refutation map isn't part of that contract. The
`_nogoods` clause is sufficient for the engine's internal
pruning. If the caller wants both the solution set AND the
refutation map for the same puzzle, they call BOTH
`gaps_solve` and `contradictions_solve`; each entry's cost
profile is honest about what it returns.

##### 3c.ii. ALIVE + `is_solved(result.kb)`

The fork's saturated kb satisfies the goal. Behaviour is
entry-specific:

```
# monotonic_solve:
if caller is monotonic_solve:
    return Solution(kb=result.kb)        # early-terminate

# gaps_solve:
if caller is gaps_solve:
    proof.solutions.append(SolutionRecord(
        commitment=C, kb=result.kb.snapshot(),
        firings=result.firings, layer=N,
    ))
    # No early-terminate — continue to find more solutions.

# contradictions_solve:
if caller is contradictions_solve:
    # A satisfying commitment is not a refutation, so we
    # don't record it into dead_commitments. We also don't
    # build proof.solutions (not part of the contract).
    # Just continue.
    pass
```

**Why snapshot only solutions.** The explanation phase (P1.6)
walks the SOLUTION node's `derivation_dag` to chain facts
to goal; it never walks dead or non-satisfying-alive nodes.
So those don't need persisted kbs. Even with
`store_lattice=True`, the per-SetNode stored kb (in
`proof.kb_index`) is a snapshot reference shared with the
SolutionRecord — not a duplicate.

##### 3c.iii. ALIVE + ¬`is_solved`

Flat root-merge of the unconditional facts, then the standard
re-saturate / recompute-alive / forced-positive cascade.
Same on all three entries.

```
for f ∈ result.unconditional_facts:
    if root._fact_by_id(f.relation_name, f.args) is None:
        root.add_fact(f); root._index_fact(f)
        stats.facts_merged += 1

if any facts merged:
    Saturator(root).saturate()
    if ContradictionDetector(root).detect():
        return Contradiction(unsat_core=…)
    alive ← compute_alive(root)
    # Forced-positive cascade: if alive shrinks to {h_unique},
    # promote h_unique to a root fact with
    # Provenance.from_rule("<forced-positive>", premises_raw=())
    # so the chain walker treats it as a terminal. Re-saturate.
    # Repeat until |alive| ≠ 1.
    alive ← promote_forced_positives(root, alive)
    if is_solved(root):
        # Same entry-specific dispatch as §3c.ii but with the
        # solution kb = root snapshot (the cascade landed it
        # at root, not at the fork).
        if caller is monotonic_solve:
            return Solution(kb=root)
        elif caller is gaps_solve:
            proof.solutions.append(SolutionRecord(
                commitment=C,         # the breakthrough commitment
                kb=root.snapshot(),
                firings=(…),
                layer=N,
            ))
        # contradictions_solve: same as §3c.ii (no record)
```

**Why this is sound: per-parent `is_unconditional_at` is
unnecessary.** If `f`'s chain at level ``N`` doesn't touch any
`h ∈ C`, then by the saturation-commutativity argument
(`sat(root) ⊆ kb(P_i)` for any `P_i ⊆ C`) the same chain is
valid at every ancestor — including root. Earlier drafts
checked `is_unconditional_at(P_i)` per parent; that predicate
is always True given a correct chain walk. Flat root-merge
suffices.

**Why `gaps_solve` records the carrier ``C`` even when the
goal fires at root.** The cascade that landed the goal at
root was triggered by C's unconditional facts; for the
explanation walk, ``C`` is the "breakthrough commitment" —
the set whose exploration produced the unconditional
consequences that closed the proof. P1.6 needs this to
render the "suppose C — derive the consequences — observe
the goal closes" narrative.

#### 3d. Layer wrap-up

When the queue is exhausted:

- `A_N` is populated with the alive size-`N` commitments
  visited (for next-layer prefix-join).
- `root._nogoods` carries every nogood emitted this layer.
- `root.facts` carries every unconditional fact merged this
  layer.
- `alive` reflects every back-propagated negative
  (single-element nogoods from layer-1 deaths + any forced
  negatives propagated up).
- `proof.solutions` carries every satisfying commitment
  (under SOLVE-exhaustive / GAPS / CONTRADICTIONS).
- `proof.dead_commitments` carries every dead set with
  unsat_core (under CONTRADICTIONS only).
- The orchestrator advances to layer `N+1` (if budget remains
  and SOLVE-early-terminate didn't return).

If `A_N` is empty AND no solution was found AND `alive` is
empty → **search complete with no remaining frontier**;
final verdict (Q1.5b.10) is derived from the cumulative state.

## Verdict synthesis (Phase 3)

After the layer loop ends without early termination:

### `monotonic_solve`

| state at termination                            | verdict                                  |
|-------------------------------------------------|------------------------------------------|
| `is_solved(root)` true (cascade fired earlier)  | `Solution(kb=root)`                      |
| `alive` empty                                   | `Contradiction(unsat_core=…)`            |
| depth cap reached with non-empty `alive`         | `Ambiguity(branches=…, unresolved=A_N)`  |

(No `LatticeProof` attached — `monotonic_solve` doesn't
build one. Early-terminate from §3c.ii has already returned
in the happy path.)

### `gaps_solve`

Always returns `Ambiguity`:

```
Ambiguity(
    branches    = tuple(Solution(kb=s.kb) for s in proof.solutions),
    unresolved  = A_N,    # surviving alive sets at depth cap
    tree        = None,
    proof       = proof   # carries solutions, kb_index (if store_lattice), learned_nogoods
)
```

Caller interprets:
- `len(branches) == 0` → no solution found within depth cap
  (puzzle is unsolvable OR depth cap too low).
- `len(branches) == 1` → uniquely solvable.
- `len(branches) > 1` → genuine multi-solution.

### `contradictions_solve`

Always returns `Contradiction`:

```
Contradiction(
    unsat_core      = frozenset().union(
                          *(d.unsat_core for d in proof.dead_commitments)
                      ),
    proof           = proof   # carries dead_commitments, kb_index, learned_nogoods
)
```

Caller interprets:
- `len(proof.dead_commitments) == 0` → no deaths observed
  (degenerate case; might still be solvable).
- Non-empty → refutation map; `unsat_core` shows the
  facts implicated.

`LatticeProof` (S1.5b.22) carries `proof.solutions` (filled
by `gaps_solve` only), `proof.dead_commitments` (filled by
`contradictions_solve` only), `proof.kb_index` (filled when
`store_lattice=True` for either entry), `proof.alive_at_end`
= A_N, and `proof.learned_nogoods` = snapshot of
`root._nogoods` at termination.

## What this algorithm no longer does

Dropped relative to earlier (multi-engine) drafts:

- **`find_parents` / multi-parent integrate**
  ([Q1.5b.4.b](open_questions.md#q15b4--set-equivalence-dedup--state-hash-dedup))
  — under monotone saturation the parent bubble for nogoods
  is N-way idempotent and the bubble for unconditional facts
  is always-True. Flat root-writes are equivalent and cheaper.
  Killed stages: S1.5b.24 (back-prop integrate), S1.5b.25
  (per-set forced-positive mining).
- **`is_unconditional_at(P_i)` per parent** — by saturation
  commutativity, unconditional-at-C ⟹ unconditional-at-every-P_i.
  Defensive code with no semantic effect.
- **Per-set kb_snapshot for ALIVE intermediates** — never
  read except by the (dropped) multi-parent integrate. Only
  SOLUTION nodes need persisted kbs (for P1.6's
  `derivation_dag` walk).
- **Sanity check (§3b in the old draft)** — kept as an
  optional dev/regression tool (S1.5b.27) but not a runtime
  step. It's a one-time validation of the saturation
  commutativity premise; if it ever fails, the M1 ruleset
  has lost its monotone property and the entire engine
  collapses.

What stays:

- Apriori prefix-join + the two filters (subset + nogood + alive).
- `try_commitment_set(root, C)` primitive — unchanged.
- Flat `emit_nogood(root, frozenset(C))` writeback.
- Flat unconditional-fact merge into root + re-saturate +
  recompute alive + forced-positive cascade.
- Optional state-hash dedup as a perf flag (off by default,
  forced off under GAPS).
- Per-set audit dumper (mode-aware, S1.5b.23).

## Cross-references

- Engine entry: [`monotonic/solver.py`](../../../ein.py/src/ein_bot/inference/monotonic/solver.py)
  (today's name; rename to `inference/solver.py` is queued
  end-of-phase).
- Filter helpers:
  [`nogoods.py`](../../../ein.py/src/ein_bot/inference/nogoods.py)
  (`matches_any_nogood`, `emit_nogood`).
- Commitment primitive:
  [`commitment.py`](../../../ein.py/src/ein_bot/inference/commitment.py)
  (`try_commitment_set`, `_reaches_commitment`).
- State hash: [`canon.py`](../../../ein.py/src/ein_bot/inference/canon.py).
- Saturation commutativity premise:
  [README § Motivation](README.md#motivation) +
  [`project-set-search-unified` memory].
- Mode + verdict trichotomy:
  [Q1.5b.7](open_questions.md#q15b7--termination--completeness--mode-handling).
- Per-set storage policy (post-merge):
  [S1.5b.21 LatticeProof](s1.5b.21_lattice_backbone.md).
- Killed: [S1.5b.24 — multi-parent integrate](s1.5b.24_lattice_integrate.md),
  [S1.5b.25 — per-set forced positives](s1.5b.25_lattice_forced_positives.md).
