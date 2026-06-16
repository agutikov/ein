#!/usr/bin/env python3
"""S1.20.I3 — feature × config bench matrix for `zebra2.ein`.

Measures the impact of each load-bearing engine lever by solving zebra2
with that lever flipped off (vs the all-on baseline), recording verdict,
the `MonotonicStats` counters, wall-time and peak RSS.

Each cell runs in a FRESH subprocess (clean wall-time + RSS, no PyPy JIT
carryover). Two modes per cell: `fast` (stop_after=1, the shipped fast
path) and `exhaustive` (stop_after=None — a disabled prune shows its full
blow-up). A cell that exceeds its budget returns an `Aborted` verdict
(the "broken / won't-finish if off" sentinel).

Run from the repo root under PyPy:

    PYTHONPATH=ein.py/src .venv-pypy/bin/python utils/feature_matrix.py

Writes the raw artifact to utils/feature_matrix_results.json and prints a
summary. Parent re-invokes itself once per (cell, mode) via --cell/--mode.
"""
from __future__ import annotations

import argparse
import json
import resource
import subprocess
import sys
import time
from pathlib import Path

PUZZLE = "examples/zebra2.ein"

# One config override per lever (vs the all-default = all-on baseline).
CELLS: dict[str, dict] = {
    "baseline":                 {},
    "no-lookahead":             {"enable_pre_branch_lookahead": False},
    "no-kill-cache":            {"enable_lookahead_kill_cache": False},
    "no-path-nogoods":          {"enable_path_nogoods": False},
    "no-symmetric-mirror":      {"enable_symmetric_mirror": False},
    "no-singleton-writeback":   {"enable_singleton_writeback": False},
    "no-forced-positive":       {"enable_forced_positive": False},
    "hypgen-most-constrained":  {"hypgen_scoring": "most-constrained"},
    "lattice-score-sum":        {"lattice_order": "score-sum"},
}
MODES = {"fast": (1, 30.0), "exhaustive": (None, 90.0)}


def run_one(cell: str, mode: str) -> dict:
    """Child: solve zebra2 with the cell's config override; print one JSON line."""
    from ein.inference.config import SolverConfig
    from ein.inference.monotonic import solve
    from ein.inference.verdict import Solution, goal_bindings
    from ein.kb.store import KnowledgeBase

    stop_after, budget = MODES[mode]
    cfg = SolverConfig(**CELLS[cell])
    kb = KnowledgeBase.from_file(PUZZLE)
    t0 = time.time()
    verdict, stats = solve(
        kb, stop_after=stop_after, config=cfg,
        max_time=budget, on_budget="verdict",
    )
    wall = time.time() - t0
    rss_kb = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    binds = goal_bindings(verdict.kb) if isinstance(verdict, Solution) else []
    return {
        "cell": cell, "mode": mode,
        "verdict": type(verdict).__name__,
        "aborted": type(verdict).__name__ == "Aborted",
        "k": stats.solution_nodes,
        "exhausted": stats.exhausted,
        "enterings_total": stats.enterings_total,
        "enterings_dead_pre": stats.enterings_dead_pre,
        "enterings_dead_post": stats.enterings_dead_post,
        "layers_explored": stats.layers_explored,
        "forced_positives": stats.forced_positives,
        "saturate_count": stats.saturate_count,
        "wall_s": round(wall, 2),
        "rss_mb": round(rss_kb / 1024, 1),
        "bindings": binds,
    }


def parent() -> None:
    results: list[dict] = []
    for cell in CELLS:
        for mode in MODES:
            print(f"… {cell} [{mode}]", file=sys.stderr, flush=True)
            proc = subprocess.run(
                [sys.executable, __file__, "--cell", cell, "--mode", mode],
                capture_output=True, text=True,
            )
            line = proc.stdout.strip().splitlines()[-1] if proc.stdout.strip() else ""
            try:
                results.append(json.loads(line))
            except (ValueError, IndexError):
                results.append({"cell": cell, "mode": mode, "error":
                                proc.stderr.strip()[-400:] or "no output"})

    out = Path("utils/feature_matrix_results.json")
    out.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")

    # Summary: slowdown vs baseline per mode.
    base = {r["mode"]: r for r in results if r.get("cell") == "baseline"}
    print(f"\n{'cell':<26}{'mode':<12}{'verdict':<13}{'k':>3} {'ent':>6} "
          f"{'wall_s':>8} {'×base':>7}  {'rss_mb':>7}")
    for r in results:
        if "error" in r:
            print(f"{r['cell']:<26}{r['mode']:<12}ERROR: {r['error'][:60]}")
            continue
        b = base.get(r["mode"], {})
        bw = b.get("wall_s") or 0
        factor = f"{r['wall_s'] / bw:.1f}x" if bw else "—"
        flag = "  ABORTED" if r.get("aborted") else ""
        print(f"{r['cell']:<26}{r['mode']:<12}{r['verdict']:<13}{r['k']:>3} "
              f"{r['enterings_total']:>6} {r['wall_s']:>8} {factor:>7}  "
              f"{r['rss_mb']:>7}{flag}")
    print(f"\nartifact: {out}")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--cell")
    ap.add_argument("--mode")
    args = ap.parse_args()
    if args.cell:
        print(json.dumps(run_one(args.cell, args.mode)))
    else:
        parent()


if __name__ == "__main__":
    main()
