#!/usr/bin/env python3
"""Process-level end-to-end timings — the denominators of P1a.6's targets.

`utils/bench_baseline.py` times engine calls *inside* one warm interpreter,
which is the right shape for comparing a `parse` against a `parse`. It is the
wrong shape for the milestone's headline claim: "`solve zebra2.ein -e`
end-to-end, 4.07 s under PyPy". End-to-end is a **process** — interpreter
start-up, imports, a cold JIT, the run, the print — and that is what a user
waits for and what `ein.rs` has to beat.

So this measures processes, three implementations against the same argv:

    utils/bench_env.sh python3 utils/e2e_baseline.py
    utils/bench_env.sh python3 utils/e2e_baseline.py --runs 7 --json out.json
    utils/bench_env.sh python3 utils/e2e_baseline.py -k 'zebra2 -e'

Reported per cell: **best, median, spread** (max - min as a % of the median)
and **peak RSS** of the child. Best-of-N is the estimator — the machine's
`powersave` governor and the other tenants on it can only make a run slower —
and the spread is printed next to it so a reader can see when best-of-N was
doing real work. A cell whose spread is large is not a measurement, it is a
request to re-run on a quieter machine.

Per-child RSS comes from `os.wait4`, not from `resource.getrusage(CHILDREN)`,
which reports the high-water mark over *all* children ever reaped and would
therefore attribute PyPy's footprint to `ein.rs`.
"""
from __future__ import annotations

import argparse
import json
import os
import shlex
import statistics
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
EIN_RS = REPO / "ein.rs" / "target" / "release" / "ein"
PYPY = REPO / ".venv-pypy" / "bin" / "python"

# One argv per row, expanded against every implementation. `-e` is the
# exhaustive path (`stop_after=None`), the bare form is the shipped default
# (`stop_after=1`).
WORKLOADS: list[tuple[str, list[str]]] = [
    ("solve zebra2 -e", ["solve", "examples/zebra2.ein", "-e"]),
    ("solve zebra2", ["solve", "examples/zebra2.ein"]),
    ("solve zebra -e", ["solve", "examples/zebra.ein", "-e"]),
    ("solve zebra", ["solve", "examples/zebra.ein"]),
    ("render zebra2", ["render", "rules", "examples/zebra2.ein"]),
    ("saturate zebra2", ["saturate", "examples/zebra2.ein"]),
]


def implementations(only: str | None) -> list[tuple[str, list[str]]]:
    impls: list[tuple[str, list[str]]] = [
        ("ein.py CPython", [sys.executable, "-m", "ein.cli"]),
    ]
    if PYPY.exists():
        impls.append(("ein.py PyPy", [str(PYPY), "-m", "ein.cli"]))
    if EIN_RS.exists():
        impls.append(("ein.rs release", [str(EIN_RS)]))
    if only:
        impls = [i for i in impls if only in i[0]]
    return impls


def run_once(argv: list[str], env: dict[str, str]) -> tuple[float, int, int]:
    """One child, timed around `fork`+`wait`: (seconds, peak RSS KiB, rc)."""
    t0 = time.perf_counter()
    proc = subprocess.Popen(
        argv, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env
    )
    _pid, status, usage = os.wait4(proc.pid, 0)
    elapsed = time.perf_counter() - t0
    proc.returncode = os.waitstatus_to_exitcode(status)
    return elapsed, int(usage.ru_maxrss), proc.returncode


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--runs", type=int, default=5, help="timed runs per cell (default 5)")
    ap.add_argument("--warmup", type=int, default=1, help="untimed runs first (default 1)")
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR",
                    help="only workloads whose label contains SUBSTR")
    ap.add_argument("--impl", default=None, metavar="SUBSTR",
                    help="only implementations whose label contains SUBSTR")
    ap.add_argument("--json", type=Path, default=None, metavar="FILE")
    args = ap.parse_args()

    env = dict(os.environ)
    env.pop("EIN_STDLIB", None)
    env["PYTHONPATH"] = str(REPO / "ein.py" / "src")
    env["LC_ALL"] = "C"

    impls = implementations(args.impl)
    rows: list[dict] = []
    print(f"{'workload':<18}{'impl':<17}{'best':>10}{'median':>10}"
          f"{'spread':>9}{'peak RSS':>11}", file=sys.stderr)
    print("─" * 75, file=sys.stderr)
    for label, argv in WORKLOADS:
        if args.only and args.only not in label:
            continue
        for impl, prefix in impls:
            full = [*prefix, *argv]
            for _ in range(args.warmup):
                run_once(full, env)
            samples, rss, rc = [], 0, 0
            for _ in range(args.runs):
                t, peak, rc = run_once(full, env)
                if rc != 0:
                    break
                samples.append(t)
                rss = max(rss, peak)
            if rc != 0 or not samples:
                print(f"{label:<18}{impl:<17}   exit {rc} — skipped", file=sys.stderr)
                rows.append({"workload": label, "impl": impl, "error": f"exit {rc}",
                             "argv": shlex.join(full)})
                continue
            best, med = min(samples), statistics.median(samples)
            spread = (max(samples) - min(samples)) / med * 100
            print(f"{label:<18}{impl:<17}{best * 1e3:>8.1f}ms{med * 1e3:>8.1f}ms"
                  f"{spread:>8.1f}%{rss / 1024:>9.1f}MB", file=sys.stderr)
            rows.append({
                "workload": label, "impl": impl, "argv": shlex.join(full),
                "best_ms": round(best * 1e3, 2), "median_ms": round(med * 1e3, 2),
                "spread_pct": round(spread, 2), "peak_rss_mb": round(rss / 1024, 2),
                "runs": len(samples), "samples_ms": [round(s * 1e3, 2) for s in samples],
            })
    if args.json:
        args.json.write_text(json.dumps({"rows": rows}, indent=2) + "\n",
                             encoding="utf-8")
        print(f"\nartifact: {args.json}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
