"""Contradictions-view backbone tests — S1.5b.23 (P1.7a refit, 2026-06-16).

The former ``contradictions_solve`` entry (which ALWAYS returned
:class:`Contradiction` regardless of the real model count) was removed.
The contradictions *view* — the refutation map (every refuted commitment
with its unsat-core + learned clause) — is now read off the one sound
entry :func:`solve` run exhaustively (``stop_after=None``) with
``store_lattice=True``: the verdict TYPE is read from ``k`` and the
refutation map rides along in ``verdict.proof.dead_commitments``
(``solve`` collects deads for every run, not only ``k == 0`` ones).

So a multi-model fixture (branching/04) is an :class:`Ambiguity` whose
proof still carries the deads explored along the way; an UNSAT fixture
(lattice/02, or a root-``(false)``) is a :class:`Contradiction` whose
``unsat_core`` is the union of its deads' cores.

``proof.kb_index`` is **always empty** for ``solve`` — it does not build
the per-SetNode DAG (that was the removed lattice entries' job; the sound
data is the solutions + deads). The state-hash dedup MERGE path is a
contradictions-entry mechanism exercised here at the unit level via
:func:`_record_setnode`.

Cross-references:

- Algorithm:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md``
- Sibling gaps tests:
  ``ein.py/tests/inference/lattice/test_gaps_backbone.py``
- Sibling lattice-proof tests:
  ``ein.py/tests/inference/lattice/test_lattice_proof.py``
"""
from __future__ import annotations

from pathlib import Path

