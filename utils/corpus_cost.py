#!/usr/bin/env python3
"""What every corpus cell costs, on the engine that ships — S1a.9.0's instrument.

`corpus/corpus.toml` marks entries `slow = true`, and until
[S1a.9.0](../docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced) that flag had
no threshold and no measurement behind it: it was set from a CPython probe in
2026-08-17 and never re-taken. This is what re-takes it, and what a reader runs
when they doubt a flag.

    utils/bench_env.sh python3 utils/corpus_cost.py                 # every declared cell
    utils/bench_env.sh python3 utils/corpus_cost.py --slow-only --runs 3
    utils/bench_env.sh python3 utils/corpus_cost.py -k square-unique \\
        --also 'solve,solve -e' --timeout 900 --runs 1
    python3 utils/corpus_cost.py --check --json cost.json           # flags vs the threshold

Reported per cell: **mean, sd, relative sd and n**, which is
[`criterion_table.py`](criterion_table.py)'s statistic rather than
[`e2e_baseline.py`](e2e_baseline.py)'s best-of-N — a `slow` flag is a claim
about what a sweep *costs*, so the estimator has to be the one that includes
the machine's bad runs, and a mean without a deviation is not a measurement.
Per entry it also prints what the declared runs cost **together**, which is
the number the `slow` threshold applies to — the flag's job is the sweep's
budget, and the sum is what an entry costs it (`corpus/README.md` § `slow`) —
with the slowest single run beside it for context.

`--check` compares what it measured against the manifest — the `slow` flag
against `--slow-ms`, and each entry's recorded `cost_ms` against the wall
clock — and exits 1 on a disagreement, so the flag cannot rot a second time
without something saying so. The same claim is a test twice over:
`ein-corpus`'s `slow_matches_the_recorded_cost` checks the manifest against
itself, and `corpus_cli`'s `the_slow_flag_still_describes_the_sweep` checks it
against the sweep it just ran.

**A cell can also die.** `exit -9` in the table is a child killed by a signal,
and on this corpus it means the **OOM killer**: four entries have no finite
hypothesis space, so at the `solve` default of `-m 5` they grow until the
kernel stops them (`journalctl -k | grep -i oom` has the size — 14.3 GB for
`features/04_open`). That is reported rather than retried, because it is the
measurement. What is *not* reported is peak RSS: this times cells with a
blocking `wait`, and the `os.wait4` polling loop that would yield `ru_maxrss`
costs a millisecond of resolution on cells that take three.

**Argv is `ein-corpus/src/plan.rs`'s rule, mirrored.** A run name is the `ein`
argv with the file position elided; `{out}` expands to the cell's output
directory and every `solve` run silently gains `--json-summary`. Two rules and
six lines, mirrored here rather than shelled out for, because the alternative
is a `cargo test` in the middle of a timing loop.
"""
from __future__ import annotations

import argparse
import json
import os
import shlex
import shutil
import statistics
import subprocess
import sys
import time
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANIFEST = REPO / "corpus" / "corpus.toml"
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

#: The `slow` threshold, in milliseconds of an entry's declared runs summed.
#: Stated in `corpus/README.md` § `slow` and checked by `ein-corpus`'s
#: `slow_matches_the_recorded_cost`; repeated here — as `ein_corpus::manifest`'s
#: `SLOW_MS`, which is the definition — because `--check` is what measures it.
SLOW_MS = 1000.0


def argv_for(run: str, file: str, out: Path) -> list[str]:
    """`ein-corpus::plan::argv`, mirrored. See the module docstring."""
    toks = [t.replace("{out}", str(out)) for t in run.split()]
    if toks[0] == "render":
        argv = [toks[0], *toks[1:2], file, *toks[2:]]
    else:
        argv = [toks[0], file, *toks[1:]]
    if toks[0] == "solve":
        argv += ["--json-summary", f"{out}/summary.json"]
    return argv


def slug(run: str) -> str:
    """`ein-corpus::plan::slug`, mirrored — a filesystem-safe cell directory."""
    out = ""
    for c in run:
        if c.isascii() and (c.isalnum() or c in "-."):
            out += c
        elif not out.endswith("_"):
            out += "_"
    return out.strip("_")


def all_runs(entry: dict) -> list[str]:
    """The declared runs, then one `solve <lever>` per lever — `Entry::all_runs`."""
    return [*entry.get("runs", []), *(f"solve {lv}" for lv in entry.get("levers", []))]


