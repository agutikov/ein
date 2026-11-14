#!/usr/bin/env python3
"""Where an arithmetic program's time goes: the tables, the parse, the calculation (M1f S1f.6.15).

    utils/bench_env.sh python3 utils/arith_cost.py            # every table, as Markdown
    utils/bench_env.sh python3 utils/arith_cost.py --quick    # the shipped examples and the split alone
    python3 utils/arith_cost.py --only digits --ops mul div   # one sweep, some operations

The user, 2026-11-11: *"add examples with long numbers and many calculations so we can compare load time
with calculation time"*. `ein saturate FILE` prints three times of its own, and this script reads them:

    parse:      the program's own text, lexed and parsed
    kb load:    macros, imports and facts: for a program that imports `std.math.decimal-arith`, mostly the 1 664 rows
                of `std.math.digits`, parsed and stored, whatever the program asks
    saturate:   the calculation, with its firings

The tables it prints, each a section:

    split      what a program pays before its first request: no import, the tables alone, the theory
    examples   the three generated examples of `examples/arith/` (`utils/gen_arith_examples.py`), as
               text and saved as a `.einb` (`ein kb save`), under `ein saturate`, `ein solve` and `ein test`
    digits     one request an operation, by the length of its operands
    requests   many requests an operation on nine-digit operands, by their count
    chain      the factorial chain of `factorial.ein`, by its last number: nothing written, all derived
    solve      `ein solve` beside `ein saturate` on the same programs: the read-out writes the state

`--biggest` is another mode (M1f S1f.6.29), run by hand and by nothing else: for each of `ein saturate` and
`ein solve`, and for each shape, the largest size whose run ends within a memory cap, about 16 GB by the user's
reply of 2026-11-12 (*"pick the one that fits into memory, say ~16GB; do not call it in tests and in CI, just
for manual solve call"*), and within a time limit:

    utils/bench_env.sh python3 utils/arith_cost.py --biggest                          # every command and shape
    utils/bench_env.sh python3 utils/arith_cost.py --biggest --commands solve --shapes chain

    product    one request, `mul`, two operands of N digits; the parser bounds N at 999
    quotient   one request, `div`, a dividend of 2N digits by a divisor of N; the parser bounds N at 499
    requests   N requests of nine-digit numbers, the five operations in turn (`--op mix`)
    chain      the factorial chain to N!

Each program is written by `gen_arith_examples.py`'s `on_demand`, the function behind its `--op`, under
`--work`. **Every run is held in a cgroup of its own** (`systemd-run --user --scope -p MemoryMax=… -p
MemorySwapMax=0`), so a run that would pass the cap is killed by the kernel inside its scope and reported
as such, and its `oom_score_adj` is 1000, so that if the host ran out of memory first the kernel would end
it before anything of the user's. A run past `--time-limit` is killed and reported. **No run starts
where the host has less than the cap and `--keep-free` (4 GB) available**: the sweep stops there, prints
what it found, and exits 3. The search doubles a
size until a run fails or the parser's bound is reached, then narrows between the last size that ended and
the first that did not, to `--resolution`. *rss* is the run's peak resident memory (`wait4`), and every
run is one process, measured once. `free`'s *available* is printed before each run.

Every program past the shipped examples is written here, under `build/arith_cost/`, by the generator's own
functions with its seed; none is a corpus entry. Each time is the best of `--best` runs (5), a run a process;
*wall* is the whole process, and *rss* its peak resident memory (`wait4`). Numbers taken on an idle machine
through `utils/bench_env.sh`, which prints the machine they were taken on.

The engine is `$EIN_BIN`, or `ein.rs/target/release/ein`. No dependency beyond the standard library.
"""
from __future__ import annotations

import argparse
import math
import os
import re
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path(__file__).resolve().parent))
import gen_arith_examples as gen  # noqa: E402

EIN = os.environ.get("EIN_BIN", str(REPO / "ein.rs" / "target" / "release" / "ein"))
WORK = REPO / "build" / "arith_cost"
LIMIT_S = 120.0     # a run past this is reported as such and not waited for


