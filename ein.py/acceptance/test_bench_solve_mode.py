"""Acceptance — `ein solve` on zebra2 (the merged solver command).

`ein solve` drives the sound `solve()` engine (the former `ein search`, merged
into `solve` 2026-06-16). This pins that it solves `zebra2` **correctly** — the
full, right 25/25 model — rather than the removed first-goal-match dead-end
that wrongly committed `(color-loc Green House-4)` and stopped.

Invoked as a subprocess, exactly as a user runs it. Slow (~6 s under PyPy for
the `stop_after=1` fast path; ~25 s for `--exhaustive`), hence in the
acceptance phase, not the unit suite.
"""
from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
SOLVE = [sys.executable, "-m", "ein.cli", "solve"]
ZEBRA2 = REPO / "examples" / "zebra2.ein"


def _run(*args: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [*SOLVE, str(ZEBRA2), *args],
        capture_output=True, text=True, timeout=180,
    )


def test_solve_solves_zebra2_correctly():
    """`ein solve` (default stop_after=1) → the correct 25/25 model. Exercises
    --print-final-state (REASONING residue) and --print-final-hfacts (the
    hypothesis commitments, all layers)."""
    proc = _run("--print-final-state", "--print-final-hfacts", "--stats")
    assert proc.returncode == 0, proc.stderr
    out = proc.stdout

    # The result-driven answer line + the fast-path solution count.
    assert re.search(r"norwegian.*water.*japanese.*zebra", out, re.I | re.S), out[:300]
    assert "solutions (k)    1" in out

    # The CORRECT colour grid — the removed first-goal-match put Green@House-4
    # (and Ivory@House-3, Red@House-5); the sound engine gets Green@House-5.
    for cell in (
        "(color-loc Yellow House-1)",
        "(color-loc Blue House-2)",
        "(color-loc Red House-3)",
        "(color-loc Ivory House-4)",
        "(color-loc Green House-5)",
    ):
        assert cell in out, f"missing/incorrect grid cell: {cell}"

    # The canonical answer cells (derived in the final state).
    assert "(drink-loc Water House-1)" in out
    assert "(pet-loc Zebra House-5)" in out
    assert "(nation-loc Japanese House-5)" in out

    # --print-final-hfacts spans every layer, so it DOES echo the given
    # (nation-loc Norwegian House-1) — condition (10), a FACT-layer fact that
    # the REASONING-only --print-final-state omits. The hfacts header lists
    # exactly the five *-loc hypothesis-target relations.
    assert "(nation-loc Norwegian House-1)" in out
    assert re.search(r":hrules \[.*'nation-loc'.*\]", out), out[-800:]


def test_solve_exhaustive_certifies_unique():
    """`--exhaustive` exhausts the lattice → k=1, exhausted=true (unique)."""
    proc = _run("--exhaustive", "--stats")
    assert proc.returncode == 0, proc.stderr
    out = proc.stdout
    assert "solutions (k)    1" in out
    assert re.search(r"exhausted\s+true", out), out[-500:]