def run_once(argv: list[str], env: dict[str, str], out: Path,
             timeout: float) -> tuple[float, int | None]:
    """One child, timed. Returns (seconds, exit code) — code `None` on timeout."""
    out.mkdir(parents=True, exist_ok=True)
    t0 = time.perf_counter()
    with subprocess.Popen(argv, cwd=REPO, stdin=subprocess.DEVNULL,
                          stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                          env=env) as proc:
        try:
            code = proc.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait()
            return time.perf_counter() - t0, None
    return time.perf_counter() - t0, code


def measure(argv: list[str], env: dict[str, str], out: Path, args) -> dict:
    """One cell: warm it, then sample until `--runs` or `--budget` is spent.

    The warm-up is kept as a sample when the cell turns out to cost more than
    `--warmup-max`: discarding a 78 s run spends a minute on the page cache of
    a binary the previous cell already paged in. Whatever the budget says,
    every cell that finishes yields at least one sample.
    """
    samples: list[float] = []
    code: int | None = 0
    elapsed, code = run_once(argv, env, out, args.timeout)
    if code is None:
        return {"timeout_s": args.timeout, "n": 1, "code": None}
    if not (args.warmup and elapsed < args.warmup_max):
        samples.append(elapsed)
    spent = elapsed
    while len(samples) < max(1, args.runs) and (not samples or spent < args.budget):
        elapsed, code = run_once(argv, env, out, args.timeout)
        if code is None:
            return {"timeout_s": args.timeout, "n": len(samples) + 1, "code": None}
        samples.append(elapsed)
        spent += elapsed
    mean = statistics.fmean(samples)
    sd = statistics.stdev(samples) if len(samples) > 1 else 0.0
    return {
        "mean_ms": round(mean * 1e3, 3), "sd_ms": round(sd * 1e3, 3),
        "rsd_pct": round(sd / mean * 100, 2) if mean else 0.0,
        "min_ms": round(min(samples) * 1e3, 3), "n": len(samples), "code": code,
    }


def fmt_ms(ms: float) -> str:
    if ms >= 100_000:
        return f"{ms / 1e3:.0f} s"
    if ms >= 1000:
        return f"{ms / 1e3:.2f} s"
    return f"{ms:.1f} ms"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", type=Path, default=EIN, help="the ein binary ($EIN_BIN)")
    ap.add_argument("--runs", type=int, default=5, help="timed runs per cell (default 5)")
    ap.add_argument("--warmup", type=int, default=1, help="discard a first run (default 1)")
    ap.add_argument("--warmup-max", type=float, default=2.0, metavar="SEC",
                    help="only discard a first run that cost less than this (default 2)")
    ap.add_argument("--budget", type=float, default=20.0, metavar="SEC",
                    help="stop repeating a cell after this much wall clock (default 20)")
    ap.add_argument("--timeout", type=float, default=300.0, metavar="SEC",
                    help="kill a run after this long, and report the ceiling "
                         "(default 300, EIN_CORPUS_TIMEOUT's)")
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR",
                    help="only entries whose path contains SUBSTR")
    ap.add_argument("-r", "--run", action="append", default=None, metavar="RUN",
                    help="only these run names; repeatable")
    ap.add_argument("--slow-only", action="store_true", help="only `slow = true` entries")
    ap.add_argument("--also", default=None, metavar="RUN,RUN",
                    help="price these runs too, declared or not — how a dropped "
                         "run gets a number (S1a.9.0 T1a.9.0.3)")
    ap.add_argument("--slow-ms", type=float, default=SLOW_MS, metavar="MS",
                    help=f"the `slow` threshold (default {SLOW_MS:.0f})")
    ap.add_argument("--check", action="store_true",
                    help="exit 1 if a `slow` flag or a `cost_ms` disagrees with "
                         "the measurement")
    ap.add_argument("--check-factor", type=float, default=2.0, metavar="X",
                    help="how far a recorded cost_ms may be from the wall clock "
                         "before --check complains (default 2)")
    ap.add_argument("--json", type=Path, default=None, metavar="FILE")
    args = ap.parse_args()

    if not args.bin.exists():
        sys.exit(f"{args.bin} does not exist — cargo build --release -p ein-cli")
    manifest = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))
    entries = [e for e in manifest["entry"]
               if (not args.only or args.only in e["path"])
               and (not args.slow_only or e.get("slow"))]
    if not entries:
        sys.exit("no entries selected")
    extra = [r.strip() for r in args.also.split(",")] if args.also else []

    env = dict(os.environ)
    env.pop("EIN_STDLIB", None)
    env["LC_ALL"] = "C"
    root = Path(os.environ.get("TMPDIR", "/tmp")) / f"ein-corpus-cost-{os.getpid()}"

    w = max(len(e["path"]) for e in entries) + 2
    print(f"{'entry':<{w}}{'run':<26}{'mean':>11}{'sd':>10}{'rsd':>7}{'n':>3}",
          file=sys.stderr)
    print("─" * (w + 57), file=sys.stderr)
    rows: list[dict] = []
    for i, entry in enumerate(entries):
        path = entry["path"]
        declared = all_runs(entry)
        for run in [*declared, *(r for r in extra if r not in declared)]:
            if args.run and run not in args.run:
                continue
            out = root / f"{i:04d}" / slug(run)
            argv = [str(args.bin), *argv_for(run, path, out)]
            r = measure(argv, env, out, args)
            shutil.rmtree(out, ignore_errors=True)  # `--dump-states` × n reps
            row = {"path": path, "run": run, "declared": run in declared,
                   "slow": bool(entry.get("slow")), "argv": shlex.join(argv), **r}
            rows.append(row)
            if "mean_ms" in r:
                print(f"{path:<{w}}{run:<26}{fmt_ms(r['mean_ms']):>11}"
                      f"{fmt_ms(r['sd_ms']):>10}{r['rsd_pct']:>6.1f}%{r['n']:>3}"
                      + ("" if row["declared"] else "  (not declared)")
                      + ("" if r["code"] == 0 else f"  exit {r['code']}"),
                      file=sys.stderr)
            else:
                print(f"{path:<{w}}{run:<26}{'> ' + fmt_ms(r['timeout_s'] * 1e3):>11}"
                      f"{'—':>10}{'—':>7}{r['n']:>3}  killed", file=sys.stderr)

    rc = summarise(rows, entries, args)
    if args.json:
        args.json.write_text(json.dumps(
            {"slow_ms": args.slow_ms, "bin": str(args.bin), "rows": rows},
            indent=2) + "\n", encoding="utf-8")
        print(f"artifact: {args.json}", file=sys.stderr)
    return rc


