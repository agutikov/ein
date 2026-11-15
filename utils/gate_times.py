#!/usr/bin/env python3
"""Where a gate run's time and memory go, per step and per test binary — M1f S1f.6.39's instrument.

    python3 utils/gate_times.py run --out build/t/gate.log -- ./run_tests.sh
    python3 utils/gate_times.py run --out build/t/nightly.log -- ./run_tests.sh --nightly
    python3 utils/gate_times.py run --out build/t/ci.log -- utils/ci_container.sh
    python3 utils/gate_times.py report build/t/gate.log            # the tables
    python3 utils/gate_times.py report build/t/gate.log --tsv t.tsv
    gh run view RUN --log > r.log; python3 utils/gate_times.py report r.log   # a CI run's

`run` runs the command with its stdout and stderr merged, echoes every line,
and writes it to `--out` stamped with the wall clock (`<epoch>\\t<line>`). A
thread samples the machine every `--every` seconds into the same file: the
memory in use (`MemTotal - MemAvailable`, so a tmpfs counts, as it does
against the OOM killer) and whether the mains supply is online. The power is
the point of the second column: gate #3's mid-run slowdown was the laptop on
battery (the user, 2026-11-15), and a log that does not say so cannot be told
from an engine regression. `run` prints a warning and records the moment the
supply goes off; it does not stop the command.

`report` reads such a log, or a GitHub Actions log (`gh run view --log`, whose
lines carry an ISO stamp after the job and step names), and prints:

- **steps**: every `run_tests.sh` banner (`── label ───`), with its wall
  time to the next banner or the end, and the peak memory over the step above
  the least the machine held during the run, or, where `run` sampled it,
  the peak of the run's own process tree, summed;
- **binaries**: every `Running …` line cargo prints, with the wall time to the
  next one, or to the next banner, and the harness's own `finished in` beside
  it (cargo's number excludes the binary's start and the doc tests; the wall
  one includes them);
- the power state: `AC throughout`, or each interval on battery, so a reader
  can drop the steps it touches.

Nothing here is a gate step: it is how a gate run is measured, and it imports
nothing outside the standard library.
"""
from __future__ import annotations

import argparse
import datetime as dt
import os
import re
import subprocess
import sys
import threading
import time
from pathlib import Path

# A banner is ruled to a fixed width, so a label past it has no rule at all.
BANNER = re.compile(r"^── (.+?)(?: ─*)?\s*$")
GH_STEP = re.compile(r"^##\[group\]Run (.+)$")
RUNNING = re.compile(r"Running (?:unittests )?(\S+) \(([^)]*)\)")
DOCTESTS = re.compile(r"Doc-tests (\S+)")
FINISHED = re.compile(r"test result: (\w+)\. (\d+) passed; (\d+) failed.*finished in ([0-9.]+)s")
GH_STAMP = re.compile(r"^[^\t]*\t[^\t]*\t(\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d\.\d+)Z (.*)$")
ANSI = re.compile(r"\x1b\[[0-9;]*m|\^\[\[[0-9;]*m")


def meminfo_kb() -> dict[str, int]:
    out = {}
    with open("/proc/meminfo") as f:
        for line in f:
            k, v = line.split(":", 1)
            out[k] = int(v.split()[0])
    return out


def used_kb() -> int:
    m = meminfo_kb()
    return m["MemTotal"] - m["MemAvailable"]


PAGE_KB = os.sysconf("SC_PAGE_SIZE") // 1024


def tree_kb(root: int) -> int:
    """The resident memory of `root` and every process below it, in kB: what
    the gate's own processes hold, which the machine's other tenants do not
    move. A container's processes are not below `run` (the daemon starts
    them), so for `ci_container.sh` this is the client alone and the machine's
    figure is the one to read."""
    parent: dict[int, int] = {}
    rss: dict[int, int] = {}
    for d in os.listdir("/proc"):
        if not d.isdigit():
            continue
        try:
            with open(f"/proc/{d}/stat") as f:
                stat = f.read()
            with open(f"/proc/{d}/statm") as f:
                pages = int(f.read().split()[1])
        except OSError:
            continue
        # The command may hold spaces and parentheses: the fields after the
        # last `)` are fixed.
        fields = stat[stat.rfind(")") + 2:].split()
        parent[int(d)] = int(fields[1])
        rss[int(d)] = pages * PAGE_KB
    children: dict[int, list[int]] = {}
    for pid, pp in parent.items():
        children.setdefault(pp, []).append(pid)
    total, todo = 0, [root]
    while todo:
        pid = todo.pop()
        total += rss.get(pid, 0)
        todo.extend(children.get(pid, []))
    return total


