# P1.5b — Set-indexed search engines (monotonic + lattice)

**Estimate:** unknown — design phase; per-stage budget will be
drafted as the bootstrap + backbone rounds settle the
implementation surface.
**Status:** **active implementation** — phase opened 2026-05-25
after the user observation that ordered-path tree search wastes
``d!`` work on hypothesis permutations that are semantically
identical under M1's monotone saturation. Two engines ship
under this phase with **distinct purposes** (below). Stage
plan: monotonic stages first (S1.5b.0–S1.5b.10), then lattice
from S1.5b.20 onward. [S1.5b.1](s1.5b.1_file_split_refactor.md)
shipped 2026-05-27 (`22a4ebd`, file-split refactor —
`inference/{tree,monotonic,lattice}/`);
[S1.5b.2](s1.5b.2_common_apriori_gen.md) shipped 2026-05-27
(`80d47df`, common `apriori.py` — prefix-join + filter
helpers; 17 tests);
[S1.5b.3](s1.5b.3_common_set_batch_primitive.md) shipped
2026-05-27 (`424c9ac`, common `commitment.py` —
`try_commitment_set` + `CommitmentSetResult`; 7 tests covering
the alive / dead-pre / dead-post trichotomy +
unconditional-fact extraction + isolation +
empty-commitment sentinel);
[S1.5b.4](s1.5b.4_monotonic_skeleton.md) shipped 2026-05-27
(`cb7da8a`, monotonic engine skeleton — `inference/monotonic/`
stub modules + `demo/bench_monotonic.py` CLI; backbone fills
the stubs in S1.5b.5);
[S1.5b.5](s1.5b.5_monotonic_backbone.md) shipped 2026-05-27
**partial** (`8692e7c`) — monotonic backbone live (Apriori +
`try_commitment_set` + root-merge of unconditional facts +
Phase-1 ContradictionDetector); 8 tests pass; branching/01 →
Solution (root-only), branching/04 → Ambiguity (genuine
multi-solution); **branching/03 → Ambiguity not Solution**
(forced-positive promotion gap — see stage Ship notes). [S1.5b.5](s1.5b.5_monotonic_backbone.md)
(monotonic backbone — the main loop wiring `apriori` +
`try_commitment_set` + nogoods) is now the next implementation
surface.
**Depends on:** —
P1.5b owns its own isolation model: commitment-set `try_commitment_set` ↓ +
per-set `integrate` ↑, with no ancestor chains (commitments
are sets, not paths). The pre-2026-05-26 dependency on
[P1.5a S1.5a.20](../p1.5a_zebra_solution/s1.5a.20_branch_isolation_rearch.md)
(branch-isolation re-architecture) was dropped together with
S1.5a.20 itself — that stage's contract is delivered natively
here. `BranchResult`-shaped payloads still flow up via
integrate; the .20 file is retained as historical design notes
+ channel inventory.
Composes with
[S1.5a.18](../p1.5a_zebra_solution/s1.5a.18_path_condition_nogoods.md)
(path-condition no-goods become the unified CDCL prune
mechanism for both engines) +
[S1.5a.19](../p1.5a_zebra_solution/s1.5a.19_d0_negative_completion_gap.md)
(d=0 closure shrinks both engines' base layer).
**Blocks:** —

## Motivation

Under M1's rule set, saturation is **monotone** (no retraction;
every firing whose premises are present eventually fires) and
**order-commutative** (rule firings reach the same fixpoint
regardless of queue order). So for any commitment set
``C = {h_1, …, h_k}``, the post-saturation kb
``saturate(root ∪ C)`` is fully determined by ``C`` — the
ordering ``(h_σ(1), …, h_σ(k))`` for any permutation σ produces
the same final state.

The current [P1.5](../p1.5_hypothesis_loop/) /
[P1.5a](../p1.5a_zebra_solution/) implementation explores
**ordered paths**: a depth-``d`` search visits up to ``N!/(N−d)!``
distinct sequences of hypothesis enterings. State-hash dedup
collapses converging paths *after* they reach depth ``d`` —
catching the redundancy but paying the
``d! × try_branch + d! × intermediate-saturation`` cost first.

The ratio:

