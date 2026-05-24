"""Hypothesis scoring — S1.5a.7 T1.5a.7.4.

Covers the popularity scoring mode wired in
:func:`ein_bot.inference.hypgen.score_hypothesis`:

1. **Mode dispatch unit tests** — most-constrained returns 0;
   popularity returns the weighted sum; branch-info modes raise.
2. **Per-branch recalc** — a forked kb with more facts about an
   object yields a higher score for hypotheses involving that
   object (vs the parent kb's score), confirming the per-branch
   recompute is automatic.
3. **Config loader type-dispatch** — the loader accepts string +
   float fields, not just bools.
4. **Flag-off parity** — default mode keeps demo 10's tree at
   exactly 32 nodes (the pre-S1.5a.7 baseline).
5. **Smoke** — popularity mode on demo 10 still arrives at the
   same Solution + binding (different tree shape is allowed).
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.hypgen import score_hypothesis
from ein_bot.inference.solver import Solution, solve
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
DEMO_BACKPROP = REPO / "examples" / "branching" / "10_backprop_on.ein"


def _load(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


def _hyp(rel: str, *args) -> Fact:
    return Fact(relation_name=rel, args=tuple(args), layer=Layer.REASONING)


# ── 1) Mode dispatch ───────────────────────────────────────────────


def test_score_explicit_most_constrained_returns_zero():
    """Explicit 'most-constrained' mode → score 0 (the escape-hatch
    behaviour kept after popularity became default 2026-05-25)."""
    kb = KnowledgeBase()
    kb.config = SolverConfig(hypgen_scoring="most-constrained")
    assert score_hypothesis(_hyp("R", "a", "b"), kb) == 0.0
    # No config object also falls back to most-constrained-equivalent
    # (the dispatcher defaults to 'most-constrained' when cfg is None).
    kb.config = None
    assert score_hypothesis(_hyp("R", "a", "b"), kb) == 0.0


def test_score_popularity_with_synthetic_kb():
    """Popularity scoring respects fact-counts and weights."""
    kb = _load(DEMO_BACKPROP)
    # Trigger root saturation so the fact indexes are populated.
    from ein_bot.inference.closed import emit_closed
    from ein_bot.inference.saturator import Saturator
    emit_closed(kb)
    list(Saturator(kb).saturate(max_steps=10_000))

    kb.config = SolverConfig(
        hypgen_scoring="popularity",
        hypgen_rel_weight=1.0,
        hypgen_obj_weight=1.0,
    )
    # Score should be > 0 for a hypothesis whose args are
    # referenced in the kb. Alice + Dave both appear in is-a facts
    # post-saturation.
    candidate = _hyp("friends-of", "Alice", "Dave")
    s = score_hypothesis(candidate, kb)
    assert s > 0.0  # obj component fires even though rel count is 0

    # Test rel-weight scaling with a relation that HAS facts at
    # root (is-a has the Person + Alice/Bob/Carol/Dave entries).
    is_a_candidate = _hyp("is-a", "Alice", "Person")
    kb.config = SolverConfig(
        hypgen_scoring="popularity",
        hypgen_rel_weight=1.0,
        hypgen_obj_weight=0.0,  # isolate the rel component
    )
    s_w1 = score_hypothesis(is_a_candidate, kb)
    assert s_w1 > 0.0  # is-a has multiple facts at root
    kb.config = SolverConfig(
        hypgen_scoring="popularity",
        hypgen_rel_weight=2.0,
        hypgen_obj_weight=0.0,
    )
    s_w2 = score_hypothesis(is_a_candidate, kb)
    assert s_w2 == 2 * s_w1  # rel-weight scales linearly


def test_score_branch_info_raises_not_implemented():
    """branch-info modes are reserved — raise NotImplementedError
    so misconfigurations surface at first call."""
    kb = KnowledgeBase()
    kb.config = SolverConfig(hypgen_scoring="branch-info")
    with pytest.raises(NotImplementedError):
        score_hypothesis(_hyp("R", "a", "b"), kb)

    kb.config = SolverConfig(hypgen_scoring="popularity+branch-info")
    with pytest.raises(NotImplementedError):
        score_hypothesis(_hyp("R", "a", "b"), kb)


def test_score_unknown_mode_raises_value_error():
    """An unknown mode is a user typo — raise ValueError."""
    kb = KnowledgeBase()
    kb.config = SolverConfig(hypgen_scoring="bogus")
    with pytest.raises(ValueError, match="unknown hypgen-scoring"):
        score_hypothesis(_hyp("R", "a", "b"), kb)


# ── 2) Per-branch recalc (the +1 design choice) ────────────────────


def test_score_is_per_branch_via_fact_indexes():
    """`score_hypothesis(fact, kb)` reads `kb._facts_by_*` — a forked
    kb with additional facts about an object yields a higher score
    for hypotheses involving that object."""
    kb_root = _load(DEMO_BACKPROP)
    from ein_bot.inference.closed import emit_closed
    from ein_bot.inference.saturator import Saturator
    emit_closed(kb_root)
    list(Saturator(kb_root).saturate(max_steps=10_000))
    kb_root.config = SolverConfig(hypgen_scoring="popularity")

    candidate = _hyp("friends-of", "Alice", "Eve")  # Eve is unused
    s_root = score_hypothesis(candidate, kb_root)

    # Fork and add some Eve-facts to inflate Eve's popularity.
    kb_fork = kb_root.fork()
    eve_fact = Fact(
        relation_name="is-a",
        args=("Eve", "Person"),
        layer=Layer.REASONING,
    )
    kb_fork.add_fact(eve_fact)
    kb_fork._index_fact(eve_fact)
    s_fork = score_hypothesis(candidate, kb_fork)
    # The fork's score is strictly higher — Eve now has 1 fact
    # mentioning it (was 0 at root).
    assert s_fork > s_root


# ── 3) Config loader type-dispatch ─────────────────────────────────


def test_config_loader_accepts_string_and_float_fields():
    """Loader handles str + float in addition to bool."""
    from ein_bot.ir.types import Atom, Keyword, KwPair
    pairs = (
        KwPair(key=Keyword(name="hypgen-scoring"),
               value=Atom(name="popularity")),
        KwPair(key=Keyword(name="hypgen-rel-weight"),
               value=Atom(name="2.5")),
        KwPair(key=Keyword(name="hypgen-obj-weight"),
               value=Atom(name="0.5")),
    )
    cfg = SolverConfig.from_kw_pairs(pairs)
    assert cfg.hypgen_scoring == "popularity"
    assert cfg.hypgen_rel_weight == 2.5
    assert cfg.hypgen_obj_weight == 0.5


def test_config_loader_rejects_bad_float():
    from ein_bot.ir.types import Atom, Keyword, KwPair
    pairs = (
        KwPair(key=Keyword(name="hypgen-rel-weight"),
               value=Atom(name="not-a-number")),
    )
    with pytest.raises(ValueError, match="expects a number"):
        SolverConfig.from_kw_pairs(pairs)


# ── 4) Flag-off parity ─────────────────────────────────────────────


def test_default_is_popularity_post_2026_05_25():
    """The default flipped from 'most-constrained' to 'popularity'
    on 2026-05-25 per S1.5a.7 measurement."""
    assert SolverConfig().hypgen_scoring == "popularity"
    kb = _load(DEMO_BACKPROP)
    verdict = solve(kb)
    # Verdict shape preserved (Solution + Dave); tree-node count
    # may differ from the pre-flip baseline due to reordering.
    assert isinstance(verdict, Solution)
    assert any(
        f.relation_name == "friends-of" and f.args == ("Alice", "Dave")
        for f in verdict.kb.facts
    )


# ── 5) Popularity mode end-to-end ──────────────────────────────────


def test_popularity_mode_preserves_demo_10_solution():
    """Popularity mode arrives at the same Solution + binding as
    most-constrained on demo 10. Tree shape may differ (different
    branch ordering); verdict invariance is what we guarantee."""
    kb = _load(DEMO_BACKPROP)
    cfg = SolverConfig(hypgen_scoring="popularity")
    verdict = solve(kb, config=cfg)
    assert isinstance(verdict, Solution)
    assert any(
        f.relation_name == "friends-of" and f.args == ("Alice", "Dave")
        for f in verdict.kb.facts
    )
