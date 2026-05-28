"""Lattice-entry skeleton tests — S1.5b.20 T1.5b.20.3.

Pins the unified engine's two non-monotonic public entries
(:func:`gaps_solve`, :func:`contradictions_solve`, both
sitting alongside :func:`monotonic_solve` in
:mod:`ein_bot.inference.monotonic`) + the LatticeProof
data-class surface and the LatticeDumper class shape. Both
entries currently raise :class:`NotImplementedError`; the
backbones land in S1.5b.21 / S1.5b.23 respectively.

The three ``pytest.skip`` markers below name the stages that
fill in the real tests — they're placeholders so the test
file's growth is visible during the lattice block's
implementation rounds.

Cross-references:

- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.20_lattice_skeleton.md``
- Sibling tests (monotonic, already shipped):
  ``ein.py/tests/inference/monotonic/test_monotonic_skeleton.py``
"""
from __future__ import annotations

import pytest

from ein_bot.inference.monotonic import (
    DeadCommitment,
    LatticeDumper,
    LatticeProof,
    LatticeStats,
    SetNode,
    SolutionRecord,
    contradictions_solve,
    gaps_solve,
)
from ein_bot.kb.store import KnowledgeBase


def test_imports_resolve():
    """Every public symbol resolves through the package init."""
    assert callable(gaps_solve)
    assert callable(contradictions_solve)
    # Data class names exist (S1.5b.22 fills the fields).
    assert LatticeProof is not None
    assert SolutionRecord is not None
    assert DeadCommitment is not None
    assert SetNode is not None
    assert LatticeStats is not None
    # Dumper class exists (S1.5b.29 fills the hooks).
    assert LatticeDumper is not None


def test_gaps_solve_raises_notimplementederror():
    """S1.5b.21 fills the backbone."""
    kb = KnowledgeBase()
    with pytest.raises(NotImplementedError, match=r"S1\.5b\.21"):
        gaps_solve(kb)


def test_contradictions_solve_raises_notimplementederror():
    """S1.5b.23 fills the backbone."""
    kb = KnowledgeBase()
    with pytest.raises(NotImplementedError, match=r"S1\.5b\.23"):
        contradictions_solve(kb)


def test_dumper_no_op_hooks():
    """LatticeDumper's hooks are callable no-ops in S1.5b.20.

    The class shape exists so :mod:`bench_lattice` and the
    public entries can accept a ``dumper`` parameter. S1.5b.29
    fills the real file-writing implementation.
    """
    dumper = LatticeDumper()
    # Each hook accepts its expected args without raising.
    dumper.root_initial(None)
    dumper.layer_start(1, None, 0)
    dumper.entering(
        1, (), None,
        facts_merged=0,
        nogood_emitted=False,
        nogood_subsumed=False,
    )
    dumper.layer_end(1, None, 0, 0)
    dumper.solution_recorded(None, 1)
    dumper.dead_recorded(None)
    dumper.proof_summary(None)
    dumper.summary(None, None)
    dumper.close()


# ── Placeholders for the feature stages ────────────────────


@pytest.mark.skip(reason="gaps_solve backbone — S1.5b.21")
def test_gaps_solve_returns_ambiguity_on_solvable_puzzle():
    pass


@pytest.mark.skip(reason="LatticeProof data shapes — S1.5b.22")
def test_lattice_proof_carries_solutions_under_gaps():
    pass


@pytest.mark.skip(reason="contradictions_solve backbone — S1.5b.23")
def test_contradictions_solve_returns_contradiction():
    pass
