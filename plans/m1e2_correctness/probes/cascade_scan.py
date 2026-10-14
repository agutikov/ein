#!/usr/bin/env python3
"""Corpus scan: does the forced-positive cascade change any verdict?
Runs every entry of scan_all.json under `solve -e` twice — as written, and with
`:enable-forced-positive false` merged into the program's own (config …) block
(a later block replaces an earlier one wholesale, so the key is spliced in).
Scratch instrument: copies to a temp dir, never writes the repo."""
import json, os, re, subprocess, sys, tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../.."))
EIN = os.path.join(ROOT, "ein.rs/target/release/ein")


def with_cfg(src, kv):
    i = src.rfind("(config")
    if i < 0:
        return src + "\n(config %s)\n" % kv
    return src[:i] + "(config %s " % kv + src[i + len("(config"):]


def run(text):
    with tempfile.TemporaryDirectory() as td:
        p = os.path.join(td, "probe.ein")
        open(p, "w").write(text)
        js = os.path.join(td, "s.json")
        try:
            subprocess.run([EIN, "solve", "-e", p, "--json-summary", js], cwd=ROOT,
                           capture_output=True, text=True, timeout=90)
        except subprocess.TimeoutExpired:
            return None
        if not os.path.exists(js):
            return None
        d = json.load(open(js))
        v = d["verdict"]
        st = d["stats"]
        return (v["type"], v["k"], v["exhausted"], st.get("forced_positives"),
                st.get("enterings_total"), len(v.get("unsat_core") or []))


scan = json.load(open(sys.argv[1]))
out = []
for rec in scan:
    if rec.get("error"):
        continue
    src = open(os.path.join(ROOT, rec["path"])).read()
    a = run(src)
    b = run(with_cfg(src, ":enable-forced-positive false"))
    flag = ""
    if a and b and (a[0], a[1]) != (b[0], b[1]):
        flag = "DIFF"
    out.append({"path": rec["path"], "default": a, "fp_off": b, "flag": flag})
    if flag or (a and a[3]):
        print(rec["path"], a, b, flag, flush=True)
json.dump(out, open(sys.argv[2], "w"), indent=1)
