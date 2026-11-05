#!/usr/bin/env python3
"""M1e2 inventory, section C: a bare `ein solve FILE --json-summary J` (describe,
default budget) on every corpus entry and every register witness, one process at
a time, under nice. Writes corpus_describe.json beside this script.

    EIN_STDLIB=src/stdlib nice -n 19 python3 sweep_describe.py [--bin B] [--timeout S] [-k KEY]

Read-only against the snapshot in ./src (commit cdcc3994). Every scratch file
goes under ./scratch/describe/.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import time
import tomllib
from pathlib import Path

HERE = Path(__file__).resolve().parent
SRC = HERE / "src"
SCRATCH = HERE / "scratch" / "describe"


def selection() -> list[dict]:
    entries = tomllib.loads((SRC / "corpus/corpus.toml").read_text())["entry"]
    out = {e["path"]: {"path": e["path"], "group": e["group"], "slow": bool(e.get("slow")),
                       "source": "corpus", "runs": e.get("runs", [])} for e in entries}
    reg = tomllib.loads((SRC / "corpus/wrong_answers.toml").read_text())

    def walk(node):
        if isinstance(node, dict):
            for k, v in node.items():
                if k == "witness" and isinstance(v, str):
                    yield v
                else:
                    yield from walk(v)
        elif isinstance(node, list):
            for v in node:
                yield from walk(v)

    for w in walk(reg["case"]):
        if w in out:
            out[w]["source"] = "corpus+register"
        else:
            out[w] = {"path": w, "group": "", "slow": False, "source": "register", "runs": []}
    return sorted(out.values(), key=lambda r: r["path"])


FORBID = re.compile(r"^rule '[^']*': concludes .* from \(absent")


def classify_stderr(err: str) -> str:
    first = err.strip().splitlines()[0] if err.strip() else ""
    if FORBID.search(first) or "forbidden since M1e2 S1e2.2.5" in err:
        return "refused-forbid"
    if "no fact can hold it" in first or "wildcard _ as its relation" in first:
        return "refused-load-pattern"
    return "load-error"


def summarise(j: dict) -> dict:
    r = j.get("region") or {}
    m = j.get("monotonicity") or {}
    hg = ((j.get("root") or {}).get("hypgen") or {})
    toa = hg.get("types_as_objects") or {}
    st = m.get("status") or {}
    state = m.get("state") or {}
    paths = m.get("paths") or {}
    sites = m.get("sites") or []
    holes = r.get("holes") or {}
    return {
        "k_s": r.get("k_s"), "k_c": r.get("k_c"), "exhausted": r.get("exhausted"),
        "stopped": r.get("stopped"), "complete_up_to": r.get("complete_up_to"),
        "root": r.get("root"), "root_owes": r.get("root_owes"), "empty": r.get("empty"),
        "nogoods": r.get("nogoods"), "discharging": r.get("discharging"),
        "holes": holes.get("count") if isinstance(holes, dict) else holes,
        "route": r.get("route"), "licences": r.get("licences"),
        "candidates": (r.get("universe") or {}).get("candidates"),
        "dead_alone": (r.get("universe") or {}).get("dead_alone"),
        "core": r.get("core"), "refuted_by": r.get("refuted_by"),
        "monotone": m.get("monotone"),
        "sites": [{"rule": s.get("rule"), "kind": s.get("kind"), "relation": s.get("relation"),
                   "concludes": s.get("concludes"), "at_stake": s.get("at_stake")} for s in sites],
        "cycles": m.get("cycles"), "order": m.get("order"),
        "status_read": st.get("read"), "status_bit": st.get("bit"), "status_by": st.get("by"),
        "status_holes": st.get("holes"),
        "dead_not_upward_closed": st.get("dead_not_upward_closed"),
        "discharged_not_upward_closed": st.get("discharged_not_upward_closed"),
        "stale_nodes": state.get("stale_nodes"), "state_nodes": state.get("nodes"),
        "state_read": state.get("read"),
        "paths_differ": len(paths.get("differ") or []) if paths else None,
        "paths_parents": paths.get("parents") if paths else None,
        "outside": hg.get("outside"),
        "types_as_objects": toa.get("objects") or [],
        "types_candidates": toa.get("candidates"),
        "claim": j.get("claim"),
        "work": j.get("work"),
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--bin", default=str(HERE / "target/release/ein"))
    ap.add_argument("--timeout", type=float, default=600.0)
    ap.add_argument("-k", "--key")
    ap.add_argument("--out", default=str(HERE / "corpus_describe.json"))
    a = ap.parse_args()
    SCRATCH.mkdir(parents=True, exist_ok=True)
    rows = selection()
    if a.key:
        rows = [r for r in rows if a.key in r["path"]]
    env = dict(os.environ)
    env.setdefault("EIN_STDLIB", str(SRC / "stdlib"))
    env["TMPDIR"] = str(SCRATCH)
    t0 = time.time()
    for i, row in enumerate(rows):
        js = SCRATCH / "summary.json"
        if js.exists():
            js.unlink()
        argv = [a.bin, "solve", row["path"], "--json-summary", str(js)]
        t = time.perf_counter()
        try:
            p = subprocess.run(argv, cwd=SRC, capture_output=True, text=True,
                               timeout=a.timeout, env=env)
            row["exit"] = p.returncode
            out, err = p.stdout, p.stderr
        except subprocess.TimeoutExpired:
            row["exit"] = None
            out, err = "", f"timeout at {a.timeout:g} s"
        row["wall_s"] = round(time.perf_counter() - t, 3)
        row["stderr"] = err.strip().splitlines()[:3]
        # stdout rows worth keeping verbatim
        keep = []
        for line in out.splitlines():
            s = line.strip()
            if re.match(r"^(states \(k_s\)|commitments \(k_c\)|exhausted|lattice|status|state |paths|outside|types|note:|empty|N |universe|route|assumes|cycle|order|minimal hole|claims?|stopped)", s):
                keep.append(s)
        row["stdout_rows"] = keep
        if js.exists():
            try:
                j = json.loads(js.read_text())
                row["loaded"] = True
                row.update(summarise(j))
            except Exception as e:  # noqa: BLE001
                row["loaded"] = None
                row["summary_error"] = repr(e)
        else:
            row["loaded"] = False
            row["refusal"] = classify_stderr(err) if row["exit"] is not None else "timeout"
        print(f"[{i+1}/{len(rows)}] {row['path']} exit={row['exit']} {row['wall_s']}s "
              f"k_s={row.get('k_s')} mono={row.get('monotone')} {row.get('refusal','')}",
              file=sys.stderr, flush=True)
    Path(a.out).write_text(json.dumps({"taken": time.strftime("%Y-%m-%d %H:%M:%S"),
                                       "bin": a.bin, "commit": "cdcc3994",
                                       "wall_total_s": round(time.time() - t0, 1),
                                       "rows": rows}, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
