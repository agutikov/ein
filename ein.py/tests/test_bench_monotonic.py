"""S1.5b.4 skeleton — bench script smoke.

The script's arg-parsing + import surface should work even
though :func:`monotonic_solve` raises ``NotImplementedError``.
Once S1.5b.5 lands, this can run a real puzzle.
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
SCRIPT = REPO / "demo" / "bench_monotonic.py"


def test_help_smoke():
    """``--help`` works → arg-parsing is wired."""
    proc = subprocess.run(
        [sys.executable, str(SCRIPT), "--help"],
        capture_output=True, text=True, check=True,
    )
    assert "monotonic" in proc.stdout.lower()
    assert "--max-set-size" in proc.stdout
    assert "--dump-states" in proc.stdout
