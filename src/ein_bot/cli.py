"""Console entry point: load a conditions file and dump it as DOT.

Invoked either via the ``ein-bot`` console script or as
``python -m ein_bot.cli``.
"""
from __future__ import annotations

import argparse
import sys
from collections.abc import Sequence

from . import State, load_file


def main(argv: Sequence[str] | None = None) -> int:
    p = argparse.ArgumentParser(
        prog="ein-bot",
        description="Load a conditions file (obj rel obj triples) and print DOT.",
    )
    p.add_argument("conditions", help="path to a conditions file")
    p.add_argument("--no-color", action="store_true",
                   help="emit plain DOT (no HTML-coloured labels)")
    p.add_argument("--dump", action="store_true",
                   help="dump the round-tripped text instead of DOT")
    args = p.parse_args(argv)

    state = State()
    load_file(state, args.conditions)
    if args.dump:
        sys.stdout.write(state.dump())
    else:
        sys.stdout.write(state.dot(colorfull=not args.no_color))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
