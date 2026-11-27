#!/usr/bin/env python3
"""Every corpus program over std.slots, and over std.debts, which imports it,
under the old stdlib and the new one, by one binary: each `solve` run the
corpus declares, the read-out's rows that count R, the query classes' line,
and the work, and whether the read-out's text moved at all.

    python3 moves.py OLD/stdlib out/moves.json

The new stdlib is the checkout's; the binary is $EIN_BIN, else
ein.rs/target/gate/ein."""
import os, re, subprocess, sys, tomllib, json
from pathlib import Path
REPO = Path(__file__).resolve().parents[5]
BIN = os.environ.get("EIN_BIN", str(REPO / "ein.rs/target/gate/ein"))
OLD = str(Path(sys.argv[1]).resolve())
man = tomllib.loads((REPO / "corpus/corpus.toml").read_text())
progs = []
for e in man["entry"]:
    p = REPO / e["path"]
    t = p.read_text()
    if any(w in t for w in ("std.slots", "slot-partition", "std.debts")):   # std.debts imports std.slots
        progs.append(e)
ROWS = ("states (k_s)", "commitments (k_c)", "exhausted", "verdict", "maximal line", "minimal line", "work", "∅", "empty", "over", "count")
def readout(path, args, env):
    p = subprocess.run([BIN, "solve", path, *args], capture_output=True, text=True, env=env, cwd=REPO, timeout=600)
    rows = {}
    qs = []
    for l in (p.stdout).split("\n"):
        m = re.match(r"  (\S.*?)\s{2,}(.*)$", l)
        if not m: continue
        k, v = m.group(1), m.group(2)
        if k == "over": qs.append(v)
        elif k in ROWS: rows[k] = v
    rows["over"] = qs
    return p.returncode, rows, p.stdout
res = []
for e in progs:
    runs = [r for r in e["runs"] if r.startswith("solve")] or ["solve"]
    for r in runs:
        args = r.split()[1:]
        env_old = dict(os.environ, EIN_STDLIB=OLD)
        env_new = dict(os.environ); env_new.pop("EIN_STDLIB", None)
        c0, a, ta = readout(e["path"], args, env_old)
        c1, b, tb = readout(e["path"], args, env_new)
        same_text = ta == tb
        res.append(dict(path=e["path"], run=r, slow=e.get("slow", False), old=a, new=b, same_text=same_text, codes=(c0, c1)))
        diff = {k: (a.get(k), b.get(k)) for k in set(a) | set(b) if a.get(k) != b.get(k)}
        print(f"{e['path']:60} {r:16} text {'same' if same_text else 'DIFFERS'}  {diff if diff else ''}")
Path(sys.argv[2]).parent.mkdir(parents=True, exist_ok=True)
Path(sys.argv[2]).write_text(json.dumps(res, indent=1, ensure_ascii=False))
