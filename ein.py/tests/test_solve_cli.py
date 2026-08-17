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
    stdout keeps the solve table."""
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


# ── --json-summary (M1a S1a.0.1 T1a.0.1.4) ─────────────────────────
#
# The structured T0/T1 surface the conformance harness diffs. What matters
# here is not the numbers (the engine's own tests own those) but the four
# properties the harness relies on: the file is written, it is additive, it
# is order-free, and it is reproducible.


def _summary(tmp_path, *args: str, check: bool = True) -> dict:
    import json
    out = tmp_path / "summary.json"
    proc = _run(*args, "--json-summary", str(out), check=check)
    assert out.is_file(), proc.stderr
    return json.loads(out.read_text(encoding="utf-8"))


def test_json_summary_carries_verdict_stats_root_and_config(tmp_path):
    """One object per run: what was proved, every engine counter, the
    root-saturation shape, and the resolved config."""
    d = _summary(tmp_path, str(FIXTURE), "-m", "3", "-e")
    assert d["schema"] == "ein-summary/1"
    assert d["verdict"]["type"] == "Ambiguity"
    assert d["verdict"]["k"] == 2 and d["verdict"]["exhausted"] is True
    assert len(d["verdict"]["solutions"]) == 2
    # T1's counter set: MonotonicStats, plus the three blocks that live
    # outside it (root saturator NAF, hypgen filters, compiled plan count).
    for counter in ("enterings_total", "enterings_alive", "enterings_dead_pre",
                    "enterings_dead_post", "layers_explored", "saturate_count",
                    "nogoods_emitted", "nogoods_subsumed", "facts_merged",
                    "forced_positives"):
        assert counter in d["stats"], counter
    assert d["root"]["saturator"]["naf_dropped"] == 0   # structural since S1.21.8
    assert set(d["root"]["hypgen"]) == {"raw", "emitted", "filtered",
                                        "pre_candidate"}
    assert d["root"]["plans"] > 0
    assert d["config"]["lattice-order"] == "lex"


def test_json_summary_is_additive(tmp_path):
    """Writing it changes nothing else — same stdout, stderr and exit code,
    so one invocation can serve every parity tier at once."""
    # `-p` for a large stdout; not `-s`, whose `wall` line is wall-clock and
    # so differs run-to-run for reasons that have nothing to do with the flag.
    with_ = _run(str(FIXTURE), "-m", "2", "-p",
                 "--json-summary", str(tmp_path / "s.json"))
    without = _run(str(FIXTURE), "-m", "2", "-p")
    assert (with_.stdout, with_.stderr, with_.returncode) == \
           (without.stdout, without.stderr, without.returncode)


def test_json_summary_is_order_free_and_reproducible(tmp_path):
    """Every set-shaped observable is sorted, so two runs are byte-identical
    and a diff reports semantics rather than iteration order."""
    a = tmp_path / "a.json"
    b = tmp_path / "b.json"
    _run(str(FIXTURE), "-m", "3", "-e", "--json-summary", str(a))
    _run(str(FIXTURE), "-m", "3", "-e", "--json-summary", str(b))
    assert a.read_bytes() == b.read_bytes()
    import json
    d = json.loads(a.read_text(encoding="utf-8"))
    facts = d["verdict"]["solutions"][0]["facts"]
    assert facts == sorted(facts)
    by_rel = list(d["root"]["facts_by_relation"])
    assert by_rel == sorted(by_rel)


def test_json_summary_on_contradiction_carries_the_core(tmp_path):
    """A k=0 verdict has no model to report, so the core takes its place."""
    d = _summary(tmp_path, str(EXAMPLES / "ein-bugs" / "zebra2-bad.ein"),
                 "-E", "20")
    assert d["verdict"]["type"] == "Contradiction"
    assert d["verdict"]["solutions"] == []
    assert d["verdict"]["unsat_core"]


def test_json_summary_on_abort(tmp_path):
    """A budget abort exits 2 and still writes a summary — partial stats,
    `exhausted` false, and the reason. `--max-enterings` is the parity-safe
    budget; `--max-time` is not (it depends on machine speed)."""
    d = _summary(tmp_path, str(EXAMPLES / "zebra2.ein"), "-e", "-E", "3",
                 check=False)
    assert d["verdict"]["type"] == "Aborted"
    assert d["verdict"]["reason"] == "max-enterings (3) reached"
    assert d["verdict"]["exhausted"] is False


def test_json_summary_is_shuffle_invariant(tmp_path):
    """The verdict, the counters and the root shape are the same under every
    `--shuffle` seed — the pinned property (S1.5b.31), now diffable.

    The `solutions` array is sorted by model for exactly this reason: which of
    k models is found first is a traversal fact, so leaving it in engine order
    would make T0 report a difference on the runs whose point is that there is
    none. Only `config.lattice-order-seed` — the *input* — differs.
    """
    import json
    seen = []
    for seed in ("7", "99", "1234"):
        out = tmp_path / f"{seed}.json"
        _run(str(FIXTURE), "-m", "3", "-e", "-z", "-d", seed,
             "--json-summary", str(out))
        d = json.loads(out.read_text(encoding="utf-8"))
        assert d["verdict"]["k"] == 2
        seen.append({k: d[k] for k in ("verdict", "stats", "root")})
    assert seen[0] == seen[1] == seen[2]