def mains_online() -> str:
    """`1`, `0`, or `?` where no supply of type Mains exists."""
    root = Path("/sys/class/power_supply")
    seen = "?"
    for d in sorted(root.glob("*")) if root.is_dir() else []:
        try:
            if (d / "type").read_text().strip() != "Mains":
                continue
            v = (d / "online").read_text().strip()
        except OSError:
            continue
        if v == "1":
            return "1"
        seen = v
    return seen


def cmd_run(args: argparse.Namespace) -> int:
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    lock = threading.Lock()
    f = out.open("w", buffering=1)
    done = threading.Event()

    def write(line: str) -> None:
        with lock:
            f.write(f"{time.time():.3f}\t{line}\n")

    write(f"#start\t{' '.join(args.command)}\tcwd={os.getcwd()}\tcpus={os.cpu_count()}")
    power = mains_online()
    write(f"#mem\t{used_kb()}\t{power}\t0")
    if power == "0":
        print("gate_times: ON BATTERY at the start — these timings are not the machine's",
              file=sys.stderr)

    proc_box: list = []

    def sample() -> None:
        last = power
        while not done.wait(args.every):
            p = mains_online()
            tree = tree_kb(proc_box[0].pid) if proc_box else 0
            write(f"#mem\t{used_kb()}\t{p}\t{tree}")
            if p != last:
                print(f"gate_times: mains online {last} -> {p}", file=sys.stderr)
                last = p

    t = threading.Thread(target=sample, daemon=True)
    t.start()
    proc = subprocess.Popen(args.command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                            text=True, errors="replace", bufsize=1)
    proc_box.append(proc)
    assert proc.stdout is not None
    for line in proc.stdout:
        line = line.rstrip("\n")
        sys.stdout.write(line + "\n")
        sys.stdout.flush()
        write(line)
    code = proc.wait()
    done.set()
    t.join()
    write(f"#mem\t{used_kb()}\t{mains_online()}\t0")
    write(f"#exit\t{code}")
    f.close()
    return code


def read_log(path: Path) -> tuple[list[tuple[float, str]], list[tuple[float, int, str, int]], int | None]:
    """`(lines, samples, exit)`: every output line with its stamp, every memory/power sample."""
    lines: list[tuple[float, str]] = []
    samples: list[tuple[float, int, str, int]] = []
    code = None
    text = path.read_text(errors="replace").splitlines()
    for raw in text:
        m = GH_STAMP.match(raw)
        if m:
            stamp = dt.datetime.fromisoformat(m.group(1)[:26]).replace(tzinfo=dt.timezone.utc)
            line = ANSI.sub("", m.group(2))
            # A GitHub step opens a group named after its command: read it as
            # the banner `run_tests.sh` prints for the same step.
            g = GH_STEP.match(line)
            lines.append((stamp.timestamp(), f"── {g.group(1)} ───" if g else line))
            continue
        head, _, rest = raw.partition("\t")
        try:
            t = float(head)
        except ValueError:
            continue
        if rest.startswith("#mem\t"):
            parts = rest.split("\t")
            kb, power = parts[1], parts[2]
            tree = int(parts[3]) if len(parts) > 3 else 0
            samples.append((t, int(kb), power, tree))
        elif rest.startswith("#exit\t"):
            code = int(rest.split("\t")[1])
        elif rest.startswith("#start\t"):
            continue
        else:
            lines.append((t, ANSI.sub("", rest)))
    # A GitHub log is per job, and the jobs ran side by side: in time order,
    # one job's lines interleave with the other's. A stamped local log is in
    # order already, and a stable sort keeps it so.
    lines.sort(key=lambda x: x[0])
    return lines, samples, code


def fmt(s: float) -> str:
    return f"{s:8.1f} s" if s < 600 else f"{s:8.1f} s ({s / 60:.1f} min)"


