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
    # gaps_solve is no longer a stub (S1.5b.21 shipped the
    # backbone). The contract tests for gaps_solve live in
    # test_gaps_backbone.py.


def test_contradictions_solve_callable_post_s1_5b_23():
    """S1.5b.23 lifted the NotImplementedError stub. The
    public entry is now callable on an empty kb; the verdict
    is :class:`Contradiction` (mode contract — always) with
    empty ``proof.dead_commitments`` for a kb that has no
    hypothesis-bearing relations. Detailed contradictions
    contract tests live in ``test_contradictions_backbone.py``."""
    from ein_bot.inference.verdict import Contradiction
    kb = KnowledgeBase()
    verdict, stats = contradictions_solve(kb)
    assert isinstance(verdict, Contradiction)
    assert verdict.proof is not None
    assert verdict.proof.dead_commitments == ()
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


# ── Placeholders for the feature stages ────────────────────
#
# S1.5b.22 shipped — LatticeProof now carries real fields. The
# detailed proof-shape contract tests live in
# ``test_lattice_proof.py``. The placeholder below stays as a
# minimal smoke check that the public surface is wired.


def test_lattice_proof_carries_solutions_under_gaps():
    """Smoke: ``gaps_solve(kb).proof`` is a :class:`LatticeProof`
    with the populated fields. Detailed shape tests in
    ``test_lattice_proof.py``."""
    from pathlib import Path

    from ein_bot.ir import parse
    repo = Path(__file__).resolve().parents[4]
    kb = KnowledgeBase.from_ir(
        parse((repo / "examples" / "branching"
               / "04_two_levels.ein").read_text()),
    )
    verdict, _ = gaps_solve(kb, max_set_size=3)
    assert isinstance(verdict.proof, LatticeProof)
    assert len(verdict.proof.solutions) == 2
    for rec in verdict.proof.solutions:
        assert isinstance(rec, SolutionRecord)


# S1.5b.23 shipped — contradictions_solve is wired. Detailed
# contract tests live in ``test_contradictions_backbone.py``.
