"""Console entry point.

Subcommands:

    ein-bot ir parse <file>     # parse IR, dump canonical text to stdout
    ein-bot ir lint  <file>     # parse-only; exit non-zero on errors
    ein-bot ir dot   <file>     # render parsed IR as DOT (per docs/ir.md §6)

Invoked via the ``ein-bot`` console script or ``python -m ein_bot.cli``.
"""
from __future__ import annotations

import argparse
import sys
from collections.abc import Sequence
from pathlib import Path

from .ir import IRParseError, dump_canonical, parse, to_dot


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


def _cmd_ir_dot(args: argparse.Namespace) -> int:
    path = Path(args.file)
    try:
        nodes = parse(path.read_text(encoding="utf-8"), filename=str(path))
    except IRParseError as e:
        print(e, file=sys.stderr)
        return 1
    sys.stdout.write(to_dot(nodes, rule_mode=args.rule_mode,
                            trace_view=args.trace_view))
    sys.stdout.write("\n")
    return 0


def _build_parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(
        prog="ein-bot",
        description="Graph-based reasoner for Zebra-style puzzles.",
    )
    sub = p.add_subparsers(dest="cmd", required=True)

    ir = sub.add_parser("ir", help="IR utilities")
    ir_sub = ir.add_subparsers(dest="ir_cmd", required=True)

    ir_parse = ir_sub.add_parser("parse", help="parse and re-dump canonical IR")
    ir_parse.add_argument("file")
    ir_parse.set_defaults(func=_cmd_ir_parse)

    ir_lint = ir_sub.add_parser("lint", help="parse-only check; non-zero on error")
    ir_lint.add_argument("file")
    ir_lint.set_defaults(func=_cmd_ir_lint)

    ir_dot = ir_sub.add_parser("dot", help="render parsed IR as DOT (per docs/ir.md §6)")
    ir_dot.add_argument("file")
    ir_dot.add_argument("--rule-mode", choices=["a", "c"], default="c",
                        help="rule rendering: 'a' side-by-side, 'c' overlay (default)")
    ir_dot.add_argument("--trace-view", choices=["a", "b", "c"], default="a",
                        help="trace view: 'a' per-step (default), 'b' aggregate, 'c' DAG")
    ir_dot.set_defaults(func=_cmd_ir_dot)

    return p


def main(argv: Sequence[str] | None = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
