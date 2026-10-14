#!/usr/bin/env python3
"""Corpus scan for wrong-answer signatures (scratch, read-only on the repo).

For every non-negative, non-slow corpus entry that declares any `solve` run:
  1. lattice `solve -e -v -g 1 --json-summary` : verdict, k, exhausted, and the
     per-entering outcomes -> commitments that are alive (not solution), whose
     every entered child at the next layer died / none was entered, with the
     run exhausted  ==> "missed maximal" (Q-M1e.8's fix, computed outside).
  2. recorded models whose positive fact set strictly contains another's.
  3. EIN_TRAVERSAL=tree solve -e --json-summary --events : did the tree accept?
     if so compare model sets (positive facts) with the lattice.
"""
import json, os, re, subprocess, sys, tomllib, tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../.."))
EIN = os.path.join(ROOT, "ein.rs/target/release/ein")
OUT = sys.argv[1]
TIMEOUT = float(os.environ.get("SCAN_TIMEOUT", "60"))

man = tomllib.load(open(os.path.join(ROOT, "corpus/corpus.toml"), "rb"))
entries = [e for e in man["entry"]
           if e["group"] in ("positive", "stdlib", "regression")
           and not e.get("slow")
           and any(r.split()[0] == "solve" for r in e["runs"])]

ENT = re.compile(r"^\s+e=\s*(\d+) layer=(\d+)\s+(.*?)\s+->\s+(\S+)\s+solution-nodes=")


def run(args, env=None, timeout=TIMEOUT):
    try:
        p = subprocess.run(args, cwd=ROOT, capture_output=True, text=True,
                           timeout=timeout, env=env)
        return p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired:
        return None, "", ""


def positives(facts):
    return frozenset(f for f in facts if not f.startswith("(not ")
                     and not f.startswith("(relation "))


def split_facts(s):
    # "(p A) (p B)" -> ["(p A)", "(p B)"] ; balanced parens
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
    return tuple(sorted(out))


only = os.environ.get("FILES")
if only:
    entries = [{"path": p} for p in only.split(",")]
results = []
for e in entries:
    path = e["path"]
    rec = {"path": path}
    with tempfile.TemporaryDirectory() as td:
        js = os.path.join(td, "s.json")
        rc, so, se = run([EIN, "solve", "-e", "-v", "-g", "1", path,
                          "--json-summary", js])
        rec["rc"] = rc
        if rc is None or not os.path.exists(js):
            rec["error"] = "timeout" if rc is None else "no summary"
            results.append(rec)
            continue
        d = json.load(open(js))
        v = d["verdict"]
        rec.update(type=v["type"], k=v["k"], exhausted=v["exhausted"],
                   declared=d.get("owes", {}).get("declared"))
        models = [positives(s["facts"]) for s in v["solutions"]]
        sup = []
        for i, a in enumerate(models):
            for j, b in enumerate(models):
                if i != j and a < b:
                    sup.append((i, j, sorted(b - a)))
        rec["superset_pairs"] = sup
        # per-entering outcomes
        ents = []
        for line in se.splitlines():
            m = ENT.match(line)
            if m:
                ents.append((int(m.group(2)), split_facts(m.group(3)), m.group(4)))
        rec["n_ent"] = len(ents)
        by_layer = {}
        for L, c, o in ents:
            by_layer.setdefault(L, []).append((frozenset(c), o))
        missed = []
        if v["exhausted"]:
            for L, lst in by_layer.items():
                nxt = by_layer.get(L + 1, [])
                for c, o in lst:
                    if o != "alive":
                        continue
                    kids = [(k2, o2) for k2, o2 in nxt if c < k2]
                    if not any(o2 in ("alive", "solution") for _, o2 in kids):
                        missed.append((L, sorted(c), len(kids)))
        rec["missed_maximal"] = missed
        # tree
        ev = os.path.join(td, "t.jsonl")
        jt = os.path.join(td, "t.json")
        env = dict(os.environ, EIN_TRAVERSAL="tree")
        rc2, so2, se2 = run([EIN, "solve", "-e", path, "--json-summary", jt,
                             "--events", ev], env=env)
        acc = None
        if rc2 is not None and os.path.exists(ev):
            with open(ev) as f:
                for line in f:
                    if '"e":"traversal"' in line.replace(" ", ""):
                        t = json.loads(line)
                        if t.get("depth") in (None, 0) or "depth" not in t:
                            acc = t.get("verdict")
                        if acc is None:
                            acc = t.get("verdict")
        rec["tree"] = acc
        if acc and acc != "declined" and os.path.exists(jt):
            dt = json.load(open(jt))
            vt = dt["verdict"]
            mt = [positives(s["facts"]) for s in vt["solutions"]]
            rec["tree_k"] = vt["k"]
            rec["tree_type"] = vt["type"]
            sl, st = set(models), set(mt)
            rec["tree_only"] = [sorted(x) for x in st - sl]
            rec["lattice_only"] = [sorted(x) for x in sl - st]
    results.append(rec)
    flag = []
    if rec.get("superset_pairs"):
        flag.append("SUPERSET")
    if rec.get("missed_maximal"):
        flag.append("MISSED%d" % len(rec["missed_maximal"]))
    if rec.get("tree_only") or rec.get("lattice_only"):
        flag.append("TREEDIFF")
    print(f'{path:70s} {rec.get("type","?"):13s} k={rec.get("k")} ex={rec.get("exhausted")} decl={rec.get("declared")} tree={rec.get("tree")} {" ".join(flag)}', flush=True)

json.dump(results, open(OUT, "w"), indent=1, default=list)
