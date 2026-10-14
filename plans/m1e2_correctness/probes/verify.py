#!/usr/bin/env python3
"""Second pass over scan_all.json: is a 'missed maximal' commitment C really a
model the engine did not record?  Re-solve `program + C-as-facts`; if that is
consistent its model is a candidate missed solution (compared against the
recorded models).  For a Contradiction that entered anything, also test root:
`program + (not h)` for every layer-1 death, i.e. root with what the search
says is entailed.  Scratch instrument; reads the repo, writes only /tmp."""
import json, os, re, subprocess, sys, tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../.."))
EIN = os.path.join(ROOT, "ein.rs/target/release/ein")
ENT = re.compile(r"^\s+e=\s*(\d+) layer=(\d+)\s+(.*?)\s+->\s+(\S+)\s+solution-nodes=")


def split_facts(s):
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch == "(":
            depth += 1
        if depth > 0:
            cur += ch
        if ch == ")":
            depth -= 1
            if depth == 0:
                out.append(cur)
                cur = ""
    return out


def positives(facts):
    return frozenset(f for f in facts if not f.startswith("(not ")
                     and not f.startswith("(relation "))


def solve(text, srcdir, extra=()):
    # write next to nothing in the repo: a temp dir, but keep imports working
    # by running with cwd=ROOT (imports resolve via the stdlib, not the file dir)
    with tempfile.TemporaryDirectory() as td:
        p = os.path.join(td, "probe.ein")
        open(p, "w").write(text)
        js = os.path.join(td, "s.json")
        try:
            r = subprocess.run([EIN, "solve", "-e", p, "--json-summary", js, *extra],
                               cwd=ROOT, capture_output=True, text=True, timeout=120)
        except subprocess.TimeoutExpired:
            return None
        if not os.path.exists(js):
            return {"err": (r.stdout + r.stderr)[-300:]}
        d = json.load(open(js))
        v = d["verdict"]
        return {"type": v["type"], "k": v["k"], "exhausted": v["exhausted"],
                "models": [positives(s["facts"]) for s in v["solutions"]],
                "enterings": d["stats"].get("enterings_total")}


scan = json.load(open(sys.argv[1]))
report = []
for rec in scan:
    path = rec["path"]
    if rec.get("error"):
        continue
    want_root = rec.get("type") == "Contradiction" and rec.get("exhausted") and rec.get("n_ent", 0) > 0
    if not rec.get("missed_maximal") and not want_root:
        continue
    src = open(os.path.join(ROOT, path)).read()
    # the original recorded models, for comparison
    orig = solve(src, None)
    out = {"path": path, "orig": {k: v for k, v in orig.items() if k != "models"} if orig else None,
           "checks": []}
    seen = set()
    for (L, c, nkids) in rec.get("missed_maximal", []):
        key = tuple(c)
        if key in seen:
            continue
        seen.add(key)
        res = solve(src + "\n;; probe\n" + "\n".join(c) + "\n", None)
        if res is None or "err" in res:
            out["checks"].append({"C": c, "result": res})
            continue
        new = [m for m in res["models"] if m not in (orig["models"] if orig else [])]
        out["checks"].append({"C": c, "layer": L, "type": res["type"], "k": res["k"],
                              "exhausted": res["exhausted"],
                              "models_not_recorded_originally": [sorted(m) for m in new]})
    if want_root:
        # layer-1 deaths, from a verbose run
        r = subprocess.run([EIN, "solve", "-e", "-v", "-g", "1", path], cwd=ROOT,
                           capture_output=True, text=True, timeout=120)
        dead1 = []
        for line in r.stderr.splitlines():
            m = ENT.match(line)
            if m and m.group(2) == "1" and m.group(4).startswith("dead"):
                fs = split_facts(m.group(3))
                if len(fs) == 1:
                    dead1.append(fs[0])
        if dead1:
            negs = "\n".join("(not %s)" % f for f in dead1)
            res = solve(src + "\n;; root probe\n" + negs + "\n", None)
            out["root_probe"] = {"dead_singletons": dead1,
                                 "result": None if res is None else {k: (v if k != "models" else [sorted(m) for m in v]) for k, v in res.items()}}
    report.append(out)
    print(json.dumps(out, default=list)[:1500], flush=True)

json.dump(report, open(sys.argv[2], "w"), indent=1, default=list)
