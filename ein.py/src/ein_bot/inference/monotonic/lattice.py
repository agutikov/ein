"""Lattice data structures for the unified set-search engine.

The :func:`gaps_solve` and :func:`contradictions_solve` entries
(siblings of :func:`monotonic_solve` in this same package)
return verdicts wrapping a :class:`LatticeProof` artefact.
This module defines the proof's data shape + the per-record
companion types.

**Skeleton stage — S1.5b.20.** Empty :class:`dataclass` stubs
so imports resolve. S1.5b.22 fills each class with its real
fields + ``with_*`` builder helpers used during the loop.
Until S1.5b.22 lands, do not depend on field layout — these
stubs only exist to keep the package's import graph
well-formed and to give :mod:`bench_lattice` + the future
entry implementations a typed surface to import against.

Cross-references:

- Target shapes per class:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.22_lattice_dedup.md``
- Algorithm spec (which class is filled by which entry):
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md``
  § Verdict synthesis.
- Engine layout rationale (one engine, three entries,
  unified monotonic+lattice in ``inference/monotonic/``):
  ``project_set_search_unified`` memory + the 2026-05-28
  conversation.
"""
from __future__ import annotations

from dataclasses import dataclass


@dataclass
class LatticeStats:
    """Cumulative counters for one :func:`gaps_solve` /
    :func:`contradictions_solve` run.

    S1.5b.22 will populate with: enterings_total,
    enterings_alive, enterings_dead_pre, enterings_dead_post,
    solutions_found, facts_merged, forced_positives,
    saturate_count, layers_explored, nogoods_emitted,
    nogoods_subsumed, state_hash_merges, elapsed_seconds.

    Sibling to :class:`MonotonicStats` (in
    :mod:`ein_bot.inference.monotonic.solver`); the two
    overlap on most counters and may merge into one class
    end-of-phase.
    """


@dataclass(frozen=True)
class SolutionRecord:
    """One satisfying commitment + its saturated kb snapshot.

    Filled by :func:`gaps_solve` for every satisfying
    commitment encountered (the only persisted per-set kb —
    alive intermediates are not stored).

    S1.5b.22 will populate with: commitment, kb (snapshot),
    firings, layer.
    """


@dataclass(frozen=True)
class DeadCommitment:
    """One refuted commitment + its unsat-core.

    Filled by :func:`contradictions_solve` for every dead
    commitment encountered.

    S1.5b.22 will populate with: commitment, unsat_core,
    learned_clause, layer, kind.
    """


@dataclass(frozen=True)
class SetNode:
    """One cross-set state-hash merge target.

    Populated when ``store_lattice=True``. Under
    :func:`contradictions_solve` multiple labels may collapse
    into one node (state-hash dedup merge); under
    :func:`gaps_solve` each label keeps its own node (no
    merge — GAPS contract requires distinct satisfying
    commitments to register separately).

    S1.5b.22 will populate with: state_hash, canonical_set,
    labels, verdict, layer.
    """


@dataclass(frozen=True)
class LatticeProof:
    """Set-search engine's proof artefact for non-monotonic entries.

    Returned in ``Ambiguity.proof`` from :func:`gaps_solve` or
    ``Contradiction.proof`` from :func:`contradictions_solve`.
    Carries the entry-specific collected records plus the
    orthogonal ``store_lattice``-gated SetNode storage.

    S1.5b.22 will populate with: solutions,
    dead_commitments, kb_index, alive_at_end,
    learned_nogoods, stats.
    """
