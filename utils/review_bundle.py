#!/usr/bin/env python3
"""The review bundle — M1e2 P1e2.1.

    utils/review_bundle.py [--out DIR] [CASE | FILE.ein …]      the bundle
    utils/review_bundle.py --corpus [--out DIR]                 every corpus entry
    utils/review_bundle.py --smoke [--keep DIR]                 the gate's check
    utils/review_bundle.py --regions [--out DIR]                the region gallery (build/review/regions/)
    utils/review_bundle.py run FILE [SOLVE FLAGS…] --out DIR [--env NAME=V]…

**The bundle** (S1e2.1.6), default `build/review/`: an index, and one page per
row of `corpus/wrong_answers.toml` — the row's `[case.now]` region run beside
the walk's wrong answer, control and twins (the register's record of them:
the walk has no CLI path since S1e2.1a.5's switch) and the right answer, the
models diffed, § The region — R by the oracle with this run drawn on it,
beside the register's lines (S1e2.1a.7) — links to where it went wrong (the
row's `look_at`), every artefact two clicks away, and the checklist the
review answers. A fixed row says what fixed it (`[case.fixed]`). A CASE is a
row id (`W1/branching-15`) or a case (`W1`, every row of it); a `.ein` path
gets a page of its own with one `solve` run (flags after `--`; none describes
all of R). `--corpus` builds one for every corpus entry with a declared
`solve` run, at that run's flags and without events.
`--smoke` builds one small case, and one program whose solve is refused, into
a temporary directory and link-checks it: the per-commit gate's step, using
the debug `ein` `cargo test` built. Since M1f S1f.5.19 it also builds two
cases whose run is not a solve, and first reads every register row with no
run through the index and the case pages' register-only parts. What it
checks, in order: the register pass; the four builds, each a case that
builds and every DOT drawn (below); every SVG non-empty; and every link and
image reference resolving (`doc_audit.py --links --check`). `--keep DIR`
builds into DIR, new or empty, and leaves it, so a red smoke's DOTs can be
drawn again elsewhere (S1f.5.29).

**Every DOT drawn** (M1f S1f.5.29): the bundle, the smoke's and the
nightly's whole one alike, exits 1 on a DOT Graphviz refused, printing its
path and, indented under it, what Graphviz said (kept beside the DOT as
`.refused.txt`). A DOT kept and not laid out by design, a lattice past
`DAG_NODES_DRAWN`, says so beside it (`.undrawn.txt`) and is not a refusal.
A KB view suggests `fdp`, so the Graphviz needs a layout plugin beside
`dot`'s: on Ubuntu 26.04 that is `libgvplugin-neato-layout8`, which
`apt-get install graphviz` does not pull, and without it every KB view is
refused (`There is no layout engine support for "fdp"`).

**A row whose `[case.now]` is a command's run** (the register's schema /16:
`saturate`, a `render` view, `kb save`) gets a page of its own (S1f.5.19,
`review/command.py`): the command as run, its exit, stdout and stderr in full,
against the register's `wrong`, `now` and `right` as
`ein-cli/tests/wrong_answers.rs` reads them, and every artefact it writes,
linked and drawn. A `container` row runs on the witness saved as a `.einb`.
A case that fails to build makes the command exit 1, as a refused DOT does.
`--regions` (S1e2.1a.8) surveys every corpus program's region and writes a
gallery: the synthetic programs `utils/region_synth.py` wrote, the biggest
regions and the most interesting, each with a page (`review/regions.py`).

One run, fully explicit (S1e2.1.5): every artefact `ein` can write for
`ein solve FILE FLAGS`, and `ein oracle FILE --dump`'s cube beside it, kept
verbatim under DIR/raw/ and rendered as pages under DIR/pages/, with
DIR/run.md as the table of contents. Every graph is an SVG, every JSON a
table, and DIR/pages/consistency.md checks the artefacts against each other
and the run against the oracle.

The engine is `--bin`, else `$EIN_BIN`, else `ein.rs/target/release/ein`.
Graphviz (`dot`) must be on PATH.
"""

from __future__ import annotations

import os
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from review.run import RunSpec, asked_text, build_run, solve_flags  # noqa: E402
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
    print(f"{r.out}/run.md — {asked_text(r.summary)}{', ' + r.verdict if r.verdict else ''}, "
          f"{r.k} state(s) listed, exhausted = {str(r.exhausted).lower()}; "
          f"{len(r.pages)} pages; {len(applicable) - len(fails)}/{len(applicable)} cross-checks pass; "
          f"{time.time() - t:.1f} s")
    for c, _, d in fails:
        print(f"  MISMATCH {c}: {d}")
    return 0


