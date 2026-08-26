#!/usr/bin/env python3
"""What a layer of the search costs and what it buys — S1d.10.1's instrument.

The engine's search is a complete BFS over commitment-set cardinality
([design/07](../docs/history/m1a_rust/design/07_search_layer.md)), and the one
mechanism that makes it affordable is **death**: a dead commitment licenses a
learned clause, and clauses filter the next layer's candidates before anything
forks. So a layer that kills nothing learns nothing, and the next layer is the
full prefix join.

That sentence had never been a number. `nogoods_emitted` says what the deaths
produced; nothing said what the clauses *removed*. This counts both ends, per
layer, per corpus entry — the column `dropped_nogood` is the one that was
missing, and [M1d P1d.10](../docs/history/m1d_satisfiability/README.md#p1d10--exhaustive-search)
is the phase that needs it.

    utils/layer_census.py                        # the table, to stdout
    utils/layer_census.py --json census.json     # + the machine copy
    utils/layer_census.py -k zebra2-minus-15 --layers    # one entry, per layer
    utils/layer_census.py --all-runs             # every declared search run

**One run per entry, and it is `solve -e`.** A regime is a property of a
puzzle, not of a flag: `solve` stops at the first model and never reaches the
layers this is about, and the levers (`-L`, `-K`, `-o score-sum`) vary the
engine rather than the workload. `--all-runs` sweeps every declared run that
reaches the search, which is what says whether a lever moves a regime.

**An entry that does not finish is depth-capped, not dropped.** `--timeout`
(default 60 s) kills the run and retries at `-m 3`, `-m 2`, `-m 1` until one
fits; the cap that succeeded is in the `cap` column, and a capped row's last
layer is truncated by the cap rather than by the lattice. That is the honest
form of the phase's opening case — `examples/zebra2-minus-15.ein` finds all 32
models at depth 3 and does not finish at 5 — and it is why the depth is a
column instead of a footnote.

The transport is the `layer` event
([events.md](../docs/kernel/inference/events.md)), added for this stage: one
JSON line per layer carrying the row. Everything here is that line, tabulated
— no counter is re-derived from the entering stream, because a census that
reconstructs its own subject is a census that can disagree with the engine.

**`--events` goes to a FIFO, never to a file.** An exhaustive
`zebra2-minus-15` at `-m 3` narrates 72.6 M events; keeping them cost a 16 GiB
`/tmp` before the sweep reached its second entry. A named pipe costs no disk at
any depth, and sixteen integers per layer is all this keeps.

Argv follows `ein-corpus/src/plan.rs`, mirrored the way
[`corpus_cost.py`](corpus_cost.py) and [`stdlib_census.py`](stdlib_census.py)
mirror it, and for the same reason: a `cargo test` in the middle of a sweep is
a worse dependency than six lines.
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
MANIFEST = REPO / "corpus" / "corpus.toml"
EIN = Path(os.environ.get("EIN_BIN", REPO / "ein.rs" / "target" / "release" / "ein"))

#: The depth caps tried, in order, when a run outlives `--timeout`. `None` is
#: the engine's own default (`-m 5`), i.e. no cap at all.
CAPS = (None, 3, 2, 1)

#: Subcommands that run the search *and* take `--events`. `saturate` reaches a
#: fixpoint and stops — no layers, so nothing to count; `render` has no
#: `--events` at all and `render lattice`'s solve is a subset of `solve -e`'s.
SEARCH_SUBCOMMANDS = ("solve", "test")


# ── the sweep ───────────────────────────────────────────────

def argv_for(run: str, file: str, out: Path) -> list[str]:
    """`ein-corpus::plan::argv`, mirrored — see the module docstring."""
    toks = [t.replace("{out}", str(out)) for t in run.split()]
    if toks[0] == "render":
        return [toks[0], *toks[1:2], file, *toks[2:]]
    return [toks[0], file, *toks[1:]]


def all_runs(entry: dict) -> list[str]:
    return [*entry.get("runs", []), *(f"solve {lv}" for lv in entry.get("levers", []))]


def search_runs(entry: dict, every: bool) -> list[str]:
    """The runs to sweep: `solve -e`, or every declared search run.

    `solve -e` is synthesised when the entry does not declare it — which is the
    case the phase exists for. `examples/zebra2-minus-15.ein` drops `solve -e`
    from its `runs` precisely because the run does not finish, and a census
    that honoured that would be blind to the regime it is measuring.
    """
    if not every:
        return ["solve -e"]
    seen, out = set(), []
    for run in all_runs(entry):
        if run.split()[0] in SEARCH_SUBCOMMANDS and run not in seen:
            seen.add(run)
            out.append(run)
    if "solve -e" not in seen:
        out.append("solve -e")
    return out


PAGE_KIB = os.sysconf("SC_PAGE_SIZE") // 1024

#: What to ask the kernel for on the event pipe. Bigger is fewer wake-ups and
#: less back-pressure on the engine; `/proc/sys/fs/pipe-max-size` is the
#: ceiling an unprivileged process may set, and 1 MiB is the usual value.
PIPE_BYTES = 1 << 20


def rss_kib(pid: int) -> int:
    """The child's resident set, now — `/proc/<pid>/statm` field 2, in KiB."""
    try:
        with open(f"/proc/{pid}/statm", encoding="ascii") as fh:
            return int(fh.read().split()[1]) * PAGE_KIB
    except (OSError, IndexError, ValueError):
        return 0


