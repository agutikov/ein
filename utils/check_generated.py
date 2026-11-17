#!/usr/bin/env python3
"""check_generated.py — every generated project file is what its generator writes.

    python3 utils/check_generated.py [--bin ein.rs/target/gate/ein]

A generated file in the project says it is generated and names no tool; which
script writes it is recorded here and in `utils/README.md`. This runs each
generator's `--check`, which writes nothing and exits 1 where a file differs
from what the script would write, so a hand edit, or an edit to a source the
script reads that was not followed by a regeneration, fails the gate. It is a
step of the gate after the tests, since two generators write pages from the
binary the tests ran.

| generator | what it writes |
|---|---|
| `gen_arith_tables.py` | `stdlib/math/digits.ein` and `stdlib/math/bits.ein`, the calculator's tables |
| `gen_num9.py` | `stdlib/math/num9.ein`, and the theory at width 2, `ein-infer/tests/golden/num2_theory.ein` |
| `gen_gates.py` | `stdlib/bool/gates.ein`'s rows and the programs under `tests/stdlib/bool/gates/` and `props/` |
| `gen_arith_examples.py` | three `examples/arith/` programs |
| `gen_zebra2_variants.py` | the four `zebra2` variants under `examples/` |
| `gen_send_written.py` | `examples/beyond_arith/send_more_money_written.ein` |
| `gen_random_programs.py` | `ein-infer/tests/golden/random_programs.txt`, the detector's random programs |
| `read_out_examples.py` | `docs/ein/user/04-reference/09-read-out-examples.md`, from the binary (`--bin`) |
| `gen_stdlib_rule_pages.py` | the rules' regions of the pages under `docs/ein/user/04-reference/04-stdlib/`, each rule as its module writes it, and the sources of the rules' drawings under that folder's `diagrams/`, each as the binary draws its rule (`--bin`) |

A generator whose `--check` fails prints what differs; the fix is to run it
without `--check` (and, for a stdlib module, `stdlib_manifest.py --write`).
The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`.
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
UTILS = Path(__file__).resolve().parent

STATIC = [
    "gen_arith_tables.py",
    "gen_num9.py",
    "gen_gates.py",
    "gen_arith_examples.py",
    "gen_zebra2_variants.py",
    "gen_send_written.py",
    "gen_random_programs.py",
]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--bin", default=os.environ.get("EIN_BIN",
                                                    str(ROOT / "ein.rs/target/release/ein")))
    args = ap.parse_args()
    runs = [[script, "--check"] for script in STATIC]
    runs.append(["read_out_examples.py", "--check", "--bin", str(Path(args.bin).resolve())])
    runs.append(["gen_stdlib_rule_pages.py", "--check", "--bin", str(Path(args.bin).resolve())])
    stale = []
    for run in runs:
        r = subprocess.run([sys.executable, str(UTILS / run[0]), *run[1:]], cwd=ROOT,
                           capture_output=True, text=True)
        if r.returncode != 0:
            stale.append(run[0])
            print(f"check_generated: `{' '.join(run)}` exits {r.returncode}:\n{r.stdout}{r.stderr}",
                  file=sys.stderr)
    if stale:
        print(f"check_generated: {len(stale)} generator(s) would write otherwise: "
              f"{', '.join(stale)}", file=sys.stderr)
        return 1
    print(f"check_generated: {len(runs)} generators, every file they write as written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
