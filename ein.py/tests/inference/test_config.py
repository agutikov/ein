"""SolverConfig + `(config …)` IR form — T1.5.4.4."""
from __future__ import annotations

import pytest

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.hypothesis import solve
from ein_bot.ir import parse
from ein_bot.kb.from_ir import KBLoadError
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# ── Defaults ───────────────────────────────────────────────────────


def test_defaults_match_ship_decisions():
    """Defaults follow the per-task ship-stage call in
    s1.5.4_hypgen_improvements.md."""
    c = SolverConfig()
    assert c.enable_alive_inherit is True
    assert c.enable_pre_branch_negated is True
    assert c.enable_pre_branch_lookahead is True
    assert c.enable_back_prop_unconditional is False
    assert c.enable_auto_closure is False
    assert c.print_alive is False


def test_no_config_block_means_kb_config_is_none():
    """Loading a puzzle with no `(config …)` head leaves
    `kb.config` as None — `solve()` falls back to
    SolverConfig() defaults."""
    kb = _kb("(ontology (relation r T T))")
    assert kb.config is None


# ── IR loading ─────────────────────────────────────────────────────


def test_loads_empty_config_block():
    """`(config)` with no body resolves to all defaults."""
    kb = _kb("""
    (ontology (relation r T T))
    (config)
    """)
    assert kb.config == SolverConfig()


def test_loads_single_flag():
    """One :flag override; others stay at defaults."""
    kb = _kb("""
    (ontology (relation r T T))
    (config :print-alive true)
    """)
    assert kb.config is not None
    assert kb.config.print_alive is True
    # Other fields stay default:
    assert kb.config.enable_alive_inherit is True


def test_loads_multiple_flags():
    """All six flags settable in one block."""
    kb = _kb("""
    (ontology (relation r T T))
    (config
      :enable-alive-inherit            false
      :enable-pre-branch-negated       false
      :enable-pre-branch-lookahead     false
      :enable-back-prop-unconditional  true
      :enable-auto-closure             true
      :print-alive                     true)
    """)
    c = kb.config
    assert c.enable_alive_inherit is False
    assert c.enable_pre_branch_negated is False
    assert c.enable_pre_branch_lookahead is False
    assert c.enable_back_prop_unconditional is True
    assert c.enable_auto_closure is True
    assert c.print_alive is True


def test_last_config_block_wins():
    """Two `(config …)` blocks → the second one's flags win."""
    kb = _kb("""
    (ontology (relation r T T))
    (config :print-alive true)
    (config :print-alive false)
    """)
    assert kb.config.print_alive is False


def test_unknown_flag_is_load_error():
    """Typo'd flag surfaces as a `KBLoadError` at parse time, not
    as silent default behaviour."""
    with pytest.raises(KBLoadError, match="unknown config flag :enable-typo"):
        _kb("""
        (ontology (relation r T T))
        (config :enable-typo true)
        """)


def test_non_bool_value_is_load_error():
    """A `:flag <not-a-bool>` rejects at load time."""
    with pytest.raises(KBLoadError, match="expects true/false"):
        _kb("""
        (ontology (relation r T T))
        (config :print-alive maybe)
        """)


# ── solve() kwarg resolution ───────────────────────────────────────


def test_solve_uses_kb_config_when_no_kwarg():
    """`solve(kb)` with no kwarg picks up `kb.config`."""
    kb = _kb("""
    (ontology (relation r T T))
    (facts (r A B :source "(1)"))
    (config :print-alive true)
    (query :mode solve :goal (r A B))
    """)
    solve(kb)
    assert kb.config is not None
    assert kb.config.print_alive is True


def test_solve_kwarg_overrides_kb_config():
    """`solve(kb, config=…)` precedence beats `kb.config`."""
    kb = _kb("""
    (ontology (relation r T T))
    (facts (r A B :source "(1)"))
    (config :print-alive true)
    (query :mode solve :goal (r A B))
    """)
    override = SolverConfig(print_alive=False)
    solve(kb, config=override)
    # kb.config is updated to the effective config (the override):
    assert kb.config.print_alive is False


def test_solve_with_no_config_falls_back_to_defaults():
    """No kwarg, no IR `(config …)` → defaults applied + stashed
    on kb.config so downstream code can read it without
    threading."""
    kb = _kb("""
    (ontology (relation r T T))
    (facts (r A B :source "(1)"))
    (query :mode solve :goal (r A B))
    """)
    assert kb.config is None
    solve(kb)
    assert kb.config == SolverConfig()
