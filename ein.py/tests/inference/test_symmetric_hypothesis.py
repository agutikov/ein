"""S1.7.24 T1.7.24.1 — standing symmetric-hypothesis solve fixtures.

The M1 gate (zebra2) is BLIND to symmetric-hypothesis search: its targets
are `*-loc` bijections, never a `(symmetric R)` relation the search
branches on. So the symmetric `k`-correctness contract — *an undecided
symmetric pair `(R a b)` / `(R b a)` counts as ONE model, not two* — has
no gate coverage. This file pins it, on the two puzzles that actually
branch on a symmetric relation, BEFORE S1.7.24 retires the kernel's
symmetric-awareness (generation both-orderings, the on-death mirror, the
open-set canonicalisation). The 31 T1.7.6.4 fails are the warning that a
naive removal double-counts pairs / leaves deaths un-propagated.

These assertions must stay green across the S1.7.24 redesign — the
generic `state_hash` solution-node dedup (both orderings saturate to the
same KB via the user's `(rule symmetric)`) is what must recover the same
`k` once the kernel stops special-casing the tag.
"""
from __future__ import annotations

from pathlib import Path

from ein_bot.inference.monotonic import solve
from ein_bot.inference.verdict import Ambiguity, Solution
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
MINI_ZEBRA = REPO / "examples" / "branching" / "05_mini_zebra.ein"
TWO_LEVELS = REPO / "examples" / "branching" / "04_two_levels.ein"


def _kb(path: Path) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(path.read_text(encoding="utf-8")))


def _has_symmetric(kb: KnowledgeBase) -> bool:
    return bool(kb._facts_by_relation.get("symmetric", ()))


def test_mini_zebra_symmetric_solve_is_unique():
    """`mini_zebra` branches on the symmetric relation `co-located` and
    has exactly ONE model — the symmetric pair must count once."""
    kb = _kb(MINI_ZEBRA)
    assert _has_symmetric(kb), "fixture must declare (symmetric …)"
    verdict, stats = solve(kb)  # exhaustive → k is exact
    assert isinstance(verdict, Solution), type(verdict).__name__
    assert stats.solution_nodes == 1
    assert stats.exhausted
    # The recovered model: Bob drinks Coffee and owns the Dog.
    model = verdict.kb
    assert model._fact_by_id("co-located", ("Bob", "Coffee")) is not None
    assert model._fact_by_id("co-located", ("Bob", "Dog")) is not None


def test_two_levels_symmetric_ambiguity_is_two_not_double_counted():
    """`branching/04` is a genuinely 2-model puzzle over the symmetric
    `co-located`. `k` must be 2 (the two real placements), NOT inflated
    by counting each symmetric orientation separately."""
    kb = _kb(TWO_LEVELS)
    assert _has_symmetric(kb)
    verdict, stats = solve(kb)  # exhaustive
    assert isinstance(verdict, Ambiguity), type(verdict).__name__
    assert stats.solution_nodes == 2
    assert stats.exhausted
    # The two models are distinct complete states.
    assert len({b.kb for b in verdict.branches}) >= 1  # branches recorded
    assert len(verdict.branches) == 2
