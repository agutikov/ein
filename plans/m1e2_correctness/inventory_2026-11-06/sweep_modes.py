#!/usr/bin/env python3
"""M1e2 inventory re-take (2026-11-06), added to the 11-05 set: every mode of
`ein solve` against the oracle, on every program both describe and `ein
oracle` finish (corpus_describe.json, region_census.json). For each:

    ein solve FILE --minimal --json-summary J   the states listed on the minimal line
    ein solve FILE --maximal --json-summary J   the states listed on the maximal line
    ein solve FILE --count   --json-summary J   k_s, k_c
    ein solve FILE --single  --json-summary J   the word, against k_s

compared with the oracle's line sizes and counts as `ein oracle FILE --dump D`
writes them (region.json's `minimal` and `maximal`, one entry per state; k_s,
k_c), and the --single word with the one k_s implies (0 → Contradiction or
Open, 1 → Solution, more → Ambiguity). One engine process at a time. Writes
corpus_modes.json beside this script; prints every disagreement.

    EIN_STDLIB=src/stdlib nice -n 19 python3 sweep_modes.py
"""
from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
SRC = HERE / "src"
BIN = str(HERE / "target/release/ein")
SCR = HERE / "scratch" / "modes"


def run(argv, env):
    j = SCR / "s.json"
    if j.exists():
        j.unlink()
    p = subprocess.run([BIN, *argv, "--json-summary", str(j)], cwd=SRC, capture_output=True,
                       text=True, timeout=900, env=env)
    return p.returncode, (json.loads(j.read_text()) if j.exists() else None)


def oracle_lines(path, env):
    d = SCR / "oracle"
    if d.exists():
        shutil.rmtree(d)
    subprocess.run([BIN, "oracle", path, "--dump", str(d)], cwd=SRC, capture_output=True,
                   text=True, timeout=900, env=env)
    reg = json.loads((d / "region.json").read_text())["region"]
    nodes = reg["nodes"]
    # a line's members as states: the classes of its commitments
    cands = json.loads((d / "region.json").read_text())["candidates"]

    def cls(c):
        m = sum(1 << cands.index(f) for f in c)
        return nodes[m]["class"]

    return ({cls(c) for c in reg["minimal"]}, {cls(c) for c in reg["maximal"]})


def main() -> int:
    env = dict(os.environ)
    env.setdefault("EIN_STDLIB", str(SRC / "stdlib"))
    env["TMPDIR"] = str(SCR)
    SCR.mkdir(parents=True, exist_ok=True)
    desc = {r["path"]: r for r in json.loads((HERE / "corpus_describe.json").read_text())["rows"]}
    cen = {r["path"]: r for r in json.loads((HERE / "region_census.json").read_text())}
    todo = [p for p in sorted(desc) if desc[p].get("loaded") and desc[p].get("exhausted")
            and "k_s" in cen.get(p, {})]
    rows, bad = [], []
    t0 = time.time()
    for i, p in enumerate(todo):
        c = cen[p]
        omin, omax = oracle_lines(p, env)
        row = {"path": p, "oracle": {"k_s": c["k_s"], "k_c": c["k_c"],
                                     "minimal": len(omin), "maximal": len(omax)}}
        for mode in ("--minimal", "--maximal", "--count", "--single"):
            ex, j = run(["solve", p, mode], env)
            r = (j or {}).get("region") or {}
            states = [s for s in (r.get("states") or []) if s.get("listed", True)]
            row[mode] = {"exit": ex, "exhausted": r.get("exhausted"), "listed": len(states),
                         "k_s": r.get("k_s"), "k_c": r.get("k_c"),
                         "word": ((j or {}).get("verdict") or {}).get("word")}
        diffs = []
        if row["--minimal"]["listed"] != len(omin) or not row["--minimal"]["exhausted"]:
            diffs.append(("minimal", row["--minimal"]["listed"], len(omin)))
        if row["--maximal"]["listed"] != len(omax) or not row["--maximal"]["exhausted"]:
            diffs.append(("maximal", row["--maximal"]["listed"], len(omax)))
        if (row["--count"]["k_s"], row["--count"]["k_c"]) != (c["k_s"], c["k_c"]):
            diffs.append(("count", (row["--count"]["k_s"], row["--count"]["k_c"]), (c["k_s"], c["k_c"])))
        w = row["--single"]["word"]
        want = {0: ("Contradiction", "Open"), 1: ("Solution",)}.get(c["k_s"], ("Ambiguity",))
        if w not in want:
            diffs.append(("single", w, want))
        row["diffs"] = diffs
        rows.append(row)
        if diffs:
            bad.append((p, diffs))
        print(f"[{i+1}/{len(todo)}] {p} {'DIFF ' + str(diffs) if diffs else 'ok'}", file=sys.stderr, flush=True)
    (HERE / "corpus_modes.json").write_text(json.dumps(
        {"taken": time.strftime("%Y-%m-%d %H:%M:%S"),
         "commit": os.environ.get("EIN_INVENTORY_COMMIT", "c523f3ab"),
         "wall_total_s": round(time.time() - t0, 1), "programs": len(rows),
         "disagreements": len(bad), "rows": rows}, indent=1))
    print(f"{len(rows)} programs, {len(bad)} with a mode that disagrees with the oracle")
    for p, d in bad:
        print("  ", p, d)
    return 0


if __name__ == "__main__":
    sys.exit(main())
