"""S1.5b.9 — monotonic / tree parity on branching demos.

Parametrised regression test: for every fixture under
``examples/branching/``, run the tree engine and the monotonic
engine and compare the resulting verdicts + goal bindings.

The two engines agree for 10 of the 11 fixtures. One fixture
(``04_two_levels.ein``) is a documented SOLVE-mode divergence:

- Tree depth-first search at the depth cap finds **both**
  satisfying branches (Blue↔H3 and Green↔H3) and packages them
  as an :class:`Ambiguity`.
- Monotonic per Q1.5b.7 / algorithm_layer_n.md §3d.vii terminates
  on the **first** goal-satisfying commitment (lex order picks
  ``(co-located Blue H3)``) and returns :class:`Solution`.

Both behaviours are correct under their respective verdict
contracts — SOLVE mode for monotonic is "give me **a** solution";
the tree's Ambiguity captures the depth-cap halt with multiple
open satisfying leaves. The divergence is recorded in
``parity_baselines.md`` and below.

Items NOT verified here:

- Per-branch refutation order (different shapes between engines).
- Per-firing trace (different firing sequences even when verdicts
  match).

Both engines are run on the same kb-from-parse so configuration
in the puzzle's ``(config …)`` head is honoured identically.
"""
from __future__ import annotations

from pathlib import Path

import pytest

from ein_bot.inference.compile import JoinPlan, compile_pattern
from ein_bot.inference.match import run as match_run
from ein_bot.inference.monotonic import monotonic_solve
from ein_bot.inference.tree.solver import (
    Ambiguity,
    Solution,
)
from ein_bot.inference.tree.solver import (
    solve as tree_solve,
)
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[4]
BRANCHING = REPO / "examples" / "branching"


# Each row: (fixture, max_depth, tree_verdict_type, mono_verdict_type, mono_bindings).
#
# `max_depth` doubles for both engines (tree --max-depth, monotonic
# --max-set-size). `mono_bindings` is the single binding row the
# monotonic verdict's kb yields against the query goal; None means
# "do not check bindings" (e.g. when the test focuses on verdict
# shape rather than content).
FIXTURES: list[tuple[str, int, type, type, dict[str, str] | None]] = [
    ("01_saturate_only.ein",                 1, Solution,  Solution, {"c": "Blue"}),
    ("02_one_dead_one_alive.ein",            5, Solution,  Solution, {"c": "Blue"}),
    ("03_five_hyps_one_alive.ein",           3, Solution,  Solution, {"h": "H5"}),
    # Two-branch ambiguous demo — Q1.5b.7 divergence (see module docstring).
    ("04_two_levels.ein",                    3, Ambiguity, Solution, {"c": "Blue"}),
    ("05_mini_zebra.ein",                    3, Solution,  Solution, {"n": "Bob", "p": "Dog"}),
    ("06_lookahead_on.ein",                  3, Solution,  Solution, {"h": "H5"}),
    ("07_lookahead_off.ein",                 5, Solution,  Solution, {"h": "H5"}),
    ("08_hypothesis_relation_whitelist.ein", 3, Solution,  Solution, {"h": "H3"}),
    ("09_hrule.ein",                         3, Solution,  Solution, {"h": "H3"}),
    ("10_backprop_on.ein",                   5, Solution,  Solution, {"p": "Dave"}),
    ("11_backprop_off.ein",                  5, Solution,  Solution, {"p": "Dave"}),
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
    "fixture,max_depth,tree_verdict,mono_verdict,mono_bindings",
    FIXTURES,
    ids=[f for f, *_ in FIXTURES],
)
def test_monotonic_tree_parity(
    fixture: str,
    max_depth: int,
    tree_verdict: type,
    mono_verdict: type,
    mono_bindings: dict[str, str] | None,
) -> None:
    text = (BRANCHING / fixture).read_text()

    # Tree side
    kb_tree = _kb(text)
    v_tree = tree_solve(kb_tree, max_depth=max_depth)
    assert isinstance(v_tree, tree_verdict), (
        f"{fixture}: tree got {type(v_tree).__name__}, "
        f"expected {tree_verdict.__name__}"
    )

    # Monotonic
    kb_mono = _kb(text)
    v_mono, _stats = monotonic_solve(kb_mono, max_set_size=max_depth)
    assert isinstance(v_mono, mono_verdict), (
        f"{fixture}: monotonic got {type(v_mono).__name__}, "
        f"expected {mono_verdict.__name__}"
    )

    # Goal bindings on monotonic's verdict kb — single row expected
    # in SOLVE mode (per is_solved's `len(matches) == 1`).
    if mono_bindings is not None and isinstance(v_mono, Solution):
        rows = _goal_bindings(v_mono.kb)
        assert rows == [mono_bindings], (
            f"{fixture}: monotonic bindings {rows}, "
            f"expected [{mono_bindings}]"
        )

    # When both engines return Solution, their bindings should
    # match — the engines disagree on multi-solution ambiguity
    # shape (04), not on which single solution to return for a
    # uniquely-solvable demo.
    if (
        isinstance(v_tree, Solution)
        and isinstance(v_mono, Solution)
        and mono_bindings is not None
    ):
        tree_rows = _goal_bindings(v_tree.kb)
        # Tree's verdict.kb may carry the satisfying leaf's full
        # kb; the goal-binding extraction should hit the same
        # single row.
        assert mono_bindings in tree_rows, (
            f"{fixture}: monotonic bound {mono_bindings} but "
            f"tree's leaf kb yields {tree_rows}"
        )
