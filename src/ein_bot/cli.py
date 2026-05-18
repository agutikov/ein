"""Console entry point.

Subcommands:

    ein-bot ir parse <file>     # parse IR, dump canonical text to stdout
    ein-bot ir lint  <file>     # parse-only; exit non-zero on errors
    ein-bot legacy   <file>     # 2021 PoC: load conditions file, emit DOT

Invoked via the ``ein-bot`` console script or ``python -m ein_bot.cli``.
"""
from __future__ import annotations

import argparse
import sys
from collections.abc import Sequence
from pathlib import Path

from . import State, load_file
from .ir import IRParseError, dump_canonical, parse


def _cmd_ir_parse(args: argparse.Namespace) -> int:
    path = Path(args.file)
    try:
        nodes = parse(path.read_text(encoding="utf-8"), filename=str(path))
    except IRParseError as e:
        print(e, file=sys.stderr)
        return 1
    sys.stdout.write(dump_canonical(nodes))
    return 0


def _cmd_ir_lint(args: argparse.Namespace) -> int:
    path = Path(args.file)
    try:
        parse(path.read_text(encoding="utf-8"), filename=str(path))
    except IRParseError as e:
        print(e, file=sys.stderr)
        return 1
    return 0


def _cmd_legacy(args: argparse.Namespace) -> int:
    state = State()
    load_file(state, args.conditions)
    if args.dump:
        sys.stdout.write(state.dump())
    else:
        sys.stdout.write(state.dot(colorfull=not args.no_color))
    return 0


def _build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="ein-bot",
        description="Graph-based reasoner for Zebra-style puzzles.",
    )
    sub = p.add_subparsers(dest="cmd", required=True)

    # ir ...
    ir = sub.add_parser("ir", help="IR utilities")
    ir_sub = ir.add_subparsers(dest="ir_cmd", required=True)

    ir_parse = ir_sub.add_parser("parse", help="parse and re-dump canonical IR")
    ir_parse.add_argument("file")
    ir_parse.set_defaults(func=_cmd_ir_parse)

    ir_lint = ir_sub.add_parser("lint", help="parse-only check; non-zero on error")
    ir_lint.add_argument("file")
    ir_lint.set_defaults(func=_cmd_ir_lint)

    # legacy ...
    legacy = sub.add_parser(
        "legacy",
        help="2021 PoC: load conditions file, emit DOT or dumped text",
    )
    legacy.add_argument("conditions", help="path to a conditions file")
    legacy.add_argument("--no-color", action="store_true",
                        help="emit plain DOT (no HTML-coloured labels)")
    legacy.add_argument("--dump", action="store_true",
                        help="dump the round-tripped text instead of DOT")
    legacy.set_defaults(func=_cmd_legacy)

    return p


def main(argv: Sequence[str] | None = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