from ein.inference.monotonic import (
    DeadCommitment,
    LatticeProof,
    MonotonicStats,
    solve,
)
from ein.inference.monotonic.solver import (
    _LatticeLoopState,
    _record_setnode,
)
from ein.inference.verdict import Ambiguity, Contradiction
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"
LATTICE = REPO / "examples" / "lattice"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _kb_inline(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _solve(kb: KnowledgeBase, **kw):
    return solve(kb, stop_after=None, store_lattice=True, **kw)


# ── Verdict shape ──────────────────────────────────────────


def test_solve_branching_04_is_ambiguity_with_proof():
    """``(verdict, stats)`` shape; ``branching/04_two_levels`` has TWO
    models (k=2) → :class:`Ambiguity`, ``stats`` a :class:`MonotonicStats`,
    and ``verdict.proof`` a :class:`LatticeProof`."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    result = _solve(kb, max_set_size=3)
    assert isinstance(result, tuple)
    assert len(result) == 2
    verdict, stats = result
    assert isinstance(verdict, Ambiguity)
    assert isinstance(stats, MonotonicStats)
    assert isinstance(verdict.proof, LatticeProof)


# ── Dead enumeration (refutation map) ──────────────────────


def test_solve_branching_04_collects_deads_in_proof():
    """``branching/04_two_levels`` refutes several layer-2 commitments
    along the way; ``solve`` records them in ``proof.dead_commitments``
    even though the verdict is :class:`Ambiguity`."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _stats = _solve(kb, max_set_size=3)
    assert len(verdict.proof.dead_commitments) >= 1
    # Each dead carries an unsat_core + learned_clause.
    for d in verdict.proof.dead_commitments:
        assert isinstance(d, DeadCommitment)
        assert d.kind in ("dead-pre", "dead-post")
        assert isinstance(d.unsat_core, frozenset)
        assert isinstance(d.learned_clause, frozenset)
        assert d.layer >= 1
    # Stats coherence: every recorded dead is counted (solve does not run
    # the contradictions-side state-hash merge that skips the append).
    lstats = verdict.proof.stats
    assert lstats.enterings_dead_pre + lstats.enterings_dead_post == (
        len(verdict.proof.dead_commitments)
    )


# ── unsat_core is the union of dead cores (UNSAT fixture) ──


def test_solve_unsat_core_is_union_of_dead_cores():
    """For a genuinely UNSAT fixture (``ein-bugs/zebra2-bad`` — an injected
    ``(color-loc Green House-1)`` clashes with the spatial chain during root
    saturation → no model → k=0 → :class:`Contradiction`),
    ``verdict.unsat_core`` is the set union of every recorded ``d.unsat_core``
    across ``proof.dead_commitments``."""
    kb = _kb_from(REPO / "examples" / "ein-bugs" / "zebra2-bad.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    assert isinstance(verdict, Contradiction)
    expected = frozenset()
    for d in verdict.proof.dead_commitments:
        expected = expected | d.unsat_core
    assert verdict.unsat_core == expected
    assert verdict.unsat_core, "the unsat core must be non-empty"


# ── Phase 1 root contradiction — empty deads ───────────────


def test_solve_root_contradiction_records_root_dead():
    """A puzzle whose root saturates to ``(false)`` in Phase 1 finds no
    solution node (k=0) → :class:`Contradiction`. ``solve``'s Phase-1
    root-contradiction path records a single root :class:`DeadCommitment`
    (``commitment=()``) carrying the source-frontier core, so
    ``verdict.unsat_core`` is non-empty (it traces back to the
    ``(trigger a)`` source fact). No commitment was tried, so
    ``enterings_total == 0``.

    NB this is the one place ``solve`` differs from the removed
    ``contradictions_solve`` Phase-1 path (which left deads empty): the
    sound entry always packages the root core as a dead record.
    """
    kb = _kb_inline("""
    (rule always-false ()
      :match (trigger ?x)
      :assert (false)
      :why "always" :priority 100)
    (type T)
    (relation trigger T)
    (instance a T)
    (trigger a :source "(1)")
    (query :goal (trigger ?x))
    """)
    verdict, stats = _solve(kb, max_set_size=1)
    assert isinstance(verdict, Contradiction)
    # Phase-1 root dead: one record with the empty commitment.
    assert len(verdict.proof.dead_commitments) == 1
    assert verdict.proof.dead_commitments[0].commitment == ()
    # Its core traces back to the (trigger a) source → non-empty union.
    assert verdict.unsat_core
    assert stats.enterings_total == 0


# ── Solvable puzzle — proof still attached ─────────────────


def test_solve_solvable_puzzle_has_proof():
    """``branching/01_saturate_only`` resolves to a unique model
    (k=1 → :class:`Solution`); the proof rides along under
    ``store_lattice=True`` with the (possibly empty) refutation map."""
    from ein.inference.verdict import Solution
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    verdict, _ = _solve(kb)
    assert isinstance(verdict, Solution)
    assert isinstance(verdict.proof, LatticeProof)


# ── State-hash dedup MERGE (contradictions-entry unit path) ──


def test_record_setnode_state_hash_merge_via_unit_helper():
    """The contradictions-side state-hash dedup MERGE path (still present
    on :func:`_record_setnode`, used by the lattice-DAG builders) collapses
    distinct dead commitments saturating to the same kb into one
    multilabel :class:`SetNode`. ``solve`` itself never builds the DAG, so
    this exercises the recorder directly."""
    lstate = _LatticeLoopState()
    fake_kb = KnowledgeBase()
    c1 = (("p", ("a",)),)
    c2 = (("q", ("b",)),)
    # First arrival — inserts.
    merged_1 = _record_setnode(
        lstate, entry="contradictions", commitment=c1,
        result_kb=fake_kb, verdict_label="dead", layer=1,
    )
    # Second arrival with same state_hash — merges labels.
    merged_2 = _record_setnode(
        lstate, entry="contradictions", commitment=c2,
        result_kb=fake_kb, verdict_label="dead", layer=1,
    )
    assert merged_1 is False
    assert merged_2 is True
    assert lstate.state_hash_merges == 1
    assert len(lstate.kb_index) == 1
    sole = next(iter(lstate.kb_index.values()))
    assert set(sole.labels) == {c1, c2}


def test_solve_kb_index_is_always_empty():
    """``solve`` does NOT build the per-SetNode DAG (intentional — the
    sound proof data is the solutions + deads; ``render_lattice`` falls
    back to the solution view when ``kb_index`` is empty). So
    ``proof.kb_index`` is ``{}`` and ``stats.state_hash_merges`` is 0 even
    under ``store_lattice=True``; the deads are still recorded."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    assert verdict.proof.kb_index == {}
    assert verdict.proof.stats.state_hash_merges == 0
    # dead_commitments collection is independent of the DAG.
    assert len(verdict.proof.dead_commitments) >= 1


# ── Learned nogoods snapshot ───────────────────────────────


def test_solve_learned_nogoods_present():
    """Every dead commitment emits a nogood; ``proof.learned_nogoods``
    mirrors ``root._nogoods`` at termination."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _solve(kb, max_set_size=3)
    lstats = verdict.proof.stats
    assert lstats.nogoods_emitted >= 1
    assert len(verdict.proof.learned_nogoods) >= 1
    for clause in verdict.proof.learned_nogoods:
        assert isinstance(clause, frozenset)
