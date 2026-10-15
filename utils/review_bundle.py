#!/usr/bin/env python3
"""The review bundle — M1e2 P1e2.1.

    utils/review_bundle.py [--out DIR] [CASE | FILE.ein …]      the bundle
    utils/review_bundle.py --corpus [--out DIR]                 every corpus entry
    utils/review_bundle.py --smoke                              the gate's check
    utils/review_bundle.py --regions [--out DIR]                the region gallery (build/review/regions/)
    utils/review_bundle.py run FILE [SOLVE FLAGS…] --out DIR [--env NAME=V]…

**The bundle** (S1e2.1.6), default `build/review/`: an index, and one page per
row of `corpus/wrong_answers.toml` — the wrong run, its control and twins
side by side with the right answer, the models diffed, links to where it went
wrong (the row's `look_at`), every artefact two clicks away, and the
checklist the review answers. A CASE is a row id (`W1/branching-15`) or a
case (`W1`, every row of it); a `.ein` path gets a page of its own with one
`solve -e` run (flags after `--`). `--corpus` builds one for every corpus entry
with a declared `solve` run, at that run's flags and without events.
`--smoke` builds one small case into a temporary directory and link-checks
it: the per-commit gate's step, using the debug `ein` `cargo test` built.
A case that fails to build makes the command exit 1.
`--regions` (S1e2.1a.8) surveys every corpus program's region and writes a
gallery: the synthetic programs `utils/region_synth.py` wrote, the biggest
regions and the most interesting, each with a page (`review/regions.py`).

One run, fully explicit (S1e2.1.5): every artefact `ein` can write for
`ein solve FILE FLAGS`, kept verbatim under DIR/raw/ and rendered as pages
under DIR/pages/, with DIR/run.md as the table of contents. Every graph is an
SVG, every JSON a table, every snapshot pair a diff, and DIR/pages/consistency.md
checks the artefacts against each other.

The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`.
Graphviz (`dot`) must be on PATH.
"""

from __future__ import annotations

import os
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


def cmd_bundle(args: list[str]) -> int:
    from review.case import case_page, index_page, program_case, register
    bin = ein_bin(take(args, "--bin"))
    out = Path(take(args, "--out") or REPO / "build/review").resolve()
    flags = []
    if "--" in args:
        flags = args[args.index("--") + 1:]
        args = args[:args.index("--")]
    rows = register()
    chosen, files = [], []
    for a in args:
        if a.endswith(".ein"):
            files.append(a)
            continue
        hit = [r for r in rows if r["id"] == a or r["case"] == a]
        if not hit:
            print(f"error: {a!r} is neither a register row, a case, nor a .ein file", file=sys.stderr)
            return 2
        chosen += [r for r in hit if r not in chosen]
    if not args:
        chosen = rows
    out.mkdir(parents=True, exist_ok=True)
    built, extra, failed = [], [], []
    t = time.time()
    for row in chosen:
        try:
            page, cols = case_page(row, out / row["id"], bin)
            built.append((row, page, cols))
            print(f"{row['id']:32s} {cols[0].result.verdict} k={cols[0].result.k} — {page.relative_to(out)}")
        except Exception as e:  # noqa: BLE001 — a case that fails is reported, not fatal
            failed.append((row["id"], f"{type(e).__name__}: {e}"))
            print(f"{row['id']:32s} FAILED: {e}", file=sys.stderr)
    for f in files:
        rel_file = str(Path(f).resolve().relative_to(REPO)) if Path(f).resolve().is_relative_to(REPO) else f
        try:
            page, r = program_case(rel_file, flags or ["-e"], out / "programs" / rel_file.removesuffix(".ein"), bin)
            extra.append((rel_file, page, r))
            print(f"{rel_file:32s} {r.verdict} k={r.k}")
        except Exception as e:  # noqa: BLE001
            failed.append((rel_file, f"{type(e).__name__}: {e}"))
    index = index_page(out, built, extra, failed, bin)
    print(f"{index} — {len(built)} cases, {len(extra)} programs, {len(failed)} failed; {time.time() - t:.1f} s")
    return 1 if failed else 0


def cmd_corpus(args: list[str]) -> int:
    import tomllib
    from review.case import index_page, program_case
    bin = ein_bin(take(args, "--bin"))
    out = Path(take(args, "--out") or REPO / "build/review-corpus").resolve()
    corpus = tomllib.loads((REPO / "corpus/corpus.toml").read_text(encoding="utf-8"))
    extra, failed = [], []
    for e in corpus["entry"]:
        run_name = next((r for r in e.get("runs", []) if r.split()[0] == "solve"), None)
        if run_name is None or e.get("slow"):
            continue
        flags = [f for f in run_name.split()[1:] if "{out}" not in f]
        try:
            page, r = program_case(e["path"], flags, out / "programs" / e["path"].removesuffix(".ein"), bin,
                                   events=False, timeout_note=f"The corpus's first declared run: `{run_name}`.")
            extra.append((e["path"], page, r))
        except Exception as ex:  # noqa: BLE001
            failed.append((e["path"], f"{type(ex).__name__}: {ex}"))
    index = index_page(out, [], extra, failed, bin)
    print(f"{index} — {len(extra)} programs, {len(failed)} failed")
    return 1 if failed else 0


def cmd_smoke(args: list[str]) -> int:
    """One small register case into a temporary directory, link-checked, with
    every SVG non-empty — the per-commit gate's step (S1e2.1.6)."""
    import subprocess
    import tempfile
    debug = REPO / "ein.rs/target/debug/ein"
    bin = take(args, "--bin") or os.environ.get("EIN_BIN") or str(debug)
    with tempfile.TemporaryDirectory(prefix="ein-review-smoke-") as tmp:
        code = cmd_bundle(["--bin", bin, "--out", tmp, "W2/forced-2"])
        if code:
            return code
        svgs = list(Path(tmp).rglob("*.svg"))
        empty = [p for p in svgs if p.stat().st_size == 0]
        if not svgs or empty:
            print(f"error: {len(svgs)} SVGs, {len(empty)} empty", file=sys.stderr)
            return 1
        check = subprocess.run([sys.executable, str(REPO / "utils/doc_audit.py"), "--links", "--check", tmp])
        print(f"smoke: {len(svgs)} SVGs, link check exit {check.returncode}")
        return check.returncode


def main(argv: list[str]) -> int:
    if argv and argv[0] in ("-h", "--help"):
        print(__doc__)
        return 0
    if argv and argv[0] == "run":
        return cmd_run(argv[1:])
    if "--corpus" in argv:
        argv.remove("--corpus")
        return cmd_corpus(argv)
    if "--smoke" in argv:
        argv.remove("--smoke")
        return cmd_smoke(argv)
    if "--regions" in argv:
        argv.remove("--regions")
        from review.regions import cmd_regions
        return cmd_regions(Path(take(argv, "--out") or REPO / "build/review/regions").resolve(),
                           ein_bin(take(argv, "--bin")))
    return cmd_bundle(argv)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
