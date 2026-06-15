# P1.5b — Open questions

Phase-scoped — discuss + decide before drafting the per-stage
plan. Once a question resolves, mark its row with the
resolution + the dated note so the stage-plan reviewer can
reconstruct the rationale.

## Index

| Q       | Title                                                                                     | Status         |
|---------|-------------------------------------------------------------------------------------------|----------------|
| Q1.5b.1 | Co-existence layout — folder slicing + entry-script duplication                            | ✅ resolved 2026-05-25 |
| Q1.5b.2 | Frontier strategy — BFS variants + alive-set refresh cadence                               | ✅ resolved 2026-05-25 |
| Q1.5b.3 | Lattice node representation — canonical tuple vs frozenset                                 | ✅ resolved 2026-05-25 — canonical tuple |
| Q1.5b.4 | Set-equivalence dedup × state-hash dedup — which is primary, how do they interact?        | ✅ resolved 2026-05-25 — merge + multilabel + multi-parent |
| Q1.5b.5 | Refutation semantics — unify with S1.5a.18 nogoods, or distinct mechanic?                 | ✅ resolved 2026-05-25 — unified |
| Q1.5b.6 | Reasoning-path post-solve phase — P1.5b or P1.6?                                          | ✅ resolved 2026-05-25 — both phases own a piece |
| Q1.5b.7 | Termination + completeness + mode handling — when does each engine stop? what trichotomy? | ✅ resolved 2026-05-25 — monotonic terminates on first goal-sat; lattice exhaustive to `max_set_size` (Q1.5b.10 merged in) |
| Q1.5b.8 | Engine bridge — commitment-set `try_commitment_set` vs incremental `try_step` (S1.5a.20 primitive reuse)  | ✅ resolved 2026-05-25 — copy-on-modify; bootstrap → backbone → features |
| Q1.5b.9 | Scoring within a layer — defer or include in initial design?                              | ✅ resolved 2026-05-25 |
| Q1.5b.10 | *(merged into Q1.5b.7)* Mode handling — SOLVE vs GAPS vs CONTRADICTIONS                  | ✅ merged 2026-05-25 |

---

## Q1.5b.1 — Co-existence layout — ✅ resolved 2026-05-25

### Resolution

```
ein.py/src/ein/inference/
   ├── *.py            ← common (Saturator, ContradictionDetector,
   │                     Lookahead, hypgen, kb-store helpers, base
   │                     dumper utilities)
   ├── tree/           ← current tree-search; reference until parity
   └── lattice/            ← new lattice-search

ein.py/demo/
   ├── bench_solve.py  ← unchanged, tree entry point
   └── bench_dag.py    ← NEW, lattice entry point

ein.py/tests/inference/
   ├── *.py            ← common-kernel tests
   ├── tree/           ← tree-search tests
   └── lattice/            ← lattice-search tests
```

Explicit selection per entry script; **no default flag** that
picks one or the other; tests reach the chosen path by import.

**Q1.5b.1.a — copy-on-modify rule (user direction 2026-05-25):**
> *"if need to modify file for dag - copy, if not needed leave"*

No upfront file inventory needed. A module starts in
`inference/` (common); when DAG implementation needs to modify
it in a way incompatible with tree, **copy** it to
`inference/lattice/` and let the copy diverge. Tree's copy stays
under `inference/tree/` (also a copy if a tree-only change is
made). The rule reads each module's location as a
de-facto answer to "did this need search-specific changes?".

Stage S1.5b.1's task is the **initial split** — files known to
diverge from day one (the current `solver.py`, `back_prop.py`,
likely `nogoods.py`) get moved into `inference/tree/`; the rest
stay in `inference/`. The lattice/ folder starts mostly empty and
fills as lattice-specific code lands.

**Q1.5b.1.b — split-mirror tests** (user direction 2026-05-25,
*"yes, split mirror"*). Same copy-on-modify rule applies to
tests.

---

## Q1.5b.2 — Frontier strategy

**Status:** ✅ resolved 2026-05-25.

### User's proposal (BFS by size)

```
Layer 1: try each {h_i} for h_i ∈ alive_root
         → contradictions back-prop ¬h_i to root + invalidate supersets
         → re-saturate root, recompute alive
A_1   := { {h_i} that survived (no contradiction) }
D_1   := { {h_i} that died at layer 1 — root now carries ¬h_i }

Layer 2: enumerate { {h_i, h_j} : h_i, h_j ∈ alive_root_after_layer_1,
                                   {h_i} ∈ A_1, {h_j} ∈ A_1, i ≠ j }
         → filter the cartesian by current KB (drop pairs containing any
           hypothesis the post-layer-1 root has since negated)
         → try each surviving pair as a set commitment
A_2   := surviving pairs
D_2   := pairs that died as a pair (contributing 2-element nogoods)

Layer k: enumerate { S : |S| = k, ∀(k−1)-subset T of S, T ∈ A_{k−1},
                                  ∀ 1-subset {h} of S, {h} ∈ A_1 }
         → filter by current KB
         → try
```

**Question:** is this Apriori-style enumeration the right
default? Three sub-decisions:

**Q1.5b.2.a — Re-saturation + alive-recompute cadence.**
✅ resolved 2026-05-25 — Option A (user direction
*"after every integration - after every hyp set processing;
we also need to re-calculate every level alive set to prune
lattice search space correctly"*).

Cadence:
1. Each entering's `integrate(parent_kb, child_result)` lands.
2. Parent re-saturates immediately.
3. Alive-set is re-computed (against the post-resat
   `_negated_facts` + the newly derived positives).
4. The lattice's pending frontier (size-k candidates being
   enumerated) is re-filtered against the new alive.

