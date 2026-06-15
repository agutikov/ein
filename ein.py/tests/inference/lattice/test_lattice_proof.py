"""LatticeProof data-class + ``store_lattice`` wiring tests —
S1.5b.22 T1.5b.22.7.

Pins the GAPS-side LatticeProof contract:

- ``Ambiguity.proof.solutions`` is populated for every gaps
  return, regardless of ``store_lattice``.
- ``Ambiguity.proof.kb_index`` is empty under default flags
  and non-empty under ``store_lattice=True``.
- Under GAPS, ``state_hash_merges`` stays 0 regardless of
  whether two distinct commitments share a post-saturation
  state_hash (per-commitment keying keeps them separate).
- The SolutionRecord's kb is a :meth:`KnowledgeBase.snapshot`
  so later root mutations don't corrupt the branch view.
- ``alive_at_end`` carries the surviving size-N frontier
  when the depth cap is the natural loop terminator.

Cross-references:

- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.22_lattice_dedup.md``.
- Algorithm:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md``
  § Verdict synthesis.
- Sibling backbone tests:
  ``ein.py/tests/inference/lattice/test_gaps_backbone.py``.
"""
from __future__ import annotations

from pathlib import Path

from ein.inference.monotonic import (
    LatticeProof,
    LatticeStats,
    SetNode,
    SolutionRecord,
    gaps_solve,
)
from ein.inference.monotonic.solver import (
    _LatticeLoopState,
    _record_setnode,
)
from ein.inference.verdict import Ambiguity
from ein.ir import parse
from ein.kb.entities import Fact, Layer
from ein.kb.provenance import Provenance
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


# ── proof.solutions populated for every gaps return ──────────


def test_gaps_solve_proof_solutions_populated():
    """``branching/04_two_levels`` enumerates both Blue↔H3 and
    Green↔H3 — the verdict's ``proof.solutions`` carries both
    as SolutionRecord entries with kb snapshots."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, stats = gaps_solve(kb, max_set_size=3)
    assert isinstance(verdict, Ambiguity)
    assert isinstance(stats, LatticeStats)

    proof = verdict.proof
    assert isinstance(proof, LatticeProof)
    assert len(proof.solutions) == 2
    for rec in proof.solutions:
        assert isinstance(rec, SolutionRecord)
        # Each carries a kb snapshot — a KnowledgeBase instance
        # (not a fork's live reference).
        assert isinstance(rec.kb, KnowledgeBase)
        # Snapshot is independent: types is shared by reference
        # but facts is its own list.
        assert rec.kb.facts is not kb.facts

    # solutions_found on lattice stats matches.
    assert stats.solutions_found == 2


# ── kb_index only with store_lattice ─────────────────────────


def test_gaps_solve_proof_kb_index_only_with_store_lattice():
    """Default flags: ``proof.kb_index == {}``.
    ``store_lattice=True``: ``proof.kb_index`` non-empty."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")

    # Default: kb_index empty.
    verdict_off, _ = gaps_solve(kb, max_set_size=3)
    assert verdict_off.proof is not None
    assert verdict_off.proof.kb_index == {}

    # Fresh kb (gaps_solve mutates root via flat-writes — re-load
    # to ensure clean state for the second run).
    kb2 = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict_on, _ = gaps_solve(kb2, max_set_size=3, store_lattice=True)
    assert verdict_on.proof is not None
    assert len(verdict_on.proof.kb_index) > 0
    # Every entry is a SetNode.
    for node in verdict_on.proof.kb_index.values():
        assert isinstance(node, SetNode)


# ── No merge under GAPS — state_hash_merges stays 0 ──────────


