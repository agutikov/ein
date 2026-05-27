"""Run the monotonic set-search engine on a .ein file.

Mirrors :mod:`bench_solve`'s CLI shape but reaches into
``inference/monotonic/`` instead of ``inference/tree/``. Output
is one-shot: print the verdict + bindings + a brief per-layer
summary; with ``--dump-states DIR`` produce a minimal monotonic
dump (root snapshot per layer + ``00_timeline.jsonl``).

Stub — backbone wired in S1.5b.5.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

# Make `from ein_bot.…` resolve when running from a checkout.
sys.path.insert(
    0, str(Path(__file__).resolve().parents[1] / "src"),
)

from ein_bot.inference.config import SolverConfig
from ein_bot.inference.monotonic.solver import monotonic_solve
from ein_bot.inference.monotonic.state_dump import MonotonicDumper
from ein_bot.ir import parse
from ein_bot.kb.store import KnowledgeBase


def _build_argparser() -> argparse.ArgumentParser:
    ap = argparse.ArgumentParser(
        description=__doc__.split("\n", 1)[0],
    )
    ap.add_argument("puzzle", type=Path,
                    help="path to .ein puzzle file")
    ap.add_argument("--max-set-size", type=int, default=5,
                    help="largest commitment size to enumerate "
                         "(default: 5)")
    ap.add_argument("--dump-states", type=Path, default=None,
                    help="if set, write a minimal monotonic dump "
                         "to this directory")
    ap.add_argument("--max-time", type=float, default=None,
                    help="abort after N seconds")
    ap.add_argument("--verbose", "-v", action="store_true",
                    help="per-layer progress")
    return ap


def main(argv: list[str] | None = None) -> int:
    args = _build_argparser().parse_args(argv)
    text = args.puzzle.read_text()
    kb = KnowledgeBase.from_ir(parse(text))

    config = SolverConfig()  # default; flags can extend later

    dumper = (
        MonotonicDumper(out_dir=args.dump_states)
        if args.dump_states is not None else None
    )
    # NOTE — S1.5b.5 wires `dumper` into `monotonic_solve`'s
    # signature; current stub doesn't accept it.
    _ = dumper

    verdict = monotonic_solve(
        kb,
        max_set_size=args.max_set_size,
        config=config,
    )

    print(f"verdict: {type(verdict).__name__}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
