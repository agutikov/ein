#!/usr/bin/env python3
"""M1e2 inventory, section C: join corpus_describe.json (bare `ein solve`),
region_census.json (`ein oracle` via utils/region_census.py --all) and
corpus_cores.json (`ein solve --dump-answer`'s nogoods.json) into one row per
corpus entry, assign the program classes, and write corpus.csv and
corpus_classes.json. Runs no engine.
"""
from __future__ import annotations

import csv
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent

desc = {r["path"]: r for r in json.loads((HERE / "corpus_describe.json").read_text())["rows"]}
cen = {r["path"]: r for r in json.loads((HERE / "region_census.json").read_text())}
cores = {r["path"]: r for r in json.loads((HERE / "corpus_cores.json").read_text())["rows"]}


def refusal_kind(r: dict) -> str | None:
    if r["loaded"]:
        return None
    k = r.get("refusal")
    if k == "refused-forbid":
        return "forbid"
    if k == "refused-load-pattern":
        return "load: range / head _ (S1e2.2.20)"
    if r["group"].endswith("negative"):
        return f"load: {r['group']} (by design)"
    return "load: query goal / rule compile / :assert var (by design)"


rows = []
for p, r in sorted(desc.items()):
    c = cen.get(p, {})
    k = cores.get(p, {})
    ref = refusal_kind(r)
    classes = []
    if ref == "forbid":
        classes.append("refused-forbid")
    elif ref:
        classes.append("refused-load")
    else:
        if r["monotone"]:
            classes.append("monotone-complete" if r["exhausted"] else "monotone-stopped")
        else:
            classes.append("non-monotone-complete" if r["exhausted"] else "non-monotone-stopped")
        if r.get("outside"):
            classes.append("H-scoped")
        if r.get("types_as_objects"):
            classes.append("types-as-objects")
    if r.get("monotone") is False:
        detector = f"non-monotone: {r.get('status_bit')}" + (" (cube)" if r.get("status_read") == "the cube" else "")
    elif r.get("monotone") is True:
        detector = "monotone (static check)"
    else:
        detector = "— (refused before the check)"
    row = {
        "path": p, "group": r["group"], "classes": " ".join(classes),
        "exit": r["exit"], "refusal": ref or "",
        "refusal_line": (r["stderr"][0][:160] if (ref and r["stderr"]) else ""),
        "detector": detector,
        "sites": "; ".join(f"{s['rule']}:{s['kind']}" for s in (r.get("sites") or [])),
        "holes_describe": r.get("holes"),
        "stale_nodes": r.get("stale_nodes"), "state_nodes": r.get("state_nodes"),
        "paths_differ": r.get("paths_differ"),
        "describe_completes": "" if ref else ("yes" if r["exhausted"] else f"stopped: {r['stopped'].get('why')}"),
        "k_s": r.get("k_s"), "k_c": r.get("k_c"), "N": r.get("nogoods"),
        "n_H": r.get("candidates"), "root": r.get("root"), "empty": r.get("empty"),
        "outside": " ".join((o.get("relation") if isinstance(o, dict) else str(o)) for o in (r.get("outside") or [])),
        "types_as_objects": " ".join(r.get("types_as_objects") or []),
        "oracle_k_s": c.get("k_s"), "oracle_k_c": c.get("k_c"), "oracle_N": c.get("nogoods"),
        "oracle_holes": c.get("holes"),
        "oracle_violations": json.dumps(c.get("violations")) if c.get("violations") else "",
        "oracle_n": c.get("n"), "oracle_exit": c.get("exit"),
        # 2026-11-06: the five fields § C.1 names, not three. Describe prints no
        # hole count on a monotone program (`holes` null: none, by the theorem
        # monotonicity.md § 3 states), so a null there reads as 0.
        "agree_describe_oracle": ("" if ("k_s" not in c or not r["loaded"] or not r["exhausted"]) else
                                  str(c["k_s"] == r["k_s"] and c["k_c"] == r["k_c"] and c["nogoods"] == r["nogoods"]
                                      and c.get("holes") == (0 if (r.get("holes") is None and r.get("monotone")) else r.get("holes"))
                                      and c.get("monotone") == r.get("monotone"))),
        "N_cores_within_root": k.get("core_within_root") if k.get("root") in ("in-r", "owing") else "",
        "N_cores_empty": k.get("core_empty") if k else "",
        # 2026-11-06: N6 (S1e2.3.9) writes a member past the limit `core: null`,
        # `core_status: "not-kept"`; W23 (S1e2.2.23) a cut core `cut-…`.
        "N_cores_null": k.get("core_null") if k else "",
        "N_core_status": json.dumps(k.get("core_status")) if k else "",
        "root_core_status": r.get("core_status") or "",
        "wall_s": r.get("wall_s"),
    }
    rows.append(row)

with open(HERE / "corpus.csv", "w", newline="") as f:
    w = csv.DictWriter(f, fieldnames=list(rows[0].keys()))
    w.writeheader()
    w.writerows(rows)
(HERE / "corpus_classes.json").write_text(json.dumps(rows, indent=1))

from collections import Counter
cnt = Counter()
for r in rows:
    for c in r["classes"].split():
        cnt[c] += 1
print(dict(cnt), len(rows))
