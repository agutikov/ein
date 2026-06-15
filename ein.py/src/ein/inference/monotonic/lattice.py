"""Lattice data structures for the unified set-search engine.

The :func:`gaps_solve` and :func:`contradictions_solve` entries
(siblings of :func:`solve` in this same package)
return verdicts wrapping a :class:`LatticeProof` artefact. This
module defines the proof's data shape + the per-record companion
types + the cumulative-counters class :class:`LatticeStats`.

Filled by which entry — quick lookup
------------------------------------

================  ==========================  ============================
Field             Filled by                   Notes
================  ==========================  ============================
solutions          gaps_solve                 Every satisfying commitment.
dead_commitments   contradictions_solve       Every refuted commitment.
kb_index           store_lattice=True only    Under gaps: keyed per
                                              commitment (no merge);
                                              under contradictions: keyed
                                              by post-saturation
                                              :func:`state_hash`
                                              (collisions merge labels).
alive_at_end       both                       Size-N alive sets at depth
                                              cap; ``()`` if not capped.
learned_nogoods    both                       Snapshot of
                                              ``root_kb._nogoods`` at
                                              return.
stats              both                       Cumulative counters.
================  ==========================  ============================

Cross-references:

- Algorithm spec (which field is filled by which entry):
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md``
  § Verdict synthesis.
- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.22_lattice_dedup.md``.
- Engine layout rationale (one engine, three entries, unified
  monotonic+lattice in ``inference/monotonic/``):
  ``project_set_search_unified`` memory + the 2026-05-28
  conversation.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal

from ein.inference.apriori import CanonicalSetId
from ein.inference.firing import Firing
from ein.kb.entities import Fact
from ein.kb.provenance import FactId
from ein.kb.store import KnowledgeBase


@dataclass
class _BaseStats:
    """Per-candidate counters shared by every set-search entry — the
    base of both :class:`MonotonicStats` (``solve``, in
    :mod:`ein.inference.monotonic.solver`) and :class:`LatticeStats`
    (``gaps`` / ``contradictions``).

    Factoring these here (F-ENG-9) retires a hand-maintained field-copy
    in ``solver._build_lattice_stats`` that silently went stale when a
    counter was added to one stats class but not the other. A counter
    added here now flows into both subclasses — and the generic copy —
    automatically.
    """

    enterings_total:     int = 0
    enterings_alive:     int = 0
    enterings_dead_pre:  int = 0
    enterings_dead_post: int = 0
    facts_merged:        int = 0
    forced_positives:    int = 0
    saturate_count:      int = 0
    layers_explored:     int = 0
    # S1.5b.6 — CDCL counters.
    nogoods_emitted:     int = 0
    nogoods_subsumed:    int = 0


@dataclass
class LatticeStats(_BaseStats):
    """Cumulative counters for one :func:`gaps_solve` /
    :func:`contradictions_solve` run.

    Inherits the shared per-candidate counters from :class:`_BaseStats`;
    adds the lattice-only :attr:`solutions_found`,
    :attr:`state_hash_merges`, and :attr:`elapsed_seconds`.
    :class:`MonotonicStats` (in
    :mod:`ein.inference.monotonic.solver`) is the sibling subclass —
    neither inherits the other, so each public entry advertises its own
    full counter set at the type level.

    NB: under the :class:`_BaseStats` split, :attr:`solutions_found` now
    serialises *after* the shared counters (it previously sat at field
    index 4). Every consumer reads stats by field name, so this is
    observable only in the key order of a dumper's ``summary.json`` stats
    block — which nothing asserts on.
    """

    solutions_found:     int = 0
    state_hash_merges:   int = 0
    elapsed_seconds:     float = 0.0


@dataclass(frozen=True)
class SolutionRecord:
    """One satisfying commitment + its saturated kb snapshot.

    Filled by :func:`gaps_solve` for every satisfying commitment
    encountered (the only persisted per-set kb — alive
    intermediates are not stored). The :attr:`kb` is a
    :meth:`KnowledgeBase.snapshot` so it stays stable across
    later mutations of root.

    :attr:`commitment` is the empty tuple ``()`` when root itself
    satisfied (Phase 1 short-circuit, or a forced-positive
    cascade landing root in the solved state).
    """

    commitment: CanonicalSetId
    kb:         KnowledgeBase
    firings:    tuple[Firing, ...]
    layer:      int


@dataclass(frozen=True)
class DeadCommitment:
    """One refuted commitment + its unsat-core.

    Filled by :func:`contradictions_solve` for every dead
    commitment encountered (backbone lands in S1.5b.23).
    ``kind`` records whether the contradiction surfaced
    pre-saturation (``dead-pre``) or only after rule firing
    (``dead-post``).
    """

    commitment:     CanonicalSetId
    unsat_core:     frozenset[Fact]
    learned_clause: frozenset[FactId]
    layer:          int
    kind:           Literal["dead-pre", "dead-post"]
    # S1.7.23/.24 — `state_hash` of the (dead) post-saturation kb. The
    # orientation-invariant key for result-level snapshots: two
    # orientations of a symmetric dead commitment saturate to the same
    # dead state. Defaults to 0 for records predating the field.
    state_hash:     int = 0


@dataclass(frozen=True)
class SetNode:
    """One cross-set state-hash merge target.

    Populated when ``store_lattice=True``. Under
    :func:`contradictions_solve` multiple labels may collapse
    into one node (state-hash dedup merge — distinct dead
    commitments that saturate to identical kbs share a
    refutation node); under :func:`gaps_solve` each commitment
    keeps its own node (no merge — GAPS contract requires
    distinct satisfying commitments to register separately).

    :attr:`state_hash` always carries
    :func:`ein.inference.canon.state_hash` of the
    post-saturation kb regardless of how the enclosing
    ``kb_index`` dict is keyed.
    """

    state_hash:    int
    canonical_set: CanonicalSetId
    labels:        tuple[CanonicalSetId, ...]
    verdict:       Literal["alive", "dead", "solution"]
    layer:         int


@dataclass(frozen=True)
class LatticeProof:
    """Set-search engine's proof artefact for non-monotonic entries.

    Returned in ``Ambiguity.proof`` from :func:`gaps_solve` or
    ``Contradiction.proof`` from :func:`contradictions_solve`.
    Carries the entry-specific collected records plus the
    orthogonal ``store_lattice``-gated SetNode storage and the
    pair of derivation-state snapshots
    (:attr:`learned_nogoods`, :attr:`alive_at_end`) that survive
    the return.

    The :attr:`kb_index` dict is *empty* when ``store_lattice``
    is False (default) — distinct from "loop ran but found
    nothing" (which is signalled by zero-length
    :attr:`solutions` / :attr:`dead_commitments`). The dict
    keying differs by entry: under :func:`gaps_solve` the keys
    are per-commitment unique ids (so distinct commitments stay
    separate); under :func:`contradictions_solve` the keys are
    :func:`state_hash` values (so distinct commitments collapse
    on collision).
    """

    solutions:        tuple[SolutionRecord, ...]      = ()
    dead_commitments: tuple[DeadCommitment, ...]      = ()
    kb_index:         dict[int, SetNode]              = field(default_factory=dict)
    alive_at_end:     tuple[CanonicalSetId, ...]      = ()
    learned_nogoods:  frozenset[frozenset[FactId]]    = frozenset()
    stats:            LatticeStats                    = field(default_factory=LatticeStats)
