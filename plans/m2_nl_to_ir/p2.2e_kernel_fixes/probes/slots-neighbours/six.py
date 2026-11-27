#!/usr/bin/env python3
"""The six Zebra encodings, whole and with sentence (15) dropped, under one
binary and one stdlib: --maximal's count, the answer query's classes folded to
(?who_water, ?who_zebra), the work; and --single on the whole puzzle.

    python3 six.py out/after                          # the checkout's stdlib
    python3 six.py out/before --stdlib OLD/stdlib     # another, e.g. `git archive 0ab06c69 stdlib | tar -x -C OLD`

The binary is $EIN_BIN, else ein.rs/target/gate/ein. Each variant is the
example with the one form whose :source is sentence (15)'s taken out, written
to the output folder beside its --maximal dump."""
import collections, json, os, re, shutil, subprocess, sys
from pathlib import Path

REPO = Path(__file__).resolve().parents[5]
OUT = Path(sys.argv[1])            # where variants and dumps go
OUT.mkdir(parents=True, exist_ok=True)
ENV = dict(os.environ)
if "--stdlib" in sys.argv:
    ENV["EIN_STDLIB"] = str(Path(sys.argv[sys.argv.index("--stdlib") + 1]).resolve())
BIN = os.environ.get("EIN_BIN", str(REPO / "ein.rs/target/gate/ein"))
ENC = ["zebra", "zebra3.1", "zebra3.2", "zebra3.3", "zebra2", "zebra3.4"]
SRC15 = ('"condition (15)"', '"The Norwegian lives next to the blue house."')

def drop15(text):
    lines = text.split("\n")
    hit = [i for i, l in enumerate(lines) if any(s in l for s in SRC15)]
    assert len(hit) == 1, hit
    i = hit[0]
    start = i
    while not lines[start].startswith("("):
        start -= 1
    depth, end, instr = 0, None, False
    for j in range(start, len(lines)):
        k = 0; l = lines[j]
        while k < len(l):
            c = l[k]
            if instr:
                if c == "\\": k += 1
                elif c == '"': instr = False
            elif c == '"': instr = True
            elif c == ";": break
            elif c == "(": depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0: end = j; break
            k += 1
        if end is not None: break
    removed = "\n".join(lines[start:end+1])
    return "\n".join(lines[:start] + lines[end+1:]), removed

def classes(answer_txt):
    t = answer_txt.read_text()
    q = t.index("\n  query ")
    tally = collections.Counter(); n = 0
    cur = None
    for line in t[q:].split("\n"):
        m = re.match(r"\s+class \d+ of \d+\s+(\d+) states? ", line)
        if m: cur = int(m.group(1)); continue
        if cur is not None and "?who_water" in line:
            b = dict(x.strip().split(" = ", 1) for x in line.strip().split(" · "))
            tally[(b["?who_water"], b["?who_zebra"])] += cur; n += cur; cur = None
    return tally, n

def run(args):
    p = subprocess.run([BIN, "solve", *args], capture_output=True, text=True, env=ENV)
    return p.stdout + p.stderr

res = {}
for e in ENC:
    src = (REPO / "examples" / f"{e}.ein").read_text()
    var, removed = drop15(src)
    f = OUT / f"{e}.drop-15.ein"
    f.write_text(var)
    d = OUT / f"dump-{e}"
    shutil.rmtree(d, ignore_errors=True)
    out = run([str(f), "--maximal", "--dump-answer", str(d)])
    mx = re.search(r"maximal line\s+(\d+) states?", out).group(1)
    work = re.search(r"work\s+(.*)", out).group(1)
    tally, n = classes(d / "answer.txt")
    single = run([str(REPO / "examples" / f"{e}.ein"), "--single"])
    verdict = re.search(r"verdict\s+(\w+)", single).group(1)
    swork = re.search(r"work\s+(.*)", single).group(1)
    res[e] = dict(maximal=int(mx), work=work, classes=len(tally), states_in_classes=n,
                  tally={f"{a}/{b}": v for (a, b), v in sorted(tally.items())},
                  full_single=verdict, full_work=swork, removed=removed)
    print(f"{e:9} drop-15 --maximal {mx:>3} states, {len(tally):>2} classes ({n} states) · {work:40} | whole --single {verdict} · {swork}")
(OUT / "six.json").write_text(json.dumps(res, indent=1))
base = res["zebra2"]["tally"]
for e in ENC:
    print(e, "classes equal zebra2's:", res[e]["tally"] == base)
