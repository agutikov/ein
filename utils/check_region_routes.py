#!/usr/bin/env python3
"""check_region_routes.py — the oracle's region against two routes it does not share code with.

    python3 utils/check_region_routes.py [--bin ein.rs/target/gate/ein]

`ein oracle FILE --dump DIR` computes a program's region, the n-cube over its
candidates with every node classified, in the library. Two scripts here
reach the same answer another way, and this check holds the binary to both,
as a step of the gate after the tests (it needs the binary the tests ran):

1. **The per-subset route**, `region.py`: one `ein saturate` process a node,
   on the seven programs of Q-M1e2.7's tables, a program where saturation
   moves a node inside the cube, one whose dead nodes die by a pair `X`,
   `(not X)`, and one with a hole. Node for node: every node's status (dead,
   owing n, or in R), every alive node's closure, the two lines, the holes,
   the components' sizes, the maximal sub-cubes, N, and the N side of the
   duality. The one of the seven *forbid* refuses (Q-M1e2.5) is refused by
   the oracle, exit 1, and dumps nothing.
2. **The synthesizer**, `region_synth.py --random 300 --seed 1 --max-n 6`:
   three hundred regions drawn at random, half with implications so that
   saturation moves nodes, each written as a program; the oracle must give
   back the drawn region node for node, and call the program monotone
   exactly when the region is.

The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
UTILS = Path(__file__).resolve().parent

# The seven programs of Q-M1e2.7's tables.
SEVEN = [
    "examples/branching/15_lookahead_two_step_on.ein",
    "examples/lattice/02_genuine_3set_death.ein",
    "examples/ein-bugs/wrong-answers/forced-2-two.ein",
    "examples/ein-bugs/wrong-answers/owed-two-step.ein",
    "examples/ein-bugs/wrong-answers/owed-seats.ein",
    "examples/lattice/05_owed_seats_bounded.ein",
    "examples/ein-bugs/naf-upward-closure.ein",
]
# The one of the seven *forbid* refuses.
REFUSED = "examples/ein-bugs/naf-upward-closure.ein"
# Saturation moves a node inside the cube, so the closure check is not vacuous.
MOVES = "examples/saturation/transitive/taxonomy.ein"
# Dead nodes die by a pair, which derives no `(false)` fact.
PAIR = "examples/branching/09_hrule.ein"
# A hole: an obligation's trigger, owing.
HOLE = "examples/lattice/04_obligation_trigger_hole.ein"


def sets(v) -> list[list[str]]:
    return sorted(sorted(s) for s in v)


def one_program(file: str, bin: str, scratch: Path) -> tuple[list[str], bool]:
    """The differences between the oracle's dump and `region.py` on `file`,
    and whether any alive node's closure moved."""
    d = scratch / Path(file).stem
    r = subprocess.run([bin, "oracle", file, "--dump", str(d)], cwd=ROOT,
                       capture_output=True, text=True)
    if file == REFUSED:
        bad = []
        if r.returncode != 1 or not r.stderr.startswith("rule 'bad': "):
            bad.append(f"{file}: forbid's refusal expected, got exit {r.returncode}: {r.stderr}")
        if (d / "region.json").exists():
            bad.append(f"{file}: a refusal dumps nothing")
        return bad, False
    if r.returncode != 0:
        return [f"{file}: the oracle exits {r.returncode}: {r.stderr}"], False
    region = json.loads((d / "region.json").read_text(encoding="utf-8"))
    cands = list(region["candidates"])
    py = subprocess.run([sys.executable, "-I", str(UTILS / "region.py"), "--json",
                         str(ROOT / file), *cands], cwd=ROOT, capture_output=True, text=True,
                        env={**os.environ, "EIN_BIN": bin})
    if py.returncode != 0:
        return [f"{file}: region.py exits {py.returncode}: {py.stderr}"], False
    py = json.loads(py.stdout)
    rg = region["region"]
    bad = []

    def status(n) -> str:
        if n["status"] == "R":
            return "ok"
        return f"owes {n['owes']}" if n["status"] == "owing" else "dead"

    ours = sorted((sorted(n["facts"]), status(n)) for n in rg["nodes"])
    theirs = sorted((list(n["facts"]), n["status"]) for n in py["nodes"])
    if ours != theirs:
        bad.append(f"{file}: the nodes differ")

    def named(mask: int) -> list[str]:
        return sorted(cands[b] for b in range(len(cands)) if mask >> b & 1)

    moved = False
    ours_c = []
    for n in rg["nodes"]:
        if n["status"].startswith("dead"):
            continue
        if n["closure"] != n["mask"]:
            moved = True
        ours_c.append((named(n["mask"]), named(n["closure"])))
    theirs_c = [(list(n["facts"]), list(n["closure"])) for n in py["nodes"]
                if n["closure"] is not None]
    if sorted(ours_c) != sorted(theirs_c):
        bad.append(f"{file}: the closures differ")
    for key in ["minimal", "maximal", "holes", "nogoods"]:
        if sets(rg[key]) != sets(py[key]):
            bad.append(f"{file}: {key} differ: {sets(rg[key])} against {sets(py[key])}")
    if sorted(c["size"] for c in rg["components"]) != list(py["components"]):
        bad.append(f"{file}: the components differ")
    ours_s = sorted((sorted(c["from"]), sorted(c["to"])) for c in rg["subcubes"])
    theirs_s = sorted((sorted(c[0]), sorted(c[1])) for c in py["subcubes"])
    if ours_s != theirs_s:
        bad.append(f"{file}: the sub-cubes differ")
    if rg["duality"]["maximal_holds"] != py["duality_n"]:
        bad.append(f"{file}: the N side of the duality differs")
    return bad, moved


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--bin", default=os.environ.get("EIN_BIN",
                                                    str(ROOT / "ein.rs/target/release/ein")))
    args = ap.parse_args()
    bin = str(Path(args.bin).resolve())
    bad = []
    moved = False
    with tempfile.TemporaryDirectory(prefix="check-region-routes-") as tmp:
        for file in SEVEN + [MOVES, PAIR, HOLE]:
            b, m = one_program(file, bin, Path(tmp))
            bad.extend(b)
            moved |= m
    if not moved:
        bad.append("no node moved: the closure check compared identities")
    synth = subprocess.run([sys.executable, "-I", str(UTILS / "region_synth.py"),
                            "--random", "300", "--seed", "1", "--max-n", "6", "--bin", bin],
                           cwd=ROOT, capture_output=True, text=True)
    if synth.returncode != 0 or "300 targets, seed 1, 0 failed" not in synth.stdout:
        bad.append(f"region_synth.py --random 300 --seed 1 --max-n 6 exits {synth.returncode}:\n"
                   f"{synth.stdout}{synth.stderr}")
    for b in bad:
        print(f"check_region_routes: {b}", file=sys.stderr)
    if bad:
        print(f"check_region_routes: {len(bad)} finding(s)", file=sys.stderr)
        return 1
    print("check_region_routes: the oracle agrees with the per-subset route on 10 programs, "
          "and gives back 300 random regions written as programs")
    return 0


if __name__ == "__main__":
    sys.exit(main())
