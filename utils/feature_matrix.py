#!/usr/bin/env python3
"""S1a.6.7 — the `features.md` lever matrix, for **either** implementation.

Which `SolverConfig` knobs are load-bearing, measured by flipping exactly one
lever off the puzzle's own all-on configuration and re-solving. Recorded per
cell: the verdict, every `MonotonicStats` counter, the goal bindings, the
engine's own solve time, the whole process's wall clock and its peak RSS.

Was S1.20.I3, which drove ein.py in-process. S1a.6.7 makes it drive **both
engines as processes**, so the ein.rs column of `features.md` is measured the
way the ein.py column is, and the two can be cross-checked cell by cell.

    utils/bench_env.sh python3 utils/feature_matrix.py
    utils/bench_env.sh python3 utils/feature_matrix.py --impl ein.rs --runs 5
    utils/bench_env.sh python3 utils/feature_matrix.py \
        --puzzle examples/zebra.ein --cells lookahead --modes exhaustive

## How a lever reaches an engine

Through the IR, not through the CLI. `ein solve` exposes five of these ten
levers as flags; the `(config …)` head exposes all of them, and **both loaders
keep the last block in the file** (`from_ir.py` `config_blocks[-1]`,
`from_ir.rs` `config_blocks.last()`). So a cell is a copy of the puzzle with
one generated `(config …)` block appended, holding the puzzle's *own resolved*
configuration with one key changed.

The base comes from a baseline `--json-summary`, which reports the resolved
config — so the cells are not hand-transcribed from the puzzle's source, and
every cell differs from the baseline in exactly one key. Each run reads its own
summary's `config` block back and checks it: a cell that did not flip the lever
it names is reported as `config!` rather than quietly measured. Float-valued
knobs are skipped because the surface lexer has no float literal — `(config
:hypgen-rel-weight 1.0)` is a parse error in *both* engines, which is also why
no puzzle can set one.

## What `wall_s` means

The engine's own **solve** — root saturation + hypothesis search — read from
the `--timing` table, which is the same quantity the in-process harness timed
around `solve()` and therefore comparable to the 2026-08-17 column in
`features.md`. `proc_s` is the whole process (start-up, parse, load, solve,
print); under PyPy those front phases are ~0.6 s of constant, which compresses
every ratio, and under ein.rs they are ~2 ms.

Best-of-`--runs`, spread reported next to it, one fresh process per run, and
the runs go **round-robin over the cells** rather than finishing one cell
before starting the next. That is not fastidiousness: measured cell-by-cell on
this machine, PyPy's baseline — which runs first, on the coolest core — reads
20 % faster than it does mid-run, and since every ratio in the table is
divided by the baseline, eight inert levers came out as a uniform 1.2x tax.
ein.rs measured the same eight at exactly 1.0x with the same entering counts,
which is what identified the bias as the harness's rather than the engine's.

A cell that *aborts* is run once, because an abort is a budget, not a timing.

## Cross-checking

`--check` (on by default when both engines run) compares every cell's verdict,
`k`, exhaustion, counters and goal bindings between the two engines and exits
non-zero on a mismatch. That is a T1 parity check dressed as a benchmark: a
timing comparison between two engines that explored different numbers of
commitments would be meaningless.
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
EIN_RS = REPO / "ein.rs" / "target" / "release" / "ein"
PYPY = REPO / ".venv-pypy" / "bin" / "python"

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
# what T0/T1 compare, so agreement here *is* the parity check.
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


def implementations(only: str | None, python: str) -> list[tuple[str, list[str]]]:
    """The two engines, as argv prefixes — the labels `e2e_baseline.py` uses."""
    impls: list[tuple[str, list[str]]] = []
    kind = "PyPy" if Path(python) == PYPY else "CPython"
    impls.append((f"ein.py {kind}", [python, "-m", "ein.cli"]))
    if EIN_RS.exists():
        impls.append(("ein.rs release", [str(EIN_RS)]))
    if only:
        impls = [i for i in impls if only in i[0]]
    return impls


def child_env() -> dict[str, str]:
    env = dict(os.environ)
    env["PYTHONPATH"] = str(REPO / "ein.py" / "src")
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


def resolved_config(impl: list[str], puzzle: Path, env: dict[str, str],
                    tmp: Path) -> dict:
    """The puzzle's own config, as the engine resolves it — the cells' base."""
    out = tmp / "resolve.json"
    proc = subprocess.run([*impl, "solve", str(puzzle), "--max-enterings", "0",
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
        ";;; The puzzle's own resolved config with one lever changed. Both\n"
        ";;; loaders keep the LAST (config …) block, so this one wins.\n"
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


def measure(impls: list[tuple[str, list[str]]], cells: list[str], mode: str,
            files: dict[str, Path], puzzle: str, runs: int, env: dict[str, str],
            tmp: Path) -> list[dict]:
    """Every (impl, cell) of one mode, **round-robin over the runs**.

    Not cell-by-cell: the first cell measured is the one the machine is
    coolest for, and on this box that is worth 20 % — enough to make eight
    inert levers read as a uniform 1.2x tax on the engine they were measured
    against. Round-robin spreads any drift over every cell instead of
    concentrating it on the baseline, which is the one cell every ratio is
    divided by. A cell that *aborts* is not repeated: an abort is a budget.
    """
    exhaustive, budget = MODES[mode]
    acc: dict[tuple[str, str], dict] = {}
    for run_i in range(runs):
        for cell in cells:
            for label, prefix in impls:
                key = (label, cell)
                state = acc.setdefault(key, {"walls": [], "solves": [], "rss": 0,
                                             "summary": None, "aborted": False})
                if state["aborted"] and run_i:
                    continue
                if run_i == 0:
                    print(f"… {puzzle} {cell} [{mode}] {label}", file=sys.stderr,
                          flush=True)
                summary_path = tmp / f"cell-{plan_slug(label)}.json"
                argv = [*prefix, "solve", str(files[cell]), "-t",
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

    rows = []
    for (label, cell), state in acc.items():
        rows.append(row_of(label, puzzle, cell, mode, state))
    return rows


def plan_slug(label: str) -> str:
    return re.sub(r"[^A-Za-z0-9]+", "-", label).strip("-")


def row_of(label: str, puzzle: str, cell: str, mode: str, state: dict) -> dict:
    """One artifact row from the accumulated runs of one (impl, cell)."""
    if state.get("summary") is None:
        return {"impl": label, "puzzle": puzzle, "cell": cell, "mode": mode,
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
        "impl": label, "puzzle": puzzle, "cell": cell, "mode": mode,
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


def cross_check(rows: list[dict]) -> int:
    """Verdict + counters + bindings, engine against engine. T1, as a table."""
    impls = sorted({r["impl"] for r in rows})
    if len(impls) < 2:
        return 0
    a, b = impls[0], impls[1]
    by_key = {(r["impl"], r["puzzle"], r["cell"], r["mode"]): r for r in rows}
    fields = ("verdict", "k", "bindings", "n_facts", "unsat_core", *COUNTERS)
    bad = budget = 0
    print(f"\ncross-check — {a} vs {b}", file=sys.stderr)
    for key, ra in sorted(by_key.items()):
        if key[0] != a:
            continue
        rb = by_key.get((b, *key[1:]))
        if rb is None or "error" in ra or "error" in rb:
            continue
        if ra.get("aborted") or rb.get("aborted"):
            # One side stopped on a budget the other did not reach. The two
            # runs did different amounts of work, so there is nothing to
            # compare — and calling that a parity failure would train the
            # reader to ignore the column.
            budget += 1
            print(f"  ~ {key[1]} {key[2]} [{key[3]}]  budget: "
                  f"{a}={ra['verdict']}@{ra.get('enterings_total')} "
                  f"{b}={rb['verdict']}@{rb.get('enterings_total')}",
                  file=sys.stderr)
            continue
        diffs = [f"{f}: {ra.get(f)} vs {rb.get(f)}"
                 for f in fields if ra.get(f) != rb.get(f)]
        if diffs:
            bad += 1
            print(f"  ✗ {key[1]} {key[2]} [{key[3]}]  " + "; ".join(diffs[:4]),
                  file=sys.stderr)
    if not bad:
        n = sum(1 for k in by_key if k[0] == a) - budget
        print(f"  ✓ {n} cells agree on verdict, k, bindings and "
              f"{len(COUNTERS)} counters"
              + (f" ({budget} exempt: one side stopped on its budget)"
                 if budget else ""), file=sys.stderr)
    return bad


def report(rows: list[dict]) -> None:
    """Per puzzle × mode: the cells, sorted by the engine's own solve time."""
    for puzzle in dict.fromkeys(r["puzzle"] for r in rows):
        for mode in MODES:
            here = [r for r in rows if r["puzzle"] == puzzle and r["mode"] == mode]
            if not here:
                continue
            print(f"\n{puzzle} [{mode}]", file=sys.stderr)
            print(f"  {'cell':<26}{'impl':<16}{'verdict':<12}{'k':>2}"
                  f"{'enter':>7}{'dead':>6}{'solve':>10}{'xbase':>7}"
                  f"{'proc':>10}{'spread':>8}{'rss':>8}", file=sys.stderr)
            for impl in dict.fromkeys(r["impl"] for r in here):
                mine = [r for r in here if r["impl"] == impl]
                base = next((r["wall_s"] for r in mine if r["cell"] == "baseline"), 0)
                for r in mine:
                    if "error" in r:
                        print(f"  {r['cell']:<26}{impl:<16}ERROR {r['error']}",
                              file=sys.stderr)
                        continue
                    factor = f"{r['wall_s'] / base:.1f}x" if base else "—"
                    dead = r.get("enterings_dead_pre", 0) + r.get("enterings_dead_post", 0)
                    flags = ("  ABORTED" if r["aborted"] else "") + \
                            ("  config!" if not r["config_ok"] else "")
                    print(f"  {r['cell']:<26}{impl:<16}{r['verdict']:<12}"
                          f"{r['k']:>2}{r.get('enterings_total', 0):>7}{dead:>6}"
                          f"{r['wall_s'] * 1e3:>8.0f}ms{factor:>7}"
                          f"{r['proc_s'] * 1e3:>8.0f}ms{r['spread_pct']:>7.1f}%"
                          f"{r['rss_mb']:>6.0f}MB{flags}", file=sys.stderr)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--impl", default=None, metavar="SUBSTR",
                    help="only implementations whose label contains SUBSTR")
    ap.add_argument("--puzzle", action="append", default=None, metavar="PATH",
                    help="puzzle to run the matrix on (repeatable; "
                         "default examples/zebra2.ein)")
    ap.add_argument("--cells", default=None, metavar="SUBSTR",
                    help="only cells whose name contains SUBSTR (baseline always runs)")
    ap.add_argument("--modes", default=",".join(MODES), metavar="A,B")
    ap.add_argument("--runs", type=int, default=3, help="timed runs per cell (default 3)")
    ap.add_argument("--python", default=str(PYPY if PYPY.exists() else sys.executable),
                    metavar="PATH", help="the interpreter ein.py runs under")
    ap.add_argument("--json", type=Path, metavar="FILE",
                    default=REPO / "utils" / "feature_matrix_results.json")
    ap.add_argument("--no-check", action="store_true",
                    help="skip the engine-against-engine cross-check")
    args = ap.parse_args()

    puzzles = [Path(p) for p in (args.puzzle or ["examples/zebra2.ein"])]
    modes = [m for m in args.modes.split(",") if m in MODES]
    cells = [c for c in CELLS
             if c in ("baseline", "control") or not args.cells or args.cells in c]
    impls = implementations(args.impl, args.python)
    if not impls:
        sys.exit("no implementation selected")
    env = child_env()

    tmp = Path(tempfile.mkdtemp(prefix="ein-feature-matrix-"))
    rows: list[dict] = []
    try:
        for puzzle in puzzles:
            base = resolved_config(impls[0][1], puzzle, env, tmp)
            files = {}
            for cell in cells:
                files[cell] = tmp / f"{puzzle.stem}--{cell}.ein"
                write_cell(puzzle, base, CELLS[cell], files[cell])
            for mode in modes:
                rows.extend(measure(impls, cells, mode, files, str(puzzle),
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
            "python": args.python,
            "ein_rs": str(EIN_RS) if EIN_RS.exists() else None,
            "runs": args.runs,
            "budgets": {m: MODES[m][1] for m in modes},
        },
        "rows": rows,
    }
    args.json.write_text(json.dumps(artifact, indent=2) + "\n", encoding="utf-8")
    report(rows)
    bad = 0 if args.no_check else cross_check(rows)
    print(f"\nartifact: {args.json}", file=sys.stderr)
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
