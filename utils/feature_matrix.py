#!/usr/bin/env python3
"""S1a.6.7 — the `features.md` lever matrix.

Which `SolverConfig` knobs are load-bearing, measured by flipping exactly one
lever off the puzzle's own all-on configuration and re-solving. Recorded per
cell: the verdict, every `MonotonicStats` counter, the goal bindings, the
engine's own solve time, the whole process's wall clock and its peak RSS.

Was S1.20.I3, which drove ein.py in-process. S1a.6.7 made it drive both engines
as **processes**, so that the ein.rs column of `features.md` was measured the
way the ein.py column had been and the two could be cross-checked cell by cell.
[S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md)
takes the second column out: the cross-check went with the engine it compared,
the ein.py numbers in `features.md` are frozen where they stand, and what is
left is the measurement itself — one engine, one lever at a time.

    utils/bench_env.sh python3 utils/feature_matrix.py
    utils/bench_env.sh python3 utils/feature_matrix.py --runs 5
    utils/bench_env.sh python3 utils/feature_matrix.py \
        --puzzle examples/zebra.ein --cells lookahead --modes exhaustive

## How a lever reaches the engine

Through the IR, not through the CLI. `ein solve` exposes five of these ten
levers as flags; the `(config …)` head exposes all of them, and the loader
**keeps the last block in the file** (`from_ir.rs` `config_blocks.last()`). So
a cell is a copy of the puzzle with one generated `(config …)` block appended,
holding the puzzle's *own resolved* configuration with one key changed.

The base comes from a baseline `--json-summary`, which reports the resolved
config — so the cells are not hand-transcribed from the puzzle's source, and
every cell differs from the baseline in exactly one key. Each run reads its own
summary's `config` block back and checks it: a cell that did not flip the lever
it names is reported as `config!` rather than quietly measured. Float-valued
knobs are skipped because the surface lexer has no float literal — `(config
:hypgen-rel-weight 1.0)` is a parse error, which is also why no puzzle can set
one.

## What `wall_s` means

The engine's own **solve** — root saturation + hypothesis search — read from
the `--timing` table, which is the same quantity the in-process harness timed
around `solve()` and therefore comparable to the 2026-08-17 column in
`features.md`. `proc_s` is the whole process (start-up, parse, load, solve,
print); under ein.rs those front phases are ~2 ms, where under PyPy they were
~0.6 s of constant that compressed every ratio.

Best-of-`--runs`, the **process** spread reported next to it (`samples_ms` and
`solve_samples_ms` in the artifact carry both series), one fresh process per
run, and
the runs go **round-robin over the cells** rather than finishing one cell
before starting the next. That is not fastidiousness: measured cell-by-cell on
this machine, PyPy's baseline — which ran first, on the coolest core — read
20 % faster than it did mid-run, and since every ratio in the table is
divided by the baseline, eight inert levers came out as a uniform 1.2x tax.
ein.rs measured the same eight at exactly 1.0x with the same entering counts,
which is what identified the bias as the harness's rather than the engine's.
Round-robin is what remains of that finding, and it is why the `control` cell
exists: with one column and no second engine to disagree with it, the control
is the only thing that prices the method.

A cell that *aborts* is run once, because an abort is a budget, not a timing.

## What went with the second engine

`--impl`, `--python` and `--check`. The last was a T1 parity check dressed as a
benchmark — every cell's verdict, `k`, exhaustion, counters and goal bindings,
engine against engine — and it had a real job: a *timing* comparison between
two engines that explored different numbers of commitments would have been
meaningless. There is one engine, so the cross-check is the row it would have
compared, and `config_ok` (did this cell actually flip its lever?) is the only
self-check left. What still makes the table falsifiable across time is the
counters: an optimisation that moves `enterings_total` moved the search, not
the clock.
"""
from __future__ import annotations

import argparse
import json
import os
import platform
import re
import shutil
import statistics
import subprocess
import sys
import tempfile
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
EIN_RS = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

