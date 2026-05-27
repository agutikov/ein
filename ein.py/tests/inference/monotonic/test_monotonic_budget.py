"""Budget abort tests for `monotonic_solve` — Tier 1 of the
bench_monotonic CLI parity work.

Covers:

1. `max_enterings` raises `BudgetExceededError` with partial
   stats whose `enterings_total` equals the cap.
2. `max_time` raises with a `reason` mentioning the limit; the
   exception preserves whatever counters the engine accumulated.
3. `BudgetExceededError` re-exports through the package root
   (`from ein_bot.inference.monotonic import BudgetExceededError`).
4. `dumper`'s timeline has events recorded right up to the
   abort point (no `summary` event — the abort path skips it).
"""
from __future__ import annotations

import json
from pathlib import Path

import pytest

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.monotonic import (
    BudgetExceededError as _PkgBudgetError,
)
from ein_bot.inference.monotonic.solver import (
    BudgetExceededError,
    monotonic_solve,
)
from ein_bot.inference.monotonic.state_dump import MonotonicDumper
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


# Fixture with many independent layer-1 hypotheses so the budget
# trips inside the candidate loop, not in Phase 1's short-circuit.
# `_NO_LOOKAHEAD` keeps the deaths flowing through `try_commitment_set`
# instead of being pre-empted by hypgen's lookahead.
_FIXTURE = """
(rules
  (rule kill-pair ()
    :match  (and (R a b) (R c d))
    :assert (false)
    :why    "kills the specific (R a b)+(R c d) pair"
    :priority 100))
(ontology
  (relation R T T)
  (is-a Thing T)
  (is-a a Thing) (is-a b Thing)
  (is-a c Thing) (is-a d Thing))
(facts)
(query :mode solve
       :goal  (R ?x ?y)
       :hypothesis-relations R)
"""

_NO_LOOKAHEAD = SolverConfig(
    enable_pre_branch_lookahead=False,
    enable_back_prop_unconditional=False,
)


def test_max_enterings_aborts_with_partial_stats() -> None:
    kb = _kb(_FIXTURE)
    with pytest.raises(BudgetExceededError) as exc:
        monotonic_solve(
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
        monotonic_solve(
            kb, max_set_size=3, config=_NO_LOOKAHEAD,
            max_time=0.0,
        )
    assert "max-time" in exc.value.reason
    assert exc.value.stats.enterings_total >= 0  # may be 0 if it trips immediately
    assert exc.value.stats.layers_explored >= 1


def test_budget_exception_reexported_from_package() -> None:
    """``from ein_bot.inference.monotonic import BudgetExceededError``
    pulls the same class as the deep import."""
    assert _PkgBudgetError is BudgetExceededError


def test_dumper_timeline_captures_events_up_to_abort(
    tmp_path: Path,
) -> None:
    kb = _kb(_FIXTURE)
    dumper = MonotonicDumper(out_dir=tmp_path)
    with pytest.raises(BudgetExceededError):
        monotonic_solve(
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


def test_no_budget_completes_normally() -> None:
    """Sanity guard: without any limits, the engine runs to
    completion on the same fixture."""
    kb = _kb(_FIXTURE)
    verdict, _stats = monotonic_solve(
        kb, max_set_size=3, config=_NO_LOOKAHEAD,
    )
    # The puzzle's goal `(R ?x ?y)` isn't satisfied by any merged
    # root fact (hypotheses don't merge as facts), so verdict is
    # Ambiguity. The point is just that no exception fires.
    assert verdict is not None
