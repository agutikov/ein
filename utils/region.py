#!/usr/bin/env python3
"""The solution region of a small program, by brute force: Q-M1e2.7's tables.

    region.py [--json] FILE.ein FACT…   e.g.  region.py branching/15.ein '(p A)' '(p B)' '(p C)'

Banked as plans/m1e2_correctness/probes/region.py when Q-M1e2.7 was filed and
moved here by M1e2 S1e2.1a.1, which made it the **second route** the library
oracle (`ein solve --dump-region`, `ein_infer::region`) is checked against:
`ein-cli/tests/region_dump.rs` runs both on Q-M1e2.7's seven programs and
compares them node for node. `--json` is the form that test reads.

The lattice over n candidate facts is the n-cube: a node is an n-bit mask, an
edge adds or removes one fact, and layer k is the masks with k bits set. Every
node C is appended to FILE as given facts and classified: dead (`ein saturate`
derives `false`), owing n (`--json-summary`'s `owes.root.total` of a
`solve -e -m 0`), or ok. R = the ok nodes.

Printed, all by subset comparison over the cube, never by assuming R has no
holes:
  the layers         every node's status
  region             |R|, its minimal and maximal lines, its holes (nodes
                     outside R between two members of R)
  components         R's connected pieces under one-fact edges
  sub-cubes          R as a union of maximal intervals [a .. b], each with its
                     dimension |b| - |a|, the number of free facts in it
  nogoods            N, the minimal dead sets
  duality            without holes the maximal line is the complements of the
                     minimal sets that hit every member of N, kept where
                     discharged. Printed as holds, or as what differs
  edges              every cube edge classed by the statuses at its two ends,
                     plus the repair edges (hole -> R, upward)
  local vs global    nodes of R whose one-fact neighbours say "maximal"
                     (or "minimal") where the whole cube says otherwise

`--json` also gives every alive node's **closure**: the candidates its
fixpoint holds, read off `--dump-answer`'s `root.ein` — where saturation moves
the node inside the cube (S1e2.1a.2), checked against the oracle's `closure`.

The forced-positive cascade and the pre-branch lookahead are switched off in
the copy: they are search-time root writes, and the first version of this
script, which left them on, reported `{(seats Ann C1)}` on owed-seats +
functional + injective as owing nothing, because the search had promoted
`(seats Bob C2)`. The user's ruling of 2026-10-15 is that no config option may
change the answer, so the classification is saturation alone.

A dead node's debt is not read: `owes.root` reports 0 on a dead root whatever
it owes. So this route cannot compute the minimal discharging sets E on their
own, and the duality is checked on the N side only (S1e2.1a.1 reads both bits).

One process per node, 2^n of them. This is a scratch instrument and the second
route S1e2.1a.1's library oracle is checked against. It reads the repo and
writes only a temporary directory. The engine is $EIN_BIN, else
ein.rs/target/release/ein.
"""
import itertools, json, os, re, subprocess, sys, tempfile
from collections import Counter

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
EIN = os.environ.get("EIN_BIN", os.path.join(ROOT, "ein.rs/target/release/ein"))
OFF = "(config :enable-forced-positive false :enable-pre-branch-lookahead false)"


def classify(src, sub, d, H):
    """('dead' | 'owes n' | 'ok', closure) for the program plus the facts in
    `sub`. The closure is the candidates of `H` the fixpoint holds — read off
    `--dump-answer`'s `root.ein`, so on an alive node only; `None` on a dead
    one."""
    f = os.path.join(d, "x.ein")
    with open(f, "w") as fh:
        fh.write(src + "\n" + OFF + "\n" + " ".join(sub) + "\n")
    sat = subprocess.run([EIN, "saturate", f], capture_output=True, text=True).stdout
    if re.search(r"^\s+false\s+\d+", sat, re.M):
        return "dead", None
    j, a = os.path.join(d, "s.json"), os.path.join(d, "answer")
    subprocess.run([EIN, "solve", "-e", "-m", "0", f, "--json-summary", j, "--dump-answer", a],
                   capture_output=True)
    with open(j) as fh:
        owes = json.load(fh)["owes"]["root"]["total"]
    with open(os.path.join(a, "root.ein")) as fh:
        held = {re.sub(r"\s+:(rule|source|hypothesis|using)\s+(\"[^\"]*\"|\S+)", "", l.strip())
                for l in fh if l.startswith("(")}
    return (f"owes {owes}" if owes else "ok"), sorted(h for h in H if h in held)


def le(a, b):
    return a & b == a


