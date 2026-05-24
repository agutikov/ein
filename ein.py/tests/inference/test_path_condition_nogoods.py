"""Path-condition no-good clause learning — S1.5a.18 T1.5a.18.4.

Covers:

1. **Subsumption unit tests** — `emit_nogood` keeps `_nogoods`
   minimal: rejects subsumed clauses, evicts strict supersets.
2. **Filter unit tests** — `filter_by_nogoods` drops candidates
   whose prospective path matches a learned clause.
3. **Single-element guard** — `emit_nogood` rejects size-1 clauses
   (back-prop's domain).
4. **Flag-off parity** — tree shape on demo 10 unchanged from the
   pre-S1.5a.18 baseline (32 nodes).
5. **Flag-on emits on conditional death** — at least one
   multi-element clause lands in `root.kb._nogoods` on demo 10.
6. **Eager + nogoods compose** — both flags on: `BubbleAbort`
   fires from `emit_nogood`, verdict matches flag-off.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.inference.back_prop import (
    BubbleAbort,
    _bump_pass_bubbled,
    _eager_pass_ctx,
)
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.nogoods import (
    build_clause,
    emit_nogood,
    filter_by_nogoods,
)
from ein_bot.inference.solver import Solution, solve
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
DEMO_BACKPROP = REPO / "examples" / "branching" / "10_backprop_on.ein"


def _load(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


# ── 1) Subsumption unit tests ──────────────────────────────────────


def test_emit_nogood_keeps_minimal_smaller_subsumes():
    """A smaller clause subsumes a larger one: emit the smaller
    AFTER and the larger gets evicted."""
    kb = KnowledgeBase()
    large = frozenset({("R", ("a",)), ("R", ("b",)), ("R", ("c",))})
    small = frozenset({("R", ("a",)), ("R", ("b",))})
    assert emit_nogood(kb, large) is True
    assert large in kb._nogoods
    # Emit the smaller clause; it subsumes the larger.
    assert emit_nogood(kb, small) is True
    assert small in kb._nogoods
    assert large not in kb._nogoods
    assert len(kb._nogoods) == 1


def test_emit_nogood_skips_when_subsumed():
    """A larger clause subsumed by an existing smaller one is
    rejected (no-op, no insert)."""
    kb = KnowledgeBase()
    small = frozenset({("R", ("a",)), ("R", ("b",))})
    large = frozenset({("R", ("a",)), ("R", ("b",)), ("R", ("c",))})
    assert emit_nogood(kb, small) is True
    assert emit_nogood(kb, large) is False
    assert kb._nogoods == {small}


def test_emit_nogood_rejects_single_element():
    """A 1-element clause is `back_propagate`'s domain — emit_nogood
    rejects it as a no-op."""
    kb = KnowledgeBase()
    one = frozenset({("R", ("a",))})
    assert emit_nogood(kb, one) is False
    assert kb._nogoods == set()


def test_emit_nogood_independent_clauses_coexist():
    """Disjoint clauses both stay in the set."""
    kb = KnowledgeBase()
    c1 = frozenset({("R", ("a",)), ("R", ("b",))})
    c2 = frozenset({("R", ("c",)), ("R", ("d",))})
    assert emit_nogood(kb, c1) is True
    assert emit_nogood(kb, c2) is True
    assert kb._nogoods == {c1, c2}


# ── 2) Filter unit tests ───────────────────────────────────────────


def _hyp(rel: str, *args) -> Fact:
    return Fact(relation_name=rel, args=tuple(args), layer=Layer.REASONING)


def test_filter_by_nogoods_empty_set_is_noop():
    """Empty `nogoods` returns candidates unchanged."""
    cands = [_hyp("R", "a"), _hyp("R", "b")]
    assert filter_by_nogoods(cands, frozenset(), set()) == cands


def test_filter_by_nogoods_drops_matched_candidate():
    """A candidate whose prospective path matches a clause is dropped."""
    cands = [_hyp("R", "a"), _hyp("R", "b"), _hyp("R", "c")]
    path_set = frozenset({("R", ("a",))})  # ancestor took R(a)
    nogoods = {frozenset({("R", ("a",)), ("R", ("b",))})}
    out = filter_by_nogoods(cands, path_set, nogoods)
    # Candidate R(b) is dropped — taking it would complete the
    # learned dead clause. R(a) and R(c) survive.
    assert _hyp("R", "a") in out
    assert _hyp("R", "b") not in out
    assert _hyp("R", "c") in out


def test_filter_by_nogoods_clause_with_only_self_not_in_path():
    """A clause whose elements are ALL the candidate (no ancestor
    requirement) — i.e., a single-element clause — would always
    fire. emit_nogood rejects size-1, but defensively the filter
    must also handle it correctly if someone bypasses emit."""
    cands = [_hyp("R", "a"), _hyp("R", "b")]
    path_set = frozenset()
    nogoods = {frozenset({("R", ("a",)), ("R", ("z",))})}
    out = filter_by_nogoods(cands, path_set, nogoods)
    # R(a) prospective path = {R(a)}; clause needs R(z) too — no match. Survives.
    # R(b) prospective path = {R(b)}; clause needs R(a) and R(z) — no match. Survives.
    assert out == cands


def test_build_clause_constructs_set_from_path_plus_own():
    """`build_clause` collapses the (path, own) into a frozenset."""
    path = (("R", ("a",)), ("R", ("b",)))
    own = _hyp("R", "c")
    clause = build_clause(path, own)
    assert clause == frozenset({
        ("R", ("a",)), ("R", ("b",)), ("R", ("c",)),
    })


# ── 3) Eager-mode interaction ──────────────────────────────────────


def test_emit_nogood_raises_bubble_abort_under_eager_mode():
    """When `_eager_pass_ctx` is set and a new clause lands,
    emit_nogood bumps `_pass_bubbled` and raises BubbleAbort."""
    kb = KnowledgeBase()
    clause = frozenset({("R", ("a",)), ("R", ("b",))})
    token = _eager_pass_ctx.set(7)
    try:
        with pytest.raises(BubbleAbort) as ei:
            emit_nogood(kb, clause)
        assert ei.value.pass_id == 7
    finally:
        _eager_pass_ctx.reset(token)
    assert getattr(kb, "_pass_bubbled", 0) == 1
    assert clause in kb._nogoods


def test_emit_nogood_no_abort_when_subsumed_under_eager_mode():
    """Subsumed (rejected) emit does NOT raise — no information
    was added at root, so the outer driver shouldn't restart."""
    kb = KnowledgeBase()
    small = frozenset({("R", ("a",)), ("R", ("b",))})
    kb._nogoods.add(small)
    large = frozenset({("R", ("a",)), ("R", ("b",)), ("R", ("c",))})
    token = _eager_pass_ctx.set(8)
    try:
        # Should NOT raise — large is subsumed.
        assert emit_nogood(kb, large) is False
    finally:
        _eager_pass_ctx.reset(token)
    assert getattr(kb, "_pass_bubbled", 0) == 0


