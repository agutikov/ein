#!/usr/bin/env python3
"""M1e2 inventory: every core `ein solve --dump-answer` writes, on every corpus
entry whose describe run (corpus_describe.json) reported N > 0 or a dead root.
One engine process at a time. Writes corpus_cores.json beside this script.

For each member of N (nogoods.json): does its core name any fact of its set
(the commitment)? Is its core empty? Is every core fact in root's fixpoint
(`ein saturate --dump`, read as utils/region_census.py's root_facts does)?
That last one is W21's class-4 claim (S1e2.2.21 step 4): a core made only of
facts root holds is no refutation while root is alive.

    EIN_STDLIB=src/stdlib nice -n 19 python3 sweep_cores.py
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
SCR = HERE / "scratch" / "cores"
BIN = str(HERE / "target/release/ein")


def root_facts(path: str, env) -> set[str]:
    p = subprocess.run([BIN, "saturate", path, "--dump"], cwd=SRC, capture_output=True,
                       text=True, timeout=600, env=env)
    out, on = set(), False
    for line in p.stdout.splitlines():
        if "SATURATED KB DUMP" in line:
            on = True
        elif on and line.startswith("  (") and line.endswith(")"):
            fact = line.strip()
            if " :" in fact:
                fact = fact[:fact.index(" :")] + ")"
            out.add(fact)
    return out


def main() -> int:
    env = dict(os.environ)
    env.setdefault("EIN_STDLIB", str(SRC / "stdlib"))
    env["TMPDIR"] = str(SCR)
    SCR.mkdir(parents=True, exist_ok=True)
    desc = json.loads((HERE / "corpus_describe.json").read_text())["rows"]
    todo = [r for r in desc if r.get("loaded") and ((r.get("nogoods") or 0) > 0 or r.get("empty") == "dead")]
    rows = []
    t0 = time.time()
    for i, r in enumerate(todo):
        d = SCR / "ans"
        if d.exists():
            shutil.rmtree(d)
        t = time.perf_counter()
        p = subprocess.run([BIN, "solve", r["path"], "--dump-answer", str(d)], cwd=SRC,
                           capture_output=True, text=True, timeout=900, env=env)
        ng = json.loads((d / "nogoods.json").read_text()) if (d / "nogoods.json").exists() else []
        summ = json.loads((d / "answer.json").read_text()) if (d / "answer.json").exists() else {}
        reg = summ.get("region") or {}
        root = root_facts(r["path"], env)
        row = {"path": r["path"], "exit": p.returncode, "monotone": r.get("monotone"),
               "exhausted": r.get("exhausted"), "stopped": r.get("stopped"),
               "root": reg.get("root"), "empty": reg.get("empty"),
               "root_core": reg.get("core"), "root_refuted_by": reg.get("refuted_by"),
               "N": len(ng),
               "core_empty": sum(1 for x in ng if not x.get("core")),
               "core_names_no_set_fact": sum(1 for x in ng if x.get("core") and not set(x["core"]) & set(x["set"])),
               "core_within_root": sum(1 for x in ng if x.get("core") and set(x["core"]) <= root),
               "refuted_by_empty": sum(1 for x in ng if not x.get("refuted_by")),
               "examples_within_root": [x for x in ng if x.get("core") and set(x["core"]) <= root][:2],
               "examples_empty": [x for x in ng if not x.get("core")][:2],
               "wall_s": round(time.perf_counter() - t, 3)}
        rows.append(row)
        print(f"[{i+1}/{len(todo)}] {r['path']} N={row['N']} empty={row['core_empty']} "
              f"none-of-set={row['core_names_no_set_fact']} within-root={row['core_within_root']}",
              file=sys.stderr, flush=True)
    (HERE / "corpus_cores.json").write_text(json.dumps(
        {"taken": time.strftime("%Y-%m-%d %H:%M:%S"), "commit": "cdcc3994", "bin": BIN,
         "wall_total_s": round(time.time() - t0, 1), "rows": rows}, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
