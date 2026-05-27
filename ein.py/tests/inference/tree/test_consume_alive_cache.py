"""S1.5.7b — consume-loop verdict cache (stable-alive + stable-conditional-dead).

The cache caps ``_consume`` so each candidate's ``try_branch`` runs
exactly once per ``_consume`` call (under M1, no invalidation fires);
conditional-dead candidates are recorded as dead ``SearchNode``s
inline rather than re-tried by ``_descend``.

These tests pin:

- :class:`Firing.derives_positive` (T1.5.7b.5a) — the invalidation
  predicate, always False under M1, True for forced-positive
  derivations (S1.5.7 T1.5.7.6 / S1.5.8 T1.5.8.3).
- Per-solve ``ConsumeStats`` counters (T1.5.7b.2) — observable via
  ``kb.consume_stats``.
- Stable-alive cache fires on the demo 10/11 multi-sweep puzzle
  (T1.5.7b.1).
- Verdict equivalence across the branching demos with the cache
  on, on, and (for 11) off (T1.5.7b.6).
- Cache invalidation clears the alive verdict cache when a
  ``derives_positive`` firing appears in the re-saturation delta
  (T1.5.7b.5 — direct unit-level coverage of the guard).
"""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.firing import Firing
from ein_bot.inference.tree.solver import (
    Ambiguity,
    ConsumeStats,
    Solution,
    solve,
)
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


# ── T1.5.7b.5a — Firing.derives_positive ───────────────────────────


def test_firing_derives_positive_positive_conclusion():
    """A firing whose `derived` is a positive fact returns True."""
    positive = Fact(relation_name="co-located", args=("A", "B"),
                    layer=Layer.REASONING)
    f = Firing(rule="implies", activator=("friends-of", "buddy"),
               bindings={}, derived=positive, premises=())
    assert f.derives_positive() is True


def test_firing_derives_positive_negation_conclusion():
    """A firing whose `derived` is a `(not …)` fact returns False —
    the universal case under the M1 rule library (symmetric promotion,
    sibling-exclusive, etc.)."""
    inner = Fact(relation_name="buddy", args=("A", "B"),
                 layer=Layer.REASONING)
    negation = Fact(relation_name="not", args=(inner,),
                    layer=Layer.REASONING)
    f = Firing(rule="explicit-no-buddy", activator=(),
               bindings={}, derived=negation, premises=())
    assert f.derives_positive() is False


# ── T1.5.7b.2 — ConsumeStats seeded on root, shared via fork ───────


def test_consume_stats_seeded_on_root_kb():
    """`solve()` allocates a ConsumeStats on the root kb (zero
    counters when no caching opportunity arises)."""
    kb = KnowledgeBase.from_ir(parse(
        (BRANCHING / "02_one_dead_one_alive.ein").read_text(),
    ))
    solve(kb)
    assert isinstance(kb.consume_stats, ConsumeStats)


def test_consume_stats_shared_across_forks():
    """Forks share the ConsumeStats by reference — every consume call
    accumulates into the same counter instance."""
    kb = KnowledgeBase.from_ir(parse(
        (BRANCHING / "02_one_dead_one_alive.ein").read_text(),
    ))
    kb.consume_stats = ConsumeStats()
    fork = kb.fork()
    assert fork.consume_stats is kb.consume_stats


# ── T1.5.7b.1 — stable-alive cache fires on demo 10 ────────────────


def test_demo_10_alive_cache_fires():
    """Demo 10 has multi-sweep `_consume` calls (unconditional deaths
    trigger re-saturation; previously-alive candidates re-checked).
    With the cache they're skipped: alive_cached_skips > 0."""
    kb = KnowledgeBase.from_ir(parse(
        (BRANCHING / "10_backprop_on.ein").read_text(),
    ))
    solve(kb)
    cs = kb.consume_stats
    assert cs.alive_cached_skips >= 1, cs
    # Demo 10 produces no S1.5.8-shape positives under the M1 rule
    # library — invalidation must never fire here.
    assert cs.cache_invalidations == 0, cs


def test_demo_11_no_cache_activity_under_back_prop_off():
    """Demo 11 turns back-prop off — `_descend` runs instead of
    `_consume`, so no cache counters move."""
    kb = KnowledgeBase.from_ir(parse(
        (BRANCHING / "11_backprop_off.ein").read_text(),
    ))
    solve(kb)
    cs = kb.consume_stats
    assert cs.alive_cached_skips == 0
    assert cs.cond_dead_cached_skips == 0
    assert cs.cache_invalidations == 0


# ── T1.5.7b.6 — verdict equivalence across branching demos ─────────


