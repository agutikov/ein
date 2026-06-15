# P1.5b — Set-indexed search engines (monotonic + lattice)

**Estimate:** unknown — design phase; per-stage budget will be
drafted as the lattice stages settle the implementation
surface.
**Status:** **shipped 2026-05-29**. The lattice block
closed with [S1.5b.30](s1.5b.30_lattice_perf_round.md) —
zebra2 perf round measured under PyPy, all three deferred
optimisations evaluated and deferred to followup, every
phase-level acceptance criterion is either ✓-closed or
△-amended (see [Acceptance for the phase](#acceptance-for-the-phase)).
The post-phase task is **tree-solver removal** (full
deletion of `inference/tree/`, migration of the `Verdict`
types out to a neutral home; tracked outside this phase
per user direction). Original opening status preserved
below.

Phase opened 2026-05-25
after the user observation that ordered-path tree search wastes
``d!`` work on hypothesis permutations that are semantically
identical under M1's monotone saturation. **One unified engine**
ships under this phase, hosting all three public entries
under `inference/monotonic/`. Stage plan: monotonic stages
first (S1.5b.0–S1.5b.10, all shipped); lattice-feature
stages from S1.5b.20 onward extend the same package. Killed
2026-05-28 in light of monotone-saturation properties:
**S1.5b.24** (multi-parent integrate — flat root-writes
are equivalent) and **S1.5b.25** (per-set forced-positive
mining — already covered by `CommitmentSetResult.unconditional_facts`).
Also clarified 2026-05-28 (in three stages): the engine
is **NOT a single mode-driven function** (`monotonic_solve`,
`gaps_solve`, `contradictions_solve` are separate functions
with distinct return types); the engine is **NOT split
across two folders** (everything lives in
`inference/monotonic/`); the core loop is **NOT duplicated**
across the three entries (S1.5b.21 extracts a shared
`_explore_layers` helper). See
[`project-set-search-unified` memory] +
[`algorithm_layer_n.md`](algorithm_layer_n.md). [S1.5b.1](s1.5b.1_file_split_refactor.md)
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
multi-solution); branching/03 was Ambiguity due to the
forced-positive promotion gap.
[S1.5b.5b](s1.5b.5b_monotonic_forced_positive.md) shipped
2026-05-27 (`8c483d6`) — **closes the gap**: forced-positive promotion
+ symmetric canonicalisation in `_compute_alive`. 7 of 11
branching demos flipped Ambiguity → Solution; **zebra2
solves under the monotonic backbone in ~5s on CPython** (vs
tree's ~50s). 9 monotonic tests pass.
Monotonic block closed 2026-05-28 with
[S1.5b.10](s1.5b.10_monotonic_docs_tests.md) (`4bf49fe`,
docs + 38 monotonic tests).
[S1.5b.20](s1.5b.20_lattice_skeleton.md) shipped 2026-05-28
(`9f8b910`, lattice-entry skeleton — stub dataclasses,
`gaps_solve`/`contradictions_solve` raising
`NotImplementedError`, `LatticeDumper` no-op shell,
`bench_lattice.py` dispatcher).
[S1.5b.21](s1.5b.21_lattice_backbone.md) shipped 2026-05-29
(`a5efdde`, gaps_solve backbone — extracts shared
`_explore_layers(entry, …)` helper out of `monotonic_solve`;
`gaps_solve` collects every satisfying commitment;
verdict = `Ambiguity` always; 8 tests pass branching/01,
branching/03, branching/04 (2 branches), branching/05 (3
branches)).
[S1.5b.22](s1.5b.22_lattice_dedup.md) shipped 2026-05-29
(`14aae12`, `LatticeProof` + `store_lattice` flag —
fills `SolutionRecord` / `DeadCommitment` / `SetNode` /
`LatticeProof` / `LatticeStats` with real fields; adds
`KnowledgeBase.snapshot()` for archival branch kbs;
under gaps the dedup MERGE is auto-disabled
(`hash(commitment)` keying — `state_hash_merges` stays 0);
the contradictions-side merge branch is forward-compat
(S1.5b.23 lifts the upstream raise to activate it); 12 new
test bodies, 783 pytest green, ruff clean).
[S1.5b.23](s1.5b.23_lattice_dumper.md) shipped 2026-05-29
(`contradictions_solve` backbone — lifts the upstream
`entry == "contradictions"` raise in `_explore_layers`;
adds `DeadCommitment` collection at the dead branch
(skipped on state-hash-merge per S1.5b.22's subsumption
comment); entry-aware Phase 1 / Phase 2 dispatch so
contradictions does NOT short-circuit on root-is_solved or
cascade-Solution but DOES on root-contradiction or
cascade-Contradiction; fork-side is_solved falls through to
the alive flow so supersets of solved commitments are still
explored; 9 new tests, all 11 branching fixtures
smoke-cleared, 792 pytest green, ruff clean).
[S1.5b.31](s1.5b.31_lattice_shuffle_invariance.md) shipped
2026-05-29 (lattice shuffle invariance —
`SolverConfig.lattice_order_seed: int | None = None`
orthogonal to `lattice_order`; `_explore_layers` shuffles
each layer's candidates via `random.Random(seed)` after
`order_candidates`; new
`inference/monotonic/snapshot.py` ships
`LatticeSnapshotV1` + `lattice_snapshot(verdict, root_kb)`
projecting the LatticeProof into a content-addressed
permutation-invariant snapshot (state_hash-keyed
multilabel union); test harness parametrises 2 entries x
5 fixtures x 3 depths x 10 seeds = 300 triples plus 3
sanity tests, all green — the engine loop is verified
traversal-order-invariant on every shipped fixture;
1131 pytest green, ruff clean).
[S1.5b.26](s1.5b.26_lattice_scoring.md) shipped 2026-05-29
(within-layer scoring switch — `inference/apriori.py`
ships `order_candidates(candidates, mode, kb)` with the
`"lex"` / `"score-sum"` modes; wired into
`_explore_layers` via `SolverConfig.lattice_order`
(default `"lex"` — preserves regression baselines);
`score-sum` reuses S1.5a.7's `score_hypothesis` and is
informed only under non-default `hypgen_scoring`;
`bench_lattice.py --lattice-order {lex,score-sum}`
forces the config field; 8 new tests; 828 pytest green,
ruff clean).
[S1.5b.27](s1.5b.27_lattice_sanity_check.md) shipped 2026-05-29
(saturation-commutativity sanity check —
`inference/monotonic/sanity.py` ships `check_commutativity`
+ `SanityError`; gated by
`SolverConfig.lattice_sanity_check` (default False) and
`--lattice-sanity-check` CLI flag; wired into
`_explore_layers` right after the alive `_record_setnode`;
costs ``k+1`` saturations per checked size-``k`` commitment
so off by default; verified M1's premise holds on every
shipped fixture — 11 branching + 3 lattice — under the
release-regression flag; 7 new tests, 820 pytest green,
ruff clean).
[S1.5b.28](s1.5b.28_lattice_fixtures.md) shipped 2026-05-29
(lattice fixtures + state-hash collision verification —
ships `examples/lattice/01_subset_pruned.ein` (Apriori
prefix-join structural pruning),
`02_genuine_3set_death.ein` (combinatorial-core 3-set
death), `03_state_hash_collision.ein` (canonical fast
fixture for the merge path); verified zebra2-hints
naturally triggers strong collisions — 13 correct-hint
commitments collapse to one multilabel SetNode at
max_set_size=1, documented in Findings; 6 fast pinned
tests + 1 EIN_RUN_SLOW-gated zebra2-hints test;
813 pytest green, ruff clean).
[S1.5b.29](s1.5b.29_lattice_proof.md) shipped 2026-05-29
(`LatticeDumper` implementation + P1.6 handoff contract —
fills the per-set audit folder layout `root.ein` +
`00_timeline.jsonl` + `layers/{NN_pre,NN_post}.ein` +
`solutions/<C-slug>/` (gaps) + `dead/<C-slug>/`
(contradictions) + `kb_index/<state_hash_hex>/`
(store_lattice) + `proof_summary.json` + `summary.json`;
adds `validate_proof_for_explanation` checking 6 structural
invariants the P1.6 NL renderer will rely on; `_finish` now
calls `dumper.proof_summary(verdict.proof)` before
`dumper.summary` so both lattice entries emit the index;
15 new test bodies; bench-smoked on branching/04 under all
4 entry x store_lattice combinations; 807 pytest green,
ruff clean).
[S1.5b.30](s1.5b.30_lattice_perf_round.md) shipped
2026-05-29 (phase closer — `bench_lattice_pypy.sh`
runner; zebra2 measurement under PyPy across
tree/monotonic/gaps/contradictions/contradictions+store;
all three deferred optimisations evaluated and deferred;
tree-side soft-deprecation skipped per user direction
in favour of full removal as the post-phase task). The
post-phase surface is **tree-solver removal** (full
deletion of `inference/tree/` + migration of the
`Verdict` types — tracked outside this phase).
[S1.5b.32](s1.5b.32_domain_elim_vs_hyp_exploration.md)
(research-stage write-up: domain-elim rule vs explicit
hypothesis exploration) **shipped 2026-05-29** — report at
[`docs/kernel/inference/domain_elim_vs_hypothesis.md`](../../../docs/kernel/inference/domain_elim_vs_hypothesis.md),
recommendation **pathway A as default, B as fallback**. With
that the phase has no outstanding items.
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

**One engine, three sibling public entries** — all hosted
side-by-side in `inference/monotonic/`. **Not** a single
`solve(mode=…)` dispatcher; **not** two parallel engine
folders. Per user direction 2026-05-28 (in three stages):

> 1. *"not mode-driven; solution/gaps/... mode separately,
>    flag for storage for lattice nodes - separately;
>    monotonic works only with solution mode"*
> 2. *"unified monotonic-lattice engine"*
> 3. *"do not duplicate main engine, they all first iterate
>    lattice nodes, and then make decisions about results
>    depending on mode"*

The three contracts (fast-solution / complete-solution-set
/ refutation-map) get three distinct function signatures
with distinct return types — the type system carries each
contract — but they share **one** private `_explore_layers`
helper that hosts the per-candidate flow from
[`algorithm_layer_n.md`](algorithm_layer_n.md). S1.5b.21
extracts the helper from the existing `monotonic_solve`
body; S1.5b.21 + S1.5b.23 add the two new entries as thin
wrappers calling the same helper with different
discriminators.

### The three entries

```python
# inference/monotonic/solver.py — all three live here.

def monotonic_solve(
    kb,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    mode: Mode = Mode.SOLVE,    # legacy guard; raises for non-SOLVE
    dumper: MonotonicDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Verdict, MonotonicStats]:
    """SOLUTION MODE. Early-terminate on first goal-sat.
    Verdict: Solution / Ambiguity-frontier / Contradiction.
    Already shipped under S1.5b.0 to S1.5b.10."""


def gaps_solve(
    kb,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,            # orthogonal storage toggle
    dumper: LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Ambiguity, LatticeStats]:
    """GAPS MODE. Exhaustive. Collects every satisfying
    commitment into proof.solutions. Verdict is always
    Ambiguity (contract); caller checks
    len(verdict.proof.solutions) to interpret —
    == 1 means uniquely solvable, > 1 means genuine
    multi-solution, == 0 means unsolvable within the cap.
    Backbone: S1.5b.21."""


def contradictions_solve(
    kb,
    *,
    max_set_size: int = 5,
    config: SolverConfig | None = None,
    store_lattice: bool = False,
    dumper: LatticeDumper | None = None,
    max_time: float | None = None,
    max_enterings: int | None = None,
) -> tuple[Contradiction, LatticeStats]:
    """CONTRADICTIONS MODE. Exhaustive. Collects every dead
    commitment into proof.dead_commitments. Verdict is
    always Contradiction (contract); unsat_core is the
    union of every recorded dead's core plus the learned
    nogood clauses. Backbone: S1.5b.23."""
```

Under the hood (S1.5b.21):

```python
def _explore_layers(
    root_kb, *,
    entry: Literal["monotonic", "gaps", "contradictions"],
    max_set_size, config, store_lattice, dumper, max_time, max_enterings,
) -> tuple[Verdict, Stats]:
    """Shared core. Dispatches on `entry` for outcome handling +
    verdict synthesis. The three public functions are thin
    wrappers that fix `entry` and the return-type-narrowing."""
```

### `store_lattice` — orthogonal storage flag

Only on `gaps_solve` and `contradictions_solve` (monotonic
doesn't take it — no storage by design).

When `False` (default):

- `proof.solutions` (gaps) / `proof.dead_commitments`
  (contradictions) are collected — these are mandatory for
  the verdict contracts.
- No per-SetNode storage. No state-hash dedup. Lighter
  memory, faster.

When `True`:

- Additionally builds `proof.kb_index: dict[int, SetNode]`
  — one SetNode per visited commitment with kb_snapshot,
  enabling state-hash dedup as multilabel merge (under
  `contradictions_solve` only) + DAG audit + the per-set
  dumper folders (under either entry).
- For `gaps_solve`: state-hash dedup MERGE is
  auto-disabled (distinct satisfying commitments must
  register separately under GAPS contract). SetNodes are
  built but two with identical state_hash stay separate.

### Why monotonic doesn't have `store_lattice`

Monotonic is the "fastest path to first solution" entry.
It commits unconditional facts to root incrementally; the
only kbs that ever exist outside the loop are root.kb and
the surviving fork's kb (the returned Solution.kb). There's
no DAG to store — every dead commitment's clause goes
straight to `root._nogoods`, never per-SetNode. Adding a
storage flag would be feature creep against the entry's
single contract.

If you want lattice storage for a *uniquely-solvable*
puzzle (to feed P1.6 the explanation artefact), use
`gaps_solve(kb, store_lattice=True)` and assert
`len(proof.solutions) == 1`. The verdict shape becomes
`Ambiguity` (per GAPS contract) but `branches[0].kb` IS the
unique solution.

### When to use which entry

| use case                                                   | entry                                              |
|------------------------------------------------------------|----------------------------------------------------|
| "give me a solution, fast" (today's behavior)              | `monotonic_solve(kb)`                              |
| "verify uniqueness on a SOLVE-shaped puzzle"               | `gaps_solve(kb)` + check `len(solutions) == 1`     |
| "give me **all** solutions"                                 | `gaps_solve(kb)`                                    |
| "give me the proof artefact for the NL trace"              | `gaps_solve(kb, store_lattice=True)` (or contradictions_solve for the refutation half) |
| "is the puzzle contradictory? / refutation map"            | `contradictions_solve(kb)`                          |

For **uniquely-solvable puzzles** (Zebra), `monotonic_solve`
and `gaps_solve` accumulate the same set of unconditional
facts into root; monotonic stops sooner, gaps continues to
verify no second solution exists. See
[`open_questions.md`](open_questions.md#monotonic-vs-lattice-equivalence)
for the equivalence argument.

## Layout

```
ein.py/src/ein/inference/
   ├── *.py                          ← common kernel: Saturator,
   │                                    ContradictionDetector, Lookahead,
   │                                    hypgen, nogoods, apriori, commitment,
   │                                    canon (state_hash), kb-store helpers
   ├── tree/                         ← legacy tree-search (reference; deprecated end-of-phase)
   │   ├── solver.py                 ← _explore / _descend / _consume
   │   ├── back_prop.py              ← ancestor-chain back-prop (S1.5a.14)
   │   └── ...
   ├── monotonic/                    ← UNIFIED set-search engine; all three public entries
   │   ├── solver.py                 ← monotonic_solve (shipped S1.5b.0–.10)
   │   │                               + gaps_solve (S1.5b.20 stub, S1.5b.21 backbone)
   │   │                               + contradictions_solve (S1.5b.20 stub, S1.5b.23 backbone)
   │   │                               + private _explore_layers shared core (S1.5b.21)
   │   ├── lattice.py                ← LatticeProof, SolutionRecord, DeadCommitment,
   │   │                               SetNode, LatticeStats (S1.5b.20 stubs, S1.5b.22 fields)
   │   ├── state_dump.py             ← MonotonicDumper (S1.5b.7) + LatticeDumper
   │   │                               (S1.5b.20 stub, S1.5b.29 implementation)
   │   └── __init__.py               ← re-exports all three entries + all data types
   └── lattice/                      ← empty package (legacy 0-byte __init__.py;
                                        kept for future namespace use, not an engine)

ein.py/demo/
   ├── bench_solve.py                ← unchanged; legacy tree entry (deprecated end-of-phase)
   ├── bench_monotonic.py            ← monotonic_solve entry (shipped S1.5b.0–.10)
   └── bench_lattice.py              ← gaps_solve + contradictions_solve dispatcher (S1.5b.20)
                                        --gaps           → gaps_solve
                                        --contradictions → contradictions_solve
                                        --store-lattice  → orthogonal storage flag
                                        imports from inference.monotonic (the unified engine)
```

**One engine, three public entries, three demo scripts.**
The `inference/monotonic/` subpackage hosts all three
public functions (sharing the private `_explore_layers`
helper that S1.5b.21 extracts from the existing
`monotonic_solve` body) plus the data-structure module
(`lattice.py`) consumed by `gaps_solve` /
`contradictions_solve` and by P1.6's explanation walk. The
folder name `monotonic` describes one of the three
functions, not the whole engine — kept to avoid churning
imports across the codebase. The `--gaps` /
`--contradictions` flag on `bench_lattice.py` is **CLI
dispatch**, not a mode parameter on a single entry — each
public function has its own contract and return type.

**`inference/lattice/` stays empty.** The 0-byte
`__init__.py` exists from before the unification (legacy
package layout); not deleting the folder lets us reserve
the namespace for future use without churning git history.
Do NOT add solver modules there — the unified engine lives
in `inference/monotonic/`.

**Tree as reference, not legacy.** Stays around until the
two lattice entries reach parity on the regression suite;
removal queued for end of phase (S1.5b.30 perf round).

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

### Monotonic engine — ✓ shipped 2026-05-28

1. ✓ `bench_monotonic.py examples/zebra2.ein` returns
   `Solution` with the same bindings as the tree-side target
   in [S1.5a.13](../p1.5a_zebra_solution/s1.5a.13_acceptance_zebra2_solves.md)
   within the same time budget. **Closed 2026-05-27 by
   [S1.5b.8](s1.5b.8_monotonic_acceptance.md)** —
   measured **2.76 s CPython / 1.91 s PyPy** (well under the
   < 60 s ceiling and the < 5 s ideal).
2. ✓ Every demo under `examples/branching/` reaches the
   tree-side verdict (SOLVE-mode bindings match). **Closed
   2026-05-28 by [S1.5b.9](s1.5b.9_monotonic_branching_parity.md)** —
   11/11 fixtures green in
   [`parity_baselines.md`](parity_baselines.md);
   `04_two_levels.ein` is a documented Q1.5b.7 divergence
   (tree Ambiguity vs monotonic SOLVE first-solution).
3. △ The implementation grew beyond the original ≤ 200 LOC
   target as features landed (S1.5b.6 CDCL, S1.5b.7 dumper,
   S1.5b.9 fork-side `is_solved` fix). Final LOC:
   - `solver.py` — 570 LOC (~430 non-blank-non-comment).
     Backbone + CDCL emit/filter/writeback + budget gates +
     dumper hooks + fork-side `is_solved` + forced-positive
     promotion cascade.
   - `state_dump.py` — 212 LOC (`MonotonicDumper` hook framework).
   - `__init__.py` — 55 LOC (re-exports + module docstring).
   The original 200-LOC target was set for the **backbone
   alone** (S1.5b.5); with the feature stages folded in, the
   per-stage LOC delta is in the respective stage docs. Code
   complexity remains low: most lines are docstrings + the
   six hook-site sprinkles in the backbone loop; the CDCL
   path is ~20 lines.

### Lattice engine — gaps_solve

4. △ `bench_lattice.py --gaps examples/zebra2.ein` returns
   `Ambiguity` with `len(verdict.proof.solutions) == 15`
   (NOT 1 — original acceptance text outdated). The 15
   satisfying commitments are 15 distinct hint-commitments
   that each saturate to a goal-sat kb; 14 of them collapse
   to **one of 2** distinct post-saturation `state_hash`
   values (verified via direct state_hash comparison —
   the puzzle answer is shared but the per-commitment
   hypothesis-fact preserved in each branch differs).
   Acceptance amended by [S1.5b.30](s1.5b.30_lattice_perf_round.md) —
   true unique-solvability under gaps_solve requires a
   state-hash-collapsing post-projection (not exposed by
   gaps_solve, by GAPS-contract design). **Closed 2026-05-29.**
5. ✓ `bench_lattice.py --gaps examples/branching/04_two_levels.ein`
   returns `Ambiguity` with `len(proof.solutions) == 2`
   (Blue↔H3, Green↔H3 — both enumerated). **Closed
   2026-05-29 by [S1.5b.21](s1.5b.21_lattice_backbone.md)** —
   `test_gaps_solve_branching_04_returns_two_branches` in
   `test_gaps_backbone.py`.
6. ✓ `bench_lattice.py --gaps --store-lattice
   examples/zebra2.ein` additionally builds
   `proof.kb_index`. No state-hash dedup merge fires under
   GAPS, even with `--store-lattice` (auto-disabled —
   correctness). **Closed 2026-05-29 by
   [S1.5b.22](s1.5b.22_lattice_dedup.md)** —
   `test_gaps_solve_kb_index_no_merge` +
   measurement on zebra2 (`state_hash_merges == 0` even
   with 42 kb_index nodes).

### Lattice engine — contradictions_solve

7. ✓ `bench_lattice.py --contradictions
   examples/branching/04_two_levels.ein` returns
   `Contradiction` with `proof.dead_commitments` populated
   + `verdict.unsat_core` = union of all dead cores.
   **Closed 2026-05-29 by
   [S1.5b.23](s1.5b.23_lattice_dumper.md)** — 4 dead
   commitments on branching/04 + the union-invariant
   test in `test_contradictions_backbone.py`.
8. ✓ `bench_lattice.py --contradictions --store-lattice
   examples/zebra2-hints.ein` builds the dead SetNode DAG;
   at least one `SetNode` has `len(labels) > 1` on this
   fixture. **Closed 2026-05-29 by
   [S1.5b.28](s1.5b.28_lattice_fixtures.md)** —
   measured 13 correct-hint commitments collapsing into
   one multilabel SetNode (state_hash_merges = 12 at
   max_set_size=1).

### Dumper + measurement

9. ✓ **Lattice dumper** — both `--gaps` and `--contradictions`
   with `--dump-states <dir>` produce a per-set audit
   folder layout; the dump is mode-shaped. **Closed
   2026-05-29 by [S1.5b.29](s1.5b.29_lattice_proof.md)** —
   7 dumper tests covering each section's
   presence/absence + 8 P1.6 contract tests.
10. ✓ **d!-redundancy measurement** documented in
    [S1.5b.30's Measurement results](s1.5b.30_lattice_perf_round.md#measurement-results-2026-05-29).
    Five-engine table (tree / monotonic / gaps /
    contradictions / contradictions+store_lattice) with
    elapsed + visited + state_hash_merges metrics. The
    24× ceiling for `4! = 24×` applies to full-enumeration
    cases; on the hint-rich zebra2 both engines terminate
    well before depth 4 matters. **Closed 2026-05-29.**

### Phase

11. ▣ **Tree-search removal** — original spec called for a
    soft-deprecation warning on `inference/tree/`. Per
    user direction 2026-05-29, **soft-deprecation skipped**;
    full removal queued as the **next task after S1.5b.30
    ships** (tree gets deleted wholesale; `Verdict` types
    migrate out of `tree.solver` into a neutral home).
    Tracked outside this phase.

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

### Lattice-feature stages — S1.5b.20 onward

Each stage builds out the unified set-search engine in
`inference/monotonic/` — adding the lattice data types,
the two non-monotonic public entries (`gaps_solve`,
`contradictions_solve`), and the LatticeDumper. The
`store_lattice` flag is orthogonal and lands in S1.5b.22.

| ID       | Title                                                                                  |
|----------|----------------------------------------------------------------------------------------|
| S1.5b.20 | **Lattice-entry skeleton** — `inference/monotonic/lattice.py` (data class stubs); `inference/monotonic/solver.py` extended with `gaps_solve` + `contradictions_solve` stubs raising `NotImplementedError`; `inference/monotonic/state_dump.py` extended with `LatticeDumper` (no-op hooks); `bench_lattice.py` skeleton dispatcher; tests under `tests/inference/lattice/` collect with skip markers |
| S1.5b.21 | **`gaps_solve` backbone + shared `_explore_layers`** — refactor: extract the existing `monotonic_solve` loop body into a private `_explore_layers(entry="monotonic"\|"gaps"\|"contradictions", …)` helper; rewire `monotonic_solve` to call it with `entry="monotonic"`; add `gaps_solve` calling with `entry="gaps"`; exhaustive (no early-terminate when entry != "monotonic"); collects every satisfying commitment; verdict = `Ambiguity` always (mode contract); NO `LatticeProof` fields yet (S1.5b.22), NO `store_lattice` storage (S1.5b.22), NO LatticeDumper sections (S1.5b.29) |
| S1.5b.22 | **`LatticeProof` data class + `store_lattice` flag** — `inference/monotonic/lattice.py` fills `SolutionRecord`, `DeadCommitment`, `SetNode`, `LatticeProof`, `LatticeStats` with real fields; `gaps_solve` returns `Ambiguity(proof=…)`; `store_lattice=True` builds per-SetNode `kb_index` (state-hash dedup merge auto-disabled under `gaps_solve` — correctness); also adds `KnowledgeBase.snapshot()` to `kb/store.py` |
| S1.5b.23 | **`contradictions_solve` backbone** — calls `_explore_layers` with `entry="contradictions"`; collects every dead commitment into `proof.dead_commitments`; verdict = `Contradiction(unsat_core=⋃ cores, proof=…)` always; under `store_lattice=True` the state-hash dedup merge fires |
| ~~S1.5b.24~~ | ~~Back-prop integrate~~ — **KILLED** 2026-05-28: flat root-writes are equivalent under monotone saturation. See [s1.5b.24_lattice_integrate.md](s1.5b.24_lattice_integrate.md). |
| ~~S1.5b.25~~ | ~~Per-set forced-positive mining~~ — **KILLED** 2026-05-28: already provided by `CommitmentSetResult.unconditional_facts` + flat root-merge. See [s1.5b.25_lattice_forced_positives.md](s1.5b.25_lattice_forced_positives.md). |
| S1.5b.26 | Within-layer scoring switch (`lattice_order: "lex" \| "score-sum"`) — applies to all three entries via shared `_explore_layers` |
| S1.5b.27 | Optional saturation-commutativity sanity check (`--commutativity-check`) — dev/regression tool; validates the premise all three entries rest on |
| S1.5b.28 | Lattice example fixtures + state-hash collision measurement — `examples/lattice/01_subset_pruned.ein`, `02_genuine_3set_death.ein`; verify state-hash collision on `examples/zebra2-hints.ein` under `contradictions_solve --store-lattice` |
| S1.5b.29 | **`LatticeDumper` implementation + P1.6 handoff contract** — fills `inference/monotonic/state_dump.py`'s `LatticeDumper` class with the real per-set folder layout under `--dump-states`; ratifies the `LatticeProof` contract via mock-walker tests; both `gaps_solve` + `contradictions_solve` get dumper hooks in `_explore_layers` |
| S1.5b.30 | End-of-phase perf round + tree-side deprecation — subset-trie / interned set-ids; tree gets `# deprecated` tags; measure d!-redundancy ratio on zebra2; rename considerations parked (the folder name `monotonic` describes one of three entries, but renaming would churn imports across the codebase) |
| S1.5b.31 | Lattice shuffle invariance — traversal-order regression net for both lattice entries (tree-side sibling: P1.5a S1.5a.16, closed 2026-05-26) |
| S1.5b.32 | Domain-elim rule (forall) vs explicit hypothesis exploration — research stage: result/correctness/satisfiability vs performance trade-offs across the unified engine's three entries + tree side |

The boundaries between stages will shift; the numbering keeps
a 10-slot gap between the monotonic stages (S1.5b.0–.10) and
the lattice stages (S1.5b.20+) so follow-up monotonic backbone
work (S1.5b.11–.19) can land without renumbering.

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