def run(args: list[str]) -> tuple[float, str, int] | None:
    """One process: wall ms, stdout, exit code; `None` past the limit."""
    start = time.perf_counter()
    proc = subprocess.Popen([EIN, *args], stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, cwd=REPO)
    try:
        out, _ = proc.communicate(timeout=LIMIT_S)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.communicate()
        return None
    wall = (time.perf_counter() - start) * 1000
    return wall, out.decode(errors="replace"), proc.returncode


def rss_kb(args: list[str]) -> int:
    """Peak resident memory of one run of its own, by `wait4`: the child's, not the largest child's so far."""
    proc = subprocess.Popen([EIN, *args], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, cwd=REPO)
    _, _, usage = os.wait4(proc.pid, 0)
    proc.returncode = 0
    return usage.ru_maxrss


def number(rx: str, text: str) -> float | None:
    m = re.search(rx, text, re.M)
    return float(m.group(1)) if m else None


class Row(dict):
    pass


def saturate(path: Path, best: int) -> Row:
    """`ein saturate PATH`, best of `best`: its own three times, firings, facts, wall, rss."""
    row = Row(parse=None, load=None, sat=None, firings=None, facts=None, wall=None, open=None)
    for _ in range(best):
        got = run(["saturate", str(path)])
        if got is None:
            row["wall"] = None
            return row
        wall, out, code = got
        assert code == 0, f"ein saturate {path}: exit {code}"
        for key, rx in (("parse", r"^parse:\s+([0-9.]+) ms"), ("load", r"^kb load:\s+([0-9.]+) ms"),
                        ("open", r"^open:\s+([0-9.]+) ms"), ("sat", r"^saturate:\s+([0-9.]+) ms")):
            v = number(rx, out)
            if v is not None:
                row[key] = v if row[key] is None else min(row[key], v)
        row["wall"] = wall if row["wall"] is None else min(row["wall"], wall)
        row["firings"] = int(number(r"^saturate:\s+[0-9.]+ ms\s+\((\d+) firings", out))
        row["facts"] = int(number(r"^\s+facts \(total\)\s+\d+\s+(\d+)", out))
    row["rss"] = rss_kb(["saturate", str(path)])
    return row


def wall_of(args: list[str], best: int) -> float | None:
    """The whole process, best of `best`; `None` past the limit. One run where a run is long."""
    took = None
    for i in range(best):
        got = run(args)
        if got is None:
            return None
        assert got[2] == 0, f"ein {' '.join(args)}: exit {got[2]}"
        took = got[0] if took is None else min(took, got[0])
        if took > 5000:
            break
    return took


def ms(v: float | None) -> str:
    if v is None:
        return "—"
    return f"{v:.2f}" if v < 10 else f"{v:.1f}" if v < 100 else f"{v:,.0f}".replace(",", " ")


def count(v: int | None) -> str:
    return "—" if v is None else f"{v:,}".replace(",", " ")


def size(path: Path) -> str:
    b = path.stat().st_size
    return f"{b} B" if b < 1024 else f"{b / 1024:.0f} KB" if b < 1 << 20 else f"{b / (1 << 20):.1f} MB"


def table(head: list[str], rows: list[list[str]]) -> None:
    print("| " + " | ".join(head) + " |")
    print("|" + "|".join("---" if i == 0 else "---:" for i in range(len(head))) + "|")
    for r in rows:
        print("| " + " | ".join(r) + " |")
    print(flush=True)


def write(name: str, text: str) -> Path:
    WORK.mkdir(parents=True, exist_ok=True)
    path = WORK / name
    path.write_text(text)
    return path


SAT_HEAD = ["text", "parse ms", "load ms", "saturate ms", "firings", "facts", "rss MB", "wall ms"]


def sat_cells(path: Path, r: Row) -> list[str]:
    return [size(path), ms(r["parse"]), ms(r["load"]), ms(r["sat"]), count(r["firings"]), count(r["facts"]),
            f"{r['rss'] / 1024:.0f}", ms(r["wall"])]


