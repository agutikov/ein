#!/usr/bin/env python3
"""(1) Which rung builds H, for every corpus program. (2) Every program that states `:no-hypothesis` or
`:hypothesis-relations`, as written and with the list replaced by closure facts `(__closed__ R)`.
Usage: scope_census.py EIN TREE OUT_PREFIX [JOBS]"""
import json, os, re, subprocess, sys, pathlib, collections, concurrent.futures as cf
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
EIN, TREE, OUT = sys.argv[1], pathlib.Path(sys.argv[2]), sys.argv[3]
JOBS = int(sys.argv[4]) if len(sys.argv) > 4 else 8
sys.argv = [sys.argv[0], EIN, str(TREE), os.devnull, "1"]
import importlib.util
spec = importlib.util.spec_from_file_location("hr", os.path.join(os.path.dirname(os.path.abspath(__file__)), "hr_lib.py"))
hr = importlib.util.module_from_spec(spec); spec.loader.exec_module(hr)

SUFFIXES = (".nohr.ein", ".scoped.ein", ".closed.ein", ".open.ein")
files = sorted(p for p in TREE.rglob("*.ein") if p.is_file() and not p.name.endswith(SUFFIXES))

def rung(path):
    rc, out, err = hr.run([EIN, "show", "hyps", str(path)])
    t = out + err
    m = re.search(r"built by the rung `([^`]+)`", t)
    if m: return m.group(1)
    if rc is None: return "timeout"
    if "error" in t: return "refused at load"
    return "no rung line"

with cf.ThreadPoolExecutor(JOBS) as ex:
    rungs = list(ex.map(rung, files))
c = collections.Counter(rungs)
with open(OUT + ".rungs.tsv", "w") as f:
    for p, r in zip(files, rungs): f.write(f"{p.relative_to(TREE)}\t{r}\n")
print("the rung of each of", len(files), "programs:")
for k, v in c.most_common(): print(f"  {v:4d}  {k}")

def clause(form, key):
    m = re.search(re.escape(key) + r"(?![\w-])", form)
    if not m: return None
    v = hr.skip_ws(form, m.end()); e = hr.value_end(form, v)
    return m.start(), e, form[v:e]

def rewrite(src, key, closures):
    out, last = [], 0
    for a, b in hr.forms(src):
        f = src[a:b]; out.append(src[last:a]); last = b
        if hr.head(f) == "solve":
            cl = clause(f, key)
            if cl: f = f[:cl[0]] + f[cl[1]:]
            if re.fullmatch(r"\(\s*solve\s*\)", re.sub(r";[^\n]*", "", f), re.S): continue
        out.append(f)
    out.append(src[last:])
    return "".join(out) + "\n" + "\n".join(f"(__closed__ {r})" for r in closures) + "\n"

def listed(src, key):
    for a, b in hr.forms(src):
        f = src[a:b]
        if hr.head(f) == "solve":
            cl = clause(f, key)
            if cl: return re.findall(r"[^\s()]+", cl[2])
    return None

def one(path):
    src = path.read_text(); res = []
    A = None
    for key, suffix in ((":no-hypothesis", ".closed.ein"), (":hypothesis-relations", ".open.ein")):
        L = listed(src, key)
        if L is None: continue
        A = A or hr.look(path)
        if key == ":no-hypothesis":
            closures = L
        else:
            # what would be proposed with the list taken out, less the list: those are what it scopes out
            p0 = path.with_suffix(".tmp.ein"); p0.write_text(rewrite(src, key, [])); B0 = hr.look(p0); p0.unlink()
            js = str(p0) + ".json"
            if os.path.exists(js): os.remove(js)
            closures = sorted({re.match(r"\(\s*([^\s()]+)", h).group(1) for h in B0["H"]} - set(L))
        pv = path.with_suffix(suffix); pv.write_text(rewrite(src, key, closures)); V = hr.look(pv)
        ra, rv = A["region"], V["region"]
        res.append((str(path.relative_to(TREE)), key, len(L), len(closures), A["rung"], V["rung"],
                    ra if isinstance(ra, str) else "answered", rv if isinstance(rv, str) else "answered",
                    "H=" if A["H"] == V["H"] else "H≠", "R=" if hr.same_region(ra, rv) else ("R≠" if isinstance(ra, dict) and isinstance(rv, dict) else "—"),
                    (V.get("msg") or [""])[0][:200] if isinstance(rv, str) else ""))
    return res

cands = [p for p in files if re.search(r":no-hypothesis|:hypothesis-relations", p.read_text())]
rows = []
with cf.ThreadPoolExecutor(JOBS) as ex:
    for r in ex.map(one, cands): rows.extend(r)
with open(OUT + ".lists.tsv", "w") as f:
    f.write("program\tlist\t|list|\tclosures written\tA.rung\tV.rung\tA\tV\tH\tR\tV's refusal\n")
    for r in rows: f.write("\t".join(map(str, r)) + "\n")
for key in (":no-hypothesis", ":hypothesis-relations"):
    sub = [r for r in rows if r[1] == key]
    print(f"\n{key}: {len(sub)} programs")
    cc = collections.Counter((r[6], r[7], r[8], r[9]) for r in sub)
    for k, v in cc.most_common(): print(f"  {v:4d}  as written: {k[0]:9s} with closures: {k[1]:9s} {k[2]} {k[3]}")
