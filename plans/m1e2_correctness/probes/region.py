#!/usr/bin/env python3
"""The solution region of a small program, by brute force — Q-M1e2.7's table.

    region.py FILE.ein FACT…        e.g.  region.py branching/15.ein '(p A)' '(p B)' '(p C)'

Every subset C of the given candidate facts is appended to FILE as given facts
and classified: dead (`ein saturate` derives `false`), owing n (`--json-summary`'s
`owes.root.total` of a `solve -e -m 0`), or ok. R = the ok nodes; its minimal and
maximal lines are printed by subset comparison over R, never by assuming R has
no holes.

The forced-positive cascade and the pre-branch lookahead are switched off in
the copy: they are search-time root writes, and the first version of this
script, which left them on, reported `{(seats Ann C1)}` on owed-seats +
functional + injective as owing nothing, because the search had promoted
`(seats Bob C2)`. The user's ruling of 2026-10-15 is that no config option may
change the answer, so the classification is saturation alone.

One process per node, 2^n of them: a scratch instrument and the second route
S1e2.1a.1's library oracle is checked against. Reads the repo, writes only a
temporary directory. The engine is $EIN_BIN, else ein.rs/target/release/ein.
"""
import itertools, json, os, re, subprocess, sys, tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../.."))
EIN = os.environ.get("EIN_BIN", os.path.join(ROOT, "ein.rs/target/release/ein"))
OFF = "(config :enable-forced-positive false :enable-pre-branch-lookahead false)"


def classify(src, sub, d):
    f = os.path.join(d, "x.ein")
    with open(f, "w") as fh:
        fh.write(src + "\n" + OFF + "\n" + " ".join(sub) + "\n")
    sat = subprocess.run([EIN, "saturate", f], capture_output=True, text=True).stdout
    if re.search(r"^\s+false\s+\d+", sat, re.M):
        return "dead"
    j = os.path.join(d, "s.json")
    subprocess.run([EIN, "solve", "-e", "-m", "0", f, "--json-summary", j], capture_output=True)
    with open(j) as fh:
        owes = json.load(fh)["owes"]["root"]["total"]
    return f"owes {owes}" if owes else "ok"


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    # Drop the file's own (config …) lines so the switch above is the only one.
    src = "\n".join(l for l in open(sys.argv[1]).read().split("\n") if not l.startswith("(config"))
    cands = sys.argv[2:]
    rows = {}
    with tempfile.TemporaryDirectory() as d:
        for n in range(len(cands) + 1):
            for sub in itertools.combinations(cands, n):
                rows[frozenset(sub)] = classify(src, sub, d)
    ok = [s for s, c in rows.items() if c == "ok"]
    lo = [s for s in ok if not any(t < s for t in ok)]
    hi = [s for s in ok if not any(t > s for t in ok)]
    holes = [s for s, c in rows.items() if c != "ok" and any(a < s for a in ok) and any(s < b for b in ok)]
    name = lambda s: "{" + ", ".join(sorted(s)) + "}" if s else "∅"
    for n in range(len(cands) + 1):
        print(f"L{n}: " + "  ".join(f"{name(s)}={c}" for s, c in rows.items() if len(s) == n))
    print(f"region: {len(ok)} nodes;  minimal: {', '.join(map(name, lo)) or '—'};  "
          f"maximal: {', '.join(map(name, hi)) or '—'};  holes: {', '.join(map(name, holes)) or 'none'}")


if __name__ == "__main__":
    main()
