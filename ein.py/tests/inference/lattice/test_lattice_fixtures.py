"""Lattice example fixtures — pinned-behaviour tests — S1.5b.28
(P1.7a refit, 2026-06-16).

Three didactic fixtures under ``examples/lattice/`` pin the lattice
engine's signature pruning / merging mechanisms. They are now driven
through the one sound entry :func:`solve` (run exhaustively with
``store_lattice=True``); the verdict TYPE is read from ``k`` and the
refutation map rides in ``proof.dead_commitments``.

Two notes on the refit:

- ``solve`` does **not** build the per-SetNode DAG (``proof.kb_index``
  is always ``{}`` — intentional). The kb_index-visited-set assertions
  that the removed ``contradictions_solve`` enabled are therefore
  re-expressed against ``proof.dead_commitments`` / nogoods where the
  same fact is observable, and flagged TODO where they genuinely
  required the DAG.
- ``solve``'s per-layer traversal differs from the removed
  contradictions entry (no mid-search unconditional-fact merge, no
  state-hash MERGE), so exact ``enterings_total`` counts are NOT pinned
  blindly — only the structural invariants that survive the entry change.

Cross-references:

- Fixture source: ``examples/lattice/0{1,2,3}_*.ein``.
- Q1.5b.2.d (subset elimination) + Q1.5b.4.c (state-hash collision) in
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/open_questions.md``.
"""
from __future__ import annotations

import os
from pathlib import Path

import pytest

from ein.inference.monotonic import (
    solve,
    validate_proof_for_explanation,
)
from ein.inference.verdict import Ambiguity
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
LATTICE = REPO / "examples" / "lattice"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _solve(kb: KnowledgeBase, **kw):
    return solve(kb, stop_after=None, store_lattice=True, **kw)


# ── Fixture 01 — Apriori subset pruning ──────────────────


def test_subset_pruning_on_01():
    """``01_subset_pruned.ein``: only the {a, b} pair is forbidden. Under the
    unified ``solve`` (``complete`` = no open hypothesis) the other subsets
    complete consistent models, so this is satisfiable with two distinct
    models → ``k == 2`` → :class:`Ambiguity`. The point under test survives
    the verdict change: the {a, b} death is still the sole refutation, and its
    supersets {a,b,c} / {a,b,d} are pruned at generation by the Apriori
    prefix-join (so they never enter and never appear as deads); one nogood is
    emitted for it.
    """
    kb = _kb_from(LATTICE / "01_subset_pruned.ein")
    verdict, _stats = _solve(kb, max_set_size=3)
    assert isinstance(verdict, Ambiguity)
    # Exactly one dead commitment recorded — the {a, b} death.
    assert len(verdict.proof.dead_commitments) == 1
    dead = verdict.proof.dead_commitments[0]
    assert dead.layer == 2
    assert len(dead.commitment) == 2
    # One nogood emitted (the {a, b} death).
    assert verdict.proof.stats.nogoods_emitted == 1


def test_subset_pruning_no_ab_superset_in_nogoods():
    """The {a,b}-containing 3-sets {a,b,c} and {a,b,d} are never entered
    (Apriori drops them), so the {a,b} clause is the only `proposed`-pair
    nogood and no 3-superset clause of it is learned.

    (Pre-refit this read the absence directly from ``proof.kb_index``;
    ``solve`` does not build that DAG, so the same fact is asserted via
    the learned-nogood set — the only {a,b}-bearing clause is {a,b}
    itself.)"""
    kb = _kb_from(LATTICE / "01_subset_pruned.ein")
    verdict, _ = _solve(kb, max_set_size=3)

    a_id = ("proposed", ("a",))
    b_id = ("proposed", ("b",))
    c_id = ("proposed", ("c",))
    d_id = ("proposed", ("d",))

    nogoods = verdict.proof.learned_nogoods
    # The {a, b} clause is learned.
    assert frozenset({a_id, b_id}) in nogoods
    # No 3-set superset containing {a, b} was ever refuted.
    assert frozenset({a_id, b_id, c_id}) not in nogoods
    assert frozenset({a_id, b_id, d_id}) not in nogoods


