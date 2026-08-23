#!/usr/bin/env python3
"""Process-level end-to-end timings — the denominators of P1a.6's targets.

`cargo bench` times engine calls *inside* one process, which is the right shape
for comparing a `parse` against a `parse`. It is the wrong shape for the
milestone's headline claim: "`solve zebra2.ein -e` end-to-end, 4.07 s under
PyPy". End-to-end is a **process** — start-up, the run, the print — and that is
what a user waits for and what `ein.rs` had to beat.

So this measures processes, one `ein` binary per series against the same argv:

    utils/bench_env.sh python3 utils/e2e_baseline.py
    utils/bench_env.sh python3 utils/e2e_baseline.py --runs 7 --json out.json
    utils/bench_env.sh python3 utils/e2e_baseline.py -k 'zebra2 -e'
    utils/bench_env.sh python3 utils/e2e_baseline.py \
        --bin system=ein.rs/target-alloc-system/release/ein \
        --bin snmalloc=ein.rs/target/release/ein          # two builds, one series

**One engine, still worth a process timing.** Until
[S1a.10.4](../docs/history/m1a_rust/README.md#s1a104--utils-re-aimed-at-one-engine)
the rows were three implementations — CPython, PyPy, ein.rs — and the ratio
between them was the point. The PyPy and CPython columns in
[baseline.md](../docs/history/m1a_rust/measurements/baseline.md) are frozen
constants now: nothing can re-measure them. What this still answers is the
question `--bin` was added for and the one every P1a.6 stage asked — *did this
change make the shipping binary faster?* — across builds (allocator arms,
feature flags, `--profile`s) and across commits, which is why it survives the
oracle instead of collapsing into `cargo bench`. `$EIN_BIN` moves the default.

Reported per cell: **best, median, spread** (max - min as a % of the median)
and **peak RSS** of the child. Best-of-N is the estimator — the machine's
`powersave` governor and the other tenants on it can only make a run slower —
and the spread is printed next to it so a reader can see when best-of-N was
doing real work. A cell whose spread is large is not a measurement, it is a
request to re-run on a quieter machine.

Per-child RSS comes from `os.wait4`, not from `resource.getrusage(CHILDREN)`,
which reports the high-water mark over *all* children ever reaped and would
therefore attribute a previous series' footprint to this one.
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
EIN_RS = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

# One argv per row, expanded against every binary. `-e` is the
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


# The blind enumerator's own cells (S1a.6.4). **None of the six rows above
# reaches it**: zebra and zebra2 both declare an `(hrule …)`, so `generate`
# returns before `candidate_objects` ever runs, and the enumerator the rest of
# the corpus uses was invisible to this table. These five are the corpus's
# slowest `solve` cells — `features/05 -e` alone is 46x `solve zebra -e` — and
# they are what a change to the blind path is measured on. Selected with
# `--blind`.
BLIND_WORKLOADS: list[tuple[str, list[str]]] = [
    ("features/05 -e", ["solve", "examples/features/05_stdlib_domain_elim.ein", "-e"]),
    ("features/01 -e", ["solve", "examples/features/01_not_and_absent.ein", "-e"]),
    ("branching/07 -e", ["solve", "examples/branching/07_lookahead_off.ein", "-e"]),
    ("branching/06 -e", ["solve", "examples/branching/06_lookahead_on.ein", "-e"]),
    ("sq-bwd/houses -e", ["solve", "examples/saturation/square-bwd/houses.ein", "-e"]),
]


# Start-up (S1a.6.5 T1a.6.5.6). "A 5 ms engine behind a 40 ms start-up is not
# a fast tool", and neither of the two rows above can see start-up: they are
# dominated by the work. `--help` does no engine work at all, so it *is* the
# floor — process creation, the dynamic loader, `clap`'s formatter — and
# `solve friends` is that floor plus the smallest real program in the corpus
# (651 bytes, one rule, one fact, no imports). Selected with `--startup`.
STARTUP_WORKLOADS: list[tuple[str, list[str]]] = [
    ("--help", ["--help"]),
    ("solve friends", ["solve", "examples/saturation/symmetric/friends.ein"]),
    ("saturate friends", ["saturate", "examples/saturation/symmetric/friends.ein"]),
]


def binaries(extra: list[str] | None) -> list[tuple[str, list[str]]]:
    """The `ein` binaries to time, as `(label, argv prefix)`.

    Default: one row, `$EIN_BIN` or the release build. `--bin LABEL=PATH`
    replaces it with a named series, which is the reason the flag exists —
    two builds of the same engine against each other (an allocator, a layout,
    a feature) with everything else held constant.

    There is deliberately **no `--impl`**. It selected among three
    implementations and there is one; a flag that takes a single value is an
    invitation to look for the operand that is gone.
    """
    if extra:
        out = []
        for spec in extra:
            label, _, path = spec.partition("=")
            if not path:
                label, path = Path(label).parent.parent.name, label
            out.append((label, [str(Path(path).resolve())]))
        return out
    if not EIN_RS.exists():
        sys.exit(
            f"{EIN_RS} does not exist — build it with `cargo build --release "
            f"-p ein-cli`, or name one with --bin / $EIN_BIN"
        )
    return [("ein.rs release", [str(EIN_RS)])]


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
    ap.add_argument("--bin", action="append", default=None, metavar="LABEL=PATH",
                    help="compare named ein binaries instead of the default "
                         "one; repeatable")
    ap.add_argument("--blind", action="store_true",
                    help="the blind-enumerator cells instead of the milestone six")
    ap.add_argument("--startup", action="store_true",
                    help="the start-up cells instead of the milestone six")
    ap.add_argument("--json", type=Path, default=None, metavar="FILE")
    args = ap.parse_args()

    env = dict(os.environ)
    env.pop("EIN_STDLIB", None)
    env["LC_ALL"] = "C"

    bins = binaries(args.bin)
    rows: list[dict] = []
    width = max(17, *(len(b[0]) + 1 for b in bins))
    print(f"{'workload':<18}{'binary':<{width}}{'best':>10}{'median':>10}"
          f"{'spread':>9}{'peak RSS':>11}", file=sys.stderr)
    print("─" * 75, file=sys.stderr)
    selected = WORKLOADS
    if args.blind:
        selected = BLIND_WORKLOADS
    elif args.startup:
        selected = STARTUP_WORKLOADS
    for label, argv in selected:
        if args.only and args.only not in label:
            continue
        for label_bin, prefix in bins:
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
                print(f"{label:<18}{label_bin:<{width}}   exit {rc} — skipped",
                      file=sys.stderr)
                rows.append({"workload": label, "binary": label_bin, "error": f"exit {rc}",
                             "argv": shlex.join(full)})
                continue
            best, med = min(samples), statistics.median(samples)
            spread = (max(samples) - min(samples)) / med * 100
            print(f"{label:<18}{label_bin:<{width}}{best * 1e3:>8.1f}ms{med * 1e3:>8.1f}ms"
                  f"{spread:>8.1f}%{rss / 1024:>9.1f}MB", file=sys.stderr)
            rows.append({
                "workload": label, "binary": label_bin, "argv": shlex.join(full),
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