def cmd_report(args: argparse.Namespace) -> int:
    lines, samples, code = read_log(Path(args.log))
    if not lines:
        print("no stamped lines", file=sys.stderr)
        return 2
    t0, t_end = lines[0][0], lines[-1][0]
    if samples:
        t0 = min(t0, samples[0][0])
        t_end = max(t_end, samples[-1][0])
    # What the machine held when the run held least: a run frees what was in
    # use when it started too (a tmpfs emptied, a browser tab gone), so its
    # first sample is not its floor.
    base = min(kb for _, kb, _, _ in samples) if samples else 0

    # Markers: a step banner opens a step; a `Running` line opens a binary.
    steps: list[list] = []  # [label, start, end]
    bins: list[list] = []   # [step, name, start, end, finished_in, passed, failed]
    for t, line in lines:
        b = BANNER.match(line.strip())
        if b and not line.strip().startswith("── bench environment"):
            if steps:
                steps[-1][2] = t
            if bins and bins[-1][3] is None:
                bins[-1][3] = t
            steps.append([b.group(1), t, None])
            continue
        r = RUNNING.search(line) or DOCTESTS.search(line)
        if r:
            if bins and bins[-1][3] is None:
                bins[-1][3] = t
            name = r.group(1)
            if r.re is RUNNING:
                deps = r.group(2).rsplit("/", 1)[-1]
                if name.startswith("src/"):
                    name = f"{deps.rsplit('-', 1)[0]} ({name})"
            else:
                name = f"doc-tests {name}"
            bins.append([steps[-1][0] if steps else "", name, t, None, None, 0, 0])
            continue
        f = FINISHED.search(line)
        if f and bins:
            bins[-1][4] = float(f.group(4))
            bins[-1][5] = int(f.group(2))
            bins[-1][6] = int(f.group(3))
    if steps and steps[-1][2] is None:
        steps[-1][2] = t_end
    for b in bins:
        if b[3] is None:
            # The last binary of a step ends where its step does.
            owner = [s for s in steps if s[0] == b[0]]
            b[3] = owner[-1][2] if owner else t_end

    # The gate's own processes where the log has them (`run`'s fourth column),
    # else the machine's memory over the run's floor.
    own = any(tree for *_, tree in samples) and not args.machine

    def peak(a: float, z: float) -> int:
        if own:
            inside = [tree for t, _, _, tree in samples if a <= t <= z]
            return max(inside) if inside else 0
        inside = [kb for t, kb, _, _ in samples if a <= t <= z]
        return (max(inside) - base) if inside else 0

    print(f"log {args.log}: {fmt(t_end - t0).strip()} wall, exit {code}")
    if samples:
        if own:
            print(f"the run's own processes' peak resident memory {peak(t0, t_end) / 1048576:.1f} GiB "
                  "(every process below it, summed, sampled)")
        else:
            print(f"memory in use at the run's floor {base / 1048576:.1f} GiB; peak above it "
                  f"{peak(t0, t_end) / 1048576:.1f} GiB (the machine's; no process tree in this log)")
        off = [(t, p) for t, _, p, _ in samples if p == "0"]
        if not off:
            ps = {p for _, _, p, _ in samples}
            print("power: AC throughout" if ps == {"1"} else f"power: mains online {sorted(ps)}")
        else:
            print(f"power: ON BATTERY in {len(off)} of {len(samples)} samples, "
                  f"from +{off[0][0] - t0:.0f} s to +{off[-1][0] - t0:.0f} s — "
                  "drop the steps that overlap it")
    print("\n## steps\n")
    print(f"| step | wall | peak memory |")
    print("|---|---:|---:|")
    for label, a, z in steps:
        print(f"| {label} | {z - a:.1f} s | {peak(a, z) / 1048576:.2f} GiB |")
    if bins:
        print("\n## test binaries, slowest first\n")
        print("| binary | step | wall | `finished in` | tests | peak memory |")
        print("|---|---|---:|---:|---:|---:|")
        for step, name, a, z, fin, passed, failed in sorted(bins, key=lambda b: -(b[3] - b[2])):
            if args.min and z - a < args.min:
                continue
            fin_s = f"{fin:.2f} s" if fin is not None else "—"
            print(f"| {name} | {step} | {z - a:.1f} s | {fin_s} | {passed}"
                  f"{f' ({failed} failed)' if failed else ''} | {peak(a, z) / 1048576:.2f} GiB |")
    if args.tsv:
        with open(args.tsv, "w") as f:
            f.write("kind\tstep\tname\twall_s\tfinished_in_s\tpeak_gib\n")
            for label, a, z in steps:
                f.write(f"step\t{label}\t\t{z - a:.2f}\t\t{peak(a, z) / 1048576:.3f}\n")
            for step, name, a, z, fin, _, _ in bins:
                f.write(f"binary\t{step}\t{name}\t{z - a:.2f}\t{'' if fin is None else fin}"
                        f"\t{peak(a, z) / 1048576:.3f}\n")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    r = sub.add_parser("run", help="run a command, stamping its output and sampling the machine")
    r.add_argument("--out", required=True)
    r.add_argument("--every", type=float, default=1.0, help="seconds between samples (1)")
    r.add_argument("command", nargs=argparse.REMAINDER)
    p = sub.add_parser("report", help="the per-step and per-binary tables of a stamped log")
    p.add_argument("log")
    p.add_argument("--tsv")
    p.add_argument("--min", type=float, default=0.0, help="hide binaries under this many seconds")
    p.add_argument("--machine", action="store_true",
                   help="the machine's memory over the run's floor, not the run's process tree "
                        "(a container's processes are not below `run`)")
    args = ap.parse_args()
    if args.cmd == "run":
        if args.command and args.command[0] == "--":
            args.command = args.command[1:]
        if not args.command:
            ap.error("run: no command")
        return cmd_run(args)
    return cmd_report(args)


if __name__ == "__main__":
    sys.exit(main())