# ── Fixture 02 — Genuine 3-set death ─────────────────────


def test_genuine_3set_death_on_02():
    """``02_genuine_3set_death.ein``: the size-3 set dies but every 2-subset
    is alive. Under the unified ``solve`` (``complete`` = no open hypothesis),
    the alive 2-subsets already complete consistent models, so the search
    finds three distinct models and never has to enter the size-3 death — it
    is satisfiable → ``k == 3`` → :class:`Ambiguity`, with **no** dead
    commitments (the size-3 conflict is never reached). (Under the removed
    exhaustive ``contradictions_solve`` the 3-set death surfaced at layer 3;
    ``solve`` stops recording deaths once a layer yields complete models.)"""
    kb = _kb_from(LATTICE / "02_genuine_3set_death.ein")
    verdict, _stats = _solve(kb, max_set_size=3)
    assert isinstance(verdict, Ambiguity)
    assert len(verdict.branches) == 3
    assert verdict.proof.dead_commitments == ()


# ── Fixture 03 — State-hash collision ────────────────────


def test_state_hash_collision_on_03_solves_to_one_model():
    """``03_state_hash_collision.ein``: committing {h2} derives h1 and h3,
    so the fork becomes complete (every candidate decided) and consistent
    — a solution node. The two layer-2 paths {h1,h2} / {h2,h3} saturate to
    the SAME state and collapse at the state_hash solution-node dedup, so
    there is exactly ONE distinct model → ``k == 1`` → :class:`Solution`.
    No commitment dies (no conflict rule), so ``proof.dead_commitments``
    is empty.

    NB the per-SetNode multilabel MERGE that the removed
    ``contradictions_solve`` exposed (``state_hash_merges == 2``, a
    multilabel kb_index node) is a DAG-builder artefact; ``solve`` does
    not build the DAG, so that view is no longer produced here. The
    sound, entry-independent fact is that the colliding commitments yield
    ONE model.
    """
    from ein.inference.verdict import Solution
    kb = _kb_from(LATTICE / "03_state_hash_collision.ein")
    verdict, _stats = _solve(kb, max_set_size=2)
    assert isinstance(verdict, Solution)
    assert len(verdict.proof.solutions) == 1
    assert verdict.proof.dead_commitments == ()
    # solve does not build the per-SetNode DAG.
    assert verdict.proof.kb_index == {}
    assert verdict.proof.stats.state_hash_merges == 0


def test_state_hash_collision_03_contract_validates():
    """The proof still passes :func:`validate_proof_for_explanation` (the
    sole solution-node's kb is goal-satisfying; empty deads / empty
    kb_index are permitted)."""
    kb = _kb_from(LATTICE / "03_state_hash_collision.ein")
    verdict, _ = _solve(kb, max_set_size=2)
    validate_proof_for_explanation(verdict, verdict.proof)


# ── zebra2-hints state-hash collision (EIN_RUN_SLOW-gated) ──


@pytest.mark.skipif(
    not os.environ.get("EIN_RUN_SLOW"),
    reason="zebra2-hints exhaustive solve is ~16s on CPython; "
           "set EIN_RUN_SLOW=1 or run via PyPy.",
)
def test_zebra2_hints_solves():
    """``examples/zebra2-hints.ein`` is a partial-state fixture whose
    correct hints force the puzzle's saturation toward one fully-determined
    kb. Under the exhaustive ``solve`` it resolves; this test pins the
    qualitative invariant (a proof is produced and it validates) rather
    than the removed contradictions-entry merge counters.
    """
    kb = KnowledgeBase.from_ir(parse(
        (REPO / "examples" / "zebra2-hints.ein").read_text(),
    ))
    verdict, _stats = _solve(kb, max_set_size=1)
    assert verdict.proof is not None
    validate_proof_for_explanation(verdict, verdict.proof)
