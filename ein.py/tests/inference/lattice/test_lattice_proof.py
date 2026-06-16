"""LatticeProof data-class + ``store_lattice`` wiring tests —
S1.5b.22 (P1.7a refit, 2026-06-16).

The proof is now attached to the one sound entry :func:`solve` under
``store_lattice=True``. ``solve`` records every distinct
(state_hash-deduped) solution node into ``proof.solutions`` and every
refuted commitment into ``proof.dead_commitments``; the verdict TYPE is
read from ``k = len(distinct solution nodes)``.

``proof.kb_index`` is **always ``{}``** for ``solve`` (it does not build
the per-SetNode DAG — intentional; ``render_lattice`` falls back to the
solution view). The per-SetNode recorder :func:`_record_setnode` still
exists and is exercised here at the unit level for both the gaps keying
(per-commitment, no merge) and the contradictions keying (state-hash
merge into multilabel nodes).

Cross-references:

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
    solve,
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


def _solve(kb: KnowledgeBase, **kw):
    return solve(kb, stop_after=None, store_lattice=True, **kw)


# ── proof.solutions populated under store_lattice ────────────


def test_solve_proof_solutions_populated():
    """``branching/04_two_levels`` enumerates both Blue↔H3 and Green↔H3
    (k=2 → :class:`Ambiguity`) — the verdict's ``proof.solutions`` carries
    both as SolutionRecord entries with kb snapshots."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _stats = _solve(kb, max_set_size=3)
    assert isinstance(verdict, Ambiguity)
    assert isinstance(verdict.proof.stats, LatticeStats)

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
    assert proof.stats.solutions_found == 2


# ── kb_index is always empty for solve ───────────────────────


def test_solve_proof_kb_index_always_empty():
    """``solve`` does not build the per-SetNode DAG, so ``proof.kb_index``
    is ``{}`` whether or not ``store_lattice`` is set. ``proof.solutions``
    carries the model set instead."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")

    # store_lattice off → proof is None (no packaging at all).
    verdict_off, _ = solve(kb, stop_after=None, max_set_size=3)
    assert verdict_off.proof is None

    # Fresh kb (solve mutates root via flat-writes — re-load to ensure a
    # clean state for the second run).
    kb2 = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict_on, _ = _solve(kb2, max_set_size=3)
    assert verdict_on.proof is not None
    assert verdict_on.proof.kb_index == {}


# ── _record_setnode keying (unit level) ──────────────────────


def test_record_setnode_gaps_keeps_distinct_commitments():
    """The gaps keying of :func:`_record_setnode` keys by
    ``hash(commitment)``: two distinct commitments that share the same
    post-saturation ``state_hash`` register as SEPARATE entries, and
    ``state_hash_merges`` stays 0."""
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
    # Every entry is a SetNode.
    for node in lstate.kb_index.values():
        assert isinstance(node, SetNode)


def test_record_setnode_contradictions_merges():
    """Contradictions-side dedup MERGE: two distinct commitments with
    identical fork kbs collapse into one multilabel SetNode, and
    ``state_hash_merges`` ticks."""
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
    verdict, _ = _solve(kb, max_set_size=3)
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
    """``branching/04_two_levels`` with ``max_set_size=1`` — layer 1
    records the two satisfying singletons (Blue↔H3, Green↔H3) as solution
    nodes AND keeps the non-satisfying alive singletons in ``a_layer``.
    The depth cap (==1) is the natural terminator, so
    ``proof.alive_at_end`` is that surviving frontier."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=1)
    assert verdict.proof is not None
    # Frontier captured.
    assert len(verdict.proof.alive_at_end) > 0
    # Each entry is a size-1 commitment (a CanonicalSetId tuple
    # with one FactId element).
    for c in verdict.proof.alive_at_end:
        assert len(c) == 1


def test_lattice_proof_alive_at_end_empty_when_exhausted():
    """When the loop terminates because every alive commitment became a
    solution / dead (frontier exhausted) before hitting the depth cap,
    ``alive_at_end`` stays empty. ``branching/01_saturate_only`` resolves
    to its unique model once the is-a alternates all die — an
    exhaustion exit, not a depth-cap one."""
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    verdict, _ = _solve(kb)
    assert verdict.proof is not None
    assert verdict.proof.alive_at_end == ()


# ── learned_nogoods snapshot ─────────────────────────────────


def test_lattice_proof_learned_nogoods_present():
    """``proof.learned_nogoods`` mirrors ``root._nogoods`` at return
    time."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    assert verdict.proof is not None
    lstats = verdict.proof.stats
    # At least one nogood landed during the search (the branching/04
    # trace produces a dead commitment).
    assert lstats.nogoods_emitted >= 1
    assert len(verdict.proof.learned_nogoods) >= 1
    # Each is a frozenset of FactIds.
    for clause in verdict.proof.learned_nogoods:
        assert isinstance(clause, frozenset)
