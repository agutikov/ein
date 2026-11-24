#!/usr/bin/env python3
"""Cost of chosen corpus entries on the final table's builds of the engine, each on the corpus of its own commit.

Scratch: an A/B for the coordinator, not a tool of the repository. Run under utils/bench_env.sh.

    three_engines.py OUT.json ENTRY...

The engines' binaries and the old commits' trees (corpus, examples, tests, stdlib, unpacked by `git archive`) are
the main checkout's `build/costs/`; the last point's tree is the checkout this script is in.
"""
import json, os, statistics, subprocess, sys, time, tomllib
from pathlib import Path

REPO = Path("/home/user/work/ein")
HERE = Path(__file__).resolve().parents[4]
POINTS = [  # label, binary, tree
    ("before", REPO / "build/costs/bin/ein-edbe6c3b", REPO / "build/costs/tree-edbe6c3b"),
    ("after", REPO / "build/costs/bin/ein-1655f19d", REPO / "build/costs/tree-1655f19d"),
    ("improved", REPO / "build/costs/bin/ein-fa2c2914", REPO / "build/costs/tree-fa2c2914"),
    ("key", REPO / "build/costs/bin/ein-24aeebbe", HERE),
]


def argv_for(run, file, out):
    toks = [t.replace("{out}", str(out)) for t in run.split()]
    argv = [toks[0], *toks[1:2], file, *toks[2:]] if toks[0] == "render" else [toks[0], file, *toks[1:]]
    if toks[0] == "solve":
        argv += ["--json-summary", f"{out}/summary.json"]
    return argv


def main():
    entries = sys.argv[2:]
    out_path = Path(sys.argv[1])
    head = {e["path"]: e for e in tomllib.load(open(HERE / "corpus/corpus.toml", "rb"))["entry"]}
    rows = []
    for path in entries:
        runs = head[path].get("runs", [])
        for label, binary, tree in POINTS:
            if not (tree / path).exists():
                rows.append({"path": path, "point": label, "missing": True}); continue
            env = dict(os.environ, EIN_STDLIB=str(tree / "stdlib"))
            for run in runs:
                out = REPO / "build/costs/final/cell"; out.mkdir(parents=True, exist_ok=True)
                argv = [str(binary), *argv_for(run, path, out)]
                samples, code = [], None
                for i in range(4):
                    t0 = time.perf_counter()
                    p = subprocess.run(argv, cwd=tree, env=env, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
                    dt = time.perf_counter() - t0; code = p.returncode
                    if i == 0 and dt < 2: continue      # the warm-up, as corpus_cost.py discards it
                    samples.append(dt)
                    if sum(samples) > 30: break
                sat = None
                if run == "solve":
                    try:
                        s = json.load(open(out / "summary.json"))
                        sat = dict(s.get("work") or {}, k_s=(s.get("region") or {}).get("k_s"), monotone=(s.get("monotonicity") or {}).get("monotone"))
                    except Exception as e:
                        sat = {"error": str(e)[:60]}
                rows.append({"path": path, "point": label, "run": run, "code": code, "mean_ms": round(statistics.fmean(samples) * 1e3, 1),
                             "min_ms": round(min(samples) * 1e3, 1), "n": len(samples), "summary": sat})
                print(f"{label} {path} :: {run}: {rows[-1]['mean_ms']} ms (code {code})", flush=True)
    json.dump(rows, open(out_path, "w"), indent=1)


if __name__ == "__main__":
    main()