# One config override per lever, against the puzzle's own (all-on) config.
# Keys are `SolverConfig` field names — the artifact has carried them since
# S1.20.I3 — and are kebab-cased on the way into the generated `(config …)`.
CELLS: dict[str, dict] = {
    "baseline":                 {},
    "no-lookahead":             {"enable_pre_branch_lookahead": False},
    "no-kill-cache":            {"enable_lookahead_kill_cache": False},
    "no-path-nogoods":          {"enable_path_nogoods": False},
    "no-symmetric-mirror":      {"enable_symmetric_mirror": False},
    "no-singleton-writeback":   {"enable_singleton_writeback": False},
    "no-forced-positive":       {"enable_forced_positive": False},
    "no-fail-fast-fork":        {"enable_fail_fast_fork": False},
    "hypgen-most-constrained":  {"hypgen_scoring": "most-constrained"},
    "lattice-score-sum":        {"lattice_order": "score-sum"},
    # The control, and it is not decoration: byte-identical to `baseline`
    # (same overrides, a different filename) and measured **last** in every
    # round. Every ratio in the table is divided by the baseline, so the
    # baseline's own position is the one systematic error the table cannot
    # see. `control` reads it off: whatever it says about a cell that differs
    # from the baseline in nothing at all is what the method contributes to
    # every other row.
    "control":                  {},
}
# mode -> (exhaustive?, wall budget for the solve, in seconds)
MODES: dict[str, tuple[bool, float]] = {"fast": (False, 30.0), "exhaustive": (True, 90.0)}

# The counters a cell is judged by. Every one is in `summary.json`, which is
# what T0/T1 used to compare — so a cell's row is still the shape a comparison
# would have read, across commits rather than across engines.
COUNTERS = (
    "enterings_total", "enterings_alive", "enterings_dead_pre",
    "enterings_dead_post", "facts_merged", "forced_positives",
    "saturate_count", "layers_explored", "nogoods_emitted",
    "nogoods_subsumed", "solution_nodes", "exhausted",
)

TIMING_LINE = re.compile(
    r"^\s{2}(parse|kb load|root saturation|hypothesis search|solve|end-to-end)"
    r"\s+([\d.]+)\s*$|"
    r"^\s{2}(parse|kb load|root saturation|hypothesis search|solve|end-to-end)"
    r"\s+([\d.]+)\s+\(", re.M)


def engine() -> list[str]:
    """The `ein` argv prefix. `$EIN_BIN` names a different build."""
    if not EIN_RS.exists():
        sys.exit(
            f"{EIN_RS} does not exist — build it with `cargo build --release "
            f"-p ein-cli`, or name one with $EIN_BIN"
        )
    return [str(EIN_RS)]


def child_env() -> dict[str, str]:
    env = dict(os.environ)
    env["EIN_STDLIB"] = str(REPO / "stdlib")   # cells live outside the checkout
    env["LC_ALL"] = "C"
    return env


def config_literal(value) -> str | None:
    """A `(config …)` value, or None for the ones the IR cannot express."""
    if value is None or isinstance(value, float):
        return None
    if isinstance(value, bool):
        return "true" if value else "false"
    return str(value)


def resolved_config(ein: list[str], puzzle: Path, env: dict[str, str],
                    tmp: Path) -> dict:
    """The puzzle's own config, as the engine resolves it — the cells' base."""
    out = tmp / "resolve.json"
    proc = subprocess.run([*ein, "solve", str(puzzle), "--max-enterings", "0",
                           "--json-summary", str(out)],
                          cwd=REPO, env=env, capture_output=True, text=True)
    if not out.exists():
        sys.exit(f"cannot resolve {puzzle}'s config: {proc.stderr.strip()[-400:]}")
    return json.loads(out.read_text())["config"]


def write_cell(puzzle: Path, base: dict, overrides: dict, dest: Path) -> None:
    """The puzzle, plus one `(config …)` block that is base + overrides."""
    merged = dict(base)
    for field, value in overrides.items():
        merged[field.replace("_", "-")] = value
    pairs = [f"  :{k} {lit}" for k, v in merged.items()
             if (lit := config_literal(v)) is not None]
    dest.write_text(
        puzzle.read_text(encoding="utf-8")
        + "\n;;; ── utils/feature_matrix.py cell ──────────────────────────\n"
        ";;; The puzzle's own resolved config with one lever changed. The\n"
        ";;; loader keeps the LAST (config …) block, so this one wins.\n"
        "(config\n" + "\n".join(pairs) + ")\n",
        encoding="utf-8")


