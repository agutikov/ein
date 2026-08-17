"""Lattice data structures for the unified set-search engine.

The single :func:`solve` entry attaches a :class:`LatticeProof`
artefact to its verdict when called with ``store_lattice=True``. This
module defines the proof's data shape + the per-record companion
types + the cumulative-counters class :class:`LatticeStats`. The proof
carries BOTH views off the one run: the gaps view is
:attr:`LatticeProof.solutions`, the contradictions view is
:attr:`LatticeProof.dead_commitments` (+ the verdict's ``unsat_core``).

Field — quick lookup
--------------------

================  ==========================  ============================
Field             Filled by                   Notes
================  ==========================  ============================
solutions          solve (store_lattice)      Every satisfying commitment
                                              (the gaps view).
dead_commitments   solve (store_lattice)      Every refuted commitment
                                              (the contradictions view).
kb_index           store_lattice=True only    Empty under :func:`solve`'s
                                              own packaging; populated only
                                              by a DAG builder via
                                              :func:`_record_setnode`,
                                              keyed per commitment (gaps) or
                                              by post-saturation
                                              :func:`state_key`
                                              (contradictions, same-state
                                              arrivals merge labels).
alive_at_end       solve (store_lattice)      Size-N alive sets at depth
                                              cap; ``()`` if not capped.
learned_nogoods    solve (store_lattice)      Snapshot of
                                              ``root_kb._nogoods`` at
                                              return.
stats              solve (store_lattice)      Cumulative counters.
================  ==========================  ============================

Cross-references:

- Algorithm spec (per-step contract):
  ``docs/kernel/inference/algorithm_layer_n.md``
  § Verdict synthesis.
- Stage spec:
  ``M1 S1.5b.22``.
- Engine layout rationale (one engine, one entry, unified
  monotonic+lattice in ``inference/monotonic/``):
  ``project_set_search_unified`` memory + the 2026-05-28
  conversation.
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal

from ein.inference.apriori import CanonicalSetId
from ein.inference.canon import StateKey
from ein.inference.firing import Firing
from ein.kb.entities import Fact
from ein.kb.provenance import FactId
from ein.kb.store import KnowledgeBase


@dataclass
class _BaseStats:
    """Per-candidate counters shared by the set-search engine — the
    base of both :class:`MonotonicStats` (the :func:`solve` run, in
    :mod:`ein.inference.monotonic.solver`) and :class:`LatticeStats`
    (the ``store_lattice`` proof's stats).

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
    # S1.5b.6 — no-good counters.
    nogoods_emitted:     int = 0
    nogoods_subsumed:    int = 0


@dataclass
class LatticeStats(_BaseStats):
    """Cumulative counters for the ``store_lattice`` proof of one
    :func:`solve` run.

    Inherits the shared per-candidate counters from :class:`_BaseStats`;
    adds the lattice-only :attr:`solutions_found`,
    :attr:`state_key_merges`, and :attr:`elapsed_seconds`.
    :class:`MonotonicStats` (in
    :mod:`ein.inference.monotonic.solver`) is the sibling subclass —
    neither inherits the other, so the run-level stats and the proof's
    stats each advertise their own full counter set at the type level.

    NB: under the :class:`_BaseStats` split, :attr:`solutions_found` now
    serialises *after* the shared counters (it previously sat at field
    index 4). Every consumer reads stats by field name, so this is
    observable only in the key order of a dumper's ``summary.json`` stats
    block — which nothing asserts on.
    """

    solutions_found:     int = 0
    state_key_merges:    int = 0
    elapsed_seconds:     float = 0.0


@dataclass(frozen=True)
class SolutionRecord:
    """One satisfying commitment + its saturated kb snapshot.

    Collected by :func:`solve` (and surfaced in the
    ``store_lattice`` proof's ``solutions`` — the gaps view) for
    every satisfying commitment encountered (the only persisted
    per-set kb — alive intermediates are not stored). The
    :attr:`kb` is a :meth:`KnowledgeBase.snapshot` so it stays
    stable across later mutations of root.

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

    Collected by :func:`solve` (and surfaced in the
    ``store_lattice`` proof's ``dead_commitments`` — the
    contradictions view) for every dead commitment encountered
    (backbone lands in S1.5b.23). ``kind`` records whether the
    contradiction surfaced pre-saturation (``dead-pre``) or only
    after rule firing (``dead-post``).
    """

    commitment:     CanonicalSetId
    unsat_core:     frozenset[Fact]
    learned_clause: frozenset[FactId]
    layer:          int
    kind:           Literal["dead-pre", "dead-post"]
    # S1.7.23/.24, reworked P1.21 R1 — `state_key` of the (dead)
    # post-saturation kb. The orientation-invariant key for result-level
    # snapshots: two orientations of a symmetric dead commitment saturate
    # to the same dead state. Defaults to `()` for records predating the
    # field.
    #
    # NOTE(P1.21 R1): storing the full canonical key costs ~70 KiB per
    # dead on zebra2 (67 deads ≈ 5 MiB — noise next to the per-record kb
    # snapshots). If a future puzzle records >>1e3 dead states, demote
    # this field to a display digest (`canon.state_digest`) plus an
    # on-demand key — but a digest must NEVER become identity again.
    state_key:      StateKey = ()


@dataclass(frozen=True)
class SetNode:
    """One cross-set same-state merge target.

    Populated by a DAG builder via :func:`_record_setnode` (not by
    :func:`solve`'s own proof packaging). In the
    ``contradictions`` keying mode multiple labels may collapse
    into one node (state-key dedup merge — distinct dead
    commitments that saturate to identical kbs share a
    refutation node); in the ``gaps`` keying mode each commitment
    keeps its own node (no merge — distinct satisfying
    commitments register separately).

    :attr:`state_key` always carries
    :func:`ein.inference.canon.state_key` of the
    post-saturation kb regardless of how the enclosing
    ``kb_index`` dict is keyed.
    """

    state_key:     StateKey
    canonical_set: CanonicalSetId
    labels:        tuple[CanonicalSetId, ...]
    verdict:       Literal["alive", "dead", "solution"]
    layer:         int


@dataclass(frozen=True)
class LatticeProof:
    """Set-search engine's proof artefact.

    Attached to the verdict (``Solution`` / ``Ambiguity`` /
    ``Contradiction``) by :func:`solve` when called with
    ``store_lattice=True``. Carries BOTH views off the one run —
    :attr:`solutions` (the gaps view) and :attr:`dead_commitments`
    (the contradictions view, paired with the verdict's
    ``unsat_core``) — plus the orthogonal SetNode storage and the
    pair of derivation-state snapshots
    (:attr:`learned_nogoods`, :attr:`alive_at_end`) that survive
    the return.

    The :attr:`kb_index` dict is *empty* in :func:`solve`'s own
    packaging — distinct from "loop ran but found nothing" (which
    is signalled by zero-length
    :attr:`solutions` / :attr:`dead_commitments`). When a DAG
    builder populates it via :func:`_record_setnode`, the keying
    differs by mode: in the ``gaps`` mode the keys are the
    commitment tuples themselves (so distinct commitments stay
    separate); in the ``contradictions`` mode the keys are
    :func:`state_key` values (so distinct commitments reaching
    the same state collapse — an exact-equality merge, never a
    hash-collision one).
    """

    solutions:        tuple[SolutionRecord, ...]           = ()
    dead_commitments: tuple[DeadCommitment, ...]           = ()
    kb_index:         dict[StateKey | CanonicalSetId, SetNode] = field(
        default_factory=dict,
    )
    alive_at_end:     tuple[CanonicalSetId, ...]      = ()
    learned_nogoods:  frozenset[frozenset[FactId]]    = frozenset()
    stats:            LatticeStats                    = field(default_factory=LatticeStats)