def topology(n, st):
    """Everything above the layers, from the status array indexed by mask."""
    N = 1 << n
    R = [s == "ok" for s in st]
    dead = [s == "dead" for s in st]
    inR = [m for m in range(N) if R[m]]
    mn = [m for m in inR if not any(R[x] and x != m and le(x, m) for x in inR)]
    mx = [m for m in inR if not any(R[x] and x != m and le(m, x) for x in inR)]
    holes = [m for m in range(N) if not R[m]
             and any(le(a, m) for a in inR) and any(le(m, b) for b in inR)]
    comps, seen = [], set()
    for m in inR:
        if m in seen:
            continue
        seen.add(m)
        stack, piece = [m], []
        while stack:
            x = stack.pop()
            piece.append(x)
            for i in range(n):
                y = x ^ (1 << i)
                if R[y] and y not in seen:
                    seen.add(y)
                    stack.append(y)
        comps.append(sorted(piece))
    cubes = [(a, b) for a in inR for b in inR
             if le(a, b) and all(R[x] for x in range(N) if le(a, x) and le(x, b))]
    prime = [(a, b) for a, b in cubes
             if not any((c, d) != (a, b) and le(c, a) and le(b, d) for c, d in cubes)]
    nogoods = [m for m in range(N) if dead[m] and not any(dead[x] and x != m and le(x, m) for x in range(N))]
    hitting = [m for m in range(N) if all(m & g for g in nogoods)]
    transversals = [m for m in hitting if not any(x != m and le(x, m) for x in hitting)]
    full = N - 1
    dual = sorted(full ^ t for t in transversals if st[full ^ t] == "ok")
    edges = Counter()
    repairs = []
    local_max, local_min = [], []
    for x in range(N):
        for i in range(n):
            if not x >> i & 1:
                y = x | 1 << i
                cls = lambda s: "R" if s == "ok" else ("dead" if s == "dead" else "owing")
                edges[f"{cls(st[x])}→{cls(st[y])}"] += 1
                if x in holes and R[y]:
                    repairs.append((x, i, y))
    for m in inR:
        ups = [m | 1 << i for i in range(n) if not m >> i & 1]
        downs = [m & ~(1 << i) for i in range(n) if m >> i & 1]
        if all(not R[u] for u in ups) != (m in mx):
            local_max.append(m)
        if all(not R[d] for d in downs) != (m in mn):
            local_min.append(m)
    return dict(inR=inR, mn=mn, mx=mx, holes=holes, comps=comps, prime=prime, nogoods=nogoods,
                dual=dual, edges=edges, repairs=repairs, local_max=local_max, local_min=local_min)


def main():
    argv = sys.argv[1:]
    as_json = bool(argv) and argv[0] == "--json"
    if as_json:
        argv = argv[1:]
    if not argv:
        sys.exit(__doc__)
    # Drop the file's own (config …) lines so the switch above is the only one.
    src = "\n".join(l for l in open(argv[0]).read().split("\n") if not l.startswith("(config"))
    H = argv[1:]
    n = len(H)
    with tempfile.TemporaryDirectory() as d:
        out = [classify(src, [H[i] for i in range(n) if m >> i & 1], d, H) for m in range(1 << n)]
    st = [o[0] for o in out]
    t = topology(n, st)
    if as_json:
        facts = lambda m: [H[i] for i in range(n) if m >> i & 1]
        sets = lambda ms: sorted(sorted(facts(m)) for m in ms)
        print(json.dumps({
            "candidates": H,
            "nodes": [{"facts": sorted(facts(m)), "status": st[m], "closure": out[m][1]}
                      for m in range(1 << n)],
            "minimal": sets(t["mn"]),
            "maximal": sets(t["mx"]),
            "holes": sets(t["holes"]),
            "components": sorted(len(c) for c in t["comps"]),
            "subcubes": sorted([sorted(facts(a)), sorted(facts(b))] for a, b in t["prime"]),
            "nogoods": sets(t["nogoods"]),
            "duality_n": sorted(t["mx"]) == t["dual"],
        }, indent=1))
        return
    name = lambda m: "{" + ", ".join(H[i] for i in range(n) if m >> i & 1) + "}" if m else "∅"
    J = lambda ms: ", ".join(map(name, ms)) or "—"
    for k in range(n + 1):
        print(f"L{k}: " + "  ".join(f"{name(m)}={st[m]}" for m in range(1 << n) if bin(m).count("1") == k))
    print(f"region: {len(t['inR'])} nodes;  minimal: {J(t['mn'])};  maximal: {J(t['mx'])};  holes: {J(t['holes']) if t['holes'] else 'none'}")
    print(f"components: {len(t['comps'])}, sizes {[len(c) for c in t['comps']]}")
    print("sub-cubes: " + ("  ".join(f"[{name(a)} .. {name(b)}] dim {bin(a ^ b).count('1')}" for a, b in t["prime"]) or "—"))
    print(f"nogoods: {J(t['nogoods'])}")
    same = sorted(t["mx"]) == t["dual"]
    print("duality: " + ("holds" if same else f"fails — complements of the minimal transversals of N, discharged: {J(t['dual'])}"))
    print("edges: " + ", ".join(f"{k} {v}" for k, v in sorted(t["edges"].items())))
    if t["repairs"]:
        print("repairs: " + ", ".join(f"{name(x)} + {H[i]} → {name(y)}" for x, i, y in t["repairs"]))
    if t["local_max"] or t["local_min"]:
        print(f"local ≠ global: maximal {J(t['local_max'])}; minimal {J(t['local_min'])}")


if __name__ == "__main__":
    main()