def cmd_bundle(args: list[str]) -> int:
    from review.case import case_page, index_page, program_case, register
    from review.command import CommandRun
    from review.util import refused_dots
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
            got = next(c.result for c in cols if c.result is not None)
            said = (f"exit={got.main.code}" if isinstance(got, CommandRun) else
                    f"k={got.k} exhausted={str(got.exhausted).lower()}")
            print(f"{row['id']:32s} {row['now']['run']:16s} {said} — {page.relative_to(out)}")
        except Exception as e:  # noqa: BLE001 — a case that fails is reported, not fatal
            failed.append((row["id"], f"{type(e).__name__}: {e}"))
            print(f"{row['id']:32s} FAILED: {e}", file=sys.stderr)
    for f in files:
        rel_file = str(Path(f).resolve().relative_to(REPO)) if Path(f).resolve().is_relative_to(REPO) else f
        try:
            page, r = program_case(rel_file, flags, out / "programs" / rel_file.removesuffix(".ein"), bin)
            extra.append((rel_file, page, r))
            print(f"{rel_file:32s} {asked_text(r.summary)} k={r.k} exhausted={str(r.exhausted).lower()}")
        except Exception as e:  # noqa: BLE001
            failed.append((rel_file, f"{type(e).__name__}: {e}"))
    index = index_page(out, built, extra, failed, bin)
    # A DOT Graphviz refused fails the bundle as a case does (M1f S1f.5.29),
    # with what Graphviz printed: until then the page said so and the run
    # exited 0, so the nightly's step drew no KB view from 2026-11-06 and passed.
    # Only the cases built here are read, each rebuilt from an empty directory.
    refused = [hit for d in [out / row["id"] for row, _, _ in built]
               + [out / "programs" / f.removesuffix(".ein") for f, _, _ in extra] for hit in refused_dots(d)]
    for dot, said in refused:
        print(f"error: Graphviz refused {dot.relative_to(out)}", file=sys.stderr)
        print("\n".join("    " + line for line in said.splitlines()), file=sys.stderr)
    print(f"{index} — {len(built)} cases, {len(extra)} programs, {len(failed)} failed, "
          f"{len(refused)} DOTs refused; {time.time() - t:.1f} s")
    return 1 if failed or refused else 0


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
        flags = solve_flags(run_name)
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
    every SVG non-empty — the per-commit gate's step (S1e2.1.6). Beside it, a
    program whose `ein solve` is refused (its goal does not compile, exit 1),
    so it writes no dump: a page that links a dump the run did not write
    dangles, which is how the region gallery's `features/02` page did
    (M1e2 S1e2.1.7).

    Since M1f S1f.5.19 (Q-M1f.53) two cases whose run is not a solve too —
    `W40/kb-save`, a container and the container its run writes, and
    `W41/saturate-import`, whose `wrong` is a refusal — and, first, every row
    of the register read with no run (`register_pass`): the index's cells and
    each case page's register-only parts. The whole bundle, the nightly's, had
    been the only reader of a row shape, and stopped on W41's from `fc53818f`
    to S1f.5.19 behind a green gate.

    Every DOT it keeps has its drawing: a graph Graphviz refused fails it,
    each printed with what Graphviz said (`cmd_bundle`, M1f S1f.5.29). The
    check is S1f.5.19's; the message is S1f.5.29's, whose cause, a runner
    whose `fdp` has no layout plugin, the bare path did not say. `--keep DIR`
    builds into DIR, which must be new or empty, and leaves it there."""
    import subprocess
    import tempfile
    from review.case import register, register_pass
    debug = REPO / "ein.rs/target/debug/ein"
    bin = take(args, "--bin") or os.environ.get("EIN_BIN") or str(debug)
    keep = take(args, "--keep")
    if keep and Path(keep).exists() and any(Path(keep).iterdir()):
        print(f"error: --keep {keep}: not empty; the smoke builds into a new or empty directory", file=sys.stderr)
        return 2
    rows = register()
    failed = register_pass(rows)
    for rid, e in failed:
        print(f"{rid:32s} the register pass FAILED: {e}", file=sys.stderr)
    print(f"smoke: the register pass read {len(rows)} rows, {len(failed)} failed")
    if failed:
        return 1

    def build(out: Path) -> int:
        code = cmd_bundle(["--bin", bin, "--out", str(out), "W2/forced-2", "W40/kb-save", "W41/saturate-import",
                           "examples/ein-bugs/query-goal-free-head.ein"])
        if code:
            return code
        svgs = list(out.rglob("*.svg"))
        empty = [p for p in svgs if p.stat().st_size == 0]
        if not svgs or empty:
            print(f"error: {len(svgs)} SVGs, {len(empty)} empty", file=sys.stderr)
            return 1
        check = subprocess.run([sys.executable, str(REPO / "utils/doc_audit.py"), "--links", "--check", str(out)])
        print(f"smoke: {len(svgs)} SVGs, link check exit {check.returncode}")
        return check.returncode

    if keep:
        out = Path(keep).resolve()
        out.mkdir(parents=True, exist_ok=True)
        return build(out)
    with tempfile.TemporaryDirectory(prefix="ein-review-smoke-") as tmp:
        return build(Path(tmp))


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
