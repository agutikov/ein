#!/usr/bin/env python3
"""Write the detector's random programs: two hundred programs, each written by
`region_synth.py` for a random region, banked as one test fixture.

    python3 utils/gen_random_programs.py            # write the fixture
    python3 utils/gen_random_programs.py --check    # exit 1 if it differs

`ein-infer/tests/detector.rs`' `a_monotone_verdict_is_one_state_per_node_on_random_programs`
sweeps every program of the fixture the static check calls monotone, by fork,
by every split and from scratch. The programs are `region_synth.py --random 200
--seed 5 --max-n 6 --write`'s, each with its `;;;` header dropped (the target
it was written for, which the sweep does not read), and each begins at a line
`;;; program NAME`. The fixture says it is generated and names no tool; this
script is where that is written down, with `utils/README.md`. The gate runs
`--check` (`utils/check_generated.py`).
"""
from __future__ import annotations

import argparse
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import region_synth  # noqa: E402

REPO = Path(__file__).resolve().parents[1]
OUT = REPO / "ein.rs" / "crates" / "ein-infer" / "tests" / "golden" / "random_programs.txt"
COUNT, SEED, MAX_N = 200, 5, 6

HEADER = (
    ";;; Programs written for random regions, over at most six candidates, for the\n"
    ";;; detector's sweep. Generated, not edited by hand: an edit is lost the next\n"
    ";;; time the file is written. Each program begins at a line `;;; program NAME`.\n"
)


def render() -> str:
    rng = random.Random(SEED)
    out = [HEADER]
    for i in range(COUNT):
        spec = region_synth.random_spec(rng, rng.randint(1, MAX_N), i)
        try:
            text = region_synth.program(spec)
        except region_synth.RootDead:
            continue
        body = "\n".join(l for l in text.split("\n") if not l.startswith(";;;")).strip("\n")
        out.append(f";;; program {spec['name']}\n{body}\n")
    return "".join(out)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--check", action="store_true", help="exit 1 if the fixture differs")
    args = ap.parse_args()
    text = render()
    if args.check:
        if not OUT.exists() or OUT.read_text(encoding="utf-8") != text:
            print(f"{OUT.relative_to(REPO)}: differs from `python3 utils/gen_random_programs.py`",
                  file=sys.stderr)
            return 1
        return 0
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(text, encoding="utf-8")
    print(f"wrote {OUT.relative_to(REPO)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
