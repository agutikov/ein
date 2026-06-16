"""Lattice-engine skeleton tests — S1.5b.20 T1.5b.20.3 (P1.7a refit).

Pins the unified engine's single public entry (:func:`solve`, in
:mod:`ein.inference.monotonic`) + the LatticeProof data-class surface
and the LatticeDumper class shape.

2026-06-16 — the former ``gaps_solve`` / ``contradictions_solve``
sibling entries were removed (they chose the verdict by which function
was called, regardless of the actual model count). The lattice views
(solution set / refutation map) are now read off the one
``solve(..., store_lattice=True)`` result's :class:`LatticeProof`.

Cross-references:

- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.20_lattice_skeleton.md``
- Sibling tests (monotonic, already shipped):
  ``ein.py/tests/inference/monotonic/test_monotonic_skeleton.py``
"""
from __future__ import annotations

from ein.inference.monotonic import (
    DeadCommitment,
    LatticeDumper,
    LatticeProof,
    LatticeStats,
    SetNode,
    SolutionRecord,
    solve,
)
from ein.kb.store import KnowledgeBase


def test_imports_resolve():
    """Every public symbol resolves through the package init."""
    assert callable(solve)
    # Data class names exist (S1.5b.22 fills the fields).
    assert LatticeProof is not None
    assert SolutionRecord is not None
    assert DeadCommitment is not None
    assert SetNode is not None
    assert LatticeStats is not None
    # Dumper class exists (S1.5b.29 fills the hooks).
    assert LatticeDumper is not None


def test_solve_on_empty_kb_is_trivial_solution():
    """``solve`` on an empty kb: no hypothesis is open
    (``open_hypotheses`` is empty) and there is no contradiction, so the
    empty kb is itself complete ∧ consistent — one trivial solution node
    (``k == 1``) → :class:`Solution`. No commitment is tried
    (``enterings_total == 0``). The proof rides along under
    ``store_lattice``. Detailed contract tests live in
    ``test_contradictions_backbone.py``."""
    from ein.inference.verdict import Solution
    kb = KnowledgeBase()
    verdict, stats = solve(kb, stop_after=None, store_lattice=True)
    assert isinstance(verdict, Solution)
    assert verdict.proof is not None
    assert stats.enterings_total == 0


def test_dumper_no_op_hooks():
    """LatticeDumper's hooks are no-ops when ``out_dir=None``.

    The class shape exists so :mod:`bench_lattice` and the
    public entries can accept a ``dumper`` parameter without
    requiring a write target. The dumper restructure (post-
    S1.5b.30) folded the old ``solution_recorded`` /
    ``dead_recorded`` per-outcome hooks into a single
    ``entering`` hook with an ``outcome`` kwarg.
    """
    dumper = LatticeDumper()
    # Each hook accepts its expected args without raising.
    dumper.root_initial(None)
    dumper.layer_start(1, None, 0)
    dumper.entering(
        1, (), None,
        outcome="alive",
        facts_merged=0,
        nogood_emitted=False,
        nogood_subsumed=False,
    )
    dumper.layer_end(1, None, 0, 0)
    dumper.proof_summary(None)
    dumper.summary(None, None)
    dumper.close()


# ── LatticeProof smoke check ───────────────────────────────
#
# S1.5b.22 shipped — LatticeProof now carries real fields. The
# detailed proof-shape contract tests live in
# ``test_lattice_proof.py``. The check below stays as a minimal
# smoke check that the public surface is wired.


def test_lattice_proof_carries_solutions():
    """Smoke: ``solve(kb, store_lattice=True).proof`` is a
    :class:`LatticeProof` with populated fields. branching/04 has TWO
    distinct models (Blue↔H3, Green↔H3) → :class:`Ambiguity` (k=2), so
    ``proof.solutions`` carries both. Detailed shape tests in
    ``test_lattice_proof.py``."""
    from pathlib import Path

    from ein.inference.verdict import Ambiguity
    from ein.ir import parse
    repo = Path(__file__).resolve().parents[4]
    kb = KnowledgeBase.from_ir(
        parse((repo / "examples" / "branching"
               / "04_two_levels.ein").read_text()),
    )
    verdict, _ = solve(kb, stop_after=None, max_set_size=3, store_lattice=True)
    assert isinstance(verdict, Ambiguity)
    assert isinstance(verdict.proof, LatticeProof)
    assert len(verdict.proof.solutions) == 2
    for rec in verdict.proof.solutions:
        assert isinstance(rec, SolutionRecord)