class Sink:
    """The narrated run's event stream, read from a **FIFO** and never stored.

    `--events` names a path and the engine writes a line per event, which for
    the run this phase is about is not a file anybody wants: an exhaustive
    `zebra2-minus-15` at `-m 3` narrates 72.6 M events, and a corpus sweep that
    kept them filled a 16 GiB `/tmp` before it reached the second entry. A
    named pipe costs no disk at any depth, and the census keeps sixteen
    integers per layer rather than the stream that carried them.

    The read end is opened **non-blocking before the child starts**, which is
    what makes this fit inside the existing poll loop instead of needing a
    thread: with no writer yet a non-blocking `O_RDONLY` returns a usable fd
    immediately, `read` raises `BlockingIOError` while the engine has nothing
    to say, and returns `b""` only once every writer has closed — which, since
    the caller reaps the child first, is unambiguously the end.
    """

    def __init__(self, path: Path):
        self.path = path
        path.unlink(missing_ok=True)
        os.mkfifo(path)
        self.fd = os.open(path, os.O_RDONLY | os.O_NONBLOCK)
        try:
            import fcntl
            fcntl.fcntl(self.fd, 1031, PIPE_BYTES)     # F_SETPIPE_SZ
        except OSError:
            pass                                       # the default will do
        self.buf = b""
        self.layers: list[dict] = []

    def drain(self) -> None:
        """Every line available right now."""
        while True:
            try:
                chunk = os.read(self.fd, 1 << 16)
            except BlockingIOError:
                return
            if not chunk:
                return
            self.buf += chunk
            *lines, self.buf = self.buf.split(b"\n")
            for line in lines:
                if b'"layer"' in line:
                    ev = json.loads(line)
                    if ev.get("e") == "layer":
                        self.layers.append(ev)

    def close(self) -> None:
        os.close(self.fd)
        self.path.unlink(missing_ok=True)


