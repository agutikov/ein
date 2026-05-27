"""Monotonic backbone tests — S1.5b.5 T1.5b.5.3.

Pins :func:`ein_bot.inference.monotonic.monotonic_solve` across
its three verdict types + the stats counters. Fixtures are
designed for what the *minimal* backbone can do — no
forced-positive promotion (deferred to a later stage), no CDCL
nogoods (S1.5b.6), no dumper (S1.5b.7). See
[stage Ship notes][1] for the branching/03 gap.

[1]: ../../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/s1.5b.5_monotonic_backbone.md
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.inference.monotonic import monotonic_solve
from ein_bot.inference.monotonic.solver import (
    MonotonicStats,
)
from ein_bot.inference.monotonic.solver import (
    monotonic_solve as _direct,
)
from ein_bot.inference.monotonic.state_dump import MonotonicDumper
from ein_bot.inference.tree.solver import (
    Ambiguity,
    Contradiction,
    Mode,
    Solution,
)
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# ── Imports + signature ────────────────────────────────────────────


def test_imports_resolve():
    assert monotonic_solve is _direct
    assert MonotonicDumper is not None


def test_returns_tuple_shape():
    """`monotonic_solve` returns ``(verdict, stats)``."""
    kb = _kb("""
    (ontology (type T) (relation r T) (instance a T))
    (facts (r a :source "(1)"))
    (query :mode solve :goal (r ?x))
    """)
    result = monotonic_solve(kb, max_set_size=1)
    assert isinstance(result, tuple)
    assert len(result) == 2
    verdict, stats = result
    assert isinstance(verdict, (Solution, Ambiguity, Contradiction))
    assert isinstance(stats, MonotonicStats)


def test_non_solve_modes_raise():
    """GAPS / CONTRADICTIONS belong to the lattice engine (Q1.5b.7)."""
    kb = _kb("(ontology (type T))")
    with pytest.raises(NotImplementedError):
        monotonic_solve(kb, mode=Mode.GAPS)
    with pytest.raises(NotImplementedError):
        monotonic_solve(kb, mode=Mode.CONTRADICTIONS)


# ── Trivial / root-only ────────────────────────────────────────────


def test_trivial_root_only_solve():
    """Root saturation alone satisfies the goal — no hypothesis
    entering needed. `sym-r` derives `(r b a)` from `(r a b)`; goal
    `(r b ?x)` is satisfied by ?x=a after saturation.
    """
    kb = _kb("""
    (rules
      (rule sym-r ()
        :match (r ?x ?y) :assert (r ?y ?x)
        :why "symmetric r" :priority 100))
    (ontology
      (type T)
      (relation r T T)
      (instance a T) (instance b T))
    (facts (r a b :source "(1)"))
    (query :mode solve :goal (r b ?x))
    """)
    verdict, stats = monotonic_solve(kb, max_set_size=1)
    assert isinstance(verdict, Solution)
    # Phase 1 short-circuit: 1 saturate, 0 enterings.
    assert stats.saturate_count == 1
    assert stats.enterings_total == 0
    assert stats.layers_explored == 0


def test_root_contradiction_returns_contradiction():
    """Root saturates and a rule derives `(false)` directly →
    Phase 1's ContradictionDetector flags it before any hypothesis
    entering."""
    kb = _kb("""
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
    verdict, stats = monotonic_solve(kb, max_set_size=1)
    assert isinstance(verdict, Contradiction)
    assert stats.enterings_total == 0  # never entered Phase 2


# ── Ambiguity (correct verdict when puzzle has multiple solutions) ─


def test_branching_04_returns_ambiguity():
    """examples/branching/04_two_levels.ein has TWO valid answers
    (Blue↔H3 or Green↔H3) — Ambiguity is the *correct* verdict per
    the demo's own header comment.
    """
    text = (BRANCHING / "04_two_levels.ein").read_text()
    kb = KnowledgeBase.from_ir(parse(text))
    verdict, _ = monotonic_solve(kb, max_set_size=3)
    assert isinstance(verdict, Ambiguity)


# ── Forced-positive promotion (S1.5b.5b) ──────────────────────────


def test_branching_03_solves_via_forced_positive():
    """examples/branching/03_five_hyps_one_alive.ein: lookahead
    + symmetric-canonicalised hypgen shrinks alive to the
    singleton `{(co-located White H5)}` right after Phase 1's
    initial saturation. Forced-positive promotion merges it
    into root, re-saturation derives `(co-located White H5)`
    + its symmetric pair, goal `(co-located White ?h)`
    matches ?h=H5, return Solution.

    Pre-S1.5b.5b: returned Ambiguity (the conditional-fact
    extraction never merged h_unique into root).
    """
    text = (BRANCHING / "03_five_hyps_one_alive.ein").read_text()
    kb = KnowledgeBase.from_ir(parse(text))
    verdict, stats = monotonic_solve(kb, max_set_size=3)
    assert isinstance(verdict, Solution)
    assert stats.forced_positives >= 1
    # Goal binding check via the bench's helper-mirror.
    from ein_bot.inference.compile import JoinPlan, compile_pattern
    from ein_bot.inference.match import run as match_run
    goal = next(
        kp.value for kp in verdict.kb.query.kw_pairs
        if kp.key.name == "goal"
    )
    plan = JoinPlan(
        rule_name="<query>", activator_args=(), bindings_seed={},
        steps=tuple(compile_pattern(goal, {})),
        assert_template=None, why="",
    )
    rows = [dict(b) for b, _premises in match_run(plan, verdict.kb)]
    assert rows == [{"h": "H5"}]


def test_zebra2_solves_via_monotonic_backbone():
    """zebra2 — the M1 acceptance puzzle — solves under the
    monotonic backbone with max_set_size=2. Tree-side answer:
    h_water=House-1, h_zebra=House-5. Runs in ~5 s on CPython
    (1.6 s on PyPy) — fast enough for the regular suite, no
    EIN_RUN_SLOW gate needed.
    """
    text = (REPO / "examples" / "zebra2.ein").read_text()
    kb = KnowledgeBase.from_ir(parse(text))
    verdict, _ = monotonic_solve(kb, max_set_size=2)
    assert isinstance(verdict, Solution)


# ── Stats correctness ─────────────────────────────────────────────


def test_stats_counters_no_rules():
    """A no-rules puzzle with 2 leaf instances ⇒ hypgen yields 2
    candidates `(h A B)` and `(h B A)`. No contradictions, no
    derivations — every entering is alive, no facts merged
    (hypotheses themselves are conditional). Layer 1 tries 2;
    Layer 2 tries 1 (the prefix-join 2-element set). Verdict:
    Ambiguity at layer 2.
    """
    kb = _kb("""
    (ontology
      (relation h T T)
      (is-a Thing T)
      (is-a A Thing) (is-a B Thing))
    (facts)
    (query :mode solve :goal (h X Y))
    """)
    verdict, stats = monotonic_solve(kb, max_set_size=2)
    assert isinstance(verdict, Ambiguity)
    # Hand-computed: 2 layer-1 enterings + 1 layer-2 (prefix-join
    # of the 2 singletons) = 3 total, all alive.
    assert stats.enterings_total == 3
    assert stats.enterings_alive == 3
    assert stats.enterings_dead_pre == 0
    assert stats.enterings_dead_post == 0
    # No rules ⇒ no derivations ⇒ no unconditional facts to merge.
    assert stats.facts_merged == 0
    # Phase 1 saturate only — no merges trigger re-saturate.
    assert stats.saturate_count == 1
    assert stats.layers_explored == 2