This is the most reactive cadence and matches the NL trace
shape (each refutation injects new knowledge that the next
refutation immediately uses).

**Q1.5b.2.b — Generating a full + valid layer-k set from the
A_1..A_{k-1} layers.**
✅ resolved 2026-05-25 — user direction
*"implement a textbook apriori-gen, defer optimization to the
end of phase"*.

Initial implementation: **lexicographic prefix-join** (textbook
Apriori-gen):

1. Sort each set in ``A_{k-1}`` in canonical order
   (post-Q1.5b.3, sorted tuples).
2. For each prefix-class — sets that agree on their first
   ``k-2`` elements — pair every two sets
   ``S = (a_1, …, a_{k-2}, b)`` and
   ``T = (a_1, …, a_{k-2}, c)`` with ``b < c``.
3. Emit ``S ∪ T = (a_1, …, a_{k-2}, b, c)`` once per pair.

Each candidate is then filtered:
- Every ``(k-1)``-subset must be in ``A_{k-1}`` (the Apriori
  invariant — covered by construction when prefix-join is used
  on a closed family).
- No clause in ``root_kb._nogoods`` is a subset of the
  candidate (Q1.5b.2.b' — covers conditional deaths whose
  clauses propagated up from deeper layers; uses
  [`matches_any_nogood`](../../../ein.py/src/ein/inference/nogoods.py)).
- Every element is still in the current alive set (covers
  back-propagated single-element negatives written after
  layer-(k-1) closed).

**Deferred optimisations** (subset-trie lookup, interned set
ids, etc.) — parked for the **end-of-phase** perf round. The
initial implementation runs the naive prefix-join + the two
filters; performance is measured after parity is reached.

**Q1.5b.2.b' — Subsumption by existing nogoods at generation
time** (user 2026-05-25):
> *"for given hyp set (and h1 h2 h3) if it is already refuted
> by (not (and h3 h2)) in KB, while individual h1, h2 and h3 are
> still alive we may produce (and h1 h2 h3) with some alive hyp
> set generator — do we still have to add (not (and h1 h2 h3))
> to KB and what for?"*

**Answer:** no. ``(not (and h1 h2 h3))`` is **strictly
subsumed** by ``(not (and h2 h3))`` — every model that
satisfies the 2-clause also satisfies the 3-clause; adding the
3-clause to the KB is a no-op (S1.5a.18's
[`emit_nogood`](../../../ein.py/src/ein/inference/nogoods.py)
already rejects subsumed clauses on emit). The right thing is
to **filter the candidate set at generation time** — drop
``{h1, h2, h3}`` from layer 3 because ``{h2, h3} ∈ D_2``
already kills every superset.

This is **Apriori pruning** applied through the
no-good index, not through ``A_{k-1}`` membership: the
sufficient condition is *"no clause in
``root_kb._nogoods`` is a subset of the candidate set"*. Layer
generation should run BOTH checks:

1. All ``(k−1)``-subsets are in ``A_{k−1}`` (no dead subset).
2. No proper subset matches a clause in ``root_kb._nogoods``
   (covers conditional deaths from deeper layers whose clauses
   propagated up via integrate).

Check (1) is the Apriori standard. Check (2) catches the
S1.5a.18-style multi-element deaths that don't appear as
``A_{k−1}`` membership failures (because their constituent
``(k−1)``-subsets are individually alive).

**So the rule is:** generate ``A_k`` candidates from
``A_{k−1}`` × ``A_1`` (or via lexicographic prefix-join);
filter by `matches_any_nogood(candidate, root_kb._nogoods)`
([`nogoods.py`](../../../ein.py/src/ein/inference/nogoods.py));
filter by current alive (each element still alive);
deduplicate. The result is the **full and valid** layer-k
candidate set.

**Q1.5b.2.c — Within-layer ordering.**
✅ resolved 2026-05-25 — user direction
*"for now let's have a switch between lexicographic and
scoring, now implement default sum of elements scores"*.

Stage shape:
- `SolverConfig.lattice_order: Literal["lex", "score-sum"]`
  (default `"score-sum"`).
- `"lex"`: canonical-tuple order — deterministic, uninformed.
  Useful for regression tests + invariance checks.
- `"score-sum"`: per-set score = sum of
  `score_hypothesis(h, kb)` for each element — reuses
  [S1.5a.7](../p1.5a_zebra_solution/s1.5a.7_hypgen_scoring_branch_info.md)'s
  per-element scoring. Default.

Partial-layer exploration / best-first / cross-layer is a
**follow-up phase inside P1.5b** — *"Phase within the Phase"*
in the user's words. Not part of the initial implementation.

**Q1.5b.2.d — Apriori example fixtures.**
✅ resolved 2026-05-25 — not a question; user direction
*"yes add example ein files"*.

A stage task creates:
- ``examples/lattice/01_subset_pruned.ein`` — fixture where a
  3-element commitment is pruned because a 2-subset is in
  ``D_2``. The dump must show **zero** layer-3 try_sets on the
  pruned candidate.
- ``examples/lattice/02_genuine_3set_death.ein`` — fixture
  where a 3-element commitment dies but **no** 2-subset of it
  is in ``D_2``. The dump must show the layer-3 try_commitment_set
  running, the 3-element clause emitted, and the corresponding
  ``D_3`` entry recorded.

