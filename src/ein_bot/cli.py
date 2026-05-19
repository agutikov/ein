"""Console entry point.

Subcommands:

    ein-bot ir parse <file>     # parse IR, dump canonical text to stdout
    ein-bot ir lint  <file>     # parse-only; exit non-zero on errors
    ein-bot ir dot   <file>     # render IR as one DOT per top-level form
    ein-bot kb dot   <file>     # render the loaded KB as a unified DOT
                                # (S1.2.4 — per docs/kernel/ir/03-ein-lang/04_dot_rendering.md)

Invoked via the ``ein-bot`` console script or ``python -m ein_bot.cli``.
"""
from __future__ import annotations

import argparse
import sys
from collections.abc import Sequence
from pathlib import Path

from .ir import IRParseError, dump_canonical, parse
from .ir import to_dot as ir_to_dot
from .kb import KBLoadError, KnowledgeBase, Layer


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
    sys.stdout.write(ir_to_dot(nodes, rule_mode=args.rule_mode,
                               trace_view=args.trace_view))
    sys.stdout.write("\n")
    return 0


_LAYER_BY_NAME = {
    "ontology": Layer.ONTOLOGY,
    "fact": Layer.FACT,
    "facts": Layer.FACT,   # alias — the IR top-level block is `(facts …)`.
    "reasoning": Layer.REASONING,
}


def _cmd_kb_dot(args: argparse.Namespace) -> int:
    """Render the loaded KB as a single unified Graphviz digraph."""
    path = Path(args.file)
    try:
        nodes = parse(path.read_text(encoding="utf-8"), filename=str(path))
    except IRParseError as e:
        print(e, file=sys.stderr)
        return 1
    try:
        kb = KnowledgeBase.from_ir(nodes)
    except KBLoadError as e:
        print(f"kb load error: {e}", file=sys.stderr)
        return 1
    # Parse --layers spec; default to all.
    if args.layers:
        try:
            layer_set = tuple(
                _LAYER_BY_NAME[name.strip().lower()]
                for name in args.layers.split(",")
            )
        except KeyError as e:
            valid = ",".join(sorted(set(_LAYER_BY_NAME.values()) and _LAYER_BY_NAME))
            print(f"unknown layer {e}; pick from {valid}", file=sys.stderr)
            return 2
    else:
        layer_set = (Layer.ONTOLOGY, Layer.FACT, Layer.REASONING)
    sys.stdout.write(kb.to_dot(
        layers=layer_set,
        colour_by=args.colour_by,
        include_types=not args.no_types,
        include_instances=not args.no_instances,
        name=args.graph_name,
    ))
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

    ir_dot = ir_sub.add_parser(
        "dot",
        help="render parsed IR as DOT (per docs/kernel/ir/03-ein-lang/04_dot_rendering.md)",
    )
    ir_dot.add_argument("file")
    ir_dot.add_argument("--rule-mode", choices=["a", "c"], default="c",
                        help="rule rendering: 'a' side-by-side, 'c' overlay (default)")
    ir_dot.add_argument("--trace-view", choices=["a", "b", "c"], default="a",
                        help="trace view: 'a' per-step (default), 'b' aggregate, 'c' DAG")
    ir_dot.set_defaults(func=_cmd_ir_dot)

    # ── kb subcommand ──
    kb_cmd = sub.add_parser("kb", help="Knowledge-base utilities")
    kb_sub = kb_cmd.add_subparsers(dest="kb_cmd", required=True)

    kb_dot = kb_sub.add_parser(
        "dot",
        help="render the loaded KB as a unified DOT graph (S1.2.4)",
    )
    kb_dot.add_argument("file")
    kb_dot.add_argument(
        "--layers",
        default=None,
        help="comma-separated layer subset (ontology,facts,reasoning); default: all",
    )
    kb_dot.add_argument(
        "--colour-by",
        choices=["relation", "layer", "none"],
        default="relation",
        help="colour edges by relation (default), by layer, or none",
    )
    kb_dot.add_argument(
        "--no-types",
        action="store_true",
        help="omit Type nodes (boxes)",
    )
    kb_dot.add_argument(
        "--no-instances",
        action="store_true",
        help="omit Instance nodes (ovals)",
    )
    kb_dot.add_argument(
        "--graph-name",
        default="kb",
        help="DOT graph name (default 'kb')",
    )
    kb_dot.set_defaults(func=_cmd_kb_dot)

    return p


def main(argv: Sequence[str] | None = None) -> int:
    parser = _build_parser()
    args = parser.parse_args(argv)
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
