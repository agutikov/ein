"""S1.5b.9 — monotonic regression on branching demos.

Parametrised test: for every fixture under
``examples/branching/``, run :func:`monotonic_solve` and pin the
expected verdict + goal bindings.

History: this file used to compare monotonic vs tree's
``solve()`` end-to-end (the "parity" angle). With the tree
solver removed (post-S1.5b.30 task), the parity comparison is
gone; the file now reads as the **monotonic-side regression
net** pinning the expected outcome per fixture directly.

Documented divergence: ``04_two_levels.ein`` produces
:class:`Solution` under monotonic (Q1.5b.7 — SOLVE-mode
"give me **a** solution"; lex order picks
``(co-located Blue H3)``). The two-branch enumeration of this
fixture's solutions lives in
``test_gaps_backbone.test_gaps_solve_branching_04_returns_two_branches``.

Items NOT verified here:

- Per-branch refutation order (engine-internal).
- Per-firing trace (engine-internal).
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.inference.compile import JoinPlan, compile_pattern
from ein_bot.inference.match import run as match_run
from ein_bot.inference.monotonic import monotonic_solve
from ein_bot.inference.verdict import Solution
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


# Each row: (fixture, max_set_size, expected_verdict_type,
# expected_bindings). expected_bindings is the single binding
# row the monotonic verdict's kb yields against the query goal;
# None means "do not check bindings" (e.g. when the test focuses
# on verdict shape rather than content).
FIXTURES: list[tuple[str, int, type, dict[str, str] | None]] = [
    ("01_saturate_only.ein",                 1, Solution, {"c": "Blue"}),
    ("02_one_dead_one_alive.ein",            5, Solution, {"c": "Blue"}),
    ("03_five_hyps_one_alive.ein",           3, Solution, {"h": "H5"}),
    ("04_two_levels.ein",                    3, Solution, {"c": "Blue"}),
    ("05_mini_zebra.ein",                    3, Solution, {"n": "Bob", "p": "Dog"}),
    ("06_lookahead_on.ein",                  3, Solution, {"h": "H5"}),
    ("07_lookahead_off.ein",                 5, Solution, {"h": "H5"}),
    ("08_hypothesis_relation_whitelist.ein", 3, Solution, {"h": "H3"}),
    ("09_hrule.ein",                         3, Solution, {"h": "H3"}),
    ("10_backprop_on.ein",                   5, Solution, {"p": "Dave"}),
    ("11_backprop_off.ein",                  5, Solution, {"p": "Dave"}),
]


def _kb(text: str) -> KnowledgeBase:
    return KnowledgeBase.from_ir(parse(text))


def _goal_bindings(kb: KnowledgeBase) -> list[dict[str, str]]:
    """Run the kb's query :goal pattern against the kb. Returns
    the list of binding dicts (one per matching row)."""
    if kb is None or kb.query is None:
        return []
    goal = next(
        (kp.value for kp in kb.query.kw_pairs
         if kp.key.name == "goal"),
        None,
    )
    if goal is None:
        return []
    plan = JoinPlan(
        rule_name="<query>",
        activator_args=(),
        bindings_seed={},
        steps=tuple(compile_pattern(goal, {})),
        assert_template=None,
        why="",
    )
    return [dict(b) for b, _premises in match_run(plan, kb)]


@pytest.mark.parametrize(
    "fixture,max_set_size,expected_verdict,expected_bindings",
    FIXTURES,
    ids=[f for f, *_ in FIXTURES],
)
def test_monotonic_branching_fixture(
    fixture: str,
    max_set_size: int,
    expected_verdict: type,
    expected_bindings: dict[str, str] | None,
) -> None:
    """monotonic_solve on each branching fixture lands the
    expected verdict shape + bindings (SOLVE mode: exactly
    one binding row)."""
    text = (BRANCHING / fixture).read_text()
    kb = _kb(text)
    verdict, _stats = monotonic_solve(kb, max_set_size=max_set_size)
    assert isinstance(verdict, expected_verdict), (
        f"{fixture}: monotonic got {type(verdict).__name__}, "
        f"expected {expected_verdict.__name__}"
    )
    if expected_bindings is not None and isinstance(verdict, Solution):
        rows = _goal_bindings(verdict.kb)
        assert rows == [expected_bindings], (
            f"{fixture}: monotonic bindings {rows}, "
            f"expected [{expected_bindings}]"
        )