def one_request(op: str, digits: int, seed: int) -> str:
    """One request of `op` on operands of `digits` digits; a dividend has twice as many."""
    import random
    rnd = random.Random(seed * 1000 + digits)
    x = gen.draw(rnd, 2 * digits if op == "div" else digits)
    y = gen.draw(rnd, digits)
    body = f"({op} {gen.enc(x)} {gen.enc(y)})\n"
    return gen.import_line(body) + body


def batch(op: str, n: int, seed: int) -> str:
    """`n` requests of `op` on two numbers of exactly nine digits."""
    import random
    rnd = random.Random(seed)
    body = "".join(f"({op} {gen.enc(gen.draw(rnd, 9))} {gen.enc(gen.draw(rnd, 9))})\n" for _ in range(n))
    return gen.import_line(body) + body


def section_split(best: int) -> None:
    print("## What a program pays before its first request\n")
    progs = [
        ("no import: one relation", '(relation thing Thing :why "{?1} is a thing")\n'),
        ("`std.math.digits` alone, the seven tables", "(import std.math.digits :symbols (fa fs md cl nz mid adj))\n"),
        ("`std.math.decimal-arith`, the theory and the tables", gen.import_line("")),
        ("`std.math.decimal-arith` and 17 + 5",
         gen.import_line(f"(add {gen.enc(17)} {gen.enc(5)})\n") + f"(add {gen.enc(17)} {gen.enc(5)})\n"),
    ]
    rows = []
    for i, (what, text) in enumerate(progs):
        path = write(f"split{i}.ein", text)
        r = saturate(path, best)
        rows.append([what, *sat_cells(path, r)])
    table(["program", *SAT_HEAD], rows)


