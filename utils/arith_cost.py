#!/usr/bin/env python3
"""Where an arithmetic program's time goes: the tables, the parse, the calculation (M1f S1f.6.15).

    utils/bench_env.sh python3 utils/arith_cost.py            # every table, as Markdown
    utils/bench_env.sh python3 utils/arith_cost.py --quick    # the shipped examples and the split alone
    python3 utils/arith_cost.py --only digits --ops mul div   # one sweep, some operations

The user, 2026-11-11: *"add examples with long numbers and many calculations so we can compare load time
with calculation time"*. `ein saturate FILE` prints three times of its own, and this script reads them:

    parse:      the program's own text, lexed and parsed
    kb load:    macros, imports and facts: for a program that imports `std.arith`, mostly the 1 664 rows
                of `std.digits`, parsed and stored, whatever the program asks
    saturate:   the calculation, with its firings

The tables it prints, each a section:

    split      what a program pays before its first request: no import, the tables alone, the theory
    examples   the three generated examples of `examples/arith/` (`utils/gen_arith_examples.py`), as
               text and saved as a `.einb` (`ein kb save`), under `ein saturate`, `ein solve` and `ein test`
    digits     one request an operation, by the length of its operands
    requests   many requests an operation on nine-digit operands, by their count
    chain      the factorial chain of `factorial.ein`, by its last number: nothing written, all derived
    solve      `ein solve` beside `ein saturate` on the same programs: the read-out writes the state

Every program past the shipped examples is written here, under `build/arith_cost/`, by the generator's own
functions with its seed; none is a corpus entry. Each time is the best of `--best` runs (5), a run a process;
*wall* is the whole process, and *rss* its peak resident memory (`wait4`). Numbers taken on an idle machine
through `utils/bench_env.sh`, which prints the machine they were taken on.

The engine is `$EIN_BIN`, or `ein.rs/target/release/ein`. No dependency beyond the standard library.
"""
from __future__ import annotations

import argparse
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
    return f"{gen.IMPORT}({op} {gen.enc(x)} {gen.enc(y)})\n"


def batch(op: str, n: int, seed: int) -> str:
    """`n` requests of `op` on two numbers of exactly nine digits."""
    import random
    rnd = random.Random(seed)
    return gen.IMPORT + "".join(f"({op} {gen.enc(gen.draw(rnd, 9))} {gen.enc(gen.draw(rnd, 9))})\n"
                                for _ in range(n))


def section_split(best: int) -> None:
    print("## What a program pays before its first request\n")
    progs = [
        ("no import: one relation", '(relation thing Thing :why "{?1} is a thing")\n'),
        ("`std.digits` alone, the seven tables", "(import std.digits :symbols (fa fs md cl nz mid adj))\n"),
        ("`std.arith`, the theory and the tables", gen.IMPORT),
        ("`std.arith` and 17 + 5", f"{gen.IMPORT}(add {gen.enc(17)} {gen.enc(5)})\n"),
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


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--quick", action="store_true", help="the split and the examples alone")
    ap.add_argument("--only", nargs="+", choices=("split", "examples", "digits", "requests", "chain", "solve"))
    ap.add_argument("--ops", nargs="+", choices=gen.OPS, default=list(gen.OPS))
    ap.add_argument("--best", type=int, default=5)
    ap.add_argument("--seed", type=int, default=gen.SEED)
    args = ap.parse_args()
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