These fixtures pin the Apriori contract — both that pruning
works *when applicable* and that genuine deeper-only deaths
are still discovered.

### Why we don't need to ask whether BFS matches NL pattern

User direction 2026-05-25:
> *"It doesn't matter now. As I already said our tree search
> splits into 2 independent steps: solution and explanation.
> Solution is a proof — is a final resulting KB + full DAG
> state + provenances of inferred facts. Explanation is one
> path through the DAG via hyps that remain alive afterall."*

NL-trace alignment is an **explanation-phase** concern, not a
**search-phase** concern. The lattice's search produces the
proof artefact (final kb + DAG + provenances); the explanation
phase walks the artefact to recover an NL-shaped narrative.
What order the search visited sets in is bookkeeping —
discarded once the artefact is built.

See [Q1.5b.6](#q15b6--reasoning-path-post-solve-phase) for the
artefact contract.

---

## Q1.5b.3 — Lattice node representation

**Status:** ✅ resolved 2026-05-25 — canonical-ordered tuple.

User direction *"canonical-ordered tuple. all optimizations
here doesn't make sense in M1"*.

```python
CanonicalSetId = tuple[FactId, ...]
# Always sorted by (rn, args) — sort key is the same as the
# `_path_ctx` element shape, so the existing nogood/path-cond
# helpers compose without conversion.
```

Interned-id space optimisation + frozenset alternatives parked
as out-of-M1-scope.

---

## Q1.5b.4 — Set-equivalence dedup × state-hash dedup

**Status:** ✅ resolved 2026-05-25 — merge with multilabel +
multi-parent.

### Resolution

**Q1.5b.4.a — When ``saturate(root ∪ C_2)`` collides with a
previously-visited ``C_1``: option (ii) MERGE** (user
direction 2026-05-25):

> *"merge, while we preserve path - we have enough information,
> generated search lattice is not a real existing lattice which
> is defined by rules and conditions and which we constructing
> by dedup"*

Conceptual model — two distinct lattices:

- The **true semantic lattice** is the abstract object defined
  by the rule set + the puzzle's conditions: every kb-state
  reachable from root + every commitment that produces it,
  related by set-inclusion. We don't materialise this; we just
  *navigate* its shape.
- The **search lattice** is what we build by dedup — a
  finite-state approximation of the true lattice. Each
  `SetNode` is a piece of evidence we've observed, not a
  fixed point of the underlying mathematical structure. Merge
  is the natural way the search lattice converges on the
  true one.

When merge happens, the merged `SetNode` keeps BOTH commitment
sets as **labels** — see Q1.5b.4.b below.

**Q1.5b.4.b — Subtree result sharing → reframed: multi-parent
+ multilabel storage** (user 2026-05-25):

> *"with BFS we don't have deeper exploration links before
> finish layer, so the question is opposite - how we find
> parents for new generated sets in layer N+1? Maybe worth
> store all hyp sets in merged node - multilabel"*

Under BFS-by-size + Option A cadence, the lattice DAG is built
**bottom-up**. By the time layer ``k+1`` is enumerated, layer
``k`` is fully closed. So the right question is not "how do we
find the subtree for an already-explored node" but **"how does
a new layer-``k+1`` set find its parents in layer ``k``?"**

Each ``S ∈ A_k`` has ``k`` natural parents — the ``k`` distinct
``(k-1)``-subsets of ``S``, each of which is in ``A_{k-1}``
(per the Apriori invariant). Multi-parent is intrinsic; we
store the parent list explicitly.

Multilabel composes naturally: when state-hash dedup merges
two distinct sets into one `SetNode`, the node's `labels`
list grows; its `parents` list becomes the union of parents of
every label.

User direction *"save everything modulo dedup — anyway will
have to dump everything for debug"* — the data model is:

```python
@dataclass(frozen=True)
class SetNode:
    # Primary key for the search-lattice index — the canonical
    # "first arrival" set that landed here.
    canonical_set:  CanonicalSetId

    # All commitment sets that collapsed into this node via
    # state-hash dedup. labels[0] == canonical_set; subsequent
    # entries are arrival sets that merged in. Same kb_snapshot,
    # same verdict, different "logical" commitments.
    labels:         tuple[CanonicalSetId, ...]

    # All (k-1)-subsets across all labels that are parents in
    # the lattice DAG. Multi-parent intrinsic from Apriori-gen
    # (k parents per set); multi-label expands the union further
    # when sets merge.
    parents:        tuple[CanonicalSetId, ...]

    # Cached commitment-saturation outcome — shared across
    # labels (state-hash dedup guarantees the kb is identical).
    kb_snapshot:    KnowledgeBase
    firings:        tuple[Firing, ...]
    verdict:        Literal["alive", "dead", "open", "solution"]
    unsat_core:     frozenset[Fact]                 # if dead

    # Bubble-up payload (mirrors the S1.5a.20 BranchResult
    # fields the lattice integrate step reads).
    proposed_negatives:  tuple[Fact, ...]
    forced_positives:    tuple[Fact, ...]
    learned_nogoods:     tuple[frozenset[FactId], ...]
```

**For now, while the problem is small** (user direction):
- Store every label, every parent — no info loss.
- Dump everything per the per-set dump from S1.5a.20 T1.5a.20.6
  (each label gets its own `enterings/<label-slug>/` if helpful
  for debug; subtree result is per-node).

The "true lattice vs search lattice" distinction is recorded
in the README as a permanent design note — keeps future
readers from being confused when the dump shows a single node
with two distinct logical commitments.

**Q1.5b.4.c — Counter-example check** (user 2026-05-25):

> *"yes examples/zebra2-hints.ein must produce those cases,
> not 100% sure"*

Stage task in P1.5b: run `bench_dag examples/zebra2-hints.ein`
once the lattice search is runnable; inspect the dump for at
least one state-hash collision between distinct commitment
sets (= a `SetNode` with `len(labels) > 1`). If found, pin as
the merge-mechanism regression fixture. If not found,
construct a minimal one (a 3-hypothesis puzzle where
``{h_1, h_2}`` and ``{h_2, h_3}`` saturate to the same kb,
e.g., because `h_3` is derivable from `h_1` given some rule).

---

## Q1.5b.5 — Refutation semantics

**Status:** ✅ resolved 2026-05-25 — unified CDCL mechanic
shared between tree and DAG searches.

### Resolution

**Q1.5b.5.a — Unified.** User direction
*"is CDCL applicable for both Tree and DAG search? If yes —
then reuse"*.

Answer: yes, applicable to both. A no-good clause in either
search is the same shape — a `frozenset[FactId]` representing
the dead conjunction — and the same matching predicate applies:
`clause ⊆ candidate_set` ⟹ kill the candidate.

Tree-side semantics: clause = path condition (the chain of
ancestor hypotheses ending in the dying branch).
DAG-side semantics: clause = the dying commitment set itself.
Both render the same conjunction-as-frozenset; the storage
type + matching helpers are identical.

**Concrete reuse:**

- ``ein.py/src/ein/inference/nogoods.py`` — stays in the
  **common** folder. Provides `emit_nogood(target_kb, clause)`
  (subsumption-aware insert) + `matches_any_nogood(candidate,
  path_set, nogoods)`. Tree's ``solver.py`` and DAG's
  ``solver.py`` both import.
- ``KnowledgeBase._nogoods: set[frozenset[FactId]]`` — same
  attribute on the kb store; under S1.5a.20's isolation it
  becomes per-frame (with bubble-up integration), but the
  attribute itself stays in the common `kb/store.py`.

**Q1.5b.5.b — Subsumption: same policy.** User direction
*"yes same policy"*. When a strict subset clause arrives,
remove the superset. Identical to S1.5a.18's
[`emit_nogood`](../../../ein.py/src/ein/inference/nogoods.py).

**Q1.5b.5.c — Size-1 clauses: no special case.** User
direction *"no need for special case handling, what for?"*.

S1.5a.18's tree-side size-1 guard existed because `(not h)`
back-prop writes covered the same case and the guard avoided
double-cost subsumption checks. In the unified mechanic the
size-1 clause IS `frozenset({h})` — same artefact, no
duplicate write to dodge. Drop the guard for DAG; tree side
keeps it only as a perf detail (no semantic change).

---

## Q1.5b.6 — Reasoning-path post-solve phase

**Status:** ✅ resolved 2026-05-25 — both phases own a piece.
P1.5b owns the **proof artefact** (the *solution* in the
user's wording); P1.6 owns the **explanation walk** over it.

### Resolution (user direction 2026-05-25)

> *"our tree search splits into 2 independent steps: solution
> and explanation. Solution is a proof — is a final resulting
> KB + full DAG state + provenances of inferred facts.
> Explanation is one path through the DAG via hyps that remain
> alive afterall."*

### P1.5b's side — the proof artefact (the "solution")

P1.5b's search produces, at termination, a self-contained
proof artefact:

```python
@dataclass(frozen=True)
class LatticeProof:
    # The saturated kb at the satisfying commitment set —
    # carries every derived fact + its provenance.
    final_kb:           KnowledgeBase

    # The DAG of explored commitment sets — each node carries
    # its commitment, the post-saturation kb snapshot (or a
    # reference into final_kb when state-hash-deduped to the
    # solution set), and its verdict.
    lattice:            Lattice

    # The alive hypothesis subset at termination — the
    # surviving commitment(s) whose presence the explanation
    # phase can chain through.
    alive_at_end:       tuple[FactId, ...]

    # The dead-set artefact — every refuted commitment +
    # its unsat-core. Feeds the explanation phase's refutation
    # narrative (Q1.5b.6.b).
    dead_commitments:   tuple[DeadCommitment, ...]
```

P1.5b's acceptance test (item 5 in the phase README) checks
that `LatticeProof` is produced for the zebra2 satisfying set
in the shape P1.6's NL renderer can consume.

### P1.6's side — the explanation walk

P1.6 reads `LatticeProof` and produces an NL-shaped narrative
by walking the derivation DAG. The walk picks ONE path
through the alive hypotheses (`alive_at_end`) and chains the
goal's provenance back via `final_kb.derivation_dag`. The
"max-information / non-overlap" greedy from the user's
sketch is P1.6's algorithm choice — not P1.5b's.

P1.6 also reads `dead_commitments` to inject the "suppose X
— then contradiction — therefore ¬X" refutation steps the NL
trace expects (idea-08 trace fidelity).

### What this means for stages

- P1.5b's stages stop at producing + dumping `LatticeProof`.
- P1.6 grows a stage (S1.6.5 or follow-on) that reads
  `LatticeProof` + emits the NL trace.
- The boundary is clean: P1.5b never imports P1.6; P1.6 only
  reads the data class.

---

## Q1.5b.7 — Termination + completeness + mode handling

**Status:** ✅ resolved 2026-05-25, refined 2026-05-25b —
**two engines, two termination criteria**.

### Resolution (refined 2026-05-25b)

The original resolution covered the lattice engine only. After
the user introduced the monotonic engine as the simpler SOLVE
variant (sharing P1.5b's scope), the two engines have **distinct
termination + verdict semantics**:

| engine     | termination                                                                         | scope                                                              |
|------------|-------------------------------------------------------------------------------------|--------------------------------------------------------------------|
| monotonic  | **first goal-satisfaction** at root, OR layer exhaustion, OR `max_set_size` cap     | SOLVE-mode only; "converges to first met solution if any"           |
| lattice    | **exhaustive** to `max_set_size` (no early termination)                              | SOLVE / GAPS / CONTRADICTIONS; "converges to full map of solutions" |

User direction (original 2026-05-25):
> *"For M1 we explore all nodes up to max depth - and make
> available conclusions: solved, ambiguity or contradiction in
> root. Ambiguity == multiple solutions."*

User direction (refining 2026-05-25b):
> *"monotonic converges to first met solution if any, lattice
> converges to full map of all solutions"*

### Monotonic — termination + verdict

**Termination conditions** (evaluated after each entering's
integrate-to-root + re-saturation):

1. `is_solved(root.kb, SOLVE)` → terminate with `Solution(root.kb)`.
2. `current_alive` empty AND no more layers to enumerate →
   terminate with `Contradiction` (no commitment can satisfy).
3. `max_set_size` reached without (1) → terminate with `Ambiguity`
   (root has accumulated some unconditional facts but doesn't
   fully satisfy the goal; the unresolved part requires
   commitments that would have been explored at greater
   depth).

Verdict is determined by which termination condition fires.
No GAPS / CONTRADICTIONS modes — monotonic supports SOLVE
only because the architecture can't enumerate multiple
solutions or per-dead-set unsat-cores without storage.

### Lattice — termination + verdict

**Always runs until layer `max_set_size` is exhausted** (or
`current_alive` shrinks to less than the next layer's size,
naturally one layer earlier). No early termination — the goal
is the **complete map of all satisfying commitments**.

Verdict trichotomy computed at end-of-search from the
accumulated `A_*` / `D_*` frontier + per-set `is_solved`
checks (§3d.vii of [`algorithm_layer_n.md`](algorithm_layer_n.md)):

| verdict | criterion |
|---|---|
| **Solved** | exactly **one** SetNode satisfies the goal and no alive sets remain unexplored above layer `max_set_size` |
| **Ambiguity** | multiple SetNodes satisfy the goal **OR** at least one SetNode is alive at layer `max_set_size` without satisfying |
| **Contradiction** | zero SetNodes satisfy the goal; every commitment died or merged into a dead node |

Same trichotomy as tree's `Mode.SOLVE`. Lattice additionally
supports GAPS (always Ambiguity with every satisfying SetNode
listed) and CONTRADICTIONS (unsat_core = ⋃ of dead-SetNode
unsat-cores + the learned no-good clauses).

### Sub-questions previously raised — closed by the above

- **Q1.5b.7.a (one solution or all)** — distinct answers per
  engine: monotonic returns the first; lattice returns all.
- **Q1.5b.7.b (maximal set definition)** — applies to lattice
  only; derived post-hoc from the explored frontier.
- **Q1.5b.7.c (equivalence to SOLVE-mode tree)** — both
  engines map cleanly. Monotonic's "first goal-sat at root"
  ≡ tree's `Solution` when the puzzle is uniquely solvable
  (the goal landing at root happens after the same set of
  refutations either engine would discover); lattice's
  exhaustive frontier ≡ tree's complete search tree.

### Mode handling (Q1.5b.10 merged 2026-05-25)

Tree side has `Mode.SOLVE` / `Mode.GAPS` /
`Mode.CONTRADICTIONS`; both new engines keep the same `Mode`
enum but **mode availability differs**:

| mode             | monotonic                                       | lattice                                                                                                              |
|------------------|-------------------------------------------------|----------------------------------------------------------------------------------------------------------------------|
| `SOLVE`          | the trichotomy above                            | the trichotomy above                                                                                                  |
| `GAPS`           | **not supported** (no enumeration)              | always `Ambiguity` with every satisfying SetNode in `branches`; depth-cap alive sets in `unresolved`                  |
| `CONTRADICTIONS` | **not supported** (no per-set unsat-core store) | always `Contradiction` with `unsat_core` = ⋃ over every dead SetNode's unsat-core, plus the learned no-good clauses    |

Mode is read at verdict-synthesis time, not during
exploration. This matches the tree-side convention.

### Out-of-M1 followups (parked)

- **Mode-specific search shapes.** Under lattice `GAPS`, skip
  state-hash dedup so distinct satisfying sets register
  separately. Under lattice `CONTRADICTIONS`, prune
  aggressively at first satisfying set.
- **Solution uniqueness modulo state-hash.** Under lattice's
  exhaustive search the trichotomy uses `is_solved(node.kb)`
  per SetNode — but state-hash-merged SetNodes carry one kb
  with multiple labels. Two distinct logical commitments
  collapsed to one kb count as ONE solution (Q1.5b.4.a).
  Re-audit if a `GAPS` use-case ever surfaces where the
  distinction matters.

Both are perf / completeness refinements on top of the M1
baseline; the baseline's verdict is always correct.

## Monotonic vs lattice equivalence

For SOLVE mode on **uniquely-solvable puzzles** (like Zebra):

**Claim.** Monotonic's `root.kb` at termination = the set of
unconditional facts that the lattice's `LatticeProof.final_kb`
would carry.

**Argument.** A puzzle is uniquely solvable iff every solution
fact is derivable from `root ∪ rules` (modulo the search
needed to surface refutations). Both engines:

1. Iterate commitment sets via Apriori-gen.
2. For each set `C`: if contradictory → emit `frozenset(C)` to
   `root._nogoods`; if alive → extract unconditional facts
   (chain doesn't touch `C`) and merge into root.
3. Continue until root has accumulated enough facts for
   `is_solved(root.kb)` to return True.

Both engines reach the same end-state on root; monotonic
**terminates** at that point, lattice **continues** to finish
the layer (but no further refinement of root happens because
all remaining sets are either dead, redundant, or yield only
conditional facts).

**Where monotonic loses information**:

- Multi-solution enumeration (lattice has multiple satisfying
  SetNodes; monotonic returns the first).
- Per-set audit / explanation phase (lattice ships
  `LatticeProof` for P1.6; monotonic ships only root with
  provenances).
- Puzzles where solution facts are genuinely conditional on
  a specific commitment (none of these in M1; Zebra is
  uniquely solvable from root alone).

### Mode handling (Q1.5b.10 merged 2026-05-25)

Tree side has `Mode.SOLVE` / `Mode.GAPS` / `Mode.CONTRADICTIONS`;
the lattice keeps the same `Mode` enum but **mode does not
change the search shape under M1** — all three run the same
exhaustive `max_set_size` search. Only the *verdict computation*
varies:

| mode             | from the same exhausted frontier, return …                                                                                                   |
|------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| `SOLVE`          | the trichotomy above (Solved / Ambiguity / Contradiction).                                                                                   |
| `GAPS`           | always `Ambiguity` with every satisfying SetNode in `branches`; depth-cap alive sets in `unresolved`.                                        |
| `CONTRADICTIONS` | always `Contradiction` with `unsat_core` = ⋃ over every dead SetNode's unsat-core, plus the learned no-good clauses for the explanation phase. |

The mode parameter is read at the END (during verdict
synthesis), not during exploration. This matches the
tree-side convention that the *search* is mode-agnostic and
the verdict is mode-shaped.

### Out-of-M1 followups (parked)

- **Early termination under SOLVE mode.** Stop at the first
  satisfying SetNode without exhausting later layers.
  Soundness rests on showing every unvisited set either
  contains a contradicted subset, hits state-hash dedup
  against an already-visited set, or is itself a separate
  solution (the third case is what would have to be ruled
  out — currently the exhaustive search does the ruling-out).
- **Mode-specific search shapes.** Under `GAPS`, skip
  state-hash dedup so distinct satisfying sets register
  separately (today's dedup may collapse two satisfying
  commitments). Under `CONTRADICTIONS`, prune the search
  aggressively as soon as a satisfying set is found (we
  only care about deaths).
- **Solution uniqueness modulo state-hash.** Under M1's
  exhaustive search the trichotomy uses `is_solved(node.kb)`
  per SetNode — but state-hash-merged SetNodes carry one kb
  with multiple labels. Two distinct logical commitments
  collapsed to one kb count as ONE solution (the user's
  "search lattice ≠ true semantic lattice" framing from
  Q1.5b.4.a). Re-audit if a `GAPS` use-case ever surfaces
  where the distinction matters.

All three are perf / completeness refinements on top of the
exhaustive M1 baseline; the baseline's verdict is always
correct.

---

## Q1.5b.8 — Engine bridge

**Status:** ✅ resolved 2026-05-25 — copy-on-modify principle
+ bootstrap-style implementation order.

### Resolution

User direction 2026-05-25:
> *"new engine in new folder inference/lattice; if reuse - this is
> common file and unchanges, if not reuse - write new one;
> implementation strategy - bootstrap: skeleton, then backbone
> implementation, then features: dedup, backprop, etc."*

The choice between **commitment-set** ``try_commitment_set`` and **incremental**
``try_branch`` reuse is not pre-decided. The same
copy-on-modify rule from Q1.5b.1.a applies to the engine
layer: start by reusing S1.5a.20's `try_branch` unchanged
(stays in common `inference/`); if DAG implementation needs
something `try_branch` doesn't provide, **copy** to
`inference/lattice/<file>.py` and let the copy diverge.

### Implementation order (bootstrap → backbone → features)

The phase ships in three implementation rounds, each its own
stage band:

**1. Bootstrap (skeleton).**
- `inference/lattice/` populated with stub modules: `__init__.py`,
  `solver.py`, `lattice.py`, `state_dump.py`. Each module
  carries enough type signatures + docstrings to compile, but
  the entry points raise `NotImplementedError`.
- `bench_dag.py` parses arguments + calls into the stub
  solver; returns a placeholder verdict.
- Tests under `tests/inference/lattice/` verify the imports + the
  CLI surface but skip with `pytest.skip("backbone")` for the
  real solve logic.
- **Acceptance:** `bench_dag examples/zebra2.ein` runs without
  ImportError; pytest collects new tests without failures.

**2. Backbone (minimal runnable lattice search).**
- `try_branch` reused from `inference/` unchanged (incremental
  ``try_branch`` invoked once per element of a candidate set
  during commitment construction). One saturation per element
  — the textbook cost. Accept it; perf optimisation comes
  later.
- Apriori prefix-join + filter-by-alive + filter-by-nogoods.
- Per-set fork + saturate + ContradictionDetector pre/post.
- Single linear orchestrator: for layer N = 1..max_set_size,
  enumerate candidates, run each through §3a–§3d of
  [`algorithm_layer_n.md`](algorithm_layer_n.md), record
  `A_N` / `D_N`, exit when exhausted.
- Verdict synthesis (Q1.5b.7 trichotomy) at end of search.
- **No** state-hash dedup yet (every candidate gets a fresh
  `SetNode`); **no** back-prop integrate (each death emits
  to a flat root-side `_nogoods` directly); **no** forced
  positives mining; **no** dumper per-set audit; **no**
  scoring (lexicographic order only).
- **Acceptance:** zebra2 produces some verdict (Solved /
  Ambiguity / Contradiction) within a generous time budget;
  every demo under `examples/branching/` produces the
  tree-side verdict at `max_set_size = max_depth_tree`.

**3. Features.**
Each feature is its own stage; landed in this rough order so
each builds on the previous:

- **F1.** State-hash dedup checkpoint (§3d.iii) + `SetNode`
  multilabel + multi-parent storage.
- **F2.** Per-set dumper (mirrors T1.5a.20.6 layout on the
  DAG side — `lattice/state_dump.py` writes `set/<canonical-slug>/`
  folders + `00_timeline.jsonl`).
- **F3.** Back-prop integrate (per-parent bubble of dead
  commitments' nogoods + alive commitments' forced positives;
  Option A cadence — re-saturate after each integrate;
  re-prune alive between candidates).
- **F4.** Forced-positive mining (`forced_positives` in
  `BranchResult`; integrate adopts at each parent's level
  with `is_unconditional_at` check).
- **F5.** Within-layer scoring switch (`lattice_order` =
  `"lex"` / `"score-sum"`; default `"score-sum"`).
- **F6.** Optional sanity check (`--lattice-sanity-check` on
  `bench_dag.py` — saturation commutativity audit per §3b).
- **F7.** Apriori example fixtures
  (`examples/lattice/01_subset_pruned.ein`,
  `examples/lattice/02_genuine_3set_death.ein`) +
  state-hash-collision fixture (verify on `zebra2-hints.ein`
  per Q1.5b.4.c).
- **F8.** `LatticeProof` data class + handoff to P1.6
  explanation phase (per Q1.5b.6).
- **F9.** End-of-phase perf round:
  - Subset-trie / interned-set-id optimisations (Q1.5b.2.b
    deferred items).
  - Decide on commitment-set ``try_commitment_set`` primitive (this is where
    the original Q1.5b.8 choice gets re-opened — but only
    if perf measurement shows incremental saturation is the
    bottleneck).
  - Tree-side deprecation tagging.

The feature order is a sketch — the per-stage plan settles
the precise boundaries after the bootstrap + backbone land.

### Why this defers the commitment-set vs incremental decision

Under bootstrap, the simplest thing is `try_branch` invoked
incrementally — no new primitive needed, common-file unchanged,
all existing tests stay green. The cost is `k` saturations per
size-`k` set instead of 1. Whether that cost is acceptable for
M1's `max_set_size` ≤ ~5 is an empirical question, answered
after the backbone runs.

If the answer is "fine" → no `try_commitment_set`, common engine stays
common. If the answer is "too slow" → copy to
`inference/lattice/engine.py` and add `try_commitment_set` as a DAG-specific
optimisation; common engine is still unchanged. Either way,
no design decision is locked in before the data is in.

---

## Q1.5b.9 — Scoring within a layer

**Status:** ✅ resolved 2026-05-25 (merged into
[Q1.5b.2.c](#q15b2--frontier-strategy)).

Default = **sum of element scores** under S1.5a.7's existing
per-element scorer. Switch:
`SolverConfig.lattice_order: Literal["lex", "score-sum"]`,
default `"score-sum"`. Lex is the deterministic / regression
mode.

Information yield, refutation yield, goal-proximity, etc. are
parked as candidates for the **"Phase within the Phase"**
follow-up that introduces partial-layer + best-first search.
Not part of the initial implementation.

---

## Q1.5b.10 — Mode handling — merged into Q1.5b.7

See [Q1.5b.7](#q15b7--termination--completeness--mode-handling)
§ "Mode handling" for the resolution. The two questions are
the same artefact under M1: mode shapes the verdict, not the
search.

---

## Resolved questions log

### 2026-05-25 (first round)

- **Q1.5b.1 (co-existence layout) — resolved.** User
  direction: copy-on-modify split between
  `inference/{*.py, tree/, lattice/}` + matching test-tree mirror.
  No upfront file inventory; modules diverge into
  `tree/`/`lattice/` only when DAG implementation requires it.
- **Q1.5b.2.a (re-saturation cadence) — resolved Option A.**
  Re-saturate + recompute alive after every entering's
  `integrate`; the lattice's pending frontier is re-filtered
  against the new alive between enterings.
- **Q1.5b.2.b' (subsumption by existing nogoods) — answered.**
  Don't add a clause that's strictly subsumed by an existing
  one (S1.5a.18's `emit_nogood` already does the right thing).
  Generation-time filter against `root_kb._nogoods`
  via `matches_any_nogood` covers conditional deaths whose
  clauses propagated up.
- **Q1.5b.2.c (within-layer ordering) — resolved switch.**
  `SolverConfig.lattice_order: Literal["lex","score-sum"]`,
  default `"score-sum"`; partial-layer / best-first parked as
  "Phase within the Phase" follow-up.
- **Q1.5b.2.d (Apriori example fixtures) — confirmed as task,
  not question.** Stage S1.5b.N produces
  `examples/lattice/01_subset_pruned.ein` and
  `examples/lattice/02_genuine_3set_death.ein`.
- **Q1.5b.6 (reasoning-path scope) — resolved both-phases-own.**
  P1.5b produces the `LatticeProof` data class (final kb + DAG
  + provenances + alive-at-end + dead-commitments); P1.6 owns
  the NL explanation walk over it.
- **Q1.5b.9 (scoring) — merged into Q1.5b.2.c.**

### 2026-05-25 (fifth round, refinement)

- **Q1.5b.7 refined: two engines, two termination criteria.**
  Phase P1.5b ships **both** a monotonic engine
  (terminates on first goal-satisfaction at root) and the
  lattice engine (exhaustive to `max_set_size`). Same
  Apriori + CDCL machinery; differ in storage + termination
  + mode support.
- **Monotonic vs lattice equivalence recorded.** For
  uniquely-solvable SOLVE-mode puzzles (Zebra): same
  `root.kb` at termination — monotonic stops earlier;
  lattice continues but doesn't refine root further.
  Argument captured in the Q1.5b.7 body's
  "Monotonic vs lattice equivalence" subsection.
- **Stage numbering: monotonic S1.5b.0–.10, lattice S1.5b.20+**
  with 10 slots reserved (S1.5b.11–.19) for monotonic
  followups so the lattice block doesn't need renumbering.
- **`inference/monotonic/` added** as the third sibling
  alongside `inference/tree/` and `inference/lattice/`. Same
  copy-on-modify rule (Q1.5b.1.a) applies.

### 2026-05-25 (fourth round)

- **Q1.5b.7 (termination + completeness + mode handling) —
  resolved exhaustive-to-`max_set_size`** with three-way
  verdict (Solved / Ambiguity / Contradiction) computed from
  the cumulative `A_*` / `D_*` frontier. Mode is read only at
  verdict-synthesis time, not during exploration.
  **Refined 2026-05-25b** — this resolution now applies to
  the lattice engine only; monotonic terminates on first
  goal-sat. See fifth round + the Q1.5b.7 body.
- **Q1.5b.10 merged into Q1.5b.7.** The mode-handling design
  collapses under M1's exhaustive search — mode parameter
  shapes the verdict, not the search itself.
- **Q1.5b.8 (engine bridge) — resolved copy-on-modify +
  bootstrap-style implementation order.** Reuse S1.5a.20's
  `try_branch` unchanged from common `inference/`; if a
  variant needs `try_commitment_set` (or any divergence), copy to
  `inference/<variant>/`. Implementation in rounds:
  bootstrap (stubs) → backbone (minimal runnable) →
  features.

### 2026-05-25 (third round)

- **Q1.5b.5.a (CDCL reuse) — yes, unified.** `nogoods.py`
  lives in the common `inference/` folder; tree + lattice both
  import. `KnowledgeBase._nogoods` is the canonical clause
  store, attribute stays in the common `kb/store.py`.
- **Q1.5b.5.b (subsumption policy) — same as S1.5a.18.**
  Strict-subset removes superset on emit.
- **Q1.5b.5.c (size-1 special case) — dropped for DAG.** The
  unified clause set treats `frozenset({h})` as just another
  clause; no special path. Tree's existing size-1 guard
  becomes a perf detail with no semantic effect.
- **Algorithm sketch — answers in [`algorithm_layer_n.md`](algorithm_layer_n.md)**
  with the colorful DOT diagram + the per-step contract,
  including the "what next?" answers for the dead/alive paths.

### 2026-05-25 (second round)

- **Q1.5b.2.b (layer-k candidate generation) — resolved
  textbook Apriori-gen.** Lexicographic prefix-join in the
  initial implementation; subset-trie + interned-id
  optimisations deferred to end-of-phase perf round.
- **Q1.5b.3 (lattice node representation) — resolved canonical
  tuple.** `CanonicalSetId = tuple[FactId, ...]` sorted by
  `(rn, args)`. Frozenset + interned-id parked as
  out-of-M1-scope.
- **Q1.5b.4.a (state-hash collision response) — resolved
  merge** with the conceptual "true lattice vs search lattice"
  distinction made explicit. Merged nodes carry **multilabel**
  + **multi-parent** lists; `save everything modulo dedup`.
- **Q1.5b.4.b — reframed.** Original "subtree sharing on dedup
  hit" question was wrong under BFS; the real question is how
  layer ``k+1`` sets find their parents. Answer: each set has
  ``k`` parents (its ``(k-1)``-subsets in ``A_{k-1}``);
  multi-parent intrinsic; multilabel composes when sets merge.
  `SetNode` data model captures both.
- **Q1.5b.4.c (state-hash collision fixture) — task confirmed.**
  `examples/zebra2-hints.ein` is the candidate; verify on
  first runnable lattice; fall back to a constructed minimal
  fixture if no collision is observed there.
