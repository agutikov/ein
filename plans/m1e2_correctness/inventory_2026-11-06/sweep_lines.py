#!/usr/bin/env python3
"""M1e2 inventory re-take (2026-11-06), added to the 11-05 set: two rows of
describe's read-out the re-take's new items name, on every corpus program that
loads (corpus_describe.json), with `ein solve FILE --json-summary J`:

- S-17: the minimal line row says *the trivial line: no obligation is stated*
  while the program states one (the summary's `monotonicity.plans.obligation`
  > 0, or a site of kind `obligation-trigger`);
- D-14: a minimal hole that is not dead is written `core: []` (the summary's
  `region.holes.minimal`).

One engine process at a time. Writes corpus_lines.json beside this script.

    EIN_STDLIB=src/stdlib nice -n 19 python3 sweep_lines.py
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SRC = HERE / "src"
BIN = str(HERE / "target/release/ein")
SCR = HERE / "scratch" / "lines"


def main() -> int:
    env = dict(os.environ)
    env.setdefault("EIN_STDLIB", str(SRC / "stdlib"))
    env["TMPDIR"] = str(SCR)
    SCR.mkdir(parents=True, exist_ok=True)
    desc = json.loads((HERE / "corpus_describe.json").read_text())["rows"]
    out = []
    for r in desc:
        if not r.get("loaded"):
            continue
        j = SCR / "s.json"
        if j.exists():
            j.unlink()
        p = subprocess.run([BIN, "solve", r["path"], "--json-summary", str(j)], cwd=SRC,
                           capture_output=True, text=True, timeout=900, env=env)
        s = json.loads(j.read_text())
        mono = s.get("monotonicity") or {}
        obl = ((mono.get("plans") or {}).get("obligation") or 0)
        trig = any(x.get("kind") == "obligation-trigger" for x in (mono.get("sites") or []))
        minrow = [l.strip() for l in p.stdout.splitlines() if l.strip().startswith("minimal line")]
        holes = ((s.get("region") or {}).get("holes") or {})
        hm = holes.get("minimal") if isinstance(holes, dict) else None
        out.append({
            "path": r["path"], "monotone": mono.get("monotone"),
            "exhausted": (s.get("region") or {}).get("exhausted"),
            "obligation_plans": obl, "trigger_site": trig,
            "minimal_row": minrow[0] if minrow else None,
            "says_no_obligation": bool(minrow and "no obligation is stated" in minrow[0]),
            "holes_minimal": hm,
            "owing_holes_core_empty": sum(1 for h in (hm or []) if not h.get("dead") and h.get("core") == []),
        })
    (HERE / "corpus_lines.json").write_text(json.dumps(out, indent=1))
    bad = [x for x in out if x["says_no_obligation"] and (x["obligation_plans"] or x["trigger_site"])]
    print(f"{len(out)} programs; minimal row says no obligation is stated where one is: {len(bad)}")
    for x in bad:
        print("  ", x["path"], x["obligation_plans"], x["minimal_row"])
    h = [x for x in out if x["owing_holes_core_empty"]]
    print(f"owing minimal holes written core []: {sum(x['owing_holes_core_empty'] for x in h)} on {len(h)} programs")
    for x in h:
        print("  ", x["path"], x["owing_holes_core_empty"])
    return 0


if __name__ == "__main__":
    sys.exit(main())
