"""contradictions_solve backbone tests — S1.5b.23 T1.5b.23.5.

Pins :func:`ein_bot.inference.monotonic.contradictions_solve`
across the CONTRADICTIONS-mode contract:

- Verdict is always :class:`Contradiction` (mode contract).
- ``proof.dead_commitments`` collects every refuted commitment
  with its unsat-core + learned clause + layer + kind.
- ``verdict.unsat_core`` is the union of every recorded dead's
  core (empty frozenset when no deads observed — the
  degenerate "actually solvable" case).
- Phase 1 root contradiction returns immediately with empty
  ``proof.dead_commitments``.
- State-hash dedup MERGE activates under
  ``store_lattice=True`` — distinct dead commitments
  saturating to the same kb collapse into one multilabel
  :class:`SetNode` and the prior arrival's records subsume
  the new one.
- Fork-side ``is_solved`` does NOT short-circuit Phase 2 —
  supersets of solved commitments may still die under
  additional hypotheses.

Cross-references:

- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.23_lattice_dumper.md``
- Algorithm:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md``
  §3c.i + § Verdict synthesis (contradictions_solve).
- Sibling gaps tests:
  ``ein.py/tests/inference/lattice/test_gaps_backbone.py``
- Sibling lattice-proof tests:
  ``ein.py/tests/inference/lattice/test_lattice_proof.py``
"""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.monotonic import (
    DeadCommitment,
    LatticeProof,
    LatticeStats,
    contradictions_solve,
)
from ein_bot.inference.monotonic.solver import (
    _LatticeLoopState,
    _record_setnode,
)
from ein_bot.inference.verdict import Contradiction
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _kb_inline(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# ── Verdict shape ──────────────────────────────────────────


def test_contradictions_solve_always_returns_contradiction_tuple():
    """``(verdict, stats)`` shape; verdict is :class:`Contradiction`,
    stats is :class:`LatticeStats`."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    result = contradictions_solve(kb, max_set_size=3)
    assert isinstance(result, tuple)
    assert len(result) == 2
    verdict, stats = result
    assert isinstance(verdict, Contradiction)
    assert isinstance(stats, LatticeStats)
    assert isinstance(verdict.proof, LatticeProof)


# ── Multi-dead enumeration ─────────────────────────────────


def test_contradictions_solve_branching_04_collects_deads():
    """``branching/04_two_levels`` has 4 layer-2 dead
    commitments (each pairing of Blue/Green ↔ H1/H2 against
    Red↔H1 + sibling-exclusive). contradictions_solve
    enumerates them in ``proof.dead_commitments``."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, stats = contradictions_solve(kb, max_set_size=3)
    assert isinstance(verdict, Contradiction)
    assert len(verdict.proof.dead_commitments) >= 1
    # Each dead carries an unsat_core + learned_clause.
    for d in verdict.proof.dead_commitments:
        assert isinstance(d, DeadCommitment)
        assert d.kind in ("dead-pre", "dead-post")
        assert isinstance(d.unsat_core, frozenset)
        assert isinstance(d.learned_clause, frozenset)
        assert d.layer >= 1
    # Stats coherence.
    assert stats.enterings_dead_pre + stats.enterings_dead_post == (
        len(verdict.proof.dead_commitments)
    )


# ── unsat_core is the union of dead cores ──────────────────


def test_contradictions_solve_unsat_core_is_union_of_dead_cores():
    """``verdict.unsat_core`` is the set union of every
    recorded ``d.unsat_core`` across
    ``proof.dead_commitments``. Holds by construction in
    ``_finalise_lattice_verdict`` — this test pins the
    invariant against the contract."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = contradictions_solve(kb, max_set_size=3)
    expected = frozenset()
    for d in verdict.proof.dead_commitments:
        expected = expected | d.unsat_core
    assert verdict.unsat_core == expected


# ── Phase 1 root contradiction — empty deads ───────────────


def test_contradictions_solve_root_contradiction_returns_empty_deads():
    """A puzzle whose root saturates to ``(false)`` in Phase 1
    returns :class:`Contradiction` with empty
    ``proof.dead_commitments`` (no commitment was ever
    tried). ``verdict.unsat_core`` is the empty frozenset
    (no recorded dead's core to union)."""
    kb = _kb_inline("""
    (rules
      (rule always-false ()
        :match (trigger ?x)
        :assert (false)
        :why "always" :priority 100))
    (ontology
      (type T)
      (relation trigger T)
      (instance a T))
    (facts (trigger a :source "(1)"))
    (query :mode solve :goal (trigger ?x))
    """)
    verdict, stats = contradictions_solve(kb, max_set_size=1)
    assert isinstance(verdict, Contradiction)
    assert verdict.proof.dead_commitments == ()
    assert verdict.unsat_core == frozenset()
    assert stats.enterings_total == 0


# ── Solvable puzzle — degenerate "no deaths" case ──────────


def test_contradictions_solve_solvable_puzzle_may_have_no_deaths():
    """``branching/01_saturate_only`` solves at Phase 1 root.
    Under :func:`contradictions_solve` Phase 1 doesn't
    short-circuit on root-is_solved — it proceeds to Phase 2.
    There may or may not be deads; the contract is that
    verdict is :class:`Contradiction` (degenerate case)."""
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    verdict, stats = contradictions_solve(kb)
    assert isinstance(verdict, Contradiction)
    # Phase 1 short-circuit suppressed for contradictions —
    # the engine entered Phase 2 (layers_explored may be 0 if
    # alive was empty post-Phase-1, but the verdict is still
    # Contradiction by the mode contract).
    _ = stats  # no specific stat assertion — counter values
    # depend on Phase 1's forced-positive cascade behaviour
    # for this specific fixture.


# ── State-hash dedup MERGE wires under store_lattice=True ──


def test_contradictions_solve_state_hash_merge_via_unit_helper():
    """The contradictions-side state-hash dedup MERGE path is
    activated by S1.5b.23 lifting the upstream raise; this
    test exercises the merge path directly through
    :func:`_record_setnode`. A natural-collision integration
    fixture lives under S1.5b.28's lattice fixtures
    (TBD); the unit-level check here confirms the wiring."""
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


def test_contradictions_solve_store_lattice_populates_kb_index():
    """Under ``store_lattice=True``, every visited non-``dead-pre``
    commitment registers in ``proof.kb_index``. Distinct
    commitments map to distinct entries (state_hash keying);
    collisions would collapse but branching/04 produces
    distinct kb-states per commitment."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, stats = contradictions_solve(
        kb, max_set_size=3, store_lattice=True,
    )
    assert isinstance(verdict, Contradiction)
    assert len(verdict.proof.kb_index) > 0
    # branching/04 has no naturally-occurring state_hash
    # collisions, so no merges fire here. (The unit-level
    # merge test above pins the merge wiring.)
    assert stats.state_hash_merges == 0


def test_contradictions_solve_store_lattice_off_keeps_kb_index_empty():
    """``store_lattice=False`` (default) — ``proof.kb_index``
    stays an empty dict, but ``proof.dead_commitments`` is
    still populated."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = contradictions_solve(kb, max_set_size=3)
    assert verdict.proof.kb_index == {}
    # dead_commitments collection is independent of
    # store_lattice — it's the contradictions contract.
    assert len(verdict.proof.dead_commitments) >= 1


# ── Learned nogoods snapshot ───────────────────────────────


def test_contradictions_solve_learned_nogoods_present():
    """Every dead commitment emits a nogood; ``proof.learned_nogoods``
    mirrors ``root._nogoods`` at termination."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, stats = contradictions_solve(kb, max_set_size=3)
    assert stats.nogoods_emitted >= 1
    assert len(verdict.proof.learned_nogoods) >= 1
    for clause in verdict.proof.learned_nogoods:
        assert isinstance(clause, frozenset)
