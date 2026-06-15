"""P1.6 handoff contract tests — S1.5b.29 T1.5b.29.5.

Calls :func:`validate_proof_for_explanation` on the verdict's
proof for every combination of lattice entry x store_lattice on
small fixtures. The validator's body is in
:mod:`ein.inference.monotonic.contract`; this file pins
the contract holds end-to-end on real solver runs.

Cross-references:

- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.29_lattice_proof.md``
- Validator implementation:
  ``ein.py/src/ein/inference/monotonic/contract.py``
- P1.6 consumer side:
  ``plans/m1_core_graph_reasoning/p1.6_rendering_and_trace/``
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.inference.monotonic import (
    contradictions_solve,
    gaps_solve,
)
from ein.inference.monotonic.contract import (
    validate_proof_for_explanation,
)
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


# ── gaps_solve x {store_lattice: F/T} on branching/04 ─────


def test_p16_contract_gaps_branching_04():
    """``gaps_solve`` on branching/04 — 2 distinct solutions
    enumerated. Contract validates."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
    assert len(verdict.proof.solutions) == 2


def test_p16_contract_gaps_branching_04_store_lattice():
    """``gaps_solve --store-lattice`` on branching/04 — kb_index
    populated but no multi-label SetNodes (gaps contract).
    Validator confirms."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(
        kb, max_set_size=3, store_lattice=True,
    )
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
    assert len(verdict.proof.kb_index) > 0
    # Under gaps, no SetNode may have len(labels) > 1.
    for node in verdict.proof.kb_index.values():
        assert len(node.labels) == 1, (
            f"gaps SetNode with multi-label: {node.labels}"
        )


def test_p16_contract_gaps_branching_05():
    """``gaps_solve`` on branching/05 — ONE distinct solution (the
    puzzle has a unique model). Its snapshotted kb satisfies is_solved.

    S1.7.24 — was 3 pre-dedup (the same model via three commitment
    paths); gaps now dedups ``proof.solutions`` by state_hash to
    distinct MODELS."""
    kb = _kb_from(BRANCHING / "05_mini_zebra.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
    assert len(verdict.proof.solutions) == 1


# ── contradictions_solve x {store_lattice: F/T} on branching/04 ──


def test_p16_contract_contradictions_branching_04():
    """``contradictions_solve`` on branching/04 — 4 deads;
    learned_nogoods subsumes every per-record clause."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = contradictions_solve(kb, max_set_size=3)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
    assert len(verdict.proof.dead_commitments) >= 1


def test_p16_contract_contradictions_branching_04_store_lattice():
    """``contradictions_solve --store-lattice`` on branching/04 —
    kb_index populated; multi-label SetNodes permitted under
    this entry (no natural collisions on this fixture so each
    node has len(labels)==1, but the validator only forbids
    multi-label under non-contradictions verdicts)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = contradictions_solve(
        kb, max_set_size=3, store_lattice=True,
    )
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)


# ── Phase-1 root contradiction ────────────────────────────


def test_p16_contract_root_contradiction():
    """A puzzle whose Phase 1 root saturates to ``(false)`` —
    empty solutions / empty deads. Validator passes (no
    invariants are violated by emptiness)."""
    kb = KnowledgeBase.from_ir(parse("""
    (rule always-false ()
      :match (trigger ?x)
      :assert (false)
      :why "always" :priority 100)
    (type T)
    (relation trigger T)
    (instance a T)
    (trigger a :source "(1)")
    (query :mode solve :goal (trigger ?x))
    """))
    verdict, _ = contradictions_solve(kb, max_set_size=1)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)


# ── Negative test — the validator catches violations ──────


def test_p16_contract_rejects_mismatched_proof():
    """``verdict.proof is proof`` identity check fires when a
    caller passes the wrong proof object."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3)
    # Re-run to get a different proof instance.
    kb2 = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict2, _ = gaps_solve(kb2, max_set_size=3)
    with pytest.raises(AssertionError, match=r"verdict\.proof"):
        validate_proof_for_explanation(verdict, verdict2.proof)


def test_p16_contract_stats_solutions_found_matches():
    """``stats.solutions_found`` is wired correctly (the
    coherence check in the validator)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, stats = gaps_solve(kb, max_set_size=3)
    assert stats.solutions_found == len(verdict.proof.solutions)
    validate_proof_for_explanation(verdict, verdict.proof)