def einb_of(path: Path) -> Path:
    out = WORK / (path.stem + ".einb")
    WORK.mkdir(parents=True, exist_ok=True)
    subprocess.run([EIN, "kb", "save", str(path), str(out)], check=True, cwd=REPO,
                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    return out


def section_examples(best: int) -> None:
    print("## The three examples\n")
    names = ("add.ein", "long_numbers.ein", "many_calculations.ein", "factorial.ein")
    rows, rows_b, rows_c = [], [], []
    for name in names:
        path = REPO / "examples" / "arith" / name
        r = saturate(path, best)
        rows.append([f"`{name}`", *sat_cells(path, r)])
        einb = einb_of(path)
        b = saturate(einb, best)
        rows_b.append([f"`{name}`", size(einb), ms(b["open"]), ms(b["sat"]), count(b["firings"]), ms(b["wall"]),
                       ms(r["wall"])])
        rows_c.append([f"`{name}`", ms(r["wall"]), ms(wall_of(["solve", str(path)], best)),
                       ms(wall_of(["test", str(path)], best)), ms(b["wall"]),
                       ms(wall_of(["solve", str(einb)], best))])
    print("`ein saturate FILE`, the program as text (`add.ein`, one sum and one by names, for scale):\n")
    table(["example", *SAT_HEAD], rows)
    print("The same programs saved by `ein kb save FILE OUT.einb`, under `ein saturate OUT.einb`, which opens "
          "the container where the text is parsed and loaded:\n")
    table(["example", ".einb", "open ms", "saturate ms", "firings", "wall ms", "wall ms as text"], rows_b)
    print("The whole process, by command:\n")
    table(["example", "saturate", "solve", "test", "saturate .einb", "solve .einb"], rows_c)


def section_digits(best: int, ops: list[str], seed: int) -> None:
    print("## One request, by the length of its operands\n")
    print("A dividend has twice the digits of its divisor. A written number nests a level a digit, and the "
          "parser nests to 1 000, so 900 digits is near the longest number a program can write.\n")
    rows = []
    for op in ops:
        for d in ((9, 30, 100, 300, 900) if op in ("add", "sub", "cmp") else (9, 30, 100, 300)):
            path = write(f"digits-{op}-{d}.ein", one_request(op, d, seed))
            r = saturate(path, 3 if d >= 300 else best)
            what = f"{op}, {2 * d} by {d}" if op == "div" else f"{op}, {d} digits"
            rows.append([what, *sat_cells(path, r)])
    table(["request", *SAT_HEAD], rows)


def section_requests(best: int, ops: list[str], seed: int) -> None:
    print("## Many requests on nine-digit operands, by their count\n")
    rows = []
    for op in ops:
        # A product or a quotient of nine digits is some 500 firings and as many stored facts, and
        # 10 000 of them hold 6 GB (2026-11-11), so those two stop at 4 000.
        for n in ((100, 1000, 10000) if op in ("add", "sub", "cmp") else (100, 1000, 4000)):
            path = write(f"requests-{op}-{n}.ein", batch(op, n, seed))
            r = saturate(path, 3 if n >= 10000 else best)
            per = f"{r['firings'] / n:.1f}" if r["firings"] else "—"
            rows.append([f"{count(n)} x {op}", *sat_cells(path, r), per])
    table(["requests", *SAT_HEAD, "firings a request"], rows)


def section_chain(best: int) -> None:
    print("## The factorial chain, by its last number\n")
    rows = []
    for last in (gen.FACTORIAL, 50, 100, 200):
        path = write(f"chain-{last}.ein", gen.factorial(last))
        r = saturate(path, 3 if last >= 100 else best)
        import math
        rows.append([f"{last}!, {len(str(math.factorial(last)))} digits", *sat_cells(path, r)])
    table(["chain", *SAT_HEAD], rows)


def section_solve(best: int, seed: int) -> None:
    print("## `ein solve` beside `ein saturate`\n")
    print("The same program, the whole process. `ein solve` saturates once, as `ein saturate` does, and then "
          f"reads the state out. A run past {LIMIT_S:.0f} s is not waited for.\n")
    progs = [(f"`examples/arith/{n}`", REPO / "examples" / "arith" / n)
             for n in ("add.ein", "long_numbers.ein", "many_calculations.ein", "factorial.ein")]
    progs += [(f"add, {d} digits", write(f"digits-add-{d}.ein", one_request("add", d, seed))) for d in (100, 300, 900)]
    progs += [(f"mul, {d} digits", write(f"digits-mul-{d}.ein", one_request("mul", d, seed))) for d in (30, 100)]
    progs += [(f"div, {2 * d} by {d}", write(f"digits-div-{d}.ein", one_request("div", d, seed))) for d in (30, 100)]
    progs += [(f"{count(n)} x add, nine digits", write(f"requests-add-{n}.ein", batch("add", n, seed))) for n in (1000, 10000)]
    progs += [(f"{count(n)} x div, nine digits", write(f"requests-div-{n}.ein", batch("div", n, seed))) for n in (100, 1000)]
    progs += [(f"{k}!", write(f"chain-{k}.ein", gen.factorial(k))) for k in (50, 100)]
    rows = []
    for what, path in progs:
        r = saturate(path, 3)
        s = wall_of(["solve", str(path)], 3)
        ratio = "—" if s is None or not r["wall"] else f"{s / r['wall']:.1f}"
        rows.append([what, count(r["firings"]), count(r["facts"]), ms(r["wall"]),
                     ms(s) if s is not None else f"past {LIMIT_S:.0f} s", ratio])
    table(["program", "firings", "facts", "saturate wall ms", "solve wall ms", "solve / saturate"], rows)


# ── The biggest calculation (S1f.6.29) ─────────────────────────────────────

MEM_CAP = 16_000_000_000       # bytes: "say ~16GB", the user's reply of 2026-11-12
TIME_LIMIT = 600.0             # s: a run past this is killed and reported
WINDOW = (50.0, 150.0)         # s: the user's first reply, "100+-50 seconds on this host"

# shape: (the generator's arguments for a size, the first size, the parser's bound on the size)
SHAPES = {
    "product": (lambda n: dict(op="mul", digits=n, dividend=2 * n, count=1, n=None), 100, 999),
    "quotient": (lambda n: dict(op="div", digits=n, dividend=2 * n, count=1, n=None), 100, 499),
    "requests": (lambda n: dict(op="mix", digits=9, dividend=18, count=n, n=None), 2000, None),
    "chain": (lambda n: dict(op="factorial", digits=None, dividend=None, count=1, n=n), 100, None),
}


def available() -> int:
    """`MemAvailable`, in bytes."""
    for line in Path("/proc/meminfo").read_text().splitlines():
        if line.startswith("MemAvailable:"):
            return int(line.split()[1]) * 1024
    return 0


class Short(Exception):
    """The host has less memory available than a run may take and the margin it must leave."""


KEEP_FREE = 4_000_000_000      # bytes the host keeps beside a run at the cap


def capped(args: list[str], out: Path, mem: int, limit: float, keep_free: int = KEEP_FREE) -> dict:
    """One `ein` process in a cgroup of its own, capped at `mem` bytes and `limit` seconds; not started where
    the host has less than `mem + keep_free` available, so a run at its cap never takes the host to its own
    limit (2026-11-13: a run at the cap with 1.4 GB left beside it had the session's background command reaped)."""
    def adjust() -> None:
        with open("/proc/self/oom_score_adj", "w") as f:
            f.write("1000")
    cmd = ["systemd-run", "--user", "--scope", "--quiet", "-p", f"MemoryMax={mem}", "-p", "MemorySwapMax=0",
           "-p", "OOMPolicy=continue", "--", EIN, *args]
    avail = available()
    if avail < mem + keep_free:
        raise Short(f"`MemAvailable` is {avail / 1e9:.1f} GB, under the cap and the margin, "
                    f"{(mem + keep_free) / 1e9:.1f} GB: `ein {' '.join(args)}` not started")
    with open(out, "wb") as so, open(out.with_suffix(".err"), "wb") as se:
        start = time.perf_counter()
        proc = subprocess.Popen(cmd, stdout=so, stderr=se, cwd=REPO, preexec_fn=adjust)
        killed = False
        while True:
            pid, status, usage = os.wait4(proc.pid, os.WNOHANG)
            if pid:
                break
            if time.perf_counter() - start > limit:
                proc.kill()
                killed = True
                pid, status, usage = os.wait4(proc.pid, 0)
                break
            time.sleep(0.05)
        wall = time.perf_counter() - start
    proc.returncode = 0  # reaped by wait4
    if killed:
        # a process held at its cgroup's cap without swap is not killed at once: the kernel evicts its
        # code pages and it crawls, so a run at the cap when the limit passed is the cap's
        how = "memory" if usage.ru_maxrss * 1024 >= 0.95 * mem else "time"
    elif os.WIFSIGNALED(status):
        how = "memory" if os.WTERMSIG(status) == 9 else f"signal {os.WTERMSIG(status)}"
    else:
        how = "ok" if os.WEXITSTATUS(status) == 0 else f"exit {os.WEXITSTATUS(status)}"
    return dict(how=how, wall=wall, rss=usage.ru_maxrss * 1024, available=avail)


def counts(out: Path) -> tuple[int | None, int | None]:
    """Firings and stored facts, as `ein saturate` prints them."""
    text = out.read_text(errors="replace")
    f = number(r"^saturate:\s+[0-9.]+ ms\s+\((\d+) firings", text)
    n = number(r"^\s+facts \(total\)\s+\d+\s+(\d+)", text)
    return (int(f) if f is not None else None, int(n) if n is not None else None)


def expected(kw: dict, seed: int) -> list[dict[str, str]]:
    """Python's answer to each query `on_demand` writes, in its order, as `ein solve` prints the bindings."""
    def num(v: int) -> str:
        return str(v)
    if kw["op"] == "factorial":
        return [{"?f": num(math.factorial(kw["n"]))}]
    _, batches = gen.operands(kw["op"], kw["digits"], kw["dividend"], kw["count"], seed)
    out = []
    for o in gen.OPS:
        if o not in batches:
            continue
        x, y = batches[o][0]
        out.append({"add": {"?s": num(x + y)},
                    "sub": {"?d": num(max(x - y, 0)), "?r": num(max(y - x, 0))},
                    "cmp": {"?o": "lt" if x < y else "gt" if x > y else "eq"},
                    "mul": {"?p": num(x * y)},
                    "div": {"?q": num(x // y), "?r": num(x % y)}}[o])
    return out


def printed(text: str) -> list[dict[str, str]]:
    """Each query's bindings as `ein solve` prints them, a number read back from `std.math.decimal-arith`'s spelling."""
    def value(v: str) -> str:
        v = v.strip()
        if v == "z":
            return "0"
        if v.startswith("(n "):
            return str(int("".join(re.findall(r" (\d)\)", v))))
        return v
    found = []
    for m in re.finditer(r"^\s+class 1 of 1\s+.*\n\s+(\?.*)$", text, re.M):
        found.append({k: value(v) for k, v in (b.split(" = ", 1) for b in m.group(1).split(" · "))})
    return found


def biggest_run(command: str, shape: str, size: int, work: Path, mem: int, limit: float, seed: int,
                log: list, keep_free: int = KEEP_FREE) -> dict:
    """Write the program of `shape` at `size`, run `ein COMMAND` on it under the cap, and log the run."""
    kw = SHAPES[shape][0](size)
    path = work / f"{shape}-{size}.ein"
    if not path.exists():
        path.write_text(gen.on_demand(seed=seed, expect=True, **kw))
    out = work / f"{shape}-{size}.{command}.out"
    r = capped([command, str(path)], out, mem, limit, keep_free)
    r.update(command=command, shape=shape, size=size, path=str(path.relative_to(REPO)),
             bytes=path.stat().st_size, date=time.strftime("%Y-%m-%d"),
             generate=gen.command(seed=seed, expect=True, **kw).replace("-o FILE", f"-o {path.relative_to(REPO)}"))
    if command == "saturate" and r["how"] == "ok":
        r["firings"], r["facts"] = counts(out)
    if command == "solve" and r["how"] == "ok":
        text = out.read_text(errors="replace")
        r["claims_held"] = text.count(":expect           holds")
        r["claims_failed"] = len(re.findall(r"^  :expect\s+(?!holds)", text, re.M))
        r["answers_right"] = printed(text) == expected(kw, seed)
        if not r["answers_right"]:
            print(f"  !! {path}: an answer `ein solve` printed is not Python's", file=sys.stderr, flush=True)
    log.append(r)
    print(f"  {command} {shape} {size}: {r['how']}, {r['wall']:.1f} s, rss {r['rss'] / 1e9:.2f} GB, "
          f"available {r['available'] / 1e9:.1f} GB", file=sys.stderr, flush=True)
    return r


def next_size(oks: list[dict], bad: dict | None, bound: int | None, mem: int, limit: float,
              resolution: float) -> int:
    """The next size to try: doubled while nothing failed, then narrowed, each step aimed by the power law
    the two largest sizes that ended draw for the measure that binds (rss for the cap, wall for the limit)."""
    best = oks[-1]
    floor = best["size"] + max(1, int(best["size"] * resolution))
    def aimed(measure: str, target: float) -> int | None:
        if len(oks) < 2:
            return None
        a, b = oks[-2], oks[-1]
        if a[measure] <= 0 or b[measure] <= a[measure]:
            return None
        k = math.log(b[measure] / a[measure]) / math.log(b["size"] / a["size"])
        return int(b["size"] * (target / b[measure]) ** (1 / k)) if k > 0.1 else None
    at_cap, at_limit = aimed("rss", 0.95 * mem), aimed("wall", 0.9 * limit)
    if bad is None:
        size = 2 * best["size"]
        for guess in (at_cap, at_limit):
            if guess is not None:
                size = min(size, max(guess, floor))
        return min(size, bound) if bound is not None else size
    guess = at_cap if bad["how"] == "memory" else at_limit
    ceiling = bad["size"] - max(1, int(bad["size"] * resolution / (1 + resolution)))
    if guess is None:
        guess = int(round((best["size"] * bad["size"]) ** 0.5))
    return max(floor, min(guess, ceiling)) if floor < bad["size"] else best["size"] + 1


def biggest_search(command: str, shape: str, work: Path, mem: int, limit: float, seed: int, resolution: float,
                   log: list, keep_free: int = KEEP_FREE) -> dict:
    """The largest size of `shape` whose `ein COMMAND` ends within the cap and the limit."""
    _, first, bound = SHAPES[shape]
    oks: list[dict] = []
    bad: dict | None = None
    size = first
    while True:
        r = biggest_run(command, shape, size, work, mem, limit, seed, log, keep_free)
        if r["how"] == "ok":
            oks.append(r)
            oks.sort(key=lambda o: o["size"])
        elif bad is None or r["size"] < bad["size"]:
            bad = r
        if not oks:
            return dict(command=command, shape=shape, best=None, next=bad, bound="none ended")
        best = oks[-1]
        if bad is None and bound is not None and best["size"] >= bound:
            return dict(command=command, shape=shape, best=best, next=None, bound="the parser")
        if bad is not None and bad["size"] - best["size"] <= max(1, int(best["size"] * resolution)):
            return dict(command=command, shape=shape, best=best, next=bad, bound=bad["how"])
        size = next_size(oks, bad, bound, mem, limit, resolution)


def section_biggest(args: argparse.Namespace) -> None:
    import json
    work = args.work if args.work.is_absolute() else REPO / args.work
    work.mkdir(parents=True, exist_ok=True)
    print("## The biggest calculation\n")
    print(f"Every run in a cgroup of its own, `MemoryMax={args.mem}` ({args.mem / 1e9:.1f} GB, "
          f"{args.mem / 2**30:.1f} GiB) and no swap, and killed past {args.time_limit:.0f} s; the size doubled, "
          f"then narrowed to {args.resolution:.0%}; no run started with less than "
          f"{(args.mem + args.keep_free) / 1e9:.1f} GB available. *available* is `MemAvailable` before the first "
          f"run: {available() / 1e9:.1f} GB.\n", flush=True)
    log: list = []
    found = []
    record = work / f"biggest-{time.strftime('%Y%m%d-%H%M%S')}.json"
    short = None
    for command in args.commands:
        for shape in args.shapes:
            print(f"{command} {shape}:", file=sys.stderr, flush=True)
            try:
                found.append(biggest_search(command, shape, work, args.mem, args.time_limit, args.seed,
                                            args.resolution, log, args.keep_free))
            except Short as e:
                short = f"{command} {shape}: {e}"
                print(f"  !! {short}; the sweep stops here", file=sys.stderr, flush=True)
                break
            record.write_text(json.dumps(dict(found=found, runs=log), indent=1))
        if short:
            break
    rows = []
    for f in found:
        b, n = f["best"], f["next"]
        if b is None:
            rows.append([f"`ein {f['command']}`", f["shape"], "—", "—", "—", "—", "—", "—", "—", f["bound"]])
            continue
        if f["command"] == "solve" and "facts" not in b:
            # the same program's saturation, for its counts: `ein solve` prints neither
            out = work / f"{f['shape']}-{b['size']}.saturate.out"
            try:
                sat = capped(["saturate", b["path"]], out, args.mem, args.time_limit, args.keep_free)
            except Short as e:
                print(f"  !! {e}", file=sys.stderr, flush=True)
                sat = dict(how="short")
            if sat["how"] == "ok":
                b["firings"], b["facts"] = counts(out)
        inside = "yes" if WINDOW[0] <= b["wall"] <= WINDOW[1] else "no"
        what = {"memory": "killed at the cap", "time": "killed at the limit"}
        stop = ("the parser's limit" if f["bound"] == "the parser" else
                f"{f['bound']}: {count(n['size'])} {what.get(n['how'], n['how'])}, {n['rss'] / 1e9:.1f} GB at "
                f"{n['wall']:.0f} s")
        answers = ("right" if b.get("answers_right") else "**wrong**") if f["command"] == "solve" else "not printed"
        rows.append([f"`ein {f['command']}`", f["shape"], count(b["size"]), f"{b['wall']:.1f}",
                     f"{b['rss'] / 1e9:.2f}", count(b.get("firings")), count(b.get("facts")), inside, answers, stop])
    table(["command", "shape", "size", "wall s", "peak RSS GB", "firings", "facts", "in 100 ± 50 s",
           "answers", "what stopped the next size"], rows)
    print("The commands, each program written by the first and run by the second:\n")
    for f in found:
        if f["best"] is not None:
            print(f"    {f['best']['generate']}\n    ein {f['command']} {f['best']['path']}\n")
    print("Every run, in the order taken:\n")
    table(["command", "shape", "size", "text", "ended", "wall s", "peak RSS GB", "available GB"],
          [[f"`ein {r['command']}`", r["shape"], count(r["size"]), size(REPO / r["path"]), r["how"],
            f"{r['wall']:.1f}", f"{r['rss'] / 1e9:.2f}", f"{r['available'] / 1e9:.1f}"] for r in log])
    if short:
        print(f"**Stopped:** {short}.\n")
    record.write_text(json.dumps(dict(found=found, runs=log, stopped=short), indent=1))
    if short:
        sys.exit(3)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--quick", action="store_true", help="the split and the examples alone")
    ap.add_argument("--only", nargs="+", choices=("split", "examples", "digits", "requests", "chain", "solve"))
    ap.add_argument("--ops", nargs="+", choices=gen.OPS, default=list(gen.OPS))
    ap.add_argument("--best", type=int, default=5)
    ap.add_argument("--seed", type=int, default=gen.SEED)
    big = ap.add_argument_group("the biggest calculation (S1f.6.29): run by hand, by no test")
    big.add_argument("--biggest", action="store_true", help="the size search, and nothing else")
    big.add_argument("--commands", nargs="+", choices=("saturate", "solve"), default=["saturate", "solve"])
    big.add_argument("--shapes", nargs="+", choices=tuple(SHAPES), default=list(SHAPES))
    big.add_argument("--mem", type=int, default=MEM_CAP, help="the cap, in bytes (default 16 000 000 000)")
    big.add_argument("--time-limit", type=float, default=TIME_LIMIT, help="seconds a run may take")
    big.add_argument("--resolution", type=float, default=0.03, help="how close the search narrows")
    big.add_argument("--keep-free", type=int, default=KEEP_FREE,
                     help="bytes the host must have available beyond the cap before a run starts (default 4 GB)")
    big.add_argument("--work", type=Path, default=WORK / "biggest", help="where the programs are written")
    args = ap.parse_args()
    if args.biggest:
        lines = subprocess.run([EIN, "--version"], capture_output=True, text=True, cwd=REPO).stdout.splitlines()
        stdlib = next((l.split(None, 1)[1] for l in lines if l.startswith("stdlib")), "?")
        print(f"`{lines[0]}` (`{EIN}`), stdlib `{stdlib}`, seed {args.seed}; "
              f"`python3 utils/arith_cost.py {' '.join(sys.argv[1:])}`\n")
        section_biggest(args)
        return 0
    which = args.only or (["split", "examples"] if args.quick else
                          ["split", "examples", "digits", "requests", "chain", "solve"])
    version = subprocess.run([EIN, "--version"], capture_output=True, text=True, cwd=REPO).stdout.splitlines()[0]
    print(f"`{version}`, best of {args.best}, seed {args.seed}; `python3 utils/arith_cost.py"
          f"{' ' + ' '.join(sys.argv[1:]) if sys.argv[1:] else ''}`\n")
    if "split" in which:
        section_split(args.best)
    if "examples" in which:
        section_examples(args.best)
    if "digits" in which:
        section_digits(args.best, args.ops, args.seed)
    if "requests" in which:
        section_requests(args.best, args.ops, args.seed)
    if "chain" in which:
        section_chain(args.best)
    if "solve" in which:
        section_solve(args.best, args.seed)
    return 0


if __name__ == "__main__":
    sys.exit(main())