def _answer(verdict) -> set:
    """Union of the query goal's bindings across solution branches."""
    from ein_bot.inference.compile import JoinPlan, compile_pattern
    from ein_bot.inference.match import run as match_run

    def _solutions(v):
        if isinstance(v, Solution):
            return (v,)
        if isinstance(v, Ambiguity):
            return v.branches
        return ()

    rows: set = set()
    for s in _solutions(verdict):
        kb = s.kb
        if kb is None or kb.query is None:
            continue
        goal = next((kp.value for kp in kb.query.kw_pairs
                     if getattr(kp, "key", None) is not None
                     and kp.key.name == "goal"), None)
        if goal is None:
            continue
        plan = JoinPlan(rule_name="<q>", activator_args=(),
                        bindings_seed={},
                        steps=tuple(compile_pattern(goal, {})),
                        assert_template=None, why="")
        rows |= {tuple(sorted(b.items())) for b, _ in match_run(plan, kb)}
    return rows


def _solve_file(name: str, *, back_prop: bool | None = None):
    """Solve a branching demo; `back_prop=None` honours the file's
    own `(config …)` block (no kwarg override)."""
    kb = KnowledgeBase.from_ir(parse((BRANCHING / name).read_text()))
    cfg = None
    if back_prop is not None:
        cfg = SolverConfig(enable_back_prop_unconditional=back_prop)
    return solve(kb, config=cfg)


def test_verdict_equivalence_across_branching_demos():
    """Cache must not change verdict type or answer set on any
    branching demo, compared to the flag-off baseline."""
    for name in ("02_one_dead_one_alive.ein",
                 "03_five_hyps_one_alive.ein",
                 "04_two_levels.ein",
                 "05_mini_zebra.ein",
                 "10_backprop_on.ein"):
        on = _solve_file(name, back_prop=True)
        off = _solve_file(name, back_prop=False)
        assert type(on) is type(off), name
        assert _answer(on) == _answer(off), name


def test_demo_10_tree_size_unchanged_by_cache():
    """The cache is a `try_branch`-call deduper, not a tree shaper.
    Tree node count on demo 10 must match the pre-S1.5.7b baseline
    (32 nodes — pinned to catch any inadvertent tree growth/shrink)."""
    on = _solve_file("10_backprop_on.ein", back_prop=True)
    assert len(on.tree.nodes) == 32


# ── T1.5.7b.5 — invalidation guard (Firing.derives_positive) ───────


def test_cache_invalidation_clears_alive_entries(monkeypatch):
    """Simulate an S1.5.8 re-saturation: subclass `Saturator` so the
    first re-saturation pass on the *root* kb that already carries
    back-prop writes (i.e. `_consume`'s post-sweep re-saturate, not
    `try_branch`'s fork saturator and not `solve`'s pre-back-prop
    initial saturator) appends a `derives_positive` synthetic firing.
    Verify `cache_invalidations` increments — the only point of
    coverage for the invalidation path before S1.5.8 ships its
    real `domain-elimination` rule (under M1 the predicate is
    universally False).
    """
    from ein_bot.inference import saturator as sat_mod
    from ein_bot.inference.tree import solver as solver_mod

    kb = KnowledgeBase.from_ir(parse(
        (BRANCHING / "10_backprop_on.ein").read_text(),
    ))

    positive = Fact(relation_name="forced", args=("X",),
                    layer=Layer.REASONING)
    fake = Firing(rule="<fake-domain-elimination>", activator=(),
                  bindings={}, derived=positive, premises=())

    fired_once: list[bool] = []
    root_kb = kb

    def has_backprop(target_kb) -> bool:
        for f in target_kb.facts:
            if f.relation_name != "not" or f.provenance is None:
                continue
            if getattr(f.provenance, "rule", None) == "<back-prop-unconditional>":
                return True
        return False

    class InjectingSaturator(sat_mod.Saturator):
        def saturate(self, *args, **kwargs):
            real = list(super().saturate(*args, **kwargs))
            # `_consume`'s re-sat is the unique saturator whose
            # `self.kb is root_kb` AND root_kb already carries
            # back-prop facts. `try_branch`'s fork saturator runs on
            # a fork (different object); `solve`'s initial root
            # saturator runs *before* any back-prop write.
            if (not fired_once
                    and self.kb is root_kb
                    and has_backprop(self.kb)):
                real.append(fake)
                fired_once.append(True)
            return iter(real)

    monkeypatch.setattr(solver_mod, "Saturator", InjectingSaturator)

    solve(kb)
    cs = kb.consume_stats
    assert cs.cache_invalidations >= 1, cs
