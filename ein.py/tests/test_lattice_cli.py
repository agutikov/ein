"""`ein lattice` CLI smoke — the --shuffle + --verbose flags.

`ein lattice` is the promoted `bench_lattice` command (P1.11 S1.11.3). This
pins the two follow-up options added afterwards: `--shuffle` (per-layer
hypothesis-order randomisation, which must stay verdict-invariant) and
`--verbose` / `--progress-every` (per-layer streaming via ProgressDumper,
parity with `ein search --verbose`).
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
CMD = [sys.executable, "-m", "ein.cli", "lattice"]
# examples/ lives at the project root (one level above ein.py/).
FIXTURE = REPO.parent / "examples" / "branching" / "04_two_levels.ein"


def _run(*args: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [*CMD, *args], capture_output=True, text=True, check=True,
    )


def test_help_lists_the_new_flags():
    out = subprocess.run(
        [*CMD, "--help"], capture_output=True, text=True, check=True,
    ).stdout
    for flag in ("--shuffle", "--seed", "--verbose", "--progress-every"):
        assert flag in out, f"missing flag in help: {flag}"


def test_verbose_streams_progress_to_stderr():
    """--verbose drives ProgressDumper, which streams per-layer + summary
    lines to stderr while the verdict block still lands on stdout."""
    proc = _run(
        "--gaps", str(FIXTURE), "--max-set-size", "2",
        "--verbose", "--progress-every", "1",
    )
    assert "root saturated" in proc.stderr
    assert "layer 1" in proc.stderr
    assert "verdict" in proc.stdout


def test_shuffle_is_verdict_invariant():
    """Two different --seed values must reach the same verdict (the search
    order changes; the answer does not — S1.5b.31 shuffle invariance)."""
    verdicts = []
    for seed in ("1", "7"):
        proc = _run(
            "--gaps", str(FIXTURE), "--max-set-size", "3",
            "--shuffle", "--seed", seed,
        )
        line = next(
            ln for ln in proc.stdout.splitlines() if ln.startswith("verdict")
        )
        verdicts.append(line.split()[1])
    assert verdicts[0] == verdicts[1], verdicts


def test_print_final_positive_dumps_each_gaps_branch():
    """--print-final-positive emits one final-state block per gaps solution
    branch — this fixture has two distinct models."""
    proc = _run(
        "--gaps", str(FIXTURE), "--max-set-size", "2",
        "--print-final-positive",
    )
    out = proc.stdout
    assert "── branch 1/2 ──" in out
    assert "── branch 2/2 ──" in out
    assert out.count("final-state facts") == 2


def test_print_final_contradictions_dumps_unsat_core():
    """--contradictions has no model, so --print-final-* dumps the unsat-core
    facts instead."""
    fixture = REPO.parent / "examples" / "lattice" / "02_genuine_3set_death.ein"
    proc = _run(
        "--contradictions", str(fixture), "--max-set-size", "3",
        "--print-final-state",
    )
    assert "unsat-core facts" in proc.stdout
