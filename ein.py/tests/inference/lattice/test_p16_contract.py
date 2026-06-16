"""P1.6 handoff contract tests — S1.5b.29 (P1.7a refit, 2026-06-16).

Calls :func:`validate_proof_for_explanation` on the verdict's proof for
real :func:`solve` runs (exhaustive, ``store_lattice=True``) across small
fixtures. The validator's body is in
:mod:`ein.inference.monotonic.contract`; this file pins that the contract
holds end-to-end. The verdict TYPE is read from ``k`` (the distinct
solution-node count), so the fixture's true nature drives the assertions:
branching/04 → :class:`Ambiguity` (k=2), branching/05 → :class:`Solution`
(k=1), a root-``(false)`` puzzle → :class:`Contradiction` (k=0).

Cross-references:

- Validator implementation:
  ``ein.py/src/ein/inference/monotonic/contract.py``
- P1.6 consumer side:
  ``plans/m1_core_graph_reasoning/p1.6_rendering_and_trace/``
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein.inference.monotonic import solve
from ein.inference.monotonic.contract import (
    validate_proof_for_explanation,
)
from ein.inference.verdict import Ambiguity, Solution
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _solve(kb: KnowledgeBase, **kw):
    return solve(kb, stop_after=None, store_lattice=True, **kw)


# ── branching/04 (k=2 → Ambiguity) ────────────────────────


def test_p16_contract_branching_04():
    """``solve`` on branching/04 — 2 distinct models enumerated
    (:class:`Ambiguity`). Contract validates."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    assert isinstance(verdict, Ambiguity)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
    assert len(verdict.proof.solutions) == 2


def test_p16_contract_branching_04_kb_index_empty():
    """``solve`` does not build the per-SetNode DAG, so ``proof.kb_index``
    is ``{}``; the validator still passes (empty kb_index is permitted)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
    assert verdict.proof.kb_index == {}


def test_p16_contract_branching_04_collects_deads():
    """``solve`` on branching/04 records the deads explored along the way
    (in ``proof.dead_commitments``) even though the verdict is
    :class:`Ambiguity`; learned_nogoods subsumes every per-record clause
    (checked by the validator)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
    assert len(verdict.proof.dead_commitments) >= 1


# ── branching/05 (k=1 → Solution) ─────────────────────────


def test_p16_contract_branching_05():
    """``solve`` on branching/05 — ONE distinct model (unique). Its
    snapshotted kb satisfies is_solved (checked by the validator)."""
    kb = _kb_from(BRANCHING / "05_mini_zebra.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    assert isinstance(verdict, Solution)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
    assert len(verdict.proof.solutions) == 1


# ── Phase-1 root contradiction ────────────────────────────


def test_p16_contract_root_contradiction():
    """A puzzle whose Phase 1 root saturates to ``(false)`` → k=0 →
    :class:`Contradiction`. ``solve`` packages the root core as a single
    root :class:`DeadCommitment` with the EMPTY commitment.

    TODO(P1.7a/P1.6): ``validate_proof_for_explanation`` clause 4 (every
    dead's ``learned_clause`` must be subsumed by some learned nogood)
    does NOT hold for this root dead — its ``learned_clause`` is the empty
    frozenset and ``_root_dead`` emits no nogood, so the empty clause is
    not subsumed. This is a real validator/``solve``-root-dead interaction
    that needs the engine owner's call (either ``_root_dead`` should emit
    a trivial nogood, or the validator should special-case the empty
    root-dead clause). Until then this test asserts only the verdict +
    proof-presence, NOT ``validate_proof_for_explanation`` on the root
    dead.
    """
    from ein.inference.verdict import Contradiction
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
    verdict, _ = _solve(kb, max_set_size=1)
    assert isinstance(verdict, Contradiction)
    assert verdict.proof is not None
    # Root dead recorded with the empty commitment + non-empty core.
    assert len(verdict.proof.dead_commitments) == 1
    assert verdict.proof.dead_commitments[0].commitment == ()
    assert verdict.unsat_core


# ── Negative test — the validator catches violations ──────


def test_p16_contract_rejects_mismatched_proof():
    """``verdict.proof is proof`` identity check fires when a caller passes
    the wrong proof object."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    # Re-run to get a different proof instance.
    kb2 = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict2, _ = _solve(kb2, max_set_size=3)
    with pytest.raises(AssertionError, match=r"verdict\.proof"):
        validate_proof_for_explanation(verdict, verdict2.proof)


def test_p16_contract_stats_solutions_found_matches():
    """``proof.stats.solutions_found`` is wired correctly (the coherence
    check in the validator)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    assert verdict.proof.stats.solutions_found == len(verdict.proof.solutions)
    validate_proof_for_explanation(verdict, verdict.proof)
