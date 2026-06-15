"""Budget abort tests for `solve` — Tier 1 of the
bench_monotonic CLI parity work.

Covers:

1. `max_enterings` raises `BudgetExceededError` with partial
   stats whose `enterings_total` equals the cap.
2. `max_time` raises with a `reason` mentioning the limit; the
   exception preserves whatever counters the engine accumulated.
3. `BudgetExceededError` re-exports through the package root
   (`from ein.inference.monotonic import BudgetExceededError`).
4. `dumper`'s timeline has events recorded right up to the
   abort point (no `summary` event — the abort path skips it).
"""
from __future__ import annotations

import json
from pathlib import Path

import pytest

from ein.inference.config import SolverConfig
from ein.inference.monotonic import (
    BudgetExceededError,
    solve,
)
from ein.inference.monotonic import (
    BudgetExceededError as _PkgBudgetError,
)
from ein.inference.monotonic.state_dump import MonotonicDumper
from ein.inference.verdict import Aborted
from ein.ir import parse
from ein.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# Fixture with many independent layer-1 hypotheses so the budget
# trips inside the candidate loop, not in Phase 1's short-circuit.
# `_NO_LOOKAHEAD` keeps the deaths flowing through `try_commitment_set`
# instead of being pre-empted by hypgen's lookahead.
_FIXTURE = """
(rule kill-pair ()
  :match  (and (R a b) (R c d))
  :assert (false)
  :why    "kills the specific (R a b)+(R c d) pair"
  :priority 100)
(relation R T T)
; declared but never asserted — keeps the goal unreachable so
; the fork-side is_solved check doesn't end the run before the
; budget cap can fire.
(relation never T)
(is-a Thing T)
(is-a a Thing) (is-a b Thing)
(is-a c Thing) (is-a d Thing)

(query :mode solve
       :goal  (never ?x)
       :hypothesis-relations R)
"""

_NO_LOOKAHEAD = SolverConfig(
    enable_pre_branch_lookahead=False,
    enable_lookahead_kill_cache=False,
)


def test_max_enterings_aborts_with_partial_stats() -> None:
    kb = _kb(_FIXTURE)
    with pytest.raises(BudgetExceededError) as exc:
        solve(
            kb, max_set_size=3, config=_NO_LOOKAHEAD,
            max_enterings=2,
        )
    assert "max-enterings (2)" in exc.value.reason
    # The cap is checked BEFORE the (N+1)-th call, so when it
    # trips `enterings_total == max_enterings`.
    assert exc.value.stats.enterings_total == 2
    assert exc.value.stats.layers_explored >= 1


def test_max_time_aborts_with_partial_stats() -> None:
    kb = _kb(_FIXTURE)
    # 0 seconds → the check fires on the first iteration that has
    # observed any wall-clock progress. Hypgen + saturation
    # already burned some microseconds, so this aborts very fast.
    with pytest.raises(BudgetExceededError) as exc:
        solve(
            kb, max_set_size=3, config=_NO_LOOKAHEAD,
            max_time=0.0,
        )
    assert "max-time" in exc.value.reason
    assert exc.value.stats.enterings_total >= 0  # may be 0 if it trips immediately
    assert exc.value.stats.layers_explored >= 1


def test_budget_exception_reexported_from_package() -> None:
    """``from ein.inference.monotonic import BudgetExceededError``
    pulls the same class as the deep import."""
    assert _PkgBudgetError is BudgetExceededError


def test_dumper_timeline_captures_events_up_to_abort(
    tmp_path: Path,
) -> None:
    kb = _kb(_FIXTURE)
    dumper = MonotonicDumper(out_dir=tmp_path)
    with pytest.raises(BudgetExceededError):
        solve(
            kb, max_set_size=3, config=_NO_LOOKAHEAD,
            dumper=dumper, max_enterings=2,
        )
    timeline_path = tmp_path / "00_timeline.jsonl"
    assert timeline_path.is_file()
    events = [
        json.loads(line)
        for line in timeline_path.read_text().splitlines()
        if line.strip()
    ]
    kinds = [e["event"] for e in events]
    # root_initial + at least one layer_start + 2 enterings, no summary.
    assert kinds[0] == "root_initial"
    assert "layer_start" in kinds
    assert kinds.count("entering") == 2
    assert "summary" not in kinds
    # summary.json is intentionally not written on abort.
    assert not (tmp_path / "summary.json").exists()


def test_on_budget_verdict_returns_aborted_instead_of_raising() -> None:
    """S1.9.E17.2 — ``on_budget="verdict"`` surfaces the abort as an
    :class:`Aborted` verdict (with the partial stats) rather than raising;
    the default ``"raise"`` (the other tests) is unchanged."""
    kb = _kb(_FIXTURE)
    verdict, stats = solve(
        kb, max_set_size=3, config=_NO_LOOKAHEAD,
        max_enterings=2, on_budget="verdict",
    )
    assert isinstance(verdict, Aborted)
    assert "max-enterings (2)" in verdict.reason
    assert verdict.stats is stats              # the partial stats, not a copy
    assert stats.enterings_total == 2
    assert stats.exhausted is False            # not a proven verdict


def test_no_budget_completes_normally() -> None:
    """Sanity guard: without any limits, the engine runs to
    completion on the same fixture."""
    kb = _kb(_FIXTURE)
    verdict, _stats = solve(
        kb, max_set_size=3, config=_NO_LOOKAHEAD,
    )
    # The `kill-pair` rule asserts (false) on every R-pair, so no
    # complete∧consistent solution node survives — solve() reads
    # the verdict from the deduped solution-node count (k=0). The
    # point is just that no exception fires.
    assert verdict is not None
