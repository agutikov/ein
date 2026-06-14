#!/usr/bin/env python3
"""Measure the redundant-firing fraction — sizes the B2.v incremental-
saturation opportunity (S1.8a, after S1.8.B-idx).

The profile showed ~12 saturation runs (1 root + ~11 ``try_commitment_set``
forks + a few forced-positive re-saturations) yielding ~10,706 firings.
Each fork re-saturates a fork of the **already-saturated root** from
scratch, so it re-derives the root's facts as ``redundant=True`` firings
(`saturator.py:28`, `firing.py:61`) before cascading the hypothesis delta.
Incremental fork-saturation (B2.v) would skip exactly those redundant
re-derivations.

This wraps ``Saturator.saturate`` to tally, **per saturation run**,
redundant vs productive firings, then reports the **B2.v ceiling** =
redundant / total (the fraction of firing-work incremental saturation
could eliminate — an upper bound, since the forks still pay for the new
cascade + NAF re-validation).

Counts are deterministic + interpreter-independent — run under PyPy
(`.venv-pypy/bin/python`, fast) or CPython.

Usage:
  python demo/measure_redundant_firings.py [puzzle] [--stop-after N | --exhaustive]
Defaults: examples/zebra2.ein, --stop-after 1.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

_REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(_REPO / "ein.py" / "src"))

from ein_bot.inference import saturator as _sat_mod  # noqa: E402
from ein_bot.inference.config import SolverConfig  # noqa: E402
from ein_bot.inference.monotonic.solver import solve  # noqa: E402
from ein_bot.ir import parse  # noqa: E402
from ein_bot.kb.store import KnowledgeBase  # noqa: E402

# (redundant, productive) per saturation run, in completion order.
RUNS: list[tuple[int, int]] = []
_orig_saturate = _sat_mod.Saturator.saturate


def _wrapped_saturate(self, *args, **kwargs):
    """Tally redundant vs productive firings for one saturation run."""
    red = 0
    new = 0
    for f in _orig_saturate(self, *args, **kwargs):
        if f.redundant:
            red += 1
        else:
            new += 1
        yield f
    RUNS.append((red, new))


_sat_mod.Saturator.saturate = _wrapped_saturate


def _default_puzzle() -> Path:
    return _REPO / "examples" / "zebra2.ein"


def _build_argparser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(
        description=__doc__.split("\n", 1)[0],
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    ap.add_argument("puzzle", type=Path, nargs="?", default=None,
                    help="path to .ein puzzle (default: examples/zebra2.ein)")
    ap.add_argument("--stop-after", type=int, default=1,
                    help="stop at the Nth solution node (default 1)")
    ap.add_argument("--exhaustive", action="store_true",
                    help="exhaust the lattice (stop_after=None)")
    ap.add_argument("--max-set-size", type=int, default=5)
    return ap


def main(argv: list[str] | None = None) -> int:
    args = _build_argparser().parse_args(argv)
    puzzle = args.puzzle or _default_puzzle()
    stop_after = None if args.exhaustive else args.stop_after

    kb = KnowledgeBase.from_ir(parse(puzzle.read_text()))
    verdict, _stats = solve(
        kb, stop_after=stop_after, max_set_size=args.max_set_size,
        config=kb.config or SolverConfig(),
    )

    total_red = sum(r for r, _ in RUNS)
    total_new = sum(n for _, n in RUNS)
    total = total_red + total_new
    pct = (total_red / total * 100) if total else 0.0

    # The root is the run with the most productive firings (it builds the
    # base extent); fork re-derivations show up as its redundant tally.
    runs_sorted = sorted(RUNS, key=lambda rn: -(rn[0] + rn[1]))
    root = max(RUNS, key=lambda rn: rn[1]) if RUNS else (0, 0)
    fork_runs = [r for r in RUNS if r is not root]
    fork_red = sum(r for r, _ in fork_runs)
    fork_new = sum(n for _, n in fork_runs)

    print(f"file              {puzzle}")
    print(f"interpreter       {sys.implementation.name} "
          f"{sys.version.split()[0]}")
    print(f"verdict           {type(verdict).__name__}")
    print(f"stop_after        {stop_after}")
    print()
    print(f"saturation runs   {len(RUNS)}")
    print(f"firings total     {total}")
    print(f"  redundant       {total_red}  ({pct:.0f}%)")
    print(f"  productive      {total_new}  ({100 - pct:.0f}%)")
    print()
    print(f"root run          redundant={root[0]} productive={root[1]} "
          f"(the base saturation — kept by B2.v)")
    print(f"fork runs ({len(fork_runs)})     redundant={fork_red} "
          f"productive={fork_new}")
    fork_total = fork_red + fork_new
    fork_pct = (fork_red / fork_total * 100) if fork_total else 0.0
    print(f"  fork redundancy {fork_pct:.0f}% of fork firing-work")
    print()
    print(f"B2.v ceiling      ~{pct:.0f}% of all firing-work is "
          f"redundant re-derivation")
    print("  (upper bound — forks still pay the new cascade + NAF "
          "re-validation)")
    print()
    print("per-run (redundant, productive), largest first:")
    for r, n in runs_sorted[:14]:
        print(f"  red={r:>5d}  new={n:>5d}  total={r + n:>5d}")
    if len(runs_sorted) > 14:
        print(f"  … +{len(runs_sorted) - 14} more runs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