def summarise(rows: list[dict], entries: list[dict], args) -> int:
    """Per entry: what its declared runs cost **together**, which is what
    `slow` is a claim about, with the slowest single run beside it for
    context. `corpus/README.md` § `slow` says why the sum and not the run."""
    w = max(len(e["path"]) for e in entries) + 2
    print(f"\n{'entry':<{w}}{'total':>11}   {'slowest declared run':<26}{'cost':>11}"
          f"{'flag':>7}{'recorded':>11}", file=sys.stderr)
    print("─" * (w + 69), file=sys.stderr)
    recorded = {e["path"]: e.get("cost_ms") for e in entries}
    bad: list[str] = []
    for path in dict.fromkeys(r["path"] for r in rows):
        mine = [r for r in rows if r["path"] == path and r["declared"]]
        if not mine:
            continue
        cost = lambda r: r.get("mean_ms", r.get("timeout_s", 0) * 1e3)
        worst = max(mine, key=cost)
        total = sum(cost(r) for r in mine)
        killed = any("mean_ms" not in r for r in mine)
        flag, want = worst["slow"], total >= args.slow_ms
        note = recorded.get(path)
        print(f"{path:<{w}}{('> ' if killed else '') + fmt_ms(total):>11}   "
              f"{worst['run']:<26}{fmt_ms(cost(worst)):>11}"
              f"{'slow' if flag else '—':>7}{fmt_ms(note) if note else '—':>11}"
              f"{'' if flag == want else '   ← flag disagrees'}", file=sys.stderr)
        if flag != want:
            bad.append(f"{path}: slow = {str(flag).lower()}, but its declared runs "
                       f"cost {fmt_ms(total)} together, against a "
                       f"{args.slow_ms:.0f} ms threshold")
        if note is not None and not killed:
            lo, hi = note / args.check_factor, note * args.check_factor
            if not lo <= total <= hi:
                bad.append(f"{path}: cost_ms = {note:.0f}, measured {fmt_ms(total)} "
                           f"(outside {args.check_factor:g}×)")
        if flag and note is None:
            bad.append(f"{path}: slow = true with no cost_ms — the flag has no "
                       f"measurement behind it")
    if not args.check:
        return 0
    for line in bad:
        print(f"  ✗ {line}", file=sys.stderr)
    print(f"\n{len(bad)} disagreement(s) between the manifest and the wall clock",
          file=sys.stderr)
    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main())
