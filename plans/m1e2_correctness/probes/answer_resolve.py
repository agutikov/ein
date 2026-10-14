#!/usr/bin/env python3
"""M1e2 S1e2.1.2 — does each `--dump-answer` state file re-solve to itself?

Over every run `corpus/wrong_answers.toml` records, plus `zebra2 -e`: each
`models/NN.ein` re-solved with the run's own flags should answer `Solution`,
k = 1, with exactly the file's state; each `root.ein` the original verdict and k.
Taken 2026-10-14: 63/63 and 42/42 (docs/kernel/inference/answer_dump.md).
Run from the repo root; `EIN_BIN` overrides the binary.
"""
import json, os, subprocess, sys, tomllib, tempfile, shutil
E = os.environ.get("EIN_BIN", "ein.rs/target/release/ein")
reg = tomllib.load(open("corpus/wrong_answers.toml", "rb"))
runs = set()
for r in reg["case"]:
    for a in [r["wrong"]] + ([r["control"]] if "control" in r else []) + r.get("twin", []):
        runs.add((a.get("witness", r["witness"]), a["run"], tuple(a.get("env", []))))
runs.add(("examples/zebra2.ein", "solve -e", ()))
def solve(f, run, env, extra):
    t = run.split()
    d = tempfile.mkdtemp()
    p = subprocess.run([E, t[0], f, *t[1:], *extra, "--json-summary", d + "/s.json"], capture_output=True, text=True,
                       env={**os.environ, **dict(e.split("=", 1) for e in env)})
    try: s = json.load(open(d + "/s.json"))
    except FileNotFoundError: s = None
    shutil.rmtree(d); return p.returncode, s
def key(s):
    v = s["verdict"]; return (v["type"], v["k"], sorted(tuple(m["facts"]) for m in v["solutions"] + v["open_states"]))
model_ok = model_n = root_ok = root_n = 0
odd = []
for f, run, env in sorted(runs):
    d = tempfile.mkdtemp()
    t = run.split()
    subprocess.run([E, t[0], f, *t[1:], "--dump-answer", d], capture_output=True, env={**os.environ, **dict(e.split("=", 1) for e in env)})
    if not os.path.exists(d + "/index.json"): odd.append((f, run, "no dump")); continue
    idx = json.load(open(d + "/index.json"))
    orig = json.load(open(d + "/answer.json"))
    for stem in idx["models"]:
        model_n += 1
        rec = json.load(open(f"{d}/{stem}.json"))
        code, s = solve(f"{d}/{stem}.ein", run, env, [])
        if s and s["verdict"]["type"] == "Solution" and s["verdict"]["solutions"][0]["facts"] == rec["facts"]:
            model_ok += 1
        else:
            odd.append((f, run, stem, s and (s["verdict"]["type"], s["verdict"]["k"])))
    root_n += 1
    code, s = solve(f"{d}/root.ein", run, env, [])
    if s and key(s)[:2] == key(orig)[:2]:
        root_ok += 1
    else:
        odd.append((f, run, "root", s and (s["verdict"]["type"], s["verdict"]["k"]), (orig["verdict"]["type"], orig["verdict"]["k"])))
    shutil.rmtree(d)
print(f"models re-solve to themselves: {model_ok}/{model_n}; root re-solves to the same verdict and k: {root_ok}/{root_n}")
for o in odd: print("  ", o)
