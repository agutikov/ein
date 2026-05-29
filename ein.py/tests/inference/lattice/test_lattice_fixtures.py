"""Lattice example fixtures — pinned-behaviour tests — S1.5b.28
T1.5b.28.5.

Three didactic fixtures under ``examples/lattice/`` pin behaviour
against the lattice engine's three signature pruning / merging
mechanisms:

- ``01_subset_pruned.ein`` — Apriori prefix-join structurally
  drops the supersets of a dead 2-subset. The {a,b}-containing
  3-sets are never generated; the matches_any_nogood filter is
  a redundant guard.
- ``02_genuine_3set_death.ein`` — the genuine combinatorial-core
  case: a 3-set dies but no 2-subset is in D_2. Apriori cannot
  prune; the death surfaces at layer 3.
- ``03_state_hash_collision.ein`` — distinct commitments
  saturating to identical post-saturation kbs collapse into one
  multilabel SetNode under ``store_lattice=True``. Verifies
  Q1.5b.4.c's resolution constructively. The same phenomenon
  occurs naturally on ``examples/zebra2-hints.ein`` (see the
  EIN_RUN_SLOW-gated test below) but at ~16s wall-clock; the
  small fixture saturates in microseconds.

Cross-references:

- Fixture source: ``examples/lattice/0{1,2,3}_*.ein``.
- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.28_lattice_fixtures.md``.
- Q1.5b.2.d (subset elimination) +
  Q1.5b.4.c (state-hash collision) in
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/open_questions.md``.
"""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from ein_bot.inference.monotonic import (
    contradictions_solve,
    validate_proof_for_explanation,
)
from ein_bot.inference.verdict import Contradiction
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
LATTICE = REPO / "examples" / "lattice"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


# ── Fixture 01 — Apriori subset pruning ──────────────────


def test_subset_pruning_on_01():
    """``01_subset_pruned.ein``: layer-3 candidates containing
    {a, b} are NEVER generated (Apriori prefix-join can't form
    them from the surviving layer-2 pairs).

    Concrete counts:
      Layer 1: 4 singletons (all alive).
      Layer 2: 6 pairs, 1 dead ({a,b}), 5 alive.
      Layer 3: Apriori joins the 5 alive pairs → 2 candidates
               ({a,c,d} from (a,c)+(a,d); {b,c,d} from
               (b,c)+(b,d)). The would-be {a,b,c}, {a,b,d}
               are never produced because (a,b) is absent
               from a_prev.

    enterings_total = 4 + 6 + 2 = 12.
    """
    kb = _kb_from(LATTICE / "01_subset_pruned.ein")
    verdict, stats = contradictions_solve(kb, max_set_size=3)
    assert isinstance(verdict, Contradiction)
    assert stats.enterings_total == 12
    assert stats.layers_explored == 3
    # One nogood emitted (the {a, b} death).
    assert stats.nogoods_emitted == 1
    # Exactly 1 dead commitment recorded.
    assert len(verdict.proof.dead_commitments) == 1
    dead = verdict.proof.dead_commitments[0]
    assert dead.layer == 2
    assert len(dead.commitment) == 2


def test_subset_pruning_visits_no_ab_triples():
    """The {a,b}-containing 3-sets {a,b,c} and {a,b,d} are
    absent from the kb_index even under ``store_lattice=True``
    — direct evidence that they were never entered."""
    kb = _kb_from(LATTICE / "01_subset_pruned.ein")
    verdict, _ = contradictions_solve(
        kb, max_set_size=3, store_lattice=True,
    )
    # Collect every commitment visited (via kb_index labels).
    visited: set[frozenset] = set()
    for node in verdict.proof.kb_index.values():
        for c in node.labels:
            visited.add(frozenset(c))

    a_id = ("proposed", ("a",))
    b_id = ("proposed", ("b",))
    c_id = ("proposed", ("c",))
    d_id = ("proposed", ("d",))

    # {a, b, c} and {a, b, d} were never entered.
    assert frozenset({a_id, b_id, c_id}) not in visited
    assert frozenset({a_id, b_id, d_id}) not in visited
    # {a, c, d} and {b, c, d} WERE entered.
    assert frozenset({a_id, c_id, d_id}) in visited
    assert frozenset({b_id, c_id, d_id}) in visited


# ── Fixture 02 — Genuine 3-set death ─────────────────────


