"""Gaps-view backbone tests — S1.5b.21 (P1.7a refit, 2026-06-16).

The former ``gaps_solve`` entry (which ALWAYS returned :class:`Ambiguity`
regardless of the real model count) was removed. The gaps *view* — the
full set of distinct models — is now read off the one sound entry
:func:`solve` run exhaustively (``stop_after=None``) with
``store_lattice=True``: the verdict TYPE is read from ``k`` (the number of
distinct, state_hash-deduped solution nodes) and the model set rides along
in ``verdict.proof.solutions``.

These tests pin, per fixture, the SOUND verdict (Solution / Ambiguity /
Contradiction read from ``k``) and the gaps-view content
(``proof.solutions``). The lattice counters live on
``verdict.proof.stats`` (a :class:`LatticeStats`); ``solve``'s own second
return value is a :class:`MonotonicStats`.

Cross-references:

- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.21_lattice_backbone.md``
- Algorithm:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md``
- Sibling monotonic tests:
  ``ein.py/tests/inference/monotonic/test_monotonic_skeleton.py``
"""
from __future__ import annotations

from pathlib import Path

from ein.inference.monotonic import (
    LatticeStats,
    MonotonicStats,
    solve,
)
from ein.inference.verdict import Ambiguity, Contradiction, Solution
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _kb_inline(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _gaps(kb: KnowledgeBase, **kw):
    """Exhaustive solve with the lattice proof attached — the gaps view."""
    return solve(kb, stop_after=None, store_lattice=True, **kw)


# ── Verdict shape ──────────────────────────────────────────


def test_solve_returns_verdict_stats_tuple():
    """``(verdict, stats)`` shape; ``stats`` is a :class:`MonotonicStats`,
    and the lattice counters live on ``verdict.proof.stats``
    (:class:`LatticeStats`)."""
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    result = _gaps(kb)
    assert isinstance(result, tuple)
    assert len(result) == 2
    verdict, stats = result
    assert isinstance(stats, MonotonicStats)
    assert isinstance(verdict.proof.stats, LatticeStats)


# ── Single-model outcomes ──────────────────────────────────


def test_solve_branching_01_is_unique_solution():
    """``branching/01_saturate_only`` — the goal is matched at root, but
    the is-a-alternate hypotheses remain open until ``functional`` kills
    each; once they all die the root is the unique complete∧consistent
    model. ``solve`` (which records on consistent∧complete, NOT goal-match)
    therefore returns :class:`Solution` (k=1) with one
    ``proof.solutions`` record.
    """
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    verdict, _ = _gaps(kb)
    assert isinstance(verdict, Solution)
    assert len(verdict.proof.solutions) == 1


def test_solve_branching_03_returns_one_model():
    """``branching/03_five_hyps_one_alive`` resolves to the single model
    ``(co-located White H5)`` — :class:`Solution` (k=1), one
    ``proof.solutions`` record."""
    kb = _kb_from(BRANCHING / "03_five_hyps_one_alive.ein")
    verdict, _ = _gaps(kb, max_set_size=3)
    assert isinstance(verdict, Solution)
    assert len(verdict.proof.solutions) == 1


# ── Multi-model outcomes (the headline feature) ───────────


def test_solve_branching_04_returns_two_models():
    """``branching/04_two_levels`` has TWO valid commitments (Blue↔H3 and
    Green↔H3) → ``k == 2`` → :class:`Ambiguity` with 2 distinct Solution
    branches. Each branch's kb satisfies the goal — verifiable by
    re-running the goal pattern against each branch's kb.
    """
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _gaps(kb, max_set_size=3)
    assert isinstance(verdict, Ambiguity)
    assert len(verdict.branches) == 2
    # Each branch's kb satisfies the goal; verify by re-running
    # the goal pattern against each branch's kb.
    from ein.inference.compile import JoinPlan, compile_pattern
    from ein.inference.match import run as match_run
    bindings_seen: list[dict[str, str]] = []
    for branch in verdict.branches:
        goal = next(
            kp.value for kp in branch.kb.query.kw_pairs
            if kp.key.name == "goal"
        )
        plan = JoinPlan(
            rule_name="<query>", activator_args=(), bindings_seed={},
            steps=tuple(compile_pattern(goal, {})),
            assert_templates=(), why="",
        )
        rows = [dict(b) for b, _premises in match_run(plan, branch.kb)]
        assert rows, f"branch {branch} doesn't satisfy goal"
        bindings_seen.extend(rows)
    # The two solutions bind ?c to Blue and Green respectively
    # (both valid for H3 under the puzzle's constraints).
    colours = {row.get("c") for row in bindings_seen}
    assert colours == {"Blue", "Green"}, (
        f"expected branches binding ?c ∈ {{Blue, Green}}, got {colours}"
    )
    # The proof's solution set matches the branch set.
    assert len(verdict.proof.solutions) == 2


def test_solve_branching_05_returns_one_distinct_model():
    """``branching/05_mini_zebra`` has a UNIQUE model (k=1: Bob drinks
    Coffee, owns Dog) — :class:`Solution`, one ``proof.solutions`` record.

    The model is reached via two orientations of the symmetric
    ``co-located`` commitment, but they saturate to the same KB and
    collapse at the ``state_hash`` solution-node dedup → one model.
    """
    kb = _kb_from(BRANCHING / "05_mini_zebra.ein")
    verdict, _ = _gaps(kb, max_set_size=3)
    assert isinstance(verdict, Solution)
    assert len(verdict.proof.solutions) == 1


# ── Unsat outcomes (k == 0 → Contradiction) ───────────────


def test_solve_root_contradiction_returns_contradiction():
    """Root saturates to ``(false)`` directly → Phase 1 detects it → no
    solution node is ever recorded → ``k == 0`` → :class:`Contradiction`
    with empty ``proof.solutions``. Phase 1 detected the contradiction;
    no candidates ran (``enterings_total == 0``).
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
    verdict, stats = _gaps(kb, max_set_size=1)
    assert isinstance(verdict, Contradiction)
    assert verdict.proof.solutions == ()
    # Phase 1 detected the contradiction; no candidates ran.
    assert stats.enterings_total == 0


# ── Stats coherence ───────────────────────────────────────


def test_solve_stats_solutions_found_matches_proof():
    """``branching/04_two_levels`` — ``verdict.proof.stats.solutions_found``
    equals ``len(proof.solutions)`` (== 2 for this fixture)."""
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, _ = _gaps(kb, max_set_size=3)
    lstats = verdict.proof.stats
    assert lstats.solutions_found == len(verdict.proof.solutions) == 2
    # Layer-by-layer exploration happened (the two models are at depth ≥ 1).
    assert lstats.layers_explored >= 1
    assert lstats.enterings_total >= 1