def run_once(argv: list[str], env: dict[str, str]) -> tuple[float, int, int, str]:
    """One child: (wall seconds, peak RSS KiB, exit code, stdout)."""
    t0 = time.perf_counter()
    proc = subprocess.Popen(argv, cwd=REPO, env=env, stdout=subprocess.PIPE,
                            stderr=subprocess.DEVNULL, text=True)
    out = proc.stdout.read() if proc.stdout else ""
    _pid, status, usage = os.wait4(proc.pid, 0)
    return (time.perf_counter() - t0, int(usage.ru_maxrss),
            os.waitstatus_to_exitcode(status), out)


def phases(stdout: str) -> dict[str, float]:
    """The `--timing` table, as a dict of milliseconds."""
    got: dict[str, float] = {}
    for m in TIMING_LINE.finditer(stdout):
        name, value = (m.group(1), m.group(2)) if m.group(1) else (m.group(3), m.group(4))
        got[name] = float(value)
    return got


def measure(ein: list[str], cells: list[str], mode: str,
            files: dict[str, Path], puzzle: str, runs: int, env: dict[str, str],
            tmp: Path) -> list[dict]:
    """Every cell of one mode, **round-robin over the runs**.

    Not cell-by-cell: the first cell measured is the one the machine is
    coolest for, and on this box that is worth 20 % — enough to make eight
    inert levers read as a uniform 1.2x tax on the engine they were measured
    against. Round-robin spreads any drift over every cell instead of
    concentrating it on the baseline, which is the one cell every ratio is
    divided by. A cell that *aborts* is not repeated: an abort is a budget.
    """
    exhaustive, budget = MODES[mode]
    acc: dict[str, dict] = {}
    summary_path = tmp / "cell.json"
    for run_i in range(runs):
        for cell in cells:
            state = acc.setdefault(cell, {"walls": [], "solves": [], "rss": 0,
                                          "summary": None, "aborted": False})
            if state["aborted"] and run_i:
                continue
            if run_i == 0:
                print(f"… {puzzle} {cell} [{mode}]", file=sys.stderr, flush=True)
            argv = [*ein, "solve", str(files[cell]), "-t",
                    "--max-time", str(budget),
                    "--json-summary", str(summary_path)]
            if exhaustive:
                argv.append("-e")
            if summary_path.exists():
                summary_path.unlink()
            wall, peak, rc, out = run_once(argv, env)
            if not summary_path.exists():
                state["error"] = f"exit {rc}, no summary"
                state["aborted"] = True
                continue
            state["summary"] = json.loads(summary_path.read_text())
            state["walls"].append(wall)
            state["solves"].append(phases(out).get("solve", 0.0) / 1000.0)
            state["rss"] = max(state["rss"], peak)
            state["aborted"] = rc == 2

    return [row_of(puzzle, cell, mode, state) for cell, state in acc.items()]


def row_of(puzzle: str, cell: str, mode: str, state: dict) -> dict:
    """One artifact row from the accumulated runs of one cell."""
    if state.get("summary") is None:
        return {"puzzle": puzzle, "cell": cell, "mode": mode,
                "error": state.get("error", "no summary")}
    summary = state["summary"]
    verdict = summary["verdict"]
    stats = summary["stats"]
    solutions = verdict.get("solutions") or []
    binds = solutions[0].get("goal_bindings", []) if solutions else []
    got_config = summary["config"]
    want = {k.replace("_", "-"): v for k, v in CELLS[cell].items()}
    walls, solves = state["walls"], state["solves"]
    row = {
        "puzzle": puzzle, "cell": cell, "mode": mode,
        "verdict": verdict["type"],
        "aborted": verdict["type"] == "Aborted",
        "k": verdict.get("k", 0),
        "config_ok": all(got_config.get(k) == v for k, v in want.items()),
        "wall_s": round(min(solves), 3),          # the engine's own solve
        "proc_s": round(min(walls), 3),           # the whole process
        "spread_pct": round((max(walls) - min(walls))
                            / statistics.median(walls) * 100, 1),
        "runs": len(walls),
        "samples_ms": [round(w * 1e3, 1) for w in walls],
        "solve_samples_ms": [round(w * 1e3, 1) for w in solves],
        "rss_mb": round(state["rss"] / 1024, 1),
        "bindings": binds,
        "n_facts": len(solutions[0]["facts"]) if solutions else 0,
        "unsat_core": len(verdict.get("unsat_core") or []),
    }
    row.update({c: stats[c] for c in COUNTERS if c in stats})
    return row