def test_genuine_3set_death_on_02():
    """``02_genuine_3set_death.ein``: the 3-set {h1, h2, h3}
    dies but every 2-subset is alive. Apriori cannot prune;
    the death surfaces at layer 3."""
    kb = _kb_from(LATTICE / "02_genuine_3set_death.ein")
    verdict, stats = contradictions_solve(kb, max_set_size=3)
    assert isinstance(verdict, Contradiction)
    # 3 singletons + 3 pairs + 1 triple = 7 enterings.
    assert stats.enterings_total == 7
    assert stats.layers_explored == 3
    # The one dead is the size-3 commitment.
    assert len(verdict.proof.dead_commitments) == 1
    dead = verdict.proof.dead_commitments[0]
    assert dead.layer == 3
    assert len(dead.commitment) == 3
    # learned_clause matches the commitment.
    assert dead.learned_clause == frozenset(dead.commitment)


def test_genuine_3set_no_2_subset_in_d2():
    """The 2-subsets of the dead 3-set are NOT in D_2: their
    learned clauses don't subsume the 3-set's clause."""
    kb = _kb_from(LATTICE / "02_genuine_3set_death.ein")
    verdict, _ = contradictions_solve(kb, max_set_size=3)
    dead = verdict.proof.dead_commitments[0]
    dead_clause = frozenset(dead.commitment)
    # Every size-2 subset of the dead-3 set is NOT a member
    # of learned_nogoods (no proper subset is a nogood).
    # The full clause IS in learned_nogoods.
    proper_subsets = {
        frozenset({dead.commitment[0], dead.commitment[1]}),
        frozenset({dead.commitment[0], dead.commitment[2]}),
        frozenset({dead.commitment[1], dead.commitment[2]}),
    }
    for sub in proper_subsets:
        assert sub not in verdict.proof.learned_nogoods
    assert dead_clause in verdict.proof.learned_nogoods


# ── Fixture 03 — State-hash collision (fast) ─────────────


def test_state_hash_collision_on_03():
    """``03_state_hash_collision.ein``: two layer-2 commits
    ({h1,h2} and {h2,h3}) saturate to the same kb as {h2}.
    Under contradictions+store_lattice the merge fires
    twice; the multilabel SetNode has 3 labels."""
    kb = _kb_from(LATTICE / "03_state_hash_collision.ein")
    verdict, stats = contradictions_solve(
        kb, max_set_size=2, store_lattice=True,
    )
    assert isinstance(verdict, Contradiction)
    assert stats.state_hash_merges == 2
    # 4 distinct SetNodes from 6 visited commitments.
    assert len(verdict.proof.kb_index) == 4
    # Exactly one multilabel SetNode.
    multilabel = [
        n for n in verdict.proof.kb_index.values()
        if len(n.labels) > 1
    ]
    assert len(multilabel) == 1
    node = multilabel[0]
    # 3 labels — {h2}, {h1,h2}, {h2,h3}.
    assert len(node.labels) == 3


def test_state_hash_collision_03_contract_validates():
    """The merged proof still passes
    :func:`validate_proof_for_explanation` — multilabel
    SetNodes are permitted under :class:`Contradiction`
    verdicts."""
    kb = _kb_from(LATTICE / "03_state_hash_collision.ein")
    verdict, _ = contradictions_solve(
        kb, max_set_size=2, store_lattice=True,
    )
    validate_proof_for_explanation(verdict, verdict.proof)


# ── zebra2-hints state-hash collision (EIN_RUN_SLOW-gated) ──


@pytest.mark.skipif(
    not os.environ.get("EIN_RUN_SLOW"),
    reason="zebra2-hints contradictions_solve is ~16s on "
           "CPython; set EIN_RUN_SLOW=1 or run via PyPy.",
)
def test_state_hash_collision_on_zebra2_hints():
    """``examples/zebra2-hints.ein`` naturally triggers a strong
    state-hash collision under contradictions_solve: every
    correct-hint commitment forces the puzzle's saturation
    to the same fully-determined kb. With
    ``max_set_size=1, store_lattice=True``, the documented
    measurement is:

    - enterings_total ≈ 36 (1 per alive layer-1 candidate +
      deads)
    - state_hash_merges ≈ 12 (alive commitments collapsed
      into the canonical correct-state node)
    - kb_index size: 2 distinct SetNodes
    - multilabel SetNodes: 1 (carrying ~13 commitments)

    The exact numbers depend on the hint set in
    zebra2-hints.ein; this test pins the qualitative
    invariant (collisions exist + multilabel observed)
    rather than the exact counts.
    """
    kb = KnowledgeBase.from_ir(parse(
        (REPO / "examples" / "zebra2-hints.ein").read_text(),
    ))
    verdict, stats = contradictions_solve(
        kb, max_set_size=1, store_lattice=True,
    )
    assert isinstance(verdict, Contradiction)
    assert stats.state_hash_merges >= 1, (
        "zebra2-hints failed to trigger any state-hash "
        "collision — the engine's saturation-determinism "
        "premise (Q1.5b.4.c) is degraded; investigate"
    )
    multilabel = [
        n for n in verdict.proof.kb_index.values()
        if len(n.labels) > 1
    ]
    assert len(multilabel) >= 1
