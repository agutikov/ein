#!/usr/bin/env python3
"""The review bundle — M1e2 P1e2.1.

    utils/review_bundle.py run FILE [SOLVE FLAGS…] --out DIR [--env NAME=V]…

One run, fully explicit (S1e2.1.5): every artefact `ein` can write for
`ein solve FILE FLAGS`, kept verbatim under DIR/raw/ and rendered as pages
under DIR/pages/, with DIR/run.md as the table of contents. Every graph is an
SVG, every JSON a table, every snapshot pair a diff, and DIR/pages/consistency.md
checks the artefacts against each other.

The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`.
Graphviz (`dot`) must be on PATH.
"""

from __future__ import annotations

import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from review.run import RunSpec, build_run  # noqa: E402
from review.util import REPO, ein_bin  # noqa: E402


def take(args: list[str], flag: str, many: bool = False):
    vals = []
    while flag in args:
        i = args.index(flag)
        vals.append(args[i + 1])
        del args[i:i + 2]
    return vals if many else (vals[-1] if vals else None)


def cmd_run(args: list[str]) -> int:
    bin = ein_bin(take(args, "--bin"))
    out = take(args, "--out")
    env = dict(e.split("=", 1) for e in take(args, "--env", many=True))
    no_events = "--no-events" in args and not args.remove("--no-events")
    no_trace = "--no-trace" in args and not args.remove("--no-trace")
    if not args or out is None:
        print(__doc__, file=sys.stderr)
        return 2
    file, flags = args[0], args[1:]
    path = Path(file)
    if path.is_absolute():
        file = str(path.relative_to(REPO)) if path.is_relative_to(REPO) else file
    t = time.time()
    r = build_run(RunSpec(file, flags, env, events=not no_events, trace=not no_trace), Path(out).resolve(), bin)
    fails = [c for c in r.checks if c[1] is False]
    applicable = [c for c in r.checks if c[1] is not None]
    print(f"{r.out}/run.md — {r.verdict}, k = {r.k}, exhausted = {r.exhausted}; "
          f"{len(r.pages)} pages; {len(applicable) - len(fails)}/{len(applicable)} cross-checks pass; "
          f"{time.time() - t:.1f} s")
    for c, _, d in fails:
        print(f"  MISMATCH {c}: {d}")
    return 0


def main(argv: list[str]) -> int:
    if not argv or argv[0] in ("-h", "--help"):
        print(__doc__)
        return 0
    if argv[0] == "run":
        return cmd_run(argv[1:])
    print(f"unknown command {argv[0]!r}\n{__doc__}", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
