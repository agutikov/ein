"""Unified monotonic + lattice set-search engine.

This package hosts **all three** public entries for the
set-indexed search engine — the design decision finalised
2026-05-28 was a single engine with three sibling functions
rather than two parallel engines or one mode-dispatch
function. All three share the per-candidate flow from
``algorithm_layer_n.md`` (Apriori prefix-join +
``try_commitment_set`` + flat root-writes); they differ only
in whether the loop early-terminates and what gets collected:

- :func:`monotonic_solve` — solution mode. Early-terminates
  on first goal-satisfying fork. No :class:`LatticeProof`
  collection. Returns :class:`Solution` /
  :class:`Ambiguity` (frontier) / :class:`Contradiction`.
  **The first-solution-wins shape that suffices for solving
  uniquely-solvable puzzles fast.** Already shipped under
  S1.5b.0 through S1.5b.10.
- :func:`gaps_solve` — GAPS contract. Exhaustive Apriori-gen.
  Collects every satisfying commitment into
  ``proof.solutions``. Returns :class:`Ambiguity` always.
  Backbone: **S1.5b.21**.
- :func:`contradictions_solve` — CONTRADICTIONS contract.
  Exhaustive Apriori-gen. Collects every dead commitment into
  ``proof.dead_commitments``. Returns :class:`Contradiction`
  always (``unsat_core`` = union of every dead core +
  learned nogoods). Backbone: **S1.5b.23**.

**Orthogonal ``store_lattice`` flag.** On :func:`gaps_solve`
+ :func:`contradictions_solve`, opts into per-SetNode
:attr:`LatticeProof.kb_index` storage. Under
:func:`contradictions_solve` this enables state-hash dedup
*merge* (distinct dead commitments with identical
post-saturation kbs collapse). Under :func:`gaps_solve`
the storage is built but the merge is **auto-disabled** —
distinct satisfying commitments must register separately per
the GAPS contract. :func:`monotonic_solve` doesn't take the
flag (no storage by design).

**Algorithm sketch (shared by all three).** Single
:class:`KnowledgeBase` instance (root); commitment sets
generated layer-by-layer via the Apriori prefix-join
(:mod:`ein_bot.inference.apriori`) and entered via the common
:func:`ein_bot.inference.commitment.try_commitment_set`
primitive. Flat root-writes on every outcome (no per-parent
bubble; saturation commutativity makes the bubble
mechanically redundant — see ``algorithm_layer_n.md`` § What
this algorithm no longer does).

**Augmentations** (default-on, gated by SolverConfig):

- CDCL nogoods — every dead entering emits ``frozenset(C)``
  into ``root_kb._nogoods``; subsequent layers' candidate
  filter (:func:`ein_bot.inference.nogoods.matches_any_nogood`)
  catches supersets.
- Singleton-death writeback — for size-1 dead clauses, writes
  ``(not h)`` to ``root_kb._negated_facts`` so subsequent
  ``_compute_alive`` drops h.
- Forced-positive promotion — when alive shrinks to a
  singleton, promote it to a root fact.

**Diagnostics.** Two dumper classes:
:class:`MonotonicDumper` (S1.5b.7) for monotonic_solve;
:class:`LatticeDumper` (S1.5b.29 — stub today) for
gaps_solve + contradictions_solve. Both share the lifecycle
hook shape; the lattice dumper adds entry-specific sections
(``solutions/``, ``dead/``, ``kb_index/``).

See [P1.5b README](../../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/README.md)
+ ``project_set_search_unified`` memory for the design
rationale.
"""
from ein_bot.inference.monotonic.contract import (
    validate_proof_for_explanation,
    verdict_entry,
)
from ein_bot.inference.monotonic.lattice import (
    DeadCommitment,
    LatticeProof,
    LatticeStats,
    SetNode,
    SolutionRecord,
)
from ein_bot.inference.monotonic.solver import (
    BudgetExceededError,
    MonotonicStats,
    contradictions_solve,
    gaps_solve,
    monotonic_solve,
)
from ein_bot.inference.monotonic.state_dump import LatticeDumper

__all__ = [
    "BudgetExceededError",
    "DeadCommitment",
    "LatticeDumper",
    "LatticeProof",
    "LatticeStats",
    "MonotonicStats",
    "SetNode",
    "SolutionRecord",
    "contradictions_solve",
    "gaps_solve",
    "monotonic_solve",
    "validate_proof_for_explanation",
    "verdict_entry",
]
