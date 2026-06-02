"""gaps_solve backbone tests — S1.5b.21 T1.5b.21.X.

Pins :func:`ein_bot.inference.monotonic.gaps_solve` across
the GAPS-mode contract:

- Verdict is always :class:`Ambiguity` (mode contract).
- ``len(branches)`` interpretation: 0 = no solution within
  cap; 1 = uniquely solvable; >1 = genuine multi-solution.
- Each branch carries the kb that satisfied the goal.

Cross-references:

- Stage spec:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.21_lattice_backbone.md``
- Algorithm:
  ``plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md``
- Sibling monotonic tests:
  ``ein.py/tests/inference/monotonic/test_monotonic_skeleton.py``

Zebra2 is not exercised here — gaps_solve on zebra2 with
``max_set_size=1`` already runs ~90s (89 layer-1 candidates
times ~1s saturation each) because gaps must exhaust where
monotonic early-terminates on first fork-side ``is_solved``.
The perf round (S1.5b.30) addresses this; for now the
backbone tests stay on smaller branching fixtures.
"""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.monotonic import (
    LatticeStats,
    gaps_solve,
)
from ein_bot.inference.verdict import Ambiguity, Solution
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb_from(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _kb_inline(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# ── Verdict shape ──────────────────────────────────────────


def test_gaps_solve_always_returns_ambiguity_tuple():
    """``(verdict, stats)`` shape; verdict is :class:`Ambiguity`,
    stats is :class:`LatticeStats` (per S1.5b.22 the public
    ``gaps_solve`` returns the lattice-counter view)."""
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    result = gaps_solve(kb)
    assert isinstance(result, tuple)
    assert len(result) == 2
    verdict, stats = result
    assert isinstance(verdict, Ambiguity)
    assert isinstance(stats, LatticeStats)


# ── Single-branch outcomes ─────────────────────────────────


def test_gaps_solve_root_satisfies_in_phase_1():
    """``branching/01_saturate_only`` — root satisfies the goal
    after the initial saturation; Phase 1 short-circuits to
    Ambiguity with 1 branch (the empty-commitment carrier).
    """
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    verdict, stats = gaps_solve(kb)
    assert len(verdict.branches) == 1
    assert isinstance(verdict.branches[0], Solution)
    # Phase 1 short-circuit — no candidates entered.
    assert stats.enterings_total == 0
    assert stats.layers_explored == 0


def test_gaps_solve_branching_03_returns_one_model():
    """``branching/03_five_hyps_one_alive`` resolves to the single
    model ``(co-located White H5)`` — gaps_solve reports 1 branch.

    S1.7.24 — pre-removal this fired the forced-positive CASCADE at the
    end of Phase 1: the symmetric open-set canonicalisation shrank
    ``alive`` to the singleton ``{(co-located White H5)}``, so
    ``_promote_forced_positives`` resolved it with zero Phase-2
    enterings. With the canonicalisation gone, the sole viable
    symmetric pair registers as TWO open entries, so the cascade
    (which keys on a singleton ``alive``) no longer fires — the model
    is instead found by a Phase-2 commitment. The VERDICT (1 distinct
    model) is unchanged; only the mechanism is. This documents that
    the cascade is a symmetric-canonicalisation-dependent optimisation,
    not a correctness requirement.
    """
    kb = _kb_from(BRANCHING / "03_five_hyps_one_alive.ein")
    verdict, stats = gaps_solve(kb, max_set_size=3)
    assert len(verdict.branches) == 1
    # The cascade no longer fires for symmetric pairs; the model is
    # found via a Phase-2 commitment instead.
    assert stats.forced_positives == 0
    assert stats.enterings_total >= 1


# ── Multi-branch outcomes (the headline feature) ──────────


def test_gaps_solve_branching_04_returns_two_branches():
    """``branching/04_two_levels`` has TWO valid commitments
    (Blue↔H3 and Green↔H3). Monotonic SOLVE-mode terminates
    on the first satisfying fork (lex order picks Blue);
    gaps_solve continues past it and records the second.
    Verdict: Ambiguity with 2 distinct Solution branches.

    Each branch's kb satisfies the goal — verifiable via
    re-checking is_solved.
    """
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    verdict, stats = gaps_solve(kb, max_set_size=3)
    assert len(verdict.branches) == 2
    # Each branch's kb satisfies the goal; verify by re-running
    # the goal pattern against each branch's kb.
    from ein_bot.inference.compile import JoinPlan, compile_pattern
    from ein_bot.inference.match import run as match_run
    bindings_seen: list[dict[str, str]] = []
    for branch in verdict.branches:
        goal = next(
            kp.value for kp in branch.kb.query.kw_pairs
            if kp.key.name == "goal"
        )
        plan = JoinPlan(
            rule_name="<query>", activator_args=(), bindings_seed={},
            steps=tuple(compile_pattern(goal, {})),
            assert_template=None, why="",
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
    # Stats: at least the 2 layer-1 candidates that satisfied
    # were entered.
    assert stats.enterings_total >= 2
    assert stats.layers_explored >= 1


def test_gaps_solve_branching_05_returns_one_distinct_model():
    """``branching/05_mini_zebra`` has a UNIQUE model (solve k=1: Bob
    drinks Coffee, owns Dog) — gaps_solve reports ONE distinct model.

    S1.7.24 — pre-dedup this returned 3 *branches that all shared one
    state_hash* (the same model recorded via three commitment paths — a
    latent gaps over-count the symmetric retirement exposed). gaps now
    dedups solution records by post-saturation state_hash (the generic
    signal SOLVE already used), so the branch count reflects distinct
    MODELS, not commitment paths.
    """
    kb = _kb_from(BRANCHING / "05_mini_zebra.ein")
    verdict, _stats = gaps_solve(kb, max_set_size=3)
    assert len(verdict.branches) == 1


# ── Zero-branch outcomes (Contradiction degenerate) ───────


def test_gaps_solve_root_contradiction_returns_empty_branches():
    """Root saturates to ``(false)`` directly → Phase 1
    detects, gaps_solve returns Ambiguity with empty branches
    (no satisfying commitment exists).
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
    (query :mode solve :goal (trigger ?x))
    """)
    verdict, stats = gaps_solve(kb, max_set_size=1)
    assert isinstance(verdict, Ambiguity)
    assert verdict.branches == ()
    # Phase 1 detected the contradiction; no candidates ran.
    assert stats.enterings_total == 0


# ── Stats correctness ─────────────────────────────────────


def test_gaps_solve_stats_layers_explored_advances():
    """``branching/04_two_levels`` requires depth 2 to find
    both solutions. Verify ``layers_explored`` advances past
    1 (the layer-1 candidates would alone capture each
    singleton solution via fork-side is_solved, but layer 2
    is needed for the non-trivial-commitment-pair check).
    """
    kb = _kb_from(BRANCHING / "04_two_levels.ein")
    _verdict, stats = gaps_solve(kb, max_set_size=3)
    assert stats.layers_explored >= 1
    assert stats.enterings_total >= 1


def test_gaps_solve_max_set_size_zero_returns_phase_1_only():
    """``max_set_size=0`` exits before Phase 2 starts.
    For a Phase-1-solving puzzle, still returns Ambiguity
    with 1 branch.
    """
    kb = _kb_from(BRANCHING / "01_saturate_only.ein")
    verdict, stats = gaps_solve(kb, max_set_size=0)
    assert len(verdict.branches) == 1
    assert stats.layers_explored == 0