```
tree path count at depth d  =  N · (N−1) · … · (N−d+1)   ≈  N!/(N−d)!
set-indexed subset count    =  C(N, d)                   =  N!/(d!·(N−d)!)
                                                            ────────────────
                                                            ratio = d!
```

At depth 4 on zebra2 with N ≈ 89 candidates, the ceiling is
``4! = 24×`` redundancy. Set-indexed search recovers it.

## What this phase delivers

Two co-existing **set-indexed** search engines, both built on the
saturation-commutativity premise:

### Monotonic engine (`inference/monotonic/`)

> **Converges to the first met solution if any.**

The minimalist implementation. One ``KnowledgeBase`` instance
(root) accumulates unconditional facts as commitment sets are
tried; **terminates as soon as ``root.kb`` satisfies the goal**.
No per-set storage, no dedup, no DAG, no proof artefact beyond
the final root snapshot.

- Apriori prefix-join for layer-by-layer candidate generation.
- Commitment-set `try_commitment_set(root, C)` primitive — fork from root once
  per commitment, add all hypotheses, saturate once. No
  intermediate state shared between enterings.
- Each entering's **unconditional** facts (chain doesn't touch
  any element of `C`) merge into root; re-saturate root;
  re-prune alive.
- Conditional facts discarded with the fork.
- Early termination on goal-satisfaction. SOLVE-mode only.

### Lattice engine (`inference/lattice/`)

> **Converges to the full map of all solutions.**

The exhaustive implementation. Per-set `SetNode` storage with
multilabel + multi-parent; state-hash dedup as merge; per-set
audit trail in the dumper; produces a `LatticeProof` data
class (full DAG + provenances) for P1.6's explanation phase.

- Same Apriori prefix-join + commitment-set primitive.
- **Does not** terminate on first solution — exhausts to
  ``max_set_size`` so the full set of satisfying commitments
  is enumerated.
