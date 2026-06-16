#!/usr/bin/env python3
"""Fresh P1.8a performance baseline — profile one ``solve()``.

P1.8a's first mandated task. Every stage README repeats it: *"re-measure
before optimising — the baseline moved"* — P1.7b's ``_explore_layers``
decomposition and the P1.7a sound search both landed after the original
2026-05-21 "saturation is INSANE" rant, so the old numbers are stale.

This answers the phase-gating question — *where does a solve actually
spend its time?* — so the first optimisation is chosen by data, not by
the README's guesses:

  - time dominated by ``fork()``                 → COW overlay  (B1-B4)
  - time dominated by ``saturate`` / ``match``   → indexes      (B-idx)
  - large unused ``(not …)`` volume              → neg-fact     (Theme C)

It mirrors ``bench_monotonic.main``'s load + solve exactly
(``KnowledgeBase.from_ir(parse(text))`` — resolves ``std.*`` imports;
``solve(stop_after=…, max_set_size=5)``), wraps the solve in cProfile,
and prints four blocks: full ``MonotonicStats``, the cProfile top-K by
tottime + cumtime, a targeted *by-subsystem* table (fork / saturate /
match / contradiction / hypgen / alive / canon), and the saturated-root
negative-fact volume (Theme C / S1.8.C1).

Run under **CPython** for cProfile attribution (relative %); cProfile
distorts PyPy's JIT, so take wall-clock from PyPy
(``./ein_pypy.sh search``) instead. ``--no-profile`` gives a
clean wall-clock here too.

Usage:
  python3 demo/profile_solve.py [puzzle] [--stop-after N | --exhaustive]
                                [--top K] [--max-set-size N] [--no-profile]

Defaults: examples/zebra2.ein, --stop-after 1, --top 25.
"""
from __future__ import annotations

import argparse
import cProfile
import pstats
import sys
import time
from dataclasses import fields as dc_fields
from pathlib import Path

from ein.inference.config import SolverConfig
from ein.inference.monotonic.solver import solve
from ein.ir import parse
from ein.kb.store import KnowledgeBase

# Substrings grouping cProfile rows into engine subsystems. First match
# wins (order matters — `monotonic/solver` before generic `match`).
_SUBSYSTEMS: list[tuple[str, tuple[str, ...]]] = [
    ("fork/copy",     ("store.py:fork", "_copy_fact_indexes_into",
                       "snapshot")),
    ("saturate",      ("saturator.py", "/compile.py", "firing.py")),
    ("match/bind",    ("match.py", "/resolve.py")),
    ("contradiction", ("contradiction.py", "nogoods.py", "naf_deps.py")),
    ("hypgen/branch", ("hypgen.py", "lookahead.py", "commitment.py")),
    ("alive/closed",  ("solver.py:_compute_alive", "closed.py",
                       "solution.py")),
    ("canon/hash",    ("canon.py", "state_hash")),
    ("apriori/elim",  ("apriori.py", "predicates.py", "why.py")),
]


def _default_puzzle() -> Path:
    # repo root = …/src/ein/cli/profile.py → parents[4]; examples/ lives there.
    return Path(__file__).resolve().parents[4] / "examples" / "zebra2.ein"


