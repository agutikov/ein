#!/usr/bin/env python3
"""Every cell of absent_open_not.md, run: `not`, `absent` and `open`, each alone and each inside each,
as a premise (M), a conclusion (A) and a fact a program writes (F); and the probes of open_and_owe.md.

    python3 absent_open_not/run.py [--bin BIN] [--stdlib DIR]

Writes the programs into a fresh temporary directory and prints one line a run."""
import argparse, os, pathlib, re, subprocess, tempfile

ROOT = pathlib.Path(__file__).resolve().parents[4]
ap = argparse.ArgumentParser()
ap.add_argument("--bin", default=os.environ.get("EIN_BIN", str(ROOT / "build/review-bin/ein-head-946ee788")))
ap.add_argument("--stdlib", default=str(ROOT / "build/review-stdlib/946ee788"))
args = ap.parse_args()
env = dict(os.environ, EIN_STDLIB=args.stdlib)
K = ["not", "absent", "open"]

def wrap(ws, inner):
    for w in reversed(ws):
        inner = f"({w} {inner})"
    return inner

def run(path):
    r = subprocess.run([args.bin, "saturate", str(path), "--dump"], capture_output=True, text=True, env=env)
    out = r.stdout + r.stderr
    err = [l.strip() for l in out.split("\n") if l.startswith("kb load error") or re.match(r".*\.ein:\d+:\d+:", l)]
    if err:
        return "REFUSED  " + re.sub(r"^.*?\.ein:", "", err[0])[:200]
    facts, on = [], False
    for l in out.split("\n"):
        if l.startswith(";; ──"):
            on = True
        elif on and l.startswith("  ("):
            facts.append(re.sub(r"\s+:rule \S+\)$", ")", l.strip()))
    return "LOADS    holds " + " ".join(facts)

programs = {}
for ws in [[k] for k in K] + [[o, i] for o in K for i in K]:
    n = "-".join(ws)
    programs[f"{n}.M"] = f'(q A) (p A)\n(rule r () :match (and (q ?x) {wrap(ws, "(p ?x)")}) :assert (hit ?x) :why "r")\n'
    programs[f"{n}.A"] = f'(q A)\n(rule r () :match (q ?x) :assert {wrap(ws, "(p ?x)")} :why "r")\n'
    programs[f"{n}.F"] = f'(q A)\n{wrap(ws, "(p A)")}\n'
# what each premise that loads matches: A has the inner form held, B has not
programs["absent.M2"] = '(q A) (q B) (p A)\n(rule r () :match (and (q ?x) (absent (p ?x))) :assert (hit ?x) :why "r")\n'
programs["absent-absent.M2"] = '(q A) (q B) (p A)\n(rule r () :match (and (q ?x) (absent (absent (p ?x)))) :assert (hit ?x) :why "r")\n'
programs["absent-not.M2"] = '(q A) (q B) (not (p A))\n(rule r () :match (and (q ?x) (absent (not (p ?x)))) :assert (hit ?x) :why "r")\n'
programs["not.M2"] = '(q A) (q B) (not (p A))\n(rule r () :match (and (q ?x) (not (p ?x))) :assert (hit ?x) :why "r")\n'
programs["not-not.M2"] = '(q A) (q B) (not (not (p A)))\n(rule r () :match (and (q ?x) (not (not (p ?x)))) :assert (hit ?x) :why "r")\n'
# open_and_owe.md § 2: the user's form, and a requirement held as a fact
programs["owe-hole"] = '(is-a Ann Guest) (is-a C1 Chair)\n(rule need (?R) :match (is-a ?a Guest) :assert (owe (?R ?a ?_x)) :why "r")\n(need seats)\n'
programs["owe-held"] = """(is-a Ann Guest) (is-a Bob Guest) (is-a C1 Chair) (is-a C2 Chair)
(seats Ann C1)
(rule need (?R ?A ?B) :match (is-a ?a ?A) :assert (owe ?R ?a ?B) :why "r")
(rule paid (?R ?A ?B) :match (and (owe ?R ?a ?B) (is-a ?b ?B) (?R ?a ?b)) :assert (paid ?R ?a ?B) :why "r")
(rule unpaid () :match (and (owe ?R ?a ?B) (absent (paid ?R ?a ?B))) :assert (still-owed ?R ?a ?B) :why "r")
(need seats Guest Chair) (paid seats Guest Chair)
"""
with tempfile.TemporaryDirectory() as d:
    for name, text in programs.items():
        p = pathlib.Path(d) / f"{name}.ein"
        p.write_text(text)
        print(f"{name:18} {run(p)}")
