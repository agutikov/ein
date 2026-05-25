"""T1.5a.2a.2 — branch-order shuffle invariance.

`solve(kb, max_depth=D)` is supposed to produce content-identified
output (overall verdict, root-bubbled `(not h)` set, solution KB
states, union of unsat-cores) that's a *function of (kb, rules, D)
alone* — independent of the order ``_candidates_for`` returned the
alive set in. S1.5a.1a established a deterministic default order
(content-sort); this stage's `SolverConfig.candidate_order_seed`
applies a content-mixed seeded permutation AFTER the sort so we
can probe whether the depth-D output stays invariant.

Counter-examples would surface caching / back-prop / NAF-timing
leaks (see S1.5a.16's "Why this is suspected" section); a passing
test is positive evidence that the four-set claim holds on the
puzzles + depths exercised here.

Zebra2 is *not* exercised here — even at ``max_depth=1`` it costs
~30 s per solve on CPython, which would balloon the test runtime
past the project's CI budget. Run ``./bench_solve_pypy.sh
examples/zebra2.ein --max-depth 1`` with different
``candidate-order-seed`` configs for the manual cross-check; if
shuffle invariance regresses on zebra2 specifically, promote a
dedicated slow-marked test.
"""
from __future__ import annotations

from dataclasses import replace
from pathlib import Path

import pytest

from ein_bot.inference.canon import state_hash
from ein_bot.inference.config import SolverConfig
from ein_bot.inference.solver import Ambiguity, Contradiction, Solution, solve
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase

REPO = Path(__file__).resolve().parents[3]


# ── Helpers ───────────────────────────────────────────────────────


def _run(puzzle_path: Path, max_depth: int, seed: int):
    """Solve once. ``seed < 0`` means the default content-sort order
    (S1.5a.1a); ``seed >= 0`` shuffles the sorted candidate list with
    a content-mixed seeded RNG (S1.5a.16 / T1.5a.2a.2). Returns
    ``(verdict, kb)`` so the caller can project both invariants.
    """
    text = puzzle_path.read_text(encoding="utf-8")
    kb = KnowledgeBase.from_ir(parse(text))
    base = kb.config or SolverConfig()
    cfg = replace(base, candidate_order_seed=seed)
    verdict = solve(kb, max_depth=max_depth, config=cfg)
    return verdict, kb


def _invariants(verdict, root_kb):
    """Content-identified projections that should be shuffle-invariant.

    - **verdict_type** — ``"Solution" | "Ambiguity" | "Contradiction"``.
    - **bubbled** — ``root_kb._negated_facts`` (the ``(not h)`` set
      back-propped to root by the consume loop). Independent of which
      sibling died first.
    - **solution_states** — ``state_hash`` of every solution-leaf's
      ``kb_snapshot``. State hashes are content-derived (S1.5.3 dedup
      already relies on this), so equivalent KB states hash identically
      across orderings.
    - **leaves_by_verdict** — Counter of leaf verdicts; structural
      sanity (same number of dead / open / solution leaves).
    - **unsat_union** — every ``(rel, args)`` mentioned in any node's
      ``unsat_core``. The *witness fact set* is order-independent even
      if the per-branch core attribution shuffles.
    """
    from collections import Counter

    tree = verdict.tree
    leaves = [n for n in tree.nodes.values() if not n.children]
    return {
        "verdict_type": type(verdict).__name__,
        "bubbled": frozenset(root_kb._negated_facts),
        "solution_states": frozenset(
            state_hash(n.kb_snapshot)
            for n in leaves
            if n.verdict == "solution" and n.kb_snapshot is not None
        ),
        "leaves_by_verdict": Counter(n.verdict for n in leaves),
        "unsat_union": frozenset(
            (f.relation_name, f.args)
            for n in tree.nodes.values()
            for f in n.unsat_core
        ),
    }


def _diff(a: dict, b: dict) -> str:
    """Render a per-key diff for assertion failure messages."""
    lines = []
    for k in a:
        if a[k] != b[k]:
            lines.append(f"  {k}:")
            if isinstance(a[k], (frozenset, set)):
                only_a = a[k] - b[k]
                only_b = b[k] - a[k]
                if only_a:
                    lines.append(f"    only in default: {sorted(only_a)[:5]}{'…' if len(only_a) > 5 else ''}")
                if only_b:
                    lines.append(f"    only in shuffle: {sorted(only_b)[:5]}{'…' if len(only_b) > 5 else ''}")
            else:
                lines.append(f"    default: {a[k]!r}")
                lines.append(f"    shuffle: {b[k]!r}")
    return "\n".join(lines) if lines else "(no diff but != fired?)"


# ── Test matrix ────────────────────────────────────────────────────


# Branching demos with at least one fork at d <= 2. Demos 02/03 only
# fork once (single hypothesis surviving root saturation) — no
# permutation possible, no signal. 04 forks 22 nodes (genuinely
# ambiguous); 05 forks 4 (small cross-attribute join).
DEMO_PUZZLES = [
    "examples/branching/04_two_levels.ein",
    "examples/branching/05_mini_zebra.ein",
]

SHUFFLE_SEEDS = (0, 1)


@pytest.mark.parametrize("puzzle", DEMO_PUZZLES)
@pytest.mark.parametrize("max_depth", (0, 1, 2))
@pytest.mark.parametrize("seed", SHUFFLE_SEEDS)
def test_demo_shuffle_invariance(puzzle, max_depth, seed):
    """Each demo at each depth: shuffled order must reproduce
    default-order invariants."""
    path = REPO / puzzle
    v_default, kb_default = _run(path, max_depth, -1)
    v_shuffled, kb_shuffled = _run(path, max_depth, seed)
    inv_a = _invariants(v_default, kb_default)
    inv_b = _invariants(v_shuffled, kb_shuffled)
    assert inv_a == inv_b, (
        f"shuffle invariance violated:\n"
        f"  puzzle: {puzzle}\n"
        f"  max_depth: {max_depth}\n"
        f"  seed: {seed}\n"
        f"{_diff(inv_a, inv_b)}"
    )


# ── Sanity test for the shuffle knob itself ────────────────────────


def test_shuffle_actually_permutes_at_root():
    """The shuffle isn't a no-op: with a non-zero seed the root's
    candidate list visits in a different first-branch order than the
    default sort (on a puzzle with enough root candidates that any
    permutation is observable)."""
    path = REPO / "examples" / "branching" / "04_two_levels.ein"
    v_default, _ = _run(path, max_depth=1, seed=-1)
    v_shuf,    _ = _run(path, max_depth=1, seed=1)

    def first_hypothesis(verdict):
        for nid in sorted(verdict.tree.nodes):
            n = verdict.tree.nodes[nid]
            if n.hypothesis is not None:
                return (n.hypothesis.relation_name, n.hypothesis.args)
        return None

    # Not strictly required by invariance (some shuffles may
    # coincide with the sort), but seed=1 on demo 04 is known to
    # land a different first branch — keeps the negative test honest.
    assert first_hypothesis(v_default) != first_hypothesis(v_shuf), (
        "shuffle produced the same first branch as the default sort "
        "— knob may not be wired"
    )