def report(rows: list[dict]) -> int:
    """Per puzzle × mode: every cell against the baseline.

    Returns the number of cells that are not a measurement — an error, or a
    `config!` (the generated `(config …)` did not take). Non-zero is the exit
    code, which is what makes a silently-inert lever a failure rather than a
    row nobody reads.
    """
    bad = 0
    for puzzle in dict.fromkeys(r["puzzle"] for r in rows):
        for mode in MODES:
            here = [r for r in rows if r["puzzle"] == puzzle and r["mode"] == mode]
            if not here:
                continue
            print(f"\n{puzzle} [{mode}]", file=sys.stderr)
            print(f"  {'cell':<26}{'verdict':<12}{'k':>2}"
                  f"{'enter':>7}{'dead':>6}{'solve':>10}{'xbase':>7}"
                  f"{'proc':>10}{'spread':>8}{'rss':>8}", file=sys.stderr)
            base = next((r["wall_s"] for r in here
                         if r["cell"] == "baseline" and "error" not in r), 0)
            for r in here:
                if "error" in r:
                    print(f"  {r['cell']:<26}ERROR {r['error']}", file=sys.stderr)
                    bad += 1
                    continue
                factor = f"{r['wall_s'] / base:.1f}x" if base else "—"
                dead = r.get("enterings_dead_pre", 0) + r.get("enterings_dead_post", 0)
                # `config!` is the only self-check left after the second
                # engine: a cell that did not flip the lever it names is
                # measuring the baseline twice, and would otherwise read as an
                # inert lever.
                flags = ("  ABORTED" if r["aborted"] else "") + \
                        ("  config!" if not r["config_ok"] else "")
                if not r["config_ok"]:
                    bad += 1
                print(f"  {r['cell']:<26}{r['verdict']:<12}"
                      f"{r['k']:>2}{r.get('enterings_total', 0):>7}{dead:>6}"
                      f"{r['wall_s'] * 1e3:>8.0f}ms{factor:>7}"
                      f"{r['proc_s'] * 1e3:>8.0f}ms{r['spread_pct']:>7.1f}%"
                      f"{r['rss_mb']:>6.0f}MB{flags}", file=sys.stderr)
    return bad


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--puzzle", action="append", default=None, metavar="PATH",
                    help="puzzle to run the matrix on (repeatable; "
                         "default examples/zebra2.ein)")
    ap.add_argument("--cells", default=None, metavar="SUBSTR",
                    help="only cells whose name contains SUBSTR (`baseline` and "
                         "`control` always run — the second is what states the "
                         "column's resolution)")
    ap.add_argument("--modes", default=",".join(MODES), metavar="A,B")
    ap.add_argument("--runs", type=int, default=3, help="timed runs per cell (default 3)")
    ap.add_argument("--json", type=Path, metavar="FILE",
                    default=REPO / "utils" / "feature_matrix_results.json")
    args = ap.parse_args()

    puzzles = [Path(p) for p in (args.puzzle or ["examples/zebra2.ein"])]
    modes = [m for m in args.modes.split(",") if m in MODES]
    cells = [c for c in CELLS
             if c in ("baseline", "control") or not args.cells or args.cells in c]
    ein = engine()
    env = child_env()

    tmp = Path(tempfile.mkdtemp(prefix="ein-feature-matrix-"))
    rows: list[dict] = []
    try:
        for puzzle in puzzles:
            base = resolved_config(ein, puzzle, env, tmp)
            files = {}
            for cell in cells:
                files[cell] = tmp / f"{puzzle.stem}--{cell}.ein"
                write_cell(puzzle, base, CELLS[cell], files[cell])
            for mode in modes:
                rows.extend(measure(ein, cells, mode, files, str(puzzle),
                                    args.runs, env, tmp))
    finally:
        shutil.rmtree(tmp, ignore_errors=True)

    artifact = {
        "provenance": {
            "date": time.strftime("%Y-%m-%dT%H:%M:%S%z"),
            "commit": subprocess.run(["git", "-C", str(REPO), "rev-parse", "--short", "HEAD"],
                                     capture_output=True, text=True).stdout.strip(),
            "machine": f"{platform.platform()} / "
                       f"{platform.processor() or platform.machine()}",
            "ein_bin": str(EIN_RS),
            "runs": args.runs,
            "budgets": {m: MODES[m][1] for m in modes},
        },
        "rows": rows,
    }
    args.json.write_text(json.dumps(artifact, indent=2) + "\n", encoding="utf-8")
    bad = report(rows)
    print(f"\nartifact: {args.json}", file=sys.stderr)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