- Supports SOLVE / GAPS / CONTRADICTIONS modes (mode chooses
  the verdict shape; search is mode-agnostic per
  [Q1.5b.7](open_questions.md#q15b7--termination--completeness--mode-handling)).

### When to use which

| use case                                     | engine     |
|----------------------------------------------|------------|
| "give me a solution, fast"                   | monotonic  |
| "give me **all** solutions (GAPS mode)"      | lattice    |
| "give me the proof artefact for the trace"   | lattice    |
| "is the puzzle contradictory?"               | lattice    |
| M1 SOLVE acceptance on uniquely-solvable puzzles (Zebra) | monotonic suffices — its terminating root.kb has the same content as the lattice's accumulated unconditional facts |

For **uniquely-solvable SOLVE-mode puzzles**, the engines are
equivalent: both terminate (monotonic at first goal-satisfaction,
lattice on layer exhaustion) with the same `root.kb` content.
Monotonic gets there faster and with much less code; lattice
preserves the full audit + explanation surface.

See [`open_questions.md`](open_questions.md#monotonic-vs-lattice-equivalence)
for the equivalence argument.

## Co-existence model

Per user direction 2026-05-25:

```
ein.py/src/ein_bot/inference/
   ├── *.py                          ← common kernel: Saturator,
   │                                    ContradictionDetector, Lookahead,
   │                                    hypgen, nogoods, kb-store helpers,
   │                                    base dumper utilities
   ├── tree/                         ← current tree-search (reference)
   │   ├── solver.py                 ← _explore / _descend / _consume
   │   ├── back_prop.py              ← ancestor-chain back-prop (S1.5a.14)
   │   └── ...
   ├── monotonic/                    ← monotonic engine
   │   ├── solver.py                 ← single-root accumulator
   │   ├── apriori.py                ← prefix-join candidate generator
   │   └── ...
   └── lattice/                          ← lattice engine
       ├── solver.py                 ← BFS orchestrator
       ├── lattice.py                ← SetNode + multilabel + multi-parent
       ├── frontier.py               ← enumeration strategies
       ├── state_dump.py             ← per-set audit
       └── ...

ein.py/demo/
   ├── bench_solve.py                ← unchanged; tree entry
   ├── bench_monotonic.py            ← NEW; monotonic entry
   └── bench_dag.py                  ← NEW; lattice entry
```

**Three engines, three entry scripts, three import paths.**
Explicit selection per script — no engine-internal default
flag that picks one or the other. Copy-on-modify rule
(Q1.5b.1.a) applies: a module starts in common `inference/`;
when an engine needs to modify it incompatibly, **copy** to
the engine's folder and let the copy diverge.

**Tree as reference, not legacy.** Stays around until both
monotonic and lattice reach parity on the regression suite;
removal queued for end of phase.

## Scope boundary

P1.5b owns:
- The monotonic engine — full implementation.
- The lattice engine — full implementation including
  `SetNode`, per-set dumper, `LatticeProof`.
- The shared kernel pieces both engines need (Apriori-gen,
  commitment-set primitive, common nogood store).
- A new shared dumper base under `inference/state_dump.py`
  (split into common helpers + engine-specific layouts).
- Example fixtures under `examples/lattice/` exercising
  Apriori pruning + state-hash collision.

P1.5b does NOT own:
- The d=0 inference completeness fix
  ([S1.5a.19](../p1.5a_zebra_solution/s1.5a.19_d0_negative_completion_gap.md))
  — orthogonal; shrinks the search base for both engines.
- The NL trace renderer
  ([P1.6 S1.6.4](../p1.6_rendering_and_trace/))
  — consumes the lattice's `LatticeProof`.

## Acceptance for the phase

### Monotonic engine

1. `bench_monotonic.py examples/zebra2.ein` returns `Solution`
   with the same bindings as the tree-side target in
   [S1.5a.13](../p1.5a_zebra_solution/s1.5a.13_acceptance_zebra2_solves.md)
   within the same time budget (target: < 60s; ideally < 5s
   after S1.5a.19).
2. Every demo under `examples/branching/` reaches the
   tree-side verdict (SOLVE-mode bindings match).
3. The implementation is small — target ≤ 200 LOC including
   imports + helpers, excluding the common kernel modules
   it imports unchanged.

### Lattice engine

4. `bench_dag.py examples/zebra2.ein --max-set-size N`
   produces the same verdict as monotonic on Zebra (Solution
   with same bindings) AND additionally produces a
   `LatticeProof` artefact P1.6's NL renderer can consume.
5. **Feature parity** with the tree side on every fixture
   under `examples/branching/` and `examples/zebra2.ein` —
   same verdict, same bindings, same set of derived
   facts/negatives, same set of refutations.
6. **Per-set audit trail** in the dumper — every visited
   commitment set has its own folder; `00_timeline.jsonl`
   shows the lattice's exploration order; an `integrate`
   event lands per entering.
7. **State-hash dedup observed** — on
   `examples/zebra2-hints.ein` (or a constructed fixture per
   [Q1.5b.4.c](open_questions.md#q15b4--set-equivalence-dedup--state-hash-dedup)),
   at least one `SetNode` has `len(labels) > 1`.
8. **d!-redundancy measurement** documented: on zebra2 with
   `--max-set-size 4`, lattice visits ≤ C(N, ≤4) sets; tree
   visits up to N!/(N−4)! paths. Report the ratio.

### Phase

9. **Tree-search removal** queued — last stage tags the tree
   side as `# deprecated 2026-XX-XX, remove after P1.6/P1.7
   green on monotonic + lattice`.

## Stages

### Monotonic engine — S1.5b.0 to S1.5b.10

| ID       | Title                                                                                  |
|----------|----------------------------------------------------------------------------------------|
| S1.5b.0  | Discussion summary + decisions log (closes [`open_questions.md`](open_questions.md))   |
| S1.5b.1  | File-split refactor — `inference/{,tree/,monotonic/,lattice/}` + tests mirror              |
| S1.5b.2  | Common Apriori-gen module — `inference/apriori.py` (prefix-join + filters)             |
| S1.5b.3  | Common commitment-set primitive — `try_commitment_set(root_kb, commitment) → CommitmentSetResult`            |
| S1.5b.4  | `inference/monotonic/` skeleton + `bench_monotonic.py` skeleton                        |
| S1.5b.5  | Monotonic backbone — single-root accumulator; early terminate on goal                  |
| S1.5b.6  | Monotonic CDCL — unified `_nogoods` store; `matches_any_nogood` filter                 |
| S1.5b.7  | Monotonic dumper — root snapshot per layer (minimal — no per-set storage)              |
| S1.5b.8  | Monotonic acceptance — `bench_monotonic examples/zebra2.ein` returns `Solution`        |
| S1.5b.9  | Monotonic feature parity on `examples/branching/*` against tree                        |
| S1.5b.10 | Monotonic docs + tests — `tests/inference/monotonic/`; module docstrings; cross-links  |

### *(S1.5b.11 to S1.5b.19 reserved — monotonic perf / extension space)*

### Lattice engine — S1.5b.20 onward

| ID       | Title                                                                                  |
|----------|----------------------------------------------------------------------------------------|
| S1.5b.20 | `inference/lattice/` skeleton + `bench_dag.py` skeleton                                    |
| S1.5b.21 | Lattice backbone — `lattice.py` (`CanonicalSetId`, `SetNode` without dedup yet); `solver.py` exhaustive BFS; verdict trichotomy from [Q1.5b.7](open_questions.md#q15b7--termination--completeness--mode-handling) |
| S1.5b.22 | F1 — state-hash dedup checkpoint + `SetNode` multilabel + multi-parent storage         |
| S1.5b.23 | F2 — `lattice/state_dump.py` per-set audit (`set/<canonical-slug>/`; `00_timeline.jsonl`)  |
| S1.5b.24 | F3 — back-prop integrate (multi-parent bubble; Option A cadence)                       |
| S1.5b.25 | F4 — forced-positive mining + per-level `is_unconditional_at` in integrate             |
| S1.5b.26 | F5 — within-layer scoring switch (`lattice_order: "lex" \| "score-sum"`)               |
| S1.5b.27 | F6 — optional sanity check (`--lattice-sanity-check`)                                  |
| S1.5b.28 | F7 — example fixtures (`examples/lattice/01_subset_pruned.ein`, `02_genuine_3set_death.ein`) + state-hash-collision verification on `examples/zebra2-hints.ein` |
| S1.5b.29 | F8 — `LatticeProof` data class + P1.6 handoff contract (per [Q1.5b.6](open_questions.md#q15b6--reasoning-path-post-solve-phase)) |
| S1.5b.30 | F9 — end-of-phase perf round: subset-trie / interned set-ids; `try_commitment_set` commitment-set perf measurement; tree-side deprecation tag |
| S1.5b.31 | Lattice shuffle invariance — traversal-order regression net (tree-side sibling: P1.5a S1.5a.16, closed 2026-05-26) |
| S1.5b.32 | Domain-elim rule (forall) vs explicit hypothesis exploration — research stage: result/correctness/satisfiability vs performance trade-offs across both engines |

The boundaries between stages will shift; the numbering keeps
a 10-slot gap between monotonic and lattice so follow-up
monotonic work (S1.5b.11–.19) can land without renumbering the
lattice block.

## Cross-links

- Historical design notes (no longer prerequisite — superseded
  by this phase 2026-05-26):
  [P1.5a S1.5a.20](../p1.5a_zebra_solution/s1.5a.20_branch_isolation_rearch.md)
  — channel inventory + per-child branch-dump schema (T1.5a.20.6)
  that lattice's per-set dumper still references.
- Composes with:
  - [P1.5a S1.5a.18](../p1.5a_zebra_solution/s1.5a.18_path_condition_nogoods.md)
    — nogoods become the unified CDCL prune mechanism.
  - [P1.5a S1.5a.19](../p1.5a_zebra_solution/s1.5a.19_d0_negative_completion_gap.md)
    — d=0 closure shrinks both engines' base.
  - [P1.5a S1.5a.7](../p1.5a_zebra_solution/s1.5a.7_hypgen_scoring_branch_info.md)
    — hypothesis scoring re-aimed at per-set selection (lattice F5).
- Background:
  - [P1.5](../p1.5_hypothesis_loop/) — the ordered-tree search.
  - [P1.5a](../p1.5a_zebra_solution/) — the perf-lever chain
    whose downstream stages share infrastructure.
  - [idea-08](../../../docs/ideas/08-human-style-deductive-trace.md)
    — the NL trace target.
- Open questions for design phase:
  [`open_questions.md`](open_questions.md).
- Algorithm diagrams: [`diagrams/`](diagrams/) +
  [`lattice_diagrams.md`](lattice_diagrams.md) +
  [`algorithm_layer_n.md`](algorithm_layer_n.md).
