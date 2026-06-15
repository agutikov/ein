"""Lattice shuffle-invariance harness — S1.5b.31 T1.5b.31.3.

Verifies that the lattice engine's per-layer traversal order
does NOT leak into the final lattice content. For every
(puzzle, max_set_size, seed) triple, the shuffled run's
:class:`LatticeSnapshotV1` must compare equal to the reference
(unshuffled) run's snapshot. A mismatch indicates one of three
likely leak sources:

- forced-positive integration order (the order in which a
  layer's alive commitments merge their unconditional facts
  into root_kb),
- nogood subsumption order (the order in which dead clauses
  hit ``emit_nogood``),
- multilabel representative-id leak (a SetNode field's
  first-arrival semantics propagating into the snapshot).

The snapshot serialiser at
:mod:`ein.inference.monotonic.snapshot` canonicalises the
multilabel reps, so a real failure here points at one of the
first two suspects.

Cross-references:

- Tree-side sibling:
  ``ein.py/tests/inference/test_shuffle_invariance.py``.
- Sister stage on per-set commutativity:
  ``ein.py/tests/inference/lattice/test_lattice_sanity.py``
  (S1.5b.27).
"""
from __future__ import annotations

from dataclasses import replace
from pathlib import Path

import pytest

from ein.inference.config import SolverConfig
from ein.inference.monotonic import (
    contradictions_solve,
    gaps_solve,
    lattice_snapshot,
)
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"
LATTICE = REPO / "examples" / "lattice"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


_SEEDS = list(range(10))
_FIXTURES = [
    BRANCHING / "04_two_levels.ein",
    BRANCHING / "05_mini_zebra.ein",
    LATTICE / "01_subset_pruned.ein",
    LATTICE / "02_genuine_3set_death.ein",
    LATTICE / "03_state_hash_collision.ein",
]


# ── gaps_solve shuffle invariance ────────────────────────


@pytest.mark.parametrize("fixture", _FIXTURES)
@pytest.mark.parametrize("max_set_size", [1, 2, 3])
@pytest.mark.parametrize("seed", _SEEDS)
def test_gaps_shuffle_invariance(
    fixture: Path, max_set_size: int, seed: int,
):
    """For each ``(fixture, max_set_size, seed)`` triple,
    :func:`gaps_solve` with ``lattice_order_seed=seed``
    produces a :class:`LatticeSnapshotV1` equal to the
    reference (unshuffled) snapshot.

    Solution-list ORDER inside ``proof.solutions`` may differ
    between shuffled runs (set-vs-tuple), but the
    snapshot's ``solutions`` field is a frozenset — so the
    SET of satisfying commitments must be identical.
    """
    # Reference: default order.
    kb_ref = _kb_from(fixture)
    verdict_ref, _ = gaps_solve(
        kb_ref, max_set_size=max_set_size, store_lattice=True,
    )
    snap_ref = lattice_snapshot(verdict_ref, kb_ref)

    # Shuffled.
    kb = _kb_from(fixture)
    cfg = replace(kb.config or SolverConfig(), lattice_order_seed=seed)
    verdict, _ = gaps_solve(
        kb, max_set_size=max_set_size,
        store_lattice=True, config=cfg,
    )
    snap = lattice_snapshot(verdict, kb)

    assert snap == snap_ref, (
        f"shuffle leak on gaps_solve({fixture.name}, "
        f"max_set_size={max_set_size}, seed={seed}):\n"
        f"  ref nodes={len(snap_ref.nodes_by_state_hash)} "
        f"vs shuffle nodes={len(snap.nodes_by_state_hash)}\n"
        f"  ref solutions={snap_ref.solutions}\n"
        f"  shuffle solutions={snap.solutions}"
    )


# ── contradictions_solve shuffle invariance ──────────────


@pytest.mark.parametrize("fixture", _FIXTURES)
@pytest.mark.parametrize("max_set_size", [1, 2, 3])
@pytest.mark.parametrize("seed", _SEEDS)
def test_contradictions_shuffle_invariance(
    fixture: Path, max_set_size: int, seed: int,
):
    """For each ``(fixture, max_set_size, seed)`` triple,
    :func:`contradictions_solve` with
    ``lattice_order_seed=seed`` produces a snapshot equal to
    the unshuffled reference."""
    kb_ref = _kb_from(fixture)
    verdict_ref, _ = contradictions_solve(
        kb_ref, max_set_size=max_set_size, store_lattice=True,
    )
    snap_ref = lattice_snapshot(verdict_ref, kb_ref)

    kb = _kb_from(fixture)
    cfg = replace(kb.config or SolverConfig(), lattice_order_seed=seed)
    verdict, _ = contradictions_solve(
        kb, max_set_size=max_set_size,
        store_lattice=True, config=cfg,
    )
    snap = lattice_snapshot(verdict, kb)

    assert snap == snap_ref, (
        f"shuffle leak on contradictions_solve({fixture.name}, "
        f"max_set_size={max_set_size}, seed={seed}):\n"
        f"  ref nodes={len(snap_ref.nodes_by_state_hash)} "
        f"vs shuffle nodes={len(snap.nodes_by_state_hash)}\n"
        f"  ref deads={snap_ref.deads}\n"
        f"  shuffle deads={snap.deads}"
    )


# ── Sanity: snapshot serialiser invariants ───────────────


def test_lattice_snapshot_requires_proof():
    """``lattice_snapshot`` rejects a verdict whose ``proof``
    is None (the ``solve`` fast-path case, which carries no
    :class:`LatticeProof`)."""
    from ein.inference.monotonic import solve
    from ein.inference.verdict import Solution

    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    verdict, _ = solve(kb)
    assert isinstance(verdict, Solution)
    assert verdict.proof is None
    with pytest.raises(ValueError, match=r"LatticeProof"):
        lattice_snapshot(verdict, kb)


def test_lattice_snapshot_is_hashable():
    """:class:`LatticeSnapshotV1` is a frozen dataclass — must
    be hashable so test harnesses can pool snapshots into a
    set."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(
        kb, max_set_size=3, store_lattice=True,
    )
    snap = lattice_snapshot(verdict, kb)
    # ``hash()`` must succeed — no unhashable nested types.
    assert isinstance(hash(snap), int)


def test_lattice_snapshot_default_seed_idempotent():
    """Re-running gaps_solve with ``lattice_order_seed=None``
    twice produces identical snapshots (the default ordering
    is deterministic regardless of randomisation)."""
    kb1 = _kb_from(BRANCHING / "04_two_levels.ein")
    v1, _ = gaps_solve(kb1, max_set_size=3, store_lattice=True)
    snap1 = lattice_snapshot(v1, kb1)

    kb2 = _kb_from(BRANCHING / "04_two_levels.ein")
    v2, _ = gaps_solve(kb2, max_set_size=3, store_lattice=True)
    snap2 = lattice_snapshot(v2, kb2)

    assert snap1 == snap2
