"""S1.7.23 T1.7.23.1 — standing blind-path solve fixture.

`examples/branching/12_typed_blind_solve.ein` is a no-`(hrule …)`,
NON-`T`-typed puzzle that solves via the *blind* hypothesis enumerator
(`hypgen._generate`'s else-branch). It is the regression witness for
retiring the kernel `is-a` type-filter (S1.7.23 T1.7.23.2): the M1 gate
(zebra2) is hrule-driven and never exercises that filter, so without this
fixture the removal would be silent.

These tests pin the INVARIANTS that survive the type-filter's removal —
"solves on the blind path, one model, a genuine House→Color bijection".
"""
from __future__ import annotations

from pathlib import Path

from ein.inference.monotonic import solve
from ein.inference.verdict import Solution
from ein.ir import parse
from ein.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]
FIXTURE = REPO / "examples" / "branching" / "12_typed_blind_solve.ein"


def _kb() -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(FIXTURE.read_text(encoding="utf-8")))


def test_fixture_runs_the_blind_enumerator_not_an_hrule():
    """The fixture declares no `(hrule …)`, so `hypgen` takes the blind
    combinatorial path — the one that holds the `is-a` type-filter."""
    assert not _kb().hrules, "fixture must be blind-path (no hrule)"


def test_blind_path_solves_to_a_single_model():
    """`solve(stop_after=1)` returns a Solution on the blind path.

    This is the standing assertion that retiring the kernel type-filter
    (T1.7.23.2) does not break a blind-path solve — it must stay green
    before and after that removal.
    """
    verdict, stats = solve(_kb(), stop_after=1)
    assert isinstance(verdict, Solution), (
        f"blind-path fixture must solve, got {type(verdict).__name__}"
    )
    assert stats.solution_nodes == 1


def test_model_is_a_house_color_bijection():
    """The recovered model assigns every house a distinct colour — a real
    3x3 bijection, not a partial dead-end (the S1.7.3 soundness shape)."""
    verdict, _ = solve(_kb(), stop_after=1)
    model = verdict.kb
    cells = [
        f.args
        for f in model._facts_by_relation.get("color-of", ())
        if len(f.args) == 2
    ]
    houses = {a for a, _ in cells}
    colours = {b for _, b in cells}
    assert houses == {"H1", "H2", "H3"}, f"every house assigned: {cells}"
    assert colours == {"Red", "Green", "Blue"}, f"every colour used: {cells}"
    assert len(cells) == 3, f"a bijection has exactly 3 positives: {cells}"
    # The given anchor is respected.
    assert ("H1", "Red") in cells
