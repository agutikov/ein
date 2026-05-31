"""Acceptance — the sound `solve` mode wired into `bench_monotonic.py`.

`bench_monotonic.py` (run via `./bench_solve_monotonic_pypy.sh`) drives the
sound `solve()` engine since `monotonic_solve` was removed (P1.7a). This pins
that the bench solves `zebra2` **correctly** — the full, right 25/25 model —
rather than the old `monotonic_solve` first-goal-match dead-end, which wrongly
committed `(color-loc Green House-4)` and stopped.

Invoked as a subprocess, exactly as a user runs it
(`bench_solve_monotonic_pypy.sh … examples/zebra2.ein --print-final-state`).
Slow (~6 s under PyPy: solve's `stop_after=1` fast path), hence in the
acceptance phase, not the unit suite.
"""
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
BENCH = REPO / "ein.py" / "demo" / "bench_monotonic.py"
ZEBRA2 = REPO / "examples" / "zebra2.ein"


def _run_bench(*args: str) -> subprocess.CompletedProcess:
    # sys.executable is whatever interpreter pytest runs under (PyPy via
    # run_tests.sh); ein_bot is importable there (installed in the venv).
    return subprocess.run(
        [sys.executable, str(BENCH), str(ZEBRA2), *args],
        capture_output=True, text=True, timeout=180,
    )


def test_bench_solve_mode_solves_zebra2_correctly():
    """Default bench run (solve, stop_after=1) → the correct 25/25 model."""
    proc = _run_bench("--print-final-state")
    assert proc.returncode == 0, proc.stderr
    out = proc.stdout

    # Sound verdict + the fast-path solution-node count.
    assert re.search(r"verdict\s+Solution", out), out[:500]
    assert "solution_nodes (k) 1" in out

    # The CORRECT colour grid — the removed monotonic_solve put Green@House-4
    # (and Ivory@House-3, Red@House-5); the sound engine gets Green@House-5.
    for cell in (
        "(color-loc Yellow House-1)",
        "(color-loc Blue House-2)",
        "(color-loc Red House-3)",
        "(color-loc Ivory House-4)",
        "(color-loc Green House-5)",
    ):
        assert cell in out, f"missing/incorrect grid cell: {cell}"

    # The canonical answer cells (derived in the final state). Norwegian@H1
    # is the input condition (10), so --print-final-state doesn't echo it;
    # the derived Japanese@H5 + Water@H1 + Zebra@H5 pin the answer.
    assert "(drink-loc Water House-1)" in out
    assert "(pet-loc Zebra House-5)" in out
    assert "(nation-loc Japanese House-5)" in out


def test_bench_solve_mode_exhaustive_certifies_unique():
    """`--exhaustive` exhausts the lattice → k=1, exhausted=true (unique)."""
    proc = _run_bench("--exhaustive")
    assert proc.returncode == 0, proc.stderr
    out = proc.stdout
    assert "solution_nodes (k) 1" in out
    assert re.search(r"exhausted\s+true", out), out[-500:]
