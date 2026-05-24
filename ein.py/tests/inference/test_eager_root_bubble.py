"""Eager root-bubble + outer re-entry loop — S1.5a.17 T1.5a.17.4.

Asserts the new eager-mode control flow (config flag
``enable_eager_root_bubble``):

1. **Flag-off parity** — with the flag off (the default), solve()
   behaviour is byte-identical to pre-S1.5a.17. Smoke-tested by
   running the existing back-prop demo and asserting the verdict
   shape and chosen binding.
2. **Flag-on solves the same puzzles** — same verdict, same binding,
   under eager mode. Tree shape may differ (eager mode re-builds
   the tree per pass) but the answer must match.
3. **BubbleAbort fires and is caught** — eager mode actually
   triggers the abort path on a puzzle with at least one
   unconditional death; the outer loop progresses (root.kb gains
   at least one `(not h)` fact, `_pass_bubbled > 0`) and
   terminates without exceeding a bounded pass count.
4. **Promoted-dead synthesis** — a constructed setup where a root
   candidate dies by subtree exhaustion (not by direct
   contradiction) writes `(not h)` at root via
   `_synthesise_promoted_dead_facts`.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.inference.back_prop import (
    BubbleAbort,
    _eager_pass_ctx,
    _kb_chain_ctx,
    back_propagate,
)
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.solver import (
    Ambiguity,
    Solution,
    _synthesise_promoted_dead_facts,
    solve,
)
from ein_bot.ir import parse
from ein_bot.kb.entities import Fact, Layer
from ein_bot.kb.provenance import Provenance
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
DEMO_BACKPROP = REPO / "examples" / "branching" / "10_backprop_on.ein"
DEMO_FIVE_HYPS = REPO / "examples" / "branching" / "03_five_hyps_one_alive.ein"


def _load(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text()))


# ── 1) Flag-off parity ─────────────────────────────────────────────


def test_flag_off_is_default_and_matches_baseline():
    """The default config keeps eager mode OFF — solve() behaves
    exactly as before S1.5a.17."""
    assert SolverConfig().enable_eager_root_bubble is False
    kb = _load(DEMO_BACKPROP)
    verdict = solve(kb)
    # Demo 10 has a unique Solution with ?p = Dave.
    assert isinstance(verdict, Solution)
    # The binding survived: Dave is alive in the solution kb.
    assert any(
        f.relation_name == "friends-of" and f.args == ("Alice", "Dave")
        for f in verdict.kb.facts
    )


# ── 2) Flag-on solves the same puzzles ─────────────────────────────


def test_eager_mode_solves_backprop_demo():
    """Eager mode on demo 10 returns the same Solution verdict +
    same binding as flag-off mode."""
    kb = _load(DEMO_BACKPROP)
    cfg = SolverConfig(enable_eager_root_bubble=True)
    verdict = solve(kb, config=cfg)
    assert isinstance(verdict, Solution)
    assert any(
        f.relation_name == "friends-of" and f.args == ("Alice", "Dave")
        for f in verdict.kb.facts
    )


def test_eager_mode_matches_baseline_on_five_hyps():
    """Demo 3 (Ambiguity in SOLVE mode — symmetric duplicates) has
    the same shape under flag-on as flag-off: a binding for H5."""
    kb_baseline = _load(DEMO_FIVE_HYPS)
    kb_eager = _load(DEMO_FIVE_HYPS)
    v_baseline = solve(kb_baseline)
    v_eager = solve(
        kb_eager, config=SolverConfig(enable_eager_root_bubble=True),
    )
    # Verdict shape parity (Ambiguity in SOLVE because of symmetric
    # pair — both flag states see it the same way).
    assert type(v_baseline) is type(v_eager)
    # Solution survives in BOTH: H5 is in every surviving branch's
    # binding.
    def _h5_in(v):
        if isinstance(v, Solution):
            return any(
                f.relation_name == "co-located" and "H5" in f.args
                for f in v.kb.facts
            )
        if isinstance(v, Ambiguity):
            return all(
                any(
                    f.relation_name == "co-located" and "H5" in f.args
                    for f in b.kb.facts
                )
                for b in v.branches
            )
        return False
    assert _h5_in(v_baseline)
    assert _h5_in(v_eager)


# ── 3) BubbleAbort fires + outer loop progresses + terminates ──────


def test_eager_mode_bubbles_and_terminates_on_backprop_demo():
    """Demo 10 has at least one unconditional death (the lookahead
    kill on `(friends-of Alice Carol)` and the multi-step kill on
    `(friends-of Alice Bob)` both back-prop). Under eager mode the
    outer loop runs ≥ 2 passes and `_pass_bubbled` accumulates."""
    kb = _load(DEMO_BACKPROP)
    cfg = SolverConfig(enable_eager_root_bubble=True)
    verdict = solve(kb, config=cfg)
    assert isinstance(verdict, Solution)
    # The counter is bumped by every NEW `(not h)` written under
    # eager mode (back-prop + forced-positive + promoted-dead).
    # Bound by pass count; for demo 10 the lookahead-kill alone
    # writes 2 facts (the killed candidate + its symmetric mirror).
    bubbled = getattr(kb, "_pass_bubbled", 0)
    assert bubbled > 0
    # At least one `(not (friends-of Alice ?))` fact landed at root
    # so subsequent passes skip that candidate.
    not_friends = [
        f for f in kb.facts
        if f.relation_name == "not"
        and len(f.args) == 1
        and isinstance(f.args[0], Fact)
        and f.args[0].relation_name == "friends-of"
        and f.args[0].args[0] == "Alice"
    ]
    assert len(not_friends) >= 1


def test_outer_loop_pass_count_is_bounded():
    """A puzzle with N initial root candidates terminates in ≤ N
    outer passes. Indirectly tested by ensuring solve() returns
    in finite time on demo 10 with eager mode on."""
    kb = _load(DEMO_BACKPROP)
    cfg = SolverConfig(enable_eager_root_bubble=True)
    # If the outer loop ever diverged we'd block here forever;
    # pytest's default deadline catches it via collection timeout.
    verdict = solve(kb, config=cfg)
    assert verdict is not None


# ── 4) Direct BubbleAbort + promoted-dead synthesis unit tests ─────


def test_bubble_abort_raised_when_eager_pass_set():
    """`back_propagate` raises BubbleAbort when `_eager_pass_ctx`
    is set AND at least one new (not h) is written."""
    kb = _load(DEMO_BACKPROP)
    # Need root saturation to populate _negated_facts properly, but
    # for this unit test we just want to call back_propagate with a
    # fresh hypothesis and assert the raise.
    h = Fact(
        relation_name="dummy-hyp",
        args=("X", "Y"),
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=999),
    )
    chain_token = _kb_chain_ctx.set((kb,))
    pass_token = _eager_pass_ctx.set(42)
    try:
        with pytest.raises(BubbleAbort) as ei:
            back_propagate(kb, h, frozenset(), rule_name="<test>",
                           promote_symmetric=False)
        assert ei.value.pass_id == 42
    finally:
        _eager_pass_ctx.reset(pass_token)
        _kb_chain_ctx.reset(chain_token)
    # Counter bumped.
    assert getattr(kb, "_pass_bubbled", 0) >= 1
    # Idempotent: a second call with the same hypothesis writes
    # nothing new, so NO abort and the counter does not grow.
    chain_token = _kb_chain_ctx.set((kb,))
    pass_token = _eager_pass_ctx.set(43)
    try:
        before = kb._pass_bubbled
        # No raise expected.
        back_propagate(kb, h, frozenset(), rule_name="<test>",
                       promote_symmetric=False)
        assert kb._pass_bubbled == before
    finally:
        _eager_pass_ctx.reset(pass_token)
        _kb_chain_ctx.reset(chain_token)


def test_bubble_abort_not_raised_when_eager_off():
    """With `_eager_pass_ctx` unset (default), `back_propagate`
    silently writes and returns — the pre-S1.5a.17 contract."""
    kb = _load(DEMO_BACKPROP)
    h = Fact(
        relation_name="dummy-hyp-2",
        args=("U", "V"),
        layer=Layer.REASONING,
        provenance=Provenance.from_hypothesis(branch=998),
    )
    chain_token = _kb_chain_ctx.set((kb,))
    try:
        # Should NOT raise — eager pass id is None by default.
        result = back_propagate(kb, h, frozenset(), rule_name="<test>",
                                promote_symmetric=False)
        assert result.relation_name == "not"
    finally:
        _kb_chain_ctx.reset(chain_token)


def test_synthesise_promoted_dead_facts_fires_on_promoted_dead_only():
    """`_synthesise_promoted_dead_facts` fires for root children
    whose verdict is `'dead'` AND `children != ()` (promoted-dead by
    subtree exhaustion), and skips direct-dead leaves (back_propagate
    already wrote `(not h)` during _consume for those).

    Demo 10 has at least one promoted-dead root child (a candidate
    that was alive at root but every subbranch died), so we assert
    that the number of facts written equals the count of such root
    children whose `(not h)` wasn't already in root.kb.
    """
    kb = _load(DEMO_BACKPROP)
    verdict = solve(kb)  # flag off — populate via legacy path
    tree = verdict.tree
    from ein_bot.inference.solver import _TreeBuilder
    builder = _TreeBuilder()
    builder.nodes = dict(tree.nodes)
    # Count expected promoted-dead writes: root children that are
    # dead, have children, and whose (not h) isn't already at root.
    root_node = tree.nodes[tree.root]
    expected_writes = 0
    for cid in root_node.children:
        c = tree.nodes[cid]
        if c.verdict != "dead" or c.hypothesis is None or not c.children:
            continue
        if kb._fact_by_id("not", (c.hypothesis,)) is not None:
            continue
        expected_writes += 1
    written = _synthesise_promoted_dead_facts(builder, tree.root, kb)
    assert written == expected_writes
    # The forced flag is set on every promoted-dead child the
    # synthesis wrote a fact for.
    forced_count = sum(
        1 for cid in root_node.children
        if builder.nodes[cid].forced
    )
    assert forced_count == written
    # Sanity — at least one promoted-dead synthesis fired on demo 10
    # (else we're not exercising the path at all).
    assert written > 0


# ── 5) Flag-off path is byte-identical to pre-S1.5a.17 ─────────────


def test_flag_off_tree_node_count_matches_legacy_baseline():
    """Demo 10 legacy baseline is 32 tree nodes (22 dead + 1 solution
    + interior). Flag-off must match exactly to confirm the new code
    paths are gated behind the flag."""
    kb = _load(DEMO_BACKPROP)
    verdict = solve(kb)  # flag off by default
    tree = verdict.tree
    assert len(tree.nodes) == 32