def test_gaps_solve_kb_index_no_merge():
    """Under GAPS with ``store_lattice=True``, the dedup MERGE
    step is auto-disabled. Two distinct commitments with the
    same post-saturation :func:`state_hash` register as
    separate entries (different ``hash(commitment)`` keys), and
    the ``state_hash_merges`` counter stays at 0 regardless of
    input."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    _verdict, stats = gaps_solve(kb, max_set_size=3, store_lattice=True)
    # GAPS contract: state_hash_merges never increments.
    assert stats.state_hash_merges == 0
    # Direct unit-level check on the recorder — manufacture two
    # distinct commitments that share the same fork kb (same
    # state_hash) and verify the gaps key path produces two
    # separate kb_index entries.
    lstate = _LatticeLoopState()
    fake_kb = KnowledgeBase()
    c1 = (("p", ("a",)),)
    c2 = (("q", ("b",)),)
    _record_setnode(
        lstate, entry="gaps", commitment=c1,
        result_kb=fake_kb, verdict_label="alive", layer=1,
    )
    _record_setnode(
        lstate, entry="gaps", commitment=c2,
        result_kb=fake_kb, verdict_label="alive", layer=1,
    )
    # Both entries present; merge counter unchanged.
    assert len(lstate.kb_index) == 2
    assert lstate.state_hash_merges == 0
    # Both nodes record the same post-saturation state_hash
    # (the field is per-kb, regardless of dict keying).
    state_hashes = {n.state_hash for n in lstate.kb_index.values()}
    assert len(state_hashes) == 1


def test_record_setnode_contradictions_merges():
    """Contradictions-side dedup MERGE: two distinct commitments
    with identical fork kbs collapse into one multilabel
    SetNode, and ``state_hash_merges`` ticks. This wires the
    forward-compat path that S1.5b.23 activates (upstream
    ``contradictions_solve`` is still gated)."""
    lstate = _LatticeLoopState()
    fake_kb = KnowledgeBase()
    c1 = (("p", ("a",)),)
    c2 = (("q", ("b",)),)
    merged_1 = _record_setnode(
        lstate, entry="contradictions", commitment=c1,
        result_kb=fake_kb, verdict_label="dead", layer=1,
    )
    merged_2 = _record_setnode(
        lstate, entry="contradictions", commitment=c2,
        result_kb=fake_kb, verdict_label="dead", layer=1,
    )
    assert merged_1 is False
    assert merged_2 is True
    assert lstate.state_hash_merges == 1
    # One dict entry; labels carries both commitments.
    assert len(lstate.kb_index) == 1
    sole = next(iter(lstate.kb_index.values()))
    assert set(sole.labels) == {c1, c2}


# ── SolutionRecord.kb survives root mutation ─────────────────


def test_solution_record_kb_isolated_from_root_mutation():
    """``rec.kb`` is a snapshot — facts added to root after the
    return don't leak into the branch view."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=3)
    assert verdict.proof is not None
    rec = verdict.proof.solutions[0]
    snap_fact_count = len(rec.kb.facts)

    # Mutate root after return — add an unrelated reasoning fact.
    extra = Fact(
        relation_name="post-return-marker",
        args=("x",),
        layer=Layer.REASONING,
        provenance=Provenance.from_rule(rule="<test>"),
    )
    kb.add_fact(extra)
    kb._index_fact(kb.facts[-1])

    # Snapshot's facts list is unchanged.
    assert len(rec.kb.facts) == snap_fact_count
    assert rec.kb._fact_by_id("post-return-marker", ("x",)) is None


# ── alive_at_end on depth-cap ────────────────────────────────


def test_lattice_proof_alive_at_end():
    """``branching/04_two_levels`` with ``max_set_size=1`` —
    layer 1 collects the two satisfying singletons (Blue↔H3,
    Green↔H3) AND keeps two non-satisfying alive singletons
    (Blue↔H2, Green↔H2) in ``a_layer``. The depth cap (==1)
    is the natural terminator, so ``proof.alive_at_end`` is
    that surviving frontier."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = gaps_solve(kb, max_set_size=1)
    assert verdict.proof is not None
    # Frontier captured.
    assert len(verdict.proof.alive_at_end) > 0
    # Each entry is a size-1 commitment (a CanonicalSetId tuple
    # with one FactId element).
    for c in verdict.proof.alive_at_end:
        assert len(c) == 1


def test_lattice_proof_alive_at_end_empty_when_exhausted():
    """When the loop terminates because every alive commitment
    became a solution / dead (``a_layer`` empty) before hitting
    the depth cap, ``alive_at_end`` stays empty."""
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    verdict, _ = gaps_solve(kb)
    assert verdict.proof is not None
    # Phase 1 short-circuited — Phase 2 never entered;
    # ``alive_at_end`` left at default.
    assert verdict.proof.alive_at_end == ()


# ── learned_nogoods snapshot ─────────────────────────────────


def test_lattice_proof_learned_nogoods_present():
    """``proof.learned_nogoods`` mirrors ``root._nogoods`` at
    return time."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, stats = gaps_solve(kb, max_set_size=3)
    assert verdict.proof is not None
    # At least one nogood landed during the search (the
    # branching/04 trace produces a dead commitment).
    assert stats.nogoods_emitted >= 1
    assert len(verdict.proof.learned_nogoods) >= 1
    # Each is a frozenset of FactIds.
    for clause in verdict.proof.learned_nogoods:
        assert isinstance(clause, frozenset)