def _build_argparser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(
        description=__doc__.split("\n", 1)[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    ap.add_argument("puzzle", type=Path, nargs="?", default=None,
                    help="path to .ein puzzle (default: examples/zebra2.ein)")
    ap.add_argument("--stop-after", type=int, default=1,
                    help="stop at the Nth solution node (default: 1, the "
                         "fast answer path); ignored under --exhaustive")
    ap.add_argument("--exhaustive", action="store_true",
                    help="exhaust the lattice (stop_after=None) so k is "
                         "exact — the slow path (~90s zebra2 under PyPy)")
    ap.add_argument("--max-set-size", type=int, default=5,
                    help="largest commitment size to enumerate (default 5)")
    ap.add_argument("--top", type=int, default=25,
                    help="cProfile rows to show per ordering (default 25)")
    ap.add_argument("--no-profile", action="store_true",
                    help="skip cProfile (clean wall-clock only)")
    ap.add_argument("--callers", action="store_true",
                    help="attribute the matcher (match.run / _run_steps) "
                         "cumtime BY CALLER — splits the matching cost across "
                         "saturator / hypgen / lookahead / query")
    return ap


def _count_negatives(kb: KnowledgeBase) -> tuple[int, int, dict[str, int]]:
    """Saturate a fork of the root and count ``(not …)`` facts.

    Returns (total_facts, negative_count, per-inner-relation histogram).
    Serves Theme C / S1.8.C1 — the volume side of the measurement (the
    *consumption* side needs detector instrumentation, deferred to C1
    proper). Mirrors the root preview path bench_monotonic uses.
    """
    from collections import Counter

    from ein.inference.closed import emit_closed
    from ein.inference.saturator import Saturator
    from ein.kb.entities import Fact

    preview = kb.fork()
    emit_closed(preview)
    list(Saturator(preview).saturate())
    by_inner: Counter[str] = Counter()
    neg = 0
    for f in preview.facts:
        if f.relation_name == "not" and f.args and isinstance(f.args[0], Fact):
            neg += 1
            by_inner[f.args[0].relation_name] += 1
    return len(preview.facts), neg, dict(by_inner)


def _print_stats(stats, elapsed: float) -> None:
    print("\n── stats ──────────────────────────────────────────────")
    for f in dc_fields(stats):
        v = getattr(stats, f.name)
        if isinstance(v, (dict, list)):
            v = len(v)
        shown = str(v).lower() if isinstance(v, bool) else v
        print(f"  {f.name:<22s} {shown}")
    print(f"  {'wall_ms':<22s} {elapsed * 1000:.1f}")


def _print_negatives(kb: KnowledgeBase) -> None:
    total, neg, by_inner = _count_negatives(kb)
    pos = total - neg
    frac = (neg / total * 100) if total else 0.0
    print("\n── saturated-root negative-fact volume (Theme C) ──────")
    print(f"  root facts (post-saturation)  {total}")
    print(f"  (not …) facts                 {neg}  ({frac:.0f}% of root)")
    print(f"  positive / other              {pos}")
    if by_inner:
        print("  top (not <R> …) by inner relation:")
        for rel, n in sorted(by_inner.items(), key=lambda kv: -kv[1])[:8]:
            print(f"    (not ({rel} …))   {n}")


def _print_subsystems(pr: cProfile.Profile, total_wall: float) -> None:
    """Group the profile's tottime by engine subsystem (first-match)."""
    st = pstats.Stats(pr)
    buckets: dict[str, list[float]] = {name: [0.0, 0.0, 0]
                                       for name, _ in _SUBSYSTEMS}
    other = [0.0, 0.0, 0]
    for (fn, _lineno, name), (_cc, nc, tt, ct, _cs) in st.stats.items():
        key = f"{Path(fn).name}:{name}"
        label = None
        for sub_name, needles in _SUBSYSTEMS:
            if any(nd in key or nd in Path(fn).name for nd in needles):
                label = sub_name
                break
        tgt = buckets[label] if label else other
        tgt[0] += tt        # tottime (self)
        tgt[1] += ct        # cumtime
        tgt[2] += nc        # ncalls
    print("\n── time by subsystem (cProfile tottime, self) ─────────")
    print(f"  {'subsystem':<16s} {'tottime':>9s} {'%':>6s} "
          f"{'cumtime':>9s} {'ncalls':>10s}")
    rows = [(name, *buckets[name]) for name, _ in _SUBSYSTEMS]
    rows.append(("other", *other))
    prof_total = sum(r[1] for r in rows) or 1.0
    for name, tt, ct, nc in sorted(rows, key=lambda r: -r[1]):
        print(f"  {name:<16s} {tt:>9.3f} {tt / prof_total * 100:>5.0f}% "
              f"{ct:>9.3f} {int(nc):>10d}")
    print(f"  {'(profiled CPU)':<16s} {prof_total:>9.3f}  "
          f"(wall {total_wall:.3f}s incl. profile overhead)")


def main(argv: list[str] | None = None) -> int:
    args = _build_argparser().parse_args(argv)
    puzzle = args.puzzle or _default_puzzle()
    stop_after = None if args.exhaustive else args.stop_after

    kb = KnowledgeBase.from_ir(parse(puzzle.read_text()))
    config = kb.config or SolverConfig()

    print(f"file              {puzzle}")
    print(f"stop_after        {stop_after}")
    print(f"max_set_size      {args.max_set_size}")
    print(f"interpreter       {sys.implementation.name} "
          f"{sys.version.split()[0]}")

    pr = cProfile.Profile() if not args.no_profile else None
    t0 = time.perf_counter()
    if pr is not None:
        pr.enable()
    verdict, stats = solve(
        kb,
        stop_after=stop_after,
        max_set_size=args.max_set_size,
        config=config,
    )
    if pr is not None:
        pr.disable()
    elapsed = time.perf_counter() - t0

    print(f"verdict           {type(verdict).__name__}")
    _print_stats(stats, elapsed)

    if pr is not None:
        st = pstats.Stats(pr)
        print("\n── cProfile: top by tottime (self) ────────────────────")
        st.sort_stats("tottime").print_stats(args.top)
        print("── cProfile: top by cumtime ───────────────────────────")
        st.sort_stats("cumulative").print_stats(args.top)
        _print_subsystems(pr, elapsed)
        if args.callers:
            # Who drives the matcher? `run` is the entry every subsystem
            # calls (saturator `_enqueue_pass`, hypgen, lookahead, the query
            # goal); `_run_steps` is the recursive core. Their callers split
            # the 70%-matching cost by subsystem.
            print("\n── matcher entry: callers of match.run ────────────────")
            st.print_callers(r"match\.py:\d+\(run\)")
            print("── matcher core: callers of _run_steps ────────────────")
            st.print_callers(r"match\.py:\d+\(_run_steps\)")

    # Theme-C volume: a second (cheap) saturation of a fresh fork.
    _print_negatives(KnowledgeBase.from_ir(parse(puzzle.read_text())))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
