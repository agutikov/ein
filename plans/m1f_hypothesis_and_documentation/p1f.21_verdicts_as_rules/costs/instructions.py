#!/usr/bin/env python3
"""Instructions executed, user space, by one `ein solve FILE --jobs 1` pinned to the performance cores, on the
final table's engines and master before the key's stage, each on the corpus and the standard library of its own commit.

Scratch: an A/B for the coordinator, not a tool of the repository. Run under utils/bench_env.sh.

    instructions.py OUT.tsv [RUNS]

`perf stat -x, -e instructions:u taskset -c 0-15 ein solve FILE --jobs 1`: the two core types' lines summed (the
E-core counter counts next to nothing under the pin), `RUNS` runs (3), the mean. A row is the program, the point, the
instructions and the saturations the run's summary reports.
"""
import json, os, statistics, subprocess, sys
from pathlib import Path

from three_engines import HERE, POINTS, REPO

# The final table's engines, and master before the key was read at its own size, on the same tree as the last.
POINTS = [*POINTS[:-1], ("master", REPO / "build/costs/bin/ein-9994ac94", HERE), POINTS[-1]]

PROGRAMS = [
    "examples/beyond_arith/equation_two_unknowns.ein",
    "examples/saturation/type-exclusivity/pets.ein",
    "examples/features/01_not_and_absent.ein",
    "examples/beyond_arith/send_more_money.ein",
    "examples/zebra2.ein",
]
OUT = Path(sys.argv[1])
RUNS = int(sys.argv[2]) if len(sys.argv) > 2 else 3
cell = REPO / "build/costs/final/cell"
cell.mkdir(parents=True, exist_ok=True)

rows = []
for path in PROGRAMS:
    for label, binary, tree in POINTS:
        env = dict(os.environ, EIN_STDLIB=str(tree / "stdlib"))
        counts = []
        for _ in range(RUNS):
            p = subprocess.run(
                ["perf", "stat", "-x,", "-e", "instructions:u", "taskset", "-c", "0-15",
                 str(binary), "solve", path, "--jobs", "1"],
                cwd=tree, env=env, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                text=True)
            n = 0
            for line in p.stderr.splitlines():
                f = line.split(",")
                if len(f) > 2 and "instructions" in f[2] and f[0].isdigit():
                    n += int(f[0])
            counts.append(n)
        subprocess.run([str(binary), "solve", path, "--jobs", "1", "--json-summary", str(cell / "summary.json")],
                       cwd=tree, env=env, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL,
                       stderr=subprocess.DEVNULL)
        sat = (json.load(open(cell / "summary.json")).get("work") or {}).get("saturations")
        mean = round(statistics.fmean(counts))
        spread = (max(counts) - min(counts)) / mean if mean else 0
        rows.append((path, label, mean, sat))
        print(f"{label} {path}: {mean} instructions ({spread:.4%} spread over {RUNS}), {sat} saturations",
              flush=True)
with open(OUT, "w") as f:
    for r in rows:
        f.write("\t".join(str(x) for x in r) + "\n")