def run_once(argv: list[str], env: dict, timeout: float, ceiling_kib: int,
             sink: Sink | None = None) -> tuple[float, int, int | None, str]:
    """One child: (seconds, peak RSS KiB, exit code, why it stopped).

    Peak RSS is the child's own, from `os.wait4`, the way
    [`e2e_baseline.py`](e2e_baseline.py) takes it: `getrusage(CHILDREN)` is a
    running maximum over every child the sweep has ever reaped, so its delta is
    not this run's.

    **The ceiling is not politeness, it is the sweep being possible at all.**
    Four corpus entries have no finite hypothesis space and grow until the
    kernel stops them — 14.3 GB for `features/04_open`
    ([`corpus_cost.py`](corpus_cost.py)) — and this sweep runs `solve -e` on
    entries that deliberately do not declare it. So the poll that watches the
    clock watches `/proc/<pid>/statm` too, and a run over the ceiling is
    stopped and re-tried at the next depth cap, exactly like a run over time.
    """
    t0 = time.perf_counter()
    proc = subprocess.Popen(argv, cwd=REPO, env=env, stdin=subprocess.DEVNULL,
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    deadline = t0 + timeout
    while True:
        if sink:
            sink.drain()
        pid, status, usage = os.wait4(proc.pid, os.WNOHANG)
        if pid:
            proc.returncode = os.waitstatus_to_exitcode(status)
            if sink:
                sink.drain()
            return time.perf_counter() - t0, int(usage.ru_maxrss), proc.returncode, "ok"
        why = ("time" if time.perf_counter() > deadline else
               "rss" if rss_kib(proc.pid) > ceiling_kib else None)
        if why:
            proc.kill()
            # Keep draining while it dies: the engine may be blocked writing
            # into a full pipe, and a killed writer still has to be reaped.
            while True:
                if sink:
                    sink.drain()
                if os.wait4(proc.pid, os.WNOHANG)[0]:
                    break
                time.sleep(0.001)
            proc.returncode = -9
            return time.perf_counter() - t0, 0, None, why
        if not sink:
            time.sleep(0.002)


def measure(path: str, run: str, args, env: dict, out: Path) -> dict:
    """One (entry, run) cell — **two children**, and the split is the point.

    The first carries no `--events` and is what the `ms` and `MiB` columns
    report: [events.md](../docs/kernel/inference/events.md)'s fourth ground
    rule is that an instrumented run is never a benchmark, and it is not idle
    advice — `zebra2-minus-15 -m 2` is 1.46 s bare and 5.0 s narrated. It also
    chooses the depth cap, so "does this entry finish" means what a reader
    means by it.

    The second re-runs at that cap with `--events` into a [`Sink`], and is
    where every census column comes from. It gets `--events-factor` times the
    budget, because narrating costs what it costs; a `layer` line is written at
    the layer's close, so even a killed narration yields whole rows for the
    layers it did finish, and the row says how far it got.
    """
    caps = CAPS if args.cap_on_timeout else (None,)
    ceiling = int(args.max_rss_mb * 1024)
    why = "ok"
    for cap in caps:
        capv = ["-m", str(cap)] if cap is not None else []
        bare = [str(args.bin), *argv_for(run, path, out), *capv]
        wall, rss, rc, why = run_once(bare, env, args.timeout, ceiling)
        if rc is None:
            continue                       # over budget; try the next cap down
        sink = Sink(out / "events.fifo")
        try:
            narrated = [*bare, "--events", str(sink.path), "--events-level", "normal"]
            _w, _r, erc, ewhy = run_once(narrated, env,
                                         args.timeout * args.events_factor,
                                         ceiling, sink)
            layers = sink.layers
        finally:
            sink.close()
        return {"path": path, "run": run, "cap": cap, "wall": wall, "rss_kib": rss,
                "rc": rc, "layers": layers, "timed_out": False,
                "narration": "ok" if erc is not None else ewhy}
    return {"path": path, "run": run, "cap": caps[-1], "wall": args.timeout,
            "rss_kib": 0, "rc": None, "layers": [], "timed_out": True,
            "narration": why}


def sweep(entries: list[dict], args) -> list[dict]:
    env = dict(os.environ)
    env.pop("EIN_STDLIB", None)
    env["LC_ALL"] = "C"
    rows = []
    root = Path(tempfile.mkdtemp(prefix="ein-layer-census-"))
    try:
        for i, entry in enumerate(entries):
            out = root / f"{i:04d}"
            out.mkdir(parents=True, exist_ok=True)
            for run in search_runs(entry, args.all_runs):
                if not args.quiet:
                    print(f"  … {entry['path']} [{run}]", file=sys.stderr, flush=True)
                rows.append(measure(entry["path"], run, args, env, out))
            shutil.rmtree(out, ignore_errors=True)
    finally:
        shutil.rmtree(root, ignore_errors=True)
    return rows


# ── the classification ──────────────────────────────────────

def regime(row: dict) -> str:
    """Which regime a cell is in — from the census, never from the file name.

    Three, and the boundary between the last two is the phase's subject:

    * **no-search** — phase 2 never ran a layer, or ran one with nothing in it.
      Root decided the puzzle (or refused to load it).
    * **pruning** — layer 1 killed something. Deaths are what clauses are made
      of, so this is the regime every prune in the engine was designed against
      and measured on.
    * **barren** — layer 1 entered candidates and killed **none** of them. No
      death, no clause, no writeback: layer 2 is the full `C(n, 2)` and the
      only thing that can shrink it is a death that has not happened yet.

    "Deaths at layer 1" is the classifier because it is the one a reader can
    apply to a new puzzle after 24 ms — `solve -e -m 1` is layer 1 and nothing
    else. What it costs is that a puzzle whose layer 1 prunes and whose layer 3
    goes barren reads as *pruning*; the per-layer rows below are where that
    shows, and `barren_layers` counts them.
    """
    layers = row["layers"]
    if not layers:
        return "no-search"
    first = layers[0]
    if first["entered"] == 0:
        return "no-search"
    deaths = first["dead_pre"] + first["dead_post"]
    return "pruning" if deaths else "barren"


def barren_layers(row: dict) -> int:
    """Layers that entered candidates and emitted no clause."""
    return sum(1 for l in row["layers"]
               if l["entered"] and l["nogoods_emitted"] == 0)


def d_found(row: dict) -> int | None:
    """The last depth that yielded a model — S1d.10.2's first column, free."""
    found = [l["layer"] for l in row["layers"] if l["models"]]
    return found[-1] if found else None


def totals(row: dict) -> dict:
    l = row["layers"]
    return {
        "layers": len(l),
        "entered": sum(x["entered"] for x in l),
        "joined": sum(x["joined"] for x in l),
        "dropped_nogood": sum(x["dropped_nogood"] for x in l),
        "dropped_dead": sum(x["dropped_dead"] for x in l),
        "emitted": sum(x["nogoods_emitted"] for x in l),
        "models": sum(x["models"] for x in l),
    }


# ── the report ──────────────────────────────────────────────

def pct(num: int, den: int) -> str:
    return f"{100.0 * num / den:5.1f}%" if den else "    —"


def print_layers(row: dict) -> None:
    print(f"\n{row['path']} [{row['run']}]"
          + (f"  -m {row['cap']}" if row["cap"] else "")
          + f"   {row['wall'] * 1000:.0f} ms   {row['rss_kib'] / 1024:.0f} MiB")
    print(f"  {'L':>2} {'alive':>7} {'front':>7} {'joined':>9} {'−dead':>8} "
          f"{'−clause':>9} {'cand':>9} {'entered':>9} {'deaths':>8} "
          f"{'clauses':>8} {'wb':>5} {'models':>7} {'next':>9}  filtered")
    for x in row["layers"]:
        deaths = x["dead_pre"] + x["dead_post"]
        print(f"  {x['layer']:>2} {x['alive']:>7} {x['frontier']:>7} "
              f"{x['joined']:>9} {x['dropped_dead']:>8} {x['dropped_nogood']:>9} "
              f"{x['candidates']:>9} {x['entered']:>9} {deaths:>8} "
              f"{x['nogoods_emitted']:>8} {x['writebacks']:>5} {x['models']:>7} "
              f"{x['next']:>9}  {pct(x['dropped_dead'] + x['dropped_nogood'], x['joined'])}")


def report(rows: list[dict], args) -> int:
    w = max((len(r["path"]) for r in rows), default=20) + 2
    print(f"{'entry':<{w}}{'run':<12}{'cap':>4}{'regime':>10}{'L':>4}"
          f"{'entered':>10}{'joined':>10}{'−clause':>10}{'filt':>7}"
          f"{'clauses':>9}{'k':>5}{'d_found':>8}{'ms':>9}{'MiB':>7}")
    print("─" * (w + 92))
    buckets: dict[str, list[dict]] = {}
    for row in sorted(rows, key=lambda r: (r["path"], r["run"])):
        t = totals(row)
        reg = "timeout" if row["timed_out"] else regime(row)
        buckets.setdefault(reg, []).append(row)
        found = d_found(row)
        print(f"{row['path']:<{w}}{row['run']:<12}"
              f"{row['cap'] if row['cap'] else '—':>4}{reg:>10}{t['layers']:>4}"
              f"{t['entered']:>10}{t['joined']:>10}{t['dropped_nogood']:>10}"
              f"{pct(t['dropped_nogood'] + t['dropped_dead'], t['joined']):>7}"
              f"{t['emitted']:>9}{t['models']:>5}"
              f"{found if found is not None else '—':>8}"
              f"{row['wall'] * 1000:>9.0f}{row['rss_kib'] / 1024:>7.0f}")

    print(f"\n{'─' * 40}\nregimes")
    for reg in ("no-search", "pruning", "barren", "timeout"):
        rs = buckets.get(reg, [])
        if not rs:
            continue
        ent = sum(totals(r)["entered"] for r in rs)
        print(f"  {reg:<12}{len(rs):>4} cells{ent:>12} enterings")
        if reg in ("barren", "timeout"):
            for r in rs:
                print(f"      {r['path']} [{r['run']}]")

    joined = sum(totals(r)["joined"] for r in rows)
    dead = sum(totals(r)["dropped_dead"] for r in rows)
    clause = sum(totals(r)["dropped_nogood"] for r in rows)
    print(f"\ncorpus-wide, the two filter arms over {joined} joined candidates:")
    print(f"  dropped by a dead element   {dead:>10}  {pct(dead, joined)}")
    print(f"  dropped by a learned clause {clause:>10}  {pct(clause, joined)}")

    if args.layers:
        for row in sorted(rows, key=lambda r: (r["path"], r["run"])):
            if row["layers"]:
                print_layers(row)

    failures = [r for r in rows if r["timed_out"]]
    if failures and args.check:
        return 1
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--bin", default=EIN, type=Path,
                    help=f"the ein binary (default $EIN_BIN or {EIN})")
    ap.add_argument("-k", "--only", default=None, metavar="SUBSTR",
                    help="only entries whose path contains SUBSTR")
    ap.add_argument("--all-runs", action="store_true",
                    help="every declared run that reaches the search, not just solve -e")
    ap.add_argument("--layers", action="store_true",
                    help="print the per-layer rows under the summary table")
    ap.add_argument("--timeout", type=float, default=60.0,
                    help="seconds before a run is killed and re-tried capped (default 60)")
    ap.add_argument("--events-factor", type=float, default=8.0,
                    help="the narrated run's budget, as a multiple of --timeout (default 8)")
    ap.add_argument("--max-rss-mb", type=float, default=2048.0,
                    help="kill and re-try capped above this resident set (default 2048)")
    ap.add_argument("--no-cap-on-timeout", dest="cap_on_timeout", action="store_false",
                    help="report a timeout instead of retrying at -m 3, 2, 1")
    ap.add_argument("--json", type=Path, default=None, metavar="FILE",
                    help="also write the machine copy")
    ap.add_argument("--check", action="store_true",
                    help="exit 1 if any cell could not be measured at any depth")
    ap.add_argument("-q", "--quiet", action="store_true", help="no progress on stderr")
    args = ap.parse_args()

    if not args.bin.exists():
        print(f"{args.bin} does not exist — build it with `./build.sh --engine`, "
              f"or name one with --bin / $EIN_BIN", file=sys.stderr)
        return 2

    entries = tomllib.loads(MANIFEST.read_text(encoding="utf-8"))["entry"]
    if args.only:
        entries = [e for e in entries if args.only in e["path"]]
    if not entries:
        print("no entries selected", file=sys.stderr)
        return 2

    rows = sweep(entries, args)
    rc = report(rows, args)
    if args.json:
        args.json.write_text(json.dumps(
            {"rows": [{**r, "regime": "timeout" if r["timed_out"] else regime(r),
                       "totals": totals(r), "d_found": d_found(r),
                       "barren_layers": barren_layers(r)} for r in rows]},
            indent=1), encoding="utf-8")
    return rc


if __name__ == "__main__":
    sys.exit(main())
