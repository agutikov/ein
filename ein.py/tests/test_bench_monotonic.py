"""bench_monotonic CLI smoke + sound-solve run.

The bench now drives :func:`ein_bot.inference.monotonic.solve`
(the sound entry — verdict read off the deduped solution-node
count ``k``, not a first-goal-match). The legacy
``monotonic_solve raises NotImplementedError`` skeleton is gone,
so on top of the arg-parsing smoke we run a real puzzle and
assert the sound verdict + stats shape.
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
SCRIPT = REPO / "demo" / "bench_monotonic.py"
# examples/ lives at the project root (one level above ein.py/).
EXAMPLES = REPO.parent / "examples"
BRANCHING_FIXTURE = EXAMPLES / "branching" / "04_two_levels.ein"


def test_help_smoke():
    """``--help`` works → arg-parsing is wired."""
    proc = subprocess.run(
        [sys.executable, str(SCRIPT), "--help"],
        capture_output=True, text=True, check=True,
    )
    assert "monotonic" in proc.stdout.lower()
    assert "--max-set-size" in proc.stdout
    assert "--dump-states" in proc.stdout
    # solve()'s orthogonal stop policy is exposed as --exhaustive.
    assert "--exhaustive" in proc.stdout
    # The removed unsound --mode override must be gone.
    assert "--mode" not in proc.stdout


def test_solve_run_fast_path():
    """Default (stop_after=1) stops at the first complete∧consistent
    node — a sound ``Solution`` here, with ``k=1`` and
    ``exhausted=false`` (the fast path proves *a* model, not
    uniqueness)."""
    proc = subprocess.run(
        [
            sys.executable, str(SCRIPT), str(BRANCHING_FIXTURE),
            "--max-set-size", "2",
        ],
        capture_output=True, text=True, check=True,
    )
    out = proc.stdout
    assert "verdict           Solution" in out
    assert "solution_nodes (k) 1" in out
    assert "exhausted          false" in out
    # Goal projection still prints over the model.
    assert "goal bindings (from query :goal):" in out
    assert "c=Green" in out


def test_solve_run_exhaustive_is_ambiguity():
    """``--exhaustive`` runs the lattice to the end; this fixture has
    two distinct solution nodes, so the sound verdict is
    ``Ambiguity`` (k=2, exhausted=true) — what first-goal-match
    masked.

    S1.7.24 — uses ``--max-set-size 3`` (was 2): branching/04's
    ``co-located`` is symmetric, and with the kernel no longer
    canonicalising symmetric pairs both orientations enter the search,
    so the lattice needs depth 3 (not 2) to fully exhaust. ``k`` is
    still 2 (distinct model STATES collapse via state_hash dedup);
    only the depth needed to *certify* exhaustion grew."""
    proc = subprocess.run(
        [
            sys.executable, str(SCRIPT), str(BRANCHING_FIXTURE),
            "--max-set-size", "3", "--exhaustive",
        ],
        capture_output=True, text=True, check=True,
    )
    out = proc.stdout
    assert "verdict           Ambiguity" in out
    assert "solution_nodes (k) 2" in out
    assert "exhausted          true" in out