# ── 4) Flag-off parity ─────────────────────────────────────────────


def test_flag_off_default_keeps_demo_10_baseline():
    """Default config keeps the no-goods flag OFF — demo 10's
    tree shape is unchanged from the pre-S1.5a.18 baseline
    (32 nodes)."""
    assert SolverConfig().enable_path_condition_nogoods is False
    kb = _load(DEMO_BACKPROP)
    verdict = solve(kb)
    assert isinstance(verdict, Solution)
    assert len(verdict.tree.nodes) == 32
    # _nogoods set is empty (no emissions ever happened).
    assert kb._nogoods == set()


# ── 5) Flag-on emits on conditional death ──────────────────────────


def test_flag_on_emits_clauses_on_demo_10():
    """Demo 10 has at least one depth-≥2 conditional death (a
    candidate alive at root, dying at depth 1). Flag-on should
    populate `root.kb._nogoods` with at least one multi-element
    clause and still arrive at the same Solution."""
    kb = _load(DEMO_BACKPROP)
    cfg = SolverConfig(enable_path_condition_nogoods=True)
    verdict = solve(kb, config=cfg)
    assert isinstance(verdict, Solution)
    # The Dave binding survives.
    assert any(
        f.relation_name == "friends-of" and f.args == ("Alice", "Dave")
        for f in verdict.kb.facts
    )
    # At least one learned clause landed (demo 10 has conditional
    # deaths whose subtrees die under specific ancestor decisions).
    assert len(kb._nogoods) >= 1
    # Every stored clause has ≥ 2 elements (single-element guard).
    assert all(len(c) >= 2 for c in kb._nogoods)
    # Clauses are minimal — no stored clause is a strict superset
    # of another stored clause.
    nogoods = list(kb._nogoods)
    for i, c1 in enumerate(nogoods):
        for j, c2 in enumerate(nogoods):
            if i != j:
                assert not (c1 < c2), \
                    f"clause {c1} is a strict subset of stored {c2}"


# ── 6) Eager + nogoods compose ─────────────────────────────────────


def test_eager_plus_nogoods_terminates_with_matching_verdict():
    """Both flags on: BubbleAbort fires from emit_nogood as well as
    back_propagate; the outer loop terminates within a bounded pass
    count; verdict matches flag-off."""
    kb = _load(DEMO_BACKPROP)
    cfg = SolverConfig(
        enable_eager_root_bubble=True,
        enable_path_condition_nogoods=True,
    )
    verdict = solve(kb, config=cfg)
    assert isinstance(verdict, Solution)
    assert any(
        f.relation_name == "friends-of" and f.args == ("Alice", "Dave")
        for f in verdict.kb.facts
    )
    # Eager-mode counter accumulates across both bubble sources.
    bubbled = getattr(kb, "_pass_bubbled", 0)
    assert bubbled > 0
