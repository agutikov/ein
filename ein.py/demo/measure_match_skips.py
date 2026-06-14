#!/usr/bin/env python3
"""Estimate the S1.8.B2v D5 (semi-naive matching) win — BEFORE building it.

D5 seeds each delta re-match AT the newly-derived fact instead of re-running
`match.run` from the plan's entry Scan, which re-scans the relation's whole
extent and re-discovers already-known bindings. The win ceiling is therefore
the fraction of `match.run`'s output that is **re-discovered** (skipped by the
saturator's `_seen` / `_fired` dedup) rather than newly enqueued.

This wraps `match.run` to count total bindings yielded and `Saturator.saturate`
to count firings (= bindings that were actually enqueued + applied). The skip
ratio `(yields - firings) / yields` is a *lower bound* on D5's saving (it
misses the entry-scan iterations that fail mid-join — also re-scan waste).

  high skip ratio (≳70%) → most matcher output is re-discovery → D5 pays.
  low skip ratio          → the matches are mostly new → D5 is a wash (like D3).

Run under PyPy (counts are deterministic + interpreter-independent).
Usage: python demo/measure_match_skips.py [puzzle] [--stop-after N | --exhaustive]
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

_REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(_REPO / "ein.py" / "src"))

from ein_bot.inference import match as _match_mod  # noqa: E402
from ein_bot.inference import saturator as _sat_mod  # noqa: E402
from ein_bot.inference.config import SolverConfig  # noqa: E402
from ein_bot.inference.monotonic.solver import solve  # noqa: E402
from ein_bot.ir import parse  # noqa: E402
from ein_bot.kb.store import KnowledgeBase  # noqa: E402

YIELDS = [0]
FIRINGS = [0]

_orig_run = _match_mod.run


def _counting_run(plan, kb):
    for b in _orig_run(plan, kb):
        YIELDS[0] += 1
        yield b


_match_mod.run = _counting_run  # saturator resolves `match.run` at call time

_orig_saturate = _sat_mod.Saturator.saturate


def _counting_saturate(self, *a, **k):
    for f in _orig_saturate(self, *a, **k):
        FIRINGS[0] += 1
        yield f


_sat_mod.Saturator.saturate = _counting_saturate


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("puzzle", type=Path, nargs="?",
                    default=_REPO / "examples" / "zebra2.ein")
    ap.add_argument("--stop-after", type=int, default=1)
    ap.add_argument("--exhaustive", action="store_true")
    args = ap.parse_args(argv)

    kb = KnowledgeBase.from_ir(parse(args.puzzle.read_text()))
    verdict, _stats = solve(
        kb, stop_after=None if args.exhaustive else args.stop_after,
        max_set_size=5, config=kb.config or SolverConfig(),
    )

    yields, firings = YIELDS[0], FIRINGS[0]
    skips = yields - firings
    ratio = (skips / yields * 100) if yields else 0.0
    print(f"file              {args.puzzle.name}")
    print(f"verdict           {type(verdict).__name__}")
    print(f"match.run yields  {yields}   (every full match produced)")
    print(f"firings (enqueued){firings:>8}   (bindings actually applied)")
    print(f"re-discovered     {skips}   ({ratio:.0f}% of yields)")
    print()
    if ratio >= 70:
        print(f"→ D5 PAYS: {ratio:.0f}% of matcher output is re-discovery "
              "that seeding-from-the-delta would skip.")
    else:
        print(f"→ D5 likely a WASH: only {ratio:.0f}% re-discovery; the "
              "matches are mostly new, so semi-naive saves little.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
