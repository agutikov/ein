#!/usr/bin/env python3
"""Every corpus program that declares an hrule, as written and with its hrules taken out.

For each: the rung H is built by and H itself (`ein show hyps`), and the region (`ein solve --json-summary`),
of (A) the program, (B) the program with every `(hrule …)` form and every `:hrules` clause deleted, and
(C) B with `:hypothesis-relations` naming the relations of A's candidates, where B fell to the blind rung.
Usage: hr_census.py EIN TREE OUT.tsv [JOBS]
"""
import json, os, re, subprocess, sys, pathlib, concurrent.futures as cf

EIN, TREE, OUT = sys.argv[1], pathlib.Path(sys.argv[2]), sys.argv[3]
JOBS = int(sys.argv[4]) if len(sys.argv) > 4 else 8
TIMEOUT = 90

def forms(src):
    """(start, end) of every top-level form, comments and strings skipped."""
    out, i, n, depth, start = [], 0, len(src), 0, None
    while i < n:
        c = src[i]
        if c == ';':
            while i < n and src[i] != '\n': i += 1
            continue
        if c == '"':
            i += 1
            while i < n and src[i] != '"':
                i += 2 if src[i] == '\\' else 1
            i += 1; continue
        if c == '(':
            if depth == 0: start = i
            depth += 1
        elif c == ')':
            depth -= 1
            if depth == 0: out.append((start, i + 1))
        i += 1
    return out

def skip_ws(src, i):
    n = len(src)
    while i < n:
        if src[i] in ' \t\r\n': i += 1
        elif src[i] == ';':
            while i < n and src[i] != '\n': i += 1
        else: break
    return i

def value_end(src, i):
    """End of the one value that starts at i: a parenthesised list or a bare word."""
    if src[i] != '(':
        while i < len(src) and src[i] not in ' \t\r\n()': i += 1
        return i
    depth = 0
    while True:
        c = src[i]
        if c == ';':
            while src[i] != '\n': i += 1
        elif c == '"':
            i += 1
            while src[i] != '"': i += 2 if src[i] == '\\' else 1
        elif c == '(': depth += 1
        elif c == ')':
            depth -= 1
            if depth == 0: return i + 1
        i += 1

def head(form): return re.match(r"\(\s*([^\s()]+)", form).group(1)

def without_hrules(src, add_scope=None):
    out, last, solve_seen = [], 0, False
    for a, b in forms(src):
        f = src[a:b]; h = head(f)
        out.append(src[last:a]); last = b
        if h == "hrule": continue
        if h == "solve":
            solve_seen = True
            m = re.search(r":hrules\b", f)
            if m:
                v = skip_ws(f, m.end()); e = value_end(f, v)
                f = f[:m.start()] + f[e:]
            if add_scope:
                f = f.rstrip()[:-1] + f" :hypothesis-relations ({' '.join(add_scope)}))"
            if re.fullmatch(r"\(\s*solve\s*\)", re.sub(r";[^\n]*", "", f), re.S): continue
        out.append(f)
    out.append(src[last:])
    s = "".join(out)
    if add_scope and not solve_seen:
        s += f"\n(solve :hypothesis-relations ({' '.join(add_scope)}))\n"
    return s

def run(args):
    try:
        p = subprocess.run(args, capture_output=True, text=True, timeout=TIMEOUT)
        return p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired:
        return None, "", "TIMEOUT"

def look(path):
    rc, out, err = run([EIN, "show", "hyps", str(path)])
    text = out + err
    m = re.search(r"built by the rung `([^`]+)`: (\d+) candidate", text)
    rung = m.group(1) if m else ("TIMEOUT" if rc is None else "refused" if "error" in text else "none")
    H = sorted(l for l in out.splitlines() if l.startswith("("))
    js = str(path) + ".json"
    if os.path.exists(js): os.remove(js)
    rc2, out2, err2 = run([EIN, "solve", str(path), "--json-summary", js])
    if rc2 is None: return dict(rung=rung, H=H, region="TIMEOUT")
    if not os.path.exists(js): return dict(rung=rung, H=H, region="REFUSED", msg=(out2 + err2).strip().splitlines()[:1])
    d = json.load(open(js)); r = d["region"]
    states = sorted(tuple(sorted(s["facts"])) for s in (r.get("states") or []))
    return dict(rung=rung, H=H, region=dict(k_s=r["k_s"], k_c=r["k_c"], root=r["root"], root_owes=r["root_owes"],
                exhausted=r["exhausted"], states=states, route=r["route"]),
                mono=d.get("monotonicity", {}).get("verdict") if isinstance(d.get("monotonicity"), dict) else d.get("monotonicity"),
                sat=(d.get("work") or {}).get("saturations"))

def same_region(a, b):
    if not isinstance(a, dict) or not isinstance(b, dict): return False
    return all(a[k] == b[k] for k in ("k_s", "k_c", "root", "states"))

def one(path):
    src = path.read_text()
    A = look(path)
    pB = path.with_suffix(".nohr.ein"); pB.write_text(without_hrules(src)); B = look(pB)
    rels = sorted({re.match(r"\(\s*([^\s()]+)", h).group(1) for h in A["H"]})
    C = None
    if B["rung"] == "blind" and rels:
        pC = path.with_suffix(".scoped.ein"); pC.write_text(without_hrules(src, add_scope=rels)); C = look(pC)
    return path, A, B, C, rels

files = sorted(p for p in TREE.rglob("*.ein") if p.is_file() and not p.name.endswith((".nohr.ein", ".scoped.ein")) and re.search(r"^\(hrule\b", p.read_text(), re.M))
rows = []
with cf.ThreadPoolExecutor(JOBS) as ex:
    for path, A, B, C, rels in ex.map(one, files):
        ra, rb = A["region"], B["region"]
        def reg(x): return x if isinstance(x, str) else f"k_s={x['k_s']} k_c={x['k_c']} root={x['root']}"
        rows.append([str(path.relative_to(TREE)), A["rung"], len(A["H"]), reg(ra), str(ra.get("root_owes") if isinstance(ra, dict) else ""),
                     B["rung"], len(B["H"]), reg(rb), "H=" if A["H"] == B["H"] else "H≠", "R=" if same_region(ra, rb) else "R≠",
                     (C["rung"] if C else ""), (len(C["H"]) if C else ""), (reg(C["region"]) if C else ""),
                     ("H=" if C and A["H"] == C["H"] else "H≠" if C else ""), ("R=" if C and same_region(ra, C["region"]) else "R≠" if C else ""),
                     " ".join(rels)])
with open(OUT, "w") as f:
    f.write("\t".join(["program", "A.rung", "A.|H|", "A.region", "A.root_owes", "B.rung", "B.|H|", "B.region", "B.H", "B.R",
                       "C.rung", "C.|H|", "C.region", "C.H", "C.R", "relations of A's H"]) + "\n")
    for r in rows: f.write("\t".join(map(str, r)) + "\n")
print(len(rows), "programs")
