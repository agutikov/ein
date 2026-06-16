"""`ein solve` CLI smoke — the one merged solver command.

`ein solve` replaced the former `search` (sound solve) and `lattice` (gaps /
contradictions) engine-runner subcommands (2026-06-16). One command, one sound
engine: the verdict is read from the result, the stop policy is single (default)
/ `--solutions N` / `--exhaustive`, and the output is the answer (solution[s] or
unsat core), with the markdown trace going to a file via `--trace`.
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
CMD = [sys.executable, "-m", "ein.cli", "solve"]
EXAMPLES = REPO.parent / "examples"
FIXTURE = EXAMPLES / "branching" / "04_two_levels.ein"


def _run(*args: str, check: bool = True) -> subprocess.CompletedProcess:
    return subprocess.run([*CMD, *args], capture_output=True, text=True, check=check)


def test_help_lists_stop_policy_and_drops_modes():
    out = _run("--help").stdout
    # stop policy + the result-driven surface
    for flag in ("--solutions", "--exhaustive", "--trace", "--stats",
                 "--print-final-state"):
        assert flag in out, f"missing flag in help: {flag}"
    # the unsound mode/gaps/contradictions selectors are gone
    for gone in ("--mode", "--gaps", "--contradictions"):
        assert gone not in out, f"removed flag still in help: {gone}"


def test_default_single_solution():
    """Default stop policy stops at the first solution → exit 0, an answer
    line, and k=1 in --stats."""
    proc = _run(str(FIXTURE), "--max-set-size", "2", "--stats")
    assert proc.returncode == 0, proc.stderr
    assert "solutions (k)    1" in proc.stdout
    assert "exhausted        false" in proc.stdout


def test_exhaustive_certifies():
    """--exhaustive runs the lattice to the end; this fixture has two distinct
    models, so the sound verdict is Ambiguity (k=2, exhausted=true)."""
    proc = _run(str(FIXTURE), "--max-set-size", "3", "--exhaustive", "--stats")
    assert proc.returncode == 0, proc.stderr
    assert "solutions (k)    2" in proc.stdout
    assert "exhausted        true" in proc.stdout
    assert "ambiguous" in proc.stdout.lower()


def test_solutions_n_stop_policy():
    """--solutions N stops after N distinct solutions (here 2 → Ambiguity)."""
    proc = _run(str(FIXTURE), "--max-set-size", "3", "--solutions", "2")
    assert proc.returncode == 0, proc.stderr
    assert "ambiguous" in proc.stdout.lower()


def test_removed_mode_flags_error():
    for gone in ("--mode=solve", "--gaps", "--contradictions"):
        proc = _run(str(FIXTURE), gone, check=False)
        assert proc.returncode != 0, f"{gone} should be rejected"
        assert "unrecognized arguments" in proc.stderr


def test_trace_goes_to_a_file(tmp_path):
    """--trace writes the markdown derivation trace to a file (never stdout);
    stdout keeps the one-line answer."""
    out_md = tmp_path / "trace.md"
    proc = _run(str(FIXTURE), "--max-set-size", "2", "--trace", str(out_md))
    assert proc.returncode == 0, proc.stderr
    assert out_md.exists() and out_md.stat().st_size > 0
    # the markdown is in the file, not on stdout
    assert "```" not in proc.stdout
    assert "# " in out_md.read_text()


def test_shuffle_is_verdict_invariant():
    """--shuffle reorders the within-layer traversal but not the verdict
    (S1.5b.31): two seeds, exhaustive, agree on the answer + k; the seed is
    echoed to stderr."""
    for seed in ("3", "11"):
        proc = _run(str(FIXTURE), "--max-set-size", "3", "--exhaustive",
                    "--shuffle", "--seed", seed, "--stats")
        assert proc.returncode == 0, proc.stderr
        assert f"shuffle seed: {seed}" in proc.stderr
        assert "ambiguous" in proc.stdout.lower()
        assert "solutions (k)    2" in proc.stdout


def test_verbose_streams_progress_to_stderr():
    """--verbose streams per-layer / per-entering progress to stderr while the
    answer stays on stdout."""
    proc = _run(str(FIXTURE), "--max-set-size", "2", "--verbose",
                "--progress-every", "1")
    assert proc.returncode == 0, proc.stderr
    assert "layer 1" in proc.stderr
    assert proc.stdout.strip()                 # answer on stdout


def test_timing_prints_phase_table():
    """--timing prints a per-phase wall-clock table covering every step."""
    proc = _run(str(FIXTURE), "--max-set-size", "2", "--timing")
    assert proc.returncode == 0, proc.stderr
    out = proc.stdout
    assert "timing (ms)" in out
    for phase in ("parse", "kb load", "compile", "root saturation",
                  "hypothesis search", "per hypothesis", "solve", "end-to-end"):
        assert phase in out, f"missing timing phase: {phase}"


def test_short_keys_parse_like_long():
    """Every option has a short key; -m/-e/-s behave like the long forms."""
    proc = _run(str(FIXTURE), "-m", "3", "-e", "-s")
    assert proc.returncode == 0, proc.stderr
    assert "ambiguous" in proc.stdout.lower()
    assert "solutions (k)    2" in proc.stdout
